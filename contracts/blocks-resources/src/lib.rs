#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec, Map};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Resource(Symbol),
    MultisigConfig,
    Transaction(u32),
    TransactionCount,
}

#[derive(Clone)]
#[contracttype]
pub struct Resource {
    pub id: Symbol,
    pub name: Symbol,
    pub resource_type: Symbol,
    pub quantity: u32,
    pub origin: Symbol,
    pub carbon_footprint: u32,
    pub owner: Address,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct MultisigConfig {
    pub signers: Vec<Address>,
    pub required_signatures: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct Transaction {
    pub id: u32,
    pub resource_id: Symbol,
    pub from: Address,
    pub to: Address,
    pub quantity: u32,
    pub signatures: Vec<Address>,
    pub executed: bool,
}

#[contract]
pub struct BlocksResources;

#[contractimpl]
impl BlocksResources {
    /// Initialize the contract with multisig configuration
    pub fn initialize(env: Env, signers: Vec<Address>, required_signatures: u32) {
        assert!(signers.len() >= required_signatures, "Invalid configuration");
        
        let config = MultisigConfig {
            signers: signers.clone(),
            required_signatures,
        };
        
        env.storage().persistent().set(&DataKey::MultisigConfig, &config);
    }

    /// Register a new environmental resource
    pub fn register_resource(
        env: Env,
        id: Symbol,
        name: Symbol,
        resource_type: Symbol,
        quantity: u32,
        origin: Symbol,
        carbon_footprint: u32,
        owner: Address,
    ) {
        owner.require_auth();
        
        let resource = Resource {
            id: id.clone(),
            name,
            resource_type,
            quantity,
            origin,
            carbon_footprint,
            owner: owner.clone(),
            timestamp: env.ledger().timestamp(),
        };
        
        env.storage().persistent().set(&DataKey::Resource(id), &resource);
    }

    /// Create a transfer transaction (requires multisig)
    pub fn create_transfer(
        env: Env,
        resource_id: Symbol,
        from: Address,
        to: Address,
        quantity: u32,
    ) -> u32 {
        from.require_auth();
        
        // Verify resource exists and owner
        let resource: Resource = env.storage().persistent().get(&DataKey::Resource(resource_id.clone()))
            .expect("Resource not found");
        
        assert!(resource.owner == from, "Not the resource owner");
        assert!(resource.quantity >= quantity, "Insufficient quantity");
        
        // Get transaction count
        let count = env.storage().persistent().get(&DataKey::TransactionCount).unwrap_or(0);
        let tx_id = count + 1;
        
        let tx = Transaction {
            id: tx_id,
            resource_id: resource_id.clone(),
            from: from.clone(),
            to: to.clone(),
            quantity,
            signatures: Vec::new(&env),
            executed: false,
        };
        
        env.storage().persistent().set(&DataKey::Transaction(tx_id), &tx);
        env.storage().persistent().set(&DataKey::TransactionCount, &tx_id);
        
        tx_id
    }

    /// Sign a transaction (multisig)
    pub fn sign_transaction(env: Env, tx_id: u32, signer: Address) {
        signer.require_auth();
        
        let config: MultisigConfig = env.storage().persistent().get(&DataKey::MultisigConfig)
            .expect("Not initialized");
        
        // Verify signer is authorized
        let mut is_authorized = false;
        for i in 0..config.signers.len() {
            if config.signers.get(i).unwrap() == signer {
                is_authorized = true;
                break;
            }
        }
        assert!(is_authorized, "Unauthorized signer");
        
        // Get transaction
        let mut tx: Transaction = env.storage().persistent().get(&DataKey::Transaction(tx_id))
            .expect("Transaction not found");
        
        assert!(!tx.executed, "Transaction already executed");
        
        // Check if already signed
        let mut already_signed = false;
        for i in 0..tx.signatures.len() {
            if tx.signatures.get(i).unwrap() == signer {
                already_signed = true;
                break;
            }
        }
        assert!(!already_signed, "Already signed");
        
        // Add signature
        tx.signatures.push_back(signer);
        
        // Execute if enough signatures
        if tx.signatures.len() >= config.required_signatures {
            // Update resource
            let mut resource: Resource = env.storage().persistent().get(&DataKey::Resource(tx.resource_id.clone()))
                .expect("Resource not found");
            
            resource.quantity -= tx.quantity;
            env.storage().persistent().set(&DataKey::Resource(tx.resource_id.clone()), &resource);
            
            // Create new resource for recipient
            let new_resource_id = symbol_short!("new");
            let new_resource = Resource {
                id: new_resource_id.clone(),
                name: resource.name,
                resource_type: resource.resource_type,
                quantity: tx.quantity,
                origin: resource.origin,
                carbon_footprint: resource.carbon_footprint,
                owner: tx.to.clone(),
                timestamp: env.ledger().timestamp(),
            };
            
            env.storage().persistent().set(&DataKey::Resource(new_resource_id), &new_resource);
            
            tx.executed = true;
        }
        
        env.storage().persistent().set(&DataKey::Transaction(tx_id), &tx);
    }

    /// Get resource details
    pub fn get_resource(env: Env, id: Symbol) -> Resource {
        env.storage().persistent().get(&DataKey::Resource(id))
            .expect("Resource not found")
    }

    /// Get transaction details
    pub fn get_transaction(env: Env, tx_id: u32) -> Transaction {
        env.storage().persistent().get(&DataKey::Transaction(tx_id))
            .expect("Transaction not found")
    }

    /// Get multisig configuration
    pub fn get_config(env: Env) -> MultisigConfig {
        env.storage().persistent().get(&DataKey::MultisigConfig)
            .expect("Not initialized")
    }
}
