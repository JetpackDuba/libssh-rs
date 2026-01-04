use std::ffi::{CStr, CString};

use crate::{Error, SshKey, SshResult};

use crate::pki::PkiContext;
use libssh_rs_sys as sys;

pub enum SignAlgorithm {
    SHA256,
    SHA512,
}

pub enum SignNamespace {
    File,
    Email,
}

pub fn ssh_sign(
    data: &[u8],
    key: SshKey,
    algorithm: SignAlgorithm,
    context: Option<PkiContext>,
    namespace: String,
) -> SshResult<String> {
    let sys_algo = match algorithm {
        SignAlgorithm::SHA256 => sys::sshsig_digest_e::SSHSIG_DIGEST_SHA2_256,

        SignAlgorithm::SHA512 => sys::sshsig_digest_e::SSHSIG_DIGEST_SHA2_512,
    };

    let namespace = CString::new(namespace)?;

    let context = match context {
        None => std::ptr::null_mut(),
        Some(context) => context.context,
    };

    let mut signature_ptr = std::ptr::null_mut();

    let res = unsafe {
        sys::sshsig_sign(
            data.as_ptr() as _,
            data.len(),
            key.key,
            context,
            namespace.as_ptr() as _,
            sys_algo,
            &mut signature_ptr,
        ) as u32
    };

    if res == sys::SSH_OK {
        let signature = unsafe { CStr::from_ptr(signature_ptr) }
            .to_string_lossy()
            .to_string();
        unsafe { sys::ssh_string_free_char(signature_ptr) };

        Ok(signature)
    } else {
        Err(Error::fatal("Signing failed"))
    }
}
