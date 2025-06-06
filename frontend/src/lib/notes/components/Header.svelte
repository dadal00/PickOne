<script lang="ts">
  const handleClick = (event: MouseEvent) => {
    console.log('hello')
  }
</script>

<style>
  .header-container {
    /* background-color: pink; */
    position: fixed;
    left: 0;
    right: 0;
    padding-left: 3vw;
    padding-right: 3vw;
    height: 14vh;
    user-select: none;
    display: flex;
    flex-direction: row;
    padding-top: 3vh;
  }
  .login-button {
    width: fit-content;
    height: fit-content;
  }
  .login-button:active {
    background-color: red;
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
    padding: 1.8vh 2vw 1.8vh 2vw;
    display: flex;
    flex-wrap: row;
  }
  .text-options-container {
    display: flex;
    flex-wrap: row;
    gap: 2.5vw;
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
    margin: calc(100px * var(--scale) / -2 + 5px) 1.5vw 0 1.5vw;
    align-self: center;
  }
  .layout-sprite {
    background-image: url('/sprites.png');
    background-position: calc(-1px * var(--scale)) calc(-135px * var(--scale));
    background-size: calc(298px * var(--scale)) calc(357px * var(--scale));
    width: calc(159px * var(--scale));
    height: calc(132px * var(--scale));
    background-repeat: no-repeat;
    margin-top: calc(19px * var(--scale) / -2);
    align-self: center;
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
</style>

<div class="header-container">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="login-button" role="button" onclick={handleClick} tabindex="0">
    <p class="login-text">Login</p>
  </div>
  <div class="options-container">
    <div class="text-options-container">
      <div class="bold-sprite"></div>
      <div class="italics-sprite"></div>
      <div class="highlight-sprite"></div>
    </div>
    <div class="divider"></div>
    <div class="layout-sprite"></div>
  </div>
</div>
