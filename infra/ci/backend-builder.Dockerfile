FROM cr.yandex/crpav8o4hj057l1dbgqa/the-qsb/stocks-tracker/muslrust:1.88.0-nightly

RUN curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin \
    # install utils
    && apt update \
    && apt install -y sccache yq jq xmlstarlet \
    # install encryption tool
    && curl -LO https://github.com/getsops/sops/releases/download/v3.11.0/sops-v3.11.0.linux.amd64 \
    && mv sops-v3.11.0.linux.amd64 /usr/local/bin/sops \
    && chmod +x /usr/local/bin/sops \
    && sops -v \
    # install lint tool
    && rustup component add clippy \
    # install fmt tool
    && rustup component add rustfmt \
    # install complexity tool
    && git clone https://github.com/mozilla/rust-code-analysis.git /tmp/rust-code-analysis \
    && cd /tmp/rust-code-analysis \
    && cargo build --release -p rust-code-analysis-cli --target x86_64-unknown-linux-gnu \
    && ls -la ./target/release \
    && ls -la ./target/x86_64-unknown-linux-gnu/release \
    && ln ./target/x86_64-unknown-linux-gnu/release/rust-code-analysis-cli /usr/local/bin/rust-code-analysis-cli \
    && cd - \
    && rust-code-analysis-cli --version

CMD ["/bin/bash"]
