# Dynamic Token

A Dynamic token authenticates client applications with the server via a shared API key, timestamp, random characters and an optional uuid. Unlike a JWT token, a dynamic token is not reusable and does not validate a user's session. It may supplement a JWT token and provides enhanced security without having to issue a temporary access key. As dynamic tokens change every millisecond with randomised injection of the encoded API key with extra random noise characters in a base-64-encoded string, it is almost impossible to intercept the token.
If the decoded timestamp falls outside a narrow time range, by default 5 minutes, it will be rejected. This allows for long request times and minor discrepancies in system clock times. However, the only time that matters is the initial request, not the time it takes to process and send the response. In theory, the same token would work for this limited period.


## Comparison with other authentication systems

Many systems require a long access key and client secret. While cryptic, they are static and exposed directly in the header, payload or query string. Others require a handshake, where the static API key identfies the client, but issues a temporary access token for subsequent requests. This access token may be valid for an extended user session or only long enough to let the client send a second request to fetch the required data. This complicates data flow between two tightly web applications, especially micro-services and backend APIs for mobile apps and Web applications.

### Customisation Options

At its most basic dynamic tokens only require a shared API key. Both the server and the client must use compatibile dynamic token libaries. The Rust crate is ideal for server to server communication as an HTTP header added and authenticated as middleware. The dynamic token does not expose the API key or timestamp, unless someone has deconstructed the algorithm used to inject and encode these components into the string before base-64 encoding. Even if a dynamic token is intercepted, it has a limited lifetime.

### Suggested configurations:

#### Lightweight API endpoint with a small payload and simple response

```rust
let options = AuthOptions::new("my_cryptic_shared_api_key").set_tolerance_secs(15);
```
This will work if the client and server clocks are synchronised, shared the same API key and request does not take longer than 15 seconds. Short time-outs help prevent DDOS attacks

#### Heavyweight API endpoint with a large payload such a file upload

```rust
let options = AuthOptions::new("my_cryptic_shared_api_key").set_tolerance_mins(5);
```
In practice, allowing times over 5 minutes is not recommended as large uploads will reach the server in packets and the HTTP header with the token would only be authenticated at the beginning of the file transfer process.

#### Define custom randome split characters
```rust
let options = AuthOptions::new("my_cryptic_shared_api_key").set_rand_char_str("%@,.?£$");
```
The server and client must share the same API key and custom split characters. These may be any valid utf-8 characters except for letters, numerals or underscores (_). These characters will be base-64-encoded and thus add to the randomness of the encoded token.

#### Reequire a valid UUID

A UUID is a universal unique identifier, typically a 24 or 32-character-long hexadecimal string. Some systems will add separators, which should stripped. These are used not only in MongoDB, but may be generated in most modern frameworks and in other database systems such MySQL or PostGres.

```rust
let options = AuthOptions::new("my_cryptic_shared_api_key").check_uuid(true);

let to_key = to_dynamic_key(&options, Some("5d00012de43dcd165cceb295"));

let result = from_dynamic_key(&to_key, &options);

if result.valid() {
  // We have a valid result with a hexadecimal string that may be a UUID
  // Let's see if it matches a user in our database
  if let Some(user) = fetch_user_by_id(&result.uuid()).await {
    if user.is_admin() {
      // Ok proceed
    } else {
      // not authorised for this endpoint
    }
  } else {
    // user not found
  }
}

```
If UUIDs are required, the client must send a valid hexadecimal UUID. The from_dynamic_token crate will only decode the injected UUID and validate its presence and format, any hexadecimal string with at least 16 characters is valid even if 24 or 32 character strings are customary. You can perform additional validation with the extract UUID.
If the client sends a UUID, but the server does not require it, the UUID component will be ignored.
