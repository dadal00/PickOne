<script lang="ts">
  let bold = $state(false)
  let italics = $state(false)
  let highlight = $state(false)
  let twoLayout = $state(false)

  const login = (event: MouseEvent) => {
    console.log('login')
  }
  const toggleBold = (event: MouseEvent) => {
    bold = !bold
  }
  const toggleItalics = (event: MouseEvent) => {
    italics = !italics
  }
  const toggleHighlight = (event: MouseEvent) => {
    highlight = !highlight
  }
  const toggleLayout = (event: MouseEvent) => {
    twoLayout = !twoLayout
  }
</script>

<style>
  .header-container {
    position: fixed;
    left: 0;
    right: 0;
    padding-left: 3vw;
    padding-right: 3vw;
    height: 14vh;
    display: flex;
    flex-direction: row;
    padding-top: 3vh;
  }
  .login-button {
    width: fit-content;
    height: fit-content;
  }
  .clickable-container {
    border-radius: 3px;
    padding: 3px;
    height: fit-content;
  }
  .clickable-container:hover {
    background-color: #ffe3c6;
    opacity: 65%;
  }
  .clickable-container:active {
    background-color: #ffe3c6;
  }
  .clickable-container-on {
    background-color: #ffe3c6;
  }
  .login-text {
    margin: 0;
    padding: 0;
    font-size: min(1.5rem, 3.5vw, 4.5vh);
  }
  .options-container {
    margin-top: -1vh;
    margin-left: auto;
    width: fit-content;
    height: 2.5vh;
    background-color: white;
    border: 1px solid #cfcdcd;
    border-radius: 11px;
    padding: 1.5vh 2vw 1.5vh 2vw;
    display: flex;
    flex-wrap: row;
  }
  .text-options-container {
    display: flex;
    flex-wrap: row;
    gap: 3vw;
    height: fit-content;
    align-self: center;
  }
  .bold-sprite {
    background-image: url('/sprites.png');
    background-position: calc(-190px * var(--scale)) calc(-1px * var(--scale));
    background-size: calc(298px * var(--scale)) calc(357px * var(--scale));
    width: calc(81px * var(--scale));
    height: calc(113px * var(--scale));
    background-repeat: no-repeat;
  }
  .italics-sprite {
    background-image: url('/sprites.png');
    background-position: calc(-1px * var(--scale)) calc(-1px * var(--scale));
    background-size: calc(298px * var(--scale)) calc(357px * var(--scale));
    width: calc(68px * var(--scale));
    height: calc(113px * var(--scale));
    background-repeat: no-repeat;
  }
  .highlight-sprite {
    background-image: url('/sprites.png');
    background-position: calc(-71px * var(--scale)) calc(-1px * var(--scale));
    background-size: calc(298px * var(--scale)) calc(357px * var(--scale));
    width: calc(117px * var(--scale));
    height: calc(132px * var(--scale));
    background-repeat: no-repeat;
    margin-top: calc(19px * var(--scale) / -2);
  }
  .divider {
    background-image: url('/sprites.png');
    background-position: calc(-273px * var(--scale)) calc(-1px * var(--scale));
    background-size: calc(298px * var(--scale)) calc(357px * var(--scale));
    width: calc(24px * var(--scale));
    height: calc(213px * var(--scale));
    background-repeat: no-repeat;
    margin: calc(100px * var(--scale) / -2 + 5px) -1vw 0 -1vw;
  }
  .layout-sprite {
    background-image: url('/sprites.png');
    background-position: calc(-1px * var(--scale)) calc(-135px * var(--scale));
    background-size: calc(298px * var(--scale)) calc(357px * var(--scale));
    width: calc(159px * var(--scale));
    height: calc(132px * var(--scale));
    background-repeat: no-repeat;
    margin-top: calc(19px * var(--scale) / -2 + 1px);
  }
  @media screen and (max-height: 800px) {
    .options-container {
      height: 3vh;
    }
  }
  @media screen and (max-height: 700px) {
    .options-container {
      height: 3.5vh;
    }
  }
  @media screen and (max-height: 600px) {
    .options-container {
      height: 4.5vh;
    }
  }
  @media screen and (max-height: 500px) {
    .options-container {
      height: 6vh;
    }
    .login-button {
      margin-top: 0.75vh;
    }
  }
  @media screen and (max-height: 400px) {
    .options-container {
      height: 8vh;
    }
    .login-button {
      margin-top: 2vh;
    }
  }
  @media screen and (max-width: 700px) {
    .text-options-container {
      gap: 5vw;
    }
    .divider {
      margin: calc(100px * var(--scale) / -2 + 5px) -2vw 0 -2vw;
    }
  }
  @media screen and (max-width: 450px) {
    .options-container {
      padding: 1.8vh 4vw 1.8vh 4vw;
    }
    .text-options-container {
      gap: 8vw;
    }
    .divider {
      margin: calc(100px * var(--scale) / -2 + 5px) -3vw 0 -3vw;
    }
    .login-text {
      font-size: 1.5rem;
    }
  }
  @media screen and (max-aspect-ratio: 0.57) and (max-width: 450px) {
    .clickable-container:hover {
      background-color: none;
      opacity: 100%;
    }
  }
</style>

<div class="header-container">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="login-button clickable-container" role="button" onclick={login} tabindex="0">
    <p class="login-text">Login</p>
  </div>
  <div class="options-container">
    <div class="text-options-container">
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div
        class="clickable-container {bold ? 'clickable-container-on' : ''}"
        role="button"
        onclick={toggleBold}
        tabindex="0"
      >
        <div class="bold-sprite"></div>
      </div>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div
        class="clickable-container {italics ? 'clickable-container-on' : ''}"
        role="button"
        onclick={toggleItalics}
        tabindex="0"
      >
        <div class="italics-sprite"></div>
      </div>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div
        class="clickable-container {highlight ? 'clickable-container-on' : ''}"
        role="button"
        onclick={toggleHighlight}
        tabindex="0"
      >
        <div class="highlight-sprite"></div>
      </div>
      <div class="divider"></div>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div
        class="clickable-container {twoLayout ? 'clickable-container-on' : ''}"
        role="button"
        onclick={toggleLayout}
        tabindex="0"
      >
        <div class="layout-sprite"></div>
      </div>
    </div>
  </div>
</div>
