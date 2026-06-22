---
name: add-openapi
description: blog-v2 OpenAPI 도메인 스펙을 현재 라우트 기준으로 재생성·갱신한다. 서비스의 라우트 등록·핸들러·타입을 읽고, doc-source/openapi/specs/<service>/<domain>.yaml을 정확히 미러링해 작성한다(auth/auth.yaml 컨벤션). 엔드포인트 추가/변경 뒤 사용.
argument-hint: "<service>/<domain>  (예: rust/posts, go/hackernews) — 또는 변경된 라우트 파일 경로"
allowed-tools: Read, Grep, Glob, Edit, Write, Bash(redocly lint:*), Bash(find:*), Bash(ls:*)
---

> 영어 원본: `SKILL.md` (실제 로드되는 source of truth). 이 파일은 읽기용 한국어
> 미러 — 한쪽을 고치면 같은 턴에 다른 쪽도 반영한다.

blog-v2 OpenAPI 도메인 스펙을 실제 라우트와 동기 상태로 유지한다. 인자는
`<service>/<domain>`(예: `rust/posts`, `go/hackernews`)이거나, 방금 변경된
라우트/핸들러 파일 경로다 — 후자라면 거기서 서비스·도메인을 추론한다.

## 단계

1. **라우트를 찾는다.** 대상 도메인의 라우트 등록과 핸들러 + 요청/응답 타입을 읽는다:
   - rust 서비스: `services/<svc>/src/routes/*.rs`, `services/<svc>/src/handlers/*.rs`, `services/<svc>/src/types/*`
   - go 서비스: `services/go/**/*.go` (`mux.HandleFunc` 등록과 json 태그가 붙은 struct)
   도메인의 모든 메서드 + 경로를 열거한다. 

2. **스펙을 작성한다** — `doc-source/openapi/specs/<service>/<domain>.yaml`에, 이 레포의
   `CLAUDE.md`와 `auth/auth.yaml` 템플릿 컨벤션을 정확히 따른다:
   - `openapi: 3.1.0`; `info`에 title/description과 고유한 `x-api-id`.
   - `servers`: prod `https://api.mogumogu.dev/<service>` + 로컬 포트(auth 8001,
     rust 8002, go 8003, haskell 8004). 경로는 서비스 prefix 없이.
   - 도메인용 `tags` 1개.
   - 공개 오퍼레이션엔 `security: []`; 인증 오퍼레이션엔 핸들러에 맞는 보안 스킴
     (`bearerAuth` JWT, `apiSecret`, `hackernewsSecret` 등).
   - `components`(schemas/parameters/responses)를 재사용해 DRY하게; 공유 `Error`
     스키마 `{error: string}`; nullable은 OpenAPI 3.1 `type: [T, 'null']`.
   - `/health`는 넣지 않는다.
   파일이 이미 있으면 통째로 다시 쓰지 말고 그 자리에서 갱신한다.

3. **검증** — 별도 툴 없이: 유효한 YAML인지, blog-doc 번들러가 에러 없이 병합하는지
   확인한다(문서 내부 `$ref`만 써서 병합 문서에서 해소되게). `redocly lint <spec>`은
   설치돼 있으면 선택적 심화 검증.

4. **보고**: 최종 엔드포인트 목록, 검증 결과, 코드 불일치. 커밋은 하지 않는다 —
   사용자에게 맡긴다.

blog-doc 사이트가 빌드 때 `specs/**/*.yaml`을 하나의 Scalar 번들로 병합하므로, 여기선
유효한 standalone 스펙 하나면 충분하다 — 이 레포엔 번들링 단계가 없다.
