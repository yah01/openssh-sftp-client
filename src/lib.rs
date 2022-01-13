//! This crate provides a set of APIs to access the remote filesystem using
//! the sftp protocol and is implemented in pure Rust.
//!
//! It supports sending multiple requests concurrently using [`WriteEnd`]
//! (it can be `clone`d), however receiving responses have to be done sequentially
//! using [`ReadEnd::read_in_one_packet`].
//!
//! To create [`WriteEnd`] and [`ReadEnd`], simply pass the `stdin` and `stdout` of
//! the `sftp-server` launched at remote to [`connect`].
//!
//! This crate supports all operations supported by sftp v3, in additional to
//! the following extensions:
//!  - `WriteEnd::send_limits_request`
//!  - `WriteEnd::send_expand_path_request`
//!  - `WriteEnd::send_fsync_request`
//!  - `WriteEnd::send_hardlink_requst`
//!  - `WriteEnd::send_posix_rename_request`

#![forbid(unsafe_code)]

mod awaitable_responses;
mod awaitables;
mod buffer;
mod connection;
mod error;
mod read_end;
mod write_end;
mod writer;

/// Default size of buffer for up/download in openssh-portable
pub const OPENSSH_PORTABLE_DEFAULT_COPY_BUFLEN: usize = 32768;

/// Default number of concurrent outstanding requests in openssh-portable
pub const OPENSSH_PORTABLE_DEFAULT_NUM_REQUESTS: usize = 64;

/// Minimum amount of data to read at a time in openssh-portable
pub const OPENSSH_PORTABLE_MIN_READ_SIZE: usize = 512;

/// Maximum depth to descend in directory trees in openssh-portable
pub const OPENSSH_PORTABLE_MAX_DIR_DEPTH: usize = 64;

/// Default length of download buffer in openssh-portable
pub const OPENSSH_PORTABLE_DEFAULT_DOWNLOAD_BUFLEN: usize = 20480;

/// Default length of upload buffer in openssh-portable
pub const OPENSSH_PORTABLE_DEFAULT_UPLOAD_BUFLEN: usize = 20480;

pub use awaitable_responses::Id;

pub use buffer::*;

pub use openssh_sftp_protocol::file_attrs::{
    FileAttrs, FileType, Permissions, UnixTimeStamp, UnixTimeStampError,
};
pub use openssh_sftp_protocol::open_options::{CreateFlags, OpenOptions};
pub use openssh_sftp_protocol::request::OpenFileRequest;
pub use openssh_sftp_protocol::response::{
    ErrMsg as SftpErrMsg, ErrorCode as SftpErrorKind, Extensions, Limits, NameEntry,
};
pub use openssh_sftp_protocol::{Handle, HandleOwned};

pub use connection::connect;
pub use error::Error;
pub use read_end::ReadEnd;
pub use write_end::*;

pub use awaitables::{
    AwaitableAttrs, AwaitableData, AwaitableHandle, AwaitableLimits, AwaitableName,
    AwaitableNameEntries, AwaitableStatus, Data,
};
