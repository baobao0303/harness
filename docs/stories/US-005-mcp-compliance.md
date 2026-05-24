# US-005 JSON-RPC Compliance and Formatting Fixes for MCP Server

## Status

implemented

## Lane

normal

## Product Contract

Ensure the Harness MCP server correctly implements the standard JSON-RPC 2.0 protocol and does not fail on client initialisation due to incorrect notification handling or invalid error responses. Additionally, ensure the codebase complies with formatting checks to prevent CI build failures.

## Relevant Product Docs

- `docs/HARNESS.md`
- `src/mcp.rs`

## Acceptance Criteria

- Cargo formatting checks (`cargo fmt --check`) pass without any violations.
- JSON-RPC notifications (requests without an `id` field) are processed but do not return any output to stdout (per standard JSON-RPC 2.0).
- Unrecognized methods return standard JSON-RPC error format with error code `-32601` rather than nesting under the `result` field.
- The Harness MCP server initializes and lists tools successfully without causing "invalid request" or "failed to get tools" errors on the client.

## Design Notes

- Core Rust changes in `src/mcp.rs` to structure `JsonRpcResponse` and method execution logic to adhere to JSON-RPC 2.0.
- Execute `cargo fmt` to clean up syntax styling.

## Validation

| Layer | Expected proof |
| --- | --- |
| Unit | Format checks pass |
| Integration | MCP server can be successfully queried and parsed |
| E2E | |
| Platform | |
| Release | CI Build passes |

## Harness Delta

- Added robust JSON-RPC notification and error handling support to ensure stable communication between AI agents and the Harness MCP daemon.

## Evidence

- **Cargo Format Verification:** Formatted `src/mcp.rs` and ran `cargo fmt --check` verifying zero formatting violations.
- **Cargo Unit Tests:** Ran `cargo test` verifying all 10 tests passed successfully.
- **Release Compilation Verification:** Successfully built the release package using `scripts/build-harness-cli-release.sh` verifying all compiler rules and code compatibility.
- **MCP Server Protocol Fixes:** Upgraded the JSON-RPC message pump to process but ignore notifications (no response printed to stdout) and return standard JSON-RPC 2.0 error objects (code `-32601` for unrecognized methods) instead of wrapping inside success results. This guarantees successful client handshake and initialization.
