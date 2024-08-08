const socket = new WebSocket("/dev.ws");

socket.onmessage = ({ data }) => {
    if (data === "reload") location.reload();
    else if (data === "remount") window.OBJECTION.mount();
};

socket.onclose = async () => {
    await pollTillOnline();
    location.reload();
};

async function pollTillOnline() {
    while (true) {
        try {
            await fetch("/");
            break;
        } catch (_) {
            await new Promise((resolve) => setTimeout(resolve, 1000));
        }
    }
}
