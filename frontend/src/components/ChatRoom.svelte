<script>
import {
    onMount,
    beforeUpdate,
    afterUpdate
} from "svelte";

import {
    api_url
} from '../API';
import {
    createChannelStore
} from "../channel/store";
import ChatPreviousMessage from "./ChatPreviousMessage.svelte";
import ChatRoomHeader from "./ChatRoomHeader.svelte";
import ChatNewMessage from "./ChatNewMessage.svelte";

export let roomId;
let chat_state = {
    messages: [],
    user_id: 1
}

let div;
let autoscroll;

beforeUpdate(() => {
    autoscroll =
        div && div.offsetHeight + div.scrollTop > div.scrollHeight - 20;
});

afterUpdate(() => {
    if (autoscroll) div.scrollTo(0, div.scrollHeight);
});

const handleSendMessage = async e => {
    await fetch(api_url + `/room/${roomId}/send`, {
        body: JSON.stringify({
            message: e.detail.text,
            user_id: chat_state.user_id,
            time: Date.now()
        }),
        headers: {
            "Content-Type": "application/json"
        },
        method: "POST"
    });
};

onMount(() => {
    const store = createChannelStore(roomId);

    store.subscribe(incoming_chat_state => {
        chat_state = incoming_chat_state;
    });

    return store.close;
});
</script>

<div class="container max-w-2xl shadow-lg rounded-lg">
    <ChatRoomHeader roomId={roomId} userId={chat_state.user_id} messageCount={chat_state.messages.length} />
    <div class="flex flex-row justify-between bg-white">
        <div class="flex-grow" />
        <div class="w-full px-5 flex flex-col justify-between">
            <div class="flex flex-col mt-5 overflow-y-auto" style="min-height:300px; max-height: calc(100vh - 280px)" bind:this={div}>
                {#each chat_state.messages as message, i}
                <ChatPreviousMessage {message} thisUserId={chat_state.user_id} />
           {/each}
        
          </div>
            <ChatNewMessage on:message={handleSendMessage} />
        </div>
    </div>
</div>

