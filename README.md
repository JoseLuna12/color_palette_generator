# Generating color palette from images
```
cargo run <image source type> <file>
```
### For internet images
```
cargo run url "https://images.mubicdn.net/images/film/340287/cache-774789-1672938813/image-w1280.jpg"
```
### For local images
```
cargo run file images/dryan.jpeg
```

### Output
The file output is always called output.png

#### Inputs source type
- Image source type
    - url
    - file

Another string would make the program panic.