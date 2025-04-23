use crate::{
    database::LocalStore,
    mesh::Network,
    models::{SyncBlock, SyncRecord, SyncRecordType},
};
use anyhow::{Result, anyhow};
use blake3::Hash;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::time::{self, Duration};
use uuid::Uuid;

pub struct Service {
    db: LocalStore,
    network: Network,
    node_id: String,
    last_sync: chrono::DateTime<chrono::Utc>,
    pending_records: Vec<SyncRecord>,
    vector_clock: HashMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VectorClock {
    node_id: String,
    timestamp: u64,
}

impl Service {
    pub fn new(db: LocalStore, network: Network) -> Result<Self> {
        Ok(Self {
            db,
            network,
            node_id: Uuid::new_v4().to_string(),
            last_sync: Utc::now(),
            pending_records: Vec::new(),
            vector_clock: HashMap::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut interval = time::interval(Duration::from_secs(60));
        
        loop {
            interval.tick().await;
            
            // Create a new sync block if we have pending records
            if !self.pending_records.is_empty() {
                let block = self.create_sync_block().await?;
                self.network.broadcast_block(block).await?;
            }
            
            // Request updates from peers
            self.request_updates().await?;
            
            // Clean up old records
            self.cleanup_old_records().await?;
        }
    }

    pub async fn add_record(&mut self, record_type: SyncRecordType, data: Vec<u8>) -> Result<()> {
        let record = SyncRecord {
            record_type,
            data,
            hash: self.calculate_record_hash(&data)?,
        };
        
        self.pending_records.push(record);
        
        // If we have enough records, create and broadcast a new block
        if self.pending_records.len() >= 10 {
            let block = self.create_sync_block().await?;
            self.network.broadcast_block(block).await?;
        }
        
        Ok(())
    }

    async fn create_sync_block(&mut self) -> Result<SyncBlock> {
        let previous_block = self.db.get_latest_sync_block().await?;
        let previous_hash = previous_block
            .map(|b| b.id.to_string())
            .unwrap_or_else(|| "genesis".to_string());

        // Update vector clock
        let current_time = self.vector_clock
            .entry(self.node_id.clone())
            .or_insert(0);
        *current_time += 1;

        let block = SyncBlock {
            id: Uuid::new_v4(),
            previous_hash,
            timestamp: Utc::now(),
            node_id: self.node_id.clone(),
            records: std::mem::take(&mut self.pending_records),
            signature: self.sign_block()?,
        };

        // Save the block locally
        self.db.save_sync_block(&block).await?;

        Ok(block)
    }

    async fn process_block(&mut self, block: SyncBlock) -> Result<()> {
        // Verify block signature
        if !self.verify_block_signature(&block)? {
            return Err(anyhow!("Invalid block signature"));
        }

        // Check if we already have this block
        if let Some(_) = self.db.get_sync_block(&block.id).await? {
            return Ok(());
        }

        // Process each record in the block
        for record in &block.records {
            match record.record_type {
                SyncRecordType::RecipientRegistration => {
                    let recipient = serde_json::from_slice(&record.data)?;
                    self.db.save_recipient(&recipient).await?;
                }
                SyncRecordType::AidDistribution => {
                    let distribution = serde_json::from_slice(&record.data)?;
                    self.db.save_distribution(&distribution).await?;
                }
                SyncRecordType::BiometricTemplate => {
                    let template = serde_json::from_slice(&record.data)?;
                    self.db.save_biometric_template(&template).await?;
                }
                SyncRecordType::VerificationProof => {
                    // Process verification proofs
                    // TODO: Implement verification proof processing
                }
            }
        }

        // Save the block
        self.db.save_sync_block(&block).await?;

        // Update vector clock
        let node_time = self.vector_clock
            .entry(block.node_id.clone())
            .or_insert(0);
        *node_time = std::cmp::max(*node_time + 1, block.timestamp.timestamp() as u64);

        Ok(())
    }

    fn calculate_record_hash(&self, data: &[u8]) -> Result<String> {
        Ok(blake3::hash(data).to_string())
    }

    fn sign_block(&self) -> Result<String> {
        // TODO: Implement actual block signing using ed25519 or similar
        Ok("signature".to_string())
    }

    fn verify_block_signature(&self, block: &SyncBlock) -> Result<bool> {
        // TODO: Implement actual signature verification
        Ok(true)
    }

    async fn request_updates(&mut self) -> Result<()> {
        // Request updates from all peers since our last sync
        for peer in self.network.peers.iter() {
            self.network.request_blocks(*peer, self.last_sync).await?;
        }
        self.last_sync = Utc::now();
        Ok(())
    }

    async fn cleanup_old_records(&mut self) -> Result<()> {
        // Remove records older than 30 days
        let cutoff = Utc::now() - chrono::Duration::days(30);
        
        // TODO: Implement cleanup logic while maintaining data integrity
        
        Ok(())
    }

    fn resolve_conflicts(&self, local: &SyncRecord, remote: &SyncRecord) -> Result<SyncRecord> {
        // Implement conflict resolution strategy
        // For now, take the record with the higher hash value
        if local.hash > remote.hash {
            Ok(local.clone())
        } else {
            Ok(remote.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_sync_service() {
        let temp_dir = tempdir().unwrap();
        let db = LocalStore::new(temp_dir.path()).unwrap();
        let network = Network::new().unwrap();
        
        let mut service = Service::new(db, network).unwrap();
        
        // Add a test record
        let data = vec![1, 2, 3, 4];
        service.add_record(SyncRecordType::RecipientRegistration, data).await.unwrap();
        
        assert!(!service.pending_records.is_empty());
    }

    #[tokio::test]
    async fn test_block_creation() {
        let temp_dir = tempdir().unwrap();
        let db = LocalStore::new(temp_dir.path()).unwrap();
        let network = Network::new().unwrap();
        
        let mut service = Service::new(db, network).unwrap();
        
        // Add some test records
        for i in 0..5 {
            let data = vec![i as u8];
            service.add_record(SyncRecordType::RecipientRegistration, data).await.unwrap();
        }
        
        let block = service.create_sync_block().await.unwrap();
        assert_eq!(block.records.len(), 5);
        assert!(service.pending_records.is_empty());
    }
} 