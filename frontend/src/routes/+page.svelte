<script>
  import { onMount, onDestroy } from 'svelte'
  import { websocket } from '$lib/stores/websocket'
  import BarChart from '$lib/components/BarChart.svelte'
  import VoteButton from '$lib/components/VoteButton.svelte'
  import TotalHeader from '$lib/components/TotalHeader.svelte'
  import { browser } from '$app/environment'

  const labels = {
    red: 'Red 40',
    blue: 'Ocean',
    green: 'Grass',
    purple: 'Grape',
  }

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
    <BarChart {labels} />
  </div>
  <div class="footer-container">
    <VoteButton color="red" text={labels['red']} />
    <VoteButton color="blue" text={labels['blue']} />
    <VoteButton color="green" text={labels['green']} />
    <VoteButton color="purple" text={labels['purple']} />
  </div>
</main>
