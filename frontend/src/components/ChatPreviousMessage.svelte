<script>
    import ChatUserIcon from "./ChatUserIcon.svelte";

export let alignRight = false;
export let message = {
    user_id: 0,
    message: "",
    time: Date.now()
};
</script>

{#if alignRight}
<div class="flex justify-end mb-4">
    <div
        class="mr-2 py-3 px-4 bg-blue-400 rounded-bl-3xl rounded-tl-3xl rounded-tr-xl text-white"
        >
        {`${new Date(message.time).toLocaleDateString()} ${new Date(message.time).toLocaleTimeString()}`}
        &nbsp; &nbsp;
        {message.user_id}
        &nbsp; &nbsp;
        {message.message}
    </div>
    <ChatUserIcon userId={message.user_id} /> 
</div>

{:else}

<div class="flex justify-start mb-4">
    <ChatUserIcon userId={message.user_id} /> 
    <div
        class="ml-2 py-3 px-4 bg-gray-400 rounded-br-3xl rounded-tr-3xl rounded-tl-xl text-white"
        >
        {message.user_id}
        &nbsp;
        {`${new Date(message.time).toLocaleDateString()} ${new Date(message.time).toLocaleTimeString()}`}
        &nbsp;
        {message.message}
    </div>
</div>

{/if}
