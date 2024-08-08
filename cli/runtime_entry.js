"IMPORTS";

if (!globalThis.window.OBJECTION) {
	throw new Error("JS should only be run from an objection-built platform");
}

const namespace = globalThis.window.OBJECTION;

namespace.selectComponentRenderer = (component) => {
	"COMPONENT_CASES";
};

const safeJoin = (path) =>
	path.map((item) => item.replaceAll(":", "\\:")).join("::");
namespace.getActionId = (actionKey) => safeJoin(actionKey.actionPath);
namespace.getEventId = (eventKey) => safeJoin(eventKey.eventPath);

namespace.actionListeners = new Map();

namespace.signalApplicationReady = async () => {
	await namespace.sendEvent({ eventPath: ["root_app_ready"] }, {
		token: localStorage.getItem("token"),
	});
};

namespace.sendEvent = async (key, data) => {
	if (!namespace.sessionId) namespace.sessionId = crypto.randomUUID();

	if (
		namespace.engineUrl.protocol !== "http:" &&
		namespace.engineUrl.protocol !== "https:"
	) {
		throw new Error(
			`'${namespace.engineUrl.protocol}' engineUrls are not supported`,
		);
	}

	const response = await fetch(namespace.engineUrl, {
		method: "POST",
		body: JSON.stringify({
			sessionId: namespace.sessionId,
			events: [{ key, data }],
		}),
		headers: { "content-type": "application/json" },
	})
		.catch(() => null);

	if (!response) {
		console.error("You appear to be offline. Retrying in 1s");
		await new Promise((resolve) => setTimeout(resolve, 1000));
		return await sendEvent(key, data);
	}
	if (!response.ok) throw new Error(await response.text());

	const actions = await response.json();

	for (const action of actions) {
		const listener = namespace.actionListeners.get(
			namespace.getActionId(action.key),
		);
		if (!listener) {
			throw new Error(
				`No action listener was specified for action: ${
					JSON.stringify(action, null, "\t")
				}`,
			);
		}

		listener(action.data);
	}
};

namespace.registerActionListener = (key, listener) => {
	const joinedKey = namespace.getActionId(key);

	namespace.actionListeners.set(joinedKey, listener);

	return () => {
		namespace.actionListeners.delete(joinedKey);
	};
};

namespace.mount = async () => {
	let mountData = null;

	namespace.registerActionListener(
		{ actionPath: ["root_mount"] },
		(data) => {
			mountData = data;
		},
	);

	await namespace.signalApplicationReady();

	if (!mountData) {
		throw new Error(
			"Engine did not send mount data when recieving the ready event",
		);
	}
	namespace.startRuntime(mountData);
};

namespace.startRuntime = createStarter();

if (namespace.existingState) namespace.startRuntime(namespace.existingState);
else namespace.mount();
