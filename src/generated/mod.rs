/* Copyright (c) Fortanix, Inc.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::api_model::*;
use crate::client::*;
use crate::operations::*;
use serde::{Deserialize, Serialize};
use simple_hyper_client::Method;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::net::IpAddr;
use strum::EnumIter;
use uuid::Uuid;

mod accounts_generated;
mod approval_requests_generated;
mod apps_generated;
mod common_generated;
mod crypto_generated;
mod external_roles_generated;
mod groups_generated;
mod keys_generated;
mod plugins_generated;
mod session_generated;
mod users_generated;
mod version_generated;

pub use self::accounts_generated::*;
pub use self::approval_requests_generated::*;
pub use self::apps_generated::*;
pub use self::common_generated::*;
pub use self::crypto_generated::*;
pub use self::external_roles_generated::*;
pub use self::groups_generated::*;
pub use self::keys_generated::*;
pub use self::plugins_generated::*;
pub use self::session_generated::*;
pub use self::users_generated::*;
pub use self::version_generated::*;

// Convenience methods

impl CryptMode {
    pub fn rsa_oaep(hash: DigestAlgorithm) -> Self {
        CryptMode::Rsa(RsaEncryptionPadding::Oaep {
            mgf: Mgf::Mgf1 { hash },
        })
    }
}

impl SignatureMode {
    pub fn rsa_pss(hash: DigestAlgorithm) -> Self {
        SignatureMode::Rsa(RsaSignaturePadding::Pss {
            mgf: Mgf::Mgf1 { hash },
        })
    }
}

// Fixes

impl Default for SobjectEncoding {
    fn default() -> Self {
        SobjectEncoding::Json
    }
}

impl Default for RsaOptions {
    fn default() -> Self {
        RsaOptions {
            key_size: None,
            public_exponent: None,
            encryption_policy: Vec::new(),
            signature_policy: Vec::new(),
            minimum_key_length: None,
        }
    }
}

impl ToString for AppRole {
    fn to_string(&self) -> String {
        match *self {
            AppRole::Admin => "admin".to_string(),
            AppRole::Crypto => "app".to_string(),
        }
    }
}

impl ToString for ObjectType {
    fn to_string(&self) -> String {
        match *self {
            ObjectType::Aes => "AES".to_string(),
            ObjectType::Des => "DES".to_string(),
            ObjectType::Des3 => "DES3".to_string(),
            ObjectType::Rsa => "RSA".to_string(),
            ObjectType::Dsa => "DSA".to_string(),
            ObjectType::Ec => "EC".to_string(),
            ObjectType::Opaque => "OPAQUE".to_string(),
            ObjectType::Hmac => "HMAC".to_string(),
            ObjectType::Secret => "SECRET".to_string(),
            ObjectType::Seed => "SEED".to_string(),
            ObjectType::Round5Beta => "ROUND5BETA".to_string(),
            ObjectType::LedaBeta => "LEDABETA".to_string(),
            ObjectType::Lms => "LMS".to_string(),
            ObjectType::Certificate => "CERTIFICATE".to_string(),
            ObjectType::Pbe => "PBE".to_string(),
        }
    }
}

impl fmt::Display for SobjectEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SobjectEncoding::Json => write!(f, "json"),
            SobjectEncoding::Value => write!(f, "value"),
        }
    }
}

impl fmt::Display for ApprovalStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApprovalStatus::Pending => write!(f, "PENDING"),
            ApprovalStatus::Approved => write!(f, "APPROVED"),
            ApprovalStatus::Denied => write!(f, "DENIED"),
            ApprovalStatus::Failed => write!(f, "FAILED"),
        }
    }
}

impl Default for AppSort {
    fn default() -> Self {
        AppSort::ByAppId {
            order: Order::Ascending,
            start: None,
        }
    }
}

impl Default for SobjectSort {
    fn default() -> Self {
        SobjectSort::ByKid {
            order: Order::Ascending,
            start: None,
        }
    }
}

impl Default for PluginSort {
    fn default() -> Self {
        PluginSort::ByPluginId {
            order: Order::Ascending,
            start: None,
        }
    }
}

impl Default for UserSort {
    fn default() -> Self {
        UserSort::ByUserId {
            order: Order::Ascending,
            start: None,
        }
    }
}

impl Default for AppRole {
    fn default() -> Self {
        AppRole::Crypto
    }
}

impl Default for SubscriptionType {
    fn default() -> SubscriptionType {
        SubscriptionType::Trial { expires_at: None }
    }
}
