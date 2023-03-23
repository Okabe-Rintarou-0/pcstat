# pcstat

A command-line tool written in Rust that helps you analyze the page cache status of a given process or files.

## How to use

```shell
cargo build
pcstat --pid <process pid> -f <file1> <file2> ... <fileN>
```

## Examples

The following example shows how to checkout the page cache status of process with pid `1`, and also the status of given files.

command: 

```shell
echo "Hello world!" > test.txt
sudo ./pcstat --pid 1 -f test.txt
```

output(not complete): 

| path                                              | size    | pages | cached | uncached | percent            | timestamp  | mtime      |
|---------------------------------------------------|---------|-------|--------|----------|--------------------|------------|------------|
| test.txt                                          | 14      | 1     | 1      | 0        | 100                | 1679559177 | 1679545906 |
| /usr/lib/x86_64-linux-gnu/libapparmor.so.1.6.1    | 80736   | 20    | 20     | 0        | 100                | 1679559177 | 1589907589 |
| /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0       | 387768  | 95    | 95     | 0        | 100                | 1679559177 | 1644240815 |
| /usr/lib/x86_64-linux-gnu/libgcrypt.so.20.2.5     | 1168056 | 286   | 228    | 58       | 79.72027972027972  | 1679559177 | 1631644584 |

## How it works
+ use syscall `mmap`
+ use syscall `mincore`
+ use syscall `munmap`

## References

+ [tobert/pcstat](https://github.com/tobert/pcstat): golang versioned pcstat
+ [fasterthanlime/mincore](https://github.com/fasterthanlime/mincore): it teaches me how to use mincore syscall in rust
+ [zhiburt/tabled](https://github.com/zhiburt/tabled): a very nice table!
+ [mmap(2)](https://man7.org/linux/man-pages/man2/mmap.2.html): mmap manual
+ [munmap(2)](http://www.tin.org/bin/man.cgi?section=2&topic=munmap): munmap manual
+ [mincore](https://yitype.com/man/htmlman2/mincore.2.html): mincore manual
