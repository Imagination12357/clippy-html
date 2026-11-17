# Full Audit — clippy-html — FA-001

- Status: **Complete**
- 기준: `main`, `08945ba063cd6ac9515e3e29e2eefcf8fcfa2272`. 검수 전후 추적 소스 변경 없음. 감사 기록은 기준점 비교에서 제외했다.
- 범위: 전체 자체 Rust 소스, JS/CSS, 템플릿, 테스트·fixture, example, manifest·lockfile, CI 및 현재 사용자 계약 문서.
- 제외: `target/`, `node_modules/`, `.git/`, `.audit/`, `.superpowers/`, 외부 의존성과 vendored 구현 자체. 외부 자산의 자체 연결·출력 코드는 포함했다. 과거 계획·검증 기록과 ignored `DETAILS.md`는 배경 자료로만 사용했다.
- 환경: Windows x86_64, Rust/Cargo 1.99.1, Node 24.14.1. 별도로 설치된 Rust 1.88.0의 전체 target 컴파일도 확인했다.

## Summary

**확정 결함 2건: Important 1건, Minor 1건. Critical 0건.** 미확정 결함은 없으며, 조사 후 기각한 후보 1건을 별도로 보존했다.

우선순위는 Windows의 새 보고서에 내부 metadata를 요청했을 때 경로 대소문자 차이로 생성과 재시도가 모두 실패하는 문제다. 다른 결함은 특정 소스 페이지를 Settings 전용 페이지로 잘못 분류하는 문제다. 둘 다 기존 테스트가 통과하는 기준점에서 추가 재현으로 확인했다.

애플리케이션·테스트·manifest·Git 이력은 수정하지 않았다. 감사 상태와 재현 스크립트만 이 실행 디렉터리에 저장했다. 판정의 원본은 `state.yaml`이다.

## Findings

### FA-001-003 — Windows 경로 대소문자 차이로 내부 metadata 요청과 재시도 실패

- Severity / Confidence: **Important (Should Fix) / Confirmed**
- 위치: `src/metadata/output.rs:53-66`, `src/output.rs:134-138`. 호출 경로는 `src/main.rs`의 metadata 준비 → staging → HTML publication이다.
- 조건: 대소문자를 구분하지 않는 Windows 파일시스템에서 출력 폴더가 아직 없고 다음처럼 동일한 물리적 폴더의 표기가 다르다.

```text
cargo-clippy-html --offline --output Report --metadata report/navigation.json -- --offline
```

예상 결과는 HTML과 내부 JSON을 함께 staging한 뒤 게시하는 것이다. 실제로는 Clippy가 정상 종료하고 진단 수집도 완전하지만, 도구가 `Output path changed during generation`으로 종료 1을 반환한다. HTML은 생성되지 않고 marker 없는 빈 `report` 폴더가 남는다. `--metadata Report/navigation.json`으로 표기를 맞춰 재시도해도 `Refusing to overwrite non-report directory`로 거부된다. 사용자가 남은 폴더를 수동 정리해야 새 보고서를 만들 수 있다. 기존 보고서나 소스의 손실은 관찰하지 않았다.

원인은 `MetadataTarget::prepare`의 대소문자 구분 `Path::strip_prefix`다. 새 폴더는 아직 canonicalize할 수 없어 `Report`와 `report`의 표기가 남는다. 따라서 내부 JSON을 `External`로 오인한다. 이후 `stage`가 실제 출력 폴더를 미리 만들고, HTML 게시의 경로·소유권 재검사가 이를 거부한다.

**검증:** `python .audit/full-audit/001/probe-projects.py`를 승인된 sandbox 외부 실행으로 수행했다. 모든 프로젝트와 생성물은 격리 임시 폴더에만 생성했다.

| 검사 | 결과 |
|---|---|
| 새 `Report`, JSON `report/navigation.json` | 종료 1, `directoryLeft=true`, `indexExists=false` |
| 남은 폴더에서 JSON을 `Report/navigation.json`으로 재시도 | 종료 1, non-report directory 거부 |
| 별도 새 `Control`, JSON `Control/navigation.json` | 종료 0, `index.html` 생성 |

