# xtask

`xtask` is the internal task runner for maintenance owned by the `shared`
repository.

## Example

Run `update-shared-revisions` from the repository root to update a consumer's
pinned Cargo dependencies from `stayhydated/shared` to the head of `master` and
refresh the affected downstream lockfile packages:

```bash
cargo xtask update-shared-revisions --workspace-root /path/to/downstream
```

The shared GitHub Action invokes this command in downstream repositories. The
command edits matching Cargo manifests and runs a targeted Cargo update when
revisions change. Review the downstream manifest and lockfile diff before
committing it. Reusable-workflow revision pins are maintained separately.
