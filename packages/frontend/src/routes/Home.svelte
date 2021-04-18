<script>
import {
    push
} from "svelte-spa-router";

let roomId = "";
let username = "";

const handleJoin = () => {
    push(`/room/${roomId}?user=${username}`);
};
</script>

 
<div>
    <div>
        <span>Channel</span>
        <input bind:value={roomId} type="text" />
    </div>
    <div>
        <span>Username</span>
        <input bind:value={username} type="text" />
    </div>
    <button on:click={handleJoin}>Join !</button>
    <br>
</div>
