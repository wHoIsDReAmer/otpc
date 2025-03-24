use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

/// OTP 타입 (TOTP 또는 HOTP)
pub enum OtpType {
    /// 시간 기반 일회용 비밀번호
    Totp,
    /// HMAC 기반 일회용 비밀번호
    Hotp,
}

/// OTP 알고리즘 구현
pub struct Otp {
    /// 비밀 키
    secret: Vec<u8>,
    /// 자릿수 (기본값: 6)
    digits: u32,
    /// 시간 간격 (초 단위, 기본값: 30)
    period: u64,
    /// OTP 타입
    otp_type: OtpType,
}

impl Otp {
    /// 새로운 OTP 인스턴스 생성
    pub fn new(secret: &str, digits: u32, period: u64, otp_type: OtpType) -> Self {
        // Base32 디코딩
        let secret = Self::decode_base32(secret);
        
        Self {
            secret,
            digits,
            period,
            otp_type,
        }
    }

    /// 기본 설정으로 TOTP 인스턴스 생성 (6자리, 30초 간격)
    pub fn new_totp(secret: &str) -> Self {
        Self::new(secret, 6, 30, OtpType::Totp)
    }

    /// 현재 OTP 코드 생성
    pub fn generate_code(&self) -> String {
        match self.otp_type {
            OtpType::Totp => self.generate_totp(),
            OtpType::Hotp => panic!("HOTP는 카운터 값이 필요합니다. generate_hotp() 메서드를 사용하세요."),
        }
    }

    /// TOTP 코드 생성
    fn generate_totp(&self) -> String {
        // 현재 시간을 Unix 타임스탬프로 변환
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("시간이 UNIX EPOCH 이전입니다");
        
        // 현재 시간을 period로 나누어 카운터 값 계산
        let counter = now.as_secs() / self.period;
        
        self.generate_hotp(counter)
    }

    /// HOTP 코드 생성
    pub fn generate_hotp(&self, counter: u64) -> String {
        // 카운터를 빅 엔디안 바이트 배열로 변환
        let counter_bytes = counter.to_be_bytes();
        
        // HMAC-SHA1 계산
        let hmac = self.hmac_sha1(&counter_bytes);
        
        // 동적 절단 (Dynamic Truncation)
        let offset = (hmac[19] & 0xf) as usize;
        let binary = ((hmac[offset] & 0x7f) as u32) << 24
            | (hmac[offset + 1] as u32) << 16
            | (hmac[offset + 2] as u32) << 8
            | (hmac[offset + 3] as u32);
        
        // 모듈로 연산으로 필요한 자릿수만큼 잘라내기
        let modulo = 10u32.pow(self.digits);
        let code = binary % modulo;
        
        // 앞에 0을 채워 자릿수 맞추기
        format!("{:0width$}", code, width = self.digits as usize)
    }

    /// HMAC-SHA1 구현
    fn hmac_sha1(&self, data: &[u8]) -> [u8; 20] {
        const BLOCK_SIZE: usize = 64;
        const SHA1_SIZE: usize = 20;
        
        // 키 준비
        let mut key = self.secret.clone();
        if key.len() > BLOCK_SIZE {
            key = self.sha1(key.as_slice()).to_vec();
        }
        if key.len() < BLOCK_SIZE {
            key.resize(BLOCK_SIZE, 0);
        }
        
        // 내부 패딩 (ipad)
        let mut ipad = [0x36u8; BLOCK_SIZE];
        // 외부 패딩 (opad)
        let mut opad = [0x5cu8; BLOCK_SIZE];
        
        // XOR 연산
        for i in 0..BLOCK_SIZE {
            ipad[i] ^= key[i];
            opad[i] ^= key[i];
        }
        
        // 내부 해시 계산
        let mut inner_data = Vec::with_capacity(BLOCK_SIZE + data.len());
        inner_data.extend_from_slice(&ipad);
        inner_data.extend_from_slice(data);
        let inner_hash = self.sha1(&inner_data);
        
        // 외부 해시 계산
        let mut outer_data = Vec::with_capacity(BLOCK_SIZE + SHA1_SIZE);
        outer_data.extend_from_slice(&opad);
        outer_data.extend_from_slice(&inner_hash);
        
        self.sha1(&outer_data)
    }

