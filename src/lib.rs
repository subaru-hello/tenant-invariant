//! A small, fail-closed tenant boundary check for agent tool calls.
//!
//! The host application must authenticate the actor and resolve resource
//! ownership from a trusted source. Model-generated tool arguments are not
//! an authority for either value.

/// A tenant identifier obtained from the host application's trusted context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TenantId(String);

impl TenantId {
    pub fn new(value: impl Into<String>) -> Result<Self, InputError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(InputError::EmptyTenantId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Authenticated actor context. Do not construct this from model output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Actor {
    pub tenant: TenantId,
}

/// Ownership resolved by the server, not supplied by the agent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResourceOwner {
    Tenant(TenantId),
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DenyReason {
    CrossTenant,
    UnknownOwner,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    Allow,
    Deny(DenyReason),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputError {
    EmptyTenantId,
}

/// Checks only the tenant boundary. It does not replace role/permission checks
/// or the resource service's own authorization.
pub fn check_tenant(actor: &Actor, owner: &ResourceOwner) -> Decision {
    match owner {
        ResourceOwner::Unknown => Decision::Deny(DenyReason::UnknownOwner),
        ResourceOwner::Tenant(tenant) if tenant == &actor.tenant => Decision::Allow,
        ResourceOwner::Tenant(_) => Decision::Deny(DenyReason::CrossTenant),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn tenant(value: &str) -> TenantId {
        TenantId::new(value).unwrap()
    }

    #[test]
    fn same_tenant_is_allowed() {
        let actor = Actor {
            tenant: tenant("a"),
        };
        assert_eq!(
            check_tenant(&actor, &ResourceOwner::Tenant(tenant("a"))),
            Decision::Allow
        );
    }

    #[test]
    fn missing_owner_is_denied() {
        let actor = Actor {
            tenant: tenant("a"),
        };
        assert_eq!(
            check_tenant(&actor, &ResourceOwner::Unknown),
            Decision::Deny(DenyReason::UnknownOwner)
        );
    }

    #[test]
    fn empty_tenant_id_is_rejected() {
        assert_eq!(TenantId::new("  "), Err(InputError::EmptyTenantId));
    }

    proptest! {
        #[test]
        fn different_tenants_are_always_denied(a in "[a-z0-9]{1,12}", b in "[a-z0-9]{1,12}") {
            prop_assume!(a != b);
            let actor = Actor { tenant: tenant(&a) };
            let owner = ResourceOwner::Tenant(tenant(&b));
            prop_assert_eq!(check_tenant(&actor, &owner), Decision::Deny(DenyReason::CrossTenant));
        }
    }
}
