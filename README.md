# Lunar Pay Protocol 
Smart contract for the Lunar Pay protocol

# Contract build
```bash
mxpy --verbose contract build
```

# Deploying the smart contract

> **Note**
> Make sure to replace the pem file location with your own. In case that you are deploying to an environment different than testnet, you would need to change the `proxy` and the `chainId` properties as well.

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
            "outfile": "lunar-pay.json",
            "bytecode": "output/lunar-pay.wasm",
            "gas-limit": 30000000
        }
     }
}
```
Using the following command, you will deploy the smart contract on testnet with the above specified configuration:

```bash
mxpy contract deploy
```

