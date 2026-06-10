
```sh
cross build --target=armv7-unknown-linux-gnueabihf --offline --release; scp target/armv7-unknown-linux-gnueabihf/release/pm_tester root@target:/root
```

Скопировать файл автозапуска:

```sh
scp S99_pm_tester.sh root@target:/etc/init.d/
```

На целевой системе
```sh
chmod +x /etc/init.d/S99_pm_tester.sh
```
