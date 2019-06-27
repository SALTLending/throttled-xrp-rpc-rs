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

#[derive(Serialize)]
pub struct LedgerParams {
    ledger_hash: Option<String>,
    ledger_index: Option<usize>,
    full: Option<bool>,
    accounts: Option<bool>,
    transactions: Option<bool>,
    expand: Option<bool>,
    owner_funds: Option<bool>,
    binary: Option<bool>,
    queue: Option<bool>,
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

#[derive(Deserialize)]
pub struct AmountInfo {
    currency: String,
    issuer: String,
    value: usize,
}

#[derive(Deserialize)]
pub struct PathInfo {
    account: String,
    currency: String,
    issuer: String,
    r#type: usize,
    type_hex: String
}

#[derive(Deserialize)]
pub struct SendMaxInfo {
    currency: String,
    issuer: String,
    value: f64,
}

#[derive(Deserialize)]
pub struct FinalFieldInfo {
    Account: String,
    Balance: f64,
    Flags: isize,
    OwnerCount: usize,
    Sequence: usize,
}

#[derive(Deserialize)]
pub struct PreviousFieldInfo {
    Balance: f64,
    Sequence: usize,
}

#[derive(Deserialize)]
pub struct ModifiedNodeInfo {
    FinalFields: FinalFieldInfo,
    LedgerEntryType: String,
    LedgerIndex: String,
    PreviousFields: PreviousFieldInfo,
    PreviousTxnID: String,
    PreviousTxnLgrSeq: usize,
}

#[derive(Deserialize)]
pub struct AffectedNodeInfo {
    ModifiedNode: ModifiedNodeInfo,
}

#[derive(Deserialize)]
pub struct MetaTxInfo {
    AffectedNodes: Vec<AffectedNodeInfo>,
    TransactionIndex: usize,
    TransactionResult: String,
}

#[derive(Deserialize)]
pub struct TransactionInfo {
    Account: String,
    Amount: AmountInfo,
    Destination: String,
    Fee: usize,
    Flags: isize,
    Paths: Vec<Vec<PathInfo>>,
    SendMax: SendMaxInfo,
    Sequence: usize,
    SigningPubKey: String,
    TransactionType: String,
    TxnSignature: String,
    hash: String,
    inLedger: usize,
    ledger_index: usize,
    meta: MetaTxInfo,
    validated: bool
}

#[derive(Deserialize)]
pub struct NestedLedgerInfo {
    accepted: bool,
    account_hash: String,
    close_flags: isize,
    close_time: usize,
    close_time_human: String,
    close_time_resolution: usize,
    closed: bool,
    hash: String,
    ledger_hash: String,
    ledger_index: String,
    parent_close_time: usize,
    parent_hash: String,
    seqNum: usize,
    totalCoins: usize,
    total_coins: usize,
    transaction_hash: String,
    transactions: Option<Vec<TransactionInfo>>,
}

#[derive(Deserialize)]
pub struct LedgerInfo {
    ledger: NestedLedgerInfo,
    ledger_hash: String,
    ledger_index: String,
    status: String,
    validated: bool,
}

jsonrpc_client!(pub struct XRPClient {
    single:
        pub fn account_info(&self, params: AccountParams) -> Result<AccountInfo>;
        pub fn ledger_info(&self, params: LedgerParams) -> Result<LedgerInfo>;
    enum:
});
