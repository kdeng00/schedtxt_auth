package mock

import (
	"context"
	"errors"
	"fmt"
	"sync"

	"git.kundeng.us/phoenix/textsender-models/tx0/user"
	"github.com/google/uuid"
)

type MockServiceUserStore struct {
	ServiceUsers           map[uuid.UUID]*user.ServiceUser
	ServiceUsersByUsername map[string]*user.ServiceUser
	mu                     sync.RWMutex
	Error                  error // Optional: simulate errors
}

func NewMockServiceUserStore() *MockServiceUserStore {
	return &MockServiceUserStore{
		ServiceUsers:           make(map[uuid.UUID]*user.ServiceUser),
		ServiceUsersByUsername: make(map[string]*user.ServiceUser),
	}
}

func (m *MockServiceUserStore) Create(ctx context.Context, user *user.ServiceUser) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.Error != nil {
		return m.Error
	}

	if user.Id == uuid.Nil {
		user.Id = uuid.New()
	}

	if _, exists := m.ServiceUsersByUsername[user.Username]; exists {
		return errors.New("service User with username already exists")
	}

	m.ServiceUsers[user.Id] = user
	m.ServiceUsersByUsername[user.Username] = user
	return nil
}

func (m *MockServiceUserStore) CheckWithUsername(ctx context.Context, username string) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.Error != nil {
		return false, m.Error
	}

	var exists bool

	for _, serviceUser := range m.ServiceUsers {
		if serviceUser.Username == username {
			exists = true
			break
		}
	}

	if !exists {
		return exists, nil
	} else {
		return exists, nil
	}
}

func (m *MockServiceUserStore) GetWithUsername(ctx context.Context, username string) (*user.ServiceUser, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.Error != nil {
		return nil, m.Error
	}

	serviceUser := m.ServiceUsersByUsername[username]
	if serviceUser != nil {
		return serviceUser, nil
	} else {
		return nil, fmt.Errorf("User not found")
	}
}

func (m *MockServiceUserStore) GetWithId(ctx context.Context, id uuid.UUID) (*user.ServiceUser, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.Error != nil {
		return nil, m.Error
	}

	for _, serviceUser := range m.ServiceUsers {
		if serviceUser.Id == id {
			return serviceUser, nil
		}
	}

	return nil, nil
}
