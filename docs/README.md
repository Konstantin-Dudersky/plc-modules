```bash
typst compile --font-path ./fonts main.typ  ./output/plc-modules.pdf

typst watch --font-path ./fonts main.typ  ./output/plc-modules.pdf

typst compile --font-path ./fonts main.typ --features html --format html ./output/html
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
