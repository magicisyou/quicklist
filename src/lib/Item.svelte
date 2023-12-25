<script lang="ts">
  export let name = "";
  export let id = "";
  export let checked = false;
  export let onClick = () => {};
  export let onDelete = () => {};
</script>

<div class="item">
  {#if checked}
    <button class="name" on:click|preventDefault={onClick} {id}>{name}</button>
  {:else}
    <button class="name-striked" on:click|preventDefault={onClick} {id}
      >{name}</button
    >
  {/if}
  <button class="remove-btn" on:click|preventDefault={onDelete} {id}>-</button>
</div>

<style scoped>
  .item {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    border-radius: 5px;
    background-color: #cdcdcd;
  }
  .name {
    text-align: left;
    width: 480px;
    border: none;
    background-color: transparent;
    padding: 10px;
  }
  .name-striked {
    color: #888;
    text-align: left;
    width: 480px;
    border: none;
    background-color: transparent;
    text-decoration: line-through;
  }
  .remove-btn {
    padding: 10px;
    border: none;
  }

  @media (prefers-color-scheme: dark) {
    .item {
      background-color: #555;
    }

    .name {
      color: #f2f2f2;
    }
  }
</style>
