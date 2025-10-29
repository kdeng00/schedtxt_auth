package version

import "fmt"

var (
	Version   = "dev"
	BuildTime = "unknown"
	Commit    = "unknown"
	GoVersion = "unknown"
)

func String() string {
	return fmt.Sprintf(
		"Version: %s\nBuild Date: %s\nCommit: %s\nGo Version: %s",
		Version, BuildTime, Commit, GoVersion,
	)
}
