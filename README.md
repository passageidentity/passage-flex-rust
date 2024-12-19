![passage-flex-rust](https://storage.googleapis.com/passage-docs/github-md-assets/passage-flex-rust.png)

![crates.io](https://img.shields.io/crates/v/passage_flex.svg) [![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](#) ![GitHub License](https://img.shields.io/github/license/passageidentity/passage-flex-rust)
![Static Badge](https://img.shields.io/badge/Built_by_1Password-grey?logo=1password)

## About

[Passage by 1Password](https://1password.com/product/passage) unlocks the passwordless future with a simpler, more secure passkey authentication experience. Passage handles the complexities of the [WebAuthn API](https://blog.1password.com/what-is-webauthn/), and allows you to implement passkeys with ease.

Use [Passkey Flex](https://docs.passage.id/flex) to add passkeys to an existing authentication experience.

Use [Passkey Complete](https://docs.passage.id/complete) as a standalone passwordless auth solution.

Use [Passkey Ready](https://docs.passage.id/passkey-ready) to determine if your users are ready for passkeys.

### In passage-flex-rust

Use passage-flex-rust to implement Passkey Flex into your Rust backend to authenticate requests and manage users.

| Product                                                                                                                                  | Compatible                                                                                                    |
| ---------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| ![Passkey Flex](https://storage.googleapis.com/passage-docs/github-md-assets/passage-passkey-flex-icon.png) Passkey **Flex**             | ✅                                                                                                            |
| ![Passkey Complete](https://storage.googleapis.com/passage-docs/github-md-assets/passage-passkey-complete-icon.png) Passkey **Complete** | ✖️ For Passkey Complete, check out the [Passkey Complete APIs](https://docs.passage.id/complete/backend-sdks) |
| ![Passkey Ready](https://storage.googleapis.com/passage-docs/github-md-assets/passage-passkey-ready-icon.png) Passkey **Ready**          | ✖️ For Passkey Ready, check out [Authentikit](https://www.npmjs.com/package/@passageidentity/authentikit)     |

## Getting Started

### Check Prerequisites

<p>
 You'll need a free Passage account and a Passkey Flex app set up in <a href="https://console.passage.id/">Passage Console</a> to get started. <br />
 <sub><a href="https://docs.passage.id/home#passage-console">Learn more about Passage Console →</a></sub>
</p>

### Install

```shell
cargo add passage_flex
```

### Import

```rust
use passage_flex::PassageFlex;
```

### Initialize

```rust
let passage_flex = PassageFlex::new(
    std::env::var("YOUR_PASSAGE_APP_ID").unwrap(),
    std::env::var("YOUR_PASSAGE_API_KEY").unwrap(),
);
```

### Go Passwordless

Find more details about Passkey Flex on our [Passkey Flex Documentation](https://docs.passage.id/flex) and [Docs.rs](https://docs.rs/passage_flex/latest/passage_flex/) pages.

## API Reference

### Create a Registration Transaction

To create a transaction to start a user passkey registration, use the `create_register_transaction` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

let external_id = "a unique immutable string that represents your user".to_string();
let passkey_display_name =
    "the label for the user's passkey that they will see when logging in".to_string();

let transaction = passage_flex
    .auth
    .create_register_transaction(external_id, passkey_display_name)
    .await
    .unwrap();
```

### Create an Authentication Transaction

To create a transaction to start a user passkey authentication, use the `create_authenticate_transaction` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

let external_id = "a unique immutable string that represents your user".to_string();

let transaction = passage_flex
    .auth
    .create_authenticate_transaction(external_id)
    .await
    .unwrap();
```

## Verify a Nonce

To verify a nonce that you received from the end of of passkey registration or authentication ceremony, use the `verify_nonce` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

let nonce =
    "a unique single-use value received from the client after a passkey ceremony".to_string();

match passage_flex.auth.verify_nonce(nonce).await {
    Ok(external_id) => {
        // use external_id to do things like generate and send your own auth token
    }
    Err(err) => {
        // nonce was invalid or unable to be verified
    }
}
```

## Retrieve User Info

To retrieve information about a user by their external ID -- which is the unique, immutable ID you supply to associate the Passage user with your user -- use the `get` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

// this is the same value used when creating a transaction
let external_id = your_user.id;

// get user info
let passage_user = passage_flex.user.get(external_id).await.unwrap();
println!("{:?}", passage_user.webauthn_devices);
```

## Retrieve a user's passkey devices

To retrieve information about a user's passkey devices, use the `list_devices` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

// this is the same value used when creating a transaction
let external_id = your_user.id;

// list devices
let passkey_devices = passage_flex.user.list_devices(external_id).await.unwrap();
for device in passkey_devices {
    println!("{}", device.usage_count);
}
```

## Revoke a User's Passkey Device

To revoke a user's passkey device, use the `revoke_device` method.

```rust
use passage_flex::PassageFlex;
use chrono::{Duration, NaiveDate, Utc};

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

// this is the same value used when creating a transaction
let external_id = your_user.id;
let last_year = Utc::now().naive_utc().date() - Duration::days(365);

// list devices
let passkey_devices = passage_flex.user.list_devices(external_id.clone()).await.unwrap();

for device in passkey_devices {
    // revoke old devices that haven't been used in the last year
    let last_login_at_parsed =
        NaiveDate::parse_from_str(&device.last_login_at, "%Y-%m-%dT%H:%M:%S%z").unwrap();

    if last_login_at_parsed < last_year {
        if let Err(err) = passage_flex
            .user
            .revoke_device(external_id.clone(), device.id)
            .await
        {
            // device couldn't be revoked
        }
    }
}
```

## Support & Feedback

We are here to help! Find additional docs, the best ways to get in touch with our team, and more within our [support resources](https://github.com/passageidentity/.github/blob/main/SUPPORT.md).

<br />

---

<p align="center">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://storage.googleapis.com/passage-docs/github-md-assets/passage-by-1password-dark.png">
      <source media="(prefers-color-scheme: light)" srcset="https://storage.googleapis.com/passage-docs/github-md-assets/passage-by-1password-light.png">
      <img alt="Passage by 1Password Logo" src="https://storage.googleapis.com/passage-docs/github-md-assets/passage-by-1password-light.png">
    </picture>
</p>

<p align="center">
    <sub>Passage is a product by <a href="https://1password.com/product/passage">1Password</a>, the global leader in access management solutions with nearly 150k business customers.</sub><br />
    <sub>This project is licensed under the MIT license. See the <a href="LICENSE">LICENSE</a> file for more info.</sub>
</p>
