<script>
import {
    push
} from "svelte-spa-router";

let roomId = "";

const handleJoin = () => {
    push(`/room/${roomId}`);
};
</script>

<div class="flex h-screen justify-center items-center bg-gray-400">
    <div class="text-center">
        <form class="m-4 flex">
            <input bind:value={roomId} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white" placeholder="Room name"/>
            <button on:click={handleJoin} class="px-8 rounded-r-lg bg-yellow-400  text-gray-800 font-bold p-4 uppercase border-yellow-500 border-t border-b border-r">Join</button>
        </form>
    </div>
</div>