# `rex`: use Regular Expressions to eXtract fields from a string

`rex` uses [Rust's `Regex` crate] and its syntax for regular expressions. 
 
```shell
$ ls -la | ./rex '(?P<permissions>[rwx-]+) .*(?P<name>Cargo)\.(?P<extension>[^ ]*)' -j 
{"extension":"lock","name":"Cargo","permissions":"-rw-rw-r--"}
{"extension":"toml","name":"Cargo","permissions":"-rw-rw-r--"}
```

```shell
$ ls -la | ./rex '([rwx-]+) .*(Cargo)\.([^ ]*)'
-rw-rw-r--      Cargo   lock
-rw-rw-r--      Cargo   toml
```

```shell
$ hyperfine --warmup 3 \
        "sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null" \
        "rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null"
Benchmark 1: sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null
  Time (mean ± σ):      6.425 s ±  0.563 s    [User: 6.375 s, System: 0.049 s]
  Range (min … max):    6.006 s …  7.846 s    10 runs
 
Benchmark 2: rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null
  Time (mean ± σ):      1.775 s ±  0.170 s    [User: 1.690 s, System: 0.084 s]
  Range (min … max):    1.598 s …  2.033 s    10 runs
 
Summary
  rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null ran
    3.62 ± 0.47 times faster than sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null
```


 [Rust's `Regex` crate]: https://docs.rs/regex/latest/regex/
