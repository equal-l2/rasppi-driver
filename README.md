# Rasppi-driver

[![Build Status](https://travis-ci.org/equal-l2/rasppi-driver.svg?branch=master)](https://travis-ci.org/equal-l2/rasppi-driver)

Motor controller via web interface.

## Prerequisites
- Rust nightly toolchain

## Config
Set pin numbers by GPIO.toml.  
Run with `cargo run`.

## TLS Keys
This software requires TLS keys to run.  
For testing purpose, you can generate keys with OpenSSL via `keygen.sh` or by executing the following command:
```
openssl req -new -newkey rsa:2048 -x509 -sha256 -days 365 -nodes -out rocketcert.crt -keyout rocketpri.key
```

## Javascript dependency
We use [JQuery](https://github.com/jquery/jquery) and [turnBox.js](https://github.com/nohtcoltd/turnbox_js) as merged and minified form `bundle.js`, which is made with uglify.js.  
Generally you don't need to make it by yourself as we vendor it, but you can via `make-bundle-js.sh` if you need.  
