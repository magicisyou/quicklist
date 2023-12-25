<script lang="ts">
  import Item from "./Item.svelte";
  import { Store } from "tauri-plugin-store-api";
  import { invoke } from "@tauri-apps/api/tauri";

  const store = new Store(".settings.dat");

  let item_name = "";

  $: items = [];

  async function addItem() {
    items = await invoke("add_item_to_list", { item: item_name });
    saveToStore();
  }

  async function saveToStore() {
    let items_json = await invoke("get_list_state");
    await store.set("list", { value: JSON.stringify(items_json) });
    store.save();
  }

  async function loadStore() {
    let val = await store.get("list");
    if (val) {
      let items_json = JSON.parse(val.value);
      items = await invoke("build_list_from_memory", { items: items_json });
    }
  }

  async function toggleChecked() {
    items = await invoke("toggle_checked_value", { itemName: this.id });
    saveToStore();
  }

  async function deleteItem() {
    items = await invoke("remove_item_from_list", { itemName: this.id });
    saveToStore();
  }

  loadStore();
</script>

<div>
  <form on:submit|preventDefault={addItem}>
    <input
      id="greet-input"
      placeholder="Enter an item to list"
      bind:value={item_name}
    />
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

<style scoped>
  .list {
    padding: 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 5px;
  }

  input,
  button {
    border: none;
    padding: 15px;
  }

  input {
    width: 300px;
  }
</style>
