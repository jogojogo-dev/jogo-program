# Jogo Solana Program

## Deploy
```shell
anchor keys sync
anchor build -p=jogo_program
anchor deploy -p=jogo_program --program-keypair=target/deploy/jogo_program-keypair.json
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