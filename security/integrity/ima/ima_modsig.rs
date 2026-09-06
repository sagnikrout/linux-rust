//! Automatically rewritten from C to Rust
//! Source: security/integrity/ima/ima_modsig.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// IMA support for appraising module-style appended signatures.
//
// Copyright (C) 2019  IBM Corporation
//
// Author:
// Thiago Jung Bauermann <bauerman@linux.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct modsig {
    pub pkcs7_msg: *mut pkcs7_message,
    pub hash_algo: enum hash_algo,
// This digest will go in the 'd-modsig' field of the IMA template.
    pub digest: *const u8,
    pub digest_size: u32,
//
// This is what will go to the measurement list if the template requires
// storing the signature.
//
    pub raw_pkcs7_len: c_int,
    pub __counted_by(raw_pkcs7_len): u8 raw_pkcs7[],
}

//
// ima_read_modsig - Read modsig from buf.
//
// Return: 0 on success, error code otherwise.
//
    int ima_read_modsig(enum ima_hooks func, const void *buf, loff_t buf_len,
    struct modsig **modsig)
    {
    let mut marker_len: usize = strlen(MODULE_SIGNATURE_MARKER);
    const struct module_signature *sig;
    struct modsig *hdr;
    size_t sig_len;
    const void *p;
    int rc;
    if (buf_len <= marker_len + sizeof(*sig))
    return -ENOENT;
    p = buf + buf_len - marker_len;
    if (memcmp(p, MODULE_SIGNATURE_MARKER, marker_len))
    return -ENOENT;
    buf_len -= marker_len;
    sig = (const struct module_signature *)(p - sizeof(*sig));
    rc = mod_check_sig(sig, buf_len, func_tokens[func]);
    if (rc)
    return rc;
    sig_len = be32_to_cpu(sig.sig_len);
    buf_len -= sig_len + sizeof(*sig);
// Allocate sig_len additional bytes to hold the raw PKCS#7 data.
    hdr = kzalloc_flex(*hdr, raw_pkcs7, sig_len);
    if (!hdr)
    return -ENOMEM;
    hdr.raw_pkcs7_len = sig_len;
    hdr.pkcs7_msg = pkcs7_parse_message(buf + buf_len, sig_len);
    if (IS_ERR(hdr.pkcs7_msg)) {
    rc = PTR_ERR(hdr.pkcs7_msg);
    kfree(hdr);
    return rc;
    }
    memcpy(hdr.raw_pkcs7, buf + buf_len, sig_len);
// We don't know the hash algorithm yet.
    hdr.hash_algo = HASH_ALGO__LAST;
// modsig = hdr;
    return 0;
    }
//
// ima_collect_modsig - Calculate the file hash without the appended signature.
// @modsig: parsed module signature
// @buf: data to verify the signature on
// @size: data size
//
// Since the modsig is part of the file contents, the hash used in its signature
// isn't the same one ordinarily calculated by IMA. Therefore PKCS7 code
// calculates a separate one for signature verification.
//
#[no_mangle]
pub unsafe extern "C" fn ima_collect_modsig(modsig: *mut modsig, buf: *const c_void, size: loff_t) {
    void ima_collect_modsig(struct modsig *modsig, const void *buf, loff_t size)
    {
    int rc;
//
// Provide the file contents (minus the appended sig) so that the PKCS7
// code can calculate the file hash.
//
    size -= modsig.raw_pkcs7_len + strlen(MODULE_SIGNATURE_MARKER) +
    sizeof(struct module_signature);
    rc = pkcs7_supply_detached_data(modsig.pkcs7_msg, buf, size);
    if (rc)
    return;
// Ask the PKCS7 code to calculate the file hash.
    rc = pkcs7_get_digest(modsig.pkcs7_msg, &modsig.digest,
    &modsig.digest_size, &modsig.hash_algo);
    }
#[no_mangle]
pub unsafe extern "C" fn ima_modsig_verify(keyring: *mut key, modsig: *const modsig) -> c_int {
    int ima_modsig_verify(struct key *keyring, const struct modsig *modsig)
    {
    return verify_pkcs7_message_sig(core::ptr::null_mut(), 0, modsig.pkcs7_msg, keyring,
    VERIFYING_MODULE_SIGNATURE, core::ptr::null_mut(), core::ptr::null_mut());
    }
    int ima_get_modsig_digest(const struct modsig *modsig, enum hash_algo *algo,
    const u8 **digest, u32 *digest_size)
    {
// algo = modsig->hash_algo;
// digest = modsig->digest;
// digest_size = modsig->digest_size;
    return 0;
    }
    int ima_get_raw_modsig(const struct modsig *modsig, const void **data,
    u32 *data_len)
    {
// data = &modsig->raw_pkcs7;
// data_len = modsig->raw_pkcs7_len;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ima_free_modsig(modsig: *mut modsig) {
    void ima_free_modsig(struct modsig *modsig)
    {
    if (!modsig)
    return;
    pkcs7_free_message(modsig.pkcs7_msg);
    kfree(modsig);
    }
