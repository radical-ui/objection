# runtime

TODO

## Unfinished

- `WindowRenderer` should be renamed to something like `RemoteSync` and should only handle the remote syncing part (perhaps notices also)
- Get rid of `Window` and create a `Title` component for displaying the title
- Put all icons in a `public/icons` directory
- Write docs
- Create a `Theme` component which will keep using twind for now (we'd want to move away from this in the long term)
- `ComponentRender` should have a index renderer provider for that last function to `start`
- Change the way updates work. Instead of doing everything in `WindowRenderer`, it is just calling an action of a particular component.
