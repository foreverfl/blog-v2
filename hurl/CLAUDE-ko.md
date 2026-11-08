# hurl — auth · rust · go 요청 파일

> 영어판 `CLAUDE.md`가 원본이다. 한쪽을 고치면 같은 턴에 다른 쪽도 맞춘다.

요청 하나가 `.hurl` 파일 하나다. 폴더 경로는 URL을 따르고 파일 이름은 메서드다: `rust/posts/recent/GET.hurl`은 `GET {{rustUrl}}/posts/recent`. 변수는 이 폴더의 `local.env`나 `production.env`에서 온다.

## 돌리기

`hurll`은 dotfiles zshrc에 있는 셸 함수다. 첫 인자는 env(local 또는 production, 생략하면 local)다. 파일이 있는 폴더에서 위로 올라가며 가장 가까운 `<env>.env`를 찾아 `--variables-file`로 넘긴다. 그래서 어느 폴더에서 쳐도 되고 절대경로도 된다.

```bash
hurll hurl/rust/health/GET.hurl                       # local
hurll production hurl/rust/posts/recent/GET.hurl
hurll hurl/rust/posts/recent/GET.hurl --verbose
```

hurll 없이:

```bash
hurl --variables-file hurl/local.env hurl/rust/posts/recent/GET.hurl
```

서버 셋을 한 번에 보려면 `--test`. 읽기 파일만 고른다. POST는 데이터를 바꾼다.

```bash
hurl --test --variables-file hurl/production.env \
  hurl/auth/health/GET.hurl \
  hurl/rust/health/GET.hurl \
  hurl/go/health/GET.hurl
```

## 변수와 비밀

- `local.env`와 `production.env`에 실제 값이 들어가고 gitignore된다(`hurl/*.env`). 커밋되는 건 `.env.example`뿐이다.
- 새 컴퓨터에서: `cp hurl/local.env.example hurl/local.env` 뒤 토큰을 채운다.
- 키: `authUrl rustUrl goUrl accessToken refreshToken apiSecret hackerNewsDate importFromDate importJobId`.

## 응답 확인(assert)

- 읽기 요청은 `HTTP 200` 한 줄만 본다.
- 데이터를 바꾸는 POST(create, fetch, summarize, translate, draw, import, pull, delete)는 2xx면 통과다:
  ```
  HTTP *
  [Asserts]
  status >= 200
  status < 300
  ```
- `auth/login/google`과 `auth/callback/google`에는 status 줄이 없다. login은 환경마다 리다이렉트가 다르고, callback은 자리만 잡아 둔 파일이다.
- 서버가 다른 코드로 답하면(예: create가 201) 그 파일의 status 줄만 고친다.

## 메모

- 401이 나면 `accessToken`이 만료된 것이다: `auth/login/google` → `auth/refresh` 뒤 env의 `accessToken`만 바꾼다.
- `rust/import/jobs/{id}/GET.hurl`은 토큰 없는 요청이 401을 받는지 본다.
