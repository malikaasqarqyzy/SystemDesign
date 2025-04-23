use crate::models::{Recipient, AidDistribution, BiometricTemplate, SyncBlock};
use anyhow::{Result, anyhow};
use serde::{de::DeserializeOwned, Serialize};
use sled::{Db, Tree};
use std::path::Path;
use uuid::Uuid;

#[derive(Clone)]
pub struct LocalStore {
    db: Db,
    recipients: Tree,
    distributions: Tree,
    biometrics: Tree,
    sync_blocks: Tree,
}

impl LocalStore {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path)?;
        
        Ok(Self {
            recipients: db.open_tree("recipients")?,
            distributions: db.open_tree("distributions")?,
            biometrics: db.open_tree("biometrics")?,
            sync_blocks: db.open_tree("sync_blocks")?,
            db,
        })
    }

    // Recipient operations
    pub async fn save_recipient(&self, recipient: &Recipient) -> Result<()> {
        let key = recipient.id.as_bytes();
        let value = serde_json::to_vec(recipient)?;
        self.recipients.insert(key, value)?;
        Ok(())
    }

    pub async fn get_recipient(&self, id: &Uuid) -> Result<Option<Recipient>> {
        let key = id.as_bytes();
        match self.recipients.get(key)? {
            Some(value) => Ok(Some(serde_json::from_slice(&value)?)),
            None => Ok(None),
        }
    }

    pub async fn find_recipients_by_biometric_hash(&self, hash: &str) -> Result<Vec<Recipient>> {
        let mut matches = Vec::new();
        
        for result in self.recipients.iter() {
            let (_, value) = result?;
            let recipient: Recipient = serde_json::from_slice(&value)?;
            if recipient.biometric_hash == hash {
                matches.push(recipient);
            }
        }
        
        Ok(matches)
    }

    // Aid distribution operations
    pub async fn save_distribution(&self, distribution: &AidDistribution) -> Result<()> {
        let key = distribution.id.as_bytes();
        let value = serde_json::to_vec(distribution)?;
        self.distributions.insert(key, value)?;
        Ok(())
    }

    pub async fn get_distributions_for_recipient(&self, recipient_id: &Uuid) -> Result<Vec<AidDistribution>> {
        let mut distributions = Vec::new();
        
        for result in self.distributions.iter() {
            let (_, value) = result?;
            let distribution: AidDistribution = serde_json::from_slice(&value)?;
            if distribution.recipient_id == *recipient_id {
                distributions.push(distribution);
            }
        }
        
        Ok(distributions)
    }

    // Biometric template operations
    pub async fn save_biometric_template(&self, template: &BiometricTemplate) -> Result<()> {
        let key = template.id.as_bytes();
        let value = serde_json::to_vec(template)?;
        self.biometrics.insert(key, value)?;
        Ok(())
    }

    pub async fn get_biometric_template(&self, id: &Uuid) -> Result<Option<BiometricTemplate>> {
        let key = id.as_bytes();
        match self.biometrics.get(key)? {
            Some(value) => Ok(Some(serde_json::from_slice(&value)?)),
            None => Ok(None),
        }
    }

    // Sync operations
    pub async fn save_sync_block(&self, block: &SyncBlock) -> Result<()> {
        let key = block.id.as_bytes();
        let value = serde_json::to_vec(block)?;
        self.sync_blocks.insert(key, value)?;
        Ok(())
    }

    pub async fn get_latest_sync_block(&self) -> Result<Option<SyncBlock>> {
        let mut latest: Option<SyncBlock> = None;
        
        for result in self.sync_blocks.iter() {
            let (_, value) = result?;
            let block: SyncBlock = serde_json::from_slice(&value)?;
            
            if let Some(current_latest) = &latest {
                if block.timestamp > current_latest.timestamp {
                    latest = Some(block);
                }
            } else {
                latest = Some(block);
            }
        }
        
        Ok(latest)
    }

    // Generic operations
    async fn save<T: Serialize>(&self, tree: &Tree, key: &[u8], value: &T) -> Result<()> {
        let serialized = serde_json::to_vec(value)?;
        tree.insert(key, serialized)?;
        Ok(())
    }

    async fn get<T: DeserializeOwned>(&self, tree: &Tree, key: &[u8]) -> Result<Option<T>> {
        match tree.get(key)? {
            Some(value) => Ok(Some(serde_json::from_slice(&value)?)),
            None => Ok(None),
        }
    }

    pub async fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Location, RecipientMetadata, AidType, Vulnerability};
    use chrono::Utc;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_recipient_operations() {
        let temp_dir = tempdir().unwrap();
        let store = LocalStore::new(temp_dir.path()).unwrap();

        let recipient = Recipient {
            id: Uuid::new_v4(),
            did: "did:example:123".to_string(),
            biometric_hash: "hash123".to_string(),
            metadata: RecipientMetadata {
                family_size: 4,
                location: Location {
                    latitude: 0.0,
                    longitude: 0.0,
                    region: "Test Region".to_string(),
                    camp_id: None,
                },
                needs: vec![AidType::Food],
                vulnerabilities: vec![Vulnerability::Elderly],
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        store.save_recipient(&recipient).await.unwrap();
        let retrieved = store.get_recipient(&recipient.id).await.unwrap().unwrap();
        assert_eq!(recipient.id, retrieved.id);
    }

    #[tokio::test]
    async fn test_distribution_operations() {
        let temp_dir = tempdir().unwrap();
        let store = LocalStore::new(temp_dir.path()).unwrap();

        let distribution = AidDistribution {
            id: Uuid::new_v4(),
            recipient_id: Uuid::new_v4(),
            aid_type: AidType::Food,
            quantity: 10.0,
            unit: "kg".to_string(),
            distributed_at: Utc::now(),
            distributor_id: "test_distributor".to_string(),
            location: Location {
                latitude: 0.0,
                longitude: 0.0,
                region: "Test Region".to_string(),
                camp_id: None,
            },
            verification: vec![],
        };

        store.save_distribution(&distribution).await.unwrap();
        let distributions = store.get_distributions_for_recipient(&distribution.recipient_id).await.unwrap();
        assert_eq!(distributions.len(), 1);
        assert_eq!(distributions[0].id, distribution.id);
    }
} 