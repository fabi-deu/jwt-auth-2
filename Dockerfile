FROM ubuntu:latest
LABEL authors="fabi"

ENTRYPOINT ["top", "-b"]