FROM alpine

RUN useradd -ms /bin/bash rust

COPY target/release/dnd-generator .

RUN chmod a+x dnd-generator

ENTRYPOINT [ "./dnd-generator" ]