use std::io::Cursor;
use std::str::FromStr;

use bitcoin::consensus::Decodable;
use bitcoin30::address::NetworkUnchecked;
use bitcoin30::consensus::Encodable;
use bitcoin30::hashes::Hash as Bitcoin30Hash;
use bitcoin_hashes::Hash as Bitcoin29Hash;

pub fn bitcoin29_to_bitcoin30_public_key(pk: bitcoin::PublicKey) -> bitcoin30::PublicKey {
    bitcoin30::PublicKey::from_slice(&pk.to_bytes())
        .expect("Failed to convert bitcoin v29 public key to bitcoin v30 public key")
}

pub fn bitcoin29_to_bitcoin30_secp256k1_public_key(
    pk: bitcoin::secp256k1::PublicKey,
) -> bitcoin30::secp256k1::PublicKey {
    bitcoin30::secp256k1::PublicKey::from_slice(&pk.serialize()).expect(
        "Failed to convert bitcoin v29 secp256k1 public key to bitcoin v30 secp256k1 public key",
    )
}

pub fn bitcoin30_to_bitcoin29_secp256k1_public_key(
    pk: bitcoin30::secp256k1::PublicKey,
) -> bitcoin::secp256k1::PublicKey {
    bitcoin::secp256k1::PublicKey::from_slice(&pk.serialize()).expect(
        "Failed to convert bitcoin v30 secp256k1 public key to bitcoin v29 secp256k1 public key",
    )
}

pub fn bitcoin29_to_bitcoin30_secp256k1_secret_key(
    sk: bitcoin::secp256k1::SecretKey,
) -> bitcoin30::secp256k1::SecretKey {
    bitcoin30::secp256k1::SecretKey::from_slice(&sk[..]).expect(
        "Failed to convert bitcoin v29 secp256k1 secret key to bitcoin v30 secp256k1 secret key",
    )
}

pub fn bitcoin30_to_bitcoin29_secp256k1_secret_key(
    sk: bitcoin30::secp256k1::SecretKey,
) -> bitcoin::secp256k1::SecretKey {
    bitcoin::secp256k1::SecretKey::from_slice(&sk[..]).expect(
        "Failed to convert bitcoin v30 secp256k1 secret key to bitcoin v29 secp256k1 secret key",
    )
}

pub fn bitcoin29_to_bitcoin30_keypair(keypair: bitcoin::KeyPair) -> bitcoin30::secp256k1::KeyPair {
    bitcoin30::secp256k1::KeyPair::from_secret_key(
        bitcoin30::secp256k1::SECP256K1,
        &bitcoin29_to_bitcoin30_secp256k1_secret_key(keypair.secret_key()),
    )
}

pub fn bitcoin30_to_bitcoin29_keypair(keypair: bitcoin30::secp256k1::KeyPair) -> bitcoin::KeyPair {
    bitcoin::KeyPair::from_secret_key(
        bitcoin::secp256k1::SECP256K1,
        &bitcoin30_to_bitcoin29_secp256k1_secret_key(keypair.secret_key()),
    )
}

pub fn bitcoin29_to_bitcoin30_schnorr_signature(
    signature: bitcoin::secp256k1::schnorr::Signature,
) -> bitcoin30::secp256k1::schnorr::Signature {
    bitcoin30::secp256k1::schnorr::Signature::from_str(&signature.to_string())
        .expect("Failed to convert bitcoin v29 schnorr signature to bitcoin v30 schnorr signature")
}

pub fn bitcoin30_to_bitcoin29_schnorr_signature(
    signature: bitcoin30::secp256k1::schnorr::Signature,
) -> bitcoin::secp256k1::schnorr::Signature {
    bitcoin::secp256k1::schnorr::Signature::from_str(&signature.to_string())
        .expect("Failed to convert bitcoin v30 schnorr signature to bitcoin v29 schnorr signature")
}

pub fn bitcoin29_to_bitcoin30_message(
    message: bitcoin::secp256k1::Message,
) -> bitcoin30::secp256k1::Message {
    bitcoin30::secp256k1::Message::from_slice(&message[..])
        .expect("Failed to convert bitcoin v29 message to bitcoin v30 message")
}

