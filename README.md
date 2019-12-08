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

# License

MIT
