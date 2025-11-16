#!/bin/bash
# Regenerate the ffi bindings
cat >binding.h <<-EOT
typedef unsigned long size_t;
#include <libssh/libssh.h>
#include <libssh/callbacks.h>
#include <libssh/server.h>
#include <libssh/sftp.h>
#include <libssh/ssh2.h>
#include <libssh/pki.h>
EOT

touch vendored/include/libssh/libssh_version.h

bindgen \
  binding.h \
  -o src/lib.rs \
  --no-layout-tests \
  --no-doc-comments \
  --raw-line "#![allow(non_snake_case)]" \
  --raw-line "#![allow(non_camel_case_types)]" \
  --raw-line "#![allow(non_upper_case_globals)]" \
  --raw-line "#![allow(clippy::unreadable_literal)]" \
  --raw-line "#![allow(clippy::upper_case_acronyms)]" \
  --default-enum-style rust \
  --constified-enum ssh_error_types_e \
  --constified-enum ssh_known_hosts_e \
  --constified-enum ssh_auth_e \
  --constified-enum ssh_keytypes_e \
  --allowlist-type '(sftp|ssh).*' \
  --allowlist-function '(sftp|ssh).*' \
  --allowlist-var 'SSH.*' \
  --verbose \
  -- \
  -Ivendored/include \
  -DHAVE_STRTOULL \
  -DHAVE_COMPILER__FUNC__ \
  -DHAVE_COMPILER__FUNCTION__ \
  -DHAVE_LIBCRYPTO \
  -DHAVE_OPENSSL_AES_H \
  -DHAVE_OPENSSL_BLOWFISH_H \
  -DHAVE_OPENSSL_DES_H \
  -DHAVE_OPENSSL_ECC \
  -DHAVE_OPENSSL_ECDH_H \
  -DHAVE_OPENSSL_ECDSA_H \
  -DHAVE_ECC \
  -DHAVE_DSA \
  -DHAVE_OPENSSL_EC_H \
  -DHAVE_STDINT_H

rm vendored/include/libssh/libssh_version.h
