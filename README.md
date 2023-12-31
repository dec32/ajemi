# Ajemi

Ajemi is an IME (input method editor) for Toki Pona. With proper font support, it allows you to type Sitelen Pona characters with ease. 

![](./doc/preview.gif)

## Install

1. Install the font [Nishiki-teki](https://umihotaru.work/nishiki-teki.zip)
2. Install RIME from its [offical site](https://rime.im/)
3. Download the zipped [config files](https://codeload.github.com/dec32/Ajemi/zip/refs/heads/master)
4. Copy all four config files:

    - `ajemi.schema.yaml`
    - `ajemi.dict.yaml`
    - `weasel.custom.yaml`
    - `default.custom.yaml`

   into: `C:/User/{YOUR_ACCOUNT}/AppData/Roaming/Rime`

5. Restart your computer

## Usage

Use Windows+SPACE to switch IME. Make sure you are switched to RIME.

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

## Uninstall

1. Run `C:\Program Files (x86)\Rime\weasel-VERSION\uninstall.exe`
2. Restart your computer

## Fonts

Ameji relies on UCSUR-compliant fonts to function. To download such fonts, please visit [this spreadsheet](https://docs.google.com/spreadsheets/d/1xwgTAxwgn4ZAc4DBnHte0cqta1aaxe112Wh1rv9w5Yk/htmlview?gid=1195574771).

The preferred font is [Nishiki-teki](https://umihotaru.work/). If you want to use other fonts, you need to modify the content of `weasel.custom.yaml` accordingly.
