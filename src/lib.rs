#![allow(non_snake_case)]
#![deny(missing_docs, missing_debug_implementations)]

//! Throttled xrp is a client for talking to a xrp node, to ask
//! it questions about the current cryptocurrency. It will ask
//! questions like figuring out the balance of an account,
//! what happened on what block, transaction. It would even ask it question that
//! are about how far along is it.
//!
//! Xrp is a currency that is supposed to take over the Western Union
//! money tranfers, so there are people that want to put money in the
//! token that is the grease for this ledger for the other money types, like USD
//! and even different crypto currencies.
//!
//! Salt uses the XRP as just the token and not as an ledger for the other
//! currencies, and won't deal with them. Instead we allow loans to be
//! backed by this token's value.

use anyhow::Result;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use throttled_json_rpc::{ClientAuth, ClientOptions, ReqBatcher, RPS};

/// A balance for xrp could be just the token or a value in
/// some other currency.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Balance {
    /// The value of just the token
    XRP(BigDecimal),
    /// Value of the other currencies
    Other {
        /// The currency is a currency code
        currency: String,
        ///
        issuer: String,
        /// Not in atomic but the canonic values of the currency
        value: BigDecimal,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/**
* Starts with r
* Length is 25-35 chars in length
* 1: https://xrpl.org/basic-data-types.html#addresses
*/
pub struct Account(String);

fn account_validate(s: &str) -> Result<String, String> {
    const MIN_LENGTH: usize = 25;
    const MAX_LENGTH: usize = 35;
    if s.len() < MIN_LENGTH {
        return Err(format!("{:?} is shorter than {} chars ", s, MIN_LENGTH));
    }
    if let Some(first_char) = s.chars().next() {
        if first_char != 'r' {
            return Err(format!("{:?} does not start with r", s));
        }
    }
    if s.len() > MAX_LENGTH {
        return Err(format!("{:?} is longer than {} chars ", s, MAX_LENGTH));
    }
    Ok(s.into())
}

impl FromStr for Account {
    type Err = String;

    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        account_validate(s).map(Account)
    }
}

/// https://xrpl.org/account_info.html
#[derive(Serialize, Debug, Clone)]
pub struct AccountInfoParams<'a> {
    ///
    pub account: &'a Account,
    /// If set to True, then the account field only accepts a public key or XRP Ledger address.
    pub strict: bool,

    ///The ledger index of the ledger to use, or a shortcut string to choose a ledger automatically
    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
    /// If true, and the MultiSign amendment is enabled, also returns any SignerList objects associated with this account
    pub queue: bool,
}

///https://xrpl.org/account_tx.html
#[derive(Serialize, Debug, Clone)]
pub struct AccountTxParams<'a, 'b> {
    ///
    pub account: &'a Account,
    ///
    pub ledger_index_min: Option<i64>,
    ///
    pub ledger_index_max: Option<i64>,
    ///
    pub ledger_hash: Option<&'b str>,

    ///
    #[serde(flatten)]
    pub ledger_index: Option<LedgerIndex>,

    ///Defaults to false. If set to true, returns transactions as hex strings instead of JSON.
    pub binary: Option<bool>,
    ///Defaults to false. If set to true, returns values indexed with the oldest ledger first. Otherwise, the results are indexed with the newest ledger first. (Each page of results may not be internally ordered, but the pages are overall ordered.)
    pub forward: Option<bool>,
    /// Default varies. Limit the number of transactions to retrieve. The server is not required to honor this value.
    pub limit: Option<u64>,
}

///https://xrpl.org/ledger.html
#[derive(Serialize, Clone, Debug)]
pub struct LedgerInfoParams {
    ///
    pub ledger_hash: Option<String>,
    ///
    #[serde(flatten)]
    pub ledger_index: Option<LedgerIndex>,
    ///Admin required If true, return full information on the entire ledger. Ignored if you did not specify a ledger version. Defaults to false. (Equivalent to enabling transactions, accounts, and expand.) Caution: This is a very large amount of data -- on the order of several hundred megabytes!
    pub full: Option<bool>,
    ///Admin required. If true, return information on accounts in the ledger. Ignored if you did not specify a ledger version. Defaults to false. Caution: This returns a very large amount of data!
    pub accounts: Option<bool>,
    ///If true, return information on transactions in the specified ledger version. Defaults to false. Ignored if you did not specify a ledger version.
    pub transactions: Option<bool>,
    /// Provide full JSON-formatted information for transaction/account information instead of only hashes. Defaults to false. Ignored unless you request transactions, accounts, or both.
    pub expand: Option<bool>,
    ///If true, include owner_funds field in the metadata of OfferCreate transactions in the response. Defaults to false. Ignored unless transactions are included and expand is true.
    pub owner_funds: Option<bool>,
    /// If true, and transactions and expand are both also true, return transaction information in binary format (hexadecimal string) instead of JSON forma
    pub binary: Option<bool>,
    ///If true, and the command is requesting the current ledger, includes an array of queued transactions in the results.
    pub queue: Option<bool>,
}

