<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { browser } from '$app/environment'
  import Header from '$lib/notes/components/Header.svelte'
  import Note from '$lib/notes/components/Note.svelte'

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
  :root {
    --scale: 0.17;
  }

  @media screen and (max-height: 500px) {
    :root {
      --scale: 0.14;
    }
  }
  @media screen and (max-height: 300px) {
    :root {
      --scale: 0.12;
    }
  }
  :global(body, html) {
    margin: 0;
    touch-action: manipulation;
    font-family: Verdana, Geneva, Tahoma, sans-serif;
    color: #5e5757;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
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

<main class="page-container">
  <Header></Header>
  <Note></Note>
</main>
