use crate::ffi::{
    client::LedgerClient,
    error::{VdrError, VdrResult},
    transaction::{Transaction, TransactionEndorsingData},
    types::SignatureData,
};
use indy_besu_vdr::{
    legacy_mapping_registry, Address, Ed25519Signature, LegacyDid, LegacyVerkey,
    ResourceIdentifier, DID,
};

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_did_mapping_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    legacy_identifier: &str,
    legacy_verkey: &str,
    ed25519_signature: &[u8],
) -> VdrResult<Transaction> {
    let transaction = legacy_mapping_registry::build_create_did_mapping_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &LegacyDid::from(legacy_identifier),
        &LegacyVerkey::from(legacy_verkey),
        &Ed25519Signature::from(ed25519_signature),
    )
    .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_did_mapping_endorsing_data(
    client: &LedgerClient,
    did: &str,
    legacy_identifier: &str,
    legacy_verkey: &str,
    ed25519_signature: &[u8],
) -> VdrResult<TransactionEndorsingData> {
    legacy_mapping_registry::build_create_did_mapping_endorsing_data(
        &client.client,
        &DID::from(did),
        &LegacyDid::from(legacy_identifier),
        &LegacyVerkey::from(legacy_verkey),
        &Ed25519Signature::from(ed25519_signature),
    )
    .await
    .map(TransactionEndorsingData::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_did_mapping_signed_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    legacy_identifier: &str,
    legacy_verkey: &str,
    ed25519_signature: &[u8],
    signature: SignatureData,
) -> VdrResult<Transaction> {
    legacy_mapping_registry::build_create_did_mapping_signed_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &LegacyDid::from(legacy_identifier),
        &LegacyVerkey::from(legacy_verkey),
        &Ed25519Signature::from(ed25519_signature),
        &signature.into(),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_did_mapping_transaction(
    client: &LedgerClient,
    legacy_identifier: &str,
) -> VdrResult<Transaction> {
    legacy_mapping_registry::build_get_did_mapping_transaction(
        &client.client,
        &LegacyDid::from(legacy_identifier),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export]
pub fn parse_did_mapping_result(client: &LedgerClient, bytes: Vec<u8>) -> VdrResult<String> {
    let did = legacy_mapping_registry::parse_did_mapping_result(&client.client, &bytes)?;
    Ok(did.to_string())
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_resource_mapping_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    legacy_issuer_identifier: &str,
    legacy_identifier: &str,
    new_identifier: &str,
) -> VdrResult<Transaction> {
    let transaction = legacy_mapping_registry::build_create_resource_mapping_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &LegacyDid::from(legacy_issuer_identifier),
        &ResourceIdentifier::from(legacy_identifier),
        &ResourceIdentifier::from(new_identifier),
    )
    .await?;
    Ok(Transaction { transaction })
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_resource_mapping_endorsing_data(
    client: &LedgerClient,
    did: &str,
    legacy_issuer_identifier: &str,
    legacy_identifier: &str,
    new_identifier: &str,
) -> VdrResult<TransactionEndorsingData> {
    legacy_mapping_registry::build_create_resource_mapping_endorsing_data(
        &client.client,
        &DID::from(did),
        &LegacyDid::from(legacy_issuer_identifier),
        &ResourceIdentifier::from(legacy_identifier),
        &ResourceIdentifier::from(new_identifier),
    )
    .await
    .map(TransactionEndorsingData::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_create_resource_mapping_signed_transaction(
    client: &LedgerClient,
    from: &str,
    did: &str,
    legacy_issuer_identifier: &str,
    legacy_identifier: &str,
    new_identifier: &str,
    signature: SignatureData,
) -> VdrResult<Transaction> {
    legacy_mapping_registry::build_create_resource_mapping_signed_transaction(
        &client.client,
        &Address::from(from),
        &DID::from(did),
        &LegacyDid::from(legacy_issuer_identifier),
        &ResourceIdentifier::from(legacy_identifier),
        &ResourceIdentifier::from(new_identifier),
        &signature.into(),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn build_get_resource_mapping_transaction(
    client: &LedgerClient,
    legacy_identifier: &str,
) -> VdrResult<Transaction> {
    legacy_mapping_registry::build_get_resource_mapping_transaction(
        &client.client,
        &ResourceIdentifier::from(legacy_identifier),
    )
    .await
    .map(Transaction::from)
    .map_err(VdrError::from)
}

#[uniffi::export]
pub fn parse_resource_mapping_result(client: &LedgerClient, bytes: Vec<u8>) -> VdrResult<String> {
    let identifier =
        legacy_mapping_registry::parse_resource_mapping_result(&client.client, &bytes)?;
    Ok(identifier.to_string())
}
