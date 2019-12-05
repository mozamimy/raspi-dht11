# raspi-dht11

My practice implementation to fetch temperature and humidity with the DHT11 sensor.

## Preparation

```sh
docker-compose build
```

## Build

```sh
docker-compose run --rm --user $(id -u):$(id -g) builder cargo build --target armv7-unknown-linux-gnueabihf
```

All code snippets are licensed under CC0 unless otherwise specified.
[![CC0](http://i.creativecommons.org/p/zero/1.0/88x31.png)](http://creativecommons.org/publicdomain/zero/1.0/)
