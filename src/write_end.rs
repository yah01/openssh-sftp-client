use super::connection::SharedData;
use super::Error;
use super::Id;
use super::Response;
use super::ToBuffer;

use core::fmt::Debug;
use core::marker::Unpin;

use std::sync::Arc;

use openssh_sftp_protocol::extensions::Extensions;
use openssh_sftp_protocol::request::{Hello, Request, RequestInner};
use openssh_sftp_protocol::serde::Serialize;
use openssh_sftp_protocol::ssh_format::Serializer;

use std::io::IoSlice;
use tokio::io::AsyncWrite;
use tokio_io_utility::AsyncWriteUtility;

/// TODO:
///  - Support IoSlice for data in `send_write_request`

#[derive(Debug)]
pub struct WriteEnd<Writer: AsyncWrite + Unpin, Buffer: ToBuffer + 'static> {
    serializer: Serializer,
    shared_data: Arc<SharedData<Writer, Buffer>>,
}

impl<Writer: AsyncWrite + Unpin, Buffer: ToBuffer + Debug + Send + Sync + 'static> Clone
    for WriteEnd<Writer, Buffer>
{
    fn clone(&self) -> Self {
        Self::new(self.shared_data.clone())
    }
}

impl<Writer: AsyncWrite + Unpin, Buffer: ToBuffer + Debug + Send + Sync + 'static>
    WriteEnd<Writer, Buffer>
{
    pub(crate) fn new(shared_data: Arc<SharedData<Writer, Buffer>>) -> Self {
        Self {
            serializer: Serializer::new(),
            shared_data,
        }
    }

    pub(crate) async fn send_hello(
        &mut self,
        version: u32,
        extensions: Extensions,
    ) -> Result<(), Error> {
        self.write(Hello {
            version,
            extensions,
        })
        .await
    }

    async fn write<T>(&mut self, value: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        self.serializer.reset();
        value.serialize(&mut self.serializer)?;

        let mut writer = self.shared_data.writer.lock().await;

        writer
            .write_vectored_all(&mut [IoSlice::new(self.serializer.get_output()?)])
            .await?;

        Ok(())
    }

    /// Create a useable response id.
    pub fn create_response_id(&self) -> Id<Buffer> {
        self.shared_data.responses.insert(None)
    }

    /// Send requests.
    ///
    /// **Please use `Self::send_write_request` for sending write requests.**
    pub async fn send_request(
        &mut self,
        id: &mut Id<Buffer>,
        request: RequestInner<'_>,
        buffer: Option<Buffer>,
    ) -> Result<Response<Buffer>, Error> {
        id.reset(buffer);

        self.write(Request {
            request_id: id.slot(),
            inner: request,
        })
        .await?;

        Ok(id.wait().await)
    }

    async fn send_write_request_impl(
        &mut self,
        request_id: u32,
        handle: &[u8],
        offset: u64,
        data: &[u8],
    ) -> Result<(), Error> {
        let header = Request::serialize_write_request(
            &mut self.serializer,
            request_id,
            handle,
            offset,
            data.len().try_into().map_err(|_| Error::BufferTooLong)?,
        )?;

        let mut writer = self.shared_data.writer.lock().await;

        let mut slices = [IoSlice::new(header), IoSlice::new(data)];
        writer.write_vectored_all(&mut slices).await?;

        Ok(())
    }

    /// Send write requests
    pub async fn send_write_request(
        &mut self,
        id: &mut Id<Buffer>,
        handle: &[u8],
        offset: u64,
        data: &[u8],
    ) -> Result<Response<Buffer>, Error> {
        self.send_write_request_impl(id.slot(), handle, offset, data)
            .await?;

        Ok(id.wait().await)
    }
}
