# multisig-server

## Install diesel cli
`cargo install diesel_cli --no-default-features --features mysql`

## Database
`echo DATABASE_URL=mysql://root:@localhost:3306/multisig-server > .env`

## Diesel setup
`diesel migration run`

## Run
```bash
cargo run
```

