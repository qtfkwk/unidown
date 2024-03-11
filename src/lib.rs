/*!
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

3. This crate can be considered an improved version of the [`markdown2unicode`] crate but contains
   no copyrighted nor GPLv3 licensed code from its original upstream source ([USBashka]'s
   [markdown2unicode]) and uses [`pulldown-cmark`] instead.

# Example

```
assert_eq!(
   unidown::convert("\
      Here is some *emphasis*, **strong**, ***strong emphasis***, ~~strike~~, \
      and `code` text.\n\n\
   "),
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

*/

// This file contains modified copies of code from the [`pulldown-cmark`] crate, which requires
// including the following license, copyright, and permission notice:
//
// ```text
// The MIT License
//
// Copyright 2015 Google Inc. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
// ```
//
// [`pulldown-cmark`]: https://github.com/pulldown-cmark/pulldown-cmark

//--------------------------------------------------------------------------------------------------
// Crates

use clap::ValueEnum;
use lazy_static::lazy_static;
use pulldown_cmark::{Alignment, CowStr, Event, Event::*, HeadingLevel, Tag, TagEnd};
use pulldown_cmark_escape::{
    escape_href, escape_html, escape_html_body_text, StrWrite, WriteWrapper,
};
use std::{
    collections::HashMap,
    io::{self, Write},
};
use veg::Veg;

//--------------------------------------------------------------------------------------------------
// Constants

// Alphabets
const REGULAR: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_-+=?/|'\"`";
const BOLD: &str =
    "𝐀𝐁𝐂𝐃𝐄𝐅𝐆𝐇𝐈𝐉𝐊𝐋𝐌𝐍𝐎𝐏𝐐𝐑𝐒𝐓𝐔𝐕𝐖𝐗𝐘𝐙𝐚𝐛𝐜𝐝𝐞𝐟𝐠𝐡𝐢𝐣𝐤𝐥𝐦𝐧𝐨𝐩𝐪𝐫𝐬𝐭𝐮𝐯𝐰𝐱𝐲𝐳𝟎𝟏𝟐𝟑𝟒𝟓𝟔𝟕𝟖𝟗!@#$%^&*()_-+=?/|'\"`";
const ITALIC: &str =
    "𝘈𝘉𝘊𝘋𝘌𝘍𝘎𝘏𝘐𝘑𝘒𝘓𝘔𝘕𝘖𝘗𝘘𝘙𝘚𝘛𝘜𝘝𝘞𝘟𝘠𝘡𝘢𝘣𝘤𝘥𝘦𝘧𝘨𝘩𝘪𝘫𝘬𝘭𝘮𝘯𝘰𝘱𝘲𝘳𝘴𝘵𝘶𝘷𝘸𝘹𝘺𝘻0123456789!@#$%^&*()_-+=?/|'\"`";
const BOLD_ITALIC: &str =
    "𝑨𝑩𝑪𝑫𝑬𝑭𝑮𝑯𝑰𝑱𝑲𝑳𝑴𝑵𝑶𝑷𝑸𝑹𝑺𝑻𝑼𝑽𝑾𝑿𝒀𝒁𝒂𝒃𝒄𝒅𝒆𝒇𝒈𝒉𝒊𝒋𝒌𝒍𝒎𝒏𝒐𝒑𝒒𝒓𝒔𝒕𝒖𝒗𝒘𝒙𝒚𝒛0123456789!@#$%^&*()_-+=?/|'\"`";
const MONO: &str =
    "𝙰𝙱𝙲𝙳𝙴𝙵𝙶𝙷𝙸𝙹𝙺𝙻𝙼𝙽𝙾𝙿𝚀𝚁𝚂𝚃𝚄𝚅𝚆𝚇𝚈𝚉𝚊𝚋𝚌𝚍𝚎𝚏𝚐𝚑𝚒𝚓𝚔𝚕𝚖𝚗𝚘𝚙𝚚𝚛𝚜𝚝𝚞𝚟𝚠𝚡𝚢𝚣𝟶𝟷𝟸𝟹𝟺𝟻𝟼𝟽𝟾𝟿!@#$%^&*()_-+=?/|'\"`";
const CURSIVE: &str =
    "𝒜ℬ𝒞𝒟ℰℱ𝒢ℋℐ𝒥𝒦ℒℳ𝒩𝒪𝒫𝒬ℛ𝒮𝒯𝒰𝒱𝒲𝒳𝒴𝒵𝒶𝒷𝒸𝒹ℯ𝒻ℊ𝒽𝒾𝒿𝓀𝓁𝓂𝓃ℴ𝓅𝓆𝓇𝓈𝓉𝓊𝓋𝓌𝓍𝓎𝓏0123456789!@#$%^&*()_-+=?/|'\"`";
