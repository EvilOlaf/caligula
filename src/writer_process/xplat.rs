use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use crate::compression::CompressionFormat;

#[cfg(target_os = "linux")]
pub fn open_blockdev(path: impl AsRef<Path>, cf: CompressionFormat) -> std::io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    use libc::O_SYNC;

    let mut opts = OpenOptions::new();
    opts.write(true);

    // Decompression is a bigger bottleneck than write, so only bypass the
    // cache if there is compression.
    if cf.is_identity() {
        opts.custom_flags(O_SYNC);
    }

    opts.open(path)
}

#[cfg(target_os = "macos")]
pub fn open_blockdev(path: impl AsRef<Path>, _cf: CompressionFormat) -> std::io::Result<File> {
    let file = OpenOptions::new().write(true).read(true).open(path)?;

    Ok(file)
}
