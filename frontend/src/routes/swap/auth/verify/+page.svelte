<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_TEMP_SESSION_DURATION_SECS, PUBLIC_SVELTE_SWAP_ROOT } from '$env/static/public'
	import { appState } from '$lib/swap/AppState.svelte'
	import { Status } from '$lib/swap/models'
	import { onDestroy, onMount } from 'svelte'
	import VerifyCode from '$lib/swap/components/auth/verify/VerifyCode.svelte'

	let auth_code: string = $state('')
	let timer: number | null = $state(null)

	onMount(() => {
		if (!appState.getStatus(Status.isVerifying)) {
			goto(PUBLIC_SVELTE_SWAP_ROOT + '/browse')
		}
		timer = setTimeout(
			() => {
				appState.setStatus(Status.isVerifying, false)
			},
			Number(PUBLIC_TEMP_SESSION_DURATION_SECS) * 1000
		)
	})

	$effect(() => {
		if (!appState.getStatus(Status.isVerifying)) {
			goto(PUBLIC_SVELTE_SWAP_ROOT + '/browse')
		}
	})

	onDestroy(() => {
		appState.setStatus(Status.isVerifying, false)
		clearTimeout(timer!)
	})
</script>

<VerifyCode bind:auth_code verification_type="verify" bind:timer />
