# iskandar - Powerful tool for isucon

### `init`

```
USAGE:
    iskandar init

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

### `deploy`

```
USAGE:
    iskandar deploy [FLAGS]

FLAGS:
    -d, --dry        Dry run
    -h, --help       Prints help information
    -V, --version    Prints version information
    -w, --wo-pull    Without git pull
```

### `benchmark`

```
USAGE:
    iskandar benchmark [FLAGS]

FLAGS:
    -a, --access-log    Save nginx access log
    -c, --commit        Commit results
    -h, --help          Prints help information
    -s, --slow-log      Save mysql slow log
    -V, --version       Prints version information
    -v, --vmstat        Run vmstat
```


## nginx

```
USAGE:
    iskandar nginx [FLAGS] <SUBCOMMAND>

FLAGS:
    -d, --dry        Dry run
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    apply     Copy nginx_conf_file from project dir to nginx_conf_dir
    help      Prints this message or the help of the given subcommand(s)
    init      Copy nginx_conf_file from nginx_conf_dir to project dir, and make backup
    reload    Reload nginx
```

## mysql

```
USAGE:
    iskandar mysql [FLAGS] <SUBCOMMAND>

FLAGS:
    -d, --dry        Dry run
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    apply      Copy mysql_conf_file from project dir to mysql_conf_dir
    help       Prints this message or the help of the given subcommand(s)
    init       Copy mysql_conf_file from mysql_conf_dir to project dir, and make backup
    restart    Restart mysql
```