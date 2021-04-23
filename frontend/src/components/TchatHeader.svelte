<script>
export let messageCount = 0;
</script>

<div class="px-5 py-5 flex justify-between items-center bg-white border-b-2">
    <div class="w-full">
        <div class="text-2xl font-semibold">SSE Chat</div>
        <span class="text-gray-500">Received {messageCount > 1 ? `${messageCount} messages` : `${messageCount} message`}</span>
    </div>
</div>
