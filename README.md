# About

Convert Markdown to Unicode:

| Input                                         | Result                      |
|-----------------------------------------------|-----------------------------|
| `` *Emphasis* ``                              | 𝘌𝘮𝘱𝘩𝘢𝘴𝘪𝘴                    |
| `` **Strong** ``                              | 𝐒𝐭𝐫𝐨𝐧𝐠                      |
| `` `Code` ``                                  | 𝙲𝚘𝚍𝚎                        |
| `` ~~Strike~~ ``                              | S̶t̶r̶i̶k̶e̶                      |
| `` ***Emphasis strong*** ``                   | 𝑬𝒎𝒑𝒉𝒂𝒔𝒊𝒔 𝒔𝒕𝒓𝒐𝒏𝒈             |
| `` ***Emphasis* strong** ``                   | 𝑬𝒎𝒑𝒉𝒂𝒔𝒊𝒔 𝐬𝐭𝐫𝐨𝐧𝐠             |
| `` ***Strong** emphasis* ``                   | 𝑺𝒕𝒓𝒐𝒏𝒈 𝘦𝘮𝘱𝘩𝘢𝘴𝘪𝘴             |
| `` *`Emphasis code`* ``                       | ℰ𝓂𝓅𝒽𝒶𝓈𝒾𝓈 𝒸ℴ𝒹ℯ               |
| `` **`Strong code`** ``                       | 𝓢𝓽𝓻𝓸𝓷𝓰 𝓬𝓸𝓭𝓮                 |
| `` ***`Emphasis strong code`*** ``            | Ⓔⓜⓟⓗⓐⓢⓘⓢ ⓢⓣⓡⓞⓝⓖ ⓒⓞⓓⓔ        |
| `` ~~*Strike emphasis*~~ ``                   | 𝘚̶𝘵̶𝘳̶𝘪̶𝘬̶𝘦̶ ̶𝘦̶𝘮̶𝘱̶𝘩̶𝘢̶𝘴̶𝘪̶𝘴̶             |
| `` ~~**Strike strong**~~ ``                   | 𝐒̶𝐭̶𝐫̶𝐢̶𝐤̶𝐞̶ ̶𝐬̶𝐭̶𝐫̶𝐨̶𝐧̶𝐠̶               |
| `` ~~***Strike emphasis strong***~~ ``        | 𝑺̶𝒕̶𝒓̶𝒊̶𝒌̶𝒆̶ ̶𝒆̶𝒎̶𝒑̶𝒉̶𝒂̶𝒔̶𝒊̶𝒔̶ ̶𝒔̶𝒕̶𝒓̶𝒐̶𝒏̶𝒈̶      |
| `` ~~`Strike code`~~ ``                       | 𝚂̶𝚝̶𝚛̶𝚒̶𝚔̶𝚎̶ ̶𝚌̶𝚘̶𝚍̶𝚎̶                 |
| `` ~~*`Strike emphasis code`*~~ ``            | 𝔖𝔱𝔯𝔦𝔨𝔢 𝔢𝔪𝔭𝔥𝔞𝔰𝔦𝔰 𝔠𝔬𝔡𝔢        |
| `` ~~**`Strike strong code`**~~ ``            | 𝕾𝖙𝖗𝖎𝖐𝖊 𝖘𝖙𝖗𝖔𝖓𝖌 𝖈𝖔𝖉𝖊          |
| `` ~~***`Strike emphasis strong code`***~~ `` | 𝕊𝕥𝕣𝕚𝕜𝕖 𝕖𝕞𝕡𝕙𝕒𝕤𝕚𝕤 𝕤𝕥𝕣𝕠𝕟𝕘 𝕔𝕠𝕕𝕖 |

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
   For instance, it's very probable that regular and monospace *look* identical in a terminal or
   text editor, because the font *is* probably monospace.
   If a non-monospace font is configured or another application is used that uses a non-monospace
   font, or the output bytes are examined more closely, you will *see* the effect.

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
  -s <STYLE>     Style [possible values: italic, bold, monospace, strike,
                 bold-italic, script, bold-script, circled, strike-italic,
                 strike-bold, strike-bold-italic, strike-monospace, fraktur,
                 bold-fraktur, double-struck, all, demo]
  -i <PATH>      Input file(s)
  -r             Print readme
  -h, --help     Print help
  -V, --version  Print version
