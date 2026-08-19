# xtask

Internal task runner for maintenance owned by the `shared` repository.

## `update-shared-revisions`

Updates pinned Cargo dependencies from `stayhydated/shared` to the head of
`master`, then narrowly refreshes the affected packages in the downstream Cargo
lockfile. The shared GitHub Action invokes this command in downstream
repositories.

```bash
cargo xtask update-shared-revisions --workspace-root /path/to/downstream
```
