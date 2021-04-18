<script>
import {
    push
} from "svelte-spa-router";

let roomId = "";

const handleJoin = () => {
    push(`/room/${roomId}`);
};
</script>

<div>
    <div>
        <span>Room</span>
        <input bind:value={roomId} type="text" />
    </div>

    <button on:click={handleJoin}>Join !</button>
    <br>
</div>
