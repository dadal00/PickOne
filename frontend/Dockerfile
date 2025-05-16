FROM node:23-alpine@sha256:139be64e98a1374a1c49ee62b23a91f688a37a628422ff8bb9fba94185678ab3 AS builder

ARG PUBLIC_BACKEND_URL
ARG PUBLIC_WS_URL
ARG PORT

ENV PUBLIC_BACKEND_URL=$PUBLIC_BACKEND_URL
ENV PUBLIC_WS_URL=$PUBLIC_WS_URL

WORKDIR /app

COPY package.json package-lock.json ./

RUN npm ci --omit=dev

COPY . .

RUN npm run build

FROM node:23-alpine@sha256:139be64e98a1374a1c49ee62b23a91f688a37a628422ff8bb9fba94185678ab3

WORKDIR /app

COPY --from=builder --chown=node:node /app/build ./build
COPY --from=builder --chown=node:node /app/node_modules ./node_modules
COPY --chown=node:node package.json .

CMD [ "node", "build" ]
