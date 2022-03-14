# Phone mailer

A small project to receive twilio SMS by mail and mails by SMS


## tldr
```
# Create your Twilio Api Key
cp .env.tp .env
# Fill your enviornment
docker-coompose up
```
Setup the callback on SMS in Twilio
Send a SMS to receive your email

## Dependencies
* [sendgrid-rs](https://github.com/gsquire/sendgrid-rs)
* [twilio API](https://www.twilio.com/docs/openapi/generating-a-rust-client-for-twilios-api)
* [Rocket](https://github.com/SergioBenitez/Rocket)


## dev
Development commands.
Since the callbacks may not be able to reach your dev environment, it may not be easy to develop from anywhere. Ensure unit tests are completed and maintained

### Run development version:
```
docker run -it --rm --name phone_mailer -v $(pwd):/usr/src/app -p 8080:8080 lerignoux/phone-mailer cargo run
```

### Run tests:
```
docker run -it --rm --name phone_mailer -v $(pwd):/usr/src/app -p 8080:8080 lerignoux/phone-mailer cargo test
```
