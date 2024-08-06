# objection_cli

Parses the runtime, checks it for errors, then generates the bindings and platform-specific project.

## Development

Current structure:

- `Collection` - takes in the runtime url and gives out a list of components, and their associated types (referred to as
  "kinds").
- `convert_interface` and `convert_type` - are used by `Collection` to do the brute work of the kind conversions.
- `Inspector` - walks down the generated collection and verifies that several necessary conditions are met
- `RustGen` - flattens the Collection, attaches constructor and builder methods, and converts it into rust code

Ideal structure:

Collecting and converting are done quite well, and should _mostly_ stay the same. The biggest necessary change is that
collecting should not do as much of the verificaiton work. `Collection` should only collect, only generating diagnostics
for issues that occur while collecting. Additionally, the collection should retain source locations, so that future
diagnostic steps can provide more egonmic diagnostics.

`Inspector` should do much the same work as it does now, except that it should take on most of the diagnostic work that
`Collection` is currently doing. Additionally, it should carry more context as it walks through the trees so that
diagnostics can be better.

A new `Resolver` struct should be created. This would convert the more nested structure of `Collection` into the flatter
structure that is generated. Additioinally, the constructors and builder methods should be computed here.

After this, there is relatively little work for `GenRust` to do, but that is good becuase we will eventually want the
corresponding `GenGo`, `GenGleam`, etc.

### Technical Debt

- All diagnostic spitting actors should be taking a mutable reference to `DiagnosticList`.

### Future Runtime Goals

- `Collection` should be able to read object literals, interface extensions and type intersections. There need not be
  another kind created for extensions or intersections, it can simply treat them as additional properties in the current
  object. Additionally, `Collection` should support the introduction of metadata in the module doc. This is how the
  runtime's deno config will be referenced.

- `EventKey` and `ActionKey` should go away. Events can be inferred from function props of the closure type. Every prop
  should be able to become an action by listenting to changes on an action key.

- Module metadata should specify the type that represents a renderable ui (normally this would be `ReactNode`). Then
  there would be no need for the `@component` jsdoc tag. A component is any function that returns the afore specified ui
  node.

- The component index idea is good in the engine, but is unnecessarily complex in the fronted, and makes it harder
  create a new runtime because the pattern is rarely used in existing componentized frontend code. This is a possible
  solution:
  - If a component accepts children, it should have a property that is the ui type (see the previous point), or a
    function that returns the ui type. The CLI would then generate code that calls `loadComponent` and provides the
    result, which will be the ui type.
  - Instead of the runtime's `start` function accepting a component tree, it accepts a closure which returns a
    `loadComponent` result, as specified above.

With all of the changes stated above, runtime_lib would no longer need to be a thing that is imported, but rather,
injected dynamically at build time.

Most of these changes refer to the way that components are loaded, making it more ergonomic to write a runtime. Note
that:

- `loadComponent` is the function that a runtime would be required to export in addition to the `start` function.
- `emitEvent` is the same as is currently exported from the runtime lib
- `createActionSubscribers` is comparable to `registerActionListener`

```js
function loadComponentFromTree(component) {
  if (component.type === "SomeComponent") {
    return loadComponent(
      SomeComponent,
      {
        field: component.def.field,
        child: () => loadComponentFromTree(component.def.child),
        event: (data) => emitEvent(component.def.event, data),
      },
      createActionSubscribers(component.actionKey)
    );
  }
  
  else if (component.type === "OtherComponent") {
    // ...
  }
}

const initial = ...
start(() => loadComponentFromTree(initial))
```
