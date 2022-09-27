# DirParser

Dead simple `tree` alternative written in rust that respects `.gitignore`.

It's just a toy project and a lot of things don't work :).

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

## Obscure .gitignore behaviour.

RN, it doesn't support all the specs of gitignore as listed in [GITIGNORE.md](./GITIGNORE.md) and because of the differnce if behaviour of gitignore specs and glob it need a lot of adjustments.

`/target/**` will list all the target sub directories but won't list the target directory's content for that you need to use `/target/*`.

`!` is not supported yet.

## Resources

- https://doc.rust-lang.org/std/result/
- https://docs.rs/glob/latest/glob/
- https://doc.rust-lang.org/std/path/enum.Component.html
