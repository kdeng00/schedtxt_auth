package store

import (
	"context"
	"fmt"

	"git.kundeng.us/phoenix/textsender-models/pkg/user"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

type ServiceStore interface {
	CheckWithUsername(ctx context.Context, username string) (bool, error)
	GetWithUsername(ctx context.Context, username string) (*user.ServiceUser, error)
	Create(ctx context.Context, serviceUser *user.ServiceUser) error
}

type PGServiceStore struct {
	db *pgxpool.Pool
}

func NewServiceStore(db *pgxpool.Pool) *PGServiceStore {
	return &PGServiceStore{db: db}
}

func (s *PGServiceStore) CheckWithUsername(ctx context.Context, username string) (bool, error) {
	var exists bool
	query := `SELECT EXISTS(SELECT 1 FROM service_users WHERE Username = $1)`

	if err := s.db.QueryRow(ctx, query, username).Scan(&exists); err != nil {
		if err == pgx.ErrNoRows {
			return exists, nil
		} else {
			return exists, fmt.Errorf("Error querying row: %v", err)
		}
	} else {
		return exists, nil
	}
}

func (s *PGServiceStore) GetWithUsername(ctx context.Context, username string) (*user.ServiceUser, error) {
	var serviceUser user.ServiceUser
	query := `SELECT id, username, passphrase, created FROM service_users WHERE username = $1`

	if err := s.db.QueryRow(ctx, query, username).Scan(&serviceUser.Id, &serviceUser.Username, &serviceUser.Passphrase, &serviceUser.Created); err != nil {
		if err == pgx.ErrNoRows {
			return nil, nil
		} else {
			return nil, fmt.Errorf("Error querying row: %v", err)
		}
	} else {
		return &serviceUser, nil
	}
}

func (s *PGServiceStore) Create(ctx context.Context, serviceUser *user.ServiceUser) error {
	query := `
		INSERT INTO service_users (username, passphrase)
		VALUES ($1, $2)
		RETURNING id, created
	`

	return s.db.QueryRow(ctx, query, serviceUser.Username, serviceUser.Passphrase).Scan(
		&serviceUser.Id, &serviceUser.Created,
	)
}
