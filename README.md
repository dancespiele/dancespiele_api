# Dancespiele API

Dancespiele API is the API to set the percentages of price coin increment for after [Dancespiele Worker](https://github.com/dancespiele/dancaspiele_worker) add stop loss when the price coin achieve this increment.

## Requirements

* [Rustup](https://rustup.rs/)
* [RabbitMQ](https://www.rabbitmq.com/)
* Account in [Mailgun](https://www.mailgun.com/)

## How to run the application

1. `git clone https://github.com/dancespiele/dancespiele_api.git`

2. `cd dancespiele_api`

3. set the .env file:

```
SLED_URL=[PATH WHERE YOU WANT THE SLED DB FILE]
SERVER_URL=[API URL]
AMPQ_ADDR=[YOUR AMPQ ADDRESS]
MAILGUN_DOMAIN=[YOUR EMAIL DOMAIN]
MAILGUN_SECRET=[YOUR MAILGUN SECRET]
MAILGUN_ENDPOINT=[YOUR MAILGUN ENDPOINT FOR EXAMPLE api.eu.mailgun.net]
MAIL_FROM=[EMAIL SENDER]
SECRET=[API SECRET]
LANGUAGE=en //for now only support en (English) or es (Spanish)
```

5. execute:

`cargo run` or with logs mode `RUST_LOG=dancespiele_api cargo=trace cargo run`

6. Enjoy!

**Note:** Before to execute Dancespiele API you need to run the RabbitMQ server first

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

### Issues

**Dancespiele API doesn't shutdown with Ctrl-C:** For now you need to kill the process to shutdown the API. I'm trying to figure out how to fix this issue

## Do you like Dancespiele apps?
If you like Dancespiele apps, help me supporting the projects:
- Sending coins to the address **0x619d3FA3bD7CF497d9899Ccd4d7b5663Ff318e52**
- BAT rewards in case that you use [Brave Browser](https://brave.com/)
- [Github Sponsors](https://github.com/sponsors/dancespiele)
- Burst coins to the address BURST-DPN6-2AT3-FCRL-9BBKG

## Do you want or you need a feature that is not implemented yet?

You can have the feature that you wish creating the issue in the repository and funding it [here](https://issuehunt.io/r/dancespiele/dancespiele_api?tab=idle) or if you use [Brave Browser](https://brave.com/) also you can give rewards with BAT in the comment issue explanation.
**How much bigger is your fund more priority will be your feature!** Results will be posted in [twitter](https://twitter.com/spielcrypto).
Of course you can contribute with you code as well.

## License
Dancespiele API is [LICENSE PARITY](LICENSE-PARITY.md) and [LICENSE PATRON](LICENSE-PATRON.md) licensed. If you need a comercial license sponsor to Dancespiele in the right tier or contact to `spielcrypto@gmail.com`

**Warning:** Each functionality has unit test to avoid bugs however the author of this app is not responsible for all the issues and losses that can happen using it. Please, read the licenses.
