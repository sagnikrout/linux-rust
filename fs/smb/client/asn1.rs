//! Automatically rewritten from C to Rust
//! Source: fs/smb/client/asn1.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-or-later

    int
    decode_negTokenInit(unsigned char *security_blob, int length,
    struct TCP_Server_Info *server)
    {
    if (asn1_ber_decoder(&cifs_spnego_negtokeninit_decoder, server,
    security_blob, length) == 0)
    return 1;
    else
    return 0;
    }
    int cifs_gssapi_this_mech(void *context, size_t hdrlen,
    unsigned char tag, const void *value, size_t vlen)
    {
    enum OID oid;
    oid = look_up_OID(value, vlen);
    if (oid != OID_spnego) {
    char buf[50];
    sprint_oid(value, vlen, buf, sizeof(buf));
    cifs_dbg(FYI, "Error decoding negTokenInit header: unexpected OID %s\n",
    buf);
    return -EBADMSG;
    }
    return 0;
    }
    int cifs_neg_token_init_mech_type(void *context, size_t hdrlen,
    unsigned char tag,
    const void *value, size_t vlen)
    {
    struct TCP_Server_Info *server = context;
    enum OID oid;
    oid = look_up_OID(value, vlen);
    if (oid == OID_mskrb5)
    server.sec_mskerberos = true;
#[no_mangle]
pub unsafe extern "C" fn if(OID_krb5u2u: oid ==) -> else {
    else if (oid == OID_krb5u2u)
    server.sec_kerberosu2u = true;
#[no_mangle]
pub unsafe extern "C" fn if(OID_krb5: oid ==) -> else {
    else if (oid == OID_krb5)
    server.sec_kerberos = true;
#[no_mangle]
pub unsafe extern "C" fn if(OID_ntlmssp: oid ==) -> else {
    else if (oid == OID_ntlmssp)
    server.sec_ntlmssp = true;
#[no_mangle]
pub unsafe extern "C" fn if(OID_IAKerb: oid ==) -> else {
    else if (oid == OID_IAKerb)
    server.sec_iakerb = true;
    else {
    char buf[50];
    sprint_oid(value, vlen, buf, sizeof(buf));
    cifs_dbg(FYI, "Decoding negTokenInit: unsupported OID %s\n",
    buf);
    }
    return 0;
    }
