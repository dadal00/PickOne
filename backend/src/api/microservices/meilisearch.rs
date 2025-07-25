use super::database::{
    init::DatabaseQueries,
    schema::{columns::boiler_swap::items, tables},
};
use crate::{
    AppError,
    api::{web::swap::database::convert_db_items, web::swap::models::ItemRow},
    config::{read_secret, try_load},
};
use anyhow::Result as anyResult;
use meilisearch_sdk::{
    client::*,
    settings::{MinWordSizeForTypos, Settings, TypoToleranceSettings},
};
use scylla::{client::session::Session, response::PagingState};
use serde::Serialize;
use std::{
    marker::{Send, Sync},
    ops::ControlFlow,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering::Relaxed},
    },
};
use tokio::task::JoinHandle;
use uuid::Uuid;

pub async fn init_meilisearch(
    database_session: Arc<Session>,
    database_queries: &DatabaseQueries,
) -> Result<
    (
        Arc<Client>,
        JoinHandle<Result<(), AppError>>,
        Arc<AtomicUsize>,
    ),
    AppError,
> {
    let meili_url = try_load::<String>("MEILI_URL", "http://meilisearch:7700").unwrap();
    let meili_client =
        Arc::new(Client::new(meili_url, Some(read_secret("MEILI_ADMIN_KEY")?)).unwrap());

    let client_clone = meili_client.clone();
    let session_clone = database_session.clone();
    let queries_clone = database_queries.clone();

    let settings = init_settings();

    meili_client
        .index(tables::boiler_swap::ITEMS)
        .set_settings(&settings)
        .await
        .unwrap();

    let item_counter = Arc::new(AtomicUsize::new(0));
    let item_counter_clone = Arc::clone(&item_counter);

    let reindex_future = tokio::spawn(async move {
        reindex(
            session_clone,
            queries_clone,
            client_clone,
            tables::boiler_swap::ITEMS,
            items::ITEM_ID,
            item_counter_clone,
        )
        .await
    });

    Ok((meili_client, reindex_future, item_counter))
}

pub async fn reindex(
    database_session: Arc<Session>,
    database_queries: DatabaseQueries,
    meili_client: Arc<Client>,
    index_name: &str,
    item_id_name: &str,
    item_counter: Arc<AtomicUsize>,
) -> Result<(), AppError> {
    let mut paging_state = PagingState::start();

    clear_index(meili_client.clone(), index_name).await?;

    loop {
        let (query_result, paging_state_response) = database_session
            .execute_single_page(&database_queries.boiler_swap.get_items, &[], paging_state)
            .await?;

        let row_result = query_result.into_rows_result()?;

        let row_vec: Vec<ItemRow> = row_result
            .rows::<ItemRow>()?
            .collect::<Result<Vec<_>, _>>()?;

        item_counter.fetch_add(row_vec.len(), Relaxed);

        add_items(
            meili_client.clone(),
            index_name,
            &convert_db_items(&row_vec),
            item_id_name,
        )
        .await?;

        match paging_state_response.into_paging_control_flow() {
            ControlFlow::Break(()) => {
                break Ok(());
            }
            ControlFlow::Continue(new_paging_state) => paging_state = new_paging_state,
        }
    }
}

pub async fn add_items<T>(
    meili_client: Arc<Client>,
    index_name: &str,
    items: &[T],
    id_name: &str,
) -> anyResult<()>
where
    T: Serialize + Send + Sync,
{
    meili_client
        .index(index_name)
        .add_documents(items, Some(id_name))
        .await?
        .wait_for_completion(&meili_client, None, None)
        .await?;

    Ok(())
}

pub async fn delete_item(meili_client: Arc<Client>, index_name: &str, key: Uuid) -> anyResult<()> {
    meili_client
        .index(index_name)
        .delete_document(key)
        .await?
        .wait_for_completion(&meili_client, None, None)
        .await?;

    Ok(())
}

pub async fn clear_index(meili_client: Arc<Client>, index_name: &str) -> anyResult<()> {
    meili_client
        .index(index_name)
        .delete_all_documents()
        .await?
        .wait_for_completion(&meili_client, None, None)
        .await?;

    Ok(())
}

fn init_settings() -> Settings {
    Settings::new()
        .with_ranking_rules([
            "words",
            "typo",
            "proximity",
            "exactness",
            "attribute",
            "sort",
        ])
        .with_distinct_attribute(Some(items::ITEM_ID))
        .with_searchable_attributes([items::TITLE, items::DESCRIPTION])
        .with_filterable_attributes([items::ITEM_TYPE, items::CONDITION, items::LOCATION])
        .with_typo_tolerance(TypoToleranceSettings {
            enabled: Some(true),
            disable_on_attributes: None,
            disable_on_words: None,
            min_word_size_for_typos: Some(MinWordSizeForTypos {
                one_typo: Some(5),
                two_typos: Some(9),
            }),
        })
}
