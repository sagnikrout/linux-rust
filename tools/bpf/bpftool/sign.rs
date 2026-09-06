//! Automatically rewritten from C to Rust
//! Source: tools/bpf/bpftool/sign.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) 2025 Google LLC.
//

// Macro flag: #define _GNU_SOURCE

pub const OPEN_SSL_ERR_BUF_LEN: c_int = 256;
// Use deprecated in 3.0 ERR_get_error_line_data for openssl < 3

    ERR_get_error_line_data(file, line, data, flags)

#[no_mangle]
unsafe extern "C" fn display_openssl_errors(l: c_int) {
    static void display_openssl_errors(int l)
    {
    char buf[OPEN_SSL_ERR_BUF_LEN];
    const char *file;
    const char *data;
    unsigned long e;
    int flags;
    int line;
    while ((e = ERR_get_error_all(&file, &line, core::ptr::null_mut(), &data, &flags))) {
    ERR_error_string_n(e, buf, sizeof(buf));
    if (data && (flags & ERR_TXT_STRING)) {
    p_err("OpenSSL %s: %s:%d: %s", buf, file, line, data);
    } else {
    p_err("OpenSSL %s: %s:%d", buf, file, line);
    }
    }
    }

    do {						 \
    bool __cond = (cond);			 \
    if (__cond && ERR_peek_error())		 \
    display_openssl_errors(__LINE__);\
    } while (0)
    static EVP_PKEY *read_private_key(const char *pkey_path)
    {
    EVP_PKEY *private_key = core::ptr::null_mut();
    BIO *b;
    b = BIO_new_file(pkey_path, "rb");
    private_key = PEM_read_bio_PrivateKey(b, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    BIO_free(b);
    DISPLAY_OSSL_ERR(!private_key);
    return private_key;
    }
    static X509 *read_x509(const char *x509_name)
    {
    unsigned char buf[2];
    X509 *x509 = core::ptr::null_mut();
    BIO *b;
    int n;
    b = BIO_new_file(x509_name, "rb");
    if (!b)
    goto cleanup;
// Look at the first two bytes of the file to determine the encoding
    n = BIO_read(b, buf, 2);
    if (n != 2)
    goto cleanup;
    if (BIO_reset(b) != 0)
    goto cleanup;
    if (buf[0] == 0x30 && buf[1] >= 0x81 && buf[1] <= 0x84)
// Assume raw DER encoded X.509
    x509 = d2i_X509_bio(b, core::ptr::null_mut());
    else
// Assume PEM encoded X.509
    x509 = PEM_read_bio_X509(b, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    cleanup:
    BIO_free(b);
    DISPLAY_OSSL_ERR(!x509);
    return x509;
    }
#[no_mangle]
pub unsafe extern "C" fn register_session_key(key_der_path: *const c_char) -> __u32 {
    __u32 register_session_key(const char *key_der_path)
    {
    unsigned char *der_buf = core::ptr::null_mut();
    X509 *x509 = core::ptr::null_mut();
    let mut key_id: c_int = -1;
    int der_len;
    if (!key_der_path)
    return key_id;
    x509 = read_x509(key_der_path);
    if (!x509)
    goto cleanup;
    der_len = i2d_X509(x509, &der_buf);
    if (der_len < 0)
    goto cleanup;
    key_id = syscall(__NR_add_key, "asymmetric", key_der_path, der_buf,
    (size_t)der_len, KEY_SPEC_SESSION_KEYRING);
    cleanup:
    X509_free(x509);
    OPENSSL_free(der_buf);
    DISPLAY_OSSL_ERR(key_id == -1);
    return key_id;
    }
#[no_mangle]
pub unsafe extern "C" fn bpftool_prog_sign(opts: *mut bpf_load_and_run_opts) -> c_int {
    int bpftool_prog_sign(struct bpf_load_and_run_opts *opts)
    {
    BIO *bd_in = core::ptr::null_mut(), *bd_out = core::ptr::null_mut();
    EVP_PKEY *private_key = core::ptr::null_mut();
    CMS_ContentInfo *cms = core::ptr::null_mut();
    let mut actual_sig_len: c_long = 0;
    X509 *x509 = core::ptr::null_mut();
    void *data = core::ptr::null_mut();
    size_t data_sz;
    let mut err: c_int = 0;
    data_sz = (size_t)opts.insns_sz + opts.data_sz;
    data = malloc(data_sz);
    if (!data) {
    err = -ENOMEM;
    goto cleanup;
    }
    memcpy(data, opts.insns, opts.insns_sz);
    if (opts.data_sz)
    memcpy((char *)data + opts.insns_sz, opts.data, opts.data_sz);
    bd_in = BIO_new_mem_buf(data, data_sz);
    if (!bd_in) {
    err = -ENOMEM;
    goto cleanup;
    }
    private_key = read_private_key(private_key_path);
    if (!private_key) {
    err = -EINVAL;
    goto cleanup;
    }
    x509 = read_x509(cert_path);
    if (!x509) {
    err = -EINVAL;
    goto cleanup;
    }
    cms = CMS_sign(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    CMS_NOCERTS | CMS_PARTIAL | CMS_BINARY | CMS_DETACHED |
    CMS_STREAM);
    if (!cms) {
    err = -EINVAL;
    goto cleanup;
    }
    if (!CMS_add1_signer(cms, x509, private_key, EVP_sha256(),
    CMS_NOCERTS | CMS_BINARY | CMS_NOSMIMECAP |
    CMS_USE_KEYID | CMS_NOATTR)) {
    err = -EINVAL;
    goto cleanup;
    }
    if (CMS_final(cms, bd_in, core::ptr::null_mut(), CMS_NOCERTS | CMS_BINARY) != 1) {
    err = -EIO;
    goto cleanup;
    }
    if (EVP_Digest(opts.insns, opts.insns_sz, opts.excl_prog_hash,
    &opts.excl_prog_hash_sz, EVP_sha256(), core::ptr::null_mut()) != 1) {
    err = -EIO;
    goto cleanup;
    }
    bd_out = BIO_new(BIO_s_mem());
    if (!bd_out) {
    err = -ENOMEM;
    goto cleanup;
    }
    if (!i2d_CMS_bio_stream(bd_out, cms, core::ptr::null_mut(), 0)) {
    err = -EIO;
    goto cleanup;
    }
    actual_sig_len = BIO_get_mem_data(bd_out, core::ptr::null_mut());
    if (actual_sig_len <= 0) {
    err = -EIO;
    goto cleanup;
    }
    if ((size_t)actual_sig_len > opts.signature_sz) {
    err = -ENOSPC;
    goto cleanup;
    }
    if (BIO_read(bd_out, opts.signature, actual_sig_len) != actual_sig_len) {
    err = -EIO;
    goto cleanup;
    }
    opts.signature_sz = actual_sig_len;
    cleanup:
    BIO_free(bd_out);
    CMS_ContentInfo_free(cms);
    X509_free(x509);
    EVP_PKEY_free(private_key);
    BIO_free(bd_in);
    free(data);
    DISPLAY_OSSL_ERR(err < 0);
    return err;
    }
