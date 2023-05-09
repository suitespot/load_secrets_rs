# load_secrets_rs

Load AWS secrets and produce an evalable output

# Usage

Run the command in your `Dockerfile` before starting the main application code.

```
if [ ! -z "$AWS_SECRETS_ID" ]; then
  eval `load_secrets`
fi
```

# Add to containers

Add to your `Dockerfile`

```
RUN GIT_TAG=v0.4.0 curl -L -o /bin/load_secrets https://github.com/suitespot/load_secrets_rs/releases/download/$GIT_TAG/load_secrets_x86_64-unknown-linux-musl && chmod +x /bin/load_secrets
```

# Build

Build a static binary with musl libc. This output target can only be used if non of the crates have a dependency on glibc

```
cargo build --target x86_64-unknown-linux-musl --release
```
