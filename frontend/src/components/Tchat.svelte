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
import Message from "./Message.svelte";
import TchatHeader from "./TchatHeader.svelte";
import TchatInput from "./TchatInput.svelte";

export let roomId;
let messages_and_user_id = {
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
            user_id: messages_and_user_id.user_id,
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

    store.subscribe(incoming_messages_and_user_id => {
        messages_and_user_id = incoming_messages_and_user_id;
    });

    return store.close;
});
</script>

<div class="container mx-auto shadow-lg rounded-lg">
    <TchatHeader title={`Chat on ${roomId} as ${messages_and_user_id.user_id}`} messageCount={messages_and_user_id.messages.length} />
    <div class="flex flex-row justify-between bg-white">
        <div class="w-full px-5 flex flex-col justify-between">
            <div class="flex flex-col mt-5" bind:this={div}>
                {#each messages_and_user_id.messages as message, i}
                <Message alignRight={i % 2} {message} />
           {/each}
          </div>
            <TchatInput on:message={handleSendMessage} />
        </div>
    </div>
</div>

