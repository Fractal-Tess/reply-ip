FROM alpine

RUN apk add libc6-compat libgcc gcompat

WORKDIR /app
COPY --chown=root:root target/release/reply-ip .

EXPOSE 3000

ENTRYPOINT ["./reply-ip" ]