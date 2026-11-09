# Clippy HTML 설계안

작성일: 2026-10-27
상태: 2026-10-27 사용자 승인 후 기능 코어 및 여러 후속 작업 구현. 2026-10-30 재평가한 현재 상태와 작업 순서는 [ROADMAP.md](../../../ROADMAP.md)를 기준으로 한다. 이 문서는 설계 계약과 당시 결정의 기록이다.
기준 명세: 저장소 루트의 `DETAILS.md`.

## 1. 목적과 합의

Rust 개발자가 실제 Clippy 진단을 rustdoc과 익숙한 외형의 정적 HTML에서 탐색하도록 한다. 단일 Cargo package, Rust 2024 edition을 유지한다. 원본 소스와 Cargo 설정은 수정하지 않는다. Cargo 실행 자체의 일반적인 빌드 산출물과 lockfile 생성 가능성까지 차단하는 도구는 아니다.

대화에서 합의한 사항:

- A1: 실제 Clippy 결과를 사용하는 최소 보고서를 첫 검토 대상으로 삼는다.
- rustdoc CSS를 직접 재사용하며, 첫 보고서부터 해당 스타일을 적용한다.
- 2026-10-28 폰트 도입 결정: 고정한 rustdoc의 기본 WOFF2 12개와 원본 OFL-1.1 고지를 동봉한다. 자체 코드와 Cargo의 MIT OR Apache-2.0 표기는 유지한다.
- 기본 결과물은 서버 없는 정적 멀티페이지 HTML이다. 진단마다 별도 페이지는 만들지 않는다.
- 기능 코어 → 탐색과 진단 이해 → 배포라는 큰 순서를 유지한다. 인터페이스 일부와 폰트 도입은 앞당겨 구현했고, 현재는 공식 category → 검색을 우선한다. 기본 플랫폼 CI는 배포 직전까지 미루지 않는다. 자세한 순서와 완료 기준은 ROADMAP.md에 기록한다.

DETAILS.md의 1절, 6절, 19절을 승인된 방향으로 동기화했다: rustdoc 전체를 fork하지 않으며, 특정 버전의 CSS와 그에 필요한 HTML 구조를 재사용한다. upstream 버전 간 호환성은 보장하지 않는다. 명세의 나머지 제품 목표는 유지한다.

비목표: 자동 수정, lint 신설, 점수, SARIF, PR bot, 서비스 서버, rustdoc 검색 엔진 및 API 호환 구현.

## 2. 검토한 접근

1. 실제 보고서 우선 + 스타일 독립 구현: 데이터 문제를 먼저 확인하지만 외형을 재현하는 작업이 크다. 대화에서 CSS 재사용으로 구체화했다.
2. 예시 데이터 기반 화면 시안 우선: 외형 합의가 빠르지만 실제 진단 구조와 다시 맞춰야 한다. 이번에는 선택하지 않았다.
3. 선택한 접근: 실제 보고서 우선 + 버전 고정 rustdoc CSS. HTML 템플릿은 필요한 class와 배치를 따르고, Clippy 고유 UI만 별도 스타일로 보완한다.

## 3. 데이터 흐름과 경계

```text
CLI 설정
  -> Cargo 프로젝트 정보 조회
  -> Clippy 실행과 stdout/stderr 동시 소비
  -> JSON 파싱
  -> 진단 정규화 / 중복 병합 / ID 부여
  -> 필요한 소스 로드
  -> 페이지 모델과 상대 링크 구성
  -> 임시 디렉터리에 HTML 및 자산 생성
  -> 완성된 보고서로 갱신
  -> 요청한 경우 브라우저 열기
  -> 실행 결과에 맞는 종료 상태
```

Renderer는 Cargo를 실행하지 않는다. 정규화기는 HTML을 만들지 않는다. source 로더는 진단을 새로 판단하지 않는다. main은 순서와 오류 전달만 관리한다. 범용 플러그인 구조나 비동기 런타임은 도입하지 않는다.

## 4. 제안하는 파일별 책임

