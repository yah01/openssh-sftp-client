#[allow(unused_imports)]
use crate::*;

#[doc(hidden)]
pub mod unreleased {}

/// # Fixed
///
/// `Drop` implementation to make sure they never panic
/// if tokio runtime is not avilable.
///
///  - [`file::TokioCompatFile::Drop`]
///  - [`dir::ReadDir::Drop`]
#[doc(hidden)]
pub mod v_0_13_7 {}

/// ## Added
///  - Add new option [`SftpOptions::tokio_compat_file_write_limit()`] to set write buffer limit
///    for [`file::TokioCompatFile`].
pub mod v_0_13_6 {}

/// ## Fixed
///  - Fixed #80 [`file::TokioCompatFile`]: Incorrect behavior about `AsyncSeek`
///  - Fixed [`file::TokioCompatFile`]: leave error of exceeding buffer len in `consume` to handle by `BytesMut`
///  - Fixed [`file::TokioCompatFile`]: Implement `PinnedDrop` to poll read and write futures to end,
///    otherwise it would drop the internal request ids too early, causing read task to fail
///    when they should not fail.
///  - Fixed [`fs::ReadDir`]: Implement `PinnedDrop` to poll future stored internally,
///    otherwise it would drop the internal request ids too early, causing read task to fail
///    when they should not fail.
/// ## Added
///  - Add new fn [`Sftp::support_expand_path`] to check if the server supports expand-path extension
///  - Add new fn [`Sftp::support_fsync`] to check if the server supports fsync extension
///  - Add new fn [`Sftp::support_hardlink`] to check if the server supports hardlink extension
///  - Add new fn [`Sftp::support_posix_rename`] to check if the server supports posix-rename extension
///  - Add new fn [`Sftp::support_copy`] to check if the server supports copy extension
pub mod v_0_13_5 {}

/// ## Improved
/// - Fix: change the drop of `OwnedHandle` to wait for the close request in order to
///   avoid invalid response id after closing file
/// - Add log for droping OwnedHandle
///
/// ## Other changes
/// - Add msrv 1.64 in `Cargo.toml`
/// - Bump `edition` to 2021 in `Cargo.toml`
pub mod v_0_13_4 {}

/// ## Improved
///  - If `Sftp` is created using `Sftp::from_session`, then dropping it would
///    also drop the `openssh::RemoteChild` and `openssh::Session` immediately
///    after sftp graceful shutdown is done to prevent any leak.
pub mod v_0_13_3 {}

/// ## Added
///  - `OpensshSession`, which is enabled by feature `openssh`
///  - `SftpAuxiliaryData::ArcedOpensshSession`, which is enabled by feature `openssh`
///  - `Sftp::from_session`, which is enabled by feature `openssh`
///  - Logging support, enabled by feature `tracing`
///
/// ## Improved
///  - Keep waiting on other tasks on failure in [`Sftp::close`]
///    to collect as much information about the failure as possible.
///  - Add [`error::RecursiveError3`] for reting 3 errs in [`Sftp::close`]
pub mod v_0_13_2 {}

/// ## Added
///  - [`SftpAuxiliaryData::PinnedFuture`]
pub mod v_0_13_1 {}

/// ## Fixed
///  - Fixed #62 [`fs::ReadDir`]: Return all entries instead of just a subset.
///
/// ## Added
///  - [`file::File::as_mut_file`]
///  - [`SftpAuxiliaryData`]
///  - [`Sftp::new_with_auxiliary`]
///
/// ## Changed
///  - Remove lifetime from [`file::OpenOptions`].
///  - Remove lifetime from [`file::File`].
///  - Remove lifetime from [`file::TokioCompatFile`].
///  - Remove lifetime from [`fs::Fs`].
///  - Remove lifetime from [`fs::Dir`].
///  - Remove lifetime from [`fs::ReadDir`].
///  - Remove lifetime `'s` from [`fs::DirBuilder`].
///  - [`fs::ReadDir`] now implements `futures_core::{Stream, FusedStream}`
///    instead of the {iterator, slice}-based interface.
///  - Remove `file::File::sftp`.
///  - Remove `file::TokioCompatFile::close`.
///  - [`file::TokioCompatFile::fill_buf`] now takes `self: Pin<&mut Self>`
///    instead of `&mut self`.
///  - [`file::TokioCompatFile::read_into_buffer`] now takes
///    `self: Pin<&mut Self>` instead of `&mut self`.
///
/// ## Other changes
///  - Clarify [`file::File::read`].
///  - Clarify [`file::File::write`].
///  - Clarify [`file::File::write_vectorized`].
///  - Clarify [`file::File::write_zero_copy`].
pub mod v_0_13_0 {}

