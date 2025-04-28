use clap::Parser;

use crate::otp::OtpType;

#[derive(Parser, Debug, Clone)]
pub enum Command {
    /// Show all saved OTP keys
    #[clap(name = "list", about = "Show all saved OTP keys")]
    List,
    
    /// Generate the current OTP code for a specific account
    #[clap(name = "code", about = "Generate the current OTP code for a specific account")]
    Code {
        /// The name or ID of the account
        #[clap(long, short)]
        account: String,

        /// The type of OTP code to generate
        #[clap(long, short, default_value = "totp")]
        otp_type: OtpType,

        /// The counter value for HOTP
        #[clap(long, short)]
        counter: Option<u64>,
    },
    
    /// Delete an account
    #[clap(name = "delete", about = "Delete an account")]
    Delete {
        /// The name or ID of the account to delete
        #[clap(long, short)]
        account: String,
    },
    
    /// Import an OTP key from a QR code image
    #[clap(name = "import", about = "Import an OTP key from a QR code image")]
    Import {
        /// The path to the QR code image file
        #[clap(long, short)]
        file: std::path::PathBuf,
    },

    /// Load an OTP key
    #[clap(name = "load", about = "Load an OTP key")]
    Load {
        /// The OTP secret key
        #[clap(long, short)]
        secret: String,

        /// The account name
        #[clap(long, short)]
        account: String,

        /// The issuer name
        #[clap(long, short)]
        issuer: Option<String>,
    },
    
}