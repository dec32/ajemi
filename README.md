# Ajemi

Ajemi is an IME (input method editor) for Toki Pona. With proper font support, it allows you to type Sitelen Pona characters with ease. 

![](./doc/preview.gif)

## Install

1. Install RIME from its [offical site](https://rime.im/)
2. Copy following files:  
    - `ajemi.schema.yaml`
    - `ajemi.dict.yaml`

   into:
   
    - `%APPDATA%\Rime` if using Windows
    - `~/Library/Rime/` if using Mac
    - `~/.config/ibus/rime/` if using GNU/Linux


For more detail, please refer to [this Wiki page](https://github.com/rime/home/wiki/RimeWithSchemata#rime-%E4%B8%AD%E7%9A%84%E6%95%B8%E6%93%9A%E6%96%87%E4%BB%B6%E5%88%86%E4%BD%88%E5%8F%8A%E4%BD%9C%E7%94%A8).

## Usage

To type an ideograph, simply type its spelling, and press space to confirm. 

![](./doc/kijetesantakalu.gif)

Suggestions from pop-ups will help you type faster. Press space to accept the highlighted one. Press number keys to pick any of them.

![](./doc/kije.gif)
To type punctuators, type: 

- `.` for MIDDLE DOT (U+F199C)
- `:` for COLON (U+F199D)

To type control characters, type:

- `+` for STACKING JOINTER (U+F1995)
- `-` for SCALING JOINER (U+F1996)
- `[]` for LONG GLYPH (U+F1997 and U+F1998)
- `<>` for CARTOUCHE (U+F1990 and U+F1991)


JOINTERs are used to create composite glyphs. LONG GLYPH provides underscores that works well with certain ideographs (especially pi). CARTOUCHE are indicators for proper names. Here's a rough demonstration of their behavior using the phrase "pi toki pona":

![](./doc/control.png)

## Fonts

Ameji relies on UCSUR-compliant fonts to function. To download such fonts, please visit [this spreadsheet](https://docs.google.com/spreadsheets/d/1xwgTAxwgn4ZAc4DBnHte0cqta1aaxe112Wh1rv9w5Yk/htmlview?gid=1195574771).

The font used above is [Nishiki-teki](https://umihotaru.work/).