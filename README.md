# Zurl
**Zurl** is a lightweight command-line tool built in Rust that allows you to make HTTP requests directly from your terminal.

## Installation

```
cargo install zurl
```

## Flags

| Flags                      | Description                          |
|----------------------------|--------------------------------------|
| `-H` or `--header`          | To add custom headers to the request |
| `-j` or `--json`            | To send JSON data   |
| `-q` or `--query`           | To add query parameters |
| `-h` or `--help`            | To list down all commands |

## Basic Usage
Once installed, you can use **zurl** from the command line to make HTTP requests.

```
zurl <METHOD> <URL>
```

## `GET` Request

```
zurl GET <URL>
```

## Add `json` to your request

```
zurl POST <URL> --json '{"key":"value"}'
```

## Add custom `headers` to your request

```
zurl GET <URL> -H "Authorization: Bearer YOUR_TOKEN" -H "Custom-Header: Value"
```

## Add `query` parameters to your request

```
zurl GET https://api.example.com/search -q "key1=value1&key2=value2"
```

---

## Contributing
We welcome contributions to Zurl! If you'd like to improve the tool, please feel free to fork the project, submit an issue, or create a pull request.

### Steps to Contribute:

1. Fork the repository.
2. Clone your fork.
3. Make your changes and commit them.
4. Push your changes.
5. Create a pull request.





