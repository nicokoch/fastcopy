extern crate libc;

use std::io;
use std::path::Path;

/// Copies the contents of one file to another. This function will also
/// copy the permission bits of the original file to the destination file.
///
/// This function will **overwrite** the contents of `to`.
///
/// Note that if `from` and `to` both point to the same file, then the file
/// will likely get truncated by this operation.
///
/// On success, the total number of bytes copied is returned and it is equal to
/// the length of the `to` file as reported by `metadata`.
///
/// # Platform-specific behavior
///
/// This function currently corresponds to the `open` function in Unix
/// with `O_RDONLY` for `from` and `O_WRONLY`, `O_CREAT`, and `O_TRUNC` for `to`.
/// `O_CLOEXEC` is set for returned file descriptors.
/// On Windows, this function currently corresponds to `CopyFileEx`. Alternate
/// NTFS streams are copied but only the size of the main stream is returned by
/// this function.
/// Note that, this [may change in the future][changes].
///
/// [changes]: ../io/index.html#platform-specific-behavior
///
/// # Errors
///
/// This function will return an error in the following situations, but is not
/// limited to just these cases:
///
/// * The `from` path is not a file.
/// * The `from` file does not exist.
/// * The current process does not have the permission rights to access
///   `from` or write `to`.
///
/// # Examples
///
/// ```no_run
/// extern crate fastcopy;
/// use std::fs;
///
/// fn main() -> std::io::Result<()> {
///     fastcopy::copy("foo.txt", "bar.txt")?;  // Copy foo.txt to bar.txt
///     Ok(())
/// }
/// ```
pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
    copy_impl::copy(from, to)
}

#[cfg(target_os = "linux")]
mod copy_impl {
    use libc;
    use std::fs;
    use std::io;
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::io::AsRawFd;
    use std::path::Path;
    use std::ptr;

    // https://github.com/rust-lang/rust/blob/6ccfe68076abc78392ab9e1d81b5c1a2123af657/src/libstd/sys_common/io.rs
    const STD_DEFAULT_BUF_SIZE: usize = 8 * 1024;

    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
        let to = to.as_ref();
        let from = from.as_ref();
        if !from.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "the source path is not an existing regular file",
            ));
        }

        let mut reader = fs::File::open(from)?;
        let mut writer = fs::File::create(to)?;
        let perm = reader.metadata()?.permissions();
        let len = {
            let metadata = reader.metadata()?;
            metadata.size() as libc::size_t
        };

        if len < STD_DEFAULT_BUF_SIZE {
            // copy_file_range is usually only useful if we can save system calls
            // except on BTRFS/NFS, were we c_f_r can leverage some fs internals
            // TODO detect if we are on BTRFS or network fs
            return io::copy(&mut reader, &mut writer);
        }

        let mut written = 0u64;
        unsafe {
            while written < len as u64 {
                match copy_file_range(
                    reader.as_raw_fd(),
                    ptr::null_mut(),
                    writer.as_raw_fd(),
                    ptr::null_mut(),
                    len,
                    0,
                ) {
                    ret if ret >= 0 => written += ret as u64,
                    ret if ret == -1 => {
                        let err = io::Error::last_os_error();
                        if err.raw_os_error().unwrap() == libc::ENOSYS {
                            // Kernel does not support copy_file_range
                            // Fallback to std implementation
                            return io::copy(&mut reader, &mut writer);
                        } else {
                            return Err(err);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        fs::set_permissions(&to, perm)?;
        Ok(written)
    }

    unsafe fn copy_file_range(
        fd_in: libc::c_int,
        off_in: *mut libc::loff_t,
        fd_out: libc::c_int,
        off_out: *mut libc::loff_t,
        len: libc::size_t,
        flags: libc::c_uint,
    ) -> libc::ssize_t {
        libc::syscall(
            libc::SYS_copy_file_range,
            fd_in,
            off_in,
            fd_out,
            off_out,
            len,
            flags,
        ) as libc::ssize_t
    }
}

#[cfg(not(target_os = "linux"))]
mod copy_impl {
    use std::fs;
    use std::io;
    use std::path::Path;

    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
        fs::copy(from, to)
    }
}

// #[cfg(not(target_os = "linux"))]
// mod fcopy {
// use std::fs;
// use std::io;
// use std::path::Path;
//     pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
//         fs::copy(from, to)
//     }
// }