/// ## Fixed
///  - Fix `read_task`: Order shutdown of flush_task on err/panic
pub mod v_0_12_2 {}

/// ## Fixed
///  - `Sftp::new` now returns future that implemens `Send`
pub mod v_0_12_1 {}

/// ## Changed
///  - Ensure stable api: Create newtype wrapper of UnixTimeStamp (#53)
///
/// ## Other
///  - Bump [`openssh-sftp-error`] to v0.3.0
pub mod v_0_12_0 {}

/// ## Other change
///
/// Bump dep
///  - `ssh_format` to v0.13.0
///  - `openssh_sftp_protocol` to v0.22.0
///  - `openssh_sftp_error` to v0.2.0
///  - `openssh_sftp_client_lowlevel` to v0.3.0
pub mod v_0_11_3 {}

/// ## Other change
///  - Bump `openssh_sftp_client_lowlevel` version and optimize
///    write buffer implementation.
///  - Optimize: Reduce monomorphization
///  - Optimize latency: `create_flush_task` first in `Sftp::new`
///    and write the hello msg ASAP.
pub mod v_0_11_2 {}

/// Nothing has changed from [`v_0_11_0_rc_3`].
///
/// ## Other changes
///  - Dependency [`bytes`] bump to v1.2.0 for its optimizations.
pub mod v_0_11_1 {}

/// Nothing has changed from [`v_0_11_0_rc_3`].
pub mod v_0_11_0 {}

/// ## Changed
///  - Rename `SftpOptions::write_end_buffer_size` to
///    [`SftpOptions::requests_buffer_size`] and improve its
///    documentation.
///  - Rename `SftpOptions::read_end_buffer_size` to
///    [`SftpOptions::responses_buffer_size`] and improve its
///    documentation.
///
/// ## Removed
///  - `SftpOptions::max_read_len`
///  - `SftpOptions::max_write_len`
pub mod v_0_11_0_rc_3 {}

/// ## Fixed
///  - Changelog of v0.11.0-rc.1
///
/// ## Added
///  - [`file::File::copy_all_to`] to copy until EOF.
///    This function is extracted from the old `copy_to`
///    function.
///  - [`file::TokioCompatFile::capacity`]
///  - [`file::TokioCompatFile::reserve`]
///  - [`file::TokioCompatFile::shrink_to`]
///
/// ## Changed
///  - [`file::File::copy_to`] now takes [`std::num::NonZeroU64`]
///    instead of `u64`.
///  - [`file::TokioCompatFile::with_capacity`] does not take
///    `max_buffer_len` anymore.
///
/// ## Removed
///  - `file::DEFAULT_MAX_BUFLEN`
pub mod v_0_11_0_rc_2 {}

/// ## Added
///  - `SftpOptions::write_end_buffer_size`
///  - `SftpOptions::read_end_buffer_size`
///
/// ## Changed
///  - All types now does not have generic parameter `W`
///    except for `Sftp::new`
///
/// ## Removed
///  - Unused re-export `CancellationToken`.
///  - Backward compatibility alias `file::TokioCompactFile`.
///  - `Sftp::try_flush`
///  - `Sftp::flush`
///  - `file::File::max_write_len`
///  - `file::File::max_read_len`
///  - `file::File::max_buffered_write`
///
/// ## Moved
///  - `lowlevel` is now moved to be another crate [openssh_sftp_client_lowlevel].
///  - All items in `highlevel` is now moved into root.
pub mod v_0_11_0_rc_1 {}

/// ## Fixed
///  - Changelog of v0.10.2
pub mod v_0_10_3 {}

/// ## Added
///  - Async fn `lowlevel::WriteEnd::send_copy_data_request`
///  - Async fn `highlevel::file::File::copy_to`
pub mod v_0_10_2 {}

