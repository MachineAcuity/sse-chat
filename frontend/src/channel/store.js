import { api_url } from '../API';
import { writable } from 'svelte/store';

export const createChannelStore = (roomId) => {
	const { subscribe, set, update } = writable({ messages: [], user_id: 0 });

	const eventSource = new EventSource(api_url + `/room/${roomId}/listen`);

	eventSource.onmessage = (e) => {
		console.log(e);
		update((chat_state) => ({
			messages: chat_state.messages.concat(JSON.parse(e.data)),
			user_id: chat_state.user_id
		}));
	};

	eventSource.addEventListener('user', (e) => {
		update((chat_state) => ({
			messages: chat_state.messages,
			user_id: parseInt(e.data, 10)
		}));
	});

	return {
		subscribe,
		reset: () => set([]),
		close: eventSource.close
	};
};
