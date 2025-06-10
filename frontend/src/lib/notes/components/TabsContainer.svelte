<script lang="ts">
  import Tab from './Tab.svelte'
</script>

<style>
  .note-tab-container {
    margin: calc(17vh - 35px) 0 0 calc(2.5vw + 4px);
    position: absolute;
    width: calc(95% - 8px);
    z-index: 0;
    height: fit-content;
    display: flex;
    flex-direction: row;
    gap: 20px;
  }
  .tabs-container {
    gap: 1vw;
    display: flex;
    flex-direction: row;
    overflow-x: scroll;
    -webkit-overflow-scrolling: touch;
    scrollbar-width: thin;
    width: 85%;
  }
  @media screen and (max-height: 600px) {
    .note-tab-container {
      margin-left: 10vw;
      width: 70%;
      margin-top: 11vh;
    }
  }
  @media screen and (max-height: 520px) {
    .note-tab-container {
      margin-top: 11vh;
    }
  }
  @media screen and (max-height: 400px) {
    .note-tab-container {
      margin-top: 8vh;
    }
  }
  @media screen and (max-height: 600px) and (max-width: 1000px) {
    .note-tab-container {
      width: 67%;
    }
  }
  @media screen and (max-height: 600px) and (max-width: 700px) {
    .note-tab-container {
      margin-left: 12vw;
      width: 60%;
    }
  }
  @media screen and (max-height: 600px) and (max-width: 550px) {
    .note-tab-container {
      margin-left: 14vw;
      width: 55%;
    }
  }
  @media screen and (max-aspect-ratio: 0.57) and (max-width: 450px) {
    .note-tab-container {
      margin-top: 13.75vh;
    }
  }
</style>

<div class="note-tab-container">
  <div class="tabs-container">
    <Tab name="Notes 1 ALJFLADFKJASF;LKSJLKJ" selected={true}></Tab>
    <Tab
      name="Notes 2 ALJFLADFKJASF;LKSJLKJALJFLADFKJASF;LKSJLKJALJFLADFKJASF;LKSJLKJ"
      selected={false}
    ></Tab>
    <Tab name="Notes 3" selected={false}></Tab>
    <Tab name="Notes 4" selected={false}></Tab>
  </div>
  <Tab name="+" add={true}></Tab>
</div>
