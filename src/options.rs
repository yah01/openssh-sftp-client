use std::{
    num::{NonZeroU16, NonZeroUsize},
    time::Duration,
};

#[cfg(feature = "__ci-tests")]
use std::num::NonZeroU32;

/// Options when creating [`super::Sftp`].
#[derive(Debug, Copy, Clone, Default)]
pub struct SftpOptions {
    read_end_buffer_size: Option<NonZeroUsize>,
    write_end_buffer_size: Option<NonZeroUsize>,
    flush_interval: Option<Duration>,
    max_pending_requests: Option<NonZeroU16>,
    tokio_compat_file_write_limit: Option<NonZeroUsize>,

    #[cfg(feature = "__ci-tests")]
    max_read_len: Option<NonZeroU32>,
    #[cfg(feature = "__ci-tests")]
    max_write_len: Option<NonZeroU32>,
}

impl SftpOptions {
    /// Create a new [`SftpOptions`].
    pub const fn new() -> Self {
        Self {
            read_end_buffer_size: None,
            write_end_buffer_size: None,
            flush_interval: None,
            max_pending_requests: None,
            tokio_compat_file_write_limit: None,

            #[cfg(feature = "__ci-tests")]
            max_read_len: None,
            #[cfg(feature = "__ci-tests")]
            max_write_len: None,
        }
    }

    /// Set `flush_interval`, default value is 0.5 ms.
    ///
    /// `flush_interval` decides the maximum time your requests would stay
    /// in the write buffer before it is actually sent to the remote.
    ///
    /// If another thread is doing flushing, then the internal `flush_task`
    /// [`super::Sftp`] started would wait for another `flush_interval`.
    ///
    /// Setting it to be larger might improve overall performance by grouping
    /// writes and reducing the overhead of packet sent over network, but it
    /// might also increase latency, so be careful when setting the
    /// `flush_interval`.
    ///
    /// If `flush_interval` is set to 0, then every packet
    /// is flushed immediately.
    ///
    /// NOTE that it is perfectly OK to set `flush_interval` to 0 and
    /// it would not slowdown the program, as flushing is only performed
    /// on daemon.
    #[must_use]
    pub const fn flush_interval(mut self, flush_interval: Duration) -> Self {
        self.flush_interval = Some(flush_interval);
        self
    }

    pub(super) fn get_flush_interval(&self) -> Duration {
        self.flush_interval
            .unwrap_or_else(|| Duration::from_micros(500))
    }

    /// Set `max_pending_requests`.
    ///
    /// If the pending_requests is larger than max_pending_requests, then the
    /// flush task will flush the write buffer without waiting for `flush_interval`.
    ///
    /// It is set to 100 by default.
    #[must_use]
    pub const fn max_pending_requests(mut self, max_pending_requests: NonZeroU16) -> Self {
        self.max_pending_requests = Some(max_pending_requests);
        self
    }

    pub(super) fn get_max_pending_requests(&self) -> u16 {
        self.max_pending_requests
            .map(NonZeroU16::get)
            .unwrap_or(100)
    }

    /// Set the init buffer size for requests.
    /// It is used to store [`bytes::Bytes`] and it will be resized
    /// to fit the pending requests.
    ///
    /// NOTE that sftp uses double buffer for efficient flushing
    /// without blocking the writers.
    ///
    /// It is set to 100 by default.
    #[must_use]
    pub const fn requests_buffer_size(mut self, buffer_size: NonZeroUsize) -> Self {
        self.write_end_buffer_size = Some(buffer_size);
        self
    }

    pub(super) fn get_write_end_buffer_size(&self) -> NonZeroUsize {
        self.write_end_buffer_size
            .unwrap_or_else(|| NonZeroUsize::new(100).unwrap())
    }

    /// Set the init buffer size for responses.
    /// If the header of the response is larger than the buffer, then the buffer
    /// will be resized to fit the size of the header.
    ///
    /// It is set to 1024 by default.
    #[must_use]
    pub const fn responses_buffer_size(mut self, buffer_size: NonZeroUsize) -> Self {
        self.read_end_buffer_size = Some(buffer_size);
        self
    }

    pub(super) fn get_read_end_buffer_size(&self) -> NonZeroUsize {
        self.read_end_buffer_size
            .unwrap_or_else(|| NonZeroUsize::new(1024).unwrap())
    }

    /// Set the write buffer limit for tokio compat file.
    /// If [`crate::file::TokioCompatFile`] has hit the write buffer limit
    /// set here, then it will flush one write buffer and continue
    /// sending (part of) the buffer to the server, which could be buffered.
    ///
    /// It is set to usize::MAX by default.
    #[must_use]
    pub const fn tokio_compat_file_write_limit(mut self, limit: NonZeroUsize) -> Self {
        self.tokio_compat_file_write_limit = Some(limit);
        self
    }

    pub(super) fn get_tokio_compat_file_write_limit(&self) -> usize {
        self.tokio_compat_file_write_limit
            .map(NonZeroUsize::get)
            .unwrap_or(usize::MAX)
    }
}

#[cfg(feature = "__ci-tests")]
impl SftpOptions {
    /// Set `max_read_len`.
    ///
    /// It can be used to reduce `max_read_len`, but cannot be used
    /// to increase `max_read_len`.
    #[must_use]
    pub const fn max_read_len(mut self, max_read_len: NonZeroU32) -> Self {
        self.max_read_len = Some(max_read_len);
        self
    }

    pub(super) fn get_max_read_len(&self) -> Option<u32> {
        self.max_read_len.map(NonZeroU32::get)
    }

    /// Set `max_write_len`.
    ///
    /// It can be used to reduce `max_write_len`, but cannot be used
    /// to increase `max_write_len`.
    #[must_use]
    pub const fn max_write_len(mut self, max_write_len: NonZeroU32) -> Self {
        self.max_write_len = Some(max_write_len);
        self
    }

    pub(super) fn get_max_write_len(&self) -> Option<u32> {
        self.max_write_len.map(NonZeroU32::get)
    }
}

#[cfg(not(feature = "__ci-tests"))]
impl SftpOptions {
    pub(super) const fn get_max_read_len(&self) -> Option<u32> {
        None
    }

    pub(super) const fn get_max_write_len(&self) -> Option<u32> {
        None
    }
}
