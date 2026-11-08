# hurl — request files for auth, rust and go

> Korean mirror: `CLAUDE-ko.md`. English is the source of truth; when you edit
> one, mirror the change in the other in the same turn.

One request per `.hurl` file. The folder path follows the URL and the file name is the method: `rust/posts/recent/GET.hurl` is `GET {{rustUrl}}/posts/recent`. Variables come from `local.env` or `production.env` in this folder.

## Running

`hurll` is a shell function in the dotfiles zshrc. Its first argument is the env (local or production, local when left out). It walks up from the file's folder to the nearest `<env>.env` and passes it as `--variables-file`, so any working folder and absolute paths work.

```bash
hurll hurl/rust/health/GET.hurl                       # local
hurll production hurl/rust/posts/recent/GET.hurl
hurll hurl/rust/posts/recent/GET.hurl --verbose
```

Without hurll:

```bash
hurl --variables-file hurl/local.env hurl/rust/posts/recent/GET.hurl
```

Check the three servers at once with `--test`. Pick read-only files only; a POST changes data.

```bash
hurl --test --variables-file hurl/production.env \
  hurl/auth/health/GET.hurl \
  hurl/rust/health/GET.hurl \
  hurl/go/health/GET.hurl
```

## Variables and secrets

- `local.env` and `production.env` hold the real values and are gitignored (`hurl/*.env`). Only the `.env.example` files are committed.
- On a new machine: `cp hurl/local.env.example hurl/local.env`, then fill in the tokens.
- Keys: `authUrl rustUrl goUrl accessToken refreshToken apiSecret hackerNewsDate importFromDate importJobId`.

## Asserts

- A read request checks `HTTP 200` and nothing else.
- A POST that changes data (create, fetch, summarize, translate, draw, import, pull, delete) checks for any 2xx:
  ```
  HTTP *
  [Asserts]
  status >= 200
  status < 300
  ```
- `auth/login/google` and `auth/callback/google` have no status line: login redirects differently per environment, and callback is a placeholder.
- When the server answers with another code (for example 201 on create), change only that file's status line.

## Notes

- A 401 means `accessToken` expired: run `auth/login/google`, then `auth/refresh`, and replace `accessToken` in the env.
- `rust/import/jobs/{id}/GET.hurl` checks that a request without a token gets 401.