///
#[derive(Deserialize, Debug)]
pub enum LedgerEntryType {
    ///
    AccountRoot, // WHY DOES THIS EVEN EXIST???
}

///https://xrpl.org/accountroot.html
#[derive(Deserialize, Debug)]
pub struct AccountData {
    ///
    pub Account: String,
    ///
    pub Balance: BigDecimal,
    ///https://xrpl.org/accountroot.html#accountroot-flags
    ///There are several options which can be either enabled or disabled for an account. These options can be changed with an AccountSet transaction. In the ledger, flags are represented as binary values that can be combined with bitwise-or operations. The bit values for the flags in the ledger are different than the values used to enable or disable those flags in a transaction. Ledger flags have names that begin with lsf.
    pub Flags: Option<BigDecimal>,
    ///
    pub LedgerEntryType: LedgerEntryType,
    ///
    pub OwnerCount: BigDecimal,
    ///
    pub PreviousTxnID: String,
    ///
    pub PreviousTxnLgrSeq: BigDecimal,
    ///
    pub Sequence: BigDecimal,
    ///
    pub index: String,
}

///
#[derive(Deserialize, Debug)]
pub struct QueuedTransaction {
    ///
    pub LastLedgerSequence: Option<BigDecimal>,
    ///
    pub auth_change: bool,
    ///
    pub fee: BigDecimal,
    ///
    pub fee_level: BigDecimal,
    ///
    pub max_spend_drops: BigDecimal,
    ///
    pub seq: BigDecimal,
}

///
#[derive(Deserialize, Debug)]
pub struct AccountTransaction {
    ///
    pub meta: serde_json::Value,
    ///
    pub tx: AccountTransactionTx,
    ///
    pub validated: bool,
}

///
#[derive(Deserialize, Debug)]
pub struct AccountTransactionTx {
    ///
    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
}

///
#[derive(Deserialize, Debug)]
pub struct QueueData {
    ///
    pub auth_change_queued: bool,
    ///
    pub highest_sequence: BigDecimal,
    ///
    pub lowest_sequence: BigDecimal,
    ///
    pub max_spend_drops_total: BigDecimal,
    ///
    pub transactions: Vec<QueuedTransaction>,
    ///
    pub txn_count: BigDecimal,
}

/// See [1]
/// 1: https://xrpl.org/basic-data-types.html#ledger-index
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum LedgerIndex {
    ///
    Current {
        ///Ledger as index
        ledger_current_index: BigDecimal,
    },
    ///
    Number {
        /// Ledger could also be a json number
        ledger_index: serde_json::Number,
    },
    ///
    StrValue {
        /// Could be something that is a string, like "current"
        ledger_index: String,
    },
}

///https://xrpl.org/account_info.html
#[derive(Deserialize, Debug)]
pub struct AccountInfo {
    ///
    pub account_data: Option<AccountData>,
    ///https://xrpl.org/transaction-cost.html#queued-transactions
    pub queue_data: Option<LaziedQueueData>,
    ///
    pub status: String,
    ///
    pub validated: Option<bool>,
    ///

    ///
    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
}

/// See [1]
/// 1: https://xrpl.org/account_tx.html
#[derive(Deserialize, Debug)]
pub struct AccountTx {
    ///
    pub account: Account,
    ///
    pub ledger_index_min: i64,
    ///
    pub ledger_index_max: i64,
    ///
    pub limit: i64,
    ///
    pub transactions: Vec<AccountTransaction>,
}

