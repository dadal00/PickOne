<script lang="ts">
  const { name, selected } = $props<{
    name: String
    selected: Boolean
  }>()
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
    /* background-color: pink; */
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
    opacity: 60%;
  }
  .tab-text {
    margin: 0;
    padding: 0;
    font-size: min(1rem, 2.2vw, 4.2vh);
    white-space: nowrap;
  }
  .tab-text:hover {
    opacity: 10%;
  }
  .x-sprite {
    --scale: 0.075;
    background-image: url('/sprites.png');
    background-position: calc(-162px * var(--scale)) calc(-216px * var(--scale));
    background-size: calc(298px * var(--scale)) calc(357px * var(--scale));
    width: calc(126px * var(--scale));
    height: calc(140px * var(--scale));
    background-repeat: no-repeat;
    margin-left: 15px;
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
  }
</style>

<div class="tab-container {selected ? '' : 'tab-container-unselected'}">
  <p class="tab-text">
    {name}
  </p>
  <div class="x-sprite"></div>
  <div class="tab-background"></div>
</div>
