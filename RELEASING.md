# Releasing dexrs

Publishing a non-prerelease GitHub release runs `.github/workflows/release.yaml`.
The workflow checks that the release commit is on `main` and its tag is exactly
`v` followed by the version in `Cargo.toml`. It runs tests and a package dry run
before publishing to crates.io. Pushes, pull requests, draft releases, and
GitHub prereleases do not publish a crate.

## One-time setup

A crate owner must add a GitHub Trusted Publisher in the
[dexrs crate settings](https://crates.io/crates/dexrs/settings):

- Repository owner: `makors`
- Repository name: `dexrs`
- Workflow filename: `release.yaml`
- Environment: leave unset (this workflow does not use a GitHub environment)

The workflow uses GitHub OIDC and a temporary crates.io token. No GitHub secret
containing a crates.io API token is needed. See the
[crates.io Trusted Publishing documentation](https://crates.io/docs/trusted-publishing).

## Release a version

1. Merge the changes to release, including any required bug fixes.
2. Update `package.version` in `Cargo.toml` and the `dexrs` package version in
   `Cargo.lock` to a new, unpublished version. Commit and merge both files.
3. Wait for the existing build/test CI to pass on `main`.
4. Create a GitHub release targeting that commit, with a matching tag such as
   `v0.1.2`. Add release notes and publish it without marking it as a prerelease.
5. Check the **Publish crate** workflow and confirm the new version on crates.io.

The workflow does not bump versions or create releases. A crates.io version
cannot be overwritten. If publishing fails, inspect the logs and the registry
before retrying; use a new version if that version was already published.
