FROM ubuntu

ARG TARGETARCH

COPY server/target/release/dnd-tools dnd-tools-amd64
COPY server/target/aarch64-unknown-linux-gnu/release/dnd-tools dnd-tools-aarch64
COPY server/static static/

RUN if [ "${TARGETARCH}" = "amd64" ] ; then mv dnd-tools-amd64 dnd-tools && rm dnd-tools-aarch64; else mv dnd-tools-aarch64 dnd-tools && rm dnd-tools-amd64; fi

RUN chmod a+x dnd-tools

COPY entrypoint.sh .

RUN chmod a+x entrypoint.sh

ENTRYPOINT [ "./dnd-tools" ]