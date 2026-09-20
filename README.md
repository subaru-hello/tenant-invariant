# tenant-boundary-kit

[![CI](https://github.com/subaru-hello/tenant-boundary-kit/actions/workflows/ci.yml/badge.svg)](https://github.com/subaru-hello/tenant-boundary-kit/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**What if an AI agent chooses a resource ID that belongs to another customer?**

`tenant-boundary-kit` is an experimental Rust library for making that boundary explicit before a tool call executes. The application supplies an authenticated actor and server-resolved resource ownership. The library allows same-tenant access and fails closed for cross-tenant or unknown ownership.

It does not trust tenant IDs produced by the model.

```text
agent proposes resource ID
  -> server resolves the resource owner from a trusted source
  -> tenant-boundary-kit compares actor and owner
  -> existing authorization and tenant-scoped data operation
```

## Try it in 30 seconds

```bash
git clone https://github.com/subaru-hello/tenant-boundary-kit.git
cd tenant-boundary-kit
cargo run --example contract_lookup
```

The example represents an actor from `tenant-a` asking for a contract owned by `tenant-b`:

```text
blocked: CrossTenant
```

Run all tests with `cargo test`.

## Use the library

Until the crate is published, depend on the Git repository:

```toml
[dependencies]
tenant-boundary-kit = { git = "https://github.com/subaru-hello/tenant-boundary-kit" }
```

Call the check after resolving ownership on the server, immediately before the protected operation:

```rust
use tenant_boundary_kit::{check_tenant, Actor, Decision, ResourceOwner, TenantId};

let actor = Actor {
    // Obtain this from authenticated application context, never model output.
    tenant: TenantId::new("tenant-a")?,
};

// Resolve this from your database or trusted resource service.
let owner = ResourceOwner::Tenant(TenantId::new("tenant-b")?);

match check_tenant(&actor, &owner) {
    Decision::Allow => {
        // Perform your existing permission check and tenant-scoped operation.
    }
    Decision::Deny(_) => {
        // Return a generic error externally; keep detailed reasons internal.
    }
}
# Ok::<(), tenant_boundary_kit::InputError>(())
```

## Scenarios covered

The test suite exercises:

- same-tenant and cross-tenant contract reads;
- unknown invoice ownership;
- a forged tenant claim in agent-generated arguments;
- a batch containing resources from multiple tenants;
- randomly generated cross-tenant pairs.

Run the integration scenarios separately with `cargo test --test scenarios`.

## Security boundary and limitations

This is a small experimental building block, **not a complete authorization system or a security guarantee**.

It checks tenant equality only. The integrating application must still:

- authenticate the actor;
- authorize the requested action, such as read versus write;
- resolve ownership from a trusted server-side source;
- enforce tenant scoping in the database or downstream API;
- avoid a separate check followed by an unscoped fetch, which can race or bypass the decision;
- avoid logging raw prompts and tool arguments by default.

The tests intentionally document two gaps: same-tenant writes require a separate permission check, and ownership can change between a separate lookup and fetch. Those tests demonstrate the boundary of this library; they do not make those operations safe.

## Project status

This project is looking for feedback from engineers building multi-tenant SaaS, MCP servers, and agent tool runtimes. Useful questions and real integration examples are welcome in [GitHub Issues](https://github.com/subaru-hello/tenant-boundary-kit/issues).

Security concerns should be reported as described in [SECURITY.md](SECURITY.md).

## License

MIT
