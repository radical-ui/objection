# Objection Specification

There are two components to the protocol. The [network section](#network) outlines how a frontend is to communicate with
the backend. The [building section](#building) outlines how a frontend is to interact with the objection cli, in order
to provide a smooth development experience.

## Network

Before a frontend displays any relevant information*, it must first connect to the backend using the following
procedure.

1. The frontend must open a websocket connection to the backend. The exact path at which the frontend requests, is
   irrelevant, except that it should be able to be customized by the backend author via a
   [build configuration](#configurations).

   - The frontend should append a `session_id` query parameter onto the url. This _must_ be a uuid, and should be unique
     to the current [ui session](#ui-session) of the frontend application. The purpose of this is so that the backend
     may seamlessly revive a session that was dropped due to a bad network connection.

   - If the server did [set an `auth_token`](#setting-an-auth-token), the frontend should also append this token to the
     url as a query param, under the name of `auth_token`.

2. Unless the websocket handshake itself is malformed, the backend must complete it. If something is wrong, and
   connection cannot continue (eg. the server encountered an error, the session id was not a uuid, etc.), the backend
   must send down an [`aknowledge` message](#downstream-aknowledge) with a `request_id` of `null` and an appropriate
   `error` set, subsequently closing the connection. The frontend should display this error to the user.

3. The backend must send down representations of the frontend's [initial objects](#initialobjects) using
   [`set_object` messages](#downstream-setobject). The backend may send down new representations of these objects at any
   time, but the frontend must not send any messages up to the backend until these initial objects are recieved for the
   first time.

4. The frontend should request other objects from the backend via [`watch_object` messages](#upstream-watchobject), as
   they are needed. However, the frontend must not request any object that is not specified as object reference (see the
   `reference` type in [schema](#schema)) in an object which has already been recieved. Thus all objects ever referenced
   anywhere must descend from the frontend's [initial objects](#initialobjects). The backend may not send down any
   objects that were not requested by the frontend.

5. The backend may keep track of all the objects that have been watched, and beam down new representations of those
   objects (via [`set_object` messages](#downstream-setobject)) as the underlying data they depend upon is updated. When
   recieving such an update, the frontend must re-render any UI corresponding to the updated object and update any
   caches it may have for the purpose of offline support.

6. If [bound data] was changed on the frontend, it must beam up an [`emit_binding_update` message].

7. If an object can no longer displayed for some reason (eg. if it's underlaying data was deleted), the backend should
   beam down a [`remove_object` message]. The frontend must respond to this operation as gracefully as possible, within
   reason.

8. When the frontend no longer requires the use of an object, it must notify the backend via an
   [`unwatch_object` message](#upstream-unwatchobject). This is done in order to free up resources on the backend, so
   that the backend can be aware that updates are no longer necessary.

9. In response to any message sent up by the frontend, the backend must respond with an
   [`aknowledge` message](#downstream-aknowledge), referencing the frontend's [`request_id`](#requestid).

10. The frontend may close the websocket connection for any reason, but if the same ui session is still desired, should
    try to reconnect within a reasonable amount if time using the same `session_id` that was originally used to open the
    connection. If the server has forgotten the session (eg. an error occurred or resources needed to be freed up), it
    should act as if the session were brand new, responding with the frontend's [initial objects](#initialobjects). The
    frontend may attempt to resume at it's previous state by watching the objects that it was watching before the
    session was reset. This is not in violation of the rule forbidding the client from asking for objects not referenced
    in existing objects because those objects would've been referenced, just in a previous ui session.

> Note(*): Some frontends may have offline support, and it is not in violation of this principal for a frontend to rely
> on cached objects when a connection cannot be established.

### Setting an Auth Token

TODO

### Ui Session

A session that lasts for the lifetime of the in-memory frontend state. In most cases, this is from the time the
application was started to the time it stopped, and it's memory was cleaned up.

### Data Bindings

TODO

### Messages

The different types of messages that are sent between the frontend and backend. Downstream messages are to be sent from
the backend to the frontend, and upstream messages are to be sent from the frontend to the backend.

All messages must be json encoded, and will contain a `$` field, which can be matched upon to determine the kind of
message.

Optional values may be set to their appropriate value, or may be `null`, or may be omitted entirely.

#### `request_id`

All upstream messages contain a `request_id` field. This must be a uuid. Once the backend has processed the message, it
must send down an [`aknowledge` message]. The client should use this process to inform the user of pending operations,
when it would be noticable.

#### Upstream `watch_object`

Sent when the frontend would like the representation of an object. The backend should then emit a
[`set_object` message](#downstream-setobject).

- `$` - Must be the string, `watch_object`.

- `id` - A string, the id of the object to watch. This id must have been referenced in an preceeding object.

#### Upstream `unwatch_object`

Sent when the frontend no longer needs the representation of a previously-watch object.

- `$` - Must be the string, `watch_object`.

- `id` - A string, the id of the object to unwatch. This id must have been previously watched.

#### Upstream `emit_binding_update`

Sent when the frontend would like to update the content of a [bound value](#data-bindings).

- `$` - Must be the string, `emit_binding_update`.

- `key` - A string, the value that was specificed in the `key` field of the binding.

- `data` - Json, the value that the binding is to be updated to. This must match the type of the original data.

#### Downstream `aknowledge`

Sent once the message referenced by `request_id` was processed.

- `$` - Must be the string, "aknowledge"

- `request_id` (optional) - A uuid, encoded as a string. This is the particular upstream message that the backend is
  aknowleging. If this property is not specified, the backend is aknowledging the websocket connection itself, something
  which it is not required to do, and would generally only do in the case of an error, after which the connection is
  terminated by the backend (see point 2 in the [network section](#network)).

- `error` (optional) - A string. Sent by the backend

- `retry_after_seconds` (optional) - A string. Should only appear with `error`, a suggestion from the backend as to when
  the request might be retried successfully.

#### Downstream `set_object`

Sent when an object is watched by the client, or if already watched (and not unwatched), when the underlying data was
updated, resulting in a different representation.

- `$` - Must be the string, `set_object`.

- `id` - A string, the id of the object that is being set. This id must be referenced as an initial object id, or have
  been requested by the frontend via a `watch_object` message.

- `data` - The object data, in json. This must match the [object schema](#schema).

**Example**

```jsonc
{
  "$": "set_object",
  "id": "some_object_id",
  "data": {
    "some_object_field": "value"
    // ...
  }
}
```

#### Downstream `remove_object`

Sent when an object that is currently watched (and not unwatch), when the underlying data was deleted.

- `$` - Must be the string, `remove_object`.

- `id` - A string, the id of the object that is to be removed. This object must have already been sent to the frontend
  via a [`set_object` message](#downstream-setobject)

## Building

TODO

### Schema

TODO

#### `object`

TODO

#### `initial_objects`

TODO

### Configurations

TODO
