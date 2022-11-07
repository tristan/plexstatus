Checks if there is any media running on plex. If so, returns with exit code 1, otherwise returns with exit code 0.

## Config

[get token](https://support.plex.tv/articles/204059436-finding-an-authentication-token-x-plex-token/) and use it to set the `token` value in  ~/.config/plexstatus/config.toml

```
url = "http://localhost:32400"
token = "..."
```
