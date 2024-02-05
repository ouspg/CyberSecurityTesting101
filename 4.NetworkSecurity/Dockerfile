FROM python:bullseye

RUN apt-get update && apt-get install -y python3-pip curl neovim iproute2 \
    && pip install --upgrade pip \
    && pip install scapy