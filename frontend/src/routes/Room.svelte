<script>
import Tchat from "../components/Tchat.svelte";

export let params = {};
// <div class="flex h-screen w-screen p-0 justify-center items-center bg-gray-400">
</script>

<div class="flex justify-center items-center  h-screen w-screen bg-gray-400">
    <Tchat roomId={params.room_id} />
</div>
