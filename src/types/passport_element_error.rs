use serde::{Deserialize, Serialize};

/// This object represents an error in the Telegram Passport element which was
/// submitted that should be resolved by the user.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerror).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementError {
    /// Error message.
    message: String,

    #[serde(flatten)]
    kind: PassportElementErrorKind,
}

// TODO: use different types?
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum PassportElementErrorKind {
    #[serde(rename = "data")]
    DataField(PassportElementErrorDataField),

    #[serde(rename = "snake_case")]
    FrontSide(PassportElementErrorFrontSide),

    #[serde(rename = "snake_case")]
    ReverseSide(PassportElementErrorReverseSide),

    #[serde(rename = "snake_case")]
    Selfie(PassportElementErrorSelfie),

    #[serde(rename = "snake_case")]
    File(PassportElementErrorFile),

    #[serde(rename = "snake_case")]
    Files(PassportElementErrorFiles),

    #[serde(rename = "snake_case")]
    TranslationFile(PassportElementErrorTranslationFile),

    #[serde(rename = "snake_case")]
    TranslationFiles(PassportElementErrorTranslationFiles),

    #[serde(rename = "snake_case")]
    Unspecified(PassportElementErrorUnspecified),
}

/// Represents an issue in one of the data fields that was provided by the
/// user. The error is considered resolved when the field's value changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrordatafield).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorDataField {
    /// The section of the user's Telegram Passport which has the error.
    pub r#type: PassportElementErrorDataFieldType,

    /// Name of the data field which has the error.
    pub field_name: String,

    /// Base64-encoded data hash.
    pub data_hash: String,
}

/// Represents an issue with the front side of a document. The error is
/// considered resolved when the file with the front side of the document
/// changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfrontside).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFrontSide {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorFrontSideType,

    /// Base64-encoded hash of the file with the front side of the
    /// document.
    pub file_hash: String,
}

/// Represents an issue with the reverse side of a document. The error is
/// considered resolved when the file with reverse side of the document
/// changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorreverseside).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorReverseSide {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorReverseSideType,

    //// Base64-encoded hash of the file with the reverse side of the
    //// document.
    pub file_hash: String,
}

//// Represents an issue with the selfie with a document. The error is
//// considered resolved when the file with the selfie changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorselfie).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorSelfie {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorSelfieType,

    /// Base64-encoded hash of the file with the selfie.
    pub file_hash: String,
}

/// Represents an issue with a document scan. The error is considered
/// resolved when the file with the document scan changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfile).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFile {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorFileType,

    /// Base64-encoded file hash.
    pub file_hash: String,
}

/// Represents an issue with a list of scans. The error is considered
/// resolved when the list of files containing the scans changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfiles).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFiles {
    /// The section of the user's Telegram Passport which has the issue.
    pub r#type: PassportElementErrorFilesType,

    /// List of base64-encoded file hashes.
    pub file_hashes: Vec<String>,
}

/// Represents an issue with one of the files that constitute the
/// translation of a document. The error is considered resolved when the
/// file changes.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrortranslationfile).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFile {
    /// Type of element of the user's Telegram Passport which has the
    /// issue.
    pub r#type: PassportElementErrorTranslationFileType,

    /// Base64-encoded file hash.
    pub file_hash: String,
}

/// Represents an issue with the translated version of a document. The
/// error is considered resolved when a file with the document translation
/// change.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrortranslationfiles).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    /// Type of element of the user's Telegram Passport which has the issue
    pub r#type: PassportElementErrorTranslationFilesType,

    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
}

/// Represents an issue in an unspecified place. The error is considered
/// resolved when new data is added.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerrorunspecified).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorUnspecified {
    /// Type of element of the user's Telegram Passport which has the
    /// issue.
    pub r#type: PassportElementErrorUnspecifiedType,

    /// Base64-encoded element hash.
    pub element_hash: String,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorDataFieldType {
    PersonalDetails,
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    Address,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFrontSideType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorReverseSideType {
    DriverLicense,
    IdentityCard,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorSelfieType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFileType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFilesType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorTranslationFileType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorTranslationFilesType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorUnspecifiedType {
    DataField,
    FrontSide,
    ReverseSide,
    Selfie,
    File,
    Files,
    TranslationFile,
    TranslationFiles,
    Unspecified,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_data_field() {
        let data = PassportElementError {
            message: "This is an error message!".to_owned(),
            kind: PassportElementErrorKind::DataField(
                PassportElementErrorDataField {
                    r#type: PassportElementErrorDataFieldType::InternalPassport,
                    field_name: "The field name".to_owned(),
                    data_hash: "This is a data hash".to_owned(),
                },
            ),
        };

        assert_eq!(
            serde_json::to_string(&data).unwrap(),
            r#"{"message":"This is an error message!","source":"data","type":"internal_passport","field_name":"The field name","data_hash":"This is a data hash"}"#
        );
    }
}