기존 Windows 테스트는 이미 존재하는 생성 파일의 case alias 충돌과 writer lock alias를 검증하지만, 새 출력의 내부/외부 metadata 분류는 다루지 않는다. 수정 시 실제 파일시스템의 경로 동일성과 새 경로의 분류를 함께 검증해야 한다. 이 감사에서는 수정하지 않았다.

### FA-001-001 — `settings.html`인 소스 페이지가 Settings 전용 페이지로 초기화됨

- Severity / Confidence: **Minor (Nice to Have) / Confirmed**
- 위치: `assets/settings.js:7,87-103`, `src/render/paths.rs:132-137`.
- 조건: 유효한 Rust 코드가 확장자 없는 `settings` 파일을 모듈로 사용하고, 그 파일에서 진단이 발생한다.

```rust
// src/lib.rs
#[path = "settings"]
mod settings;

// src/settings
pub fn demo() { let x = 1_u32; let _ = x.clone(); }
```

렌더러는 해당 소스를 정상적으로 `src/settings.html`에 출력한다. 예상 동작은 다른 소스 페이지처럼 toolbar의 Settings 팝오버를 제공하는 것이다. 실제로는 설정 폼이 소스 본문 아래에 삽입되고, toolbar Settings 메뉴는 숨김 상태로 초기화되지 않는다. 소스와 진단 탐색은 유지된다.

`settings.js`가 `window.location.pathname.endsWith("/settings.html")`만으로 전용 페이지를 판정하기 때문이다. 전용 페이지 분기에서 조기 반환하므로 일반 팝오버 초기화가 실행되지 않는다. report-root의 실제 `settings.html`에는 전용 페이지 처리가 필요하지만, 소스의 동일 basename은 그 페이지가 아니다.

**검증:** 같은 Python 재현 스크립트가 실제 Clippy로 보고서를 생성했다. 도구 종료는 0이고 metadata의 해당 소스 URL은 `src/settings.html#L1`이었다. `probe-settings.cjs`로 실제 생성 HTML에 현재 `settings-storage.js`와 `settings.js`를 초기화하자 다음을 관찰했다.

```json
{"panelInsideMain":true,"popoverExists":false,"menuHidden":true}
```

이는 DOM·분기 오류의 재현이다. 실제 브라우저의 레이아웃 검증은 수행하지 않았다. 기존 Settings 테스트는 이 basename의 소스 페이지를 검사하지 않는다. Help에는 같은 이름의 소스 페이지를 구별하는 DOM 판정과 회귀 테스트가 있다.

## Disproven leads

### FA-001-002 — CRLF/BOM에 의한 진단 byte 좌표 이동 의심

- Confidence: **Disproven**, Severity: 없음.
- 위치: `src/source.rs:67-110`, `src/render/highlight.rs:220-243`.
- LF, CRLF, UTF-8 BOM 파일 각각의 3행 `clone_on_copy` 진단을 실제 Clippy로 생성했다. 모두 종료 0이며 metadata가 fallback finding anchor 대신 검증된 `src/lib.rs.html#L3`를 내보냈다. 이 조건에서는 byte 좌표 불일치를 확인하지 못했다.
- 기존 Unicode/CRLF/다중 행 범위 테스트와 실제 Git patch 적용 테스트도 통과했다. 이 결과를 모든 가능한 BOM/좌표 조합의 보장으로 확대하지 않는다.

## Coverage and limitations

11개 검수 단위를 모두 검토했다. 파일 열람이나 테스트 통과만으로 완료 처리하지 않고, 아래의 실행 경로·계약·실패 조건을 추적했다.