pub fn bitcoin30_to_bitcoin29_message(
    message: bitcoin30::secp256k1::Message,
) -> bitcoin::secp256k1::Message {
    bitcoin::secp256k1::Message::from_slice(&message[..])
        .expect("Failed to convert bitcoin v30 message to bitcoin v29 message")
}

pub fn bitcoin29_to_bitcoin30_network(network: bitcoin::Network) -> bitcoin30::Network {
    match network {
        bitcoin::Network::Bitcoin => bitcoin30::Network::Bitcoin,
        bitcoin::Network::Testnet => bitcoin30::Network::Testnet,
        bitcoin::Network::Signet => bitcoin30::Network::Signet,
        bitcoin::Network::Regtest => bitcoin30::Network::Regtest,
    }
}

pub fn bitcoin30_to_bitcoin29_network(network: bitcoin30::Network) -> bitcoin::Network {
    match network {
        bitcoin30::Network::Bitcoin => bitcoin::Network::Bitcoin,
        bitcoin30::Network::Testnet => bitcoin::Network::Testnet,
        bitcoin30::Network::Signet => bitcoin::Network::Signet,
        bitcoin30::Network::Regtest => bitcoin::Network::Regtest,
        unknown_network => panic!(
            "Failed to convert bitcoin v30 network to bitcoin v29 network: {unknown_network}"
        ),
    }
}

pub fn bitcoin29_to_bitcoin30_amount(amount: bitcoin::Amount) -> bitcoin30::Amount {
    bitcoin30::Amount::from_sat(amount.to_sat())
}

pub fn bitcoin30_to_bitcoin29_amount(amount: bitcoin30::Amount) -> bitcoin::Amount {
    bitcoin::Amount::from_sat(amount.to_sat())
}

pub fn bitcoin29_to_bitcoin30_outpoint(outpoint: bitcoin::OutPoint) -> bitcoin30::OutPoint {
    bitcoin30::OutPoint {
        txid: bitcoin29_to_bitcoin30_txid(outpoint.txid),
        vout: outpoint.vout,
    }
}

pub fn bitcoin30_to_bitcoin29_outpoint(outpoint: bitcoin30::OutPoint) -> bitcoin::OutPoint {
    bitcoin::OutPoint {
        txid: bitcoin30_to_bitcoin29_txid(outpoint.txid),
        vout: outpoint.vout,
    }
}

pub fn bitcoin29_to_bitcoin30_witness(witness: &bitcoin::Witness) -> bitcoin30::Witness {
    bitcoin30::Witness::from_slice(&witness.to_vec())
}

pub fn bitcoin30_to_bitcoin29_witness(witness: &bitcoin30::Witness) -> bitcoin::Witness {
    bitcoin::Witness::from_vec(witness.to_vec())
}

fn bitcoin29_to_bitcoin30_txin(input: &bitcoin::TxIn) -> bitcoin30::TxIn {
    bitcoin30::TxIn {
        previous_output: bitcoin29_to_bitcoin30_outpoint(input.previous_output),
        script_sig: bitcoin29_to_bitcoin30_script(input.script_sig.clone()),
        sequence: bitcoin30::Sequence(input.sequence.0),
        witness: bitcoin29_to_bitcoin30_witness(&input.witness),
    }
}

fn bitcoin30_to_bitcoin29_txin(input: &bitcoin30::TxIn) -> bitcoin::TxIn {
    bitcoin::TxIn {
        previous_output: bitcoin30_to_bitcoin29_outpoint(input.previous_output),
        script_sig: bitcoin30_to_bitcoin29_script(&input.script_sig),
        sequence: bitcoin::Sequence(input.sequence.0),
        witness: bitcoin30_to_bitcoin29_witness(&input.witness),
    }
}

fn bitcoin29_to_bitcoin30_txout(output: &bitcoin::TxOut) -> bitcoin30::TxOut {
    bitcoin30::TxOut {
        value: output.value,
        script_pubkey: bitcoin29_to_bitcoin30_script(output.script_pubkey.clone()),
    }
}

fn bitcoin30_to_bitcoin29_txout(output: &bitcoin30::TxOut) -> bitcoin::TxOut {
    bitcoin::TxOut {
        value: output.value,
        script_pubkey: bitcoin30_to_bitcoin29_script(&output.script_pubkey),
    }
}

