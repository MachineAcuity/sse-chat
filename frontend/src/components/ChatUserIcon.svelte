<script>
export let userId = 0
export let thisUserId = 0
export let grouping = 'only'
</script>

{#if grouping !== 'first' && grouping !== 'only'}

<div class="h-1 w-12 "/>

{:else if userId === 0}

<img
    src="images/components/ChatUserIcon-Server.jpeg"
    class="object-cover h-12 w-12 rounded-full"
    alt="User"
    />

{:else if userId === thisUserId}

<img
    src="images/components/ChatUserIcon-ThisUser.jpeg"
    class="object-cover h-12 w-12 rounded-full"
    alt="User"
    />

{:else}

<img
    src="images/components/ChatUserIcon-OtherUser.jpeg"
    class="object-cover h-12 w-12 rounded-full"
    alt="User"
    />

{/if}
