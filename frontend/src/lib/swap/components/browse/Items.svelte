<script lang="ts">
	import { appState } from '$lib/swap/AppState.svelte'
	import ItemCard from '$lib/swap/components/ItemCard.svelte'
	import { search } from '$lib/swap/meiliClient'
	import { ItemFields } from '$lib/swap/models'
	import { page } from '$app/state'

	let searchTimer: number | undefined = undefined

	$effect(() => {
		if (!page.url.pathname.includes('/browse')) {
			search('', '', '', '', 0)
			return
		}

		const fullQuery = appState.getFullQuery()

		if (searchTimer) {
			clearTimeout(searchTimer)
		}

		searchTimer = setTimeout(() => {
			search(
				fullQuery.query,
				fullQuery[ItemFields.ITEM_TYPE],
				fullQuery[ItemFields.LOCATION],
				fullQuery[ItemFields.CONDITION],
				fullQuery.offset
			)
		}, 200)
	})
</script>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
	{#each appState.getHits() as hit}
		<ItemCard item={hit} />
	{/each}
</div>
