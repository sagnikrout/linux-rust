//! Automatically rewritten from C to Rust
//! Source: crypto/asymmetric_keys/x509_public_key.c
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
// Instantiate a public key crypto key from an X.509 Certificate
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Set up the signature parameters in an X.509 certificate.  This involves
// digesting the signed data and extracting the signature.
//
#[no_mangle]
pub unsafe extern "C" fn x509_get_sig_params(cert: *mut x509_certificate) -> c_int {
    int x509_get_sig_params(struct x509_certificate *cert)
    {
    struct public_key_signature *sig = cert.sig;
    struct crypto_shash *tfm;
    struct shash_desc *desc;
    size_t desc_size;
    int ret;
    pr_devel("==>%s()\n", __func__);
// Calculate a SHA256 hash of the TBS and check it against the
// blacklist.
//
    sha256(cert.tbs, cert.tbs_size, cert.sha256);
    ret = is_hash_blacklisted(cert.sha256, sizeof(cert.sha256),
    BLACKLIST_HASH_X509_TBS);
    if (ret == -EKEYREJECTED) {
    pr_err("Cert %*phN is blacklisted\n",
    (int)sizeof(cert.sha256), cert.sha256);
    cert.blacklisted = true;
    ret = 0;
    }
    sig.s = kmemdup(cert.raw_sig, cert.raw_sig_size, GFP_KERNEL);
    if (!sig.s)
    return -ENOMEM;
    sig.s_size = cert.raw_sig_size;
    if (sig.algo_takes_data) {
// The signature algorithm does whatever passes for hashing.
    sig.m = (u8 *)cert.tbs;
    sig.m_size = cert.tbs_size;
    sig.m_free = false;
    goto out;
    }
// Allocate the hashing algorithm we're going to need and find out how
// big the hash operational data will be.
//
    tfm = crypto_alloc_shash(sig.hash_algo, 0, 0);
    if (IS_ERR(tfm)) {
    if (PTR_ERR(tfm) == -ENOENT) {
    cert.unsupported_sig = true;
    return 0;
    }
    return PTR_ERR(tfm);
    }
    desc_size = crypto_shash_descsize(tfm) + sizeof(*desc);
    sig.m_size = crypto_shash_digestsize(tfm);
    ret = -ENOMEM;
    sig.m = kmalloc(sig.m_size, GFP_KERNEL);
    if (!sig.m)
    goto error;
    sig.m_free = true;
    desc = kzalloc(desc_size, GFP_KERNEL);
    if (!desc)
    goto error;
    desc.tfm = tfm;
    ret = crypto_shash_digest(desc, cert.tbs, cert.tbs_size, sig.m);
    if (ret < 0)
    goto error_2;
    error_2:
    kfree(desc);
    error:
    crypto_free_shash(tfm);
    out:
    pr_devel("<==%s() = %d\n", __func__, ret);
    return ret;
    }
//
// Check for self-signedness in an X.509 cert and if found, check the signature
// immediately if we can.
//
#[no_mangle]
pub unsafe extern "C" fn x509_check_for_self_signed(cert: *mut x509_certificate) -> c_int {
    int x509_check_for_self_signed(struct x509_certificate *cert)
    {
    let mut ret: c_int = 0;
    pr_devel("==>%s()\n", __func__);
    if (cert.raw_subject_size != cert.raw_issuer_size ||
    memcmp(cert.raw_subject, cert.raw_issuer,
    cert.raw_issuer_size) != 0)
    goto not_self_signed;
    if (cert.sig.auth_ids[0] || cert.sig.auth_ids[1]) {
// If the AKID is present it may have one or two parts.  If
// both are supplied, both must match.
//
    let mut a: bool = asymmetric_key_id_same(cert.skid, cert.sig.auth_ids[1]);
    let mut b: bool = asymmetric_key_id_same(cert.id, cert.sig.auth_ids[0]);
    if (!a && !b)
    goto not_self_signed;
    ret = -EKEYREJECTED;
    if (((a && !b) || (b && !a)) &&
    cert.sig.auth_ids[0] && cert.sig.auth_ids[1])
    goto out;
    }
    if (cert.unsupported_sig) {
    ret = 0;
    goto out;
    }
    ret = public_key_verify_signature(cert.pub, cert.sig);
    if (ret < 0) {
    if (ret == -ENOPKG) {
    cert.unsupported_sig = true;
    ret = 0;
    }
    goto out;
    }
    pr_devel("Cert Self-signature verified");
    cert.self_signed = true;
    out:
    pr_devel("<==%s() = %d\n", __func__, ret);
    return ret;
    not_self_signed:
    pr_devel("<==%s() = 0 [not]\n", __func__);
    return 0;
    }
//
// Attempt to parse a data blob for a key as an X509 certificate.
//
#[no_mangle]
unsafe extern "C" fn x509_key_preparse(prep: *mut key_preparsed_payload) -> c_int {
    static int x509_key_preparse(struct key_preparsed_payload *prep)
    {
    struct x509_certificate *cert __free(x509_free_certificate) = core::ptr::null_mut();
    struct asymmetric_key_ids *kids __free(kfree) = core::ptr::null_mut();
    char *p, *desc __free(kfree) = core::ptr::null_mut();
    const char *q;
    size_t srlen, sulen;
    cert = x509_cert_parse(prep.data, prep.datalen);
    if (IS_ERR(cert))
    return PTR_ERR(cert);
    pr_devel("Cert Issuer: %s\n", cert.issuer);
    pr_devel("Cert Subject: %s\n", cert.subject);
    pr_devel("Cert Key Algo: %s\n", cert.pub.pkey_algo);
    pr_devel("Cert Valid period: %lld-%lld\n", cert.valid_from, cert.valid_to);
    cert.pub.id_type = "X509";
    if (cert.unsupported_sig) {
    public_key_signature_free(cert.sig);
    cert.sig = core::ptr::null_mut();
    } else {
    pr_devel("Cert Signature: %s + %s\n",
    cert.sig.pkey_algo, cert.sig.hash_algo);
    }
// Don't permit addition of blacklisted keys
    if (cert.blacklisted)
    return -EKEYREJECTED;
// Propose a description
    sulen = strlen(cert.subject);
    if (cert.raw_skid) {
    srlen = cert.raw_skid_size;
    q = cert.raw_skid;
    } else {
    srlen = cert.raw_serial_size;
    q = cert.raw_serial;
    }
    desc = kmalloc(sulen + 2 + srlen * 2 + 1, GFP_KERNEL);
    if (!desc)
    return -ENOMEM;
    p = memcpy(desc, cert.subject, sulen);
    p += sulen;
// p++ = ':';
// p++ = ' ';
    p = bin2hex(p, q, srlen);
// p = 0;
    kids = kmalloc_obj(struct asymmetric_key_ids);
    if (!kids)
    return -ENOMEM;
    kids.id[0] = cert.id;
    kids.id[1] = cert.skid;
    kids.id[2] = asymmetric_key_generate_id(cert.raw_subject,
    cert.raw_subject_size,
    "", 0);
    if (IS_ERR(kids.id[2]))
    return PTR_ERR(kids.id[2]);
// We're pinning the module by being linked against it
    __module_get(public_key_subtype.owner);
    prep.payload.data[asym_subtype] = &public_key_subtype;
    prep.payload.data[asym_key_ids] = kids;
    prep.payload.data[asym_crypto] = cert.pub;
    prep.payload.data[asym_auth] = cert.sig;
    prep.description = desc;
    prep.quotalen = 100;
// We've finished with the certificate
    cert.pub = core::ptr::null_mut();
    cert.id = core::ptr::null_mut();
    cert.skid = core::ptr::null_mut();
    cert.sig = core::ptr::null_mut();
    desc = core::ptr::null_mut();
    kids = core::ptr::null_mut();
    return 0;
    }
    static struct asymmetric_key_parser x509_key_parser = {
    .owner	= THIS_MODULE,
    .name	= "x509",
    .parse	= x509_key_preparse,
    };
//
// Module stuff
//
#[no_mangle]
unsafe extern "C" fn x509_key_init() -> int __init {
    static int __init x509_key_init(void)
    {
    return register_asymmetric_key_parser(&x509_key_parser);
    }
#[no_mangle]
unsafe extern "C" fn x509_key_exit() -> void __exit {
    static void __exit x509_key_exit(void)
    {
    unregister_asymmetric_key_parser(&x509_key_parser);
    }
    module_init(x509_key_init);
    module_exit(x509_key_exit);
    MODULE_DESCRIPTION("X.509 certificate parser");
    MODULE_AUTHOR("Red Hat, Inc.");
    MODULE_LICENSE("GPL");
