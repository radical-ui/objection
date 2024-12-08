package objection

import (
	"errors"
	"fmt"
	"log/slog"
)

type object struct {
	Kind       string
	ResetKey   string
	Attributes any
	Children   []object
}

type FrontendRef struct {
	Current *Frontend
}

type Frontend struct {
	CurrentChildrenFunc func()
	stack               []object
	nodes               []object
}

func (self *Frontend) getCurrentObject() *object {
	if len(self.stack) == 0 {
		slog.Error("Cannot get current object because there is nothing on the stack")

		return &object{}
	}

	return &self.stack[len(self.stack)-1]
}

func (self *Frontend) SetAttributes(value any) error {
	object := self.getCurrentObject()
	object.Attributes = value

	return nil
}

// Creates a reset in the current object. Will overrite an existing reset if it exists. Use `GetCurrentReset` to get the key of
// the newly created reset.
func (self *Frontend) SetResetOnCurrentObject(key string) {
	currentObject := self.getCurrentObject()
	currentObject.ResetKey = key
}

// Gets the nearest reset key in the object or a parent, and an index that represents the number of parents away the key is
// (0, therefore means that the key was found in the current object). The root reset key is always called "root", so
// if no reset keys can be found, "root" will be returned.
func (self *Frontend) GetCurrentResetKey() (string, int) {
	if len(self.stack) == 0 {
		return "root", 0
	}

	for index := len(self.stack) - 1; index >= 0; index-- {
		key := self.stack[index].ResetKey

		if len(key) != 0 {
			return key, len(self.stack) - index - 1
		}
	}

	return "root", len(self.stack)
}

func (self *Frontend) StartNewObject(kind string) {
	self.stack = append(self.stack, object{Kind: kind})
}

func (self *Frontend) FinishObject() error {
	if len(self.stack) == 0 {
		return errors.New("Cannot finish object because there are not objects on the stack")
	}

	if len(self.stack) == 1 {
		self.nodes = append(self.nodes, self.stack[0])
		self.stack = []object{}

		return nil
	}

	lastElement := self.stack[len(self.stack)-1]
	secondLastElement := &self.stack[len(self.stack)-2]

	secondLastElement.Children = append(secondLastElement.Children, lastElement)
	self.stack = self.stack[:len(self.stack)-1]

	return nil
}

func (self *Frontend) GetData() (any, error) {
	if len(self.stack) != 0 {
		return nil, fmt.Errorf("cannot get frontend data because there are %d object(s) on the sack", len(self.stack))
	}

	return self.nodes, nil
}
