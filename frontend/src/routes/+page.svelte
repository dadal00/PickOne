<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { browser } from '$app/environment'

  function setVh() {
    document.documentElement.style.setProperty('--vh', `${window.innerHeight * 0.01}px`)
  }

  onMount(() => {
    setVh()
    window.addEventListener('resize', setVh)
  })

  onDestroy(() => {
    if (browser) {
      window.removeEventListener('resize', setVh)
    }
  })
</script>

<style>
  :global(body, html) {
    margin: 0;
    /* touch-action: manipulation; */
  }
  .page-container {
    display: flex;
    flex-direction: column;
    height: calc(var(--vh, 1vh) * 100);
    height: 100vh;
    width: 100vw;
    background-color: #fff6ed;
    position: fixed;
  }
</style>

<main class="page-container"></main>
