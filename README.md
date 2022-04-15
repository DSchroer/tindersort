# TinderSort
> Sort like a millennial

Sorts any list by comparing pairs individually. Assumes that each new line is a new item.

## Usage
```
tindersort 0.1.0
Dominick Schroer <dominick@schroer.ca>
Sort like a millennial

USAGE:
    tindersort [OPTIONS] <FILE>

ARGS:
    <FILE>    File to sort

OPTIONS:
    -f, --fast           Compare any item in the list instead of adjacent items
    -h, --help           Print help information
    -p, --print          Only print the options
    -s, --seed <SEED>    RNG seed to use
    -V, --version        Print version information
```

To build run `make`.
To install run `make install`.

## Use Cases

*Sort 10 items in a list of names:*
```bash
set -e
for i in {1..10}; do
  tindersort names.txt
done
```

*Display a DMENU UI to sort 10 items in a list of names:*
```bash
set -e
for i in {1..10}; do
  tindersort names.txt -s ${i}${$} -f -p | dmenu -l 2 -p "who is better?" | tindersort names.txt -s ${i}${$} -f
done
```
