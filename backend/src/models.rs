use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipient {
    pub id: Uuid,
    pub did: String,  // Decentralized Identifier
    pub biometric_hash: String,
    pub metadata: RecipientMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientMetadata {
    pub family_size: u32,
    pub location: Location,
    pub needs: Vec<AidType>,
    pub vulnerabilities: Vec<Vulnerability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub region: String,
    pub camp_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AidType {
    Food,
    Water,
    Medicine,
    Shelter,
    Clothing,
    Cash,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Vulnerability {
    Elderly,
    Child,
    Pregnant,
    Disabled,
    ChronicIllness,
    SingleParent,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AidDistribution {
    pub id: Uuid,
    pub recipient_id: Uuid,
    pub aid_type: AidType,
    pub quantity: f64,
    pub unit: String,
    pub distributed_at: DateTime<Utc>,
    pub distributor_id: String,
    pub location: Location,
    pub verification: Vec<Verification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verification {
    pub verifier_id: String,
    pub signature: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricTemplate {
    pub id: Uuid,
    pub template_type: BiometricType,
    pub template_data: Vec<u8>,  // Encrypted template
    pub metadata: BiometricMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiometricType {
    Fingerprint,
    Iris,
    FacialFeatures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricMetadata {
    pub quality_score: f32,
    pub capture_device: String,
    pub capture_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBlock {
    pub id: Uuid,
    pub previous_hash: String,
    pub timestamp: DateTime<Utc>,
    pub node_id: String,
    pub records: Vec<SyncRecord>,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRecord {
    pub record_type: SyncRecordType,
    pub data: Vec<u8>,  // Encrypted payload
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncRecordType {
    RecipientRegistration,
    AidDistribution,
    BiometricTemplate,
    VerificationProof,
} 