const BOLD_CURSIVE: &str =
    "𝓐𝓑𝓒𝓓𝓔𝓕𝓖𝓗𝓘𝓙𝓚𝓛𝓜𝓝𝓞𝓟𝓠𝓡𝓢𝓣𝓤𝓥𝓦𝓧𝓨𝓩𝓪𝓫𝓬𝓭𝓮𝓯𝓰𝓱𝓲𝓳𝓴𝓵𝓶𝓷𝓸𝓹𝓺𝓻𝓼𝓽𝓾𝓿𝔀𝔁𝔂𝔃0123456789!@#$%^&*()_-+=?/|'\"`";
const FRAKTUR: &str =
    "𝔄𝔅ℭ𝔇𝔈𝔉𝔊ℌℑ𝔍𝔎𝔏𝔐𝔑𝔒𝔓𝔔ℜ𝔖𝔗𝔘𝔙𝔚𝔛𝔜ℨ𝔞𝔟𝔠𝔡𝔢𝔣𝔤𝔥𝔦𝔧𝔨𝔩𝔪𝔫𝔬𝔭𝔮𝔯𝔰𝔱𝔲𝔳𝔴𝔵𝔶𝔷0123456789!@#$%^&*()_-+=?/|'\"`";
const BOLD_FRAKTUR: &str =
    "𝕬𝕭𝕮𝕯𝕰𝕱𝕲𝕳𝕴𝕵𝕶𝕷𝕸𝕹𝕺𝕻𝕼𝕽𝕾𝕿𝖀𝖁𝖂𝖃𝖄𝖅𝖆𝖇𝖈𝖉𝖊𝖋𝖌𝖍𝖎𝖏𝖐𝖑𝖒𝖓𝖔𝖕𝖖𝖗𝖘𝖙𝖚𝖛𝖜𝖝𝖞𝖟0123456789!@#$%^&*()_-+=?/|'\"`";
const DOUBLE: &str =
    "𝔸𝔹ℂ𝔻𝔼𝔽𝔾ℍ𝕀𝕁𝕂𝕃𝕄ℕ𝕆ℙℚℝ𝕊𝕋𝕌𝕍𝕎𝕏𝕐ℤ𝕒𝕓𝕔𝕕𝕖𝕗𝕘𝕙𝕚𝕛𝕜𝕝𝕞𝕟𝕠𝕡𝕢𝕣𝕤𝕥𝕦𝕧𝕨𝕩𝕪𝕫𝟘𝟙𝟚𝟛𝟜𝟝𝟞𝟟𝟠𝟡!@#$%^&*()_-+=?/|'\"`";
const CIRCLE: &str =
    "ⒶⒷⒸⒹⒺⒻⒼⒽⒾⒿⓀⓁⓂⓃⓄⓅⓆⓇⓈⓉⓊⓋⓌⓍⓎⓏⓐⓑⓒⓓⓔⓕⓖⓗⓘⓙⓚⓛⓜⓝⓞⓟⓠⓡⓢⓣⓤⓥⓦⓧⓨⓩ⓪①②③④⑤⑥⑦⑧⑨!@#$%^&*()_-+=?/|'\"`";
// const PARENS: &str =
//     "🄐🄑🄒🄓🄔🄕🄖🄗🄘🄙🄚🄛🄜🄝🄞🄟🄠🄡🄢🄣🄤🄥🄦🄧🄨🄩⒜⒝⒞⒟⒠⒡⒢⒣⒤⒥⒦⒧⒨⒩⒪⒫⒬⒭⒮⒯⒰⒱⒲⒳⒴⒵0⑴⑵⑶⑷⑸⑹⑺⑻⑼!@#$%^&*()_-+=?/|'\"`";

//--------------------------------------------------------------------------------------------------
// Static

