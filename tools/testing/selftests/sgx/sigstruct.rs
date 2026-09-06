//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/sgx/sigstruct.c
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


// SPDX-License-Identifier: GPL-2.0
// Copyright(c) 2016-20 Intel Corporation.
// Macro flag: #define _GNU_SOURCE

//
// FIXME: OpenSSL 3.0 has deprecated some functions. For now just ignore
// the warnings.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q1q2_ctx {
    pub bn_ctx: *mut BN_CTX,
    pub m: *mut BIGNUM,
    pub s: *mut BIGNUM,
    pub q1: *mut BIGNUM,
    pub qr: *mut BIGNUM,
    pub q2: *mut BIGNUM,
}

#[no_mangle]
unsafe extern "C" fn free_q1q2_ctx(ctx: *mut q1q2_ctx) {
    static void free_q1q2_ctx(struct q1q2_ctx *ctx)
    {
    BN_CTX_free(ctx.bn_ctx);
    BN_free(ctx.m);
    BN_free(ctx.s);
    BN_free(ctx.q1);
    BN_free(ctx.qr);
    BN_free(ctx.q2);
    }
    static bool alloc_q1q2_ctx(const uint8_t *s, const uint8_t *m,
    struct q1q2_ctx *ctx)
    {
    ctx.bn_ctx = BN_CTX_new();
    ctx.s = BN_bin2bn(s, SGX_MODULUS_SIZE, core::ptr::null_mut());
    ctx.m = BN_bin2bn(m, SGX_MODULUS_SIZE, core::ptr::null_mut());
    ctx.q1 = BN_new();
    ctx.qr = BN_new();
    ctx.q2 = BN_new();
    if (!ctx.bn_ctx || !ctx.s || !ctx.m || !ctx.q1 || !ctx.qr ||
    !ctx.q2) {
    free_q1q2_ctx(ctx);
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn reverse_bytes(data: *mut c_void, length: c_int) {
    static void reverse_bytes(void *data, int length)
    {
    let mut i: c_int = 0;
    let mut j: c_int = length - 1;
    uint8_t temp;
    uint8_t *ptr = data;
    while (i < j) {
    temp = ptr[i];
    ptr[i] = ptr[j];
    ptr[j] = temp;
    i++;
    j--;
    }
    }
    static bool calc_q1q2(const uint8_t *s, const uint8_t *m, uint8_t *q1,
    uint8_t *q2)
    {
    struct q1q2_ctx ctx;
    int len;
    if (!alloc_q1q2_ctx(s, m, &ctx)) {
    fprintf(stderr, "Not enough memory for Q1Q2 calculation\n");
    return false;
    }
    if (!BN_mul(ctx.q1, ctx.s, ctx.s, ctx.bn_ctx))
    goto out;
    if (!BN_div(ctx.q1, ctx.qr, ctx.q1, ctx.m, ctx.bn_ctx))
    goto out;
    if (BN_num_bytes(ctx.q1) > SGX_MODULUS_SIZE) {
    fprintf(stderr, "Too large Q1 %d bytes\n",
    BN_num_bytes(ctx.q1));
    goto out;
    }
    if (!BN_mul(ctx.q2, ctx.s, ctx.qr, ctx.bn_ctx))
    goto out;
    if (!BN_div(ctx.q2, core::ptr::null_mut(), ctx.q2, ctx.m, ctx.bn_ctx))
    goto out;
    if (BN_num_bytes(ctx.q2) > SGX_MODULUS_SIZE) {
    fprintf(stderr, "Too large Q2 %d bytes\n",
    BN_num_bytes(ctx.q2));
    goto out;
    }
    len = BN_bn2bin(ctx.q1, q1);
    reverse_bytes(q1, len);
    len = BN_bn2bin(ctx.q2, q2);
    reverse_bytes(q2, len);
    free_q1q2_ctx(&ctx);
    return true;
    out:
    free_q1q2_ctx(&ctx);
    return false;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_sigstruct_payload {
    pub header: sgx_sigstruct_header,
    pub body: sgx_sigstruct_body,
}

#[no_mangle]
unsafe extern "C" fn check_crypto_errors() -> bool {
    static bool check_crypto_errors(void)
    {
    int err;
    let mut had_errors: bool = false;
    const char *filename;
    int line;
    char str[256];
    for ( ; ; ) {
    if (ERR_peek_error() == 0)
    break;
    had_errors = true;
    err = ERR_get_error_line(&filename, &line);
    ERR_error_string_n(err, str, sizeof(str));
    fprintf(stderr, "crypto: %s: %s:%d\n", str, filename, line);
    }
    return had_errors;
    }
    static inline const BIGNUM *get_modulus(RSA *key)
    {
    const BIGNUM *n;
    RSA_get0_key(key, &n, core::ptr::null_mut(), core::ptr::null_mut());
    return n;
    }
    static RSA *gen_sign_key(void)
    {
    unsigned long sign_key_length;
    BIO *bio;
    RSA *key;
    sign_key_length = (unsigned long)&sign_key_end -
    (unsigned long)&sign_key;
    bio = BIO_new_mem_buf(&sign_key, sign_key_length);
    if (!bio)
    return core::ptr::null_mut();
    key = PEM_read_bio_RSAPrivateKey(bio, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    BIO_free(bio);
    return key;
    }
    enum mrtags {
    MRECREATE = 0x0045544145524345,
    MREADD = 0x0000000044444145,
    MREEXTEND = 0x00444E4554584545,
    };
#[no_mangle]
unsafe extern "C" fn mrenclave_update(ctx: *mut EVP_MD_CTX, data: *const c_void) -> bool {
    static bool mrenclave_update(EVP_MD_CTX *ctx, const void *data)
    {
    if (!EVP_DigestUpdate(ctx, data, 64)) {
    fprintf(stderr, "digest update failed\n");
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn mrenclave_commit(ctx: *mut EVP_MD_CTX, mrenclave: *mut u8) -> bool {
    static bool mrenclave_commit(EVP_MD_CTX *ctx, uint8_t *mrenclave)
    {
    unsigned int size;
    if (!EVP_DigestFinal_ex(ctx, (unsigned char *)mrenclave, &size)) {
    fprintf(stderr, "digest commit failed\n");
    return false;
    }
    if (size != 32) {
    fprintf(stderr, "invalid digest size = %u\n", size);
    return false;
    }
    return true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrecreate {
    pub tag: u64,
    pub ssaframesize: u32,
    pub size: u64,
    pub reserved: [u8; 44],
    pub __attribute__((__packed__)): },
#[no_mangle]
unsafe extern "C" fn mrenclave_ecreate(ctx: *mut EVP_MD_CTX, blob_size: u64) -> bool {
    static bool mrenclave_ecreate(EVP_MD_CTX *ctx, uint64_t blob_size)
    {
    pub mrecreate: mrecreate,
    pub encl_size: u64,
    pub ): for (encl_size = 0x1000; encl_size < blob_size;,
    pub 1: encl_size <<=,
    pub sizeof(mrecreate)): memset(&mrecreate, 0,,
    pub MRECREATE: mrecreate.tag =,
    pub 1: mrecreate.ssaframesize =,
    pub encl_size: mrecreate.size =,
    if (!EVP_DigestInit_ex(ctx, EVP_sha256(), core::ptr::null_mut()))
    pub false: return,
    pub &mrecreate): return mrenclave_update(ctx,,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mreadd {
    pub tag: u64,
    pub offset: u64,
    pub /: *mut *mut uint64_t flags; / SECINFO flags,
    pub reserved: [u8; 40],
    pub __attribute__((__packed__)): },
#[no_mangle]
unsafe extern "C" fn mrenclave_eadd(ctx: *mut EVP_MD_CTX, offset: u64, flags: u64) -> bool {
    static bool mrenclave_eadd(EVP_MD_CTX *ctx, uint64_t offset, uint64_t flags)
    {
    pub mreadd: mreadd,
    pub sizeof(mreadd)): memset(&mreadd, 0,,
    pub MREADD: mreadd.tag =,
    pub offset: mreadd.offset =,
    pub flags: mreadd.flags =,
    pub &mreadd): return mrenclave_update(ctx,,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mreextend {
    pub tag: u64,
    pub offset: u64,
    pub reserved: [u8; 48],
    pub __attribute__((__packed__)): },
    static bool mrenclave_eextend(EVP_MD_CTX *ctx, uint64_t offset,
    const uint8_t *data)
    {
    pub mreextend: mreextend,
    pub i: c_int,
    pub {: for (i = 0; i < 0x1000; i += 0x100),
    pub sizeof(mreextend)): memset(&mreextend, 0,,
    pub MREEXTEND: mreextend.tag =,
    pub i: mreextend.offset = offset +,
    if (!mrenclave_update(ctx, &mreextend))
    pub false: return,
    if (!mrenclave_update(ctx, &data[i + 0x00]))
    pub false: return,
    if (!mrenclave_update(ctx, &data[i + 0x40]))
    pub false: return,
    if (!mrenclave_update(ctx, &data[i + 0x80]))
    pub false: return,
    if (!mrenclave_update(ctx, &data[i + 0xC0]))
    pub false: return,
    }
    pub true: return,
    }
    static bool mrenclave_segment(EVP_MD_CTX *ctx, struct encl *encl,
    struct encl_segment *seg)
    {
    pub seg->size: uint64_t end =,
    pub offset: u64,
    pub {: for (offset = 0; offset < end; offset += PAGE_SIZE),
    if (!mrenclave_eadd(ctx, seg.offset + offset, seg.flags))
    pub false: return,
    if (seg.measure) {
    if (!mrenclave_eextend(ctx, seg.offset + offset, seg.src + offset))
    pub false: return,
    }
    }
    pub true: return,
    }
#[no_mangle]
pub unsafe extern "C" fn encl_measure(encl: *mut encl) -> bool {
    bool encl_measure(struct encl *encl)
    {
    pub 0x0000000000010000}: uint64_t header1[2] = {0x000000E100000006,,
    pub 0x0000000100000060}: uint64_t header2[2] = {0x0000006000000101,,
    pub &encl->sigstruct: *mut *mut sgx_sigsig=,
    pub payload: sgx_sigstruct_payload,
    pub digest: [u8; SHA256_DIGEST_LENGTH],
    pub NULL: *mut *mut EVP_MD_CTX ctx =,
    pub siglen: c_uint,
    pub NULL: *mut *mut RSA key =,
    pub i: c_int,
    pub sizeof(*sigstruct)): *mut memset(sigstruct, 0,,
    pub header1: [sigstruct->header.header1[0] =; 0],
    pub header1: [sigstruct->header.header1[1] =; 1],
    pub header2: [sigstruct->header.header2[0] =; 0],
    pub header2: [sigstruct->header.header2[1] =; 1],
    pub 3: sigstruct->exponent =,
    pub SGX_ATTR_MODE64BIT: sigstruct->body.attributes =,
    pub 3: sigstruct->body.xfrm =,
// sanity check
    if (check_crypto_errors())
    pub err: goto,
    pub gen_sign_key(): key =,
    if (!key) {
    pub err: goto,
    }
    pub sigstruct->modulus): BN_bn2bin(get_modulus(key),,
    pub EVP_MD_CTX_create(): ctx =,
    if (!ctx)
    pub err: goto,
    if (!mrenclave_ecreate(ctx, encl.src_size))
    pub err: goto,
    pub {: for (i = 0; i < encl->nr_segments; i++),
    pub &encl->segment_tbl[i]: *mut *mut encl_segment seg =,
    if (!mrenclave_segment(ctx, encl, seg))
    pub err: goto,
    }
    if (!mrenclave_commit(ctx, sigstruct.body.mrenclave))
    pub err: goto,
    pub sizeof(sigstruct->header)): memcpy(&payload.header, &sigstruct->header,,
    pub sizeof(sigstruct->body)): memcpy(&payload.body, &sigstruct->body,,
    pub digest): *mut *mut SHA256((unsigned char )&payload, sizeof(payload),,
    if (!RSA_sign(NID_sha256, digest, SHA256_DIGEST_LENGTH,
    sigstruct.signature, &siglen, key))
    pub err: goto,
    if (!calc_q1q2(sigstruct.signature, sigstruct.modulus, sigstruct.q1,
    sigstruct.q2))
    pub err: goto,
// BE -> LE
    pub SGX_MODULUS_SIZE): reverse_bytes(sigstruct->signature,,
    pub SGX_MODULUS_SIZE): reverse_bytes(sigstruct->modulus,,
    pub true: return,
    err:
    if (ctx)
    pub false: return,
    }
