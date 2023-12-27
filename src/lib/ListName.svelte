<script lang="ts">
  export let list_name = "";
  export let onClick;
  export let onDelete;
</script>

<div class="main">
  <button class="subdiv" on:click|preventDefault={onClick}>{list_name}</button>
  <button class="delete-btn" on:click|preventDefault={onDelete}>-</button>
</div>

<style scoped>
  .main {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    background-color: #cdcdcd;
  }
  .delete-btn {
    background-color: #aaa;
    padding: 10px;
    border: none;
  }
  .subdiv {
    border: none;
    width: calc(100% - 30px);
    padding: 10px;
    text-align: left;
    background-color: inherit;
    color: inherit;
  }
  @media (prefers-color-scheme: dark) {
    .main {
      background-color: #555;
    }
  }
</style>
