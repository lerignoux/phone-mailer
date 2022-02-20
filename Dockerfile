FROM rust:1.58-buster
LABEL maintainer="lerignoux@gmail.com"

RUN apt-get update && apt-get upgrade -y && apt-get install -y curl maven jq openssl

RUN mkdir -p ~/bin/openapitools
RUN curl https://raw.githubusercontent.com/OpenAPITools/openapi-generator/master/bin/utils/openapi-generator-cli.sh > ~/bin/openapitools/openapi-generator-cli
RUN chmod u+x ~/bin/openapitools/openapi-generator-cli

RUN ~/bin/openapitools/openapi-generator-cli generate -g rust -i https://raw.githubusercontent.com/twilio/twilio-oai/main/spec/json/twilio_api_v2010.json -o /usr/src/twilio-rust

RUN mkdir /usr/src/app
COPY . /usr/src/app
WORKDIR /usr/src/app/phone_mailer
RUN cd /usr/src/app/phone_mailer

RUN cargo install cargo-edit
RUN ls ~/bin/openapitools/
RUN cargo add openapi --path=/usr/src/twilio-rust
RUN cargo add dotenv
RUN cargo add tokio --features full

# RUN cargo build

CMD ["cargo", "run"]
