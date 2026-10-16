## About the moved
This app simply runs in place to redirect the traffic to new location, expecily when you migrate to new domain name for the app.
```
---> Browser / http client ------------> Moved app 
                                            |
      ---  HTTP 301/308   <------------------
      |     Location
      |--------------------------------------------> New location

```

## How to use this app

### Build the app.
1. If you do not have cargo tool chain installed. Please follow: https://rust-lang.org/tools/install/
1. Build this project by running `cargo build`. For release version please run `cargo build --release` then you main find the binary from `target/release/moved`

### Arguments
1. `--addr`, `-a` the listen address for this app. The default value is `127.0.0.1:8080`. This won't accept the remote connection. for remote connection please use the public IP of the machine, or listen on `0.0.0.0:8080`.
1. `--code`, `-c` the http return code to the client. The default value is `301` - `Moved Permanently`. You may also do `308`- `Permanent Redirect`. Or any other customized code.
1. `--location`, `-l` the target URL that the traffic eventually redirects to.
1. `--message`, `-l` the optional message that returns to the browser or client.

### Usecases
1. Returns HTTP 301 Moved Permantently:
   ```
   moved --addr 0.0.0.0:8080 --location https://www.google.com
   ```
1. Retruns HTTP 308 Permanent Redirect:
   ```
   moved --addr 0.0.0.0:8080 --location https://www.google.com -c 308
   ```
