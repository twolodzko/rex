# `rex`: use Regular Expressions to eXtract fields from strings

`rex` is a simple command-line tool for extracting fields of strings using regular expressions. It relies on Rust's
[`Regex`] crate and its syntax for (Perl-style) regular expressions. The same can be achieved by using common
command-line applications like `sed` or `awk`, but `rex` uses a simpler syntax as you only need to define the regular
expression to extract the fields. The extracted fields are returned either as columns or JSON entries.

For example, the command below extracts three fields for permissions, filename, and extension and returns them as
columns.

```shell
$ ls -la | rex '([rwx-]+) .*(Cargo)\.([^ ]*)'
-rw-rw-r--      Cargo   lock
-rw-rw-r--      Cargo   toml
```

The capturing groups can be named and the `-j` flag marks that the output should be returned as JSON entries
(aka [JSON Lines] format).

```shell
$ ls -la | rex '(?P<permissions>[rwx-]+) .*(?P<name>Cargo)\.(?P<extension>[^ ]*)' -j 
{"extension":"lock","name":"Cargo","permissions":"-rw-rw-r--"}
{"extension":"toml","name":"Cargo","permissions":"-rw-rw-r--"}
```

Moreover, as the benchmark using the [IMDB dataset] shows, the code is faster than `sed` and `gawk`.

```shell
$ hyperfine --warmup 3 \
    "sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null" \
    "gawk 'match(\$0, /(199[0-9]|20[0-9]{2})?.*,(positive|negative)/, arr) { print arr[1], '\t' arr[2] }' IMDB\ Dataset.csv > /dev/null" \
    "rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null"
Benchmark 1: sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null
  Time (mean ± σ):      6.455 s ±  0.257 s    [User: 6.397 s, System: 0.057 s]
  Range (min … max):    6.178 s …  6.962 s    10 runs
 
Benchmark 2: gawk 'match($0, /(199[0-9]|20[0-9]{2})?.*,(positive|negative)/, arr) { print arr[1], '\t' arr[2] }' IMDB\ Dataset.csv > /dev/null
  Time (mean ± σ):      7.376 s ±  0.114 s    [User: 7.351 s, System: 0.025 s]
  Range (min … max):    7.246 s …  7.610 s    10 runs
 
Benchmark 3: rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null
  Time (mean ± σ):      1.651 s ±  0.034 s    [User: 1.598 s, System: 0.053 s]
  Range (min … max):    1.624 s …  1.736 s    10 runs
 
Summary
  rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null ran
    3.91 ± 0.17 times faster than sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null
    4.47 ± 0.11 times faster than gawk 'match($0, /(199[0-9]|20[0-9]{2})?.*,(positive|negative)/, arr) { print arr[1], '\t' arr[2] }' IMDB\ Dataset.csv > /dev/null
```


 [`Regex`]: https://docs.rs/regex/latest/regex/
 [IMDB dataset]: https://www.kaggle.com/datasets/lakshmi25npathi/imdb-dataset-of-50k-movie-reviews?resource=download
 [JSON Lines]: https://jsonlines.org/
