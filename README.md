# Clippy HTML

Clippy의 구조화된 진단을 rustdoc 스타일의 정적 HTML 문서로 탐색하는 Cargo 서브커맨드입니다.

**기능 코어와 일부 후속 인터페이스가 구현되어 있습니다.** 실제 Clippy 진단 수집, 중복 병합, 홈·category·진단 종류·전체 소스 페이지, 파일 트리, 검색, Settings·테마 선택, 구문 색칠, 정밀 primary/secondary 범위, 줄 anchor, child 진단과 suggestion replacement 표시를 제공합니다. suggestion diff/copy, metadata export는 남아 있습니다. 현재 상태·완료 기준·검증 과제는 [로드맵](ROADMAP.md)을 기준으로 봅니다.

## 설치와 실행

Rust와 Clippy가 필요합니다. 이 저장소에서 설치합니다.

```powershell
rustup component add clippy
cargo install --path . --locked
```

검사할 Cargo 프로젝트에서 실행합니다.

```powershell
cargo clippy-html
cargo clippy-html --open
cargo clippy-html --output report
cargo clippy-html --offline
cargo clippy-html --offline -- --offline
cargo clippy-html -- --all-targets --all-features
cargo clippy-html -- --workspace --all-targets -- -W clippy::pedantic
```

기본 결과물은 Cargo target directory 아래의 `clippy-html/index.html`입니다. `--output`을 지정하면 그 경로를 사용합니다. 상대경로는 명령을 실행한 폴더 기준입니다. 전달한 `--target-dir`도 기본 보고서 위치에 반영됩니다.

첫 `--` 이후는 Cargo Clippy에 전달합니다. 그 안의 두 번째 `--` 이후는 rustc/Clippy 인자입니다. `--all-targets`와 `--all-features`는 기본 강제하지 않습니다. 출력 형식을 변경하는 `--message-format`과 소스를 수정하는 `--fix`는 Cargo 인자 영역에서 거부합니다. Phase 1의 `--metadata`는 미지원 옵션 오류이며 공개 metadata JSON을 자동 생성하지 않습니다.

Category는 기본적으로 진단의 공식 문서 링크에 해당하는 Clippy 버전의 분류표를 온라인으로 한 번 조회합니다. 최대 10초 안에 조회하지 못하거나 형식 검증에 실패하면 동봉 표(rust-1.97.0, 활성 lint 815개)를 이름으로 적용합니다. `cargo clippy-html --offline`은 이 조회만 생략하며 Cargo의 네트워크 사용을 막지는 않습니다. `cargo clippy-html -- --offline`은 Cargo에만 전달됩니다. 양쪽 모두 offline으로 쓰려면 위 예시처럼 두 위치에 지정합니다. Clippy 진단이 없으면 분류표를 조회하지 않습니다.

## 보고서 읽기

생성한 `index.html`을 브라우저에서 직접 엽니다. 서버나 인터넷 연결, JavaScript가 없어도 기본 탐색이 가능합니다.

Search 버튼 또는 `S`·`/`로 rustdoc처럼 본문 자리에 검색 화면을 엽니다. lint 이름·파일 경로·진단 메시지(하위 note/help 포함)·category를 검색하며, In Lints / In Files / In Findings / In Categories 탭으로 결과를 나눕니다. 여러 검색어는 모두 포함되어야 하고 대소문자를 구별하지 않습니다. 폼의 Enter는 즉시 검색하고, 아래 화살표는 결과로 이동하며, 결과의 좌우 화살표는 탭을 바꿉니다. Escape나 Exit로 원래 문서에 돌아옵니다. 검색과 Settings에는 JavaScript가 필요하며, 모든 데이터와 스크립트는 보고서에 동봉합니다. 각 탭은 최대 100개를 표시하고 전체 일치 수와 제한 여부를 알립니다.

Settings는 고정한 rustdoc의 팝오버와 별도 `settings.html` 구조를 사용합니다. `light`·`dark`·`ayu`·`system preference`를 고르고, 시스템 밝기별 선호 테마도 지정할 수 있습니다. 탐색창/목차 숨김, sans-serif 본문, 소스 줄바꿈, 단축키 비활성화, 검색 단일 결과로 바로 이동을 제공합니다. 테마는 스타일 적용 전에 복원하며 저장소가 차단되어도 현재 페이지의 설정 변경은 동작합니다. `file://`에서 페이지 사이의 저장 공유는 브라우저에 따라 다릅니다. 이식 항목과 제외 이유는 [Settings 출처 기록](assets/vendor/rustdoc/SETTINGS.md)에 있습니다.

