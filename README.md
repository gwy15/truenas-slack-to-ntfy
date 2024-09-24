# TrueNAS slack to ntfy

This program mocks slack webhook for TrueNAS and forwards notifications to a ntfy server.

## Usage
Docker recommended.

### Environments
- `BIND`: listen interface and ports. Defaults to `0.0.0.0:80`
- `NTFY_BASE_URL`: the ntfy server to forward. Defaults to `https://ntfy.sh`.
    You can use self hosted https://my-ntfy.example.com/with-path.
    > The NTFY_BASE_URL should NOT contain topic path.
- `LISTEN_BASE_PATH`: The base path for the forward server. Defaults to `/`
    > For example, you may want the server to forward https://my-ntfy.example.com/slack/topic to https://my-ntfy.example.com/topic. This is when you set `LISTEN_BASE_PATH=/slack`
