package objection

import (
	"errors"
	"fmt"
	"log/slog"
	"math/rand"
)

type object struct {
	Kind       string
	ResetKey   string
	Attributes any
	Children   []object
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

func (self *Frontend) GetCurrentResetKey() (string, error) {
	currentObject := self.getCurrentObject()

	if len(currentObject.ResetKey) == 0 {
		currentObject.ResetKey = randSeq(10)
	}

	return currentObject.ResetKey, nil
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

var letters = []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")

func randSeq(n int) string {
	b := make([]rune, n)
	for i := range b {
		b[i] = letters[rand.Intn(len(letters))]
	}
	return string(b)
}
