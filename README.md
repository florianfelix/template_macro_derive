# Derive Macro Template


## bacon usage
```bash
bacon #start default (integration tests)
```
### bacon Shortcuts
- i  integration tests
- e  expand
- o  toggle-raw-output


### expand
for syntax highlighting it was necessary for me to set
```toml
# ~/.cargo.config.toml
[expand]
color = "always"
```

## if added as subcrate
copy the tests to the parent cargo.toml and adjust the paths to match

## links

### proc macro workshop
https://github.com/dtolnay/proc-macro-workshop
https://www.youtube.com/watch?v=geovSK3wMB8

### good short introduction
https://petanode.com/posts/rust-proc-macro/