///Some fields may be omitted because the values are calculated "lazily" by the queuing mechanism. [1]
/// 1: https://xrpl.org/account_info.html
#[derive(Deserialize, Debug)]
pub struct LaziedQueueData {
    ///
    pub auth_change_queued: Option<bool>,
    ///
    pub highest_sequence: Option<BigDecimal>,
    ///
    pub lowest_sequence: Option<BigDecimal>,
    ///
    pub max_spend_drops_total: Option<BigDecimal>,
    ///
    pub transactions: Option<Vec<QueuedTransaction>>,
    ///
    pub txn_count: Option<BigDecimal>,
}

///
#[derive(Deserialize, Debug)]
pub struct PathInfo {
    ///
    pub currency: Option<String>,
    ///
    pub issuer: Option<String>,
    ///
    #[serde(rename = "type")]
    pub currency_type: BigDecimal,
    ///
    pub type_hex: String,
}

///
#[derive(Deserialize, Debug)]
pub struct FieldInfo {
    ///
    pub Account: Option<String>,
    ///
    pub Balance: Option<Balance>,
    ///
    pub Flags: Option<isize>,
    ///
    pub OwnerCount: Option<BigDecimal>,
    ///
    pub Sequence: Option<BigDecimal>,
}

///
#[derive(Deserialize, Debug)]
pub struct PreviousFieldInfo {
    ///
    pub Balance: Option<Balance>,
    ///
    pub Sequence: Option<BigDecimal>,
}

///
#[derive(Deserialize, Debug)]
pub struct ModifiedNodeInfo {
    ///
    pub FinalFields: Option<FieldInfo>,
    ///
    pub PreviousFields: Option<FieldInfo>, // is this really optional ???
    ///
    pub LedgerEntryType: String,
    ///
    pub LedgerIndex: String,
    ///
    pub PreviousTxnID: Option<String>,
    ///
    pub PreviousTxnLgrSeq: Option<BigDecimal>,
}

///
#[derive(Deserialize, Debug)]
pub struct CreatedNodeInfo {
    ///
    pub LedgerEntryType: String,
    ///
    pub LedgerIndex: String,
    ///
    pub NewFields: Option<FieldInfo>,
}

///
#[derive(Deserialize, Debug)]
pub struct DeletedNodeInfo {
    ///
    pub LedgerEntryType: String,
    ///
    pub LedgerIndex: String,
    ///
    pub FinalFields: Option<FieldInfo>,
}

///
#[derive(Deserialize, Debug)]
pub struct AffectedNodeInfo {
    ///
    pub ModifiedNode: Option<ModifiedNodeInfo>,
    ///
    pub CreatedNode: Option<CreatedNodeInfo>,
    ///
    pub DeletedNode: Option<DeletedNodeInfo>,
}

///
#[derive(Deserialize, Debug)]
pub struct MetaTxInfo {
    ///
    pub AffectedNodes: Vec<AffectedNodeInfo>,
    ///
    pub TransactionIndex: BigDecimal,
    ///
    pub TransactionResult: String,
}

///
#[derive(Deserialize, Debug)]
pub struct TransactionInfo {
    ///
    pub Account: String,
    ///
    pub Amount: Option<Balance>,
    ///
    pub Destination: Option<String>,
    ///
    pub Fee: BigDecimal,
    ///
    pub Flags: Option<isize>,
    ///
    pub Paths: Option<Vec<Vec<PathInfo>>>,
    ///
    pub SendMax: Option<Balance>,
    ///
    pub Sequence: BigDecimal,
    ///
    pub SigningPubKey: String,
    ///
    pub TransactionType: String,
    ///
    pub TxnSignature: Option<String>,
    ///
    pub hash: String,
    ///
    pub metaData: Option<MetaTxInfo>,
    ///
    pub validated: Option<bool>, //option of a bool???
}

///
#[derive(Deserialize, Debug)]
pub struct NestedLedgerInfo {
    ///
    pub accepted: bool,
    ///
    pub account_hash: String,
    ///
    pub close_flags: isize,
    ///
    pub close_time: BigDecimal,
    ///
    pub close_time_human: String,
    ///
    pub close_time_resolution: BigDecimal,
    ///
    pub closed: bool,
    ///
    pub hash: String,
    ///
    pub ledger_hash: String,
    ///
    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
    ///
    pub parent_close_time: BigDecimal,
    ///
    pub parent_hash: String,
    ///
    pub seqNum: String,
    ///
    pub totalCoins: String,
    ///
    pub total_coins: BigDecimal,
    ///
    pub transaction_hash: String,
    ///
    pub transactions: Option<Vec<TransactionInfo>>,
}

