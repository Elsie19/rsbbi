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

---

<img src="https://camo.githubusercontent.com/e704c2ea3f05768971b48a30a7372daa4aaba4ea36327ae485ccd0ed76af3830/68747470733a2f2f7777772e736566617269612e6f72672f7374617469632f696d672f706f77657265642d62792d736566617269612d62616467652e706e673f6d" width=200 alt="Powered By Sefaria badge"/>
