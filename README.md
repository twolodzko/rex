# `rex`: use Regular Expressions to eXtract fields from a string

`rex` uses [Rust's `Regex` crate] and its syntax for regular expressions. The same can be achieved by using common
command-line applications like `sed` or `awk`, but `rex` uses a simpler syntax as you only need to define the regular
expression to extract the fields. The extracted fields are returned either as columns or JSON entries.

For example, the command below extracts three fields for permissions, filename, and extension and returns them as
columns.

```shell
$ ls -la | rex '([rwx-]+) .*(Cargo)\.([^ ]*)'
-rw-rw-r--      Cargo   lock
-rw-rw-r--      Cargo   toml
```

The capturing groups can be named and the `-j` flag marks that the output should be returned as JSON entries.

```shell
$ ls -la | rex '(?P<permissions>[rwx-]+) .*(?P<name>Cargo)\.(?P<extension>[^ ]*)' -j 
{"extension":"lock","name":"Cargo","permissions":"-rw-rw-r--"}
{"extension":"toml","name":"Cargo","permissions":"-rw-rw-r--"}
```

Moreover, as the benchmark using the [IMDB dataset] shows, the code is faster than `sed`.

```shell
$ hyperfine --warmup 3 \
        "sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null" \
        "rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null"
Benchmark 1: sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null
  Time (mean ± σ):      7.428 s ±  0.678 s    [User: 7.350 s, System: 0.077 s]
  Range (min … max):    6.643 s …  8.838 s    10 runs
 
Benchmark 2: rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null
  Time (mean ± σ):      1.985 s ±  0.169 s    [User: 1.914 s, System: 0.070 s]
  Range (min … max):    1.708 s …  2.193 s    10 runs
 
Summary
  rex '(199[0-9]|20[0-9]{2})?.*,(positive|negative)' IMDB\ Dataset.csv > /dev/null ran
    3.74 ± 0.47 times faster than sed -E 's/(199[0-9]|20[0-9]{2})?.*,(positive|negative)/\1\t\2/' IMDB\ Dataset.csv > /dev/null
```


 [Rust's `Regex` crate]: https://docs.rs/regex/latest/regex/
 [IMDB dataset]: https://www.kaggle.com/datasets/lakshmi25npathi/imdb-dataset-of-50k-movie-reviews?resource=download
