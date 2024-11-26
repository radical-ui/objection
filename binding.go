package objection

type Binding[T any] struct {
	Key   string `json:"key"`
	Child string `json:"child"`
}
