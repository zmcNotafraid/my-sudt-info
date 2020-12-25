# my-sudt-info

Save sudt info like decimal, token_name, symbol and other customer info, bind with sudt cell.Design document is [here](https://talk.nervos.org/t/a-sudt-information-storage-meta-cell-design-proposal/5011)

## Validator Rules

- SUDT Info Cell's type script's args save account's lock hash, and this lock script must exist in input cells.
- SUDT Info Cell's data now only save decimal, token name and symbol.We don't validate data format in contract.

## Tutorial

Build contracts:

``` sh
capsule build
```

Run tests:

``` sh
capsule test
```

## TestNet Depoly Info
|parameter| value|
|-----|-----|
|`code_hash`|`0x72f3d72944f29511eedf806d4b12d77ca0a5cfbb2000d059d8898d283971b579`|
|`hash_type`|`type`|
|`tx_hash`|`0x81eeaaedc2909faf471cc17f8aeb66dd5e78d50ad1b7eb56e41ab821ee356330`|
|`index`|`0x0`|
|`dep_type`|`code`|

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
      lock: 256k1/blake160 lock script
      type:
        args: **ownder lock script**
        code_hash: **sudt code hash**
        hash_type: data
    //sudt info cell
    - capacity: "350.0"
      lock: 256k1/blake160 lock script
      type:
        args: **ownder lock script hash**
        code_hash: **sudt info code hash**
        hash_type: type
    //cost fee cell
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
