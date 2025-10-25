FROM debian:stable-slim

USER root

RUN apt-get update && \
    apt-get install -y unzip curl xmlstarlet && \
    curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
    unzip awscliv2.zip && \
    ./aws/install && \
    aws --version && \
    curl -Lo allure.deb https://github.com/allure-framework/allure2/releases/download/2.35.1/allure_2.35.1-1_all.deb && \
    apt-get install -y ./allure.deb && \
    allure --version

 CMD ["/bin/bash"]