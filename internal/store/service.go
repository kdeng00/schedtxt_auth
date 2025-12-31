package store

import (
	"context"
	"fmt"
	"time"

	"git.kundeng.us/phoenix/textsender-models/tx0/user"
	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

// TODO: Rename this to ServiceUserStore
type ServiceStore interface {
	CheckWithUsername(ctx context.Context, username string) (bool, error)
	GetWithUsername(ctx context.Context, username string) (*user.ServiceUser, error)
	GetWithId(ctx context.Context, id uuid.UUID) (*user.ServiceUser, error)
	Create(ctx context.Context, serviceUser *user.ServiceUser) error
	UpdateLastLogin(ctx context.Context, id uuid.UUID, lastLogin time.Time) (int64, error)
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

func (s *PGServiceStore) GetWithId(ctx context.Context, id uuid.UUID) (*user.ServiceUser, error) {
	var serviceUser user.ServiceUser
	query := `SELECT id, username, passphrase, created FROM service_users WHERE id = $1`

	if err := s.db.QueryRow(ctx, query, id).Scan(&serviceUser.Id, &serviceUser.Username, &serviceUser.Passphrase, &serviceUser.Created); err != nil {
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

func (s *PGServiceStore) UpdateLastLogin(ctx context.Context, id uuid.UUID, lastLogin time.Time) (int64, error) {
	query := `UPDATE service_users SET last_login = $1 WHERE id = $2`
	if affected, err := s.db.Exec(ctx, query, lastLogin, id); err != nil {
		return 0, err
	} else {
		return affected.RowsAffected(), nil
	}
}