- 홈에는 검사 종료 상태, 수집 완전성, 원본 진단 수와 병합 후 finding 수를 별도로 표시합니다.
- 진단 종류 페이지에는 code, 실제 severity, target, primary/secondary 위치, note/help, suggestion 묶음과 applicability를 표시합니다.
- 진단 URL은 `lint/clippy/clone_on_copy.html`, `lint/rustc/E0308.html`처럼 출처와 코드를 사용합니다. 코드 없는 진단은 `lint/general.html`에 모읍니다. 기존 해시 URL은 보고서를 재생성하면 변경됩니다.
- 소스 페이지에는 관련 workspace 파일의 전체 코드, 줄 번호, 줄 단위 진단 표시, finding 링크를 표시합니다.
- 전체 소스의 정확한 진단 범위는 기존의 옅은 줄 배경 위에 primary는 진한 배경·secondary는 조금 옅은 배경으로 표시합니다. 삽입 위치는 caret으로 표시하고, 범위의 tooltip에 lint와 위치 설명을 보존합니다. 기존 구문 색칠은 유지하며 소스가 진단 좌표와 맞지 않으면 정밀 표시 없이 원본 snippet을 제공합니다.
- 코드 범위에 포함된 줄 앞 스페이스·탭은 진한 배경에서 제외합니다. 공백만 지목한 진단과 문자열·주석 내부의 공백은 강조를 유지하며, 원래 좌표와 소스 텍스트는 바꾸지 않습니다.
- 소스 화면은 생성 시 Rust 토큰을 분류하여 rustdoc CSS로 키워드·문자열·주석·숫자·lifetime·매크로 등을 색칠합니다. 여러 줄 문자열과 중첩 주석의 문맥을 보존하며 JavaScript는 필요하지 않습니다. 타입 의미 분석이나 정의 이동은 제공하지 않습니다. [구현 출처와 범위](assets/vendor/rustdoc/HIGHLIGHTING.md)
- 소스 HTML은 `src/diagnostic/normalize.rs.html`처럼 원래 경로를 알아볼 수 있는 이름을 사용합니다. 위험한 파일명 문자는 인코딩하고 충돌 시 식별자를 붙입니다. 소스 사이드바의 Files 트리에는 진단에 포함된 파일과 진단 수를 표시하며, 현재 파일의 경로를 펼칩니다. Findings in this file 목록에서 해당 줄로 이동할 수 있습니다.
- 홈에서 category별 finding 수와 lint 종류 수를 확인하고 category → lint → 소스로 이동할 수 있습니다. 분류 출처·기준 버전·fallback 사유는 홈의 Category data에서 확인합니다. 동봉 표는 실행한 Clippy와 버전이 달라도 이름으로 적용하므로 분류가 다를 수 있습니다. 선택한 표에 없는 이름은 Unknown으로 남기고, rustc/general 진단은 Clippy category에 포함하지 않습니다. Category 정보는 severity나 finding ID를 바꾸지 않습니다.
- 동일한 위치라도 package, severity, 관련 span 또는 suggestion이 다르면 별 진단입니다. 같은 진단이 여러 target에서 발생하면 target 목록을 합칩니다.

소스의 실제 내용과 진단 위치가 맞지 않거나 파일을 읽을 수 없으면 진단 snippet과 사유를 표시합니다. 외부 dependency와 생성 코드의 전체 파일은 기본 복사하지 않습니다. 자동 수정은 하지 않습니다.

보고서에는 **원본 소스코드와 로컬 경로가 포함될 수 있습니다**. artifact나 Pages에 게시할 때 공개 범위를 확인해야 합니다.

## 실패와 기존 결과물

Clippy가 경고를 오류로 처리하거나 컴파일 오류로 실패해도 가능한 보고서를 생성하고 비영 종료 코드를 유지합니다. 보고서 생성 성공을 검사 성공으로 표시하지 않습니다. JSON 손상이나 완료 메시지 누락은 불완전 수집으로 표시합니다. Cargo 자체 시작이나 metadata 조회에 실패하면 기존 보고서를 유지합니다.

출력은 임시 폴더에 완성·검증한 뒤 교체합니다. 기존 폴더는 `.clippy-html-owned` marker가 있는 전용 보고서일 때만 교체합니다. 일반 폴더나 프로젝트/target 루트를 덮어쓰지 않습니다. 동시 출력 writer는 lock으로 구별합니다.

중단 후 lock이나 backup이 남으면 도구가 알려 주는 경로를 확인합니다. 실행 중인 생성기가 없는지 확인하고, backup의 정상 결과물을 보존한 뒤 복구합니다. 도구는 불확실한 backup이나 stale lock을 임의 삭제하지 않습니다. 여러 rename 사이의 전원 장애까지 원자적으로 복구하는 기능은 없습니다.

## 개발 검증

```powershell
cargo build --example cargo_probe --locked
cargo test --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
```

process 테스트는 먼저 빌드한 `cargo_probe` helper를 사용해 양쪽 pipe의 대량 출력을 검증합니다. 통합 테스트는 fixture를 임시 폴더에 복사하므로 저장소의 fixture 소스를 수정하지 않습니다. fixture 자체는 외부 의존성을 사용하지 않습니다.

