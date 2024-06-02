# Jogo Solana Program

## Build
```shell
anchor keys sync
anchor build -p=jogo_program
```

### Deploy
```shell
solana program deploy \
    -k=.keypairs/deployer.json \
    -u=mainnet-beta \
    --with-compute-unit-price=<0.00005> \
    --buffer=<YOUR_BUFFER_ACCOUNT> \
    --use-quic \
    --commitment=processed \
    --program-id=target/deploy/vault_program-keypair.json \
    target/deploy/vault_program.so
```

## Upgrade
```shell
anchor upgrade -p <PROGRAM_ID> target/deploy/jogo_program.so
```

## Note
需要在idl文件中手动添加metadata，如下：
```json
{
  "version": "0.1.0",
  "name": "jogo_program",
  "metadata": {
    "address": "<PROGRAM_ID>"
  }
}
```