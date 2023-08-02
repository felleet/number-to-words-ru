# number-to-words-ru

## How to use

Add crate to your project:

```shell
cargo add number-to-words
```

Then use:

```rust
let words = number_to_words(7_812_842);

println!("{words}");
```

output:

```
семь миллионов восемьсот двенадцать восемьсот сорок два
```

## Limitations

- Support value range: 0 - 999_999_999

## Roadmap

1. Support billions
