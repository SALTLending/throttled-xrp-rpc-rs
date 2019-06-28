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
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<String>,
    pub full: Option<bool>,
    pub accounts: Option<bool>,
    pub transactions: Option<bool>,
    pub expand: Option<bool>,
    pub owner_funds: Option<bool>,
    pub binary: Option<bool>,
    pub queue: Option<bool>,
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
    pub account_data: AccountData,
    pub ledger_current_index: BigDecimal,
    pub queue_data: Option<QueueData>,
    pub status: String,
    pub validated: bool,
}

#[derive(Deserialize)]
pub struct PathInfo {
    pub currency: String,
    pub issuer: Option<String>,
    #[serde(rename = "type")]
    pub currency_type: BigDecimal,
    pub type_hex: String
}

#[derive(Deserialize)]
pub struct FinalFieldInfo {
    pub Balance: Option<Balance>,
    pub Flags: isize,
    pub OwnerCount: Option<BigDecimal>,
    pub Sequence: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct PreviousFieldInfo {
    pub Balance: Option<Balance>,
    pub Sequence: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct ModifiedNodeInfo {
    pub FinalFields: FinalFieldInfo,
    pub PreviousFields: Option<PreviousFieldInfo>, // is this really optional ???
    pub LedgerEntryType: String,
    pub LedgerIndex: String,
    pub PreviousTxnID: Option<String>,
    pub PreviousTxnLgrSeq: Option<BigDecimal>,
}

#[derive(Deserialize)]
pub struct AffectedNodeInfo {
    pub ModifiedNode: Option<ModifiedNodeInfo>,
}

#[derive(Deserialize)]
pub struct MetaTxInfo {
    pub AffectedNodes: Vec<AffectedNodeInfo>,
    pub TransactionIndex: BigDecimal,
    pub TransactionResult: String,
}

#[derive(Deserialize)]
pub struct TransactionInfo {
    pub Account: String,
    pub Amount: Option<Balance>,
    pub Destination: Option<String>,
    pub Fee: BigDecimal,
    pub Flags: isize,
    pub Paths: Option<Vec<Vec<PathInfo>>>,
    pub SendMax: Option<Balance>,
    pub Sequence: BigDecimal,
    pub SigningPubKey: String,
    pub TransactionType: String,
    pub TxnSignature: String,
    pub hash: String,
    pub LedgerIndex: Option<String>,
    pub metaData: MetaTxInfo,
    pub validated: Option<bool> //option of a bool???
}

#[derive(Deserialize)]
pub struct NestedLedgerInfo {
    pub accepted: bool,
    pub account_hash: String,
    pub close_flags: isize,
    pub close_time: BigDecimal,
    pub close_time_human: String,
    pub close_time_resolution: BigDecimal,
    pub closed: bool,
    pub hash: String,
    pub ledger_hash: String,
    pub ledger_index: String,
    pub parent_close_time: BigDecimal,
    pub parent_hash: String,
    pub seqNum: String,
    pub totalCoins: String,
    pub total_coins: BigDecimal,
    pub transaction_hash: String,
    pub transactions: Option<Vec<TransactionInfo>>,
}

#[derive(Deserialize)]
pub struct LedgerInfo {
    pub ledger: NestedLedgerInfo,
    pub ledger_hash: String,
    pub ledger_index: BigDecimal,
    pub status: String,
    pub validated: bool,
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