<script lang="ts">
  let checked = $state(false)

  const share = (event: MouseEvent) => {
    console.log('share')
  }
</script>

<style>
  .note-container {
    width: 50%;
    background-color: white;
    height: 100%;
    border: 4px solid #f2dfcb;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    padding: 0 2.5vw 4vh 2.5vw;
  }
  .note-header {
    display: flex;
    flex-direction: row;
    align-items: center;
    padding-top: 1vh;
    padding-bottom: 1vh;
  }
  .text-box {
    width: 100%;
    height: 100%;
    background-color: antiquewhite;
  }
  .note-header-text {
    margin: 0;
    padding: 0;
    color: #8e8989;
    font-size: min(1rem, 2vw, 4vh);
  }
  .checkbox-container {
    position: relative;
    cursor: pointer;
    display: flex;
    flex-direction: row;
    margin-left: 1.5vw;
    align-items: center;
  }
  .checkbox-container input {
    position: absolute;
    opacity: 0;
    cursor: pointer;
    height: 0;
    width: 0;
  }
  .checkmark {
    position: absolute;
    top: 0px;
    left: 84px;
    height: 15px;
    width: 15px;
    border: 2px solid #8e8989;
    border-radius: 2px;
  }
  .checkbox-container input:checked ~ .checkmark {
    border-color: #5e5757;
    background-color: #5e5757;
  }
  .checkmark:after {
    content: '';
    position: absolute;
    display: none;
  }
  .checkbox-container input:checked ~ .checkmark:after {
    display: block;
  }
  .checked {
    color: #5e5757;
  }
  .checkbox-container .checkmark:after {
    left: 5px;
    top: 1px;
    width: 3px;
    height: 9px;
    border: solid white;
    border-width: 0 2px 2px 0;
    -webkit-transform: rotate(45deg);
    -ms-transform: rotate(45deg);
    transform: rotate(45deg);
  }
  .share-button {
    border-radius: 3px;
    padding: 3px;
    height: fit-content;
    margin-left: auto;
  }
  .share-button:hover {
    background-color: #ffe3c6;
    opacity: 65%;
  }
  .share-button:active {
    background-color: #ffe3c6;
  }
  @media screen and (max-width: 800px) {
    .checkmark {
      top: 4px;
      left: 10.5vw;
      height: 9px;
      width: 9px;
    }
    .checkbox-container .checkmark:after {
      left: 3px;
      top: -1px;
      width: 2px;
      height: 7px;
      border: solid white;
      border-width: 0 2px 2px 0;
    }
  }
  @media screen and (max-width: 700px) {
    .checkmark {
      top: 3px;
    }
  }
  @media screen and (max-width: 650px) {
    .checkmark {
      top: 1px;
    }
  }
  @media screen and (max-width: 600px) {
    .checkmark {
      top: 0px;
    }
  }
  @media screen and (max-height: 390px) {
    .checkmark {
      top: 3px;
      left: 21vh;
      height: 9px;
      width: 9px;
    }
    .checkbox-container .checkmark:after {
      left: 3px;
      top: -1px;
      width: 2px;
      height: 7px;
      border: solid white;
      border-width: 0 2px 2px 0;
    }
  }
  @media screen and (max-height: 350px) {
    .checkmark {
      top: 1px;
    }
  }
  @media screen and (max-height: 390px) and (max-width: 800px) {
    .checkmark {
      left: 64px;
    }
  }
  @media screen and (max-height: 390px) and (max-width: 750px) {
    .checkmark {
      top: 2px;
    }
  }
  @media screen and (max-height: 390px) and (max-width: 600px) {
    .checkmark {
      left: 10.5vw;
      top: 0px;
    }
  }
</style>

<div class="note-container">
  <div class="note-header">
    <p class="note-header-text {checked ? 'checked' : ''}">48m</p>
    <label class="checkbox-container note-header-text {checked ? 'checked' : ''}">
      Vanishing
      <input type="checkbox" bind:checked />
      <span class="checkmark"></span>
    </label>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="share-button" role="button" onclick={share} tabindex="0">
      <p class="note-header-text">Share</p>
    </div>
  </div>
  <div class="text-box"></div>
</div>
