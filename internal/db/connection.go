// db/connection.go
package db

import (
	"context"
	"fmt"
	"log"
	"os"
	"path"
	"strings"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"
)

var Pool *pgxpool.Pool

type Database struct {
	Pool *pgxpool.Pool
}

func NewDatabase(connString string) (*Database, error) {
	ctx := context.Background()
	// Parse the connection string and create pool configuration
	config, err := pgxpool.ParseConfig(connString)
	if err != nil {
		return nil, fmt.Errorf("unable to parse DSN: %v", err)
	}

	// Configure connection pool settings
	config.MaxConns = 25
	config.MinConns = 5
	config.MaxConnLifetime = time.Hour
	config.MaxConnIdleTime = 30 * time.Minute
	config.HealthCheckPeriod = time.Minute

	// Configure connection timeouts
	config.ConnConfig.ConnectTimeout = 10 * time.Second
	config.ConnConfig.RuntimeParams["timezone"] = "UTC"

	// Create the connection pool
	pool, err := pgxpool.NewWithConfig(ctx, config)
	if err != nil {
		return nil, fmt.Errorf("unable to create connection pool: %v", err)
	}

	// Test the connection with a short timeout
	if err := pool.Ping(ctx); err != nil {
		pool.Close()
		return nil, fmt.Errorf("unable to ping database: %v", err)
	}

	log.Printf("Successfully connected to database")
	return &Database{Pool: pool}, nil
}

func (db *Database) Close() {
	if db.Pool != nil {
		db.Pool.Close()
		log.Println("Database connection pool closed")
	}
}

// HealthCheck verifies the database connection is still alive
func (db *Database) HealthCheck(ctx context.Context) error {
	return db.Pool.Ping(ctx)
}

// GetPoolStats returns connection pool statistics
func (db *Database) GetPoolStats() string {
	stats := db.Pool.Stat()
	return fmt.Sprintf("TotalConns: %d, IdleConns: %d, AcquiredConns: %d",
		stats.TotalConns(), stats.IdleConns(), stats.AcquiredConns())
}

func (db *Database) ResetDatabase(ctx context.Context) error {
	tx, err := db.Pool.Begin(ctx)
	if err != nil {
		return fmt.Errorf("Transaction unable to begin: %v", err)
	}
	defer tx.Rollback(ctx)

	schemaContent, err := os.ReadFile("migrations/schema.sql")
	if err != nil {
		log.Println("Default migrations not found. Checking different directory")
		cwd, _ := os.Getwd()
		migrationsPath := path.Join(cwd, "../..", "migrations/schema.sql")
		schemaContent, err = os.ReadFile(migrationsPath)
		if err != nil {
			return fmt.Errorf("error reading schema file: %v", err)
		}
	}

	for _, stmt := range strings.Split(string(schemaContent), ";") {
		stmt = strings.TrimSpace(stmt)
		if stmt == "" {
			continue
		}

		if strings.HasPrefix(stmt, "--") {
			continue
		}

		_, err := tx.Exec(ctx, stmt)
		if err != nil {
			return fmt.Errorf("error executing SQL statement: %v\nStatement: %s", err, stmt)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("unable to commit transaction: %v", err)
	}

	log.Println("Database schema applied successfully")

	return nil
}
