<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }

  async function fuck(){
    console.log("Starting");
    await invoke("start_bt_server", { port: "COM4" });
  }

  async function stop() {
    console.log("Stopping");
    await invoke("stop_bt_server");
  } 
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <button on:click|preventDefault={fuck}>Start</button>
  <button on:click|preventDefault={stop}>Stop</button>

  <p>{greetMsg}</p>
</div>