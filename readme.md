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
```

WITH再帰のSQL
```
WITH RECURSIVE r AS (
SELECT id, name AS path, 1 AS depth FROM directories WHERE id = 1
UNION ALL
SELECT directories.id, r.path || directories.name as path, r.depth + 1 FROM directories INNER JOIN r ON directories.parent = r.id
)
SELECT id,path,depth FROM r order by path;
```