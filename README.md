# textsender-auth
A service that handles the authorization aspect of the textsender project.


## Getting started
Assumes that the postgresql database has already been created with privileges to
create and drop databases. Copy the `.env.sample` file to `.env`. Within the `.env`
file, update the database keys. The `SECRET_KEY` is used for token generation.


### Building api
```
make build
```


### Resetting the database
```
./textsender-auth -reset-db
```


Generate API documentation
```
go install github.com/swaggo/swag/cmd/swag@latest
go get -u github.com/swaggo/http-swagger/v2

swag init --generalInfo main.go --dir ./cmd/api,./internal/handler --output docs/ --parseDependency --parseInternal
```

The API documentation can be viewed from `http://localhost:9080/swagger/index.html`.
