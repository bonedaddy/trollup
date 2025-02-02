use ethers_core::types::U256;
use poseidon_rs::*;
use serde::{Deserialize, Serialize};
use trollup_types::{FromBabyJubjubPoint, PublicKey, ToBabyJubjubPoint, ToFr, ToU256};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tx {
    pub sender: U256,
    pub to: U256,
    pub nonce: U256,
    pub value: U256,
}

pub fn hash_tx(tx: &Tx) -> U256 {
    let sender_pk = PublicKey::from_babyjubjub_point(&tx.sender.to_babyjubjub_point());
    let to_pk = PublicKey::from_babyjubjub_point(&tx.to.to_babyjubjub_point());
    Poseidon::new()
        .hash(
            [
                sender_pk.to_fr(),
                to_pk.to_fr(),
                tx.nonce.to_fr(),
                tx.value.to_fr(),
            ]
            .to_vec(),
        )
        .unwrap()
        .to_u256()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedTx {
    pub tx: Tx,
    pub signature: String,
}

#[tarpc::service]
pub trait TrollupRPC {
    async fn submit_transaction(tx: SignedTx) -> Result<(), String>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash() {
        let tx = Tx {
            sender: U256::from_dec_str(
                "11693830015789570214896451416834991706586932551962432904221523856506008194081",
            )
            .unwrap(),
            to: U256::from_dec_str(
                "11693830015789570214896451416834991706586932551962432904221523856506008194081",
            )
            .unwrap(),
            nonce: 0.into(),
            value: 0.into(),
        };
        assert_eq!(
            hash_tx(&tx),
            U256::from_dec_str(
                "7024706519851959073508005968462078426943949097906904873031507538622023544211"
            )
            .unwrap()
        );
    }
}
