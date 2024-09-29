# EnvVar manager

Useful when you need to set the same variables all the time and update tokens or
YubiKeys.

## Usage

```bash
em --help
```

## Example

```bash
em env add TOKEN 123456
em rule add AUTH_TOKEN "Bearer {TOKEN}"

alias getenv='eval "$(em export)"'
getenv

echo "TOKEN=$TOKEN"
echo "AUTH_TOKEN=$AUTH_TOKEN"
```

## TODO

* Add 'secret' EnvVars that will be transformed to `****` in `em list`.
* Shorten long EnvVars in `em list` and add a `em get` to get the full value.
* Current transaction and connection handling is awful, should be moved out of the
  repositories, but I wanted to try to do something with lifetimes for now.
    * Probable solution: the `Db` struct should hold a connection pool. The commands
      should either request a transaction or a connection, which will be later passed
      to the repositories. The command is responsible for committing the transaction.
* I think the validation code and the resolver code could be improved, but I don't know
  how yet.
    * Actually, how are validators handled usually?
