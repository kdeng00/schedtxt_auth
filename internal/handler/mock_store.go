package handler

import (
	"context"
	"errors"
	"sync"

	"github.com/google/uuid"

	"git.kundeng.us/phoenix/textsender-models/pkg/user"
)

type MockUserStore struct {
	Users           map[uuid.UUID]*user.User
	UsersByUsername map[string]*user.User
	mu              sync.RWMutex
	Error           error // Optional: simulate errors
}

func NewMockUserStore() *MockUserStore {
	return &MockUserStore{
		Users:           make(map[uuid.UUID]*user.User),
		UsersByUsername: make(map[string]*user.User),
	}
}

func (m *MockUserStore) CreateUser(ctx context.Context, user *user.User) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.Error != nil {
		return m.Error
	}

	if user.Id == uuid.Nil {
		user.Id = uuid.New()
	}

	if _, exists := m.UsersByUsername[user.Username]; exists {
		return errors.New("User with email already exists")
	}

	m.Users[user.Id] = user
	m.UsersByUsername[user.Username] = user
	return nil
}

func (m *MockUserStore) GetUserByID(ctx context.Context, id uuid.UUID) (*user.User, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.Error != nil {
		return nil, m.Error
	}

	if m.Error != nil {
		return nil, m.Error
	}

	user, exists := m.Users[id]
	if !exists {
		return nil, errors.New("User not found")
	}

	return user, nil
}

func (m *MockUserStore) GetUserByUsername(ctx context.Context, username string) (*user.User, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.Error != nil {
		return nil, m.Error
	}

	user, exists := m.UsersByUsername[username]
	if !exists {
		return nil, errors.New("User not found")
	}

	return user, nil
}

func (m *MockUserStore) GetAllUsers(ctx context.Context) ([]*user.User, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.Error != nil {
		return nil, m.Error
	}

	users := make([]*user.User, 0, len(m.Users))
	for _, user := range m.Users {
		users = append(users, user)
	}

	return users, nil
}

func (m *MockUserStore) UserExists(ctx context.Context, username string) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.Error != nil {
		return false, m.Error
	}

	_, exists := m.UsersByUsername[username]
	if !exists {
		return exists, errors.New("User not found")
	}

	return exists, nil
}