pub fn bitcoin29_to_bitcoin30_transaction(
    transaction: &bitcoin::Transaction,
) -> bitcoin30::Transaction {
    bitcoin30::Transaction {
        version: transaction.version,
        lock_time: bitcoin30::absolute::LockTime::from_consensus(transaction.lock_time.0),
        input: transaction
            .input
            .iter()
            .map(bitcoin29_to_bitcoin30_txin)
            .collect(),
        output: transaction
            .output
            .iter()
            .map(bitcoin29_to_bitcoin30_txout)
            .collect(),
    }
}

pub fn bitcoin30_to_bitcoin29_transaction(
    transaction: &bitcoin30::Transaction,
) -> bitcoin::Transaction {
    bitcoin::Transaction {
        version: transaction.version,
        lock_time: match transaction.lock_time {
            bitcoin30::absolute::LockTime::Blocks(height) => {
                bitcoin::LockTime::from_height(height.to_consensus_u32())
                    .expect("Failed to convert bitcoin v30 lock time to bitcoin v29 lock time")
            }
            bitcoin30::absolute::LockTime::Seconds(time) => {
                bitcoin::LockTime::from_time(time.to_consensus_u32())
                    .expect("Failed to convert bitcoin v30 lock time to bitcoin v29 lock time")
            }
        }
        .into(),
        input: transaction
            .input
            .iter()
            .map(bitcoin30_to_bitcoin29_txin)
            .collect(),
        output: transaction
            .output
            .iter()
            .map(bitcoin30_to_bitcoin29_txout)
            .collect(),
    }
}

pub fn bitcoin30_to_bitcoin29_tx_merkle_node(
    node: bitcoin30::hash_types::TxMerkleNode,
) -> bitcoin::TxMerkleNode {
    bitcoin::TxMerkleNode::from_inner(*node.to_raw_hash().as_byte_array())
}

pub fn bitcoin29_to_bitcoin30_block_hash(hash: bitcoin::BlockHash) -> bitcoin30::BlockHash {
    bitcoin30::BlockHash::from_byte_array(hash.into_inner())
}

pub fn bitcoin30_to_bitcoin29_block_hash(hash: bitcoin30::BlockHash) -> bitcoin::BlockHash {
    bitcoin::BlockHash::from_slice(&hash[..])
        .expect("Failed to convert bitcoin v30 block hash to bitcoin v29 block hash")
}

pub fn bitcoin30_to_bitcoin29_block_header(
    header: bitcoin30::block::Header,
) -> bitcoin::BlockHeader {
    bitcoin::BlockHeader {
        version: header.version.to_consensus(),
        prev_blockhash: bitcoin30_to_bitcoin29_block_hash(header.prev_blockhash),
        merkle_root: bitcoin30_to_bitcoin29_tx_merkle_node(header.merkle_root),
        time: header.time,
        bits: header.bits.to_consensus(),
        nonce: header.nonce,
    }
}

pub fn bitcoin30_to_bitcoin29_partial_merkle_tree(
    tree: bitcoin30::merkle_tree::PartialMerkleTree,
) -> bitcoin::util::merkleblock::PartialMerkleTree {
    let mut writer = Vec::new();
    tree.consensus_encode(&mut writer)
        .expect("Failed to encode bitcoin v30 partial merkle tree");
    bitcoin::util::merkleblock::PartialMerkleTree::consensus_decode(&mut Cursor::new(writer))
        .expect(
            "Failed to decode bitcoin v30 partial merkle tree into bitcoin v29 partial merkle tree",
        )
}

pub fn bitcoin29_to_bitcoin30_txid(txid: bitcoin::Txid) -> bitcoin30::Txid {
    bitcoin30::Txid::from_byte_array(txid.into_inner())
}

pub fn bitcoin30_to_bitcoin29_txid(txid: bitcoin30::Txid) -> bitcoin::Txid {
    bitcoin::Txid::from_slice(&txid[..])
        .expect("Failed to convert bitcoin v30 txid to bitcoin v29 txid")
}

pub fn bitcoin29_to_bitcoin30_script(script: bitcoin::Script) -> bitcoin30::ScriptBuf {
    bitcoin30::ScriptBuf::from_bytes(script.into_bytes())
}

pub fn bitcoin30_to_bitcoin29_script(script: &bitcoin30::ScriptBuf) -> bitcoin::Script {
    script.to_bytes().into()
}

