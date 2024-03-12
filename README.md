# RSBBI

RSBBI is a terminal based Sefaria query tool.

## Commands
### Search
Search is used to search throughout the Sefaria library like so:
```bash
rsbbi search Deuteronomy 31:1-3
```

### Keyword
Keyword is used to find instances of text in the Sefaria library:
```bash
rsbbi keyword two spines
```

#### Other stuff to note
I consulted with my Jewish friend and he said that if the [Tetragrammaton](https://en.wikipedia.org/wiki/Tetragrammaton) is shown on screen, it has to be stored, so I did that. If at any point during `search` the Tetragrammaton appears, it will be logged to `~/.local/state/rsbbi/`. When you end up converting to Christianity, you can disable this feature by compiling without the `tetragrammaton-logging` feature ;)
