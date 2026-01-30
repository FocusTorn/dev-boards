# Library Versioning & Stability Policy

- **Prefer LTS:** Always suggest "Long-Term Support" (LTS) or stable versions of crates/packages.
- **Consult Manifests:** Before suggesting a new dependency, read `Cargo.toml` or `pyproject.toml` to see established versions.
- **Prohibit Bleeding Edge:** NEVER suggest "alpha", "beta", "rc" (release candidate), or versions released within the last 30 days unless explicitly asked.
- **Verify with Crates.io/PyPI:** If unsure of stability, use `run_shell_command` to check the current stable version (e.g., `cargo search` or `pip index versions`).
