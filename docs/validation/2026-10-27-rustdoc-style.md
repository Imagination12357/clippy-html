# 이 프로젝트의 rustdoc을 기준으로 한 스타일 정렬

날짜: 2026-10-27. 기준: rustdoc 1.97.1 (8bab26f4f68e0e26f0bb7960be334d5b520ea452).

## 비교 입력

- `cargo doc --no-deps --document-private-items`
- 기준 홈: target/doc/cargo_clippy_html/index.html
- 기준 항목: target/doc/cargo_clippy_html/render/pages/struct.FindingView.html
- 기준 소스: target/doc/src/cargo_clippy_html/main.rs.html
- 실제 보고서: `cargo run --locked -- --output target/rustdoc-style-preview -- --all-targets -- -W clippy::pedantic`
- 이전 fixture 미리보기도 동일한 새 스타일로 갱신한다.

## 정렬한 부분

| 요소 | 기준 문서와 보고서의 대응 |
| --- | --- |
| 본문 | main > .width-limiter > section#main-content.content. 소스는 기준 문서처럼 width-limiter를 생략 |
| 제목 | .main-heading의 원래 grid, h1·breadcrumb 간격. 임의 상단 테두리·여백·반복 브랜드 문구 제거 |
| 사이드바 | .sidebar-crate h2와 내부 .version, .sidebar-elems, .block. 자체 padding과 li 간격 덮어쓰기 제거 |
| 개요 목록 | rustdoc의 dl.item-table dt/dd 사용. 임의 HTML table 테두리와 별도 grid 제거 |
| 소스 | .rustdoc.src, .example-wrap.digits-N, pre.rust, data-nosnippet 줄 번호 사용 |
| 진단 코드 | .example-wrap.hide-lines와 pre.rust 사용. 진단 위치의 임의 좌측 테두리 제거 |
| 고지 | 탐색 sidebar에서 본문 하단 footer로 이동. 원본 저작권 고지와 MIT 전문은 보고서에도 포함 |

기존 Rust 1.90.0 CSS 대신 위 기준 문서와 동일한 1.97.1의 생성 자산을 고정했다. 원본 CSS의 SHA256와 변경은 assets/vendor/rustdoc/UPSTREAM.md에 기록했다. 자산은 바이너리에 포함되며 리포트 생성 시 cargo doc를 실행하지 않는다.

## 의도적으로 남긴 차이

- 최초 비교 시에는 폰트를 제외했지만 2026-10-28 승인된 폰트 도입으로 기본 12개 WOFF2와 원본 @font-face를 동봉한다. 현재는 본문·UI·코드의 원본 폰트 스택을 복원했다. 실제 렌더링 확인은 docs/validation/2026-10-28-rustdoc-fonts.md의 수동 항목으로 남긴다.
- 진단 severity, 검사 상태, suggestion, 위치 강조는 보고서 고유 정보이다.
- 소스 sidebar는 JavaScript 없이도 관련 finding에 접근하도록 펼친 상태를 사용한다. rustdoc의 기본 접힌 상태와 비교하면 폭이 다르다.
- 작은 화면은 JS 메뉴 대신 기본 링크를 이용할 수 있는 정적 sidebar를 유지한다.
- 검색·설정·테마 등의 작동하지 않는 rustdoc 버튼은 복제하지 않는다. 구문 강조와 정밀 span 강조는 Phase 2 범위이다.

## 검증

- 생성된 기준 문서와 보고서에서 본문 래퍼 및 dl.item-table 구조를 대조했다.
- 실제 자체 검사 보고서의 소스 페이지 6개에서 source 레이아웃과 줄 번호 구조를 확인했다.
- 최초 비교 시에는 @font-face 및 폰트 URL을 제외했다. 이후 폰트 도입에서는 별도 fonts.css의 원본 선언, 바이너리 해시와 라이선스 고지를 검증한다.
- 기존 전체 테스트 40개가 구조 변경 후 통과했다. 생성한 내부 href/anchor는 기존 validator로 확인한다.
- 앱 브라우저의 file:// 정책 제한은 계속 적용된다. 실제 브라우저에서 나란히 렌더링한 화면의 픽셀 비교를 수행했다고 주장하지 않는다. 최종 시각 차이는 사용자 화면 검토로 확인한다.