lazy_static! {
    // HashMaps with char to index in alphabet
    static ref REGULAR_I: HashMap<char, usize> = c2i(REGULAR);
    static ref BOLD_I: HashMap<char, usize> = c2i(BOLD);
    static ref ITALIC_I: HashMap<char, usize> = c2i(ITALIC);
    static ref BOLD_ITALIC_I: HashMap<char, usize> = c2i(BOLD_ITALIC);
    static ref MONO_I: HashMap<char, usize> = c2i(MONO);
    static ref CURSIVE_I: HashMap<char, usize> = c2i(CURSIVE);
    static ref BOLD_CURSIVE_I: HashMap<char, usize> = c2i(BOLD_CURSIVE);
    static ref FRAKTUR_I: HashMap<char, usize> = c2i(FRAKTUR);
    static ref BOLD_FRAKTUR_I: HashMap<char, usize> = c2i(BOLD_FRAKTUR);
    static ref DOUBLE_I: HashMap<char, usize> = c2i(DOUBLE);
    static ref CIRCLE_I: HashMap<char, usize> = c2i(CIRCLE);
    // static ref PARENS_I: HashMap<char, usize> = c2i(PARENS);

    // HashMap with char to index for all alphabets to convert any character to its index
    static ref ALL_I: HashMap<char, usize> = REGULAR_I
        .iter()
        .chain(BOLD_I.iter())
        .chain(ITALIC_I.iter())
        .chain(BOLD_ITALIC_I.iter())
        .chain(MONO_I.iter())
        .chain(CURSIVE_I.iter())
        .chain(BOLD_CURSIVE_I.iter())
        .chain(FRAKTUR_I.iter())
        .chain(BOLD_FRAKTUR_I.iter())
        .chain(DOUBLE_I.iter())
        .chain(CIRCLE_I.iter())
        // .chain(PARENS_I.iter())
        .map(|(c, i)| (*c, *i))
        .collect();

    // HashMaps with index to char
    static ref REGULAR_C: HashMap<usize, char> = i2c(REGULAR);
    static ref BOLD_C: HashMap<usize, char> = i2c(BOLD);
    static ref ITALIC_C: HashMap<usize, char> = i2c(ITALIC);
    static ref BOLD_ITALIC_C: HashMap<usize, char> = i2c(BOLD_ITALIC);
    static ref MONO_C: HashMap<usize, char> = i2c(MONO);
    static ref CURSIVE_C: HashMap<usize, char> = i2c(CURSIVE);
    static ref BOLD_CURSIVE_C: HashMap<usize, char> = i2c(BOLD_CURSIVE);
    static ref FRAKTUR_C: HashMap<usize, char> = i2c(FRAKTUR);
    static ref BOLD_FRAKTUR_C: HashMap<usize, char> = i2c(BOLD_FRAKTUR);
    static ref DOUBLE_C: HashMap<usize, char> = i2c(DOUBLE);
    static ref CIRCLE_C: HashMap<usize, char> = i2c(CIRCLE);
    // static ref PARENS_C: HashMap<usize, char> = i2c(PARENS);
}

//--------------------------------------------------------------------------------------------------
// Functions

/// Convert each char to its index
fn c2i(s: &str) -> HashMap<char, usize> {
    s.chars().enumerate().map(|(i, c)| (c, i)).collect()
}

/// Convert each index to its char
fn i2c(s: &str) -> HashMap<usize, char> {
    s.chars().enumerate().collect()
}

// Append the Combining Long Stroke Overlay to each character
fn strike(s: &str) -> String {
    s.chars().flat_map(|x| [x, '\u{0336}']).collect::<String>()
}

