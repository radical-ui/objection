package objection

import (
	"errors"
	"log/slog"
	"math/rand"
)

type Object struct {
	kind       string
	resetKey   string
	attributes map[string]any
	children   []Object
}

type Frontend struct {
	stack []Object
}

func (self *Frontend) GetCurrentObject() *Object {
	if len(self.stack) == 0 {
		slog.Error("Cannot get current object because there is nothing on the stack")

		return &Object{}
	}

	return &self.stack[len(self.stack)-1]
}

func (self *Frontend) SetAttribute(name string, value any) error {
	object := self.GetCurrentObject()
	object.attributes[name] = value

	return nil
}

func (self *Frontend) GetCurrentResetKey() (string, error) {
	currentObject := self.GetCurrentObject()

	if len(currentObject.resetKey) == 0 {
		currentObject.resetKey = randSeq(10)
	}

	return currentObject.resetKey, nil
}

func (self *Frontend) StartNewObject(kind string) {
	self.stack = append(self.stack, Object{kind: kind})
}

func (self *Frontend) FinishObject() error {
	if len(self.stack) == 0 {
		return errors.New("Cannot finish object because there are not objects on the stack")
	}

	if len(self.stack) == 1 {
		return nil
	}

	lastElement := self.stack[len(self.stack)-1]
	secondLastElement := &self.stack[len(self.stack)-2]

	secondLastElement.children = append(secondLastElement.children, lastElement)
	self.stack = self.stack[:len(self.stack)-1]

	return nil
}

var letters = []rune("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")

func randSeq(n int) string {
	b := make([]rune, n)
	for i := range b {
		b[i] = letters[rand.Intn(len(letters))]
	}
	return string(b)
}
