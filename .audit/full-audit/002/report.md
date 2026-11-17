# Full Audit — clippy-html — FA-002

- Status: **Complete**
- 기준: `main`, `d8f1e73c7db57ee0ca687848d40f255f238afbe3`. 검수 중 추적 소스 변경 없음.
- 범위: 현재 자체 Rust·JS/CSS·템플릿·테스트/fixture·example·빌드/CI·현재 공개 계약 전체. 특히 `3a8fd2e`와 `d8f1e73`의 FA-001 수정 및 주변 경계를 재검토했다.
- 제외: 생성물, cache, `node_modules/`, `.git/`, `.audit/`, `.superpowers/`, 외부 의존성과 vendored 구현 자체. 자체 코드의 자산 연결과 출처 기록은 포함했다. 과거 계획/검증 문서는 배경 자료로 사용했다.

## Summary

**확정 3건: Important 2건, Minor 1건. Critical 발견 없음.** 별도로 의심 2건을 검증 후 기각했다. 미판정 후보와 미검토 실행 코드 단위는 없다.

기존 두 재현은 Windows에서 해소됐다. 다만 경로 수정이 Windows에만 적용되어 대소문자를 구분하지 않는 비-Windows 파일시스템에는 같은 실패가 남는다. 추가로 첫 줄의 BOM 처리와 Cargo의 별도 build directory 제외 누락을 확인했다.

수정으로 새로 유입됐다고 확정한 회귀는 없다. FA-002-001은 기존 문제의 남은 플랫폼 범위이고, FA-002-002/003은 1차 검수에서 놓친 기존 결함이다. 이전 기록을 현재 판정으로 덮어쓰지 않았다.

애플리케이션·기존 테스트·manifest·Git 이력은 변경하지 않았다. 기록과 재현 스크립트는 `.audit/full-audit/002/`에만 추가했다. 원본 판정은 `state.yaml`이다.

## Findings

### FA-002-001 — 비-Windows의 대소문자 비구분 파일시스템에 내부 metadata 경로 실패 잔존

- Severity / Confidence: **Important (Should Fix) / Confirmed**
- 위치: `src/metadata/output.rs:36-82`, `src/output.rs:134-138`.
- 관계: `FA-001-003`과 관련된 잔여 플랫폼 범위. 수정으로 새로 발생한 문제라는 뜻은 아니다.

**조건:** 비-Windows 호스트에서 대소문자를 구분하지 않는 파일시스템을 사용하며, 아직 없는 출력 폴더에 다음처럼 동일한 물리적 경로의 표기를 다르게 지정한다.

```text
cargo clippy-html --offline --output Report --metadata report/navigation.json -- --offline
```

내부 JSON과 HTML이 함께 staging되어야 한다. 현재는 metadata를 외부 파일로 오인하고 출력 폴더를 먼저 만든다. HTML publication은 경로 변경 또는 소유권 marker 부재로 실패하며, 남은 폴더 때문에 표기를 맞춘 재시도도 거부된다. 기존 데이터 손실은 주장하지 않는다.

**정적 증명:**

1. `report_relative_path`의 일반 경로는 대소문자를 구분하는 `Path::strip_prefix`다.
2. 두 이름 모두 아직 없는 상태에서는 `resolve_path`가 기존 조상만 canonicalize하므로 `Report`와 `report`가 남는다.
3. 새 alias 검사 전체가 `#[cfg(windows)]` 안에 있어, 비-Windows는 `Ok(None)`을 반환한다.
4. `External` staging이 `report/`를 만든다. 조건의 파일시스템에서는 이것이 실제 출력 폴더다.
5. `publish_report`는 canonical 표기가 달라졌으면 거부하고, 같더라도 `.clippy-html-owned`가 없으므로 거부한다. 어느 경우에도 새 보고서는 게시되지 않는다.

**실행 검증:** `python .audit/full-audit/002/probe-platform-branch.py`가 현재 함수에서 비-Windows 분기만 추출한 임시 모듈과 변경 없는 실제 `output.rs`를 사용했다. 사용 가능한 대소문자 비구분 저장소에서 다음을 관찰했다.

```text
external=true
first_error=Output path changed during generation
report_dir_left=true
index_exists=false
retry_error=Refusing to overwrite non-report directory
```

실패는 렌더링 전 발생했고, 하네스의 렌더링/검증 stub은 호출되지 않았다. **이는 정적 증명과 분기 격리 실험이며, 실제 macOS/Linux 실행 결과가 아니다.** 플랫폼 한계는 유지한다. 원래 Windows 재현과 명시적 case-sensitive Windows 대조군은 정상 동작했다.

### FA-002-003 — 별도 Cargo build directory의 생성 코드를 전체 소스로 복사

- Severity / Confidence: **Important (Should Fix) / Confirmed**
- 위치: `src/cargo.rs:43-53`, `src/diagnostic/model.rs:74-78`, `src/source.rs:33,56-60`.
- 조건: workspace 내부에서 `build.build-dir`을 `target_directory`와 다른 폴더로 설정하고, `OUT_DIR` 생성 Rust 파일에서 진단이 발생한다.