아래는 설계자의 제안이며 아직 생성할 제품 파일 목록이다. 후속 Phase 파일은 필요할 때 추가한다.

| 파일 | 책임 / 입력 -> 출력 |
| --- | --- |
| src/main.rs | CLI에서 보고서 생성까지 조정, 최종 종료 상태 |
| src/cli.rs | OS 인자 -> 자체 옵션과 그대로 전달할 Clippy 인자 |
| src/cargo.rs | 설정 -> 프로젝트 정보, 구조화 메시지, 실행 결과 |
| src/diagnostic/mod.rs | diagnostic 모듈 경계 |
| src/diagnostic/model.rs | 독립적인 보고서, finding, span, suggestion 타입 |
| src/diagnostic/parse.rs | stdout 한 줄 -> 알려진 메시지 / 확장 메시지 / 텍스트 / 손상 |
| src/diagnostic/normalize.rs | Cargo 진단 -> 내부 finding 및 child/suggestion 구조 |
| src/diagnostic/dedup.rs | 정규화 진단 -> 병합한 finding, 재현 가능한 ID |
| src/source.rs | 진단의 파일 참조 -> 읽은 소스 또는 명시적인 fallback |
| src/render/mod.rs | 보고서 모델 -> 완성된 임시 보고서 |
| src/render/pages.rs | 페이지 모델 -> 공통 레이아웃, 홈, lint, source HTML |
| src/render/navigation.rs | 소스별 finding 수 집계 -> 현재 경로가 펼쳐진 파일 트리 |
| src/render/paths.rs | 내부 식별자 -> 충돌 없는 출력 경로와 상대 링크 |
| src/render/highlight.rs | 전체 Rust 소스 -> 원본 색상 클래스가 붙은 행별 텍스트 fragment |
| src/output.rs | 임시 출력 -> 검증된 디렉터리 갱신 및 복구 |
| templates/ | 공통 레이아웃과 페이지별 템플릿 |
| assets/vendor/rustdoc/ | 고정한 upstream CSS와 재사용 자산, 출처 및 라이선스 |
| assets/clippy-html.css | 진단 관련 추가 스타일과 공통 사이드바 여백 |
| src/render/assets.rs | 정적 텍스트·바이너리 자산 출력과 CSS 폰트 참조 검증 |
| tests/fixtures/ | 저장된 JSON과 작은 Rust fixture 프로젝트 |
| tests/integration.rs | 실제 바이너리와 fixture를 통한 통합 검증 |

pages.rs가 커지면 홈/lint/source별로 분리한다. 처음부터 내용 없는 파일은 만들지 않는다.

기술 방향: std::process::Command, std::thread, cargo_metadata, serde/serde_json을 사용한다. CLI parser, HTML 자동 escaping 템플릿, 고정 해시, 임시 디렉터리, 브라우저 열기는 목적에 맞는 소형 라이브러리를 선택한다. 정확한 버전과 MSRV는 구현 계획에서 공식 문서와 의존성 요구사항을 확인해 고정한다. 프런트엔드 빌드 도구는 필요하지 않다.

## 5. 실행 및 CLI 계약

