<script lang="ts">
  const { name, selected, add } = $props<{
    name: string
    selected?: boolean
    add?: boolean
  }>()
  const switchTab = (event: MouseEvent) => {
    if (!selected) {
      console.log('goto')
    }
  }
  const addTab = (event: MouseEvent) => {
    console.log('add')
  }
  const closeTab = (event: MouseEvent) => {
    console.log('close')
  }
</script>

<style>
  .tab-container {
    padding: 8px 20px 24px 20px;
    position: relative;
    border-top-left-radius: 11px;
    border-top-right-radius: 11px;
    display: flex;
    flex-direction: row;
    align-items: center;
    z-index: 0;
  }
  .add-tab {
    margin-left: auto;
    width: 5vw;
    justify-content: center;
    color: #dfc1a3;
    font-weight: bold;
  }
  .add-tab:hover {
    opacity: 70%;
  }
  .tab-background {
    position: absolute;
    background-color: white;
    border-top: 4px solid #f2dfcb;
    border-right: 4px solid #f2dfcb;
    border-left: 4px solid #f2dfcb;
    inset: 0;
    z-index: -1;
    border-top-left-radius: 11px;
    border-top-right-radius: 11px;
    padding: 10px 20px 24px 20px;
  }
  .tab-container-unselected {
    opacity: 40%;
  }
  .tab-container-unselected:hover {
    opacity: 60%;
  }
  .tab-text {
    margin: 0;
    padding: 0;
    font-size: min(1rem, 2.2vw, 4.2vh);
    white-space: nowrap;
  }
  .tab-button {
    all: unset;
  }
  .tab-close-button {
    all: unset;
    margin-left: 15px;
    padding: 3px;
    border-radius: 3px;
  }
  .tab-close-button:hover {
    background-color: #f8f1e2;
  }
  .x-sprite {
    --scale: 0.08;
    background-image: url('/sprites.png');
    background-position: calc(-162px * var(--scale)) calc(-216px * var(--scale));
    background-size: calc(298px * var(--scale)) calc(357px * var(--scale));
    width: calc(126px * var(--scale));
    height: calc(140px * var(--scale));
    background-repeat: no-repeat;
  }
  @media screen and (max-width: 550px) {
    .x-sprite {
      --scale: 0.06;
    }
  }
  @media screen and (max-aspect-ratio: 0.57) and (max-width: 450px) {
    .x-sprite {
      --scale: 0.05;
    }
    .tab-container-unselected:hover {
      opacity: 40%;
    }
  }
</style>

<div
  class="tab-container {!selected && !add ? 'tab-container-unselected' : ''} {add ? 'add-tab' : ''}"
>
  <button class="tab-button" onclick={add ? addTab : switchTab}>
    <p class="tab-text">
      {name}
    </p>
  </button>
  {#if !add}
    <button class="tab-close-button" aria-label="close tab button" onclick={closeTab}>
      <div class="x-sprite"></div>
    </button>
  {/if}
  <button class="tab-background" aria-label="tab-button" onclick={add ? addTab : switchTab}
  ></button>
</div>
