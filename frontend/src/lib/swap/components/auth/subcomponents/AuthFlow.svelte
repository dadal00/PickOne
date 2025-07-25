<script lang="ts">
	import { appState } from '$lib/swap/AppState.svelte'
	import Email from '$lib/swap/components/auth/subcomponents/Email.svelte'
	import Password from '$lib/swap/components/auth/subcomponents/Password.svelte'
	import { login, signup, forgot } from '$lib/swap/helpers/auth'
	import type { Account, TabOptions } from '$lib/swap/models'
	import { onDestroy, onMount } from 'svelte'

	let { activeTabValue = $bindable(), showTab } = $props<{
		activeTabValue: string
		showTab: (tab: TabOptions) => void
	}>()

	let account: Account = $state({ email: '', password: '', action: 'signup' })
	let confirmPassword: string = $state('')
	let error: string = $derived(appState.getAuthError())

	function authFunction(_: MouseEvent) {
		appState.setAuthError('')
		switch (activeTabValue) {
			case 'Reset':
				forgot(account.email)
				break
			case 'Login':
				login(account)
				break
			case 'Signup':
				signup(account, confirmPassword)
				break
			default:
				console.log('Unknown tab')
		}
	}

	onMount(() => {
		appState.setAuthError('')
	})

	onDestroy(() => {
		appState.setAuthError('')
	})
</script>

<div class="space-y-4">
	{#if error != ''}
		<p class="text-red-600 text-sm font-medium text-center">{error}</p>
	{/if}
	{#if activeTabValue === 'Reset'}
		<p class="text-gray-600 text-sm">
			Enter your Purdue email address and we'll send you a link to reset your password.
		</p>
	{/if}
	<Email bind:account />
	{#if activeTabValue !== 'Reset'}
		<Password bind:password={account.password} displayName={'Password'} />
		{#if activeTabValue === 'Signup'}
			<Password bind:password={confirmPassword} displayName={'Confirm Password'} />
		{/if}
	{/if}
	<button
		class="w-full {appState.getLimited()
			? 'bg-gray-300 cursor-not-allowed'
			: 'bg-yellow-400 hover:bg-yellow-500'} text-gray-800 py-2 rounded-lg transition-colors"
		onclick={authFunction}
	>
		{activeTabValue}
	</button>
	{#if activeTabValue !== 'Signup'}
		<div class="text-center">
			<button
				onclick={() => showTab(activeTabValue === 'Reset' ? 'Login' : 'Reset')}
				class="text-yellow-600 hover:underline text-sm"
			>
				{activeTabValue === 'Login' ? 'Forgot your password?' : 'Back to login'}
			</button>
		</div>
	{/if}
</div>
