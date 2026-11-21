# Modules are compiled into the host, not dynamically loaded

Status: accepted

Vorssaint's model is install-and-uninstall tools, and a reader will assume the same mechanism here. It is not. Every module is a Rust type registered in the Host behind one trait, gated by an `enabled` setting, and listed on a Modules page. Disabling a module means it is not initialised and not rendered, but the code is still in the binary. One process, one webview, no C ABI, no plugin version negotiation, no per-module packaging.

## Consequences

- The binary carries every module it was built with, including disabled ones.
- "Uninstall" is a UI and settings state, not a filesystem fact. No installer or file cleanup is implied.
- Third-party modules would need a new decision; the trait is not a public ABI and must not be treated as one.
