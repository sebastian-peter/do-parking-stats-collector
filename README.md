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

    For usage with diesel, the host name has to be localhost.

2. Create and start the docker containers. Required databases and tables are created automatically.

    ```shell
    docker compose up -d
    ```
