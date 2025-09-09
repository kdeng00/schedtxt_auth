package model

import (
	"context"
	"fmt"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

type User struct {
	Id          uuid.UUID `json:"id"`
	PhoneNumber string    `json:"phone_number"`
	Username    string    `json:"username"`
	Password    string    `json:"password"`
}

type UserStore interface {
	CreateUser(ctx context.Context, user *User) error
	GetUserByID(ctx context.Context, id uuid.UUID) (*User, error)
	GetUserByUsername(ctx context.Context, username string) (*User, error)
	GetAllUsers(ctx context.Context) ([]*User, error)
	UserExists(ctx context.Context, email string) (bool, error)
}

type PGUserStore struct {
	db *pgxpool.Pool
}

func NewUserStore(db *pgxpool.Pool) *PGUserStore {
	return &PGUserStore{db: db}
}

func (s *PGUserStore) CreateUser(ctx context.Context, user *User) error {
	query := `
		INSERT INTO users (phone_number, username, password)
		VALUES ($1, $2, $3)
		RETURNING id, phone_number, username
	`

	return s.db.QueryRow(ctx, query, user.PhoneNumber, user.Username, user.Password).Scan(
		&user.Id, &user.PhoneNumber, &user.Username,
	)
}

func (s *PGUserStore) GetUserByID(ctx context.Context, id uuid.UUID) (*User, error) {
	query := `SELECT id, username, password, phone_number FROM users WHERE id = $1`

	var user User
	err := s.db.QueryRow(ctx, query, id).Scan(
		&user.Id, &user.Username, &user.Password, &user.PhoneNumber,
	)

	if err == pgx.ErrNoRows {
		return nil, nil
	}
	if err != nil {
		return nil, fmt.Errorf("getting user by ID: %w", err)
	}

	return &user, nil
}

func (s *PGUserStore) GetUserByUsername(ctx context.Context, username string) (*User, error) {
	query := `SELECT id, username, password, phone_number FROM users WHERE username = $1`

	var user User
	err := s.db.QueryRow(ctx, query, username).Scan(
		&user.Id, &user.Username, &user.Password, &user.PhoneNumber,
	)

	if err == pgx.ErrNoRows {
		return nil, nil
	}
	if err != nil {
		return nil, fmt.Errorf("getting user by ID: %w", err)
	}

	return &user, nil
}

func (s *PGUserStore) GetAllUsers(ctx context.Context) ([]*User, error) {
	query := `SELECT id, username, password, phone_number FROM users`

	rows, err := s.db.Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("querying all users: %w", err)
	}
	defer rows.Close()

	var users []*User
	for rows.Next() {
		var user User
		if err := rows.Scan(
			&user.Id, &user.Username, &user.Password, &user.PhoneNumber,
		); err != nil {
			return nil, fmt.Errorf("scanning user row: %w", err)
		}
		users = append(users, &user)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("iterating user rows: %w", err)
	}

	return users, nil
}

func (s *PGUserStore) UserExists(ctx context.Context, username string) (bool, error) {
	query := `SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)`

	var exists bool
	err := s.db.QueryRow(ctx, query, username).Scan(&exists)
	if err != nil {
		return false, fmt.Errorf("checking if user exists: %w", err)
	}

	return exists, nil
}
