<script>
import ChatUserIcon from "./ChatUserIcon.svelte";

export let thisUserId = 0
export let message = {
    user_id: 0,
    message: "",
    time: Date.now()
};
</script>

{#if thisUserId === message.user_id}

<div class="flex justify-end mb-4">
    <div>
        <div
            class="mr-2 py-3 px-4 bg-blue-400 rounded-bl-3xl rounded-tl-3xl rounded-tr-xl text-white"
            >
            {message.message}
        </div>
        <div class="text-right mr-2  text-xs text-gray-400">
            {new Date(message.time).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
        </div>
    </div>
    <ChatUserIcon userId={message.user_id} {thisUserId} />
</div>

{:else}

<div class="flex justify-start mb-4">
    <ChatUserIcon userId={message.user_id} />
    <div>
        <div
            class="ml-2 py-3 px-4 bg-gray-400 rounded-br-3xl rounded-tr-3xl rounded-tl-xl text-white"
            >
            {message.message}
        </div>
        <div class="text-left ml-2  text-xs text-gray-400">
            {new Date(message.time).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
        </div>
    </div>
</div>

{/if}
