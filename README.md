# passage_flex

This Rust crate allows for verification of server-side authentication for applications using [Passage Passkey Flex](https://passage.id).

Install this crate using cargo:

```shell
cargo add passage_flex
```

## Create a PassageFlex instance

A Passage AppID and API key are required. The API key can be created in the Passage Console under your Application Settings. This API key grants access to the Passage management APIs to get and update information about users. This API key must be protected and stored in an appropriate secure storage location. It should never be hard-coded in the repository.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);
```

## Retrieve app info

To retrieve information about the app, use the `get_app` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

let app_info = passage_flex.get_app().await.unwrap();
println!("{}", app_info.auth_origin);
```

## Create a registration transaction

To create a transaction to kick off a user passkey registration, use the `create_register_transaction` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

let transaction = passage_flex
    .create_register_transaction(
        "a unique immutable string that represents your user".to_string(),
        "the label for the user's passkey that they will see when logging in".to_string(),
    )
    .await
    .unwrap();
```

## Create an authentication transaction

To create a transaction to kick off a user passkey authentication, use the `create_authenticate_transaction` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

let transaction = passage_flex
    .create_authenticate_transaction(
        "a unique immutable string that represents your user".to_string(),
    )
    .await
    .unwrap();
```

## Verify a nonce

To verify a nonce that you received from the end of of passkey registration or authentication ceremony, use the `verify_nonce` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

match passage_flex.verify_nonce("nonce".to_string()).await {
    Ok(external_id) => {
        // use external_id to do things like generate and send your own auth token
    }
    Err(err) => {
        // nonce was invalid or unable to be verified
    }
}
```

## Retrieve user info

To retrieve information about a user by their external ID -- which is the unique, immutable ID you supply to associate the Passage user with your user -- use the `get_user` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

// this should be the same value you used when creating the transaction
let external_id = your_user.id;

// get user info
let user_info = passage_flex.get_user(external_id).await.unwrap();
println!("{:?}", user_info.webauthn_devices);
```

## Retrieve a user's passkey devices

To retrieve information about a user's passkey devices, use the `get_devices` method.

```rust
use passage_flex::PassageFlex;

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

// this should be the same value you used when creating the transaction
let external_id = your_user.id;

// get devices
let passkey_devices = passage_flex.get_devices(external_id).await.unwrap();
for device in passkey_devices {
    println!("{}", device.usage_count);
}
```

## Revoke a user's passkey device

To revoke a user's passkey device, use the `revoke_device` method.

```rust
use passage_flex::PassageFlex;
use chrono::{Duration, NaiveDate, Utc};

let passage_flex = PassageFlex::new(
    std::env::var("PASSAGE_APP_ID").unwrap(),
    std::env::var("PASSAGE_API_KEY").unwrap(),
);

// this should be the same value you used when creating the transaction
let external_id = your_user.id;
let last_year = Utc::now().naive_utc().date() - Duration::days(365);

// get devices
let passkey_devices = passage_flex.get_devices(external_id.clone()).await.unwrap();

for device in passkey_devices {
    // revoke old devices that haven't been used
    let last_login_at_parsed =
        NaiveDate::parse_from_str(&device.last_login_at, "%Y-%m-%dT%H:%M:%S%z").unwrap();

    if last_login_at_parsed < last_year {
        if let Err(err) = passage_flex
            .revoke_device(external_id.clone(), device.id)
            .await
        {
            // device couldn't be revoked
        }
    }
}
```
