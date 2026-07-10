# xtask

Internal task runner for maintenance owned by the `shared` repository.

## `update-shared-revisions`

Updates pinned Cargo dependencies from `stayhydated/shared` to the head of
`master`. The shared GitHub Action invokes this command in downstream
repositories and then refreshes their Cargo lockfile.

```bash
cargo xtask update-shared-revisions --workspace-root /path/to/downstream
```
