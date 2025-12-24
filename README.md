<h1 align="center">Lychee API</h1>

> [!WARNING]
> The API is currently down at <a>https://lychee-engine.pages.dev/</a> because it was hosted on the community tier of <a>https://www.shuttle.dev/</a> which they have sunsetted. I have no plans for further maintaining the API anytime soon so feel free to fork it and modify on your own.

<p>This repository contains the source code for Lychee API</p>
<p>The code works the same as mentioned in <a>https://lychee-engine.pages.dev/</a></p>

<h3>Endpoints</h3>

Route | Description | Example
------|-------------|--------
index | returns the number of documents | ```localhost:3000/index```
linearsearch | returns the search results | ```localhost:3000/linearsearch?name=pokemon&platform=Gameboy Advance```
indexedsearch | returns the search results | ```localhost:3000/indexedsearch?name=pokemon&platform=Gameboy Advance```

<h4>Search Parameters</h4>

Parameter | Description | Required | Example |
----------|-------------|----------|---------|
name | name of the game | Yes | ```localhost:3000/search?name=pokemon```
platform | platform of the game | Yes | ```localhost:3000/search?name=pokemon&platform=Gameboy Advance```
Limit | number of results to return | No | ```localhost:3000/search?name=pokemon&platform=Gameboy Advance&limit=10```

<h3>Setup</h3>

1. Put your MongoDB URI in the fetch_games function in ```lib.rs``` file.

```rust
async fn fetch_games() -> Result<HashMap<String, Item>, Box<dyn Error>> {

    let uri= "Your MongoDB URI";

    let client_options= ClientOptions::parse_with_resolver_config(uri, ResolverConfig::cloudflare()).await?;
    let client= Client::with_options(client_options)?;

    let collection= client.database("Game_Cache").collection::<Item>("Games");

    let mut cursor= collection.find(None, None).await?;

    let mut cache= HashMap::new();

    while let Some(document)= cursor.try_next().await.unwrap() {

        let id= document._id.to_string();

        cache.insert(id, document);

    }

    return Ok(cache);

}
```

3. Run the main file, the API will run on ```localhost:3000```