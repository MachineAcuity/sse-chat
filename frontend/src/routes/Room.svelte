<script>
import ChatRoom from "../components/ChatRoom.svelte";

export let params = {};
</script>

<div class="flex justify-center items-center  h-screen w-screen  bg-gray-400">
    <ChatRoom roomId={params.room_id} />
</div>
