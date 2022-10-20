## Install

### With Rust toolchain

```bash
# supports all databases supported by SQLx
$ cargo install sqlx-cli

# only for postgres
$ cargo install sqlx-cli --no-default-features --features native-tls,postgres

# use vendored OpenSSL (build from source)
$ cargo install sqlx-cli --features openssl-vendored

# use Rustls rather than OpenSSL (be sure to add the features for the databases you intend to use!)
$ cargo install sqlx-cli --no-default-features --features rustls
```

## Usage

All commands require that a database url is provided. This can be done either with the `--database-url` command line option or by setting `DATABASE_URL`, either in the environment or in a `.env` file
in the current working directory.

For more details, run `sqlx <command> --help`.

```dotenv
# Postgres
DATABASE_URL=postgres://postgres@localhost/my_database
```

### Create/drop the database at `DATABASE_URL`

```bash
sqlx database create
sqlx database drop
```

---

### Create and run migrations

```bash
sqlx migrate add -r <name>
```

Creates a new file in `migrations/<timestamp>-<name>.sql`. Add your database schema changes to
this new file.

---

```bash
sqlx migrate run
```

Compares the migration history of the running database against the `migrations/` folder and runs
any scripts that are still pending.

---

Users can provide the directory for the migration scripts to `sqlx migrate` subcommands with the `--source` flag.

```bash
sqlx migrate info --source ../relative/migrations
```

---

### Reverting Migrations

If you would like to create _reversible_ migrations with corresponding "up" and "down" scripts, you use the `-r` flag when creating new migrations:

```bash
$ sqlx migrate add -r <name>
Creating migrations/20211001154420_<name>.up.sql
Creating migrations/20211001154420_<name>.down.sql
```

After that, you can run these as above:

```bash
$ sqlx migrate run
Applied migrations/20211001154420 <name> (32.517835ms)
```

And reverts work as well:

```bash
$ sqlx migrate revert
Applied 20211001154420/revert <name>
```

**Note**: attempting to mix "simple" migrations with reversible migrations with result in an error.
