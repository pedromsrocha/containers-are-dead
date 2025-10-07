# Outro

To deploy your `04_databases` example component to the cloud, first create a SQLite database using
`spin cloud sqlite create` command. You may choose any name for the database you like.

Next we need to create the tables we expected in the database, to do this we will manually execute the statements in the `migration.sql` file using the `spin cloud sqlite execute` command:

```sh
spin cloud sqlite execute --database <your database name> "CREATE TABLE IF NOT EXISTS sessions (session_id INTEGER PRIMARY KEY AUTOINCREMENT);"
spin cloud sqlite execute --database <your database name> "CREATE TABLE IF NOT EXISTS variables (session_id INTEGER, key TEXT NOT NULL, value REAL NOT NULL);"
```

Now - from the `02_http/04_databases` exercise we can deploy the component using `spin deploy`. When prompted to hook up an existing database or create a new one select the one we just created above. `spin deploy` will upload to the Fermyon Cloud which requires GitHub authentication.

Once deployment completes, the CLI will provide a live URL where your WebAssembly application is running on the internet.
