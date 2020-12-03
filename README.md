# my-sudt-info

Save sudt info like decimal, token_name, symbol and other customer info, bind with sudt cell.Design document is [here](https://talk.nervos.org/t/a-sudt-information-storage-meta-cell-design-proposal/5011)

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

## Transaction Struct

```
transaction:
  cell_deps:
    - dep_type: code
      out_point:
        index: 0
        tx_hash: **sudt tx hash**
    - dep_type: code
      out_point:
        index: 0
        tx_hash: **sudt info tx hash**
  hash:
  header_deps: []
  inputs:
    - previous_output:
  outputs:
    // sudt cell
    - capacity: "142.0"
      lock:
      type:
        args: **ownder lock script**
        code_hash: **sudt code hash**
        hash_type: data
    //sudt info cell
    - capacity: "350.0"
      lock:
        args: **sudt type script hash**
        code_hash: **sudt info code hash**
        hash_type: type
      type:
        args: **sudt type script hash**
        code_hash: **sudt info code hash**
        hash_type: data
    - capacity: "199507.9"
      lock:
      type:
  outputs_data:
    - **sudt amount**
    - **sudt info data**
    - 0x
  version: 0
  witnesses:
```
