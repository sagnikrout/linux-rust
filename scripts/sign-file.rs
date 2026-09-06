//! Automatically rewritten from C to Rust
//! Source: scripts/sign-file.c
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


// Sign a module file using the given key.
//
// Copyright © 2014-2016 Red Hat, Inc. All Rights Reserved.
// Copyright © 2015      Intel Corporation.
// Copyright © 2016      Hewlett Packard Enterprise Development LP
//
// Authors: David Howells <dhowells@redhat.com>
// David Woodhouse <dwmw2@infradead.org>
// Juerg Haefliger <juerg.haefliger@hpe.com>
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
    "Usage: scripts/sign-file [-dp] <hash algo> <key> <x509> <module> [<dest>]\n");
    fprintf(stderr,
    "       scripts/sign-file -s <raw sig> <hash algo> <x509> <module> [<dest>]\n");
    exit(2);
    }
    static const char *key_pass;
#[no_mangle]
unsafe extern "C" fn pem_pw_cb(buf: *mut c_char, len: c_int, w: c_int, v: *mut c_void) -> c_int {
    static int pem_pw_cb(char *buf, int len, int w, void *v)
    {
    int pwlen;
    if (!key_pass)
    return -1;
    pwlen = strlen(key_pass);
    if (pwlen >= len)
    return -1;
    strcpy(buf, key_pass);
// If it's wrong, don't keep trying it.
    key_pass = core::ptr::null_mut();
    return pwlen;
    }
    static EVP_PKEY *read_private_key_pkcs11(const char *private_key_name)
    {
    EVP_PKEY *private_key = core::ptr::null_mut();

    OSSL_STORE_CTX *store;
    if (!OSSL_PROVIDER_try_load(core::ptr::null_mut(), "pkcs11", true))
    ERR(1, "OSSL_PROVIDER_try_load(pkcs11)");
    if (!OSSL_PROVIDER_try_load(core::ptr::null_mut(), "default", true))
    ERR(1, "OSSL_PROVIDER_try_load(default)");
    store = OSSL_STORE_open(private_key_name, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    ERR(!store, "OSSL_STORE_open");
    while (!OSSL_STORE_eof(store)) {
    OSSL_STORE_INFO *info = OSSL_STORE_load(store);
    if (!info) {
    drain_openssl_errors(__LINE__, 0);
    continue;
    }
    if (OSSL_STORE_INFO_get_type(info) == OSSL_STORE_INFO_PKEY) {
    private_key = OSSL_STORE_INFO_get1_PKEY(info);
    ERR(!private_key, "OSSL_STORE_INFO_get1_PKEY");
    }
    OSSL_STORE_INFO_free(info);
    if (private_key)
    break;
    }
    OSSL_STORE_close(store);

    ENGINE *e;
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
    private_key = ENGINE_load_private_key(e, private_key_name, core::ptr::null_mut(), core::ptr::null_mut());
    ERR(!private_key, "%s", private_key_name);

    fprintf(stderr, "no pkcs11 engine/provider available\n");
    exit(1);

    return private_key;
    }
    static EVP_PKEY *read_private_key(const char *private_key_name)
    {
    if (!strncmp(private_key_name, "pkcs11:", 7)) {
    return read_private_key_pkcs11(private_key_name);
    } else {
    EVP_PKEY *private_key;
    BIO *b;
    b = BIO_new_file(private_key_name, "rb");
    ERR(!b, "%s", private_key_name);
    private_key = PEM_read_bio_PrivateKey(b, core::ptr::null_mut(), pem_pw_cb,
    core::ptr::null_mut());
    ERR(!private_key, "%s", private_key_name);
    BIO_free(b);
    return private_key;
    }
    }
    static X509 *read_x509(const char *x509_name)
    {
    unsigned char buf[2];
    X509 *x509;
    BIO *b;
    int n;
    b = BIO_new_file(x509_name, "rb");
    ERR(!b, "%s", x509_name);
// Look at the first two bytes of the file to determine the encoding
    n = BIO_read(b, buf, 2);
    if (n != 2) {
    if (BIO_should_retry(b)) {
    fprintf(stderr, "%s: Read wanted retry\n", x509_name);
    exit(1);
    }
    if (n >= 0) {
    fprintf(stderr, "%s: Short read\n", x509_name);
    exit(1);
    }
    ERR(1, "%s", x509_name);
    }
    ERR(BIO_reset(b) != 0, "%s", x509_name);
    if (buf[0] == 0x30 && buf[1] >= 0x81 && buf[1] <= 0x84)
// Assume raw DER encoded X.509
    x509 = d2i_X509_bio(b, core::ptr::null_mut());
    else
// Assume PEM encoded X.509
    x509 = PEM_read_bio_X509(b, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    BIO_free(b);
    ERR(!x509, "%s", x509_name);
    return x509;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut sig_info: module_signature = { .id_type = MODULE_SIGNATURE_TYPE_PKCS7 };
    char *hash_algo = core::ptr::null_mut();
    char *private_key_name = core::ptr::null_mut(), *raw_sig_name = core::ptr::null_mut();
    char *x509_name, *module_name, *dest_name;
    let mut save_sig: bool = false, replace_orig;
    let mut sign_only: bool = false;
    let mut raw_sig: bool = false;
    unsigned char buf[4096];
    unsigned long module_size, sig_size;
    const EVP_MD *digest_algo;
    EVP_PKEY *private_key;
    CMS_ContentInfo *cms = core::ptr::null_mut();
    let mut use_keyid: c_uint = 0;
    X509 *x509;
    BIO *bd, *bm;
    int opt, n;
    OpenSSL_add_all_algorithms();
    ERR_load_crypto_strings();
    ERR_clear_error();
    key_pass = getenv("KBUILD_SIGN_PIN");
    do {
    opt = getopt(argc, argv, "sdpk");
    switch (opt) {
    case 's': raw_sig = true; break;
    case 'p': save_sig = true; break;
    case 'd': sign_only = true; save_sig = true; break;
    case 'k': use_keyid = CMS_USE_KEYID; break;
    case -1: break;
    default: format();
    }
    } while (opt != -1);
    argc -= optind;
    argv += optind;
    if (argc < 4 || argc > 5)
    format();
    if (raw_sig) {
    raw_sig_name = argv[0];
    hash_algo = argv[1];
    } else {
    hash_algo = argv[0];
    private_key_name = argv[1];
    }
    x509_name = argv[2];
    module_name = argv[3];
    if (argc == 5 && strcmp(argv[3], argv[4]) != 0) {
    dest_name = argv[4];
    replace_orig = false;
    } else {
    ERR(asprintf(&dest_name, "%s.~signed~", module_name) < 0,
    "asprintf");
    replace_orig = true;
    }
// Open the module file
    bm = BIO_new_file(module_name, "rb");
    ERR(!bm, "%s", module_name);
    if (!raw_sig) {
// Read the private key and the X.509 cert the PKCS#7 message
// will point to.
//
    private_key = read_private_key(private_key_name);
    x509 = read_x509(x509_name);
// Digest the module data.
    OpenSSL_add_all_digests();
    drain_openssl_errors(__LINE__, 0);
    digest_algo = EVP_get_digestbyname(hash_algo);
    ERR(!digest_algo, "EVP_get_digestbyname");
    unsigned int flags =
    CMS_NOCERTS |
    CMS_NOATTR |
    CMS_PARTIAL |
    CMS_BINARY |
    CMS_DETACHED |
    CMS_STREAM  |
    CMS_NOSMIMECAP |

    CMS_NO_SIGNING_TIME |

    use_keyid;

    if (EVP_PKEY_is_a(private_key, "ML-DSA-44") ||
    EVP_PKEY_is_a(private_key, "ML-DSA-65") ||
    EVP_PKEY_is_a(private_key, "ML-DSA-87")) {
// ML-DSA + CMS_NOATTR is not supported in openssl-3.5
// and before.
//
    flags &= ~CMS_NOATTR;
    }

// Load the signature message from the digest buffer.
    cms = CMS_sign(core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), flags);
    ERR(!cms, "CMS_sign");
    ERR(!CMS_add1_signer(cms, x509, private_key, digest_algo, flags),
    "CMS_add1_signer");
    ERR(CMS_final(cms, bm, core::ptr::null_mut(), flags) != 1,
    "CMS_final");
    if (save_sig) {
    char *sig_file_name;
    BIO *b;
    ERR(asprintf(&sig_file_name, "%s.p7s", module_name) < 0,
    "asprintf");
    b = BIO_new_file(sig_file_name, "wb");
    ERR(!b, "%s", sig_file_name);
    ERR(i2d_CMS_bio_stream(b, cms, core::ptr::null_mut(), 0) != 1,
    "%s", sig_file_name);
    BIO_free(b);
    }
    if (sign_only) {
    BIO_free(bm);
    return 0;
    }
    }
// Open the destination file now so that we can shovel the module data
// across as we read it.
//
    bd = BIO_new_file(dest_name, "wb");
    ERR(!bd, "%s", dest_name);
// Append the marker and the PKCS#7 message to the destination file
    ERR(BIO_reset(bm) < 0, "%s", module_name);
    while ((n = BIO_read(bm, buf, sizeof(buf))),
    n > 0) {
    ERR(BIO_write(bd, buf, n) < 0, "%s", dest_name);
    }
    BIO_free(bm);
    ERR(n < 0, "%s", module_name);
    module_size = BIO_number_written(bd);
    if (!raw_sig) {
    ERR(i2d_CMS_bio_stream(bd, cms, core::ptr::null_mut(), 0) != 1, "%s", dest_name);
    } else {
    BIO *b;
// Read the raw signature file and write the data to the
// destination file
//
    b = BIO_new_file(raw_sig_name, "rb");
    ERR(!b, "%s", raw_sig_name);
    while ((n = BIO_read(b, buf, sizeof(buf))), n > 0)
    ERR(BIO_write(bd, buf, n) < 0, "%s", dest_name);
    BIO_free(b);
    }
    sig_size = BIO_number_written(bd) - module_size;
    sig_info.sig_len = htonl(sig_size);
    ERR(BIO_write(bd, &sig_info, sizeof(sig_info)) < 0, "%s", dest_name);
    ERR(BIO_write(bd, MODULE_SIGNATURE_MARKER, sizeof(MODULE_SIGNATURE_MARKER) - 1) < 0,
    "%s", dest_name);
    ERR(BIO_free(bd) != 1, "%s", dest_name);
// Finally, if we're signing in place, replace the original.
    if (replace_orig)
    ERR(rename(dest_name, module_name) < 0, "%s", dest_name);
    return 0;
    }
