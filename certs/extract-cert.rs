//! Automatically rewritten from C to Rust
//! Source: certs/extract-cert.c
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


// Extract X.509 certificate in DER form from PKCS#11 or PEM.
//
// Copyright © 2014-2015 Red Hat, Inc. All Rights Reserved.
// Copyright © 2015      Intel Corporation.
//
// Authors: David Howells <dhowells@redhat.com>
// David Woodhouse <dwmw2@infradead.org>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation; either version 2.1
// of the licence, or (at your option) any later version.
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn __attribute__(_arg: (noreturn)) -> static {
    static __attribute__((noreturn))
#[no_mangle]
pub unsafe extern "C" fn format() {
    void format(void)
    {
    fprintf(stderr,
    "Usage: extract-cert <source> <dest>\n");
    exit(2);
    }

    static const char *key_pass;

    static BIO *wb;
    static char *cert_dst;
    static bool verbose;
#[no_mangle]
unsafe extern "C" fn write_cert(x509: *mut X509) {
    static void write_cert(X509 *x509)
    {
    char buf[200];
    if (!wb) {
    wb = BIO_new_file(cert_dst, "wb");
    ERR(!wb, "%s", cert_dst);
    }
    X509_NAME_oneline(X509_get_subject_name(x509), buf, sizeof(buf));
    ERR(!i2d_X509_bio(wb, x509), "%s", cert_dst);
    if (verbose)
    fprintf(stderr, "Extracted cert: %s\n", buf);
    }
    static X509 *load_cert_pkcs11(const char *cert_src)
    {
    X509 *cert = core::ptr::null_mut();

    OSSL_STORE_CTX *store;
    if (!OSSL_PROVIDER_try_load(core::ptr::null_mut(), "pkcs11", true))
    ERR(1, "OSSL_PROVIDER_try_load(pkcs11)");
    if (!OSSL_PROVIDER_try_load(core::ptr::null_mut(), "default", true))
    ERR(1, "OSSL_PROVIDER_try_load(default)");
    store = OSSL_STORE_open(cert_src, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    ERR(!store, "OSSL_STORE_open");
    while (!OSSL_STORE_eof(store)) {
    OSSL_STORE_INFO *info = OSSL_STORE_load(store);
    if (!info) {
    drain_openssl_errors(__LINE__, 0);
    continue;
    }
    if (OSSL_STORE_INFO_get_type(info) == OSSL_STORE_INFO_CERT) {
    cert = OSSL_STORE_INFO_get1_CERT(info);
    ERR(!cert, "OSSL_STORE_INFO_get1_CERT");
    }
    OSSL_STORE_INFO_free(info);
    if (cert)
    break;
    }
    OSSL_STORE_close(store);

    ENGINE *e;
    struct {
    const char *cert_id;
    X509 *cert;
    } parms;
    parms.cert_id = cert_src;
    parms.cert = core::ptr::null_mut();
    ENGINE_load_builtin_engines();
    drain_openssl_errors(__LINE__, 1);
    e = ENGINE_by_id("pkcs11");
    ERR(!e, "Load PKCS#11 ENGINE");
    if (ENGINE_init(e))
    drain_openssl_errors(__LINE__, 1);
    else
    ERR(1, "ENGINE_init");
    if (key_pass)
    ERR(!ENGINE_ctrl_cmd_string(e, "PIN", key_pass, 0), "Set PKCS#11 PIN");
    ENGINE_ctrl_cmd(e, "LOAD_CERT_CTRL", 0, &parms, core::ptr::null_mut(), 1);
    ERR(!parms.cert, "Get X.509 from PKCS#11");
    cert = parms.cert;

    fprintf(stderr, "no pkcs11 engine/provider available\n");
    exit(1);

    return cert;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    char *cert_src;
    char *verbose_env;
    OpenSSL_add_all_algorithms();
    ERR_load_crypto_strings();
    ERR_clear_error();
    verbose_env = getenv("KBUILD_VERBOSE");
    if (verbose_env && strchr(verbose_env, '1'))
    verbose = true;

    key_pass = getenv("KBUILD_SIGN_PIN");

    if (argc != 3)
    format();
    cert_src = argv[1];
    cert_dst = argv[2];
    if (!cert_src[0]) {
// Invoked with no input; create empty file
    FILE *f = fopen(cert_dst, "wb");
    ERR(!f, "%s", cert_dst);
    fclose(f);
    exit(0);
    } else if (!strncmp(cert_src, "pkcs11:", 7)) {
    X509 *cert = load_cert_pkcs11(cert_src);
    ERR(!cert, "load_cert_pkcs11 failed");
    write_cert(cert);
    } else {
    BIO *b;
    X509 *x509;
    b = BIO_new_file(cert_src, "rb");
    ERR(!b, "%s", cert_src);
    while (1) {
    x509 = PEM_read_bio_X509(b, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if (wb && !x509) {
    let mut err: c_ulong = ERR_peek_last_error();
    if (ERR_GET_LIB(err) == ERR_LIB_PEM &&
    ERR_GET_REASON(err) == PEM_R_NO_START_LINE) {
    ERR_clear_error();
    break;
    }
    }
    ERR(!x509, "%s", cert_src);
    write_cert(x509);
    }
    }
    BIO_free(wb);
    return 0;
    }
