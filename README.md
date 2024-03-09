# About

Convert Markdown to Unicode:

* `*Emphasis*` => 𝘌𝘮𝘱𝘩𝘢𝘴𝘪𝘴
* `**Strong**` => 𝐒𝐭𝐫𝐨𝐧𝐠
* `` `Code` `` => 𝙲𝚘𝚍𝚎
* `~~Strike~~` => S̶t̶r̶i̶k̶e̶
* `***Emphasis strong***` => 𝑬𝒎𝒑𝒉𝒂𝒔𝒊𝒔 𝒔𝒕𝒓𝒐𝒏𝒈
* `***Emphasis* strong**` => 𝑬𝒎𝒑𝒉𝒂𝒔𝒊𝒔 𝐬𝐭𝐫𝐨𝐧𝐠
* `***Strong** emphasis*` => 𝑺𝒕𝒓𝒐𝒏𝒈 𝘦𝘮𝘱𝘩𝘢𝘴𝘪𝘴
* `` *`Emphasis code`* `` => ℰ𝓂𝓅𝒽𝒶𝓈𝒾𝓈 𝒸ℴ𝒹ℯ
* `` **`Strong code`** `` => 𝓢𝓽𝓻𝓸𝓷𝓰 𝓬𝓸𝓭𝓮
* `` ***`Emphasis strong code`*** `` => Ⓔⓜⓟⓗⓐⓢⓘⓢ ⓢⓣⓡⓞⓝⓖ ⓒⓞⓓⓔ
* `~~*Strike emphasis*~~` => 𝘚̶𝘵̶𝘳̶𝘪̶𝘬̶𝘦̶ ̶𝘦̶𝘮̶𝘱̶𝘩̶𝘢̶𝘴̶𝘪̶𝘴̶
* `~~**Strike strong**~~` => 𝐒̶𝐭̶𝐫̶𝐢̶𝐤̶𝐞̶ ̶𝐬̶𝐭̶𝐫̶𝐨̶𝐧̶𝐠̶
* `~~***Strike emphasis strong***` => 𝑺̶𝒕̶𝒓̶𝒊̶𝒌̶𝒆̶ ̶𝒆̶𝒎̶𝒑̶𝒉̶𝒂̶𝒔̶𝒊̶𝒔̶ ̶𝒔̶𝒕̶𝒓̶𝒐̶𝒏̶𝒈̶
* `` ~~`Strike code`~~ `` => 𝚂̶𝚝̶𝚛̶𝚒̶𝚔̶𝚎̶ ̶𝚌̶𝚘̶𝚍̶𝚎̶
* `` ~~*`Strike emphasis code`*~~ `` => 𝔖𝔱𝔯𝔦𝔨𝔢 𝔢𝔪𝔭𝔥𝔞𝔰𝔦𝔰 𝔠𝔬𝔡𝔢
* `` ~~**`Strike strong code`**~~ `` => 𝕾𝖙𝖗𝖎𝖐𝖊 𝖘𝖙𝖗𝖔𝖓𝖌 𝖈𝖔𝖉𝖊
* `` ~~***`Strike emphasis strong code`*** `` => 𝕊𝕥𝕣𝕚𝕜𝕖 𝕖𝕞𝕡𝕙𝕒𝕤𝕚𝕤 𝕤𝕥𝕣𝕠𝕟𝕘 𝕔𝕠𝕕𝕖

Uses [`pulldown-cmark`] and a modified version of its `push_html` to do *real* Markdown parsing and
rendering.
As a result, it normalizes:

* Headings: Setext headings
* Unordered lists: `*`
* Ordered lists: numbered, `.`
* Rules: `---`
* Tables

## Notes

1. This crate does **not** do syntax highlighting or terminal colors.
   For that, please check out [`bat`] and [`syntect`].

2. This crate outputs Unicode text using the [Mathematical Alphanumeric Symbols] and
   [Enclosed Alphanumerics] blocks, however your ability to *see* the effects depends on the
   specific applications (terminal, text editor, web browser, etc) you're using and their
   configurations (fonts, etc).

3. This crate can be considered an improved version of the [`markdown2unicode`] crate but contains
   no copyrighted nor GPLv3 licensed code from its original upstream source ([USBashka]'s
   [markdown2unicode]) and uses [`pulldown-cmark`] instead.

# Command line

```text
$ unidown -h
Convert Markdown to Unicode

Usage: unidown [OPTIONS] [STRING]...

Arguments:
  [STRING]...  Markdown string(s)

Options:
  -i <PATH>      Input file(s)
  -h, --help     Print help
  -V, --version  Print version
```

```text
$ unidown -V
unidown 0.1.0
```

```text
$ unidown 'Here is some *emphasis*, **strong**, ***strong emphasis***, ~~strike~~, and `code` text.'
Here is some 𝘦𝘮𝘱𝘩𝘢𝘴𝘪𝘴, 𝐬𝐭𝐫𝐨𝐧𝐠, 𝒔𝒕𝒓𝒐𝒏𝒈 𝒆𝒎𝒑𝒉𝒂𝒔𝒊𝒔, s̶t̶r̶i̶k̶e̶, and 𝚌𝚘𝚍𝚎 text.
```

# Library

```
assert_eq!(
   unidown::convert("\
      Here is some *emphasis*, **strong**, ***strong emphasis***, ~~strike~~, \
      and `code` text.\n\n   "),
   "Here is some 𝘦𝘮𝘱𝘩𝘢𝘴𝘪𝘴, 𝐬𝐭𝐫𝐨𝐧𝐠, 𝒔𝒕𝒓𝒐𝒏𝒈 𝒆𝒎𝒑𝒉𝒂𝒔𝒊𝒔, s̶t̶r̶i̶k̶e̶, and 𝚌𝚘𝚍𝚎 text.\n\n",
);
```

[`bat`]: https://crates.io/crates/bat
[`markdown2unicode`]: https://crates.io/crates/markdown2unicode
[`pulldown-cmark`]: https://crates.io/crates/pulldown-cmark
[`syntect`]: https://crates.io/crates/syntect

[Mathematical Alphanumeric Symbols]: https://en.wikipedia.org/wiki/Mathematical_Alphanumeric_Symbols
[Enclosed Alphanumerics]: https://en.wikipedia.org/wiki/Enclosed_Alphanumerics

[USBashka]: https://github.com/USBashka
[markdown2unicode]: https://github.com/USBashka/markdown2unicode

