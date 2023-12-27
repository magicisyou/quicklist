<script lang="ts">
  import Item from "./Item.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let list_name = "list1";
  let item_name = "";

  $: items = [];

  async function addItem() {
    if (item_name != "") {
      let list = await invoke("add_item_to_list", {
        list: list_name,
        item: item_name,
      });
      if (list) {
        items = list;
      }
    }
  }

  async function loadData() {
    let list = await invoke("get_list", { listName: list_name });
    if (list) {
      items = list;
    }
  }

  async function toggleChecked() {
    let list = await invoke("toggle_checked_value", {
      listName: list_name,
      itemName: this.innerText,
    });
    if (list) {
      items = list;
    }
  }

  async function deleteItem() {
    let list = await invoke("remove_item_from_list", {
      listName: list_name,
      itemName: this.id,
    });
    if (list) {
      items = list;
    }
  }
  loadData();
</script>

<div class="container">
  <div>
    <form on:submit|preventDefault={addItem}>
      <input placeholder="Enter an item to list" bind:value={item_name} />
      <button type="submit">Add</button>
    </form>
    <div class="list">
      {#each items as item}
        {#key item.name}
          <Item
            id={item.name}
            name={item.name}
            checked={item.checked}
            onClick={toggleChecked}
            onDelete={deleteItem}
          />
        {/key}
      {/each}
    </div>
  </div>
</div>

<style scoped>
  .container {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    justify-content: space-around;
    width: 100%;
  }
  .list {
    padding: 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 5px;
  }

  input,
  button {
    border: solid #888 1px;
    padding: 15px;
  }

  input {
    width: 400px;
  }

  @media (prefers-color-scheme: dark) {
    button,
    input {
      background-color: #333;
      color: #f2f2f2;
      border: solid #888 1px;
    }
  }
</style>
