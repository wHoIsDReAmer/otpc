#[derive(clap::SubCommand, Debug, Clone)]
pub enum Command {
    /// 저장된 OTP 키 목록 표시
    #[clap(name = "list", about = "저장된 모든 OTP 키 목록 표시")]
    List,
    
    /// 특정 계정의 현재 OTP 코드 생성
    #[clap(name = "code", about = "특정 계정의 현재 OTP 코드 생성")]
    Code {
        /// 계정 이름 또는 ID
        #[clap(long, short)]
        account: String,
    },
    
    /// OTP 키 삭제
    #[clap(name = "delete", about = "저장된 OTP 키 삭제")]
    Delete {
        /// 삭제할 계정 이름 또는 ID
        #[clap(long, short)]
        account: String,
    },
    
    /// QR 코드에서 OTP 키 가져오기
    #[clap(name = "import", about = "QR 코드 이미지에서 OTP 키 가져오기")]
    Import {
        /// QR 코드 이미지 파일 경로
        #[clap(long, short)]
        file: std::path::PathBuf,
    },

    /// OTP 키 불러오기
    #[clap(name = "load", about = "OTP 키 불러오기")]
    Load {
        /// OTP 시크릿 키
        #[clap(long, short)]
        secret: String,

        /// 계정 이름
        #[clap(long, short)]
        account: String,
    },
    
}