```

```text
$ unidown -V
unidown 0.9.7
```

```text
$ unidown 'Here is some *emphasis*, **strong**, ***strong emphasis***, ~~strike~~, and `code` text.'
Here is some 𝘦𝘮𝘱𝘩𝘢𝘴𝘪𝘴, 𝐬𝐭𝐫𝐨𝐧𝐠, 𝒔𝒕𝒓𝒐𝒏𝒈 𝒆𝒎𝒑𝒉𝒂𝒔𝒊𝒔, s̶t̶r̶i̶k̶e̶, and 𝚌𝚘𝚍𝚎 text.
```

```text
$ unidown -s all 'Your text here'
𝘠𝘰𝘶𝘳 𝘵𝘦𝘹𝘵 𝘩𝘦𝘳𝘦
𝐘𝐨𝐮𝐫 𝐭𝐞𝐱𝐭 𝐡𝐞𝐫𝐞
𝚈𝚘𝚞𝚛 𝚝𝚎𝚡𝚝 𝚑𝚎𝚛𝚎
Y̶o̶u̶r̶ ̶t̶e̶x̶t̶ ̶h̶e̶r̶e̶
𝒀𝒐𝒖𝒓 𝒕𝒆𝒙𝒕 𝒉𝒆𝒓𝒆
𝒴ℴ𝓊𝓇 𝓉ℯ𝓍𝓉 𝒽ℯ𝓇ℯ
𝓨𝓸𝓾𝓻 𝓽𝓮𝔁𝓽 𝓱𝓮𝓻𝓮
Ⓨⓞⓤⓡ ⓣⓔⓧⓣ ⓗⓔⓡⓔ
𝘠̶𝘰̶𝘶̶𝘳̶ ̶𝘵̶𝘦̶𝘹̶𝘵̶ ̶𝘩̶𝘦̶𝘳̶𝘦̶
𝐘̶𝐨̶𝐮̶𝐫̶ ̶𝐭̶𝐞̶𝐱̶𝐭̶ ̶𝐡̶𝐞̶𝐫̶𝐞̶
𝒀̶𝒐̶𝒖̶𝒓̶ ̶𝒕̶𝒆̶𝒙̶𝒕̶ ̶𝒉̶𝒆̶𝒓̶𝒆̶
𝚈̶𝚘̶𝚞̶𝚛̶ ̶𝚝̶𝚎̶𝚡̶𝚝̶ ̶𝚑̶𝚎̶𝚛̶𝚎̶
𝔜𝔬𝔲𝔯 𝔱𝔢𝔵𝔱 𝔥𝔢𝔯𝔢
𝖄𝖔𝖚𝖗 𝖙𝖊𝖝𝖙 𝖍𝖊𝖗𝖊
𝕐𝕠𝕦𝕣 𝕥𝕖𝕩𝕥 𝕙𝕖𝕣𝕖
```

```text
$ unidown -s demo 'Your text here'
| Input                            | Result         | Style              |
|----------------------------------|----------------|--------------------|
| `` *Your text here* ``           | 𝘠𝘰𝘶𝘳 𝘵𝘦𝘹𝘵 𝘩𝘦𝘳𝘦 | italic             |
| `` **Your text here** ``         | 𝐘𝐨𝐮𝐫 𝐭𝐞𝐱𝐭 𝐡𝐞𝐫𝐞 | bold               |
| `` `Your text here` ``           | 𝚈𝚘𝚞𝚛 𝚝𝚎𝚡𝚝 𝚑𝚎𝚛𝚎 | monospace          |
| `` ~~Your text here~~ ``         | Y̶o̶u̶r̶ ̶t̶e̶x̶t̶ ̶h̶e̶r̶e̶ | strike             |
| `` ***Your text here*** ``       | 𝒀𝒐𝒖𝒓 𝒕𝒆𝒙𝒕 𝒉𝒆𝒓𝒆 | bold-italic        |
| `` *`Your text here`* ``         | 𝒴ℴ𝓊𝓇 𝓉ℯ𝓍𝓉 𝒽ℯ𝓇ℯ | script             |
| `` **`Your text here`** ``       | 𝓨𝓸𝓾𝓻 𝓽𝓮𝔁𝓽 𝓱𝓮𝓻𝓮 | bold-script        |
| `` ***`Your text here`*** ``     | Ⓨⓞⓤⓡ ⓣⓔⓧⓣ ⓗⓔⓡⓔ | circled            |
| `` ~~*Your text here*~~ ``       | 𝘠̶𝘰̶𝘶̶𝘳̶ ̶𝘵̶𝘦̶𝘹̶𝘵̶ ̶𝘩̶𝘦̶𝘳̶𝘦̶ | strike-italic      |
| `` ~~**Your text here**~~ ``     | 𝐘̶𝐨̶𝐮̶𝐫̶ ̶𝐭̶𝐞̶𝐱̶𝐭̶ ̶𝐡̶𝐞̶𝐫̶𝐞̶ | strike-bold        |
| `` ~~***Your text here***~~ ``   | 𝒀̶𝒐̶𝒖̶𝒓̶ ̶𝒕̶𝒆̶𝒙̶𝒕̶ ̶𝒉̶𝒆̶𝒓̶𝒆̶ | strike-bold-italic |
| `` ~~`Your text here`~~ ``       | 𝚈̶𝚘̶𝚞̶𝚛̶ ̶𝚝̶𝚎̶𝚡̶𝚝̶ ̶𝚑̶𝚎̶𝚛̶𝚎̶ | strike-monospace   |
| `` ~~*`Your text here`*~~ ``     | 𝔜𝔬𝔲𝔯 𝔱𝔢𝔵𝔱 𝔥𝔢𝔯𝔢 | fraktur            |
| `` ~~**`Your text here`**~~ ``   | 𝖄𝖔𝖚𝖗 𝖙𝖊𝖝𝖙 𝖍𝖊𝖗𝖊 | bold-fraktur       |
| `` ~~***`Your text here`***~~ `` | 𝕐𝕠𝕦𝕣 𝕥𝕖𝕩𝕥 𝕙𝕖𝕣𝕖 | double-struck      |
```

```text
$ unidown -s fraktur 'Your text here'
𝔜𝔬𝔲𝔯 𝔱𝔢𝔵𝔱 𝔥𝔢𝔯𝔢
```

# Library

See the [API documentation](https://docs.rs/unidown).

[`bat`]: https://crates.io/crates/bat
[`markdown2unicode`]: https://crates.io/crates/markdown2unicode
[`pulldown-cmark`]: https://crates.io/crates/pulldown-cmark
[`syntect`]: https://crates.io/crates/syntect

[Mathematical Alphanumeric Symbols]: https://en.wikipedia.org/wiki/Mathematical_Alphanumeric_Symbols
[Enclosed Alphanumerics]: https://en.wikipedia.org/wiki/Enclosed_Alphanumerics

[USBashka]: https://github.com/USBashka
[markdown2unicode]: https://github.com/USBashka/markdown2unicode

