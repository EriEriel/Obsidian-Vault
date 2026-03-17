# PostgreSQL Fundamentals
2026-03-13  #learn #postgresql #database #sql

## what is it
Core SQL syntax for querying, manipulating, and structuring relational data in PostgreSQL.

## how it works

### table management
```sql
CREATE TABLE users (
    id         SERIAL PRIMARY KEY,
    username   VARCHAR(50) UNIQUE NOT NULL,
    email      TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (username, email)
VALUES ('johndoe', 'john@example.com');
```

### querying
```sql
-- DISTINCT removes duplicates
SELECT DISTINCT column_name FROM table_name;

-- WHERE operators: =, !=, <, >, <=, >=, BETWEEN, IN, IS NULL
-- Logic: AND, OR, NOT

-- Pattern matching
-- LIKE  → case-sensitive,   % = any chars,  _ = one char
-- ILIKE → case-insensitive
-- [] character classes NOT supported — use SIMILAR TO or regex instead
```

### aggregation & grouping
```sql
-- Aggregate functions: COUNT(), SUM(), AVG(), MIN(), MAX()
SELECT COUNT(*) AS total_rows FROM users;

-- GROUP BY — any non-aggregated column in SELECT must be in GROUP BY
SELECT department, COUNT(*)
FROM employees
GROUP BY department;
```

### JOINs
```sql
-- INNER JOIN — only matching rows from both tables
SELECT articles.title, users.name
FROM articles
INNER JOIN users ON articles.author_id = users.id;

-- LEFT JOIN — all rows from left, matched rows from right (NULL if no match)
SELECT users.name, articles.title
FROM users
LEFT JOIN articles ON users.id = articles.author_id;
```

### utilities
```sql
-- LIMIT / OFFSET
SELECT * FROM users LIMIT 10 OFFSET 20;

-- ORDER BY
SELECT * FROM users ORDER BY created_at DESC;

-- EXTRACT date parts
SELECT EXTRACT(YEAR FROM created_at) FROM users;
```

## example
```sql
-- Top 3 departments by headcount
SELECT department, COUNT(*) AS headcount
FROM employees
GROUP BY department
ORDER BY headcount DESC
LIMIT 3;
```

## gotchas
- `LIKE` does NOT support `[]` character classes — use `SIMILAR TO` or `~` (regex) instead
- `ILIKE` is PostgreSQL-specific, not standard SQL
- `SERIAL` is shorthand for `INTEGER + SEQUENCE + DEFAULT` — Postgres-specific
- Any column in `SELECT` that isn't aggregated must appear in `GROUP BY`

## links
- [[postgresql-cli-cheatsheet]]
- [[prisma]]
