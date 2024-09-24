# TrueNAS slack to ntfy

This program mocks slack webhook for TrueNAS and forwards notifications to a ntfy server.

## Usage
Docker recommended.

```yaml
services:
  truenas-slack-to-ntfy:
    image: gwy15/truenas-slack-to-ntfy:main
    container_name: truenas-slack-to-ntfy
    environment:
      - BIND=0.0.0.0:80
      - NTFY_BASE_URL=https://ntfy.sh
      - LISTEN_BASE_PATH=/
    ports:
      - 80:80
    restart: unless-stopped
```

You can also compile via `cargo build --release --target x86_64-unknown-linux-musl`
and write a systemd service file manually.

### Environments
- `BIND`: listen interface and ports. Defaults to `0.0.0.0:80`
- `NTFY_BASE_URL`: the ntfy server to forward. Defaults to `https://ntfy.sh`.
    You can use self hosted https://my-ntfy.example.com/with-path.
    > The NTFY_BASE_URL should NOT contain topic path.
- `LISTEN_BASE_PATH`: The base path for the forward server. Defaults to `/`
    > For example, you may want the server to forward https://my-ntfy.example.com/slack/topic to https://my-ntfy.example.com/topic. This is when you set `LISTEN_BASE_PATH=/slack`
