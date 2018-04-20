fm_tree
---

CLI util to format YAML/JSON file to File System tree representation.
Because indentation is hard! :)

Example YAML input:

```yaml
- src:
    - fs_tree.rs
    - main.rs
- target:
    - debug:
        - native
        - fm_tree
        - .cargo-lock
        - fm_tree.d
    - rls:
        - debug
- .gitignore
- Cargo.lock
- Cargo.toml
```

Example output:

```
.
├── src
│   ├── fs_tree.rs
│   └── main.rs
├── target
│   ├── debug
│   │   ├── native
│   │   ├── fm_tree
│   │   ├── .cargo-lock
│   │   └── fm_tree.d
│   └── rls
│       └── debug
├── .gitignore
├── Cargo.lock
└── Cargo.toml
```
