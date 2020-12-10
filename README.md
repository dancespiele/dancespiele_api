# Spielcrypto API

Spielcrypto API is the API to set the percentages of price coin increment for after [Spielcrypto Worker](https://github.com/dancespiele/spielcrypto_worker) add stop loss when the price coin achieve this increment.

## Requirements

* [Rustup](https://rustup.rs/)
* [RabbitMQ](https://www.rabbitmq.com/)
* [Postgresql](https://www.postgresql.org/)
* [Diesel client](https://diesel.rs/)
* Account in [Mailgun](https://www.mailgun.com/)

## How to run the application

1. `git clone https://github.com/dancespiele/dancespiele_api.git`

2. `cd dancespiele_api`

3. create a database in Postgresql

4. create the .env file of the root project:

```
SLED_URL=[PATH WHERE YOU WANT THE SLED DB FILE]
DATABASE_URL=[YOUR POSTGRESS DATABASE]
DB_SCHEMA=[YOUR SCHEMA]
SERVER_URL=[API URL]
AMPQ_ADDR=[YOUR AMPQ ADDRESS]
MAILGUN_DOMAIN=[YOUR EMAIL DOMAIN]
MAILGUN_SECRET=[YOUR MAILGUN SECRET]
MAILGUN_ENDPOINT=[YOUR MAILGUN ENDPOINT FOR EXAMPLE api.eu.mailgun.net]
MAIL_FROM=[EMAIL SENDER]
SECRET=[API SECRET]
LANGUAGE=en //for now only support en (English) or es (Spanish)
```

5. create a .env file inside of `refinery_migrations` folder

```
URL_DB=[YOUR POSTGRESS DATABASE]
SCHEMA=[YOUR SCHEMA]
```

6. run the migrations inside of `refinery_migrations`

`cargo run`

7. Create the file `schema.rs` in root folder

8. Print the schema of your database

`diesel print-schema --schema [YOUR DATABASE NAME] [YOUR SCHEMA]`

9. Copy everything inside of `pub mod [YOUR DATABASE NAME]` and paste in `schema.rs`.
It should looks similar to `schema.rs.example`

10. Execute:

`cargo run` or with logs mode `RUST_LOG=spielcrypto_api cargo=trace cargo run`

11. Enjoy!

**Note:** Before to execute Spielcrypto API you need to run the RabbitMQ server first

## Endpoints

### Create user

**Endpoint:** `POST /user`

**Body Example**:

```json
{
    "email": "[EMAIL]",
    "username": "[USERNAME]",
    "password": "[PASSWORD]",
    "status": "ACTIVE"
}
```

**Note:** You need to create the token for the first user, you can generate it installing this [create](https://github.com/dancespiele/token_generator)

### login

**Endpoint** `POST /login`

**Body Example:**

```json
{
    "username": "[USERNAME]",
    "password": "[PASSWORD]"
}
```

### Get user

**Endpoint:** `GET /user`

**Note:** The api will response with the user owner of the token that is sent in the `Authorization header`

### Set percentages

**Endpoint:** `POST /percentages`

**Body Example**:

```json
[
    {
        "pair": "ETHEUR",
        "new_stop_loss": "30.0",
        "next_stop_loss": "10.0"
    },
    {
        "pair": "KAVAEUR",
        "new_stop_loss": "20.0",
        "next_stop_loss": "10.0"
    }
]
```

**Note:** You need to login first to get the token to set the `Authorization` header

### get current percentages set

**Endpoint:** `GET /percentages`

**Note:** You need to login first to get the token to set the `Authorization` header

### Issues

**Spielcrypto API doesn't shutdown with Ctrl-C:** For now you need to kill the process to shutdown the API. I'm trying to figure out how to fix this issue

## Do you like Dancespiele apps?
If you like Dancespiele apps, help me supporting the projects:
- Sending coins to the address **0x619d3FA3bD7CF497d9899Ccd4d7b5663Ff318e52**
- BAT rewards in case that you use [Brave Browser](https://brave.com/)
- [Github Sponsors](https://github.com/sponsors/dancespiele)
- Burst coins to the address BURST-DPN6-2AT3-FCRL-9BBKG

## License
Dancespiele API is [LICENSE PARITY](LICENSE-PARITY.md) and [LICENSE PATRON](LICENSE-PATRON.md) licensed. If you need a comercial license sponsor to Dancespiele in the right tier or contact to `spielcrypto@gmail.com`

**Warning:** Each functionality is tested before to avoid bugs however the author of this app is not responsible for all the issues and losses that can happen using it. Please, read the licenses.
