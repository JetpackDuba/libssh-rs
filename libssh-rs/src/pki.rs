use crate::buffer::Buffer;
use crate::{Error, SshResult};
use libssh_rs_sys as sys;
use libssh_rs_sys::ssh_pki_options_e;
use std::ffi::CString;

pub struct PkiContext {
    pub(crate) context: sys::ssh_pki_ctx,
}

impl Drop for PkiContext {
    fn drop(&mut self) {
        unsafe { sys::ssh_pki_ctx_free(self.context) }
    }
}

impl PkiContext {
    pub fn new() -> SshResult<Self> {
        let ctx = unsafe { sys::ssh_pki_ctx_new() };

        if ctx.is_null() {
            Err(Error::fatal("ssh_pki_ctx_new failed"))
        } else {
            let pki_ctx = PkiContext { context: ctx };

            Ok(pki_ctx)
        }
    }

    pub fn set_option(&self, option: PkiOption) -> SshResult<()> {
        let res = unsafe {
            match option {
                PkiOption::RsaKeySize(size) => sys::ssh_pki_ctx_options_set(
                    self.context,
                    ssh_pki_options_e::SSH_PKI_OPTION_RSA_KEY_SIZE,
                    size as _,
                ),
                PkiOption::SkApplication(application) => {
                    let app = CString::new(application)?;

                    sys::ssh_pki_ctx_options_set(
                        self.context,
                        ssh_pki_options_e::SSH_PKI_OPTION_SK_APPLICATION,
                        app.as_ptr() as _,
                    )
                }
                PkiOption::SkFlags(flags) => {
                    sys::ssh_pki_ctx_options_set(
                        self.context,
                        ssh_pki_options_e::SSH_PKI_OPTION_SK_FLAGS,
                        flags as _,
                    )
                }
                PkiOption::SkUserId(user_id) => {
                    let user_id = CString::new(user_id)?;

                    sys::ssh_pki_ctx_options_set(
                        self.context,
                        ssh_pki_options_e::SSH_PKI_OPTION_SK_USER_ID,
                        user_id.as_ptr() as _,
                    )
                }
                PkiOption::SkChallenge(buffer) => {
                    sys::ssh_pki_ctx_options_set(
                        self.context,
                        ssh_pki_options_e::SSH_PKI_OPTION_SK_CHALLENGE,
                        buffer.buffer as _,
                    )
                }
            }
        } as u32;

        if res == sys::SSH_OK {
            Ok(())
        } else {
            Err(Error::fatal("set_option for PkiContext failed"))
        }
    }
}

pub enum PkiOption {
    RsaKeySize(i32),
    SkApplication(String),
    SkFlags(u8),
    SkUserId(String),
    SkChallenge(Buffer),
    // TODO SkCallbacks,
}

pub enum SkFlags {
    UserPresenceRequired,
    UserVerificationRequired,
    ForceOperation,
    ResidentKey,
}
