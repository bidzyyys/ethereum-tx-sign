use ethereum_types::{H160, U256};
use rlp::RlpStream;
use tiny_keccak::keccak256;

/// Description of a Transaction, pending or in the chain.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct RawTransaction {
    /// Nonce
    pub nonce: U256,
    /// Recipient (None when contract creation)
    pub to: Option<H160>,
    /// Transfered value
    pub value: U256,
    /// Gas Price
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    /// Gas amount
    pub gas: U256,
    /// Input data
    pub data: Vec<u8>,
}

impl RawTransaction {
    
    pub fn encode_signed_tx(&self, signature: Vec<u8>, chain_id: u64) -> Vec<u8> {
        let (sig_v, sig_r, sig_s) = self.prepare_signature(signature, chain_id);
        
        let mut tx = RlpStream::new();
        
        tx.begin_unbounded_list();
        
        self.encode(&mut tx);
        tx.append(&sig_v);
        tx.append(&sig_r);
        tx.append(&sig_s);
        
        tx.finalize_unbounded_list();
        
        tx.out()
    }

    pub fn hash(&self, chain_id: u64) -> Vec<u8> {
        let mut hash = RlpStream::new();
        
        hash.begin_unbounded_list();
        
        self.encode(&mut hash);
        hash.append(&mut chain_id.clone());
        hash.append(&mut U256::zero());
        hash.append(&mut U256::zero());
        
        hash.finalize_unbounded_list();
        
        keccak256(&hash.out()).iter().cloned().collect()
    }
    
    fn encode(&self, s: &mut RlpStream) {
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        if let Some(ref t) = self.to {
            s.append(t);
        } else {
            s.append(&vec![]);
        }
        s.append(&self.value);
        s.append(&self.data);
    }

    fn prepare_signature(&self, signature: Vec<u8>, chain_id: u64) -> (u64, Vec<u8>, Vec<u8>) {
        // TODO ugly solution
        assert_eq!(signature.len(), 65);
        
        let sig_v = signature[0];
        let sig_v = sig_v as u64 + chain_id * 2 + 35;
        
        let mut sig_r = signature.to_owned().split_off(1);
        let mut sig_s = sig_r.split_off(32);
        
        self.prepare_signature_part(&mut sig_r);
        self.prepare_signature_part(&mut sig_s);

        (sig_v, sig_r, sig_s)
    }

    fn prepare_signature_part(&self, part: &mut Vec<u8>) {
        assert_eq!(part.len(), 32);
        while part[0] == 0 {			
            part.remove(0);			
        }
    }
}

mod test {

}
