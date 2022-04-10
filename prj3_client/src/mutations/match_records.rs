use std::rc::Rc;

use async_trait::async_trait;

use bounce::prelude::*;
use bounce::query::{Mutation, MutationResult, Query, QueryResult};

use shared_types::types::{MatchQueryFilter, MatchQuerySortBy, MatchRecord, ToQueryPairs, ClientMatchData, Records};

use super::util::*;
use super::ServiceError;

/// Options for a match query request
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MatchRecordQueryOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub filters: Option<MatchQueryFilter>,
    pub sort_by: Option<MatchQuerySortBy>,
    pub asc: Option<bool>
}

/// Match Records Query
#[derive(Debug, PartialEq)]
pub struct MatchRecordQuery(pub Records<MatchRecord>);

// Use mutation here because Bounce's Query API is broken...
#[async_trait(?Send)]
impl Mutation for MatchRecordQuery {
    type Input = MatchRecordQueryOptions;
    type Error = ServiceError;

    async fn run(_states: &BounceStates, input: Rc<MatchRecordQueryOptions>) -> MutationResult<Self> {
        let records = get_records(
            input.limit,
            input.offset,
            &input.filters,
            input.sort_by,
            input.asc
        ).await?;

        Ok(MatchRecordQuery(records).into())
    }
}

/// Match Record Query for Currently Authenticated User
#[derive(Debug, PartialEq)]
pub struct UserMatchRecordQuery(pub Records<MatchRecord>);

#[async_trait(?Send)]
impl Query for UserMatchRecordQuery {
    type Input = MatchRecordQueryOptions;
    type Error = ServiceError;

    async fn query(_states: &BounceStates, input: Rc<MatchRecordQueryOptions>) -> QueryResult<Self> {
        let records = get_user_records(
            input.limit,
            input.offset,
            &input.filters,
            input.sort_by,
            input.asc
        ).await?;

        Ok(UserMatchRecordQuery(records).into())
    }
}

/// Mutation for reporting a new match to the server backend
#[derive(Debug, PartialEq)]
pub struct UserMatchRecordMutation();

#[async_trait(?Send)]
impl Mutation for UserMatchRecordMutation {
    type Input = ClientMatchData;
    type Error = ServiceError;

    async fn run(_states: &BounceStates, input: Rc<ClientMatchData>) -> MutationResult<Self> {
        post_user_record(&input).await?;

        Ok(UserMatchRecordMutation().into())
    }

}

/// POST to /user/records/add with data
async fn post_user_record(
    record: &ClientMatchData
) -> Result<(), APIError> {
    let endpoint_url = get_base_url().join("api/v1/user/records/add").unwrap();

    let client = reqwest::Client::new();

    let _response = client.post(endpoint_url)
        .json(&record)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

/// GET from /user/records using options
async fn get_user_records(
    limit: Option<i64>,
    offset: Option<i64>,
    filters: &Option<MatchQueryFilter>,
    sort_by: Option<MatchQuerySortBy>,
    asc: Option<bool>
) -> Result<Records<MatchRecord>, APIError> {
    let endpoint_url = get_base_url().join("api/v1/user/records").unwrap();

    let client = reqwest::Client::new();

    let response = client.get(endpoint_url)
        .query_pair("offset", offset)
        .query_pair("limit", limit)
        .query_pair("sort_by", sort_by)
        .query(
            &filters.as_ref()
                .map(|f| f.query_pairs())
                .unwrap_or(vec![])
        )
        .query_pair("asc", asc)
        .send()
        .await?
        .error_for_status()?
        .json::<Records<MatchRecord>>()
        .await?;

    Ok(response)
}

/// GET from /games/records using options
async fn get_records(
    limit: Option<i64>,
    offset: Option<i64>,
    filters: &Option<MatchQueryFilter>,
    sort_by: Option<MatchQuerySortBy>,
    asc: Option<bool>
) -> Result<Records<MatchRecord>, APIError> {
    let endpoint_url = get_base_url().join("api/v1/games/records").unwrap();

    let client = reqwest::Client::new();

    let response = client.get(endpoint_url)
        .query_pair("offset", offset)
        .query_pair("limit", limit)
        .query_pair("sort_by", sort_by)
        .query(
            &filters.as_ref()
                .map(|f| f.query_pairs())
                .unwrap_or(vec![])
        )
        .query_pair("asc", asc)
        .send()
        .await?
        .error_for_status()?
        .json::<Records<MatchRecord>>()
        .await?;

    Ok(response)
}
