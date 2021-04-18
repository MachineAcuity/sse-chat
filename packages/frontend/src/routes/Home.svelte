<script>
import {
    push
} from "svelte-spa-router";

let roomId = "";
let user_id = 0;

const handleJoin = () => {
    push(`/room/${roomId}?user=${user_id}`);
};
</script>

<div>
    <div>
        <span>Channel</span>
        <input bind:value={roomId} type="text" />
    </div>
    <div>
        <span>User ID</span>
        <input bind:value={user_id} type="text" />
    </div>
    <button on:click={handleJoin}>Join !</button>
    <br>
</div>