- 직접 실행과 Cargo 서브커맨드 호출을 모두 지원한다. Cargo가 넣은 첫 clippy-html 인자만 제거한다.
- 첫 -- 이후를 Clippy에 전달하고, 그 안의 rustc 구분자 --는 보존한다. Unicode/공백 경로를 위해 원래 OS 인자를 보존한다.
- Cargo 쪽 인자 영역의 --message-format은 두 표기(--message-format=x, --message-format x) 모두 거부한다. rustc 인자 영역은 임의 재해석하지 않는다.
- --fix는 읽기 전용 도구의 목적과 충돌하므로 Cargo 인자 영역에서 거부한다. 소스 수정용 Clippy 실행 모드는 지원하지 않는다.
- CARGO 환경변수가 있으면 metadata와 clippy 모두 동일한 실행 파일을 사용한다. 셸 명령 문자열을 만들지 않는다.
- 실행 디렉터리는 호출자의 디렉터리로 유지한다. metadata 조회에는 --format-version 1과 --no-deps를 사용한다.
- --manifest-path, --config, --locked, --offline, --frozen 등 metadata와 공통으로 유효한 문맥 인자만 명시적으로 전달한다. package/target 선택은 Clippy에 맡기고 요청 정보와 실제 관측 정보를 구별한다.
- 기본 target 경로는 같은 실행 문맥의 metadata 결과를 따른다. 전달된 --target-dir이 있으면 두 표기를 인식해 우선 적용한다. --output이 최우선이며 상대 경로 기준은 최초 실행 디렉터리이다.
- package 목록은 workspace 구성원, 사용자가 요청한 선택, 메시지/산출물에서 관측한 검사 대상을 구분한다. 경고가 없는 package를 미검사로 단정하지 않는다.
- stdout은 줄 단위로 읽고 stderr는 별도 thread로 계속 소비하며 터미널에 전달한다. 보고서용 stderr 보관은 상한을 두고 생략 여부를 표시한다.
- Phase 1은 --output, --open, --help, --version을 지원한다. --metadata는 Phase 3에서 추가하며 그 전에는 명확한 미지원 옵션 오류이다. 기본 공개 metadata JSON은 어느 Phase에서도 생성하지 않는다.

## 6. 파싱, 모델, 진단 동일성

주요 모델:

- Report: 프로젝트 정보, 실행 인자, 실행 상태, 수집 상태, 원본 진단 수, findings, 수집 경고.
- Finding: ID, package 식별자, target 목록, 출처(Clippy/compiler/general), code, 실제 level, category, message, spans, child 트리, suggestions, rendered fallback.
- SourceSpan: 파일 참조, byte/line/column 범위, primary 여부, label, 진단에 담긴 snippet, expansion 정보.
- Suggestion: 설명, 원래 child 위치, 묶인 replacement 목록, 각 replacement의 applicability. 여러 span의 적용을 하나의 제안으로 유지한다.

정상 JSON의 알 수 없는 reason과 추가 필드는 수집 실패로 보지 않는다. JSON 객체처럼 시작하는 손상된 줄이나 알려진 메시지의 필수 구조 오류는 수집 불완전으로 기록한다. 일반 텍스트 출력은 별도 보관하되 진단으로 파싱하지 않는다. 읽기 오류가 발생해도 프로세스 및 나머지 스트림을 정리한다.

중복 key는 package 식별자, code, level, message, 정규화한 모든 관련 span, child 트리와 suggestion 내용이다. target은 key에서 제외하고 병합 후 정렬·중복 제거한다. rendered ANSI 문자열은 key에서 제외한다. child note/help는 상위 finding의 일부이며 독립 집계하지 않는다.

구분 가능한 필드 순서와 정렬 규칙이 고정된 직렬화로 fingerprint를 만든다. 동일 fingerprint의 서로 다른 canonical 내용은 전체 내용을 비교해 구별하고 canonical 내용 순서에 따른 suffix로 충돌을 해결한다. 출력 순서는 package, 표시 경로, 위치, code, level, canonical 내용 순으로 고정한다. OS 절대 루트가 다른 입력 사이의 동일 ID는 보장하지 않는다.

## 7. 소스와 경로

