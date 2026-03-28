use crate::models::LedgerEntryPartialSerialize;
use soroban_env_host::xdr::{
    AccountEntry, AccountId, LedgerEntry, LedgerEntryData, Limits, PublicKey, SequenceNumber,
    Thresholds, Uint256, WriteXdr,
};

#[test]
fn test_partial_serialization_size() {
    let account_id = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([0u8; 32])));
    let account_entry = AccountEntry {
        account_id,
        balance: 1000,
        seq_num: SequenceNumber(1),
        num_sub_entries: 0,
        inflation_dest: None,
        flags: 0,
        home_domain: Default::default(),
        thresholds: Thresholds([1, 0, 0, 0]),
        signers: Default::default(),
        ext: Default::default(),
    };

    use soroban_env_host::xdr::{LedgerEntryExt, LedgerEntryExtensionV1, LedgerEntryExtensionV1Ext, SponsorshipDescriptor};
    let ext_v1 = LedgerEntryExtensionV1 {
        sponsoring_id: SponsorshipDescriptor(Some(account_id.clone())),
        ext: LedgerEntryExtensionV1Ext::V0,
    };

    let entry = LedgerEntry {
        last_modified_ledger_seq: 1,
        data: LedgerEntryData::Account(account_entry),
        ext: LedgerEntryExt::V1(ext_v1),
    };

    let full_xdr = entry.to_xdr(Limits::none()).expect("failed to serialize full");
    let partial_xdr = entry.partial_serialize().expect("failed to serialize partial");

    let reduction = 1.0 - (partial_xdr.len() as f64 / full_xdr.len() as f64);
    assert!(
        reduction >= 0.20,
        "Partial size {} is not 20% smaller than full size {} (reduction: {:.2}%)",
        partial_xdr.len(),
        full_xdr.len(),
        reduction * 100.0
    );

    // Verify round-trip integrity
    let deserialized = LedgerEntry::partial_deserialize(&partial_xdr).expect("failed to deserialize partial");
    assert_eq!(entry.last_modified_ledger_seq, deserialized.last_modified_ledger_seq);
    // Since partial_deserialize forces ext: V0 and we used default (which is V0) they will match.
}
