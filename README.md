# fastcopy
Copy files faster in rust

Update 11.06.2018: The approach taken by this lib has been merged into rustc in https://github.com/rust-lang/rust/pull/50772 .
So there will not be any performance gain by using this library (at least on rust nightly)

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

Notes: There is a substancial performance gain with larger files (even on ext4). This is because we always do one syscall per copy, instead of `2 * (filesize / buffersize)`.

Networked file systems and reflink-based filesystems get a really big performance boost by using copy_file_range.

## With fastcopy on same fs (ext4), different small files
```
test copy_100_small_files_fastcopy ... bench:     717,790 ns/iter (+/- 75,173)
test copy_100_small_files_std      ... bench:     728,245 ns/iter (+/- 78,612)
```
Notes: There is a small performance gain with small files when using copy_file_range compared to read/write.
The big impact happens with large files though.

On some file systems (i.e. BTRFS) copy_file_range is almost a noop (no actual copying is done until the new file is changed).
So the performance impact on those file systems would be large with small files also.

## Without fastcopy on same fs (ext4) - Atomic check does apply, different small files
```
test copy_100_small_files_fastcopy           ... bench:     784,432 ns/iter (+/- 77,554)
test copy_100_small_files_std                ... bench:     783,031 ns/iter (+/- 147,595)
```

Notes: Tested on linux kernel 3.16, where copy_file_range is not available.
The atomic HAS_COPY_FILE_RANGE will be set to false here. This value is only set on the first copy.
So every copy after that will be similar to std's implementation.
Performance impact negligable.

## With fastcopy across different mounts - Atomic check does not apply, different small files
```
test copy_100_small_files_across_fs_fastcopy ... bench:  95,675,727 ns/iter (+/- 5,113,786)
test copy_100_small_files_across_fs_std      ... bench:  83,424,127 ns/iter (+/- 15,141,230)
```

Notes: In this case, copy_file_range constantly returns -EXDEV (validated with strace). Which means we do one extra syscall per copy. This case isn't too common (and the performance impact is negligable for larger files), so it should not be too problematic.

Alternatively, maybe we can implement a solution that detects this (files mounted on different fs') before the syscall is fired an avoid the extra syscall altogether.
