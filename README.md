# KEC 전선관/허용전류 산출 프로그램

한국전기설비규정(KEC) 및 KS C IEC 60364-5-52 기준 전선관 크기 및 허용전류 계산 프로그램입니다.
MacOS 및 Windows 환경을 모두 지원하며, GitHub Actions를 통해 자동으로 빌드됩니다.

## 📥 다운로드

최신 버전의 Windows용 설치 파일(`.exe`/`.msi`) 및 macOS용 이미지(`.dmg`)는 **[Releases 페이지](https://github.com/Machrie/kec-calculator/releases)**에서 다운로드할 수 있습니다.

## 📌 적용 기준

- **허용전류:** KS C IEC 60364-5-52 부속서 B (주변온도 30°C 기준)
- **전선관 점유율:** KEC 232.2 - 내부 단면적의 1/3 (33%) 이하
- **최고 허용온도:** PVC 70°C, XLPE/EPR 90°C
- **케이블 외경:** 제조사 규격표 기준 (일반적인 평균치 적용)

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
| D1/D2 | 지중 매설 덕트 |
| E/F/G | 케이블 트레이 및 사다리 |

### 주요 기능
- ✅ **자동 필터링**: 전선 종류별 가닥수/규격 자동 필터링
- ✅ **KEC 표준 준수**: 공사방법별 허용전류 자동 계산 (IEC 60364-5-52)
- ✅ **보정 계수**: 심선 수 감소계수 및 토양 열저항(지중) 고려
- ✅ **전선관 산출**: 33% 점유율 기준 추천 전선관 크기 자동 산출
- ✅ **접지선 포함**: 접지선 굵기에 따른 단면적 포함 계산

## 🚀 개발 환경 설정

이 프로젝트는 **Rust**와 **Vanilla HTML/JS/CSS** (Tauri v2)로 구성되어 있습니다.

### 필수 요건

1. **Rust** (1.77 이상)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (LTS 버전 권장)

3. **시스템 의존성**
   - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
   - **Windows**: [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - **Linux**: `libwebkit2gtk-4.0-dev`, `build-essential`, `libssl-dev`, `libgtk-3-dev`

### 의존성 설치
```bash
npm install
# Rust dependencies are automatically handled by cargo
```

## 🔧 빌드 및 실행

### 개발 모드 (Hot Reload)
```bash
cargo tauri dev
```

### 릴리즈 빌드 (Local)
로컬 환경에 맞는 실행 파일을 생성합니다.
```bash
cargo tauri build
```
빌드 결과물 위치:
- `src-tauri/target/release/bundle/`

### 릴리즈 빌드 (GitHub Actions)
이 저장소에는 GitHub Actions 워크플로우가 포함되어 있습니다. 태그를 푸시하면 자동으로 Windows와 macOS용 설치 파일이 빌드되어 Release에 등록됩니다.

```bash
# 버전 태그 푸시
git tag v1.0.1
git push origin v1.0.1
```

## 📁 프로젝트 구조

```
kec/
├── .github/workflows/   # CI/CD (GitHub Actions)
├── ui/                  # 프론트엔드 (웹 UI)
│   ├── index.html       # 메인 HTML 구조
│   ├── styles.css       # 다크 테마 스타일링
│   └── app.js           # UI 로직 및 Tauri 통신
└── src-tauri/           # Rust 백엔드
    ├── src/main.rs      # KEC 계산 로직 (Core Logic)
    ├── tauri.conf.json  # Tauri 프로젝트 설정
    └── capabilities/    # 권한 설정
```

## ⚠️ 주의사항

- 본 프로그램은 **참고용**입니다.
- 실제 설계 시 **공인된 KEC 자료, 감리 기준 및 제조사 규격표**를 반드시 확인하세요.
- 허용전류 값은 주변온도 30°C (지중 20°C) 기준이며, 현장 조건에 따라 보정계수가 달라질 수 있습니다.

## 📚 참고 자료

- [대한전기협회 KEC 홈페이지](https://kec.kea.kr)
- KS C IEC 60364-5-52 (저압전기설비 - 배선설비)
- KEC 232.2 (전선관 규격)

## 라이선스

MIT License
