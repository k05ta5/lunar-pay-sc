# Payments Proxy 
Smart contract for Bulk Transfers

# Contract build
```bash
mxpy --verbose contract build "bulk-transfers"
```

# Deploying the smart contract

```bash
mxpy --verbose contract deploy --recall-nonce --pem="wallet.pem" --gas-limit=30000000 --proxy="https://testnet-gateway.elrond.com" --chain=T --project=bulk-transfers --send || return
```

> **Note**
> Make sure to replace the pem file location with your own. In case that you are deploying to an environment different than testnet, you would need to change the --proxy and the --chain parameters as well.

### Deploying the smart contract through erdpy.json
```json
{
    "configurations": {
        "default": {
            "proxy": "https://testnet-gateway.elrond.com",
            "chainID": "T"
        }
    },
    "contract":{
        "deploy":{
            "send": true,
            "verbose": true,
            "recall-nonce": true,
            "pem": "../path/to/wallet.pem",
            "outfile": "bulk-transfers.json",
            "bytecode": "output/bulk-transfers.wasm",
            "gas-limit": 30000000
        }
     }
}
```
Using this command, you will deploy your smart contract on testnet with the above specified "configurations"

```bash
mxpy contract deploy
```

# Upgrading the smart contract

```bash
mxpy --verbose contract upgrade __contract_address_here__ --recall-nonce --pem="wallet.pem" --gas-limit=30000000 --proxy="https://testnet-gateway.elrond.com" --chain=T --project=bulk-transfers --send || return
```

> **Note**
> Make sure to replace the contract address and the pem file location with your own. In case that you are deploying to an environment different than testnet, you would need to change the --proxy and the --chain parameters as well.