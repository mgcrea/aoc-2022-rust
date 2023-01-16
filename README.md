# aoc-rust-2022

```sh
export DAY=04
aoc download --overwrite --day ${DAY} --input-file src/inputs/${DAY}.txt --puzzle-file src/puzzles/${DAY}.md
touch src/examples/${DAY}.txt
```

```sh
export DAY=04
DEBUG=* cargo watch --clear --exec "test --bin ${DAY}"
DEBUG=* cargo watch --clear --exec run
```
