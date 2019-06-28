#![allow(non_snake_case)]

#[macro_use]
extern crate throttled_json_rpc;

use bigdecimal::BigDecimal;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Balance {
    XRP(BigDecimal),
    Other {
        currency: String,
        issuer: String,
        value: BigDecimal,
    },
}

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
    ledger_index: Option<String>,
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
    pub Balance: BigDecimal,
    pub Flags: BigDecimal,
    pub LedgerEntryType: LedgerEntryType,
    pub OwnerCount: BigDecimal,
    pub PreviousTxnID: String,
    pub PreviousTxnLgrSeq: BigDecimal,
    pub Sequence: BigDecimal,
    pub index: String,
}

#[derive(Deserialize)]
pub struct QueuedTransaction {
    pub LastLedgerSequence: Option<BigDecimal>,
    pub auth_change: bool,
    pub fee: BigDecimal,
    pub fee_level: BigDecimal,
    pub max_spend_drops: BigDecimal,
    pub seq: BigDecimal,
}

#[derive(Deserialize)]
pub struct QueueData {
    pub auth_change_queued: bool,
    pub highest_sequence: BigDecimal,
    pub lowest_sequence: BigDecimal,
    pub max_spend_drops_total: BigDecimal,
    pub transactions: Vec<QueuedTransaction>,
    pub txn_count: BigDecimal,
}

#[derive(Deserialize)]
pub struct AccountInfo {
    account_data: AccountData,
    ledger_current_index: BigDecimal,
    queue_data: Option<QueueData>,
    status: String,
    validated: bool,
}

#[derive(Deserialize)]
pub struct PathInfo {
    currency: String,
    issuer: Option<String>,
    #[serde(rename = "type")]
    currency_type: BigDecimal,
    type_hex: String
}

#[derive(Deserialize)]
pub struct FinalFieldInfo {
    Balance: Option<Balance>,
    Flags: isize,
    OwnerCount: Option<BigDecimal>,
    Sequence: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct ModifiedNodeInfo {
    FinalFields: FinalFieldInfo,
    LedgerEntryType: String,
    LedgerIndex: String,
    PreviousTxnID: Option<String>,
    PreviousTxnLgrSeq: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct AffectedNodeInfo {
    ModifiedNode: Option<ModifiedNodeInfo>,
}

#[derive(Deserialize)]
pub struct MetaTxInfo {
    AffectedNodes: Vec<AffectedNodeInfo>,
    TransactionIndex: BigDecimal,
    TransactionResult: String,
}

#[derive(Deserialize)]
pub struct TransactionInfo {
    Account: String,
    Amount: Option<Balance>,
    Destination: Option<String>,
    Fee: BigDecimal,
    Flags: isize,
    Paths: Option<Vec<Vec<PathInfo>>>,
    SendMax: Option<Balance>,
    Sequence: BigDecimal,
    SigningPubKey: String,
    TransactionType: String,
    TxnSignature: String,
    hash: String,
    LedgerIndex: Option<String>,
    metaData: MetaTxInfo,
    validated: Option<bool> //option of a bool???
}

#[derive(Deserialize)]
pub struct NestedLedgerInfo {
    accepted: bool,
    account_hash: String,
    close_flags: isize,
    close_time: BigDecimal,
    close_time_human: String,
    close_time_resolution: BigDecimal,
    closed: bool,
    hash: String,
    ledger_hash: String,
    ledger_index: String,
    parent_close_time: BigDecimal,
    parent_hash: String,
    seqNum: String,
    totalCoins: String,
    total_coins: BigDecimal,
    transaction_hash: String,
    transactions: Option<Vec<TransactionInfo>>,
}

#[derive(Deserialize)]
pub struct LedgerInfo {
    ledger: NestedLedgerInfo,
    ledger_hash: String,
    ledger_index: BigDecimal,
    status: String,
    validated: bool,
}

jsonrpc_client!(pub struct XRPClient {
    single:
        pub fn account_info(&self, params: AccountParams) -> Result<AccountInfo>;
        pub fn ledger_info(&self, params: LedgerParams) -> Result<LedgerInfo>;
    enum:
});

#[test] 
fn json_test() {
    let _:LedgerInfo = serde_json::from_reader(std::fs::File::open("ledger.json").unwrap()).unwrap();
}