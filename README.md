# jlabel

HTS-styleのフルコンテキストラベルをRustで扱うためのクレート群です．

## Crates

### jlabel

フルコンテキストラベルを表現するデータ構造（struct）を含みます．
また，文字列へのシリアライザーと文字列からのパーサーが実装されています．

### jlabel-question

htsvoice等に含まれる「質問[^1]」のパーサーと，それを表現するデータ構造を含みます．

[jlabel](#jlabel-1)と併せて使うことで，フルコンテキストラベルが「質問」の条件に合致するかを
文字列を経由させずに判定できます．

[^1]:
    ワイルドカードを含む文字列で，フルコンテキストラベルが
    特定の条件に合致するかを判定するために使われています．

## Credits

@cm-ayf さんがコードの大部分を書いてくださいました．
この場を借りて感謝申し上げます．

また，フルコンテキストラベルや「質問」の仕様については，
[hts_engine API](https://hts-engine.sourceforge.net)，
[NIT ATR503 M001](http://hts.sp.nitech.ac.jp/?Download#u879c944)
を参考にしています．

## License

BSD 3-Clause License
