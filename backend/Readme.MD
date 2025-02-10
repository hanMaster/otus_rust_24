# Axum backend for smart-house

## Start postgresql server docker image:
```
docker run --rm --name pg -p 5432:5432 \
-e POSTGRES_PASSWORD=welcome \
postgres:17
```
## (optional) To have a psql terminal on pg. In another terminal (tab) run psql:
```
docker exec -it -u postgres pg psql
```

## (optional) For pg to print all sql statements. In psql command line started above.
```
ALTER DATABASE postgres SET log_statement = 'all';
```

## dev with watch
```
cargo watch -q -c -w ./src -x run
``` 