/// The default untrusted comment.
pub const DEFAULT_COMMENT: &str = "signature from rsign secret key";

/// The default environment variable for the directory of the `rsign` tool.
pub const SIG_DEFAULT_CONFIG_DIR_ENV_VAR: &str = "RSIGN_CONFIG_DIR";

/// The default configuration directory of the `rsign` tool.
pub const SIG_DEFAULT_CONFIG_DIR: &str = ".rsign";

/// The default file name for the public key.
pub const SIG_DEFAULT_PKFILE: &str = "rsign.pub";

/// The default file name for the secret key.
pub const SIG_DEFAULT_SKFILE: &str = "rsign.key";

/// The default suffix for signatures.
pub const SIG_SUFFIX: &str = ".minisig";

pub const CHK_ALG: [u8; 2] = *b"B2";
pub const CHK_BYTES: usize = 32;
pub const COMMENT_PREFIX: &str = "untrusted comment: ";
pub const KDF_ALG: [u8; 2] = *b"Sc";
pub const KDF_SALTBYTES: usize = 32;
pub const KEYNUM_BYTES: usize = 8;
pub const MEMLIMIT: usize = 33_554_432;
pub const OPSLIMIT: u64 = 1_048_576;
pub const MEMLIMIT_MAX: usize = 1_073_741_824;
pub const N_LOG2_MAX: u8 = 20;
pub const PASSWORD_MAXBYTES: usize = 1024;
pub const PK_B64_ENCODED_LEN: usize = 56;
pub const PREHASH_BYTES: usize = 64;
pub const PUBLICKEY_BYTES: usize = 32;
pub const SECRETKEY_BYTES: usize = 64;
pub const SECRETKEY_DEFAULT_COMMENT: &str = "rsign encrypted secret key";
pub const SIGALG_PREHASHED: [u8; 2] = *b"ED";
pub const SIGALG: [u8; 2] = *b"Ed";
pub const SIGNATURE_BYTES: usize = 64;
pub const TRUSTED_COMMENT_PREFIX_LEN: usize = 17;
pub const TRUSTED_COMMENT_PREFIX: &str = "trusted comment: ";
pub const TWOBYTES: usize = 2;