- workspace member가 소유하는 파일을 기본 전체 코드 포함 대상으로 한다. workspace 밖에 둔 member도 metadata의 manifest 경계로 식별한다.
- canonical 경로를 검사해 symlink가 허용 소스 경계 밖을 가리키면 전체 파일을 복사하지 않는다. 외부 dependency, 생성 코드, 가상 파일은 진단 snippet과 위치 텍스트로 대체한다.
- finding이 없는 파일은 복사하지 않는다. source 페이지는 파일마다 하나이며 여러 package/target의 관련 finding을 연결할 수 있다.
- 원본 경로를 출력 경로에 그대로 이어 붙이지 않는다. 2026-10-28 후속 UI 결정에 따라 source는 workspace 상대 경로를 안전한 파일명으로 인코딩해 src/<relative-path>.html에 배치하며, 선두 src/ 중복을 생략한다. 외부 경로는 별도 namespace로 격리하고 Windows 예약 이름과 대소문자 충돌을 처리한다. 원래 경로는 표시 텍스트로 보존한다. lint는 후속 A1 결정에 따라 lint/clippy/<name>.html 또는 lint/rustc/<code>.html을 사용하고, 코드 없는 진단은 lint/general.html에 둔다. lint와 source 모두 안전한 파일명 인코딩 및 결정적인 충돌 처리를 적용한다.
- 동일 입력의 경로 mapping을 정렬해 고정하고 충돌은 명시적으로 해결한다. 모든 내부 href는 생성한 mapping으로만 구성한다.
- 한 source 페이지에 L<줄번호>와 finding-<ID> anchor를 유일하게 만든다. 여러 finding이 같은 줄을 가리키면 각 finding anchor를 따로 배치한다. 한 finding의 복수 span은 동일 anchor를 중복 생성하지 않는다.
- UTF-8 byte 경계, CRLF, 다중 줄 span, 파일 끝 위치를 검증한다. 현재 파일과 진단 snippet이 맞지 않거나 위치가 유효하지 않으면 정밀 강조와 diff 합성을 생략하고 이유와 원래 snippet을 표시한다.

## 8. 첫 보고서의 화면과 CSS

홈: 프로젝트/workspace 이름, 요청한 검사 옵션, 관측한 package/target, Clippy 및 수집 상태, 병합 전후 개수, severity와 진단 출처별 개수, 파일 및 진단 종류 링크를 표시한다.

lint 페이지: 동일 code의 occurrence 목록, severity, 메시지, target, 코드 위치, child note/help, replacement와 applicability를 표시한다. Clippy code는 공식 문서로 연결한다. compiler code는 별도 namespace로 구별하며 code 없는 진단은 일반 진단 묶음에 둔다. 위치 없는 오류도 홈에서 접근 가능해야 한다.

source 페이지: 전체 코드, 줄 번호, 관련 finding 목록과 상호 링크를 표시한다. 사이드바는 진단에 포함된 파일의 Files 트리와 Findings in this file 목록을 각각 접고 펼칠 수 있게 제공한다. 파일별 finding 수와 현재 파일을 표시하고 현재 경로를 기본으로 펼친다. 줄 단위 진단 위치와 primary/secondary 텍스트 구분을 제공한다. 2026-10-28 후속 구현으로 Rust 토큰 기반 구문 색칠을 추가하며 원본 rustdoc CSS 클래스를 사용한다. 파일 전체를 분석한 뒤 행별 텍스트 fragment를 템플릿에서 escape한다. 정밀 진단 span 강조는 Phase 2 범위로 남긴다.

좌측 탐색, 본문 폭, 제목 계층, 코드 블록 등은 고정한 rustdoc CSS와 맞는 구조를 사용한다. Phase 1은 light 기본 외형, 후속 Settings 이식은 light/dark/ayu/system preference 선택과 저장을 제공한다. Settings는 고정한 rustdoc의 구조와 적용 가능한 항목을 따르며, 동작하지 않는 UI는 노출하지 않는다. 현재 항목·제외 이유는 2026-11-09-settings-design.md와 ROADMAP.md에 기록한다.

가져온 Rust 릴리스와 정확한 커밋은 출처 파일에 기록한다. upstream JS는 초기 의존성이 아니다. 고정된 원본 CSS에서 기본 @font-face 선언 12개를 별도 fonts.css로 가져오고 수정 없는 WOFF2를 동봉한다. 역할별 폰트 스택, 굵기, 기울임, local(), swap, 한글 unicode-range는 원본을 따른다. Fira Mono는 기본 스택에서 사용하지 않아 제외한다. 내장 SVG와 참조 이미지도 라이선스를 확인해 포함하거나 대체한다. 손댄 부분과 고정 기준을 기록한다.

