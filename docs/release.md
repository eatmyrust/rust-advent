# Release Workflow

1. Bump the version of the program in Cargo.toml
- `cargo release version $(convco version --bump) --execute`

2. Add and commit files that were changed by previous command
- `git add . && convco commit --chore -m "Release"`

3. Tag the new commit with the new application version
- `cargo release tag --execute`

4. Push all changes to remote
- `cargo release push --execute`

5. Submit release workflow to build and publish new version of the program
- `gh workflow run release.yml`