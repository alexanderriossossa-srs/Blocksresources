use soroban_sdk::{contracttype, Address, Env, Symbol};

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

pub struct ResourceManager;

impl ResourceManager {
    pub fn register(
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

        env.storage()
            .persistent()
            .set(&crate::DataKey::Resource(id), &resource);
    }

    pub fn get(env: Env, id: Symbol) -> Resource {
        env.storage()
            .persistent()
            .get(&crate::DataKey::Resource(id))
            .expect("Recurso no encontrado")
    }

    pub fn execute_transfer(
        env: &Env,
        resource_id: Symbol,
        from: Address,
        to: Address,
        quantity: u32,
    ) {
        // Reducir cantidad del recurso original
        let mut resource: Resource = env
            .storage()
            .persistent()
            .get(&crate::DataKey::Resource(resource_id.clone()))
            .expect("Recurso no encontrado");

        resource.quantity -= quantity;
        env.storage()
            .persistent()
            .set(&crate::DataKey::Resource(resource_id.clone()), &resource);

        // Crear nuevo recurso para el destinatario
        let new_resource_id = Symbol::new(env, "NEW_RES");
        let new_resource = Resource {
            id: new_resource_id.clone(),
            name: resource.name,
            resource_type: resource.resource_type,
            quantity,
            origin: resource.origin,
            carbon_footprint: resource.carbon_footprint,
            owner: to,
            timestamp: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&crate::DataKey::Resource(new_resource_id), &new_resource);
    }
}
