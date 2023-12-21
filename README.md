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
sudo ./pcstat --docker nginx
```

And, you will see the following output:
| path                                                                                                                                         | size    | pages | cached | uncached | percent           | timestamp  | mtime      |
|----------------------------------------------------------------------------------------------------------------------------------------------|---------|-------|--------|----------|-------------------|------------|------------|
| /var/lib/docker/overlay2/4fa86dacbbc38c540010c431ce78984e3001005212f6731c29bb299af172b969/diff/usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 | 210968  | 52    | 52     | 0        | 100               | 1703137199 | 1696062665 |
| /var/lib/docker/overlay2/4fa86dacbbc38c540010c431ce78984e3001005212f6731c29bb299af172b969/diff/usr/lib/x86_64-linux-gnu/libcrypt.so.1.1.0    | 206776  | 51    | 51     | 0        | 100               | 1703137199 | 1673045857 |
| /var/lib/docker/overlay2/4fa86dacbbc38c540010c431ce78984e3001005212f6731c29bb299af172b969/diff/usr/lib/x86_64-linux-gnu/libc.so.6            | 1922136 | 470   | 470    | 0        | 100               | 1703137199 | 1696062665 |
| /var/lib/docker/overlay2/4fa86dacbbc38c540010c431ce78984e3001005212f6731c29bb299af172b969/diff/usr/lib/x86_64-linux-gnu/libz.so.1.2.13       | 121280  | 30    | 30     | 0        | 100               | 1703137199 | 1667651086 |
| /var/lib/docker/overlay2/ce2a5e72e92bf51a55f1beaf7fd7460e403f8edeb51e3fe13b340a7c26c62032/diff/usr/sbin/nginx                                | 1536808 | 376   | 372    | 4        | 98.93617021276596 | 1703137199 | 1698163831 |
| /var/lib/docker/overlay2/ce2a5e72e92bf51a55f1beaf7fd7460e403f8edeb51e3fe13b340a7c26c62032/diff/usr/lib/x86_64-linux-gnu/libcrypto.so.3       | 4713752 | 1151  | 976    | 175      | 84.79582971329279 | 1703137199 | 1698083542 |
| /var/lib/docker/overlay2/4fa86dacbbc38c540010c431ce78984e3001005212f6731c29bb299af172b969/diff/usr/lib/x86_64-linux-gnu/libpcre2-8.so.0.11.2 | 629384  | 154   | 99     | 55       | 64.28571428571429 | 1703137199 | 1672587846 |
| /var/lib/docker/overlay2/ce2a5e72e92bf51a55f1beaf7fd7460e403f8edeb51e3fe13b340a7c26c62032/diff/usr/lib/x86_64-linux-gnu/libssl.so.3          | 696352  | 171   | 106    | 65       | 61.98830409356725 | 1703137199 | 1698083542 |

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
