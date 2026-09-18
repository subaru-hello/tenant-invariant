# tenant-boundary-kit

A minimal Rust library for checking a tenant boundary before an AI agent's tool call. It is an experimental starting point, **not** a complete authorization system.

The host application supplies two independently trusted facts: the tenant from an authenticated actor context, and the resource owner resolved by the server. The agent may propose a resource ID, but its text and tool arguments must not be used as the source of either trusted fact. An unknown owner is denied.

```text
agent proposes resource ID
  -> server resolves resource owner
  -> check_tenant(authenticated actor, resource owner)
  -> existing authorization and tenant-scoped data operation
```

Run the cross-tenant example with `cargo run --example contract_lookup`. It prints `blocked: CrossTenant`. Run the tests with `cargo test`.

Run `cargo test --test scenarios` for seven usage scenarios: own-tenant and cross-tenant reads, unknown invoice ownership, a forged tenant claim in agent arguments, a mixed-tenant batch, and two documented gaps (write permission and ownership changing between check and fetch). The tests of those gaps pass because they demonstrate what this library does **not** enforce, not because the operations are safe.

The current API checks only tenant equality. The caller must still enforce role and action permissions, authenticate the actor, and scope the actual database/API operation to the tenant. A separate ownership lookup followed by an unscoped fetch can race or bypass the check; use a tenant-scoped query or equivalent backend enforcement. Do not log raw prompts or tool arguments by default.

The included property test generates different tenant IDs and checks that every cross-tenant pair is denied. It does not prove the security of an integrating application.
