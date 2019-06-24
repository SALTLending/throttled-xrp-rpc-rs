#![allow(non_snake_case)]

#[macro_use]
extern crate throttled_json_rpc;

use num_bigint::BigInt;

#[derive(Serialize)]
pub struct AccountParams<'a, 'b> {
    pub account: &'a str,
    pub strict: bool,
    pub ledger_index: &'b str,
    pub queue: bool,
}

#[derive(Deserialize)]
pub enum LedgerEntryType {
    AccountRoot, // WHY DOES THIS EVEN EXIST???
}

#[derive(Deserialize)]
pub struct AccountData {
    pub Account: String,
    pub Balance: BigInt,
    pub Flags: usize,
    pub LedgerEntryType: LedgerEntryType,
    pub OwnerCount: usize,
    pub PreviousTxnID: String,
    pub PreviousTxnLgrSeq: usize,
    pub Sequence: usize,
    pub index: String,
}

#[derive(Deserialize)]
pub struct QueuedTransaction {
    pub LastLedgerSequence: Option<usize>,
    pub auth_change: bool,
    pub fee: BigInt,
    pub fee_level: BigInt,
    pub max_spend_drops: BigInt,
    pub seq: usize,
}

#[derive(Deserialize)]
pub struct QueueData {
    pub auth_change_queued: bool,
    pub highest_sequence: usize,
    pub lowest_sequence: usize,
    pub max_spend_drops_total: BigInt,
    pub transactions: Vec<QueuedTransaction>,
    pub txn_count: usize,
}

#[derive(Deserialize)]
pub struct AccountInfo {
    account_data: AccountData,
    ledger_current_index: usize,
    queue_data: Option<QueueData>,
    status: String,
    validated: bool,
}

jsonrpc_client!(pub struct XRPClient {
    single:
        pub fn account_info(&self, params: AccountParams) -> Result<AccountInfo>;
    enum:
});