pub fn bitcoin29_to_bitcoin30_address(
    address: bitcoin::Address,
) -> bitcoin30::Address<NetworkUnchecked> {
    bitcoin30::Address::from_str(&address.to_string())
        .expect("Failed to convert bitcoin v29 address to bitcoin v30 address")
}

pub fn bitcoin30_to_bitcoin29_address(address: bitcoin30::Address) -> bitcoin::Address {
    bitcoin::Address::from_str(&address.to_string())
        .expect("Failed to convert bitcoin v30 address to bitcoin v29 address")
}

pub fn bitcoin29_to_bitcoin30_ripemd160_hash(
    hash: bitcoin::hashes::ripemd160::Hash,
) -> bitcoin30::hashes::ripemd160::Hash {
    bitcoin30::hashes::ripemd160::Hash::from_byte_array(hash.into_inner())
}

pub fn bitcoin29_to_bitcoin30_hash160_hash(
    hash: bitcoin::hashes::hash160::Hash,
) -> bitcoin30::hashes::hash160::Hash {
    bitcoin30::hashes::hash160::Hash::from_byte_array(hash.into_inner())
}

pub fn bitcoin29_to_bitcoin30_sha256_hash(
    hash: bitcoin::hashes::sha256::Hash,
) -> bitcoin30::hashes::sha256::Hash {
    bitcoin30::hashes::sha256::Hash::from_byte_array(hash.into_inner())
}

#[cfg(test)]
mod tests {
    use bitcoin30::absolute::Height;
    use bitcoin_hashes::hex::FromHex;
    use rand::thread_rng;

    use super::*;