///
#[derive(Deserialize, Debug)]
pub struct LedgerInfo {
    ///
    pub ledger: Option<NestedLedgerInfo>,
    ///
    pub ledger_hash: Option<String>,
    ///
    #[serde(flatten)]
    pub ledger_index: LedgerIndex,
    ///
    pub status: String,
    ///
    pub validated: bool,
}

/// The client of the system, try and have as few of these as possible created, better to just clone
/// them with the batcher than create a new one.
#[derive(Clone, Debug)]
pub struct XRPClient {
    request_batcher: ReqBatcher,
}
impl XRPClient {
    /// Create a client needs to know where to point, like the url, with  the passwords and
    /// limitations of the server, like how many rps to allow. There are cases where the rps may
    /// not be valid, like negative, so the result could fail.
    pub fn new(
        uri: String,
        user: Option<String>,
        pass: Option<String>,
        max_concurrency: usize,
        rps: f64,
        max_batch_size: usize,
    ) -> Result<Self> {
        Ok(XRPClient {
            request_batcher: ReqBatcher::new(ClientOptions {
                uri,
                batching: max_batch_size,
                rps: RPS::new(rps)?,
                concurrent: max_concurrency,
                client_auth: user.map(|user| ClientAuth {
                    user,
                    password: pass,
                }),
            }),
        })
    }
    /// Account info is good for getting the balance. See [1] for the source of the
    /// documentation
    ///
    /// 1:https://xrpl.org/account_info.html
    pub async fn account_info(&mut self, params: &AccountInfoParams<'_>) -> Result<AccountInfo> {
        self.request_batcher
            .request(
                "account_info".to_string(),
                vec![serde_json::to_value(params)?],
            )
            .await
    }
    /// Account retrieves the transaction fot the given account. See [1] for the source
    /// of the documentation.
    ///
    /// 1: https://xrpl.org/account_tx.html#main_content_body
    pub async fn account_tx(&mut self, params: &AccountTxParams<'_, '_>) -> Result<AccountTx> {
        self.request_batcher
            .request(
                "account_tx".to_string(),
                vec![serde_json::to_value(params)?],
            )
            .await
    }
    /// This is the meat of the scanner, it gets the information of a block. See [1]
    /// for the source of the documenation
    ///
    /// 1: https://xrpl.org/ledger.html
    pub async fn ledger(&mut self, params: &LedgerInfoParams) -> Result<LedgerInfo> {
        self.request_batcher
            .request("ledger".to_string(), vec![serde_json::to_value(params)?])
            .await
    }
}

#[test]
fn json_test() {
    let _: LedgerInfo =
        serde_json::from_reader(std::fs::File::open("ledger.json").unwrap()).unwrap();
}

#[test]
fn json_ledger_test() {
    #[derive(Deserialize)]
    struct RpcResponse<T> {
        pub result: Option<T>,
        pub error: Option<serde_json::Value>,
        pub id: Option<usize>,
    }
    let _test_data: RpcResponse<LedgerInfo> = serde_json::from_str(
        r#"{
  "result": {
    "ledger": {
      "accepted": true,
      "account_hash": "CFA12FBAFC585D54858874ADACB1003CB4218B010CF5F8AB4C4984B194E95B4B",
      "close_flags": 0,
      "close_time": 620860251,
      "close_time_human": "2019-Sep-03 21:10:51.000000000",
      "close_time_resolution": 10,
      "closed": true,
      "hash": "30BC3B59A2DCB4BC402637A1DEE3F22C6AC4D09E2CDFCAE8C84F11D7E6E251F5",
      "ledger_hash": "30BC3B59A2DCB4BC402637A1DEE3F22C6AC4D09E2CDFCAE8C84F11D7E6E251F5",
      "ledger_index": "105938",
      "parent_close_time": 620860250,
      "parent_hash": "197B5016B33A79CECA4AA704B534D5999A9674FAD9CBDD82309835D7A784A35F",
      "seqNum": "105938",
      "totalCoins": "99999999522468910",
      "total_coins": "99999999522468910",
      "transaction_hash": "55771C3FB148C470D36B4AE4F91D402F60C39920649C9D2C3E1829104E82654F"
    },
    "ledger_hash": "30BC3B59A2DCB4BC402637A1DEE3F22C6AC4D09E2CDFCAE8C84F11D7E6E251F5",
    "ledger_index": 105938,
    "status": "success",
    "validated": true
  }
}
"#,
    )
    .unwrap();
}