/**
Convert Markdown to Unicode

```
for (markdown, unicode) in &[
    ("*Emphasis*\n\n", "𝘌𝘮𝘱𝘩𝘢𝘴𝘪𝘴\n\n"),
    ("**Strong**\n\n", "𝐒𝐭𝐫𝐨𝐧𝐠\n\n"),
    ("`Code`\n\n", "𝙲𝚘𝚍𝚎\n\n"),
    ("~~Strike~~\n\n", "S̶t̶r̶i̶k̶e̶\n\n"),
    ("***Emphasis strong***\n\n", "𝑬𝒎𝒑𝒉𝒂𝒔𝒊𝒔 𝒔𝒕𝒓𝒐𝒏𝒈\n\n"),
    ("***Emphasis* strong**\n\n", "𝑬𝒎𝒑𝒉𝒂𝒔𝒊𝒔 𝐬𝐭𝐫𝐨𝐧𝐠\n\n"),
    ("***Strong** emphasis*\n\n", "𝑺𝒕𝒓𝒐𝒏𝒈 𝘦𝘮𝘱𝘩𝘢𝘴𝘪𝘴\n\n"),
    ("*`Emphasis code`*\n\n", "ℰ𝓂𝓅𝒽𝒶𝓈𝒾𝓈 𝒸ℴ𝒹ℯ\n\n"),
    ("**`Strong code`**\n\n", "𝓢𝓽𝓻𝓸𝓷𝓰 𝓬𝓸𝓭𝓮\n\n"),
    ("***`Emphasis strong code`***\n\n", "Ⓔⓜⓟⓗⓐⓢⓘⓢ ⓢⓣⓡⓞⓝⓖ ⓒⓞⓓⓔ\n\n"),
    ("~~*Strike emphasis*~~\n\n", "𝘚̶𝘵̶𝘳̶𝘪̶𝘬̶𝘦̶ ̶𝘦̶𝘮̶𝘱̶𝘩̶𝘢̶𝘴̶𝘪̶𝘴̶\n\n"),
    ("~~**Strike strong**~~\n\n", "𝐒̶𝐭̶𝐫̶𝐢̶𝐤̶𝐞̶ ̶𝐬̶𝐭̶𝐫̶𝐨̶𝐧̶𝐠̶\n\n"),
    ("~~***Strike emphasis strong***~~\n\n", "𝑺̶𝒕̶𝒓̶𝒊̶𝒌̶𝒆̶ ̶𝒆̶𝒎̶𝒑̶𝒉̶𝒂̶𝒔̶𝒊̶𝒔̶ ̶𝒔̶𝒕̶𝒓̶𝒐̶𝒏̶𝒈̶\n\n"),
    ("~~`Strike code`~~\n\n", "𝚂̶𝚝̶𝚛̶𝚒̶𝚔̶𝚎̶ ̶𝚌̶𝚘̶𝚍̶𝚎̶\n\n"),
    ("~~*`Strike emphasis code`*~~\n\n", "𝔖𝔱𝔯𝔦𝔨𝔢 𝔢𝔪𝔭𝔥𝔞𝔰𝔦𝔰 𝔠𝔬𝔡𝔢\n\n"),
    ("~~**`Strike strong code`**~~\n\n", "𝕾𝖙𝖗𝖎𝖐𝖊 𝖘𝖙𝖗𝖔𝖓𝖌 𝖈𝖔𝖉𝖊\n\n"),
    ("~~***`Strike emphasis strong code`***~~\n\n", "𝕊𝕥𝕣𝕚𝕜𝕖 𝕖𝕞𝕡𝕙𝕒𝕤𝕚𝕤 𝕤𝕥𝕣𝕠𝕟𝕘 𝕔𝕠𝕕𝕖\n\n"),
] {
    assert_eq!(unidown::convert(markdown), *unicode);
}
```
*/
pub fn convert(s: &str) -> String {
    let mut r = String::new();
    push_unicode(
        &mut r,
        pulldown_cmark::Parser::new_ext(s, pulldown_cmark::Options::all()),
    );
    r
}

#[derive(Debug)]
struct Row {
    input: String,
    result: String,
    unicode: String,
}

impl Row {
    fn new(input: &str, result: &str, unicode: &str) -> Box<Row> {
        Box::new(Row {
            input: input.to_string(),
            result: result.to_string(),
            unicode: unicode.to_string(),
        })
    }
}

impl veg::Table for Row {
    fn row(&self) -> Vec<String> {
        vec![
            self.input.clone(),
            self.result.clone(),
            self.unicode.clone(),
        ]
    }
}

