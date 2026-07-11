# kapowarr

[Kapowarr](https://github.com/Casvt/Kapowarr) manages a comic book library —
downloading, renaming, and organizing issues and volumes.

First-party [orca](https://github.com/argyle-labs/orca) plugin (service-backend).

> **Status:** deploy config + docs only. Kapowarr is not Servarr-shaped, so it
> lives in its own repo rather than the [`arr`](https://github.com/argyle-labs/arr)
> workspace. The orca service-backend implementation is planned; today this repo
> carries the self-contained deploy config so it can be stood up by hand or via orca.

| App | Port | Role |
|-----|------|------|
| **Kapowarr** | `5656` | Comic book library management |

## Deploy

```bash
docker compose up -d
```

See [`compose.yml`](compose.yml) for volumes and environment (upstream image,
`${MEDIA_PATH}` + config paths parameterized).

Operator notes: [`docs/kapowarr.md`](docs/kapowarr.md).
