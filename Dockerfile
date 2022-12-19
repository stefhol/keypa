FROM alpine:latest

COPY ./builds/api /usr/local/app/
COPY ./builds/mock /usr/local/app/
WORKDIR /usr/local/app/
ENV PORT=8080


RUN ["chmod", "+x", "./api"]
CMD ["./api"]