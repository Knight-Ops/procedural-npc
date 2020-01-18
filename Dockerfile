FROM alpine

RUN adduser -D -u 1000 rust

COPY target/release/dnd-generator .

RUN chmod a+x dnd-generator

USER rust

ENTRYPOINT [ "./dnd-generator" ]