# DirParser

Dead simple `tree` alternative written in rust that respects `.gitignore`.

It's just a toy project and a lot of things doesn't work :).

## Example

```
➜  dirparser git:(master) ✗ cargo run

    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/dirparser`
├── GITIGNORE.md
├── ./target
│   ├── .rustc_info.json
│   ├── CACHEDIR.TAG
├── Cargo.lock
├── README.md
├── .gitignore
├── .prettierrc
├── ./.vscode
│   ├── settings.json
├── ./src
│   ├── lib.rs
│   ├── main.rs
```

## Todo

- [ ] Sort the directory.
- [ ] Last file with `└`.
- [ ] Add a config.
- [ ] Read command line arguments.

## Resources

- https://doc.rust-lang.org/std/result/
- https://docs.rs/glob/latest/glob/
- https://doc.rust-lang.org/std/path/enum.Component.html
