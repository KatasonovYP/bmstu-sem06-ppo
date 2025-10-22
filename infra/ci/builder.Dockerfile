FROM clux/muslrust:1.88.0-nightly

RUN curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin
RUN apt update \
    && apt install sccache yq -y

RUN curl -LO https://github.com/getsops/sops/releases/download/v3.11.0/sops-v3.11.0.linux.amd64 \
 && mv sops-v3.11.0.linux.amd64 /usr/local/bin/sops \
 && chmod +x /usr/local/bin/sops \
 && sops -v

CMD ["/bin/bash"]
