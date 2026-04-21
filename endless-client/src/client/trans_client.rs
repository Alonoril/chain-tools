use crate::client::EnhancedClient;
use crate::client::types::{BaseTxnInfo, TryIntoTxHashValue};
use crate::error::EdsErr;
use base_infra::map_err;
use base_infra::result::AppResult;
use endless_sdk::rest_client::endless_api_types::{Event, MoveStructTag, MoveType};
use endless_sdk::rest_client::{Response, Transaction};
use serde::de::DeserializeOwned;

pub trait FilterEvent {
    fn filter_event_by_typ<T: DeserializeOwned>(&self, typ: MoveType) -> AppResult<Option<T>>;

    fn filter_event_by_tag<T: DeserializeOwned>(&self, tag: MoveStructTag) -> AppResult<Option<T>>;

    fn filter_events_by_typs(&self, typs: Vec<MoveType>) -> AppResult<Vec<Event>>;

    fn filter_events_by_tags(&self, tags: Vec<MoveStructTag>) -> AppResult<Vec<Event>>;
}

impl FilterEvent for &Vec<Event> {
    fn filter_event_by_typ<T: DeserializeOwned>(&self, typ: MoveType) -> AppResult<Option<T>> {
        let events = self.iter().find(|e| typ.eq(&e.typ));
        let Some(event) = events else {
            return Ok(None);
        };

        let data = serde_json::from_value::<T>(event.data.clone())
            .map_err(map_err!(&EdsErr::DecodeEventErr))?;
        Ok(Some(data))
    }

    fn filter_event_by_tag<T: DeserializeOwned>(&self, tag: MoveStructTag) -> AppResult<Option<T>> {
        let typ = MoveType::Struct(tag);
        self.filter_event_by_typ(typ)
    }

    fn filter_events_by_typs(&self, typs: Vec<MoveType>) -> AppResult<Vec<Event>> {
        let events = self
            .iter()
            .filter(|event| typs.iter().any(|typ| typ.eq(&event.typ)))
            .cloned()
            .collect();
        Ok(events)
    }

    fn filter_events_by_tags(&self, tags: Vec<MoveStructTag>) -> AppResult<Vec<Event>> {
        let typs = tags.into_iter().map(MoveType::Struct).collect();
        self.filter_events_by_typs(typs)
    }
}

impl FilterEvent for Transaction {
    fn filter_event_by_typ<T: DeserializeOwned>(&self, typ: MoveType) -> AppResult<Option<T>> {
        let events = self.events()?;
        events.filter_event_by_typ(typ)
    }

    fn filter_event_by_tag<T: DeserializeOwned>(&self, tag: MoveStructTag) -> AppResult<Option<T>> {
        let events = self.events()?;
        events.filter_event_by_tag(tag)
    }

    fn filter_events_by_typs(&self, typs: Vec<MoveType>) -> AppResult<Vec<Event>> {
        let events = self.events()?;
        events.filter_events_by_typs(typs)
    }

    fn filter_events_by_tags(&self, tags: Vec<MoveStructTag>) -> AppResult<Vec<Event>> {
        let events = self.events()?;
        events.filter_events_by_tags(tags)
    }
}

impl EnhancedClient {
    pub async fn get_txn_by_hash<T: TryIntoTxHashValue>(
        &self,
        hash: T,
    ) -> AppResult<Response<Transaction>> {
        let hash = hash.try_into_hash_value()?;
        self.client
            .get_transaction_by_hash(hash.0)
            .await
            .map_err(map_err!(&EdsErr::GetTxnByHash))
    }

    pub async fn get_txn_events_by_hash<H: TryIntoTxHashValue>(
        &self,
        hash: H,
        filter_tags: Vec<MoveStructTag>,
    ) -> AppResult<Response<Vec<Event>>> {
        let tx = self.get_txn_by_hash(hash).await?;
        let (inner, state) = tx.into_parts();
        let events = inner.filter_events_by_tags(filter_tags)?;
        Ok(Response::new(events, state))
    }

    pub async fn get_txns_by_versions(
        &self,
        versions: Vec<u64>,
    ) -> AppResult<Response<Vec<Transaction>>> {
        self.client
            .get_transactions_by_version(versions)
            .await
            .map_err(map_err!(&EdsErr::GetTxnsByVersions))
    }

    pub async fn get_txn_by_version(&self, version: u64) -> AppResult<Response<Transaction>> {
        self.client
            .get_transaction_by_version(version)
            .await
            .map_err(map_err!(&EdsErr::GetTxnByVersion))
    }

    pub async fn filter_txn_events_by_version(
        &self,
        version: u64,
        filter_tags: Vec<MoveStructTag>,
    ) -> AppResult<Response<BaseTxnInfo<Vec<Event>>>> {
        let tx = self.get_txn_by_version(version).await?;
        let (inner, state) = tx.into_parts();
        let events = inner.filter_events_by_tags(filter_tags)?;

        let txi = inner
            .transaction_info()
            .map_err(map_err!(&EdsErr::ParseTxnInfo))?;
        let (ts, succ, hash) = (inner.timestamp(), inner.success(), txi.hash.clone());
        let bs_txn = BaseTxnInfo::new(ts, succ, hash, events);
        Ok(Response::new(bs_txn, state))
    }
}
