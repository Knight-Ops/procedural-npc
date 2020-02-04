FROM alpine

RUN adduser -D -u 1000 rust

COPY server/src/grammars .
COPY server/target/release/dnd-tools .
COPY server/static .

RUN chmod a+x dnd-tools

USER rust

ENTRYPOINT [ "./dnd-tools" ]