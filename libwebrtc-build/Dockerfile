FROM python:3
RUN mkdir -p /workspace
WORKDIR /workspace
COPY bin/ /workspace/bin
COPY REVISION /workspace/REVISION
COPY BUILD.gn /workspace/BUILD.gn
RUN /workspace/bin/run
