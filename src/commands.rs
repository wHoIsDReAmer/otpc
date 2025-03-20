#[derive(clap::SubCommand, Debug, Clone)]
pub enum Command {
    /// OTP 키 생성
    #[clap(name = "generate", about = "새로운 OTP 키 생성")]
    Generate {
        /// 계정 이름 (예: example@gmail.com)
        #[clap(long, short)]
        account: String,
        
        /// 서비스 이름 (예: Google, GitHub)
        #[clap(long, short)]
        issuer: Option<String>,
    },
    
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
}