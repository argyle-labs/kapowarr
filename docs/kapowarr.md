# Kapowarr

Comic and manga library manager. Monitors ComicVine for new issues and imports them automatically. Integrates with Komga.

- **Port**: 5656
- **Image**: `mrcas/kapowarr`
- **Compose**: [compose.yml](../compose.yml)
- **Network**: `media` (external)

## Volumes

| Container Path | Description |
|----------------|-------------|
| `/app/db` | Kapowarr config and database |
| `/downloads` | Imported files (working area) |
| `/data/media/comics` | Comics library |
| `/data/media/manga` | Manga library |

Host paths are parameterized in [compose.yml](../compose.yml) via `KAPOWARR_CONFIG_PATH`, `DOWNLOADS_PATH`, and `MEDIA_PATH`.

## Sources

Kapowarr fetches issues through its built-in acquisition client. Configure sources in Settings.

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `TZ` | `America/Denver` | Timezone |
| `KAPOWARR_IMAGE_TAG` | `latest` | Image tag |
| `KAPOWARR_CONFIG_PATH` | `/opt/appdata/kapowarr` | Host config directory (mounted at `/app/db`) |
| `KAPOWARR_PORT` | `5656` | Host port |
| `DOWNLOADS_PATH` | `/mnt/pool/downloads` | Host imports working directory |
| `MEDIA_PATH` | `/mnt/pool/data/media` | Media library base path |

## Deploy

```bash
docker compose up -d
```

The `media` network is external and must exist beforehand:

```bash
docker network create media
```

## Troubleshooting

```bash
docker logs kapowarr
docker compose ps
```
