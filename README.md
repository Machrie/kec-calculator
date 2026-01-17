# KEC 전선관/허용전류 산출 프로그램

한국전기설비규정(KEC) 및 KS C IEC 60364-5-52 기준 전선관 크기 및 허용전류 계산 프로그램입니다.

## 📌 적용 기준

- **허용전류:** KS C IEC 60364-5-52 부속서 B (주변온도 30°C 기준)
- **전선관 점유율:** KEC 232.2 - 내부 단면적의 1/3 (33%) 이하
- **최고 허용온도:** PVC 70°C, XLPE/EPR 90°C
- **케이블 외경:** 제조사 규격표 기준

## ✨ 기능

### 지원 전선 종류
| 코드 | 종류 | 절연체 | 최고온도 |
|------|------|--------|----------|
| HFIX | 저독성 난연 전선 | XLPE | 90°C |
| TFR-CV | 난연 트레이용 케이블 | XLPE | 90°C |
| CV | 일반 전력 케이블 | XLPE | 90°C |
| FR-CV | 내화 케이블 | XLPE | 90°C |
| TFR-8 | 내열 케이블 | XLPE | 90°C |

### 지원 공사방법 (KEC 기준)
| 코드 | 설명 |
|------|------|
| A1/A2 | 단열벽 속 전선관 |
| B1/B2 | 벽면 고정 전선관 |
| C | 벽면/천정 직접 고정 (공기 중) |
| D1 | 지중 매설 덕트 |
| E/F | 케이블 트레이 |

### 주요 기능
- ✅ 전선 종류별 가닥수/규격 자동 필터링
- ✅ 공사방법별 허용전류 자동 계산
- ✅ 심선 수 감소계수 적용
- ✅ 33% 점유율 기준 추천 전선관 크기 산출
- ✅ 접지선 포함 계산

## 🚀 개발 환경 설정

### 필수 요건

1. **Rust** (1.70 이상)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Tauri CLI**
   ```bash
   cargo install tauri-cli
   ```

3. **시스템 의존성**
   - macOS: Xcode Command Line Tools
   - Windows: [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) + [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
   - Linux: `libwebkit2gtk-4.0-dev`, `build-essential`, `libssl-dev`, `libgtk-3-dev`

## 🔧 빌드 및 실행

### 개발 모드
```bash
cd src-tauri
cargo tauri dev
```

### 릴리즈 빌드 (exe 생성)
```bash
cd src-tauri
cargo tauri build
```

빌드 결과물 위치:
- **Windows**: `src-tauri/target/release/bundle/msi/` 또는 `nsis/`
- **macOS**: `src-tauri/target/release/bundle/dmg/`

## 📁 프로젝트 구조

```
kec/
├── README.md
├── ui/                      # 프론트엔드 (웹 UI)
│   ├── index.html          # 메인 HTML
│   ├── styles.css          # 다크 테마 스타일
│   └── app.js              # Tauri 연동 및 필터링 로직
└── src-tauri/               # Rust 백엔드
    ├── Cargo.toml          # 의존성
    ├── tauri.conf.json     # Tauri 설정
    ├── build.rs            # 빌드 스크립트
    ├── capabilities/       # 권한 설정
    └── src/main.rs         # KEC 계산 로직 (허용전류표, 외경 데이터)
```

## ⚠️ 주의사항

- 본 프로그램은 **참고용**입니다.
- 실제 설계 시 **공인된 KEC 자료 및 제조사 규격표**를 반드시 확인하세요.
- 허용전류 값은 주변온도 30°C 기준이며, 다른 조건에서는 보정계수를 적용해야 합니다.

## 📚 참고 자료

- [대한전기협회 KEC 홈페이지](https://kec.kea.kr)
- KS C IEC 60364-5-52 (저압전기설비 - 배선설비)
- KEC 232.2 (전선관 규격)
- KEC 232.5 (허용전류)

## 라이선스

MIT License
