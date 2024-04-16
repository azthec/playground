# quarkus-smallrye-jwt

Prototype test for JWT token based endpoint authentication.

## Setting up certificates

```bash
cd src/main/resources
openssl req -newkey rsa:2048 -new -nodes -keyout privatekey.pem -out csr.pem
openssl rsa -in privatekey.pem -pubout > publickey.pem
```

## Running the application in dev mode

You can run your application in dev mode that enables live coding using:
```shell script
./mvnw compile quarkus:dev
```