```toml
# .cargo/config.toml
[build]
build-dir = "generated-cache"
```

`README.md:58`과 초기 설계 `docs/superpowers/specs/2026-10-27-clippy-html-design.md:111`은 생성 코드 전체를 기본 복사하지 않고 snippet으로 대체한다고 명시한다. 실제로는 생성 파일의 전체 내용과 suggestion patch가 보고서에 포함된다. 따라서 보고서를 공유하면 의도한 진단 excerpt 밖의 생성 코드도 전달될 수 있다. 실제 비밀정보나 침해 사례를 가정하지 않는다.

**원인:** `load_project`와 `ProjectInfo`가 target directory만 보존하며, `load_sources`도 그것만 제외한다. 별도 build dir가 member root 아래이고 최근접 manifest가 해당 member이면 전체 소스로 허용된다. 현재 의존성 `cargo_metadata 0.23.1`에는 이미 선택적 `build_directory` 필드가 있지만 수집 과정에서 버린다.

**검증:** `probe-boundaries.py`가 외부 의존성 없는 두 임시 프로젝트에서 같은 `build.rs`로 `OUT_DIR/generated.rs`를 만들었다. 1행에는 `FULL_GENERATED_SOURCE_SENTINEL`, 3행에는 `clone_on_copy` 진단을 배치했다.

| 조건 | Cargo metadata | 보고서 |
|---|---|---|
| 기본 build dir | target=`target`, build=`target` | finding fallback anchor, diff 없음, 진단 밖 sentinel 없음 |
| 별도 build dir | target=`target`, build=`generated-cache` | 검증된 `#L3` 링크, diff 있음, 전체 소스에 1행 sentinel 포함 |

두 실행 모두 종료 0이었다. 별도 설정의 생성 소스 페이지는 `src/generated-cache/debug/build/.../out/generated.rs.html`이었다. 경로만 달라졌을 때 기본 제외 경계가 사라지는 것을 실제 생성물로 확인했다.

### FA-002-002 — BOM이 있는 파일의 첫 줄에서 정밀 범위와 정상 suggestion diff 누락

- Severity / Confidence: **Minor (Nice to Have) / Confirmed**
- 위치: `src/source.rs:84-110`, `src/render/suggestions.rs:107-111`.
- 관계: `FA-001-002`의 3행 대조군이 다루지 않은 첫 줄 조건이다. 이전 실험 결과를 부정하지 않는다.

다음 유효한 한 줄 Rust 파일 앞에 UTF-8 BOM `EF BB BF`를 붙이면 발생한다.

```rust
pub fn demo() { let x = 1_u32; let _ = x.clone(); }
```

파일은 Clippy 실행과 렌더링 사이에 바뀌지 않았다. 그런데 정상 진단이 소스 불일치로 처리돼 metadata는 `#L1` 대신 `#finding-...`를 내보내고, 정밀 범위가 사라지며 suggestion은 `Diff unavailable`로 대체된다. 원본 snippet과 진단은 남고 도구 종료는 0이다.

**원인과 검증:** `probe-boundaries.py`가 실제 compiler JSON과 생성 HTML을 함께 확인했다.

| 입력 | compiler byte 범위 | compiler column 범위 | 보고서 |
|---|---|---|---|
| LF | `39..48` | `40..49` | `#L1`, 정밀 범위·diff 있음 |
| CRLF | `39..48` | `40..49` | `#L1`, 정밀 범위·diff 있음 |
| BOM+LF | `42..51` | `40..49` | fallback anchor, 정밀 범위 0개·diff 없음 |

compiler snippet도 BOM을 제외한다. `span_matches`는 BOM을 포함한 첫 줄과 snippet을 그대로 비교하고, 열→byte 변환에서도 첫 줄 BOM을 보정하지 않는다. 따라서 실제로 같은 원문인데 검증을 통과하지 못한다. 1차 검수에서 사용한 BOM 파일의 3행 진단은 이번에도 정상 동작했다.

## Disproven leads

- **FA-002-004 — member 전용 feature 전달:** root package와 member가 있는 임시 workspace에서 `-p member --features special`, `-p member -Fspecial`을 모두 실행했다. 둘 다 종료 0으로 metadata 선택 실패를 확인하지 못했다. Severity 없음.
- **FA-002-005 — Windows case-sensitive 경로의 잘못된 합병:** 임시 폴더에 case-sensitive 플래그를 실제 활성화했다. `Report/index.html`과 별도 `report/nav.json`이 정상 생성되고 종료 0이었다. 기본 볼륨 테스트에서 건너뛰는 조건을 실행했으며 의심한 회귀는 재현되지 않았다. Severity 없음.

## 이전 수정의 재검증

| 이전 Finding / 수정 | 이번 관찰 |
|---|---|
| FA-001-001 / `d8f1e73` | 실제 생성 `src/settings.html`: `panelInsideMain=false`, `popoverExists=true`, `menuHidden=false`. 원래 오류 해소. 현재 DOM 테스트의 HTTP/file 및 standalone query/fragment 사례도 통과. |
| FA-001-003 / `3a8fd2e` | 원래 Windows alias 첫 게시·matching-case 재시도·대조군 모두 종료 0, `index.html` 생성. 다중 누락 경로·Unicode·조상 alias 거부·probe 정리 테스트 통과. 비-Windows 잔여 조건은 FA-002-001로 기록. |

