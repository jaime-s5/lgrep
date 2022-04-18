# Lgrep
## About the project
Lgrep is a reduced version of GNU grep. It allows to search a string in a file or recursively in the specified directory. It also highlights matches and they can be printed with a number of surrounding lines.

## Usage

Search in a file:

```
lgrep -s <STRING> -f <path/to/file>
```
Search recursively:

```
lgrep -s <STRING> -r <path/to/directory>
```
Specify number of surrounding lines to print:

```
lgrep -s <STRING> -r <path/to/directory> -n <NUMBER>
```
## Profiling

Just to have a reference, the command time was used in several cases to profile the peformance of lgrep vs gnu grep.

1. Search in just one file:
```
$ cat file | wc -l
1424603

$ time grep -Hn Regex::new file
real  0m0,206s
user  0m0,120s
sys   0m0,083s

$ time lgrep -s Regex::new -f file
real  0m0,598s
user  0m0,520s
sys   0m0,077s

$ time grep -Hn Regex::new file -C 2
real  0m0,629s
user  0m0,183s
sys   0m0,445s

$ time lgrep -s Regex::new -f file -n 2
real  0m0,936s
user  0m0,614s
sys   0m0,312s
```

2. Search recursively
```
$ find /path/to/dir/ | wc -l
30595

$ time let /path/to/dir -HnR
real  0m4,128s
user  0m2,201s
sys   0m1,257s

$ time lgrep -s let -r /path/to/dir/
real  0m5,361s
user  0m3,041s
sys   0m1,592s
```