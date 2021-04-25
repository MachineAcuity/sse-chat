<script>
import ChatTimestamp from "./ChatTimestamp.svelte";
import ChatUserIcon from "./ChatUserIcon.svelte";

export let thisUserId = 0
export let message = {
    user_id: 0,
    message: "",
    dtTime: 0,
    must_show_date: false,
    grouping: 'only'
};
</script>

{#if thisUserId === message.user_id}

<div class="flex justify-end mb-1">
    <ChatTimestamp position="right" mustShowDate={message.must_show_date} dtTime={message.dtTime}/>
</div>

<div class={"flex justify-end " + (message.grouping === 'only' || message.grouping === 'last'? 'mb-4' : 'mb-1')}>
    <div
        class={"mr-2 py-3 px-4 bg-blue-400  text-white rounded-bl-3xl rounded-tl-3xl " + (message.grouping === 'only' || message.grouping === 'first' ? ' rounded-tr-3xl' : '')}
        >
        {message.message}
    </div>
    <ChatUserIcon userId={message.user_id} {thisUserId} grouping={message.grouping} />
</div>

{:else}

<div class="flex justify-start mb-1">
    <ChatTimestamp position="left" mustShowDate={message.must_show_date} dtTime={message.dtTime}/>
</div>

<div class={"flex justify-start " + (message.grouping === 'only' || message.grouping === 'last'? 'mb-4' : 'mb-1')}>
    <ChatUserIcon userId={message.user_id} {thisUserId} grouping={message.grouping} />
    <div
        class={"ml-2 py-3 px-4 bg-gray-400 text-white rounded-br-3xl rounded-tr-3xl " + (message.grouping === 'only' || message.grouping === 'first' ? ' rounded-tl-3xl' : '')}
        >
        {message.message}
    </div>
</div>

{/if}
