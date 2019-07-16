use serde_json::json;
use serde_json::value::Value;
use throttled_xrp_rpc::{AccountParams, LedgerIndex, XRPClient};

// FIXME: Use "http://10.68.2.93:5005" instead
const URL: &'static str = "https://s1.ripple.com:51234/";
#[test]
fn account_info_tests() {
    let bitpay_account_id = "r9HwsqBnAUN4nF6nDqxd4sgP8DrDnDcZP3";
    let client = reqwest::Client::new();

    let account_params = AccountParams {
        account: &bitpay_account_id,
        strict: true,
        ledger_index: LedgerIndex::StrValue {
            ledger_index: "current".into(),
        },
        queue: true,
    };
    let raw_response = client
        .post(URL.clone())
        .json(&json!({
        "method": "account_info",
        "params": [
            account_params
        ]
        }))
        .send()
        .unwrap()
        .json::<Value>();
    let account_response =
        XRPClient::new(URL.into(), None, None, 0, 0, 0).account_info(account_params.clone());
    assert!(
        account_response.is_ok(),
        "Getting back an error {:?} from the server given the input {:?}, raw was {:?}",
        account_response,
        serde_json::to_string(&account_params),
        raw_response
    );

    let account_params = AccountParams {
        account: &bitpay_account_id,
        strict: false,
        ledger_index: LedgerIndex::StrValue {
            ledger_index: "current".into(),
        },
        queue: false,
    };
    let raw_response = client
        .post(URL.clone())
        .json(&json!({
        "method": "account_info",
        "params": [
            account_params
        ]
        }))
        .send()
        .unwrap()
        .json::<Value>();
    let account_response =
        XRPClient::new(URL.into(), None, None, 0, 0, 0).account_info(account_params.clone());
    assert!(
        account_response.is_ok(),
        "Getting back an error {:?} from the server given the input {:?}, raw was {:?}",
        account_response,
        serde_json::to_string(&account_params),
        raw_response
    );

    let account_params = AccountParams {
        account: &bitpay_account_id,
        strict: true,
        ledger_index: LedgerIndex::StrValue {
            ledger_index: "false".into(),
        },
        queue: true,
    };
    let raw_response = client
        .post(URL.clone())
        .json(&json!({
        "method": "account_info",
        "params": [
            account_params
        ]
        }))
        .send()
        .unwrap()
        .json::<Value>();
    let account_response =
        XRPClient::new(URL.into(), None, None, 0, 0, 0).account_info(account_params.clone());
    assert!(
        account_response.is_err(),
        "Getting back an ok {:?} from the server given the input {:?}, raw was {:?}",
        account_response,
        serde_json::to_string(&account_params),
        raw_response
    );

    let account_params = AccountParams {
        account: &bitpay_account_id,
        strict: false,
        ledger_index: LedgerIndex::Number {
            ledger_index: 48694757.into(),
        },
        queue: false,
    };
    let raw_response = client
        .post(URL.clone())
        .json(&json!({
        "method": "account_info",
        "params": [
            account_params
        ]
        }))
        .send()
        .unwrap()
        .json::<Value>();
    let account_response =
        XRPClient::new(URL.into(), None, None, 0, 0, 0).account_info(account_params.clone());
    assert!(
        account_response.is_ok(),
        "Getting back an error {:?} from the server given the input {:?}, raw was {:?}",
        account_response,
        serde_json::to_string(&account_params),
        raw_response
    );
}
