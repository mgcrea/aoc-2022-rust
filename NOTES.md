
# ts to rust

## spread operator

```ts
const x = [3, 4];
const y = [5, ...x]; // y is [5, 3, 4]
```

```rs
let x = [3,4];
// iterable
let y = [5].iter().chain(&x);
// indexable
let y: Vec<_> = [5].iter().chain(&x).map(|&x|x).collect();
```
