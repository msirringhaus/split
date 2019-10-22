# split

Tiny binary to split strings in bash and extract certain columns with the ability to squash empty columns.

It exists because `cut` can't remove empty columns and I'm too stupid to remember awk-commands.

## Usage

```
# Without giving a delimiter, it splits all whitespace
$ echo "How did that awk-command work again?" | split -c 4
awk-command

# Default-joining with one whitespace
echo "I'm................thinking" | split "."
I'm thinking

# Can rejoin with a different join-delimiter (which can be more than one char)
$ echo "How did that awk-command work again?" | split -c 1,2,3,5,6 -j '==='
How===did===that===work===again?

# Can split multi-char delimiter
$ echo "I was...like...thinking...like...how did you...like...come up with that?" | split -c 3,4 "...like..."
how did you come up with that?

# Squashes empty entries..
$ echo "I'm................thinking" | split -c 2 "."
thinking

# ..or not
$ echo "I'm................thinking" | split -k -c 17 "."
thinking

# Has the ability to use negative indices
$ echo "
Part1..................Something
Part10.................Another
Part100................Thing
Part1000...............End
" | split "." -j ": " -c 1,-1

Part1: Something
Part10: Another
Part100: Thing
Part1000: End

# Respects requested order of fields
$ echo "1 2 3 4 5 6" | cut -d " " -f 3,2,1
1 2 3
$ echo "1 2 3 4 5 6" | split -c 3,2,1
3 2 1
```
