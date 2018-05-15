# fastcopy
Copy files faster in rust

# Benchmark results
test mb_100_fastcopy     ... bench:  39,366,674 ns/iter (+/- 1,930,715)

test mb_100_std          ... bench:  50,037,671 ns/iter (+/- 2,120,651)

test mb_10_fastcopy      ... bench:   3,335,938 ns/iter (+/- 516,089)

test mb_10_std           ... bench:   4,505,315 ns/iter (+/- 503,460)

test small_file_fastcopy ... bench:       7,513 ns/iter (+/- 987)

test small_file_std      ... bench:       7,542 ns/iter (+/- 580)
