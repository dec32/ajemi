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

You can also use [***Fairfax HD***](https://www.kreativekorp.com/software/fonts/fairfaxhd/), [***Nishiki-Teki***](https://umihotaru.work/) or any other UCSUR-compliant font listed on [this page](http://antetokipona.infinityfreeapp.com/font).

## Use

To type a glyph, simply type its spelling, and press <kbd>Space</kbd> to confirm. 

![](./doc/soweli.gif)

Pressing <kbd>Enter</kbd> releases the raw ASCII text instead.

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
// This is a line of comment
a 🅰️ 
akesi 🦎 🐸
ala ❌ 
alasa 🏹 

[ #1F58C
] #1F58C
```

You can also customize the appearance and behavior of the input method by editing `%APPDATA%/Ajemi/conf.toml`. The configuration file is self-explanatory. Here's the default one for reference:

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
long_pi = false
long_glyph = false
cjk_space = false
```

## Build from Source

Have Rust with the MSVC toolchain installed and then run the following commands to setup the envirorment:

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