/// ## Fixed
///  - Changelog of v0.10.0
///  - Changelog of v0.9.0
pub mod v0_10_1 {}

/// ## Added
///  - Export mod `highlevel::file`
///  - Export mod `highlevel::fs`
///  - Export mod `highlevel::metadata`
///
/// ## Changed
///  - `lowlevel::WriteEnd` now requires `W: AsyncWrite + Unpin`
///  - `lowlevel::SharedData` now requires `W: AsyncWrite + Unpin`
///  - `lowlevel::ReadEnd` now requires `W: AsyncWrite + Unpin`
///  - `lowlevel::connect` now requires `W: AsyncWrite + Unpin`
///  - `lowlevel::connect_with_auxiliary` now requires `W: AsyncWrite + Unpin`
///  - All types in `highlevel` now requires `W: AsyncWrite + Unpin`
///    except for
///     - the re-exported type `highlevel::CancellationToken`
///     - `highlevel::SftpOptions`
///     - `highlevel::fs::DirEntry`
///     - `highlevel::fs::ReadDir`
///
/// ## Removed
///  - Trait `Writer`.
///  - `lowlevel::WriteEnd::send_write_request_direct_atomic`
///  - `lowlevel::WriteEnd::send_write_request_direct_atomic_vectored`
///  - `lowlevel::WriteEnd::send_write_request_direct_atomic_vectored2`
///  - Export of `highlevel::file::TokioCompactFile`
///  - Export of `highlevel::file::TokioCompatFile`
///  - Export of `highlevel::file::DEFAULT_BUFLEN`
///  - Export of `highlevel::file::DEFAULT_MAX_BUFLEN`
///  - Export of `highlevel::file::File`
///  - Export of `highlevel::file::OpenOptions`
///  - Export of `highlevel::fs::DirEntry`
///  - Export of `highlevel::fs::ReadDir`
///  - Export of `highlevel::fs::Dir`
///  - Export of `highlevel::fs::DirBuilder`
///  - Export of `highlevel::fs::Fs`
///  - Export of `highlevel::metadata::FileType`
///  - Export of `highlevel::metadata::MetaData`
///  - Export of `highlevel::metadata::MetaDataBuilder`
///  - Export of `highlevel::metadata::Permissions`
pub mod v0_10_0 {}

/// ## Removed
///  - `highlevel::Sftp::get_cancellation_token`
///  - `highlevel::Sftp::max_write_len`
///  - `highlevel::Sftp::max_read_len`
///  - `highlevel::Sftp::max_buffered_write`
pub mod v_0_9_0 {}

/// ## Added
///  - Type `highlevel::TokioCompatFile` to Replace
///    `highlevel::TokioCompactFile`.
pub mod v0_8_3 {}

/// ## Fixed
///  - Fix possible panic in `highlevel::max_atomic_write_len`
pub mod v0_8_2 {}

/// ## Added
///  - Reexport `highlevel::CancellationToken`.
pub mod v0_8_1 {}

/// ## Added
///  - Associated function `highlevel::FileType::is_fifo`.
///  - Associated function `highlevel::FileType::is_socket`.
///  - Associated function `highlevel::FileType::is_block_device`.
///  - Associated function `highlevel::FileType::is_char_device`.
///  - Trait `Writer`.
///
/// ## Changed
///  - Replace all use of `tokio_pipe::PipeRead` with generic bound
///    `tokio::io::AsyncRead` + `Unpin`.
///  - Replace all use of `tokio_pipe::PipeWrite` with generic bound
///    `Writer`.
///  - Replace constant `highlevel::MAX_ATOMIC_WRITE_LEN` with
///    non-`const` function `highlevel::max_atomic_write_len`.
///  - Associated function `highlevel::Sftp::fs` now only takes `&self`
///    as parameter.
///
/// ## Removed
///  - Trait `std::os::unix::fs::FileTypeExt` implementation for
///    `highlevel::FileType`.
///  - Trait `std::os::unix::fs::PermissionsExt` implementation for
///    `highlevel::Permissions`.
///  - Associated function `lowlevel::WriteEnd::send_write_request_direct`.
///  - Associated function
///    `lowlevel::WriteEnd::send_write_request_direct_vectored`.
pub mod v0_8_0 {}
