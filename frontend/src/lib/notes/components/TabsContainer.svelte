<script lang="ts">
  import Tab from './Tab.svelte'
</script>

<style>
  .tabs-container {
    margin: calc(17vh - 35px) 0 0 calc(2.5vw + 4px);
    position: absolute;
    gap: 1vw;
    display: flex;
    flex-direction: row;
    overflow-x: scroll;
    -webkit-overflow-scrolling: touch;
    scrollbar-width: thin;
    width: calc(95% - 8px);
    height: fit-content;
    z-index: 0;
  }
  /* @media screen and (max-height: 600px) {
    .tabs-container {
      margin-left: 10vw;
      width: 70%;
      margin-top: 11vh;
    }
  }
  @media screen and (max-height: 600px) and (max-width: 1000px) {
    .tabs-container {
      width: 67%;
    }
  }
  @media screen and (max-height: 600px) and (max-width: 700px) {
    .tabs-container {
      margin-left: 12vw;
      width: 60%;
    }
  }
  @media screen and (max-height: 600px) and (max-width: 550px) {
    .tabs-container {
      margin-left: 14vw;
      width: 55%;
    }
  }
  @media screen and (max-aspect-ratio: 0.57) and (max-width: 450px) {
    .tabs-container {
      margin-top: 13.5vh;
    }
  } */
</style>

<div class="tabs-container">
  <Tab name="Notes 1 ALJFLADFKJASF;LKSJLKJ" selected={true}></Tab>
  <Tab
    name="Notes 2 ALJFLADFKJASF;LKSJLKJALJFLADFKJASF;LKSJLKJALJFLADFKJASF;LKSJLKJ"
    selected={false}
  ></Tab>
  <Tab name="Notes 3" selected={false}></Tab>
  <Tab name="Notes 4" selected={false}></Tab>
</div>
