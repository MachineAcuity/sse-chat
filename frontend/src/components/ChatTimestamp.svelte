<script>
export let position = ''
export let mustShowDate = false
export let dtTime = 0
</script>

{#if position === 'left' && mustShowDate === 'time'}

<div class="text-left ml-14  text-xs text-gray-400">
    {dtTime.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
</div>

{:else if position === 'left' && mustShowDate === 'date'}

<div class="text-left ml-14  text-xs text-gray-400">
    {dtTime.toLocaleDateString()}
    {dtTime.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
</div>

{:else if position === 'right' && mustShowDate === 'time'}

<div class="text-right mr-14  text-xs text-gray-400">
    {dtTime.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
</div>

{:else if position === 'right' && mustShowDate === 'date'}

<div class="text-right mr-14  text-xs text-gray-400">
    {dtTime.toLocaleDateString()}
    {dtTime.toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
</div>

{/if}
