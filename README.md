# do-parking-stats-collector
Collects live statistics about occupied parking lots in Dortmund's parking garages from the official [Open Data API](https://open-data.dortmund.de/explore/dataset/parkhauser/information).
Parking statistics are stored in a postgres database.

## Installation

1. Create an `.env` file with variables used by postgres and collector containers:

    ```dotenv
    POSTGRES_USER=<name>
    POSTGRES_PASSWORD=<password>
    
    CONNECTION_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:5432
    DATABASE_NAME=do_parking
    DATABASE_URL=${CONNECTION_URL}/${DATABASE_NAME}
    ```

2. Create and start the docker containers. Required databases and tables are created automatically.

    ```shell
    docker compose up -d
    ```

## Bonus: Backups

- Create backup

    ```shell
    docker exec -t do-parking-stats-db-1 pg_dumpall -c -U db_user > dump_`date +%Y-%m-%d"_"%H_%M_%S`.sql
    ```

- Restore from backup (drops and replaces all data!)

    ```shell
    cat dump_<date>.sql | docker exec -i do-parking-stats-db-1 psql -U db_user
    ```
  