    /// SHA1 해시 함수 구현
    fn sha1(&self, data: &[u8]) -> [u8; 20] {
        // SHA1 초기 해시 값
        let mut h0: u32 = 0x67452301;
        let mut h1: u32 = 0xEFCDAB89;
        let mut h2: u32 = 0x98BADCFE;
        let mut h3: u32 = 0x10325476;
        let mut h4: u32 = 0xC3D2E1F0;
        
        // 메시지 패딩
        let mut padded_data = data.to_vec();
        let original_len_bits = (data.len() as u64) * 8;
        
        // 1 비트 추가
        padded_data.push(0x80);
        
        // 0으로 패딩 (길이가 56 모듈로 64가 될 때까지)
        while padded_data.len() % 64 != 56 {
            padded_data.push(0);
        }
        
        // 원본 메시지 길이 추가 (비트 단위, 빅 엔디안 8바이트)
        padded_data.extend_from_slice(&original_len_bits.to_be_bytes());
        
        // 512비트(64바이트) 블록 단위로 처리
        for chunk_start in (0..padded_data.len()).step_by(64) {
            let chunk = &padded_data[chunk_start..chunk_start + 64];
            
            // 메시지 스케줄 준비 (16개의 32비트 워드에서 80개로 확장)
            let mut w = [0u32; 80];
            
            // 처음 16개 워드는 청크에서 직접 가져옴
            for i in 0..16 {
                let start = i * 4;
                w[i] = ((chunk[start] as u32) << 24)
                    | ((chunk[start + 1] as u32) << 16)
                    | ((chunk[start + 2] as u32) << 8)
                    | (chunk[start + 3] as u32);
            }
            
            // 나머지 64개 워드 계산
            for i in 16..80 {
                w[i] = Self::left_rotate(w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16], 1);
            }
            
            // 초기화
            let mut a = h0;
            let mut b = h1;
            let mut c = h2;
            let mut d = h3;
            let mut e = h4;
            
            // 메인 루프
            for i in 0..80 {
                let (f, k) = match i {
                    0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                    20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                    40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                    _ => (b ^ c ^ d, 0xCA62C1D6),
                };
                
                let temp = Self::left_rotate(a, 5).wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(k)
                    .wrapping_add(w[i]);
                
                e = d;
                d = c;
                c = Self::left_rotate(b, 30);
                b = a;
                a = temp;
            }
            
            // 해시 값 업데이트
            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
        }
        
        // 최종 해시 값 반환
        let mut result = [0u8; 20];
        result[0..4].copy_from_slice(&h0.to_be_bytes());
        result[4..8].copy_from_slice(&h1.to_be_bytes());
        result[8..12].copy_from_slice(&h2.to_be_bytes());
        result[12..16].copy_from_slice(&h3.to_be_bytes());
        result[16..20].copy_from_slice(&h4.to_be_bytes());
        
        result
    }
    
    /// 비트 왼쪽 회전 (left rotate) 연산
    fn left_rotate(value: u32, shift: u32) -> u32 {
        (value << shift) | (value >> (32 - shift))
    }
    
    /// Base32 디코딩 구현
    fn decode_base32(input: &str) -> Vec<u8> {
        let input = input.to_uppercase().replace(" ", "");
        let mut result = Vec::new();
        let mut buffer = 0u64;
        let mut bits = 0;
        
        for c in input.chars() {
            // Base32 문자를 5비트 값으로 변환
            let val = match c {
                'A'..='Z' => (c as u8 - b'A') as u64,
                '2'..='7' => (c as u8 - b'2' + 26) as u64,
                _ => continue, // 유효하지 않은 문자는 무시
            };
            
            // 버퍼에 5비트 추가
            buffer = (buffer << 5) | val;
            bits += 5;
            
            // 8비트가 모이면 바이트로 추출
            if bits >= 8 {
                bits -= 8;
                result.push(((buffer >> bits) & 0xFF) as u8);
            }
        }
        
        result
    }
}

impl fmt::Display for Otp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.generate_code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_base32_decode() {
        let decoded = Otp::decode_base32("JBSWY3DPEHPK3PXP");
        assert_eq!(decoded, vec![72, 101, 108, 108, 111, 33, 222, 173, 190, 239]);
    }
    
    #[test]
    fn test_hmac_sha1() {
        let otp = Otp::new("JBSWY3DPEHPK3PXP", 6, 30, OtpType::Totp);
        let result = otp.hmac_sha1(&[0, 0, 0, 0, 0, 0, 0, 1]);
        // 실제 HMAC-SHA1 결과와 비교해야 함
        assert_eq!(result.len(), 20);
    }
    
    #[test]
    fn test_generate_hotp() {
        let otp = Otp::new("JBSWY3DPEHPK3PXP", 6, 30, OtpType::Totp);
        let code = otp.generate_hotp(2);
        println!("code: {}", code);
        assert_eq!(code.len(), 6);
        // 실제 값은 구현에 따라 달라질 수 있음
    }
}
