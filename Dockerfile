# Pull and extract password lists
FROM byrnedo/alpine-curl as passwords
RUN curl https://codeload.github.com/danielmiessler/SecLists/tar.gz/master -o master.tar.gz
RUN tar -C / -zxvf master.tar.gz SecLists-master/Passwords


# Build static binary
FROM clux/muslrust as builder
COPY . /volume
WORKDIR /volume
RUN cargo build --release


# Build final release stage
FROM alpine
MAINTAINER jgedeon@sofi.com

ENV TZ=America/New_York

COPY --from=passwords \
  /SecLists-master/Passwords/* \
  /passwords/

# Install the esindex binary
COPY --from=builder \
  /volume/target/x86_64-unknown-linux-musl/release/checkpass \
  /usr/bin/

CMD ["sh", "-c", "checkpass /passwords/*"]
