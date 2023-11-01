# Dashboard Server

## Setting up development env

_To change db connection string, credentials or databse name, just edit **.env** file_

### Database
Starting local database is simple as\
`docker compose up`

### Migrating database
To migrate database you first need to install sqlx cli\
`cargo install sqlx-cli`

Then you can migrate using\
`sqlx migrate run`


