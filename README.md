# Ajemi

Ajemi is an IME (input method) for Toki Pona. With proper font support, it allows you to type Sitelen Pona characters with ease. 

![](./doc/preview.gif)

## Install

1. Install [RIME](https://rime.im/)
2. Install the font [Nishiki-teki](https://umihotaru.work/nishiki-teki.zip)
3. Download the zipped [config files](https://codeload.github.com/dec32/Ajemi/zip/refs/heads/master)
4. Copy all four config files:

    - `ajemi.schema.yaml`
    - `ajemi.dict.yaml`
    - `weasel.custom.yaml`
    - `default.custom.yaml`

   into: `C:/User/{YOUR_ACCOUNT}/AppData/Roaming/Rime`

5. Restart your computer

## Usage

Use WIN + SPACE to switch IMEs. Make sure you are switched to RIME.

To type an ideograph, simply type its spelling, and press SPACE to confirm. 

![](./doc/kijetesantakalu.gif)

Suggestions from pop-ups will help you type faster. Press SPACE to accept the highlighted one. Press number keys to pick any of them.

![](./doc/kije.gif)

To type punctuators, type: 

- `.` for MIDDLE DOT (U+F199C)
- `:` for COLON (U+F199D)

To type control characters, type:

- `+` for STACKING JOINTER (U+F1995)
- `-` for SCALING JOINER (U+F1996)
- `[` for START OF LONG GLYPH (U+F1997)
- `]` for END OF LONG GLYPH (U+F1998)
- `<` for START OF CARTOUCHE (U+F1990)
- `>` for END OF CARTOUCHE (U+F1991)


JOINERs combine adjacent glyphs into a single glyph. LONG GLYPH control characters provide underscores that work well with certain ideographs (especially pi). CARTOUCHE control characters provide cartouches for proper names. Here's a rough demonstration of their behavior using the phrase "pi toki pona":

![](./doc/control.png)

## Uninstall

1. Run `C:\Program Files (x86)\Rime\weasel-{VERSION}\uninstall.exe`
2. Restart your computer

## Fonts

Ameji relies on UCSUR-compliant fonts to function. To find and install such fonts, please visit [this spreadsheet](https://docs.google.com/spreadsheets/d/1xwgTAxwgn4ZAc4DBnHte0cqta1aaxe112Wh1rv9w5Yk/htmlview?gid=1195574771).The preferred font is [Nishiki-teki](https://umihotaru.work/). If you want to use other fonts, you need to modify the content of `weasel.custom.yaml` accordingly.