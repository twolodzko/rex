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
$ hyperfine --warmup 3 "sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null" "gawk 'match(\$0, /(199[0-9]|20[0-9]{2})?.*,(positive|negative)/, arr) { print arr[1], '\t' arr[2] }' IMDB\ Dataset.csv > /dev/null" "rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null"
Benchmark 1: sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null
  Time (mean ± σ):      7.567 s ±  0.992 s    [User: 7.479 s, System: 0.087 s]
  Range (min … max):    6.561 s …  9.162 s    10 runs
 
Benchmark 2: gawk 'match($0, /(199[0-9]|20[0-9]{2})?.*,(positive|negative)/, arr) { print arr[1], '\t' arr[2] }' IMDB\ Dataset.csv > /dev/null
  Time (mean ± σ):      8.093 s ±  0.389 s    [User: 8.062 s, System: 0.029 s]
  Range (min … max):    7.673 s …  8.732 s    10 runs
 
Benchmark 3: rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null
  Time (mean ± σ):      1.677 s ±  0.017 s    [User: 1.624 s, System: 0.052 s]
  Range (min … max):    1.657 s …  1.705 s    10 runs
 
Summary
  rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null ran
    4.51 ± 0.59 times faster than sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null
    4.83 ± 0.24 times faster than gawk 'match($0, /(199[0-9]|20[0-9]{2})?.*,(positive|negative)/, arr) { print arr[1], '\t' arr[2] }' IMDB\ Dataset.csv > /dev/null
```


 [`Regex`]: https://docs.rs/regex/latest/regex/
 [IMDB dataset]: https://www.kaggle.com/datasets/lakshmi25npathi/imdb-dataset-of-50k-movie-reviews?resource=download
 [JSON Lines]: https://jsonlines.org/
