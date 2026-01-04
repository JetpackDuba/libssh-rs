use crate::{Error, SshResult};
use libssh_rs_sys::{ssh_buffer, ssh_buffer_add_data, ssh_buffer_free, ssh_buffer_get_data, ssh_buffer_get_len, ssh_buffer_new, SSH_OK};

pub struct Buffer {
    pub(crate) buffer: ssh_buffer,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { ssh_buffer_free(self.buffer) }
    }
}

impl Buffer {
    pub fn new() -> SshResult<Self> {
        let buffer = unsafe { ssh_buffer_new() };

        if buffer.is_null() {
            Err(Error::fatal("ssh_buffer_new failed"))
        } else {
            Ok(Self { buffer })
        }
    }

    pub fn add_data(&self, data: &[u8]) -> SshResult<()> {
        let res = unsafe { ssh_buffer_add_data(self.buffer, data.as_ptr() as _, data.len() as _) };
        if res == SSH_OK as i32 {
            Ok(())
        } else {
            Err(Error::fatal("error adding data to buffer"))
        }
    }

    pub fn len(&self) -> u32 {
        unsafe { ssh_buffer_get_len(self.buffer) }
    }

    pub fn get_data(&self, len: u32) -> SshResult<Vec<u8>> {
        let data = Vec::<u8>::with_capacity(len as _);
        let result = unsafe { ssh_buffer_get_data(self.buffer, data.as_ptr() as _, len as _) };

        if (result == SSH_OK) {
            Ok(data)
        } else {
            Err(Error::fatal("error getting data from buffer"))
        }
    }
}
