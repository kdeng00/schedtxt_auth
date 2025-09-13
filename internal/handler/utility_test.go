package handler

import (
	"git.kundeng.us/phoenix/textsender-auth/internal/model"
)

func GetTestUser() model.User {
	return model.User{Username: "ghost", PhoneNumber: "+1234567890", Password: "dfgdffddfd"}
}