/**
Demo mode

```
assert_eq!(
    unidown::demo("Your text here"),
    "\
        | Input                            | Result         | Style              |\n\
        |----------------------------------|----------------|--------------------|\n\
        | `` *Your text here* ``           | 𝘠𝘰𝘶𝘳 𝘵𝘦𝘹𝘵 𝘩𝘦𝘳𝘦 | italic             |\n\
        | `` **Your text here** ``         | 𝐘𝐨𝐮𝐫 𝐭𝐞𝐱𝐭 𝐡𝐞𝐫𝐞 | bold               |\n\
        | `` `Your text here` ``           | 𝚈𝚘𝚞𝚛 𝚝𝚎𝚡𝚝 𝚑𝚎𝚛𝚎 | monospace          |\n\
        | `` ~~Your text here~~ ``         | Y̶o̶u̶r̶ ̶t̶e̶x̶t̶ ̶h̶e̶r̶e̶ | strike             |\n\
        | `` ***Your text here*** ``       | 𝒀𝒐𝒖𝒓 𝒕𝒆𝒙𝒕 𝒉𝒆𝒓𝒆 | bold-italic        |\n\
        | `` *`Your text here`* ``         | 𝒴ℴ𝓊𝓇 𝓉ℯ𝓍𝓉 𝒽ℯ𝓇ℯ | script             |\n\
        | `` **`Your text here`** ``       | 𝓨𝓸𝓾𝓻 𝓽𝓮𝔁𝓽 𝓱𝓮𝓻𝓮 | bold-script        |\n\
        | `` ***`Your text here`*** ``     | Ⓨⓞⓤⓡ ⓣⓔⓧⓣ ⓗⓔⓡⓔ | circled            |\n\
        | `` ~~*Your text here*~~ ``       | 𝘠̶𝘰̶𝘶̶𝘳̶ ̶𝘵̶𝘦̶𝘹̶𝘵̶ ̶𝘩̶𝘦̶𝘳̶𝘦̶ | strike-italic      |\n\
        | `` ~~**Your text here**~~ ``     | 𝐘̶𝐨̶𝐮̶𝐫̶ ̶𝐭̶𝐞̶𝐱̶𝐭̶ ̶𝐡̶𝐞̶𝐫̶𝐞̶ | strike-bold        |\n\
        | `` ~~***Your text here***~~ ``   | 𝒀̶𝒐̶𝒖̶𝒓̶ ̶𝒕̶𝒆̶𝒙̶𝒕̶ ̶𝒉̶𝒆̶𝒓̶𝒆̶ | strike-bold-italic |\n\
        | `` ~~`Your text here`~~ ``       | 𝚈̶𝚘̶𝚞̶𝚛̶ ̶𝚝̶𝚎̶𝚡̶𝚝̶ ̶𝚑̶𝚎̶𝚛̶𝚎̶ | strike-monospace   |\n\
        | `` ~~*`Your text here`*~~ ``     | 𝔜𝔬𝔲𝔯 𝔱𝔢𝔵𝔱 𝔥𝔢𝔯𝔢 | fraktur            |\n\
        | `` ~~**`Your text here`**~~ ``   | 𝖄𝖔𝖚𝖗 𝖙𝖊𝖝𝖙 𝖍𝖊𝖗𝖊 | bold-fraktur       |\n\
        | `` ~~***`Your text here`***~~ `` | 𝕐𝕠𝕦𝕣 𝕥𝕖𝕩𝕥 𝕙𝕖𝕣𝕖 | double-struck      |\n\n\
    ",
);
```
*/
pub fn demo(s: &str) -> String {
    let mut t = Veg::table("Input|Result|Style\n-|-|-");
    for (syntax, style) in [
        ("*", "italic"),
        ("**", "bold"),
        ("`", "monospace"),
        ("~~", "strike"),
        ("***", "bold-italic"),
        ("*`", "script"),
        ("**`", "bold-script"),
        ("***`", "circled"),
        ("~~*", "strike-italic"),
        ("~~**", "strike-bold"),
        ("~~***", "strike-bold-italic"),
        ("~~`", "strike-monospace"),
        ("~~*`", "fraktur"),
        ("~~**`", "bold-fraktur"),
        ("~~***`", "double-struck"),
    ] {
        let input = format!("{syntax}{s}{}", syntax.chars().rev().collect::<String>());
        t.push(Row::new(
            &format!("`` {input} ``"),
            &convert(&input).replace("\n\n", ""),
            style,
        ));
    }
    format!("{}\n", t.markdown().unwrap())
}

/**
All mode

```
assert_eq!(
    unidown::all("Text"),
    "\
        𝘛𝘦𝘹𝘵\n\
        𝐓𝐞𝐱𝐭\n\
        𝚃𝚎𝚡𝚝\n\
        T̶e̶x̶t̶\n\
        𝑻𝒆𝒙𝒕\n\
        𝒯ℯ𝓍𝓉\n\
        𝓣𝓮𝔁𝓽\n\
        Ⓣⓔⓧⓣ\n\
        𝘛̶𝘦̶𝘹̶𝘵̶\n\
        𝐓̶𝐞̶𝐱̶𝐭̶\n\
        𝑻̶𝒆̶𝒙̶𝒕̶\n\
        𝚃̶𝚎̶𝚡̶𝚝̶\n\
        𝔗𝔢𝔵𝔱\n\
        𝕿𝖊𝖝𝖙\n\
        𝕋𝕖𝕩𝕥\n\n\
    ",
);
```
*/
pub fn all(s: &str) -> String {
    format!(
        "{}\n",
        [
            "*", "**", "`", "~~", "***", "*`", "**`", "***`", "~~*", "~~**", "~~***", "~~`",
            "~~*`", "~~**`", "~~***`",
        ]
        .iter()
        .map(|a| {
            convert(&format!("{a}{s}{}", a.chars().rev().collect::<String>())).replace("\n\n", "\n")
        })
        .collect::<String>()
    )
}

/**
Iterate over an [`Iterator`] of [`Event`]s, generate Unicode for each [`Event`], and push it to a
[`String`].
*/
pub fn push_unicode<'a, I>(s: &mut String, iter: I)
where
    I: Iterator<Item = pulldown_cmark::Event<'a>>,
{
    UnicodeWriter::new(iter, s).run().unwrap();
}

/**
Iterate over an [`Iterator`] of [`Event`]s, generate Unicode for each [`Event`], and write it out to
a writable stream.
*/
pub fn write_unicode<'a, I, W>(writer: W, iter: I) -> io::Result<()>
where
    I: Iterator<Item = Event<'a>>,
    W: Write,
{
    UnicodeWriter::new(iter, WriteWrapper(writer)).run()
}

