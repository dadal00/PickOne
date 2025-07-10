import { PUBLIC_MEILI_URL, PUBLIC_MEILI_KEY, PUBLIC_PAGE_SIZE } from '$env/static/public'
import { Meilisearch } from 'meilisearch'
import {
	ItemFields,
	ItemsTableName,
	type Condition,
	type Item,
	type ItemType,
	type Location
} from './models'
import { appState } from './AppState.svelte'

const client = new Meilisearch({
	host: PUBLIC_MEILI_URL,
	apiKey: PUBLIC_MEILI_KEY,
	requestInit: {
		credentials: 'include'
	}
})

export async function search(
	query: string,
	itemTypeFilter: ItemType | '',
	locationFilter: Location | '',
	conditionFilter: Condition | '',
	offset: number
) {
	const filters: string[] = []

	if (itemTypeFilter !== '') {
		filters.push(`${ItemFields.ITEM_TYPE} = ${itemTypeFilter}`)
	}

	if (locationFilter !== '') {
		filters.push(`${ItemFields.LOCATION} = ${locationFilter}`)
	}

	if (conditionFilter !== '') {
		filters.push(`${ItemFields.CONDITION} = ${conditionFilter}`)
	}

	const response = await client.index(ItemsTableName).search(query, {
		filter: filters,
		limit: Number(PUBLIC_PAGE_SIZE),
		offset: offset,
		attributesToHighlight: [ItemFields.TITLE, ItemFields.DESCRIPTION],
		highlightPreTag: '<mark>',
		highlightPostTag: '</mark>'
	})

	appState.setQueryResults(response.hits as Item[], response.estimatedTotalHits)
}
