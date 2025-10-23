# textsender-auth
A service that handles the authorization aspect of the textsender project.


## Getting started
Assumes that the postgresql database has already been created with privileges to
create and drop databases. Copy the `.env.sample` file to `.env`. Within the `.env`
file, update the database keys. The `SECRET_KEY` is used for token generation.


### Building api
```
go build -o textsender-auth cmd/api/main.go
```


### Resetting the database
```
./textsender-auth -reset-db
```
