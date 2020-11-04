# Dancespiele API

Dancespiele API is the API to set the percentages of price coin increment for after [Dancespiele Worker](https://github.com/dancespiele/dancaspiele_worker) add stop loss when the price coin achieve this increment.

## Requirements

* Rustup
* Account in [Mailgun](https://www.mailgun.com/)

## How to run the application

1. `git clone https://github.com/dancespiele/dancespiele_api.git`

2. `cd dancespiele_api`

3. set the .env file:

```
SLED_URL=[PATH WHERE YOU WANT THE SLED DB FILE]
SERVER_URL=[API URL]
MAILGUN_DOMAIN=[YOUR EMAIL DOMAIN]
MAILGUN_SECRET=[YOUR MAILGUN SECRET]
MAILGUN_ENDPOINT=[YOUR MAILGUN ENDPOINT FOR EXAMPLE api.eu.mailgun.net]
MAIL_FROM=[EMAIL SENDER]
SECRET=[API SECRET]
```

5. execute:

`cargo run` or with logs mode `RUST_LOG=dancespiele_api cargo=trace cargo run`

6. Enjoy!

## Endpoints

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

### get current percentages set

**Endpoint:** `GET /percentages`

**Note:** You need to create the token with the API secret manualy and set the `Authorization` header with the token generated to request to the API

## Do you like Dancespiele apps?
If you like Dancespiele apps, help us supporting the projects:
- [Github Sponsors](https://github.com/sponsors/dancespiele)

## License
Dancespiele API is [LICENSE PARITY](LICENSE-PARITY.md) and [LICENSE PATRON](LICENSE-PATRON.md) licensed. If you need a comercial license sponsor to Dancespiele in the right tier or contact to `spielcrypto@gmail.com`