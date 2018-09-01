## PostgreSql

-   create database with owner

```bash
$ sudo su - postgres
$ psql
> CREATE USER who-am-i WITH PASSWORD 'change-me';
> CREATE DATABASE db-name WITH ENCODING='UTF8';
> GRANT ALL PRIVILEGES ON DATABASE db-name TO who-am-i;
```

-   'Peer authentication failed for user', open file "/etc/postgresql/9.5/main/pg_hba.conf" change line:

```bash
local   all             all                                     peer
TO:
local   all             all                                     md5
```
