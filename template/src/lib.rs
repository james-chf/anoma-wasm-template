{% if wasm_type == "transaction" %}use anoma_tx_prelude::*;

#[transaction]
fn apply_tx(tx_data: Vec<u8>) {
    log_string(format!("apply_tx called with data: {:#?}", tx_data));
}

#[cfg(test)]
mod tests {
    use anoma_tests::tx::*;

    use super::*;

    /// An example test, checking that this transaction performs no storage
    /// modifications.
    #[test]
    fn test_no_op_transaction() {
        // The environment must be initialized first
        tx_host_env::init();

        let tx_data = vec![];
        apply_tx(tx_data);

        let env = tx_host_env::take();
        assert!(env.all_touched_storage_keys().is_empty());
    }
}{% endif %}{% if wasm_type == "validity predicate" %}use anoma_vp_prelude::*;

#[validity_predicate]
fn validate_tx(
    tx_data: Vec<u8>,
    addr: Address,
    keys_changed: BTreeSet<storage::Key>,
    verifiers: BTreeSet<Address>,
) -> bool {
    log_string(format!(
        "validate_tx called with addr: {}, key_changed: {:#?}, tx_data: \
         {:#?}, verifiers: {:?}",
        addr, keys_changed, tx_data, verifiers
    ));

    for key in keys_changed.iter() {
        let key = key.to_string();
        let pre: Option<u64> = read_pre(&key);
        let post: Option<u64> = read_post(&key);
        log_string(format!(
            "validate_tx key: {}, pre: {:#?}, post: {:#?}",
            key, pre, post,
        ));
    }
    true
}{% endif %}