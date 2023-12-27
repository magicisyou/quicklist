<script lang="ts">
  import ListName from "./ListName.svelte";

  export let list_names = [];
  export let onClick;
  export let onDelete;

  let list_name;

  function addList() {
    if (list_name != "" && !list_names.includes(list_name)) {
      list_names = [...list_names, list_name];
      list_name = "";
    }
  }
</script>

<div class="main">
  <form on:submit|preventDefault={addList}>
    <input placeholder="Enter List Name" bind:value={list_name} />
    <button type="submit">Create</button>
  </form>
  {#each list_names as list}
    <ListName list_name={list} {onClick} {onDelete} />
  {/each}
</div>

<style scoped>
  form {
    width: 100%;
    display: flex;
    flex-direction: column;
    margin-bottom: 20px;
  }

  input {
    border: solid #aaa 1px;
    padding: 10px;
  }

  button {
    border: solid #aaa 1px;
    padding: 10px;
  }
  .main {
    padding-top: 80px;
    width: 30vw;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    gap: 5px;
  }

  @media (prefers-color-scheme: dark) {
    input {
      background-color: inherit;
      color: #f2f2f2;
    }
  }
</style>
