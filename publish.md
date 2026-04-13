# Publish checklist

- Update dist: `dist init` and `dist generate --allow-dirty >> .github/workflows/release.yml`
- Updated version in Cargo.toml
- Build for lockfile update
- Commit and push change made to Cargo.toml and cargo.lock
- Add git tag with version ```git tag v0.0.0``` (replace with actual version)
- Push git tag when finished ```git push origin --tags```
- cargo publish
