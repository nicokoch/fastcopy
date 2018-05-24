# fastcopy
Copy files faster in rust

# Benchmark results
## With fastcopy on same fs (ext4), same files, different sizes
```
test mb_100_fastcopy     ... bench:  37,267,161 ns/iter (+/- 1,729,334)
test mb_100_std          ... bench:  47,247,206 ns/iter (+/- 2,381,765)
test mb_10_fastcopy      ... bench:   3,091,479 ns/iter (+/- 332,671)
test mb_10_std           ... bench:   4,042,377 ns/iter (+/- 355,336)
test small_file_fastcopy ... bench:       7,120 ns/iter (+/- 812)
test small_file_std      ... bench:       7,179 ns/iter (+/- 1,061)
```
## With fastcopy on same fs (ext4), different small files

## Without fastcopy on same fs (ext4) - Atomic check does apply, different small files

## With fastcopy across different mounts - Atomic check does not apply, different small files
