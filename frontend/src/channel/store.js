import { api_url } from '../API';
import { writable } from 'svelte/store';

export const createChannelStore = (roomId) => {
	const { subscribe, set, update } = writable({ messages: [], user_id: 0 });

	const eventSource = new EventSource(api_url + `/room/${roomId}/listen`);

	// Receive a message from server
	eventSource.onmessage = (e) => {
		const received_message = JSON.parse(e.data);
		const message_data = Object.assign(
			{ must_show_date: 'date', grouping: 'only' },
			{
				user_id: received_message.user_id,
				message: received_message.message,
				dtTime: new Date(received_message.time)
			}
		);

		update((chat_state) => {
			// We create a copy because we will be modifying it.
			const { messages } = chat_state;
			const messagesNew = [ ...messages ];

			// Determine must_show_date and grouping based on previous messages
			// If there are no previous messages, then the defauls are the correct settings
			if (messages.length > 0) {
				var previous_message_data = messages[messages.length - 1];

				// Time in milliseconds
				const previous_ms = previous_message_data.dtTime.getTime();
				const current_ms = message_data.dtTime.getTime();

				// Is the date of this message and the previous message the same?
				const previous_day_ms = previous_ms - previous_ms % 86400000;
				const current_day_ms = current_ms - current_ms % 86400000;
				if (previous_day_ms === current_day_ms) {
					// Has the minute field changed by more than one digit?
					if (
						current_ms - current_ms % 60000 - (previous_ms - previous_ms % 60000) >
						60000
					) {
						// More than one minute difference, show time
						message_data.must_show_date = 'time';
					} else {
						// Within one minute, we will not show time since we think of it
						// simply as continuation of the dialog
						message_data.must_show_date = false;
					}
				}
				// else:  Different date, show both date and time - default

				// Determine grouping. It depends on whether the current user and previous user are the same
				const usersIsSame = message_data.user_id === previous_message_data.user_id;

				if (usersIsSame) {
					message_data.grouping = 'last';
					if (previous_message_data.grouping === 'only') {
						previous_message_data.grouping = 'first';
					} else if (previous_message_data.grouping === 'last') {
						previous_message_data.grouping = 'betwixt';
					}
				}
			}

			// Add the new message. Notice that the last message might have been modified already
			messagesNew.push(message_data);

			return {
				messages: messagesNew,
				user_id: chat_state.user_id
			};
		});
	};

	// Receive user_id from server
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
