<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let chosenFile = "No file selected";

  async function browseFiles() {
    chosenFile = await invoke("file_dialogue", { selectFolder: false });
  }

  async function browseFolders() {
    chosenFile = await invoke("file_dialogue", { selectFolder: true });
  }

  async function convertFile() {
    await invoke("convert_file");
  }
</script>

<main class="container">
  <div id="chosenFile">{chosenFile}</div>
  <div id="browseGrid">
    <button id="browseFilesButton" class="button" on:click={browseFiles}>
      Browse Files</button
    >
    <button id="browseFoldersButton" class="button" on:click={browseFolders}
      >Browse Folders</button
    >
  </div>
  <button id="convertFile" class="button" on:click={convertFile}
    >Convert File</button
  >
  <div id="windowOutput" />
  <div id="gameVersionGrid">
    <div id="gameVersion">Game version:</div>
    <input id="gameVersionInput" type="text" />
  </div>
</main>

<style>
  #browseGrid {
    display: grid;
    grid-template-columns: 1fr 20px 1fr;
  }

  #gameVersionGrid {
    display: grid;
    grid-template-columns: 1fr 1fr;
  }

  #chosenFile {
    text-align: center;
    font-size: 16px;
    color: white;
    margin: 10px;
  }

  #browseFilesButton {
    grid-row: 1;
    grid-column: 1;
    margin-left: 20px;
  }

  #browseFoldersButton {
    grid-row: 1;
    grid-column: 3;
    margin-right: 20px;
  }

  #convertFile {
    text-align: center;
    /* visibility: hidden; */
    margin-top: 10px;
  }

  #windowOutput {
    padding: 10px;
    color: white;
    height: 300px;
  }

  #gameVersion {
    display: flex;
    justify-content: end;
    align-items: center;
    grid-row: 1;
    grid-column: 1;
    margin-right: 20px;
    color: white;
    font-size: 16px;
  }

  #gameVersionInput {
    grid-row: 1;
    grid-column: 2;
    font-size: 16px;
    color: white;
    width: 50px;
    height: 25px;
    background: none;
    border: none;
  }
</style>