수정 diff 외의 현재 코드를 함께 검토했다. FA-002-002/003의 관련 수집·소스 코드에는 두 수정으로 인한 변경이 없다. Settings의 URL 판별 변경에서 재현된 새 결함은 없었다.

## Coverage and limitations

11개 단위 모두 실행 경로·계약·실패 조건을 검토했다. 상태 파일에 각 단위별 근거를 기록했다.

| 단위 | 이번 주요 확인 | 상태 |
|---|---|---|
| CLI·Cargo 수집 | 인자 경계, feature 대조군, pipe·완료·종료 상태, 최신 build-directory 수집 | Reviewed |
| 모델·정규화·dedup | 재귀 note/복합 suggestion, package·severity·span 구별, target 병합·ID 충돌 | Reviewed |
| category·fetch·example | 공식 URL/버전, body/time 제한, offline/실패 fallback, category/ID 독립성 | Reviewed |
| 소스·범위·highlight | canonical 소유 경계, 첫 줄·다중 행·BOM, target/build-dir 차이 | Reviewed |
| 경로·navigation·템플릿·자산 | 안전한 이름, 상대 URL·anchor, escaping, source tree/count, 자산/검증 경로 | Reviewed |
| suggestion·copy | 전체 snippet·좌표, 경로·overlap·크기/시간 제한, 역순 적용, clipboard fallback | Reviewed |
| 출력 게시 | marker, alias, lock 수명, staging/rollback, reparse, 실패 후 폴더 상태 | Reviewed |
| metadata | DTO·실제 목적지·fallback, 내부/외부 구분, Windows 수정과 비-Windows 분기 | Reviewed |
| Search·Help | matching/rank/count, 로컬 URL, history/IME/focus, toolbar·Escape 조정 | Reviewed |
| Settings·Summary·CSS | 수정된 root URL 판별, storage·테마, hash reveal·키 입력 보호, 기존 스타일 연결 | Reviewed |
| 테스트·빌드·CI·현재 문서 | fixture 격리·플랫폼 조기 반환, dependency 모델, MSRV·CI 실패 전파·공개 계약 | Reviewed |

### 실행한 검사

| 명령 | 결과 |
|---|---|
| `cargo +1.100.0 test --locked --offline` | **80 단위 + 15 통합 + 7 metadata = 102/102 통과** |
| `npm test` | **50/50 통과** |
| `cargo +1.100.0 build --locked --offline` | 현재 바이너리 빌드 통과 |
| `cargo +1.100.0 fmt --check` | 통과 |
| `cargo +1.100.0 clippy --all-targets --locked --offline -- -D warnings` | 통과 |
| `cargo +1.88.0 check --all-targets --locked --offline` | 통과 |
| Git Bash `tests/ci-dogfood.test.sh` | stub Cargo **4개 사례 통과**, 실제 설치/배포 없음 |
| `python .audit/full-audit/001/probe-projects.py` | 현재 바이너리에서 원래 두 결함 해소 확인; 기존 대조군 정상 |
| `python .audit/full-audit/002/probe-boundaries.py` | BOM/생성 build-dir 결함 재현, feature/case-sensitive 대조군 정상 |
| `python .audit/full-audit/002/probe-platform-branch.py` | 비-Windows alias 분기의 격리 실행으로 실패·재시도 거부 확인 |

알려진 sandbox의 Windows 경로/MSYS 제한 때문에 필요한 프로세스 검사는 승인된 sandbox 외부 실행으로 수행했다. 의존성 설치, 실제 외부 category 조회 또는 원격 변경은 하지 않았다. `zg --rg`와 실제 소스로 호출 경계를 확인했다. `codeknow debt`는 기존 graph가 없어 구조 분석 결과를 제공하지 못했으며, 별도 graph를 생성하지 않았다.

### 검증 한계

- 실제 macOS/Linux 호스트 실행은 없었다. FA-002-001의 확정 근거는 조건을 명시한 정적 증명과 분기 격리 실험이다.
- 실제 Chrome/Edge/Firefox, `file://` 로딩 정책, clipboard 권한, 접근성·반응형 배치는 실행하지 않았다. jsdom과 실제 생성 HTML 검증은 DOM 근거다.
- hosted CI/Pages, 실제 설치·배포 및 live HTTPS 요청은 실행하지 않았다.
- 외부 dependency/vendored 내부 구현의 독립 감사와 과거 기록의 모든 역사적 사실 검증은 제외했다. UI 출처는 rustdoc 1.97.1 (`8bab26f4f68e0e26f0bb7960be334d5b520ea452`), Settings 탭은 1.102.0-nightly (`f5cedb83a`, 2026-11-07)의 한정 이식이며 이번 감사에서 UI는 변경하지 않았다.

**Complete는 정한 범위의 검토·판정을 마쳤다는 뜻이며, 추가 버그가 없다는 보증은 아니다.**
