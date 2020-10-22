# my-sudt-info

Save sudt info like decimal, token_name, symbol and other customer info, bind with sudt cell.

## Validator Rules

- sudt info cell's type script args must equal sudt cell's type hash
- sudt cell's type args(owner lock script hash) should be used in transaction inputs cells

## Tutorial

Build contracts:

``` sh
capsule build
```

Run tests:

``` sh
capsule test
```
