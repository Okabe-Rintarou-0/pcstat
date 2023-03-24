# pcstat

A command-line tool written in Rust that helps you analyze the page cache status of a given process or files.

## How to use

+ build with cargo
    
    ```shell
    cargo build
    ```

+ checkout the help documentation
    ```
    Usage:
    pcstat [OPTIONS]

    Get page cache status.

    Optional arguments:
    -h,--help             Show this help message and exit
    -p,--pid PID          Pid of the process you want to checkout.
    -f,--files FILES      Files you want to checkout.
    -c,--children         With children processes.
    -s,--sort SORT        Sort results(descending order by default).
    --ge GE               Cache percentage of files should be greater than ge.
    --le LE               Cache percentage of files should be less than le.
    --docker DOCKER       Docker container name or id.
    --markdown            Markdown style table.
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

If you also want to checkout its children processes' page cache status, then add flag `--children`.

It also supports checking out the page cache status of a docker container(given its id or name), by adding flag `--docker`,
like: 
```shell
docker run -itd --name nginx nginx:latest
sudo ./pcstat --docker nginx --children --sort=asc
```

And, you will see the following output:
| path                                       | size    | pages | cached | uncached | percent           | timestamp  | mtime      |
|--------------------------------------------|---------|-------|--------|----------|-------------------|------------|------------|
| /usr/lib/x86_64-linux-gnu/libssl.so.1.1    | 598104  | 147   | 141    | 6        | 95.91836734693877 | 1679646074 | 1651600176 |
| /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1 | 2954080 | 722   | 693    | 29       | 95.98337950138504 | 1679646074 | 1651600176 |
| /lib/x86_64-linux-gnu/libc-2.31.so         | 2029560 | 496   | 487    | 9        | 98.18548387096774 | 1679646074 | 1645731760 |
| /lib/x86_64-linux-gnu/libz.so.1.2.11       | 108936  | 27    | 27     | 0        | 100               | 1679646074 | 1648318854 |
| /lib/x86_64-linux-gnu/libcrypt.so.1.1.0    | 202760  | 50    | 50     | 0        | 100               | 1679646074 | 1583857471 |
| /lib/x86_64-linux-gnu/libpthread-2.31.so   | 157224  | 39    | 39     | 0        | 100               | 1679646074 | 1645731760 |
| /lib/x86_64-linux-gnu/libnss_files-2.31.so | 51832   | 13    | 13     | 0        | 100               | 1679646074 | 1645731760 |
| /lib/x86_64-linux-gnu/libdl-2.31.so        | 18816   | 5     | 5      | 0        | 100               | 1679646074 | 1645731760 |

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
