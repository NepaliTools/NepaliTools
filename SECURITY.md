# Security policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a vulnerability

Do not open a public issue for a security problem.

Use GitHub's private reporting. Open the repository's Security tab and choose
Report a vulnerability. That creates a private advisory only the maintainers can
see.

Include:

- the version (Settings, then Version) and how you obtained the build;
- your Windows version and display scaling;
- what you expected and what happened;
- a minimal reproduction, and any log lines from
  `%APPDATA%\com.aaxyat.nepalitoolkit\app.log` with personal data removed.

## Scope

- The application, its build scripts (`tools/`), the committed date data, and the
  update manifest and signing setup.
- The app makes one network request, and only when the user presses Check for
  updates in Settings: it fetches the signed release manifest and, if the user
  installs, the signed installer. It collects no telemetry. Anything that changes
  that is in scope.
- The updater verifies the installer against the public key in
  `crates/host/tauri.conf.json`. A forged manifest without the matching private key
  is a vulnerability; report it.
- Vulnerabilities in upstream dependencies should be reported upstream as well.

## What to expect

This is a small project maintained in personal time. Reports are acknowledged as
soon as possible, usually within a few days, and fixes are released as a patch
version. There is no bug bounty.
