# Jogo Solana Program

## Deploy
```shell
anchor build -p=jogo_program
deploy_jogo = "anchor deploy --program-name=jogo_program --program-keypair=target/deploy/jogo_program-keypair.json"
```

## Note
需要在idl文件中手动添加metadata，如下：
```json
{
  "version": "0.1.0",
  "name": "jogo_program",
  "metadata": {
    "address": "FHcfAFsrp1Y4i1U4RCoDSbbRBCtvnpkcyvVYgc5J5jc4"
  }
}
```