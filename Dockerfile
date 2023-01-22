#runs only the api 

FROM alpine:latest

COPY ./builds/api /usr/local/app/
COPY ./builds/mock /usr/local/app/
COPY ./translations /usr/local/app/translations
WORKDIR /usr/local/app/
ENV PORT=8080


RUN ["chmod", "+x", "./api"]
CMD ["./api"]