<script>
import Tchat from "../components/Tchat.svelte";

export let params = {};
</script>

<div class="flex h-screen justify-center items-center bg-gray-400">
    <div class="text-center">
        <Tchat roomId={params.room_id} />
    </div>
</div>