원본 저작권과 선택한 라이선스 및 자산별 고지를 저장소와 생성 보고서에 포함한다. 보고서에는 실제 동봉 자산의 라이선스 안내 페이지를 제공하고 upstream COPYRIGHT 원문은 참조 자료로 구분한다. 폰트 원본의 해시를 기록하고 저장소에서 줄바꿈 변환을 금지한다. 실행 파일 배포 묶음에도 내장 자산의 고지를 포함한다. 폰트의 별도 조건은 자체 코드 또는 검사 대상 소스의 라이선스를 바꾸지 않는다.

템플릿 기본 escaping을 사용하고 raw HTML에는 고정한 템플릿/신뢰한 구문 강조 결과만 허용한다. 검색은 Phase 2에서 로컬 classic JS로 제공하며 fetch나 module loading에 의존하지 않는다. 동적 결과는 textContent와 검증된 로컬 링크로 만든다.

## 9. 실패와 출력 갱신

Clippy 성공 여부와 수집 완전성은 별도 상태이다. exit 0, 정상 종료 메시지, 구조화 데이터 수집 무결성이 확인되어야 정상 무진단 보고서로 표시한다. compile error로 exit 101인 경우에도 수집이 완전할 수 있으며, 보고서는 실패한 검사라고 표시한다.

| 상황 | 보고서 / 종료 정책 |
| --- | --- |
| Clippy 성공, 진단 0 | 정상 빈 보고서, 0 |
| Clippy 실패, 유효 진단 있음 | 보고서 생성, Clippy의 사용 가능한 비영 종료 코드 보존 |
| Clippy 실패, 진단 전 종료 | 실행 실패 보고서와 stderr 요약, 비영 종료 |
| JSON 손상 또는 정상 완료 확인 불가 | 불완전 상태 보고서, Clippy가 0이어도 도구 오류 1 |
| Cargo 실행/metadata 조회 불가 | 설명을 stderr에 출력, 기존 보고서 유지, 비영 종료 |
| source 읽기 실패 | finding과 fallback 유지, 소스 누락 표시; 검사 결과 자체는 변경하지 않음 |
| 보고서 생성/게시 실패 | 도구 오류 1, 기존 보고서 복구, Clippy 상태도 stderr에 기록 |
| 브라우저 열기 실패 | 완성 보고서 경로와 경고 출력, Clippy/수집 종료 상태 유지 |

OS가 숫자 코드를 제공하지 않는 비정상 종료는 비영 fallback을 사용하고 실행 상태에 원인을 보존한다. --open은 완성된 보고서가 있으면 검사 실패 상태에서도 수행한다.

최종 출력과 같은 파일시스템의 sibling 임시 디렉터리에 전체 결과를 생성하고 내부 링크/필수 파일을 검증한다. 새 보고서 완성 전 기존 보고서에는 쓰지 않는다. 기존 보고서를 backup으로 rename하고 새 결과를 rename하며 실패 시 backup을 복원한다. Windows에서 열린 파일 때문에 rename이 실패하면 명확한 오류와 복구 경로를 알린다. 다중 rename의 crash atomicity는 보장하지 않는다.

기존 출력은 이 도구가 만든 전용 디렉터리임을 marker로 확인한 경우에만 교체한다. 기존의 일반 디렉터리, 소스/manifest, workspace 루트, target 루트, symlink 출력 경로는 거부한다. 자동 복구가 불확실한 backup이 있으면 삭제하지 않고 경로를 알린다. 임시 파일과 backup 정리는 자신이 생성하고 소유권을 확인한 경로에 한정한다.

## 10. Phase 1 작업 순서와 검증 지점

각 단위는 관련 계약 테스트 -> 최소 구현 -> 검증 -> 짧은 구조 설명 순으로 진행한다. 실제 명령 실행과 코드 변경은 구현 승인 이후에 한다.

