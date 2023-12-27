<script lang="ts">
  import Item from "./Item.svelte";
  import SideBar from "./SideBar.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let list_names = [];
  let list_name = "";
  let items = [];
  let item_name = "";

  async function load_list_names() {
    let list = await invoke("get_list_names");
    if (list) {
      list_names = list;
    }
  }

  async function deleteList() {
    let response = await invoke("delete_list", {
      listName: this.previousElementSibling.innerText,
    });
    if (response) {
      list_names = response;
      if (list_name == this.previousElementSibling.innerText) {
        list_name = "";
      }
    }
  }

  async function addItem() {
    if (item_name != "") {
      let list = await invoke("add_item_to_list", {
        list: list_name,
        item: item_name,
      });
      if (list) {
        items = list;
        item_name = "";
      }
    }
  }

  async function loadData() {
    let list = await invoke("get_list", { listName: this.innerText });
    if (list) {
      list_name = this.innerText;
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
      itemName: this.previousElementSibling.innerText,
    });
    if (list) {
      items = list;
    }
  }
  load_list_names();
</script>

<div class="container">
  <SideBar {list_names} onClick={loadData} onDelete={deleteList} />
  <div class="right">
    {#if list_name != ""}
      <h3>{list_name}</h3>
      <form on:submit|preventDefault={addItem}>
        <input placeholder="Enter an item to list" bind:value={item_name} />
        <button type="submit">Add</button>
      </form>
      <div class="list">
        {#each items as item}
          {#key item.name}
            <Item
              name={item.name}
              checked={item.checked}
              onClick={toggleChecked}
              onDelete={deleteItem}
            />
          {/key}
        {/each}
      </div>
    {/if}
  </div>
</div>

<style scoped>
  .container {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    justify-content: space-between;
    width: 100%;
  }
  .right {
    width: 70vw;
  }
  .list {
    margin-top: 10px;
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
