# otpc - OTP Command Line Tool

[한국어](#korean) | [English](#english)

---

## <a name="korean"></a>한국어

`otpc`는 명령줄 인터페이스(CLI)를 통해 TOTP(시간 기반 일회용 비밀번호) 및 HOTP(HMAC 기반 일회용 비밀번호) 코드를 관리하고 생성하는 간단한 도구입니다.

### 기능

*   저장된 OTP 계정 목록 표시
*   특정 계정에 대한 현재 TOTP/HOTP 코드 생성
*   계정 추가 (직접 로드 또는 QR 코드 임포트 - QR 임포트는 개발 중)
*   계정 삭제
*   설정 파일 (`~/.otpc/config.toml`)을 통한 계정 정보 관리

### 설치

1.  **Rust 설치:** Rust가 설치되어 있지 않다면 [rustup](https://rustup.rs/)을 사용하여 설치합니다.
2.  **저장소 복제:**
    ```bash
    git clone <repository-url>
    cd otpc
    ```
3.  **빌드:**
    ```bash
    cargo build --release
    ```
    생성된 바이너리는 `target/release/otpc` 에서 찾을 수 있습니다. 필요하다면 이 바이너리를 PATH 환경 변수에 등록된 디렉토리 (예: `/usr/local/bin` 또는 `~/.local/bin`)로 복사하여 어디서든 `otpc` 명령을 사용할 수 있게 하십시오.

### 사용법

```
otpc [COMMAND]
```

#### 명령어

*   **`list`**: 저장된 모든 OTP 계정 목록을 표시합니다.
    ```bash
    otpc list
    ```

*   **`code`**: 특정 계정의 현재 OTP 코드를 생성합니다.
    ```bash
    # TOTP 코드 생성 (기본값)
    otpc code -a <account_name>

    # HOTP 코드 생성 (카운터 값 필요)
    otpc code -a <account_name> --otp-type hotp -c <counter_value>
    ```
    *   `-a`, `--account`: 계정 이름 또는 ID
    *   `--otp-type`: OTP 타입 (`totp` 또는 `hotp`, 기본값: `totp`)
    *   `-c`, `--counter`: HOTP에 사용될 카운터 값 (HOTP 타입인 경우 필수)

*   **`delete`**: 특정 계정을 삭제합니다.
    ```bash
    otpc delete -a <account_name>
    ```
    *   `-a`, `--account`: 삭제할 계정 이름 또는 ID

*   **`load`**: 새 계정 정보를 직접 로드합니다.
    ```bash
    otpc load -s <secret_key> -a <account_name> [-i <issuer_name>]
    ```
    *   `-s`, `--secret`: Base32로 인코딩된 OTP 비밀 키
    *   `-a`, `--account`: 계정 이름
    *   `-i`, `--issuer` (선택 사항): 발급자 이름

*   **`import`** (개발 중): QR 코드 이미지 파일로부터 OTP 키를 가져옵니다.
    ```bash
    otpc import -f <path/to/qrcode.png>
    ```
    *   `-f`, `--file`: QR 코드 이미지 파일 경로

#### 설정 파일

계정 정보는 `~/.otpc/config.toml` 파일에 저장됩니다. 파일 형식은 다음과 같습니다:

```toml
[[accounts]]
name = "example_account_1"
secret = "JBSWY3DPEHPK3PXP"
issuer = "Example Inc."

[[accounts]]
name = "another_account"
secret = "NBQXEYLDM5WGKZTFNQXC4LBA"
issuer = "Another Service"
```

**주의:** 설정 파일에는 민감한 정보(비밀 키)가 포함되어 있으므로, 파일 권한(`0o600`)을 적절하게 유지하여 다른 사용자가 접근하지 못하도록 하십시오.

### 기여

버그 리포트나 기능 제안은 언제나 환영합니다. 이슈를 열거나 풀 리퀘스트를 보내주세요.

### 라이선스

이 프로젝트는 [라이선스 이름 - 예: MIT] 라이선스 하에 배포됩니다. 자세한 내용은 `LICENSE` 파일을 참조하십시오. (프로젝트에 라이선스 파일 추가 필요)

---

## <a name="english"></a>English

`otpc` is a simple command-line interface (CLI) tool for managing and generating TOTP (Time-based One-Time Password) and HOTP (HMAC-based One-Time Password) codes.

### Features

*   List saved OTP accounts
*   Generate the current TOTP/HOTP code for a specific account
*   Add accounts (manual load or QR code import - QR import is under development)
*   Delete accounts
*   Manage account information via a configuration file (`~/.otpc/config.toml`)

### Installation

1.  **Install Rust:** If you don't have Rust installed, install it using [rustup](https://rustup.rs/).
2.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd otpc
    ```
3.  **Build:**
    ```bash
    cargo build --release
    ```
    The compiled binary can be found at `target/release/otpc`. If desired, copy this binary to a directory in your PATH (e.g., `/usr/local/bin` or `~/.local/bin`) to use the `otpc` command globally.

### Usage

```
otpc [COMMAND]
```

#### Commands

*   **`list`**: Displays a list of all saved OTP accounts.
    ```bash
    otpc list
    ```

*   **`code`**: Generates the current OTP code for a specific account.
    ```bash
    # Generate TOTP code (default)
    otpc code -a <account_name>

    # Generate HOTP code (requires counter value)
    otpc code -a <account_name> --otp-type hotp -c <counter_value>
    ```
    *   `-a`, `--account`: The name or ID of the account.
    *   `--otp-type`: The type of OTP (`totp` or `hotp`, default: `totp`).
    *   `-c`, `--counter`: The counter value to be used for HOTP (required for HOTP type).

*   **`delete`**: Deletes a specific account.
    ```bash
    otpc delete -a <account_name>
    ```
    *   `-a`, `--account`: The name or ID of the account to delete.

*   **`load`**: Loads new account information directly.
    ```bash
    otpc load -s <secret_key> -a <account_name> [-i <issuer_name>]
    ```
    *   `-s`, `--secret`: The Base32 encoded OTP secret key.
    *   `-a`, `--account`: The account name.
    *   `-i`, `--issuer` (optional): The issuer name.

*   **`import`** (Under development): Imports an OTP key from a QR code image file.
    ```bash
    otpc import -f <path/to/qrcode.png>
    ```
    *   `-f`, `--file`: The path to the QR code image file.

#### Configuration File

Account information is stored in the `~/.otpc/config.toml` file. The file format is as follows:

```toml
[[accounts]]
name = "example_account_1"
secret = "JBSWY3DPEHPK3PXP"
issuer = "Example Inc."

[[accounts]]
name = "another_account"
secret = "NBQXEYLDM5WGKZTFNQXC4LBA"
issuer = "Another Service"
```

**Caution:** The configuration file contains sensitive information (secret keys). Ensure that the file permissions (`0o600`) are maintained appropriately to prevent access by other users.

### Contributing

Bug reports and feature suggestions are always welcome. Please open an issue or submit a pull request.

### License

This project is distributed under the [License Name - e.g., MIT] license. See the `LICENSE` file for details. (Need to add a LICENSE file to the project). 