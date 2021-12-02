mod awaitable;
mod awaitable_responses;

use super::Error;

use std::io::IoSlice;

use awaitable_responses::*;

use openssh_sftp_protocol::constants::SSH2_FILEXFER_VERSION;
use openssh_sftp_protocol::request::Hello;
use openssh_sftp_protocol::response::ServerVersion;
use openssh_sftp_protocol::serde::Deserialize;
use openssh_sftp_protocol::ssh_format::Transformer;

use tokio::io::AsyncReadExt;
use tokio_async_write_utility::AsyncWriteUtility;
use tokio_pipe::{PipeRead, PipeWrite};

pub use openssh_sftp_protocol::request::{CreateFlags, FileMode, OpenFile, RequestInner};
pub use openssh_sftp_protocol::response::{NameEntry, ResponseInner};

#[derive(Debug)]
pub struct Connection {
    writer: PipeWrite,
    reader: PipeRead,
    transformer: Transformer,
    responses: AwaitableResponses,
}
impl Connection {
    async fn read_exact(&mut self, size: usize) -> Result<(), Error> {
        self.transformer.get_buffer().resize(size, 0);
        self.reader
            .read_exact(&mut self.transformer.get_buffer())
            .await?;

        Ok(())
    }

    async fn read_and_deserialize<'a, T>(&'a mut self, size: usize) -> Result<T, Error>
    where
        T: Deserialize<'a>,
    {
        self.read_exact(size).await?;

        // Ignore any trailing bytes to be forward compatible
        Ok(self.transformer.deserialize()?.0)
    }

    async fn negotiate(&mut self) -> Result<(), Error> {
        let version = SSH2_FILEXFER_VERSION;

        // Sent hello message
        let bytes = self.transformer.serialize(Hello {
            version,
            extensions: Default::default(),
        })?;
        self.writer
            .write_vectored_all(&mut [IoSlice::new(bytes)])
            .await?;

        // Receive server version
        let len: u32 = self.read_and_deserialize(4).await?;
        self.read_exact(len as usize).await?;
        let server_version = ServerVersion::deserialize(self.transformer.get_buffer())?;

        if server_version.version != version {
            Err(Error::UnsupportedSftpProtocol)
        } else {
            Ok(())
        }
    }

    pub async fn new(reader: PipeRead, writer: PipeWrite) -> Result<Self, Error> {
        let mut val = Self {
            reader,
            writer,
            transformer: Transformer::default(),
            responses: AwaitableResponses::default(),
        };

        val.negotiate().await?;

        Ok(val)
    }
}
