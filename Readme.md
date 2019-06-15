https://www.hackerrank.com/challenges/maximize-it/problem

#### on K = 3
```
$ time python -OO ./src/main.py

14 min
```

```
$ time pypy3 -OO ./src/main.py

2min 43 sec

```

```
$ time cargo run --release --  -C target-cpu=native -C llvm-args='-cost-kind=latency'

38 sec
```
