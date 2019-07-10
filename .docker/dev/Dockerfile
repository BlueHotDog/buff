FROM node:12.5-alpine as webpack

RUN npm install -g yarn

RUN apk --update --upgrade add ca-certificates build-base bash \
    && update-ca-certificates --fresh \
    && rm -rf /var/cache/apk/*  \
    && apk del build-base

WORKDIR /app/assets

COPY server/assets/package.json assets/*yarn* ./
COPY server/deps /app/deps

RUN yarn install

COPY server/assets .

ENV NODE_ENV="development"

CMD ["tail", "-f", "/dev/null"]

###############################

FROM elixir:1.9-alpine as app

WORKDIR /app

RUN apk --update --upgrade add git build-base ca-certificates inotify-tools postgresql-client bash \
    && update-ca-certificates --fresh \
    && rm -rf /var/cache/apk/*

COPY ./.docker/dev/wait-for-postgres.sh /
COPY ./server/mix.* ./

ENV MIX_ENV="dev" \
    MIX_HOME=/opt/mix \
    HEX_HOME=/opt/hex

RUN mix local.hex --force \
    && mix local.rebar --forwwce \
    && mix deps.get \
    && mix deps.compile

CMD ["tail", "-f", "/dev/null"]
