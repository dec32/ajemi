# Ajemi

Ajemi is an IME (input method) for Toki Pona. With proper font support, it allows you to type Sitelen Pona characters with ease. 

![](./doc/preview.gif)

## Install


Click link below to download the installer.

[![[DOWNLOAD]](https://img.shields.io/badge/DOWNLOAD-ajemi--installer__x64.exe-blue)](https://github.com/dec32/Ajemi/releases/latest/download/ajemi-installer_x64.exe)

## Setup

To type and see the Sitelen Pona characters, you need to:

1. Press <kbd>Win</kbd> + <kbd>Space</kbd> to switch to the input method.
2. Set the font of your editor to ***sitelen seli kiwen juniko*** that comes with the input method.

You can also use [***Fairfax HD***](https://www.kreativekorp.com/software/fonts/fairfaxhd/), [***Nishiki-Teki***](https://umihotaru.work/) or any other [UCSUR-compliant font](http://antetokipona.infinityfreeapp.com/font).

## Use

To type a glyph, simply type its spelling, and press <kbd>Space</kbd> to confirm. 

![](./doc/soweli.gif)

Pressing <kbd>Enter</kbd> releases the raw text instead[^toggle].

![](./doc/soweli-ascii.gif)


The candidate list can help you type faster. Press <kbd>Space</kbd> to select the highlighted candidate or press <kbd>1</kbd> ~ <kbd>5</kbd> to pick any one of them.

![](./doc/sow.gif)

You can also type multiple glyphs in a row. Long glyphs will be automatically inserted for you.

![](./doc/soweli-lon-ma-kasi.gif)

To type punctuators, type: 

- `.` for middle dot
- `:` for colon
- `"` for CJK corner brackets
- `[]` for proper name cartouche

Joiners compose adjacent glyphs into compound glyphs. Type:

- `-` for zero-width joiner
- `^` for stack joiner
- `*` for scale joiner

Long glyphs are created by extending certain glyphs with special control characters. In most cases you don't need to worry about them because the input method inserts them for you. But if you want more precise control over long glyphs, you can type: 

- `()` to extend glyphs forward
- `{}` to extend glyphs backward

Here's a rough demonstration of the behavior of the control characters:

|Spelling          |Glyph                                    |
|------------------|-----------------------------------------|
|`kala-lili`       |![](./doc/control-scaling.png)           |
|`kala*lili`       |![](./doc/control-scaling.png)           |
|`kala^lili`       |![](./doc/control-stacking.png)          |
|`pi (kala lili)`  |![](./doc/control-long-glyph.png)        |
|`{kala lili} kama`|![](./doc/control-reverse-long-glyph.png)|


## Customize

The dictionary files are stored in `%APPDATA%/Ajemi/dict`. Their names end with `.dict` suffix. The format follows these 3 rules:

1. Entries are written as `{spelling} {output_0} {output_1}...{output_n}`
2. Single-character outputs can be written in their [Unicode code points](https://www.kreativekorp.com/ucsur/charts/sitelen.html) with a leading `#`
3. Comments start with `//`

Here's a minimal example:

```
a      𓍼
akesi  𓆐
ala    𓂜
alasa  𓌔 𓌸
[      #1F58C
]      #1F58C
```

## Configure

Configure the appearance and behavior of the input method by editing `%APPDATA%/Ajemi/conf.toml`. Here's the default one for reference:

```Toml
[font]
name = "sitelen seli kiwen juniko"
size = 20

[layout]
vertical = false

[color]
candidate = 0x000000
index = 0xA0A0A0
background = 0xFAFAFA
clip = 0x0078D7
highlight = 0xE8E8FF
highlighted = 0x000000

[behavior]
toggle = "Ctrl"
long_pi = false
long_glyph = false
cjk_space = false
```

## Build from Source


Prerequisites:

1. Rust (with the MSVC toolchain)
2. Git Bash[^for-just-to-work]
3. Inno Setup (only for creating installers)[^inno-setup-path]

Run the following commands to setup the development envirorment:

```
cargo install just
just setup
```

To build the project and register the newly built IME for testing, run:

```
just build
```

When you're done testing, you can unregister the IME with:

```
just unreg
```

Create an installer for the project by running:

```
just pack
```

[^toggle]: Alternatively, press <kbd>Ctrl</kbd> to toggle off the input method temporarily. <kbd>CapsLock</kbd> and <kbd>英数</kbd> also functions as toggles if configured. 

[^for-just-to-work]: [Just](https://github.com/casey/just) does not utilize Powershell and relies on Git Bash to function on Windows. Make sure the `bin` folder of Git is added to your `PATH`.

[^inno-setup-path]: Make sure `iscc` is accessible from shell by editting `PATH`