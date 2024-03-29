[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/dynamic-token)
[![crates.io](https://img.shields.io/crates/v/dynamic-token.svg)](https://crates.io/crates/dynamic-token)
[![docs.rs](https://docs.rs/dynamic-token/badge.svg)](https://docs.rs/dynamic-token)

# Dynamic Token

Time-sensitive *dynamic tokens* authenticate client applications with the server via a shared API key, timestamp, some random characters and an optional UUID (Universally Unique IDentifier) for a second stage of authorisation. Unlike JWT tokens, dynamic tokens are not reusable and do not validate or maintain a user's session. They serve as a first line of defence against unauthorised requests.

They may supplement JWT tokens as an extra security layer. As dynamic tokens change every millisecond with randomised shuffling of characters, they cannot be easily deconstructed without detailed knowledge of the algorithm.

If the decoded timestamp falls outside a narrow time range, by default 5 minutes, it will be rejected. This allows for reltively long request times and minor discrepancies in system clock times. However, the only time that matters is the initial request, not the time it takes to process and send the response. In theory, the same token would work for this limited period.

### Example

- **Shared API key**: Opus;Magna-897
- **Millisecond timestamp**: 1710105367868
- **Random noise or split characters**: %.,
- **UUID**: 6061f78686f34f52da3ef464
- **Sample encoded token**: OHR5a09wdXM7TWFnbmEtODk3MG10bDAwLDAwcjRfXzExa2JxOGloMnJfdXd6MjU2M2o4LjAwazk=

## Comparison with other authentication systems
- **Static tokens** Many microservices use a single static token. While this may ward off anonymous users, they can be easily inetercepted by analysing network requests sent from the browser to the backend.
- **Access key and client secrets** These are best suited to Web services open to a wide range of consumers on different platforms. The credentials are exposed directly in the header, payload or query string credentials. Access tokens may need to be checked against a local database.
- In **OAuth 2.0**, the client initially presents its static API key to the server to start the authentication process. If successful, the authorisation server issues an access token to the client. This access token serves as a credential that the client can use to access protected resources on behalf of the user.
- **Dynamic tokens**, by contrast, can restrict access to a select group of clients with non-reusable tokens without the need for a handshake or database queries. If user-specific authentication is required, embedded UUIDs can be used for a second database-driven authorisation step.

### Obfuscation rather than one-way encyrption
As base-64 encoding is very common, it would be possible to decode a dynamic token and through brute force guess which parts may be the shared API token by comparing two generated dynamic tokens via a standard base-64 decoding function. However, potential hackers would still need to guess how to decode and reassemble the timestamp and exclude random control characters.

### Customisation Options

At a basic level, dynamic tokens only require a shared API key. Both the server and the client must use compatibile dynamic token algorithms. The Rust crate is ideal for server to server communication as an HTTP header added and authenticated as middleware. The dynamic token does not expose the API key or timestamp in an easily hackable format. Even if a dynamic token is intercepted, it has a limited lifetime.

### Suggested configurations:

#### Lightweight API endpoint with a small payload and simple response

```rust
let options = AuthOptions::new("my_cryptic_shared_api_key").set_tolerance_secs(15);
```
This will work if the client and server clocks are synchronised, share the same API key and the initial HTTP request does not take longer than 15 seconds. Short time-outs help prevent DDOS attacks

#### Heavyweight API endpoint with a large payload such a file upload

```rust
let options = AuthOptions::new("my_cryptic_shared_api_key").set_tolerance_mins(5);
```
In practice, allowing times over 5 minutes is not recommended as large uploads will reach the server in packets and the HTTP header with the token would only be authenticated at the beginning of the file transfer process.

#### Define custom random control characters
```rust
let options = AuthOptions::new("my_cryptic_shared_api_key").set_rand_char_str("%@,.?£$");
```
The server and client must share the same API key and custom split characters. These may be any valid utf-8 characters except for letters, numerals or underscores (_). These characters will be base-64-encoded and thus add to the randomness of the encoded token.

Please note the random character sequence will be invalid if it contains Chinese ideograms, but emojis and mathematical symbols are fine as long as they are not interpreted as Greek letters. 

#### Reequire a valid UUID

A UUID is a universal unique identifier may be any hexadecimal string at least 24 characters in length. The encoder function strips any hyphens. These are used many common data systems such as MongoDB and may be generated in other database systems such MySQL or PostGres that traditionally use decimal integers as primary keys. 

```rust
let options = AuthOptions::new("api_key_with_an_emoji_😎☀︎").check_uuid(true);

// The client generates a key that may be added to the request header
let to_key = to_dynamic_key(&options, Some("5d00012de43dcd165cceb295"));

// The sever decodes the key using the shared API key and control characters
// You may use different options for different endpoints
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
If UUIDs are required, the client must send a valid hexadecimal UUID. The *from_dynamic_token()* function will decode the injected UUID and validate only its presence and format. Any hexadecimal string with at least 24 characters is valid, but 32 character strings are also supported. You can use the extracted UUID for subsequent user-specific authenticatiom.
If the client sends a UUID, but the server does not require it, the UUID component will be ignored.

### Dev Notes
This is an alpha release and will accompany Node JS and Web versions of the same utility.

Version 1.5
- The min length of unencoded UUIDs is 24. Most systems use either 24 or 32 with optional hyphens as separators.
- The encoder now strips hyphens from UUIDs, leaving only the hexademical characters available for validation.
- The crate makes the *encode_base64* and *decode_base64* functions publicly available. These are wrappers for *base64::engine::general_purpose::STANDARD.encode(&str)* and its *decode* sibling.