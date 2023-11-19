

use concordium_std::*;

#[derive(Serialize, Deserialize)]
struct Transfer {
    to: AccountAddress,
    value: u64,
}

#[derive(Serialize, Deserialize)]
struct Transaction {
    transfers: Vec<Transfer>,
    signatures: Vec<AccountAddress>,
}

#[derive(Serialize, Deserialize)]
struct MultiSigCCD {
    transactions: Vec<Transaction>,
    owners: Vec<AccountAddress>,
    num_confirmations_required: u8,
}

impl MultiSigCCD {
    pub fn new(owners: Vec<AccountAddress>, num_confirmations_required: u8) -> Self {
        Self {
            transactions: vec![],
            owners,
            num_confirmations_required,
        }
    }

    pub fn submit_transaction(&mut self, transfers: Vec<Transfer>) {
        let transaction = Transaction {
            transfers,
            signatures: vec![],
        };
        self.transactions.push(transaction);
    }

    pub fn sign_transaction(&mut self, tx_index: usize, signer: AccountAddress) {
        let transaction = &mut self.transactions[tx_index];
        if !self.owners.contains(&signer) {
            panic!("Signer is not an owner");
        }
        if transaction.signatures.contains(&signer) {
            panic!("Signer already signed this transaction");
        }
        transaction.signatures.push(signer);
    }

    pub fn execute_transaction(&mut self, tx_index: usize) {
        let transaction = &mut self.transactions[tx_index];
        if transaction.signatures.len() < self.num_confirmations_required as usize {
            panic!("Not enough signatures to execute transaction");
        }
        for transfer in &transaction.transfers {
            let _ = concordium_std::transfer_ccd(transfer.to, transfer.value);
        }
        transaction.signatures.clear();
    }
}
