import { writable } from 'svelte/store';

export const createChannelStore = (roomId) => {
	const { subscribe, set, update } = writable([]);

	const eventSource = new EventSource(`http://localhost:3000/room/${roomId}/listen`);

	eventSource.onmessage = (e) => {
		update((messages) => messages.concat(JSON.parse(e.data)));
	};

	return {
		subscribe,
		reset: () => set([]),
		close: eventSource.close
	};
};