//--------------------------------------------------------------------------------------------------
// Structs and enums

#[derive(Clone, ValueEnum)]
pub enum Style {
    Italic,
    Bold,
    Monospace,
    Strike,
    BoldItalic,
    Script,
    BoldScript,
    Circled,
    StrikeItalic,
    StrikeBold,
    StrikeBoldItalic,
    StrikeMonospace,
    Fraktur,
    BoldFraktur,
    DoubleStruck,
    All,
    Demo,
}

impl Style {
    fn syntax(&self) -> (&str, &str) {
        match self {
            Style::Italic => ("*", "*"),
            Style::Bold => ("**", "**"),
            Style::Monospace => ("`", "`"),
            Style::Strike => ("~~", "~~"),
            Style::BoldItalic => ("***", "***"),
            Style::Script => ("*`", "`*"),
            Style::BoldScript => ("**`", "`**"),
            Style::Circled => ("***`", "`***"),
            Style::StrikeItalic => ("~~*", "*~~"),
            Style::StrikeBold => ("~~**", "**~~"),
            Style::StrikeBoldItalic => ("***~~", "***~~"),
            Style::StrikeMonospace => ("~~`", "`~~"),
            Style::Fraktur => ("~~*`", "`*~~"),
            Style::BoldFraktur => ("~~**`", "`**~~"),
            Style::DoubleStruck => ("~~***`", "`***~~"),
            Style::All | Style::Demo => ("", ""),
        }
    }

    /**
    Convert text to Unicode style

    ```
    assert_eq!(unidown::Style::Fraktur.convert("Your text here"), "𝔜𝔬𝔲𝔯 𝔱𝔢𝔵𝔱 𝔥𝔢𝔯𝔢\n\n");
    ```
    */
    pub fn convert(&self, s: &str) -> String {
        match self {
            Style::All => all(s),
            Style::Demo => demo(s),
            _ => {
                let (a, b) = self.syntax();
                convert(&format!("{a}{s}{b}"))
            }
        }
    }
}

enum List {
    Ordered,
    Unordered,
}

struct Counter {
    value: u64,
}

impl Counter {
    fn new(value: u64) -> Counter {
        Counter { value }
    }

    fn next(&mut self) -> u64 {
        let r = self.value;
        self.value += 1;
        r
    }
}

struct Link {
    dest_url: String,
    title: String,
}

enum TableState {
    Head,
    Body,
}

#[derive(PartialEq, Eq, Hash)]
enum TableAlign {
    Left,
    Center,
    Right,
    None,
}

impl TableAlign {
    fn from(alignment: &Alignment) -> TableAlign {
        match alignment {
            Alignment::Left => TableAlign::Left,
            Alignment::Center => TableAlign::Center,
            Alignment::Right => TableAlign::Right,
            Alignment::None => TableAlign::None,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            TableAlign::Left => ":---",
            TableAlign::Center => ":---:",
            TableAlign::Right => "---:",
            TableAlign::None => "---",
        }
    }
}

/// Port of `pulldown_cmark::html::HtmlWriter` that writes Unicode text instead of HTML
struct UnicodeWriter<'a, I, W> {
    /// Iterator supplying events.
    iter: I,

    /// Writer to write to.
    writer: W,

    /// Whether or not the last write wrote a newline.
    end_newline: bool,

    /// Whether if inside a metadata block (text should not be written)
    in_non_writing_block: bool,

    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    numbers: HashMap<CowStr<'a>, usize>,

    // modes
    emphasis: bool,
    strong: bool,
    strike: bool,
    code: bool,

    // lists
    lists: Vec<List>,
    ol: Vec<Counter>,

    link: Option<Link>,
}

impl<'a, I, W> UnicodeWriter<'a, I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: StrWrite,
{
    fn new(iter: I, writer: W) -> Self {
        Self {
            iter,
            writer,
            end_newline: true,
            in_non_writing_block: false,
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            numbers: HashMap::new(),
            emphasis: false,
            strong: false,
            strike: false,
            code: false,
            lists: vec![],
            ol: vec![],
            link: None,
        }
    }

    /// Writes a new line.
    fn write_newline(&mut self) -> io::Result<()> {
        self.end_newline = true;
        self.writer.write_str("\n")
    }

    /// Writes a buffer, and tracks whether or not a newline was written.
    #[inline]
    fn write(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_str(s)?;
        if !s.is_empty() {
            self.end_newline = s.ends_with('\n');
        }
        Ok(())
    }

