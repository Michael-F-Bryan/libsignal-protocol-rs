use libsignal_protocol_sys as sys;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InternalError {
    NoMemory,
    InvalidArgument,
    Unknown,
    DuplicateMessage,
    InvalidKey,
    InvalidKeyId,
    InvalidMAC,
    InvalidMessage,
    InvalidVersion,
    LegacyMessage,
    NoSession,
    StaleKeyExchange,
    UntrustedIdentity,
    VerifySignatureVerificationFailed,
    InvalidProtoBuf,
    FPVersionMismatch,
    FPIdentMismatch,
    Other(i32),
}

impl InternalError {
    pub fn from_error_code(code: i32) -> Option<InternalError> {
        match code {
            sys::SG_ERR_NOMEM => Some(InternalError::NoMemory),
            sys::SG_ERR_INVAL => Some(InternalError::InvalidArgument),
            sys::SG_ERR_UNKNOWN => Some(InternalError::Unknown),
            sys::SG_ERR_DUPLICATE_MESSAGE => Some(InternalError::DuplicateMessage),
            sys::SG_ERR_INVALID_KEY => Some(InternalError::InvalidKey),
            sys::SG_ERR_INVALID_KEY_ID => Some(InternalError::InvalidKeyId),
            sys::SG_ERR_INVALID_MAC => Some(InternalError::InvalidMAC),
            sys::SG_ERR_INVALID_MESSAGE => Some(InternalError::InvalidMessage),
            sys::SG_ERR_INVALID_VERSION => Some(InternalError::InvalidVersion),
            sys::SG_ERR_LEGACY_MESSAGE => Some(InternalError::LegacyMessage),
            sys::SG_ERR_NO_SESSION => Some(InternalError::NoSession),
            sys::SG_ERR_STALE_KEY_EXCHANGE => Some(InternalError::StaleKeyExchange),
            sys::SG_ERR_UNTRUSTED_IDENTITY => Some(InternalError::UntrustedIdentity),
            sys::SG_ERR_VRF_SIG_VERIF_FAILED => {
                Some(InternalError::VerifySignatureVerificationFailed)
            }
            sys::SG_ERR_INVALID_PROTO_BUF => Some(InternalError::InvalidProtoBuf),
            sys::SG_ERR_FP_VERSION_MISMATCH => Some(InternalError::FPVersionMismatch),
            sys::SG_ERR_FP_IDENT_MISMATCH => Some(InternalError::FPIdentMismatch),
            _ => None,
        }
    }

    pub fn to_result(code: i32) -> Result<(), InternalError> {
        if code == 0 {
            return Ok(());
        }

        match InternalError::from_error_code(code) {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }
}

pub(crate) trait InternalErrorCode: Sized {
    fn to_result(self) -> Result<(), InternalError>;
}

impl InternalErrorCode for i32 {
    fn to_result(self) -> Result<(), InternalError> {
        if self == 0 {
            return Ok(());
        }

        match InternalError::from_error_code(self) {
            None => Err(InternalError::Other(self)),
            Some(e) => Err(e),
        }
    }
}

impl Display for InternalError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InternalError::NoMemory => write!(f, "No Memory"),
            InternalError::InvalidArgument => write!(f, "Invalid argument"),
            InternalError::Unknown => write!(f, "Unknown error"),
            InternalError::DuplicateMessage => write!(f, "Duplicate message"),
            InternalError::InvalidKey => write!(f, "Invalid key"),
            InternalError::InvalidKeyId => write!(f, "Invalid key ID"),
            InternalError::InvalidMAC => write!(f, "Invalid MAC"),
            InternalError::InvalidMessage => write!(f, "Invalid message"),
            InternalError::InvalidVersion => write!(f, "Invalid version"),
            InternalError::LegacyMessage => write!(f, "Legacy message"),
            InternalError::NoSession => write!(f, "No session"),
            InternalError::StaleKeyExchange => write!(f, "Stale key exchange"),
            InternalError::UntrustedIdentity => write!(f, "Untrusted identity"),
            InternalError::VerifySignatureVerificationFailed => {
                write!(f, "Verifying signature failed")
            }
            InternalError::InvalidProtoBuf => write!(f, "Invalid protobuf"),
            InternalError::FPVersionMismatch => write!(f, "FP version mismatched"),
            InternalError::FPIdentMismatch => write!(f, "FP ident mismatched"),
            InternalError::Other(code) => write!(f, "Unknown error {}", code),
        }
    }
}
