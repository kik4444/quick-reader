<!--
 *    This file is part of Quick Reader.
 *
 *    Quick Reader is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    Quick Reader is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU General Public License for more details.
 *
 *    You should have received a copy of the GNU General Public License
 *    along with Quick Reader.  If not, see <https://www.gnu.org/licenses/>.
 -->
<script lang="ts">
  import appData from "$stores/app_data";
  import { onMount } from "svelte";
  import splitText from "$lib/functions/splitter";
  import appSettings from "$stores/app_settings";

  $: $appData.chunks = splitText(
    $appData.text.trim().length > 0 ? $appData.text : "Quick Reader",
    $appData.chunkSize
  );

  $: speed = (1000 / ($appData.wpm / 60)) * $appData.chunkSize;

  $: durationSeconds =
    (speed *
      $appData.chunkSize *
      ($appData.chunks.length - $appData.currentIndex)) /
    1000 /
    $appData.chunkSize;

  $: duration = `${Math.floor((durationSeconds % 3600) / 60)}m ${Math.floor(
    durationSeconds % 60
  )}s`;

  let playing = false;
  let textarea: HTMLTextAreaElement;

  function reset() {
    $appData.wpm = 300;
    $appData.chunkSize = 1;
  }

  function stop() {
    playing = $appData.textareaLocked = false;
    $appData.currentIndex = 0;
    window.getSelection()?.empty();

    // We must unfocus the textfield after stopping play to avoid the user accidentally
    // deleting text if they press space after playing has finished
    textarea.blur();
  }

  function restart() {
    $appData.currentIndex = 0;
  }

  function togglePlaying() {
    if (playing) {
      playing = false;
    } else {
      playing = $appData.textareaLocked = true;

      // On Windows we must focus the textfield otherwise the highlight is not visible
      textarea.focus();
    }
  }

  function step() {
    if (playing) {
      if ($appData.currentIndex < $appData.chunks.length - 1) {
        ++$appData.currentIndex;
      } else {
        stop();
      }

      const selectionStart = $appData.chunks[$appData.currentIndex]!.startPos;
      const selectionStop = $appData.chunks[$appData.currentIndex]!.stopPos + 1;

      textarea.setSelectionRange(selectionStart, selectionStop);
    }

    setTimeout(step, speed);
  }

  onMount(step);

  function changedWpm(e: Event) {
    const input = e.target as HTMLInputElement;

    $appData.wpm = Math.min(
      parseInt(input.max),
      Math.max(parseInt(input.min), input.valueAsNumber)
    );
  }

  function changedChunkSize(e: Event) {
    const input = e.target as HTMLInputElement;

    const newChunkSize = Math.min(
      parseInt(input.max),
      Math.max(parseInt(input.min), input.valueAsNumber)
    );

    // Recalculate what the new chunk index should be after recreating the text chunks with a different size
    $appData.currentIndex = Math.floor(
      ($appData.currentIndex * $appData.chunkSize) / newChunkSize
    );

    $appData.chunkSize = newChunkSize;
  }

  function shortcutPressed(event: KeyboardEvent) {
    switch (event.code) {
      case "ArrowLeft":
        $appData.currentIndex = Math.max(
          $appData.currentIndex - $appSettings.playback.jumpBackChunks,
          0
        );
        break;

      case "ArrowRight":
        $appData.currentIndex = Math.min(
          $appData.currentIndex + $appSettings.playback.jumpForwardChunks,
          $appData.chunks.length - 1
        );
        break;

      case "Space":
        if ($appData.textareaLocked || textarea !== document.activeElement) {
          event.preventDefault();
          togglePlaying();
        }
        break;
    }
  }
</script>

<svelte:window on:keydown="{shortcutPressed}" />

<main>
  <!-- We should use readonly when we want to lock the textarea
         because Windows cannot highlight text it is disabled
    -->
  <textarea
    placeholder="Enter text to read quickly."
    readonly="{$appData.textareaLocked}"
    bind:this="{textarea}"
    bind:value="{$appData.text}"
    style="font-size: {$appSettings.fonts.textareaFontSize}pt;
    font-family: {$appSettings.fonts.textareaFontStyle}"></textarea>

  <p
    class="display"
    style="font-size: {$appSettings.fonts.displayFontSize}pt;
    font-family: {$appSettings.fonts.displayFontStyle}"
  >
    {$appData.chunks[$appData.currentIndex]?.chunk}
  </p>

  <div class="controls">
    <div class="chunking">
      <p>Words per minute:</p>
      <input
        type="number"
        min="60"
        max="1000"
        step="10"
        value="{$appData.wpm}"
        on:change="{changedWpm}"
      />
      <p>Chunk size:</p>
      <input
        type="number"
        min="1"
        max="10"
        value="{$appData.chunkSize}"
        on:change="{changedChunkSize}"
      />
      <button on:click="{reset}"><img src="/reset.svg" alt="" />Reset</button>
    </div>

    <div class="progress">
      <p>
        Chunk {$appData.currentIndex + 1} of {Math.floor(
          $appData.chunks.length / $appData.chunkSize
        )}
      </p>
      <div class="vertical-separator"></div>
      <p>Duration: {duration}</p>
      <div class="vertical-separator"></div>
      <input
        class="progress"
        type="range"
        min="0"
        max="{$appData.chunks.length - 1}"
        bind:value="{$appData.currentIndex}"
      />
    </div>

    <div class="playback">
      <button on:click="{stop}" disabled="{!$appData.textareaLocked}"
        ><img src="/stop.svg" alt="" />Stop</button
      >

      <button on:click="{restart}" disabled="{!$appData.textareaLocked}"
        ><img src="/restart.svg" alt="" />Restart</button
      >

      <button on:click="{togglePlaying}"
        ><img src="{playing ? '/pause.svg' : '/play.svg'}" alt="" />{playing
          ? "Pause"
          : "Start"}</button
      >
    </div>
  </div>
</main>

<style>
  main {
    height: 100%;
    display: grid;
    grid-template-rows: 35% 45% 20%;
  }

  textarea {
    padding: 15px 0;
    text-align: center;
  }

  p.display {
    text-align: center;
    align-self: center;
  }

  div.controls {
    align-self: end;
  }

  div.controls p {
    font-weight: bold;
    font-size: 9pt;
  }

  div.chunking {
    display: flex;
    gap: 5pt;
    justify-content: center;
    align-items: center;
  }

  div.progress {
    display: flex;
    gap: 3pt;
  }

  input.progress {
    flex-grow: 2;
  }

  div.vertical-separator {
    width: 1px;
    height: 30px;
    align-self: center;
    background-color: rgba(252, 252, 252, 0.3);
  }

  input {
    font-size: 13pt;
  }

  div.playback {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 5pt;
  }

  button {
    font-size: 10pt;
    height: 22pt;
  }
</style>
