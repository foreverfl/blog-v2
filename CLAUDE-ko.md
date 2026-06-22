# blog-v2 — 프로젝트 지침

> 영어 원본: `CLAUDE.md`. 영어가 source of truth이며, 한쪽을 고치면 같은 턴에
> 다른 쪽도 반영합니다.

## OpenAPI 스펙 upkeep

엔드포인트를 추가하거나 변경하면, 그에 맞춰 OpenAPI 스펙도 반드시 갱신한다 —
요청받을 때만이 아니라 작업의 일부로 매번 한다(글로벌 `.hurl` upkeep 규칙과 대칭).

- 스펙은 `doc-source/openapi/specs/<service>/<domain>.yaml`에 둔다. 백엔드 도메인마다
  독립적인 OpenAPI 3.1 문서 하나 — 예: `rust/posts.yaml`, `go/hackernews.yaml`,
  `auth/auth.yaml`. 라우트가 바뀐 도메인 파일을 갱신하고, 새 도메인이 생기면
  `<service>/<domain>.yaml`을 추가한다.
- 라우트를 **정확히** 반영한다: 경로·메서드·파라미터·요청/응답 형태·상태 코드·인증.
  실제 핸들러와 타입을 읽는다 — 추측하지 않는다.
- 기존 `auth/auth.yaml` 컨벤션을 따른다:
  - `openapi: 3.1.0`; `info`에 고유한 `x-api-id`.
  - `servers`: 서비스 베이스 — prod `https://api.mogumogu.dev/<service>` + 로컬 포트
    (auth 8001, rust 8002, go 8003, haskell 8004). 경로는 서비스 prefix 없이 적는다 —
    prefix는 서버 URL이 담는다.
  - 공개 오퍼레이션엔 `security: []`; 인증 오퍼레이션엔 보안 스킴(`bearerAuth` JWT,
    `apiSecret`, `hackernewsSecret` 등).
  - 공유 `Error` 스키마 `{error: string}`; nullable 필드는 OpenAPI 3.1
    `type: [T, 'null']`.
- 스펙에 `/health`는 넣지 않는다 — 병합 문서에서 서비스 간 충돌하고, API 표면이
  아니라 인프라용이다.
- blog-doc 문서 사이트가 빌드 때 이 스펙들을 하나의 Scalar 레퍼런스로 병합한다
  (`blog-doc/scripts/bundle-openapi.js`가 `OPENAPI_SPECS_SRC` 또는 sibling 체크아웃을
  읽음). blog-v2 자체엔 redocly·번들링 단계가 없다.
- 변경한 스펙은 별도 툴 없이 검증한다: 유효한 YAML이어야 하고, blog-doc 번들러가
  에러 없이 병합해야 한다(`blog-doc/scripts/bundle-openapi.js`가 빌드 때 모든 스펙을
  파싱 — 잘못된 파일은 거기서 실패). `$ref`는 전부 문서 내부(`#/components/...`)로 두어
  병합 문서에서 해소되게 한다. `redocly lint`는 선택적 심화 검증일 뿐 필수 아님.
- `/add-openapi <service>/<domain>`으로 현재 라우트 기준 도메인 스펙을 재생성한다.
