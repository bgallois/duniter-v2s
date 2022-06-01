# How to deploy a permanent mirror node on ĞDev network

## Publish a node

### Duniter part
- Add this docker-compose on your server :
[docker/compose/gdev-mirror.docker-compose.yml](https://git.duniter.org/nodes/rust/duniter-v2s/-/blob/master/docker/compose/gdev-mirror.docker-compose.yml)
- Edit lines 26: `"/dns/SERVER_DOMAIN/tcp/30333/p2p/PEER_ID"`
  with your domain name an the PEER_ID you get using te command in comment.
- If you have write access errors run in docker-compose.yml folder : `chmod o+rwX -R .`
- `docker-compose up -d` to start your node
### Reverse-proxy part (with Nginx)
In `/etc/nginx/sites-enabled/gdev.YOUR_DOMAIN` put (you can probably do simpler) :
```
server {
  server_name gdev.YOUR_DOMAIN.fr;

  listen 443 ssl http2;
  ssl_certificate /etc/nginx/ssl/YOUR_DOMAIN.cert;
  ssl_certificate_key /etc/nginx/ssl/YOUR_DOMAIN.key;

  root /nowhere;

  add_header X-Frame-Options SAMEORIGIN;
  add_header X-XSS-Protection "1; mode=block";
  proxy_redirect off;
  proxy_buffering off;
  proxy_set_header Host $host;
  proxy_set_header X-Real-IP $remote_addr;
  proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
  proxy_set_header X-Forwarded-Proto $scheme;
  proxy_set_header X-Forwarded-Port $server_port;
  proxy_read_timeout 90;

  location /http {
    proxy_pass http://localhost:9933;
    proxy_http_version 1.1;
  }
  location /ws {
    proxy_pass http://localhost:9944;

    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "upgrade";
    proxy_http_version 1.1;

    proxy_read_timeout 1200s;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header Host $host;
  }
}
```
and replace `YOUR_DOMAIN` by your domain each time.

- [generate your ssl certificates](https://github.com/acmesh-official/acme.sh) with let's encrypt
  if you don't already have a wildcard certificate.
- `service nginx reload`

Your node is now online as a mirror node. It's fully capable for wallet use.

To go further, read [How to become a (black)smith](./smith.md)