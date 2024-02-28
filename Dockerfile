FROM alpine:latest
RUN apk update && \
    add --no-cache sqlite
CMD ["sqlite3"]
