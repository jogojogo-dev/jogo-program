# Jogo Solana Program

## Deploy
```shell
anchor keys sync
anchor build -p=jogo_program
anchor deploy -p=jogo_program --program-keypair=target/deploy/jogo_program-keypair.json
anchor upgrade -p BaT67HoTDB1YM618xtzFvAJerFMqfvA4uwhe6jnFhPu3 target/deploy/jogo_program.so
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