    fn bitcoin30_to_bitcoin29_public_key(pk: bitcoin30::PublicKey) -> bitcoin::PublicKey {
        bitcoin::PublicKey::from_slice(&pk.to_bytes())
            .expect("Failed to convert bitcoin v30 public key to bitcoin v29 public key")
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_and_back_public_key() {
        let bitcoin29_pk: bitcoin::PublicKey =
            "037703ba67395870e5237787f380708f1b13751f7bdd97682e8f5af3f3a10a0a52"
                .parse()
                .expect("Failed to parse bitcoin v29 public key");

        let bitcoin30_pk = bitcoin29_to_bitcoin30_public_key(bitcoin29_pk);
        let bitcoin29_pk_back = bitcoin30_to_bitcoin29_public_key(bitcoin30_pk);

        assert_eq!(bitcoin29_pk, bitcoin29_pk_back);
    }

    #[test]
    fn test_bitcoin30_to_bitcoin29_and_back_public_key() {
        let bitcoin30_pk: bitcoin30::PublicKey =
            "0355aba9599a27e71eb515c813252f630c5914eb76b199b11db4265107619e8dcc"
                .parse()
                .expect("Failed to parse bitcoin v30 public key");

        let bitcoin29_pk = bitcoin30_to_bitcoin29_public_key(bitcoin30_pk);
        let bitcoin30_pk_back = bitcoin29_to_bitcoin30_public_key(bitcoin29_pk);

        assert_eq!(bitcoin30_pk, bitcoin30_pk_back);
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_and_back_secp256k1_public_key() {
        let keypair = bitcoin::KeyPair::new(bitcoin::secp256k1::SECP256K1, &mut thread_rng());
        let bitcoin29_pk: bitcoin::secp256k1::PublicKey =
            bitcoin::secp256k1::PublicKey::from_keypair(&keypair);

        let bitcoin30_pk = bitcoin29_to_bitcoin30_secp256k1_public_key(bitcoin29_pk);
        let bitcoin29_pk_back = bitcoin30_to_bitcoin29_secp256k1_public_key(bitcoin30_pk);

        assert_eq!(bitcoin29_pk, bitcoin29_pk_back);
    }

    #[test]
    fn test_bitcoin30_to_bitcoin29_and_back_secp256k1_public_key() {
        let keypair = bitcoin30::secp256k1::KeyPair::new(
            &bitcoin30::secp256k1::Secp256k1::new(),
            &mut thread_rng(),
        );
        let bitcoin30_pk: bitcoin30::secp256k1::PublicKey =
            bitcoin30::secp256k1::PublicKey::from_keypair(&keypair);

        let bitcoin29_pk = bitcoin30_to_bitcoin29_secp256k1_public_key(bitcoin30_pk);
        let bitcoin30_pk_back = bitcoin29_to_bitcoin30_secp256k1_public_key(bitcoin29_pk);

        assert_eq!(bitcoin30_pk, bitcoin30_pk_back);
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_and_back_secp256k1_secret_key() {
        let bitcoin29_sk: bitcoin::secp256k1::SecretKey =
            bitcoin::secp256k1::SecretKey::from_slice(&[
                0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45, 0x67,
                0x89, 0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45,
                0x67, 0x89, 0x01, 0x23,
            ])
            .expect("Failed to parse bitcoin v29 secp256k1 secret key");

        let bitcoin30_sk = bitcoin29_to_bitcoin30_secp256k1_secret_key(bitcoin29_sk);
        let bitcoin29_sk_back = bitcoin30_to_bitcoin29_secp256k1_secret_key(bitcoin30_sk);

        assert_eq!(bitcoin29_sk, bitcoin29_sk_back);
    }

    #[test]
    fn test_bitcoin30_to_bitcoin29_and_back_secp256k1_secret_key() {
        let bitcoin30_sk: bitcoin30::secp256k1::SecretKey =
            bitcoin30::secp256k1::SecretKey::from_slice(&[
                0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45, 0x67,
                0x89, 0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45, 0x67, 0x89, 0x01, 0x23, 0x45,
                0x67, 0x89, 0x01, 0x23,
            ])
            .expect("Failed to parse bitcoin v30 secp256k1 secret key");

        let bitcoin29_sk = bitcoin30_to_bitcoin29_secp256k1_secret_key(bitcoin30_sk);
        let bitcoin30_sk_back = bitcoin29_to_bitcoin30_secp256k1_secret_key(bitcoin29_sk);

        assert_eq!(bitcoin30_sk, bitcoin30_sk_back);
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_and_back_keypair() {
        let bitcoin29_keypair =
            bitcoin::KeyPair::new(bitcoin::secp256k1::SECP256K1, &mut thread_rng());

        let bitcoin30_keypair = bitcoin29_to_bitcoin30_keypair(bitcoin29_keypair);
        let bitcoin29_keypair_back = bitcoin30_to_bitcoin29_keypair(bitcoin30_keypair);

        assert_eq!(bitcoin29_keypair, bitcoin29_keypair_back);
    }

    #[test]
    fn test_bitcoin30_to_bitcoin29_and_back_keypair() {
        let bitcoin30_keypair = bitcoin30::secp256k1::KeyPair::new(
            &bitcoin30::secp256k1::Secp256k1::new(),
            &mut thread_rng(),
        );

        let bitcoin29_keypair = bitcoin30_to_bitcoin29_keypair(bitcoin30_keypair);
        let bitcoin30_keypair_back = bitcoin29_to_bitcoin30_keypair(bitcoin29_keypair);

        assert_eq!(bitcoin30_keypair, bitcoin30_keypair_back);
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_and_back_schnorr_signature() {
        let keypair = bitcoin::KeyPair::new(bitcoin::secp256k1::SECP256K1, &mut thread_rng());
        let bitcoin29_signature =
            keypair.sign_schnorr(bitcoin::secp256k1::Message::from_hashed_data::<
                bitcoin::secp256k1::hashes::sha256::Hash,
            >(&[0x01, 0x23]));

        let bitcoin30_signature = bitcoin29_to_bitcoin30_schnorr_signature(bitcoin29_signature);
        let bitcoin29_signature_back =
            bitcoin30_to_bitcoin29_schnorr_signature(bitcoin30_signature);

        assert_eq!(bitcoin29_signature, bitcoin29_signature_back);
    }

    #[test]
    fn test_bitcoin30_to_bitcoin29_and_back_schnorr_signature() {
        let keypair = bitcoin30::secp256k1::KeyPair::new(
            &bitcoin30::secp256k1::Secp256k1::new(),
            &mut thread_rng(),
        );
        let bitcoin30_signature =
            keypair.sign_schnorr(bitcoin30::secp256k1::Message::from_hashed_data::<
                bitcoin30::secp256k1::hashes::sha256::Hash,
            >(&[0x01, 0x23]));

        let bitcoin29_signature = bitcoin30_to_bitcoin29_schnorr_signature(bitcoin30_signature);
        let bitcoin30_signature_back =
            bitcoin29_to_bitcoin30_schnorr_signature(bitcoin29_signature);

        assert_eq!(bitcoin30_signature, bitcoin30_signature_back);
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_and_back_message() {
        let bitcoin29_message = bitcoin::secp256k1::Message::from_hashed_data::<
            bitcoin::secp256k1::hashes::sha256::Hash,
        >(&[0x01, 0x23]);

        let bitcoin30_message = bitcoin29_to_bitcoin30_message(bitcoin29_message);
        let bitcoin29_message_back = bitcoin30_to_bitcoin29_message(bitcoin30_message);

        assert_eq!(bitcoin29_message, bitcoin29_message_back);
    }

    #[test]
    fn test_bitcoin30_to_bitcoin29_and_back_message() {
        let bitcoin30_message = bitcoin30::secp256k1::Message::from_hashed_data::<
            bitcoin30::secp256k1::hashes::sha256::Hash,
        >(&[0x01, 0x23]);

        let bitcoin29_message = bitcoin30_to_bitcoin29_message(bitcoin30_message);
        let bitcoin30_message_back = bitcoin29_to_bitcoin30_message(bitcoin29_message);

        assert_eq!(bitcoin30_message, bitcoin30_message_back);
    }

    #[test]
    fn test_network_conversions() {
        assert_eq!(
            bitcoin30::Network::Bitcoin,
            bitcoin29_to_bitcoin30_network(bitcoin::Network::Bitcoin)
        );

        assert_eq!(
            bitcoin::Network::Bitcoin,
            bitcoin30_to_bitcoin29_network(bitcoin30::Network::Bitcoin)
        );
    }

    #[test]
    fn test_amount_conversions() {
        let bitcoin29_amount = bitcoin::Amount::from_sat(123456789);
        let bitcoin30_amount = bitcoin30::Amount::from_sat(123456789);

        assert_eq!(
            bitcoin29_amount,
            bitcoin30_to_bitcoin29_amount(bitcoin30_amount)
        );
        assert_eq!(
            bitcoin30_amount,
            bitcoin29_to_bitcoin30_amount(bitcoin29_amount)
        );
    }

    #[test]
    fn test_outpoint_conversions() {
        let bitcoin29_txid = bitcoin::Txid::from_hex(
            "0123456789012345678901234567890123456789012345678901234567890123",
        )
        .expect("Failed to parse bitcoin v29 txid");
        let bitcoin29_outpoint = bitcoin::OutPoint {
            txid: bitcoin29_txid,
            vout: 0,
        };

        let bitcoin30_txid = bitcoin30::Txid::from_str(
            "0123456789012345678901234567890123456789012345678901234567890123",
        )
        .expect("Failed to parse bitcoin v30 txid");
        let bitcoin30_outpoint = bitcoin30::OutPoint {
            txid: bitcoin30_txid,
            vout: 0,
        };

        assert_eq!(
            bitcoin29_outpoint,
            bitcoin30_to_bitcoin29_outpoint(bitcoin30_outpoint)
        );
        assert_eq!(
            bitcoin30_outpoint,
            bitcoin29_to_bitcoin30_outpoint(bitcoin29_outpoint)
        );
    }

    #[test]
    fn test_witness_conversions() {
        let bitcoin29_witness =
            bitcoin::Witness::from_vec(vec![vec![0x01, 0x02], vec![0x03, 0x04]]);
        let bitcoin30_witness = bitcoin30::Witness::from_slice(&[&[0x01, 0x02], &[0x03, 0x04]]);

        assert_eq!(
            bitcoin29_witness,
            bitcoin30_to_bitcoin29_witness(&bitcoin30_witness)
        );
        assert_eq!(
            bitcoin30_witness,
            bitcoin29_to_bitcoin30_witness(&bitcoin29_witness)
        );
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_transaction() {
        let bitcoin29_tx = bitcoin::Transaction {
            version: 1,
            lock_time: bitcoin::PackedLockTime(123456),
            input: vec![bitcoin::TxIn {
                previous_output: bitcoin::OutPoint {
                    txid: bitcoin::Txid::from_str(
                        "0123456789012345678901234567890123456789012345678901234567890123",
                    )
                    .expect("Failed to parse bitcoin v30 txid"),
                    vout: 0,
                },
                script_sig: bitcoin::Script::from(vec![0x01, 0x02]),
                sequence: bitcoin::Sequence(0),
                witness: bitcoin::Witness::from_vec(vec![vec![0x03, 0x04]]),
            }],
            output: vec![bitcoin::TxOut {
                value: 123456789,
                script_pubkey: bitcoin::Script::from(vec![0x05, 0x06]),
            }],
        };

        let bitcoin30_tx = bitcoin29_to_bitcoin30_transaction(&bitcoin29_tx);

        assert_eq!(
            format!("{}", bitcoin29_tx.txid()),
            format!("{}", bitcoin30_tx.txid())
        );
    }

    #[test]
    fn test_bitcoin30_to_bitcoin29_transaction() {
        let bitcoin30_tx = bitcoin30::Transaction {
            version: 1,
            lock_time: bitcoin30::absolute::LockTime::Blocks(
                Height::from_consensus(123456).expect("Failed to create bitcoin v30 lock time"),
            ),
            input: vec![bitcoin30::TxIn {
                previous_output: bitcoin30::OutPoint {
                    txid: bitcoin30::Txid::from_str(
                        "0123456789012345678901234567890123456789012345678901234567890123",
                    )
                    .expect("Failed to parse bitcoin v30 txid"),
                    vout: 0,
                },
                script_sig: bitcoin30::ScriptBuf::from_bytes(vec![0x01, 0x02]),
                sequence: bitcoin30::Sequence(0),
                witness: bitcoin30::Witness::from_slice(&[&[0x03, 0x04]]),
            }],
            output: vec![bitcoin30::TxOut {
                value: 123456789,
                script_pubkey: bitcoin30::ScriptBuf::from_bytes(vec![0x05, 0x06]),
            }],
        };

        let bitcoin29_tx = bitcoin30_to_bitcoin29_transaction(&bitcoin30_tx);

        assert_eq!(
            format!("{}", bitcoin30_tx.txid()),
            format!("{}", bitcoin29_tx.txid())
        );
    }

    #[test]
    fn test_block_hash_conversions() {
        let bitcoin29_block_hash = bitcoin::BlockHash::from_hex(
            "0123456789012345678901234567890123456789012345678901234567890123",
        )
        .expect("Failed to parse bitcoin v29 block hash");
        let bitcoin30_block_hash = bitcoin30::BlockHash::from_str(
            "0123456789012345678901234567890123456789012345678901234567890123",
        )
        .expect("Failed to parse bitcoin v30 block hash");

        assert_eq!(
            bitcoin29_block_hash,
            bitcoin30_to_bitcoin29_block_hash(bitcoin30_block_hash)
        );
        assert_eq!(
            bitcoin30_block_hash,
            bitcoin29_to_bitcoin30_block_hash(bitcoin29_block_hash)
        );
    }

    #[test]
    fn test_txid_conversions() {
        let bitcoin29_txid = bitcoin::Txid::from_hex(
            "0123456789012345678901234567890123456789012345678901234567890123",
        )
        .expect("Failed to parse bitcoin v29 txid");
        let bitcoin30_txid = bitcoin30::Txid::from_str(
            "0123456789012345678901234567890123456789012345678901234567890123",
        )
        .expect("Failed to parse bitcoin v30 txid");

        assert_eq!(bitcoin29_txid, bitcoin30_to_bitcoin29_txid(bitcoin30_txid));
        assert_eq!(bitcoin30_txid, bitcoin29_to_bitcoin30_txid(bitcoin29_txid));
    }

    #[test]
    fn test_script_conversions() {
        let bitcoin29_public_key = bitcoin::PublicKey::new(
            bitcoin::secp256k1::Secp256k1::new()
                .generate_keypair(&mut thread_rng())
                .1,
        );
        let bitcoin29_script: bitcoin::Script = bitcoin::Script::new_p2pk(&bitcoin29_public_key);

        let bitcoin30_public_key = bitcoin29_to_bitcoin30_public_key(bitcoin29_public_key);
        let bitcoin30_script: bitcoin30::ScriptBuf =
            bitcoin30::ScriptBuf::new_p2pk(&bitcoin30_public_key);

        // Assert that bitcoin30->bitcoin29 script is the same as native bitcoin29
        // script.
        assert_eq!(
            bitcoin29_script,
            bitcoin30_to_bitcoin29_script(&bitcoin30_script)
        );
        // Assert that bitcoin29->bitcoin30 script is the same as native bitcoin30
        // script.
        assert_eq!(
            bitcoin30_script,
            bitcoin29_to_bitcoin30_script(bitcoin29_script)
        );
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_address() {
        let bitcoin30_public_key = bitcoin30::PublicKey::new(
            bitcoin30::secp256k1::Secp256k1::new()
                .generate_keypair(&mut thread_rng())
                .1,
        );
        let bitcoin30_address: bitcoin30::Address =
            bitcoin30::Address::p2pkh(&bitcoin30_public_key, bitcoin30::Network::Bitcoin);

        let bitcoin29_public_key = bitcoin30_to_bitcoin29_public_key(bitcoin30_public_key);
        let bitcoin29_address: bitcoin::Address =
            bitcoin::Address::p2pkh(&bitcoin29_public_key, bitcoin::Network::Bitcoin);

        // Assert that bitcoin29->bitcoin30 address is the same as native bitcoin30
        // address.
        assert_eq!(
            bitcoin30_address,
            bitcoin29_to_bitcoin30_address(bitcoin29_address).assume_checked()
        );
    }

    #[test]
    fn test_bitcoin30_to_bitcoin29_address() {
        let bitcoin29_public_key = bitcoin::PublicKey::new(
            bitcoin::secp256k1::Secp256k1::new()
                .generate_keypair(&mut thread_rng())
                .1,
        );
        let bitcoin29_address: bitcoin::Address =
            bitcoin::Address::p2pkh(&bitcoin29_public_key, bitcoin::Network::Bitcoin);

        let bitcoin30_public_key = bitcoin29_to_bitcoin30_public_key(bitcoin29_public_key);
        let bitcoin30_address: bitcoin30::Address =
            bitcoin30::Address::p2pkh(&bitcoin30_public_key, bitcoin30::Network::Bitcoin);

        // Assert that bitcoin30->bitcoin29 address is the same as native bitcoin29
        // address.
        assert_eq!(
            bitcoin29_address,
            bitcoin30_to_bitcoin29_address(bitcoin30_address)
        );
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_ripemd160_hash() {
        let bitcoin29_hash =
            bitcoin::hashes::ripemd160::Hash::from_hex("0123456789012345678901234567890123456789")
                .expect("Failed to parse bitcoin v29 ripemd160 hash");
        let bitcoin30_hash = bitcoin30::hashes::ripemd160::Hash::from_str(
            "0123456789012345678901234567890123456789",
        )
        .expect("Failed to parse bitcoin v30 ripemd160 hash");

        // Assert that bitcoin29->bitcoin30 ripemd160 hash is the same as native
        // bitcoin30 ripemd160 hash.
        assert_eq!(
            bitcoin30_hash,
            bitcoin29_to_bitcoin30_ripemd160_hash(bitcoin29_hash)
        );
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_hash160_hash() {
        let bitcoin29_hash =
            bitcoin::hashes::hash160::Hash::from_hex("0123456789012345678901234567890123456789")
                .expect("Failed to parse bitcoin v29 hash160 hash");
        let bitcoin30_hash =
            bitcoin30::hashes::hash160::Hash::from_str("0123456789012345678901234567890123456789")
                .expect("Failed to parse bitcoin v30 hash160 hash");

        // Assert that bitcoin29->bitcoin30 hash160 hash is the same as native bitcoin30
        // hash160 hash.
        assert_eq!(
            bitcoin30_hash,
            bitcoin29_to_bitcoin30_hash160_hash(bitcoin29_hash)
        );
    }

    #[test]
    fn test_bitcoin29_to_bitcoin30_sha256_hash() {
        let bitcoin29_hash = bitcoin::hashes::sha256::Hash::from_hex(
            "0123456789012345678901234567890123456789012345678901234567890123",
        )
        .expect("Failed to parse bitcoin v29 sha256 hash");
        let bitcoin30_hash = bitcoin30::hashes::sha256::Hash::from_str(
            "0123456789012345678901234567890123456789012345678901234567890123",
        )
        .expect("Failed to parse bitcoin v30 sha256 hash");

        // Assert that bitcoin29->bitcoin30 sha256 hash is the same as native bitcoin30
        // sha256 hash.
        assert_eq!(
            bitcoin30_hash,
            bitcoin29_to_bitcoin30_sha256_hash(bitcoin29_hash)
        );
    }
}
