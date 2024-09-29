# EnvVar manager

Useful when you need to set the same variables all the time and update tokens or
YubiKeys.

## Usage

```bash
em --help
```

## Example

```bash
em add TOKEN 123456
em rule add AUTH_TOKEN "Bearer {TOKEN}"

alias getenv='eval "$(em export)"'
getenv

echo "TOKEN=$TOKEN"
echo "AUTH_TOKEN=$AUTH_TOKEN"
```

## TODO

* Generate CLI completions.
* Current transaction handling is awful, should be moved out of the repositories.
    * Possible idea: return a `Transaction` from a method, then pass them to other
      repository methods, commit at the end of a command.
* I have no idea if the way I'm handling the database connection is idiomatic, but
  I wanted to use lifetimes somewhere.
* I think the validation code and the resolver code could be improved, but I don't know
  how yet.
    * Actually, how are validators handled usually?
