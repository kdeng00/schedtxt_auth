# Getting started
Quickest way to get started is with docker, make sure that it is installed. Copy over
the `.env.docker.sample` file and name it `.env`.

For `SECRET_KEY` provide a key that is at least 32-characters. This is used for token
creation. Additionally, provide credentials for the database for the `DB_*` keys.

Make sure that `ENABLE_REGISTRATION` is set to true, otherwise registration will not work.

Build the container
```
docker compose build
```

Bring up the container
```
docker compose up
```
