## MySql

-   forgot mysql root password

create file  /tmp/reset.mysqld

```sql
SET PASSWORD FOR root@localhost = PASSWORD('change-me');
```

edit file /etc/mysql/my.cnf

```text
[mysqld]
init-file=/tmp/reset.mysqld
```
