use soroban_sdk::{contracttype, Address, Env, Map, Symbol, Vec};

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

pub struct Multisig;

impl Multisig {
    pub fn initialize(env: Env, signers: Vec<Address>, required_signatures: u32) {
        assert!(signers.len() >= required_signatures, "Configuración inválida");

        let config = MultisigConfig {
            signers: signers.clone(),
            required_signatures,
        };

        env.storage().persistent().set(&crate::DataKey::MultisigConfig, &config);
    }

    pub fn create_transfer(
        env: Env,
        resource_id: Symbol,
        from: Address,
        to: Address,
        quantity: u32,
    ) -> u32 {
        from.require_auth();

        // Verificar que el recurso existe y que el emisor es el propietario
        let resource: crate::resource::Resource = env
            .storage()
            .persistent()
            .get(&crate::DataKey::Resource(resource_id.clone()))
            .expect("Recurso no encontrado");

        assert!(resource.owner == from, "No eres el propietario del recurso");
        assert!(resource.quantity >= quantity, "Cantidad insuficiente");

        // Obtener contador de transacciones
        let count = env
            .storage()
            .persistent()
            .get(&crate::DataKey::TransactionCount)
            .unwrap_or(0);
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

        env.storage()
            .persistent()
            .set(&crate::DataKey::Transaction(tx_id), &tx);
        env.storage()
            .persistent()
            .set(&crate::DataKey::TransactionCount, &tx_id);

        tx_id
    }

    pub fn sign_transaction(env: Env, tx_id: u32, signer: Address) {
        signer.require_auth();

        let config: MultisigConfig = env
            .storage()
            .persistent()
            .get(&crate::DataKey::MultisigConfig)
            .expect("Contrato no inicializado");

        // Verificar que el firmante está autorizado
        let mut is_authorized = false;
        for i in 0..config.signers.len() {
            if config.signers.get(i).unwrap() == signer {
                is_authorized = true;
                break;
            }
        }
        assert!(is_authorized, "Firmante no autorizado");

        // Obtener transacción
        let mut tx: Transaction = env
            .storage()
            .persistent()
            .get(&crate::DataKey::Transaction(tx_id))
            .expect("Transacción no encontrada");

        assert!(!tx.executed, "Transacción ya ejecutada");

        // Verificar que no haya firmado ya
        let mut already_signed = false;
        for i in 0..tx.signatures.len() {
            if tx.signatures.get(i).unwrap() == signer {
                already_signed = true;
                break;
            }
        }
        assert!(!already_signed, "Ya has firmado esta transacción");

        // Añadir firma
        tx.signatures.push_back(signer.clone());

        // Si se alcanzan las firmas requeridas, ejecutar
        if tx.signatures.len() >= config.required_signatures {
            crate::resource::Resource::execute_transfer(
                &env,
                tx.resource_id.clone(),
                tx.from.clone(),
                tx.to.clone(),
                tx.quantity,
            );
            tx.executed = true;
        }

        env.storage()
            .persistent()
            .set(&crate::DataKey::Transaction(tx_id), &tx);
    }

    pub fn get_transaction(env: Env, tx_id: u32) -> Transaction {
        env.storage()
            .persistent()
            .get(&crate::DataKey::Transaction(tx_id))
            .expect("Transacción no encontrada")
    }

    pub fn get_config(env: Env) -> MultisigConfig {
        env.storage()
            .persistent()
            .get(&crate::DataKey::MultisigConfig)
            .expect("Contrato no inicializado")
    }
}
