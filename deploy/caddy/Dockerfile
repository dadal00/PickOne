FROM caddy:2-builder-alpine@sha256:83c0272b68761fa47e431b66dc3c4412e8cf419e107c5bc579d4d6e8f4652260

RUN xcaddy build \
    --with github.com/caddy-dns/cloudflare \
    --with github.com/greenpau/caddy-security
