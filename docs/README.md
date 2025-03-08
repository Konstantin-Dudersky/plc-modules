```bash
typst compile --font-path ./fonts --root . ./src/main.typ  ./output/plc-modules.pdf

typst compile --font-path ./docs/fonts docs/src/main.typ --root . --features html --format html ./docs/output/html
```

Схема внешних подключений:
- Выходной формат - SVG
- Каталог назначения - doc/

Размеры печатной платы:

- Формат черчения: SVG
- Включить слои:
  - User.Comments
- Чертить на всех слоях:
  - Edge.Cuts
- Каталог назначения - doc/
