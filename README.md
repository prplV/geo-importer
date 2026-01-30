# GEO importer

GEO-data importer including weather distribution, pressure level and etc via Weather API like Yandex.Weather.

> import defaultly is a daily process 

## ENV 

```bash
API_KEY = <api-key-for-service>
API_KEY_FIELD = <api-key-header-name>
API_URL = <api-endpoint>
API_NAME = <name-for-job>

PG__HOST = localhost
PG__PORT = 5432
PG__USER = dbuser
PG__PASSWORD = dbpassword
PG__DBNAME = dbname

SELF_PORT = 8081 # or any else 
```

## API 

Specs: 

### Get targets coordinates

```
GET /api/v1/place
```

Example response:

```json
[
    {
        "latitude": 52.37125,
        "longitude": 4.89388
    },
    {
        "latitude": 55.7558,
        "longitude": 37.6173
    }
]
```


### Delete target coordinate

```
DELETE /api/v1/place?lat=123&lon=123
```

### Insert target coordinate

```
POST /api/v1/place?lat=123&lon=123
```
