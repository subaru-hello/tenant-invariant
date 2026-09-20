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

## Setup

You need Git and the stable Rust toolchain. If Rust is not installed, use [`rustup`, the installer recommended by the Rust project](https://www.rust-lang.org/tools/install):

```bash
# macOS, Linux, or WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download and run `rustup-init.exe` from the same official installation page. Restart the terminal after installation if `cargo` is not found, then verify the tools:

```bash
rustc --version
cargo --version
```

Clone this repository:

```bash
git clone https://github.com/subaru-hello/tenant-boundary-kit.git
cd tenant-boundary-kit
```

No database, API key, model provider, or external service is required for the included example.

## Terminal walkthrough

Run the cross-tenant example:

```console
$ cargo run --example contract_lookup
   Compiling tenant-boundary-kit v0.1.0
    Finished `dev` profile
     Running `target/debug/examples/contract_lookup`
blocked: CrossTenant
```

This represents an authenticated actor from `tenant-a` asking for a contract that the server resolves as belonging to `tenant-b`. The proposed tool call is blocked before the contract is returned.

Run the complete test suite:

```console
$ cargo test
running 4 tests
....
test result: ok. 4 passed; 0 failed

running 7 tests
.......
test result: ok. 7 passed; 0 failed
```

The output is abbreviated; the current suite contains 11 tests in total. To see only the named integration scenarios, run:

```bash
cargo test --test scenarios
```

For contributor checks matching CI, run:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

## Add it to a Rust application

The crate is not on crates.io yet. Add the Git repository under `[dependencies]` in your application's `Cargo.toml`:

```toml
[dependencies]
tenant-boundary-kit = { git = "https://github.com/subaru-hello/tenant-boundary-kit" }
```

Then run `cargo check` in your application. Cargo records the selected Git commit in `Cargo.lock`.

Call the check after resolving ownership on the server and immediately before the protected operation:

```rust
use tenant_boundary_kit::{check_tenant, Actor, Decision, ResourceOwner, TenantId};

fn main() {
    let actor = Actor {
        // Obtain this from authenticated application context, never model output.
        tenant: TenantId::new("tenant-a").expect("tenant ID must not be empty"),
    };

    // Resolve this from your database or trusted resource service.
    let owner = ResourceOwner::Tenant(
        TenantId::new("tenant-b").expect("tenant ID must not be empty"),
    );

    match check_tenant(&actor, &owner) {
        Decision::Allow => {
            // Perform your existing permission check and tenant-scoped operation.
        }
        Decision::Deny(_) => {
            // Return a generic error externally; keep detailed reasons internal.
        }
    }
}
```

The required order is:

```text
1. authenticate actor
2. accept the resource ID proposed by the agent
3. resolve the real resource owner on the server
4. call check_tenant
5. check read/write/action permission
6. perform a tenant-scoped database or API operation
```

Do not copy `tenant_id` from model-generated arguments into `Actor` or `ResourceOwner`.

## Quick retry

After changing the example or library, rerun:

```bash
cargo run --example contract_lookup
cargo test
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
