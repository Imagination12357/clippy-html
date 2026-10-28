# 보고서 문구와 crate 버전 표시

사용자 요청에 따라 다음을 반영했다.

- Cargo metadata의 package version을 내부 모델에 보존하고 사이드바의 각 crate 이름 아래 표시한다. 여러 package의 버전이 달라도 이름과 버전을 각각 연결한다.
- Diagnostic kinds의 반복적인 `Clippy ·` 접두사를 제거했다. 열 비율은 다른 목록과 정렬되도록 rustdoc의 공통 item-table 비율을 유지한다. 목록 전용 열 확대는 사용자 피드백에 따라 되돌렸다.
- 홈의 반복적인 소스 포함/오프라인 안내문은 제거했다. 소스 포함에 관한 설명은 README에 유지한다.
- Category: Unknown은 Phase 1에 공식 lint category 테이블을 아직 도입하지 않아 표시되는 상태이다. 이번 수정에는 분류 기능 구현을 포함하지 않았다.

workspace fixture의 두 package에 서로 다른 버전(0.1.0, 0.2.0)을 지정했다. 사이드바 버전 표시가 없어서 실패하는 통합 회귀 테스트를 확인한 뒤 수정했다. 단위 33개와 실제 Clippy 통합 7개, 총 40개 테스트가 통과했다.