1. CLI와 실행 문맥: 직접/Cargo 호출, 두 개의 --, Unicode 경로, message-format 충돌, --fix 거부, output/manifest/target-dir 우선순위를 검증한다.
2. 파서와 모델: 저장 JSON으로 severity, code 없는 오류, nested child, 복합 suggestion, 확장 메시지, 손상을 검증한다.
3. 정규화와 dedup: target만 다른 진단 병합, package/level/suggestion 차이 유지, 입력 순서 불변성, 강제 해시 충돌을 검증한다.
4. Cargo 연결: 실제 작은 fixture로 0건, warning, -D warnings, compile error를 수집한다. 테스트 helper로 두 스트림 대량 출력과 비정상 종료를 재현한다.
5. source와 URL: 다중 package 동명 파일, CRLF/Unicode, span 누락, path traversal 및 symlink 경계, anchor 유일성을 검증한다.
6. rustdoc 자산과 HTML: 라이선스 및 출처를 포함하고 index/lint/source를 생성한다. HTML escape, namespace 충돌, 상대 링크와 anchor 존재를 자동 확인한다.
7. 출력 갱신과 통합: 중간 생성 실패, 교체 실패/복원, 잘못된 output을 검증한다. 실제 fixture 보고서에서 종료 코드와 표시 상태가 일치하는지 확인한다.
8. 사용자 검토와 문서화: 실제 보고서를 브라우저로 열어 세 화면, 코드 읽기, 진단 상호 이동을 확인하고 Windows 사용법과 한계를 README에 기록한다.

첫 사용자 검토는 6번까지 연결된 실제 보고서로 한다. 최종 Phase 1 완료는 7~8번 검증까지 통과한 시점이다. 브라우저는 가능한 Chromium 계열과 Firefox로 확인하며 사용할 수 없는 환경은 검증 미완료로 기록한다.

## 11. 후속 Phase와 완료 조건

Phase 2 원래 범위: 공식 category 테이블과 버전/출처 관리, Unknown fallback, category 페이지, 파일/lint/message/category 검색, contextual sidebar 보완, Light/Dark/Auto, Rust 구문 강조, 정밀 primary/secondary span, suggestion diff와 copy, 키보드와 작은 화면 접근성. 현재 contextual source sidebar와 Rust 구문 색칠은 구현했다. 남은 작업 순서는 ROADMAP.md를 따른다. 복합 수정은 같은 suggestion 단위로 diff를 만들며 적용 불가/겹침/소스 불일치 시 replacement 목록으로 대체한다.

Phase 3 원래 범위: --metadata <FILE>와 schema_version 1의 최소 schema, 지정 경로 처리 및 출력 충돌 방지, rustdoc 폰트 검토, Windows/Linux/macOS CI, 설치 검증, file:// 및 repository subpath 정적 호스팅 검증, GitHub Actions artifact 예제와 Pages 가이드, crates.io 배포 준비. 현재 폰트 도입/고지는 구현했고 로컬 설치 smoke는 수행했다. 기본 플랫폼 CI는 앞당긴다. GitHub API나 인증은 제품에 넣지 않는다. 실제 공개 배포는 별도 승인 범위이다.

각 단계 완료 시 파일 목적, 입력/출력, 의존성, 검증 결과와 미검증 항목을 설명한다. 모든 Phase가 끝나야 DETAILS.md의 초기 안정 버전 acceptance 전체를 달성했다고 주장할 수 있다.

## 12. 근거와 후속 확정 사항

- Cargo JSON 및 custom subcommand: https://doc.rust-lang.org/cargo/reference/external-tools.html
- Cargo 설정: https://doc.rust-lang.org/cargo/reference/config.html
- cargo_metadata: https://docs.rs/cargo_metadata/latest/cargo_metadata/
- rustdoc 스타일 원본: https://github.com/rust-lang/rust/tree/main/src/librustdoc/html/static
- Rust 저작권 정책: https://github.com/rust-lang/rust/blob/main/COPYRIGHT
- Rust MIT 라이선스: https://github.com/rust-lang/rust/blob/main/LICENSE-MIT

위 URL은 확인 출처이며 vendor 고정 commit을 대신하지 않는다.

구현 계획 작성 시 확정할 사항: 의존성 버전/MSRV, vendor commit과 파일 목록, 정확한 함수 시그니처 및 테스트 명령. 이들은 설계자의 기술 선택으로 기록하고 사용자가 이미 선택한 것으로 표현하지 않는다.
