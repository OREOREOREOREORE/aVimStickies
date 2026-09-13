# Security

aVimStickies is built so that what you run can be verified.

## Open source

The full source lives at <https://github.com/OREOREOREOREORE/aVimStickies>. Every line is auditable, and releases are built by GitHub Actions from tagged commits (see `.github/workflows/release.yml`).

## Downloads

The app is distributed only through this repository's GitHub Releases over HTTPS. Nothing is fetched from third-party hosts.

## Checksum verification

Every release publishes a `SHA256SUMS` file. `install.sh` downloads it and verifies the archive before installing; a mismatch aborts the install. To verify manually:

```sh
shasum -a 256 aVimStickies_aarch64.app.tar.gz   # compare against SHA256SUMS
shasum -a 256 -c SHA256SUMS                     # or check a full checkout
```

## Signed updates

The in-app updater reads this repository's `latest.json`. Each update archive is signed with an ed25519 key, and the app verifies that signature against its embedded public key before applying the update. A signature that does not match is rejected.

## No telemetry

aVimStickies collects no analytics and sends no telemetry. Its only network request is the update check against GitHub Releases.

## Minimal surface

The app is written in Rust and renders with the system WebKit webview. It uses a small set of well-known crates and runs no background services.

## Code signing status

aVimStickies is currently **not signed with an Apple Developer ID** (the project does not pay for an Apple Developer account). Consequences:

- Installing via `curl` does not set macOS's quarantine attribute, so the app launches normally.
- If you install a copy macOS has quarantined (for example, downloaded through a browser), Gatekeeper warns on first launch — right-click the app and choose **Open** once.

A signed and notarized build would remove this caveat; if you need one, please open an issue.

## Reporting a vulnerability

Please open a private security advisory, or contact the maintainer through the repository.
