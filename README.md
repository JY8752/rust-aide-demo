# rust-aide-demo

<https://zenn.dev/jy8752/scraps/4009abce743e9e>

上記のZennスクラップの成果物。

```shell
cargo init --bin --name rust-aide-demo
```

## mise

Install and trust the project:

```shell
mise trust
mise install
```

Examples:

```shell
mise run check
mise run test
mise run ci
mise run run
mise run up
mise run migrate-apply
mise run migrate-diff add_users_table
```

## OpenAPI

After starting the server:

```shell
mise run serve
```

- Swagger UI: `http://127.0.0.1:3000/docs`
- OpenAPI JSON: `http://127.0.0.1:3000/openapi.json`
