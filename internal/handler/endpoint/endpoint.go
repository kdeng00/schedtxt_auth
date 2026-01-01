package endpoint

const (
	// Endpoint for registering a user
	Register       = "/api/v1/register"
	Login          = "/api/v1/login"
	UpdatePassword = "/api/v1/user/password/update"
	UpdateName     = "/api/v1/user/name/update"

	CreateServiceUser = "/api/v1/service/register"
	LoginServiceUser  = "/api/v1/service/login"

	TokenRefresh = "/api/v1/token/refresh"
)
