//! Automatically rewritten from C to Rust
//! Source: crypto/asymmetric_keys/mscode_parser.c
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
// Parse a Microsoft Individual Code Signing blob
//
// Copyright (C) 2014 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Parse a Microsoft Individual Code Signing blob
//
    int mscode_parse(void *_ctx, const void *content_data, size_t data_len,
    size_t asn1hdrlen)
    {
    struct pefile_context *ctx = _ctx;
    content_data -= asn1hdrlen;
    data_len += asn1hdrlen;
    pr_devel("Data: %zu [%*ph]\n", data_len, (unsigned)(data_len),
    content_data);
    return asn1_ber_decoder(&mscode_decoder, ctx, content_data, data_len);
    }
//
// Check the content type OID
//
    int mscode_note_content_type(void *context, size_t hdrlen,
    unsigned char tag,
    const void *value, size_t vlen)
    {
    enum OID oid;
    oid = look_up_OID(value, vlen);
    if (oid == OID__NR) {
    char buffer[50];
    sprint_oid(value, vlen, buffer, sizeof(buffer));
    pr_err("Unknown OID: %s\n", buffer);
    return -EBADMSG;
    }
//
// pesign utility had a bug where it was putting
// OID_msIndividualSPKeyPurpose instead of OID_msPeImageDataObjId
// So allow both OIDs.
//
    if (oid != OID_msPeImageDataObjId &&
    oid != OID_msIndividualSPKeyPurpose) {
    pr_err("Unexpected content type OID %u\n", oid);
    return -EBADMSG;
    }
    return 0;
    }
//
// Note the digest algorithm OID
//
    int mscode_note_digest_algo(void *context, size_t hdrlen,
    unsigned char tag,
    const void *value, size_t vlen)
    {
    struct pefile_context *ctx = context;
    char buffer[50];
    enum OID oid;
    oid = look_up_OID(value, vlen);
    switch (oid) {
    case OID_sha1:
    ctx.digest_algo = "sha1";
    break;
    case OID_sha256:
    ctx.digest_algo = "sha256";
    break;
    case OID_sha384:
    ctx.digest_algo = "sha384";
    break;
    case OID_sha512:
    ctx.digest_algo = "sha512";
    break;
    case OID_sha3_256:
    ctx.digest_algo = "sha3-256";
    break;
    case OID_sha3_384:
    ctx.digest_algo = "sha3-384";
    break;
    case OID_sha3_512:
    ctx.digest_algo = "sha3-512";
    break;
    case OID__NR:
    sprint_oid(value, vlen, buffer, sizeof(buffer));
    pr_err("Unknown OID: %s\n", buffer);
    return -EBADMSG;
    default:
    pr_err("Unsupported content type: %u\n", oid);
    return -ENOPKG;
    }
    return 0;
    }
//
// Note the digest we're guaranteeing with this certificate
//
    int mscode_note_digest(void *context, size_t hdrlen,
    unsigned char tag,
    const void *value, size_t vlen)
    {
    struct pefile_context *ctx = context;
    ctx.digest = kmemdup(value, vlen, GFP_KERNEL);
    if (!ctx.digest)
    return -ENOMEM;
    ctx.digest_len = vlen;
    return 0;
    }
