package handler

import (
	"git.kundeng.us/phoenix/textsender-models/pkg/user"
)

func GetTestUser() user.User {
	return user.User{Username: "ghost", PhoneNumber: "+1234567890", Password: "dfgdffddfd"}
}
