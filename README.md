# Distance calculator service

Extremely simple service to calculate distance between coordinates on a sphere or ellipsoid.

## Setup

### Using cargo

#### Development

If you want to run development build, you should first create `.env` file, you can just use

```bash
cp .env.example .env
```

to use sensible defaults. Then you can simply run

```bash
cargo run
```

#### Running release build

In order to run release build, the app will expect to have `API_USERNAME` and `API_PASSWORD` environment variables set. You can either export them

```bash
export API_USERNAME=sample_username
export API_PASSWORD=sample_password
```

and then run

```bash
cargo run --release
```

or set them while executing cargo run

```bash
API_USERNAME=sample_username API_PASSWORD=sample_password cargo run --release
```


### Running service in Docker

#### Using compose

In order to run the app in Docker using docker-compose, it should as simple as running

```bash
docker-compose up --build
```

## Available endpoints

* `/docs` - Swagger UI
* `/docs/spec` - Swagger spec
* `/health` - simple healthcheck
* `/api/distance/cordinates` - calculate distance between list of coordinates
* `/api/distance/airports` - calculate distance between list of airports
* `/api/airports/iatas` - returns a list of unique iatas the service knows of

To see full request/response models, refer to Swagger docs.

## Airports Database

For now, the service uses the global airports database taken from https://www.partow.net/miscellaneous/airportdatabase/ version `0.0.2 - 20170321` available in accordance with the MIT License. 
