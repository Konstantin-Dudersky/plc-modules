
Сборка исполняемого файла

```sh
cargo build --release; cp target/release/inventree_bom ~/.cargo/bin/
```

## Настройка Kicad

Сгенерировать токен в Inventree для доступа к API

В Kicad "Настройки" -> "Настроить пути..." добавить переменную `INVENTREE_TOKEN` со значением токена (`inv-...`)

Добавить задачу в jobset:

```sh
/home/konstantin/.cargo/bin/inventree_bom \
-i ./export/v${version}/${article}.BOM.csv \
-o ./export/v${version}/inventree_bom \
-t ${INVENTREE_TOKEN}
```
