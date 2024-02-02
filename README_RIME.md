# Ajemi with RIME

## Install

1. Install [RIME](https://rime.im/)
2. Install the font [Nishiki-teki](https://umihotaru.work/nishiki-teki.zip)
3. Download the zipped [config files](https://github.com/dec32/Ajemi/releases/download/rime-schema/Ajemi.zip)
4. Copy all three config files:

    - `ajemi.schema.yaml`
    - `ajemi.dict.yaml`
    - `default.custom.yaml`

   into: `C:/User/{YOUR_ACCOUNT}/AppData/Roaming/Rime`

5. Press <kbd>Win</kbd>+<kbd>Space</kbd> to switch to RIME and reload it via:

    ![](./doc/reload.jpg)

P.S. If you are using mac or Linux, copy the config files into:

|Platform                        |Directory                    |
|--------------------------------|-----------------------------|
|macOS                           |`~/Library/Rime`             |
|Linux                           |`~/.config/ibus/rime/`       |
|Linux with ibus 0.9.1 or lower  |`~/.ibus/rime/`              |
|Linux with fcitx5               |`~/.local/share/fcitx5/rime/`|

## Customize

RIME is a highly customizable input method. Here's a very breif instruction on how to customize a couple of things.

To use a different font for the input method, edit the value of `"style/font_face"` in `weasel.custom.yaml`. Notice that Ameji relies on UCSUR-compliant fonts to function. To find and install such fonts, please visit [this spreadsheet](https://docs.google.com/spreadsheets/d/1xwgTAxwgn4ZAc4DBnHte0cqta1aaxe112Wh1rv9w5Yk/htmlview?gid=1195574771).

To customize the spellings of the glyphs, go to `ajemi.dict.yaml` to edit the mapping. Each glyph and its corresponding spelling are seperated by a TAB character. You can also map a sequence of glyphs to a spelling. That will create a phrase.

To customize punctuators and control characters, edit the `punctuator` section of `ajemi.schema.yaml`.

Remember to reload RIME after editing the config files to deploy the customization.

## Uninstall

1. Run `C:\Program Files (x86)\Rime\weasel-{VERSION}\uninstall.exe`
2. Reboot your device