| 검수 단위 | 검토 내용 | 상태 |
|---|---|---|
| CLI·Cargo 수집 | 두 인자 경계, metadata 문맥, 두 pipe drain, JSON 손상, build-finished, 종료 코드 | Reviewed |
| 진단 모델·정규화·병합 | recursive child, multipart suggestion, package/severity/span 구별, target 병합, ID 충돌 | Reviewed |
| category·fetch·생성 example | 공식 selector, HTTPS/body/time 제한, offline·실패 fallback, Unknown, ID 독립성 | Reviewed |
| 소스·범위·highlight | workspace/최근접 manifest/target 경계, UTF-8·snippet·좌표, 겹침·삽입·들여쓰기 | Reviewed |
| 페이지·경로·템플릿·자산 | 안전한 이름, 충돌, 상대 링크, anchor, escaping, notices/fonts 및 staging 검증 | Reviewed |
| suggestion·copy | 전체 원문 확인, overlap·path·size·deadline, 역순 적용, 실제 Git patch, clipboard fallback | Reviewed |
| 출력 게시 | 보호 root, ownership, symlink/reparse, lock 수명, staging, rollback·backup 보존 | Reviewed |
| navigation metadata | v1 DTO, 실제 URL/fragment, 대표 위치 fallback, 내부 충돌·외부 게시; FA-001-003 | Reviewed |
| 검색·Help | Unicode AND/rank, 로컬 URL, 제한/실제 count, history·IME·focus, 팝오버·Escape 조정 | Reviewed |
| Settings·Summary·CSS·favicon | 저장 실패·테마, 페이지 판정, hash reveal, 키 입력 보호, asset 연결; FA-001-001 | Reviewed |
| 테스트·fixture·빌드·CI·문서 | 임시 복사 격리, assertion, manifests/MSRV, CI matrix·실패 전파, 현재 공개 계약 | Reviewed |

### 실행한 검사

| 명령 | 결과 |
|---|---|
| `cargo test --locked --offline` | 단위 79 + 통합 15 + metadata 5 = **99/99 통과** |
| `npm test` | **49/49 통과** |
| `cargo fmt --check` | 통과 |
| `cargo clippy --all-targets --locked --offline -- -D warnings` | 통과 |
| `cargo +1.88.0 check --all-targets --locked --offline` | 통과 |
| Git Bash로 `tests/ci-dogfood.test.sh` | stub Cargo의 **4개 사례 통과**; 실제 설치·배포 없음 |
| `python .audit/full-audit/001/probe-projects.py` | LF/CRLF/BOM 대조군 정상; 위 두 결함 재현; Windows 경로의 matching-case 대조군 정상 |

초기 sandbox 실행은 Windows canonicalize/manifest/temp 경로 접근이나 MSYS object 접근에서 거부됐다. 필요한 동일 검사는 승인을 받아 sandbox 밖에서 재실행했고 통과했다. 이 환경 실패를 제품 결함으로 집계하지 않았다. 의존성을 설치하지 않았다.

`zg`는 workspace 인덱스가 없어 `--rg`로 대체했다. `codeknow debt`도 기존 graph가 없어 실행 가능한 graph 분석 결과를 얻지 못했다. 별도 인덱스나 graph는 만들지 않고 실제 소스에서 경계와 호출 관계를 확인했다.

### 남은 검증 경계

- Chrome/Edge/Firefox, 실제 `file://` 정책, clipboard 권한, 접근성, 작은 화면 레이아웃은 실행하지 않았다. jsdom 결과는 이를 대신하지 않는다.
- Linux/macOS 및 원격 Actions/Pages 실행 결과를 이번 감사에서 확인하지 않았다.
- category의 실제 HTTPS 서버 요청, 실제 설치·게시·배포를 실행하지 않았다. fetch guard와 제어된 resolver 실패 테스트는 검토했다.
- 외부 의존성·vendored 구현 자체의 독립 감사는 제외했다. 연결·고정 버전·자산 출력 경로를 검토했다. 기록된 UI 출처는 rustdoc 1.97.1 (`8bab26f4f68e0e26f0bb7960be334d5b520ea452`), Settings 탭은 1.102.0-nightly (`f5cedb83a`, 2026-11-07)의 한정 이식이다. UI나 자산은 변경하지 않았다.
- 과거 계획·검증 로그의 모든 역사적 사실을 재검증하지 않았다. ignored `DETAILS.md`는 SHA256 `A3B0A52FF1FCC48F3131ACC2B1E9A967D3757FFA35DEA94F7600883E52A1D5D8`의 배경 문서로만 참고했다. README/ROADMAP의 일부 과거 단계 표현은 현재 기능 설명과 섞여 있지만, 이를 별도 실행 결함으로 집계하지 않았다.

현재 미검토 실행 코드 단위와 미판정 결함 후보는 없다. **Complete는 정한 검수 단위를 평가했다는 뜻이며, 추가 버그가 없다는 보증은 아니다.**
