# How to release

1. Set the new version in `Cargo.toml`, commit and push.
2. Tag and push the tag:

   ```shell
   git tag v1.x.y
   git push origin v1.x.y
   ```

3. GitHub Actions builds the binaries for Linux (x86_64, arm64), macOS
   (x86_64, arm64) and Windows (x86_64), and attaches them to a draft release.
   The tag must match the version in `Cargo.toml` or the build fails.
4. Review and publish the draft at
   <https://github.com/ivanizag/iz-cpm/releases>.
