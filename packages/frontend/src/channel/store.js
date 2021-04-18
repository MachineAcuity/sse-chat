import { writable } from 'svelte/store';

export const createChannelStore = (roomId) => {
	const { subscribe, set, update } = writable({ messages: [], user_id: 0 });

	const eventSource = new EventSource(`http://localhost:3000/room/${roomId}/listen`);

	eventSource.onmessage = (e) => {
		console.log(e);
		update((messages_and_user_id) => ({
			messages: messages_and_user_id.messages.concat(JSON.parse(e.data)),
			user_id: messages_and_user_id.user_id
		}));
	};

	eventSource.addEventListener('user', (e) => {
		update((messages_and_user_id) => ({
			messages: messages_and_user_id.messages,
			user_id: parseInt(e.data, 10)
		}));
	});

	return {
		subscribe,
		reset: () => set([]),
		close: eventSource.close
	};
};
