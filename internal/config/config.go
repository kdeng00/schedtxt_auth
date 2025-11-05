package config

import (
	"flag"
	"fmt"
	"log"
	"os"
	"strconv"

	"github.com/joho/godotenv"

	"git.kundeng.us/phoenix/textsender-auth/internal/version"
)

type Config struct {
	DBConnString string
	ServerPort   string
	ResetDB      bool
}

type ConnectionInfo struct {
	Username string
	Password string
	Database string
	Host     string
	Port     int
	SslMode  string
}

const Port = "9080"
const App_Name = "textsender_auth"

func (ci ConnectionInfo) Parse() string {
	return fmt.Sprintf("postgres://%s:%s@%s:%d/%s?sslmode=%s", ci.Username, ci.Password, ci.Host, ci.Port, ci.Database, ci.SslMode)
}

func PrintName() {
	fmt.Println(App_Name)
	fmt.Println(version.String())
}

func Load() *Config {
	versionFlag := flag.Bool("version", false, "Print version information")
	resetDb := flag.Bool("reset-db", false, "Reset the database schema and exit")
	port := flag.String("port", Port, "Server port")
	flag.Parse()

	if *versionFlag {
		fmt.Println(version.String())
		os.Exit(-1)
	}

	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	unpackedConnString := UnpackDBConnString()
	dbConnString := unpackedConnString.Parse()

	return &Config{
		DBConnString: dbConnString,
		ServerPort:   *port,
		ResetDB:      *resetDb,
	}
}

func GetSecretKey() string {
	return os.Getenv("SECRET_KEY")
}

func UnpackDBConnString() (connInfo ConnectionInfo) {
	username := os.Getenv("DB_USER")
	password := os.Getenv("DB_PASSWORD")
	host := os.Getenv("DB_HOST")
	port := os.Getenv("DB_PORT")
	database := os.Getenv("DB_NAME")
	sslMode := os.Getenv("DB_SSLMODE")

	if username != "" {
		connInfo.Username = username
	} else {
		connInfo.Username = "user"
	}

	if password != "" {
		connInfo.Password = password
	} else {
		connInfo.Password = "password"
	}

	if host != "" {
		connInfo.Host = host
	} else {
		connInfo.Host = "localhost"
	}

	if port != "" {
		num, err := strconv.Atoi(port)
		if err != nil {
			return
		}
		connInfo.Port = num
	} else {
		connInfo.Port = 5432
	}

	if database != "" {
		connInfo.Database = database
	} else {
		connInfo.Database = "textsender_auth_db"
	}

	if sslMode != "" {
		connInfo.SslMode = sslMode
	} else {
		connInfo.SslMode = "disable"
	}

	return
}

func (c *Config) GetDBConnString() string {
	return c.DBConnString
}