검색 엔진은 Node의 기본 테스트 runner로 검증합니다. DOM 동작 검증에만 테스트용 jsdom을 사용하며, 제품 생성·실행에는 Node/npm이 필요하지 않습니다.

```powershell
node --test tests/search-engine.test.cjs
npm install --prefix target/search-qa --no-save --ignore-scripts jsdom@29.1.1
$env:NODE_PATH = (Resolve-Path target/search-qa/node_modules).Path
node --test tests/search-engine.test.cjs tests/search-ui.test.cjs tests/settings.test.cjs
```

DOM 테스트는 실제 브라우저의 배치나 로컬 파일 정책 검증을 대신하지 않습니다. 검색 UI의 고정 rustdoc 대조와 남은 수동 확인은 [검색 검증 기록](docs/validation/2026-11-07-search.md)에 기록합니다.

Rust 1.94에서 locked compile check를 확인했습니다. 현재 실행 환경과 검증 범위는 [검증 기록](docs/validation/2026-10-27-phase-1.md)에 남깁니다.

## 구조와 라이선스

CLI → Cargo 수집 → 진단 정규화/병합 → 소스 로드 → HTML 생성 → 출력 교체 순서입니다. parser와 renderer는 독립적으로 테스트할 수 있습니다. 파일별 책임은 [설계 문서](docs/superpowers/specs/2026-10-27-clippy-html-design.md)에 있습니다.

프로젝트 코드와 Cargo 라이선스 표기는 MIT 또는 Apache-2.0을 유지합니다. rustdoc CSS는 이 프로젝트의 비교용 문서를 생성한 Rust 1.97.1, 커밋 `8bab26f4f68e0e26f0bb7960be334d5b520ea452`을 기준으로 MIT 조건으로 재사용했습니다. normalize.css의 별도 MIT 라이선스와 rustdoc 저작권 고지도 포함합니다.

동봉 Clippy category 표는 공식 문서에서 이름과 분류만 추출했습니다. MIT 원본 고지와 출처·변경·재생성 방법은 [분류 데이터 안내](assets/vendor/clippy/README.md)에 있습니다. 분류표 갱신 도구는 로컬 HTML 입력으로 실행하며, 런타임 조회 실패가 Clippy의 성공/실패나 진단 수집 완전성을 바꾸지는 않습니다.

기본 폰트는 Fira Sans(UI), Source Serif 4(본문), Source Code Pro(코드), NanumBarunGothic(한글 보완)입니다. rustdoc의 WOFF2 12개를 수정 없이 동봉하며 폰트에는 각각 OFL-1.1이 적용됩니다. 원본 저작권·예약 이름·라이선스 전문을 함께 배포합니다. [실제 동봉 자산과 고지](THIRD-PARTY-NOTICES.md), [폰트 출처와 해시](assets/vendor/rustdoc/FONTS.md)를 확인할 수 있습니다. 생성 보고서에서도 footer의 라이선스 안내를 통해 같은 고지에 접근할 수 있습니다. 이 조건이 검사 대상 소스의 라이선스를 바꾸지는 않습니다.

폰트와 고지는 실행 파일에 내장되어 보고서에 출력되므로 실행 시 rustdoc 설치나 폰트 다운로드가 필요하지 않습니다. 실행 파일을 배포할 때는 THIRD-PARTY-NOTICES.md에 명시된 라이선스 파일과 출처 고지를 배포 묶음에도 포함합니다. 원본 rustdoc의 local() 우선순위와 font-display: swap을 유지하며, Source Code Pro는 원본처럼 동봉 파일을 직접 사용합니다.

출처와 변경은 [UPSTREAM.md](assets/vendor/rustdoc/UPSTREAM.md)에 기록합니다. 필요한 고지는 생성 보고서의 `licenses/`에도 함께 배포됩니다. rustdoc의 버전 간 DOM/API 호환성은 보장하지 않습니다.

보고서의 favicon은 Rust 기어 안에 클립을 넣은 별도 수정 자산입니다. SVG와 16·32·48px ICO를 동봉하며, 원본 Rust 로고의 CC BY 4.0 출처·변경 고지는 [favicon 안내](assets/favicon/README.md)에 기록합니다. Clippy HTML은 Rust Project/Rust Foundation의 공식 제품이 아닙니다.

## rustdoc 스타일 비교

외부 예제 대신 이 프로젝트의 문서를 비교 기준으로 사용합니다.

```powershell
cargo doc --no-deps --document-private-items
cargo run --locked -- --output target/rustdoc-style-preview -- --all-targets -- -W clippy::pedantic
```

`target/doc/cargo_clippy_html/index.html`과 `target/rustdoc-style-preview/index.html`을 나란히 엽니다. 비교용 Clippy 실행에만 pedantic을 전달하여 lint·소스 화면을 살펴볼 진단을 확보합니다. 도구의 기본 lint 설정을 바꾸지는 않습니다. 최신 요소별 대조와 유지한 차이는 [2026-10-30 비교 기록](docs/validation/2026-10-30-rustdoc-audit.md)에 있습니다.
