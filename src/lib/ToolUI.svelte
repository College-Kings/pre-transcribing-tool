<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    export let currentTool: string;

    //#region Episode
    let episode: number;
    invoke<number>("get_episode_number").then((value) => (episode = value));

    async function setEpisode(value: number) {
        await invoke("set_episode_number", { value: value });
    }

    async function episodeChange(event: Event) {
        let target = event.target as HTMLInputElement;
        await setEpisode(parseInt(target.value));
    }
    //#endregion

    //#region File
    let chosenFile = "No file selected";

    async function browseFiles() {
        chosenFile = await invoke("file_dialogue", { selectFolder: false });
    }

    async function browseFolders() {
        chosenFile = await invoke("file_dialogue", { selectFolder: true });
    }

    async function runWritingFormatter() {
        await invoke("run_writing_formatter");
    }
    
    async function convertFile() {
        await invoke("convert_file");
    }

    async function createRenderTable() {
        await invoke("create_render_table");
    }
    //#endregion
</script>

<div id="chosenFile">{chosenFile}</div>
<div id="browseGrid">
    <button id="browseFilesButton" class="button" on:click={browseFiles}
        >Browse Files</button
    >
    <button id="browseFoldersButton" class="button" on:click={browseFolders}
        >Browse Folders</button
    >
</div>
{#if currentTool === "writingFormatter"}
    <button class="button" on:click={runWritingFormatter}>Convert File</button>
{:else if currentTool === "fileFormatter"}
    <button class="button" on:click={convertFile}>Convert File</button>
{:else if currentTool === "renderTableCreator"}
    <button class="button" on:click={createRenderTable}
        >Create Render Table</button
    >
{/if}
<div id="windowOutput" />
<div id="gameVersionGrid">
    <div id="gameVersion">Episode Number:</div>
    <input
        id="gameVersionInput"
        type="text"
        bind:value={episode}
        on:change={episodeChange}
    />
</div>

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
