FROM arm32v7/debian:buster-slim
# FROM --platform=linux/arm64 debian:buster-slim
# FROM --platform=linux/arm/v7 debian:buster-slim
ENV DEBIAN_FRONTEND=noninteractive \
    DEBCONF_NONINTERACTIVE_SEEN=true
COPY cross/rustup-init /tmp
COPY cross/container-setup.sh /tmp/setup.sh
RUN /tmp/setup.sh
CMD ["uname", "-a"]
