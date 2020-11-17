$ curl -X POST -d "email=ichikawa.fumiya@gmail.com" localhost:3000/login

```
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at timestamp default now()
)
```