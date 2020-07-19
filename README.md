# rest-shakespearean-pokemon 
The Rest API Server Shakespearean Pokemon 

## Install

To compile this project you must have installed cargo and rust. To install them you could use `rustup` utility.
See: https://www.rust-lang.org/tools/install

To run project you can use `cargo run`.
To run project tests you can use `cargo tests`.

### Docker

If you have an installed Docker environment you could build an image running:
```shell
docker build -t shakespearean-pokemon-api .
``` 
and then running created image with command:
```shell
docker run --rm -d -p8080:8080 -t shakespearean-pokemon-api
```

## Use

This API service has only one route to retrieve a Shakespearean Pokemon description:

`GET /pokemon/{pokemon-name}`

The response body looks like this:
```json
{
  "name": "mew",
  "description": "So rare yond 't is still did doth sayeth to beest a mirage by many experts. Only a few people hath't seen 't worldwide."
}
```

You could invoke this API with curl utility:
```shell
curl --location --request GET 'http://localhost:8080/pokemon/mew'
```
