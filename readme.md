$ curl -X POST -d "email=ichikawa.fumiya@gmail.com" localhost:3000/login

```
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at timestamp default now()
)
```

```
insert into directories (name, parent)values('/', 0);
insert into directories (name, parent)values('home/', 1);
insert into directories (name, parent)values('user1/', 2);
insert into directories (name, parent)values('user2/', 2);
insert into directories (name, parent)values('user3/', 2);
insert into directories (name, parent)values('workspace/', 4);
insert into directories (name, parent)values('var/', 1);
insert into directories (name, parent)values('opt/', 1);
insert into directories (name, parent)values('logs/', 7);
insert into directories (name, parent)values('oracle/', 8);
insert into directories (name, parent)values('httpd/', 9);
insert into directories (name, parent)values('app/', 9);

select * from directories;
 id |    name    | parent |         created_at         |         updated_at         
----+------------+--------+----------------------------+----------------------------
  1 | /          |      0 | 2020-11-17 11:10:44.731212 | 2020-11-17 11:10:44.731212
  2 | home/      |      1 | 2020-11-17 11:11:24.514859 | 2020-11-17 11:11:24.514859
  3 | user1/     |      2 | 2020-11-17 11:11:41.866475 | 2020-11-17 11:11:41.866475
  4 | user2/     |      2 | 2020-11-17 11:11:50.338142 | 2020-11-17 11:11:50.338142
  5 | user3/     |      2 | 2020-11-17 11:11:53.495925 | 2020-11-17 11:11:53.495925
  6 | workspace/ |      4 | 2020-11-17 11:12:07.943518 | 2020-11-17 11:12:07.943518
  7 | var/       |      1 | 2020-11-17 11:12:35.926546 | 2020-11-17 11:12:35.926546
  8 | opt/       |      1 | 2020-11-17 11:12:41.326963 | 2020-11-17 11:12:41.326963
  9 | logs/      |      7 | 2020-11-17 11:12:53.947549 | 2020-11-17 11:12:53.947549
 10 | oracle/    |      8 | 2020-11-17 11:13:23.380067 | 2020-11-17 11:13:23.380067
 11 | httpd/     |      9 | 2020-11-17 11:13:46.184615 | 2020-11-17 11:13:46.184615
 12 | app/       |      9 | 2020-11-17 11:13:52.890487 | 2020-11-17 11:13:52.890487
(12 rows)

```

WITH再帰のSQL
```
WITH RECURSIVE r AS (
SELECT id, name AS path, 1 AS depth FROM directories WHERE id = 1
UNION ALL
SELECT directories.id, r.path || directories.name as path, r.depth + 1 FROM directories INNER JOIN r ON directories.parent = r.id
)
SELECT id,path,depth FROM r order by path;

id |          path          | depth 
----+------------------------+-------
  1 | /                      |     1
  2 | /home/                 |     2
  7 | /var/                  |     2
  8 | /opt/                  |     2
  3 | /home/user1/           |     3
  4 | /home/user2/           |     3
  5 | /home/user3/           |     3
  9 | /var/logs/             |     3
 10 | /opt/oracle/           |     3
  6 | /home/user2/workspace/ |     4
 11 | /var/logs/httpd/       |     4
 12 | /var/logs/app/         |     4
```