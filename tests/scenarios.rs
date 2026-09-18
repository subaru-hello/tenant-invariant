use tenant_boundary_kit::{check_tenant, Actor, Decision, DenyReason, ResourceOwner, TenantId};

fn actor(tenant: &str) -> Actor {
    Actor {
        tenant: TenantId::new(tenant).unwrap(),
    }
}

fn owned_by(tenant: &str) -> ResourceOwner {
    ResourceOwner::Tenant(TenantId::new(tenant).unwrap())
}

#[test]
fn support_agent_reads_own_tenant_contract() {
    assert_eq!(
        check_tenant(&actor("acme"), &owned_by("acme")),
        Decision::Allow
    );
}

#[test]
fn support_agent_cannot_read_another_tenants_contract() {
    assert_eq!(
        check_tenant(&actor("acme"), &owned_by("globex")),
        Decision::Deny(DenyReason::CrossTenant)
    );
}

#[test]
fn unknown_invoice_owner_is_denied() {
    assert_eq!(
        check_tenant(&actor("acme"), &ResourceOwner::Unknown),
        Decision::Deny(DenyReason::UnknownOwner)
    );
}

#[test]
fn tenant_claim_in_agent_arguments_cannot_override_trusted_actor() {
    let _agent_argument_tenant = "globex";
    let authenticated_actor = actor("acme");
    let server_resolved_owner = owned_by("globex");
    assert_eq!(
        check_tenant(&authenticated_actor, &server_resolved_owner),
        Decision::Deny(DenyReason::CrossTenant)
    );
}

#[test]
fn mixed_tenant_batch_requires_every_item_to_pass() {
    let actor = actor("acme");
    let selected_invoices = [owned_by("acme"), owned_by("globex")];
    assert!(!selected_invoices
        .iter()
        .all(|owner| check_tenant(&actor, owner) == Decision::Allow));
}

#[test]
fn same_tenant_write_needs_a_separate_permission_check() {
    // The library has no action or role input. This is ALLOW even when the
    // caller intends to mutate a record; the host must check write permission.
    let read_only_agent = actor("acme");
    assert_eq!(
        check_tenant(&read_only_agent, &owned_by("acme")),
        Decision::Allow
    );
}

#[test]
fn stale_owner_lookup_needs_backend_tenant_scoping() {
    // If ownership changes after this check, a later unscoped fetch is unsafe.
    // The library cannot detect a race between separate operations.
    let owner_at_check_time = owned_by("acme");
    let owner_at_fetch_time = owned_by("globex");
    assert_eq!(
        check_tenant(&actor("acme"), &owner_at_check_time),
        Decision::Allow
    );
    assert_eq!(
        check_tenant(&actor("acme"), &owner_at_fetch_time),
        Decision::Deny(DenyReason::CrossTenant)
    );
}
