use tenant_invariant::{check_tenant, Actor, Decision, ResourceOwner, TenantId};

struct Contract {
    id: &'static str,
    tenant: &'static str,
    title: &'static str,
}

fn main() {
    // In a real service, this must come from an authenticated session.
    let actor = Actor {
        tenant: TenantId::new("tenant-a").unwrap(),
    };

    // The agent can propose an ID, but cannot provide its owner.
    let proposed_contract_id = "contract-b";
    let contracts = [Contract {
        id: "contract-b",
        tenant: "tenant-b",
        title: "Private contract",
    }];

    // Demo only: resolve ownership server-side. Production data access must
    // also enforce tenant scoping in the same query/operation.
    let contract = contracts.iter().find(|c| c.id == proposed_contract_id);
    let owner = contract
        .map(|c| ResourceOwner::Tenant(TenantId::new(c.tenant).unwrap()))
        .unwrap_or(ResourceOwner::Unknown);

    match check_tenant(&actor, &owner) {
        Decision::Allow => println!("{}", contract.unwrap().title),
        Decision::Deny(reason) => println!("blocked: {reason:?}"),
    }
}
