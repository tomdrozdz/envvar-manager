# EnvVar manager

For when you need to set the same variables all the time, concatenate some of them
together, share them between different shells, and update tokens or YubiKeys.

Small warning: I made this only because I wanted to learn Rust. This is not good code.
It is probably considered insecure, but I'm actually using it and it is quite useful.

## Usage

```bash
em --help
```

## Example

```bash
em env add TOKEN 123456 --secret
em template add AUTH_TOKEN "Bearer {TOKEN}"

alias getenv='eval "$(em export)"'
getenv

echo "TOKEN=$TOKEN"
echo "AUTH_TOKEN=$AUTH_TOKEN"
```

## TODO

* Tests.
* Add a `em get` to get the full value of a single variable/template.
* Could I refactor the commands not to be hardcoded to SQLite? Could the database
  backend be more easily swapped at compile time?
* I think the validation code and the resolver code could be improved, but I don't know
  how yet.
    * Actually, how are validators handled usually?
* "List" commands do not need a transaction, but on the other hand it doesn't matter.
