# PostgreSQL CLI Cheatsheet
2026-03-13  #learn #postgresql #database #cli

## what is it
Quick reference for managing the PostgreSQL service and `psql` CLI on Arch Linux.

## how it works

### service management
```bash
sudo systemctl start postgresql      # start
sudo systemctl stop postgresql       # stop
sudo systemctl restart postgresql    # restart
sudo systemctl status postgresql     # check if running
sudo systemctl enable postgresql     # auto-start on boot
```

### connect
```bash
sudo -u postgres psql                # connect as superuser
psql -U postgres -d mydb             # connect to specific database
\q                                   # quit psql
```

### inside psql
```bash
\l                                   # list all databases
\c dbname                            # switch to a database
\dt                                  # list tables in current db
\d tablename                         # describe a table
\du                                  # list users/roles
```

### add a new database from a dump
```bash
cd ~/Downloads
unzip dvdrental.zip
sudo -u postgres psql -c "CREATE DATABASE dvdrental;"
pg_restore -U postgres -d dvdrental ~/Downloads/dvdrental.tar
```

## gotchas
- On Arch, `pg_hba.conf` lives at `/var/lib/postgres/data/pg_hba.conf` — not the Debian path
- If auth method is `trust`, passwords are ignored — only username and database need to exist
- `:DBUI` in Neovim → Add connection → `postgresql://postgres:yourpassword@localhost:5432/dvdrental`

## links
- [[postgresql]]
- [[prisma]]
