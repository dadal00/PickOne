<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { websocket } from '$lib/voting/stores/websocket'
  import BarChart from '$lib/voting/components/BarChart.svelte'
  import VoteButton from '$lib/voting/components/VoteButton.svelte'
  import TotalHeader from '$lib/voting/components/TotalHeader.svelte'
  import { browser } from '$app/environment'
  import { Color } from '$lib/voting/models'

  function setVh() {
    document.documentElement.style.setProperty('--vh', `${window.innerHeight * 0.01}px`)
  }

  onMount(() => {
    websocket.connect()
    setVh()
    window.addEventListener('resize', setVh)
  })

  onDestroy(() => {
    if (browser) {
      window.removeEventListener('resize', setVh)
    }
    websocket.disconnect()
  })
</script>

<style>
  :global(body, html) {
    margin: 0;
    touch-action: manipulation;
  }

  .page-container {
    display: flex;
    flex-direction: column;
    height: calc(var(--vh, 1vh) * 100);
    height: 100vh;
    width: 100vw;
    background-color: #faf4ee;
    position: fixed;
  }

  .footer-container {
    bottom: 0;
    left: 0;
    right: 0;
    position: fixed;
    min-height: 4rem;
    height: calc(var(--vh, 1vh) * 17);
    background-color: #f5f0e6;
    border-top: 1px solid rgb(188, 185, 178);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6vw;
  }

  .body-container {
    overflow: hidden;
    background-color: #faf4ee;
    height: calc(var(--vh, 1vh) * 83);
  }
</style>

<main class="page-container">
  <div class="body-container">
    <TotalHeader />
    <BarChart />
  </div>
  <div class="footer-container">
    <VoteButton color={Color.Red} />
    <VoteButton color={Color.Blue} />
    <VoteButton color={Color.Green} />
    <VoteButton color={Color.Purple} />
  </div>
</main>
