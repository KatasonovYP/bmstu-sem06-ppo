FROM cr.yandex/crpav8o4hj057l1dbgqa/the-qsb/stocks-tracker/muslrust:1.88.0-nightly

RUN curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin \
    && apt update \
    && apt install sccache yq xmlstarlet -y \
    && curl -LO https://github.com/getsops/sops/releases/download/v3.11.0/sops-v3.11.0.linux.amd64 \
    && mv sops-v3.11.0.linux.amd64 /usr/local/bin/sops \
    && chmod +x /usr/local/bin/sops \
    && sops -v \
    && rustup component add clippy \
    && rustup component add rustfmt

CMD ["/bin/bash"]