    fn run(mut self) -> io::Result<()> {
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => {
                    self.start_tag(tag)?;
                }
                End(tag) => {
                    self.end_tag(tag)?;
                }
                Text(text) => {
                    if !self.in_non_writing_block {
                        let mut t = String::new();
                        escape_html_body_text(&mut t, &text)?;
                        let alphabet: &HashMap<usize, char> = if self.strong && self.emphasis {
                            &BOLD_ITALIC_C
                        } else if self.strong {
                            &BOLD_C
                        } else if self.emphasis {
                            &ITALIC_C
                        } else {
                            &REGULAR_C
                        };
                        let u = t
                            .chars()
                            .map(|c| {
                                if let Some(i) = ALL_I.get(&c) {
                                    alphabet[i]
                                } else {
                                    c
                                }
                            })
                            .collect::<String>();
                        let u = if self.strike { strike(&u) } else { u };
                        self.write(&u)?;
                        self.end_newline = text.ends_with('\n');
                    }
                }
                Code(text) => {
                    let mut t = String::new();
                    escape_html_body_text(&mut t, &text)?;
                    let alphabet: &HashMap<usize, char> =
                        match (self.strike, self.emphasis, self.strong) {
                            (false, false, false) => &MONO_C,        // default
                            (true, false, false) => &MONO_C,         // strike
                            (false, false, true) => &BOLD_CURSIVE_C, // strong
                            (true, false, true) => &BOLD_FRAKTUR_C,  // strike strong
                            (false, true, false) => &CURSIVE_C,      // emphasis
                            (true, true, false) => &FRAKTUR_C,       // strike emphasis
                            (false, true, true) => &CIRCLE_C,        // emphasis strong
                            (true, true, true) => &DOUBLE_C,         // strike emphasis strong
                        };
                    let u = t
                        .chars()
                        .map(|c| {
                            if let Some(i) = ALL_I.get(&c) {
                                alphabet[i]
                            } else {
                                c
                            }
                        })
                        .collect::<String>();
                    let u = if self.strike && !(self.emphasis || self.strong) {
                        strike(&u)
                    } else {
                        u
                    };
                    self.write(&u)?;
                }
                Html(html) | InlineHtml(html) => {
                    self.write(&html)?;
                }
                SoftBreak => {
                    self.write_newline()?;
                }
                HardBreak => {
                    self.write("\n")?;
                }
                Rule => {
                    if !self.end_newline {
                        self.write("\n")?;
                    }
                    self.write("---\n\n")?;
                }
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    let number = *self.numbers.entry(name.clone()).or_insert(len);
                    write!(&mut self.writer, "[{}][^", number)?;
                    escape_html(&mut self.writer, &name)?;
                    self.write("]")?;
                }
                TaskListMarker(true) => {
                    self.write("* [X] ")?;
                }
                TaskListMarker(false) => {
                    self.write("* [ ] ")?;
                }
            }
        }
        if !self.end_newline {
            self.write("\n")?;
        }
        Ok(())
    }

    /// Writes the start of an HTML tag.
    fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        match tag {
            Tag::HtmlBlock => Ok(()),
            Tag::Paragraph => {
                if self.end_newline {
                    Ok(())
                } else {
                    self.write("\n")
                }
            }
            Tag::Heading {
                level,
                id: _,
                classes: _,
                attrs: _,
            } => {
                if self.end_newline {
                    self.end_newline = false;
                } else {
                    self.write("\n")?;
                }
                self.strong = true;
                write!(
                    &mut self.writer,
                    "{}",
                    match level {
                        HeadingLevel::H1 => "# ",
                        HeadingLevel::H2 => "## ",
                        HeadingLevel::H3 => "### ",
                        HeadingLevel::H4 => "#### ",
                        HeadingLevel::H5 => "##### ",
                        HeadingLevel::H6 => "###### ",
                    }
                )
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments;
                Ok(())
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.table_cell_index = 0;
                Ok(())
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                Ok(())
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => {
                        self.write("| ")?;
                    }
                    TableState::Body => {
                        self.write("| ")?;
                    }
                }
                Ok(())
            }
            Tag::BlockQuote => {
                if self.end_newline {
                    self.write("> ")
                } else {
                    self.write("\n> ")
                }
            }
            Tag::CodeBlock(_info) => {
                if !self.end_newline {
                    self.write_newline()?;
                }
                self.code = true;
                Ok(())
            }
            Tag::List(Some(start)) => {
                if !self.end_newline {
                    self.write("\n")?;
                }
                self.lists.push(List::Ordered);
                self.ol.push(Counter::new(start));
                Ok(())
            }
            Tag::List(None) => {
                if !self.end_newline {
                    self.write("\n")?;
                }
                self.lists.push(List::Unordered);
                Ok(())
            }
            Tag::Item => {
                if !self.end_newline {
                    self.write("\n")?;
                }
                match self.lists.last().unwrap() {
                    List::Ordered => {
                        let n = self.ol.last_mut().unwrap().next();
                        write!(&mut self.writer, "{n}. ")
                    }
                    List::Unordered => self.write("* "),
                }
            }
            Tag::Emphasis => {
                self.emphasis = true;
                Ok(())
            }
            Tag::Strong => {
                self.strong = true;
                Ok(())
            }
            Tag::Strikethrough => {
                self.strike = true;
                Ok(())
            }
            Tag::Link {
                link_type: _,
                dest_url,
                title,
                id: _,
            } => {
                self.link = Some(Link {
                    dest_url: dest_url.to_string(),
                    title: title.to_string(),
                });
                self.write("[")
            }
            Tag::Image {
                link_type: _,
                dest_url,
                title,
                id: _,
            } => {
                self.write("![")?;
                self.raw_text()?;
                self.write("](")?;
                escape_href(&mut self.writer, &dest_url)?;
                if title.is_empty() {
                    self.write(")")
                } else {
                    self.write(" \"")?;
                    escape_html(&mut self.writer, &title)?;
                    self.write("\")")
                }
            }
            Tag::FootnoteDefinition(name) => {
                if !self.end_newline {
                    self.write("\n")?;
                }
                let len = self.numbers.len() + 1;
                let number = *self.numbers.entry(name).or_insert(len);
                write!(&mut self.writer, "{}. ", number)
            }
            Tag::MetadataBlock(_) => {
                self.in_non_writing_block = true;
                Ok(())
            }
        }
    }

    fn end_tag(&mut self, tag: TagEnd) -> io::Result<()> {
        match tag {
            TagEnd::HtmlBlock => {}
            TagEnd::Paragraph => {
                self.write("\n\n")?;
            }
            TagEnd::Heading(_level) => {
                self.strong = false;
                self.write("\n\n")?;
            }
            TagEnd::Table => {
                self.write("\n")?;
            }
            TagEnd::TableHead => {
                self.table_state = TableState::Body;
                write!(
                    &mut self.writer,
                    "|\n|{}|\n",
                    self.table_alignments
                        .iter()
                        .map(|x| TableAlign::from(x).as_str().to_owned())
                        .collect::<Vec<String>>()
                        .join("|")
                )?;
            }
            TagEnd::TableRow => {
                self.write("|\n")?;
            }
            TagEnd::TableCell => {
                match self.table_state {
                    TableState::Head => {
                        self.write(" ")?;
                    }
                    TableState::Body => {
                        self.write(" ")?;
                    }
                }
                self.table_cell_index += 1;
            }
            TagEnd::BlockQuote => {
                self.write("\n")?;
            }
            TagEnd::CodeBlock => {
                self.code = false;
                self.write("\n")?;
            }
            TagEnd::List(true) => {
                self.write("\n")?;
            }
            TagEnd::List(false) => {
                self.write("\n")?;
            }
            TagEnd::Item => {
                self.write("\n")?;
            }
            TagEnd::Emphasis => {
                self.emphasis = false;
            }
            TagEnd::Strong => {
                self.strong = false;
            }
            TagEnd::Strikethrough => {
                self.strike = false;
            }
            TagEnd::Link => {
                let link = self.link.take().unwrap();
                escape_href(&mut self.writer, &link.dest_url)?;
                if !link.title.is_empty() {
                    self.write(" \"")?;
                    escape_html(&mut self.writer, &link.title)?;
                }
                self.write("\"]")?;
            }
            TagEnd::Image => (), // shouldn't happen, handled in start
            TagEnd::FootnoteDefinition => {
                self.write("\n")?;
            }
            TagEnd::MetadataBlock(_) => {
                self.in_non_writing_block = false;
            }
        }
        Ok(())
    }

    // run raw text, consuming end tag
    fn raw_text(&mut self) -> io::Result<()> {
        let mut nest = 0;
        while let Some(event) = self.iter.next() {
            match event {
                Start(_) => nest += 1,
                End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Html(_) => {}
                InlineHtml(text) | Code(text) | Text(text) => {
                    // Don't use escape_html_body_text here.
                    // The output of this function is used in the `alt` attribute.
                    escape_html(&mut self.writer, &text)?;
                    self.end_newline = text.ends_with('\n');
                }
                SoftBreak | HardBreak | Rule => {
                    self.write(" ")?;
                }
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    let number = *self.numbers.entry(name).or_insert(len);
                    write!(&mut self.writer, "[{}]", number)?;
                }
                TaskListMarker(true) => self.write("[x]")?,
                TaskListMarker(false) => self.write("[ ]")?,
            }
        }
        Ok(())
    }
}
