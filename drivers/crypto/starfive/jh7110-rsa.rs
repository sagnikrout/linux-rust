//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/starfive/jh7110-rsa.c
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
//
// StarFive Public Key Algo acceleration driver
//
// Copyright (c) 2022 StarFive Technology
//

pub const STARFIVE_PKA_REGS_OFFSET: c_uint = 0x400;

// R ^ 2 mod N and N0'
pub const CRYPTO_CMD_PRE: c_uint = 0x0;
// A * R mod N   ==> A
pub const CRYPTO_CMD_ARN: c_uint = 0x5;
// A * E * R mod N ==> A
pub const CRYPTO_CMD_AERN: c_uint = 0x6;
// A * A * R mod N ==> A
pub const CRYPTO_CMD_AARN: c_uint = 0x7;
pub const STARFIVE_RSA_RESET: c_uint = 0x2;
#[no_mangle]
pub unsafe extern "C" fn starfive_pka_wait_done(ctx: *mut starfive_cryp_ctx) -> c_int {
    static inline int starfive_pka_wait_done(struct starfive_cryp_ctx *ctx)
    {
    struct starfive_cryp_dev *cryp = ctx.cryp;
    u32 status;
    return readl_relaxed_poll_timeout(cryp.base + STARFIVE_PKA_CASR_OFFSET, status,
    status & STARFIVE_PKA_DONE, 10, 100000);
    }
#[no_mangle]
unsafe extern "C" fn starfive_rsa_free_key(key: *mut starfive_rsa_key) {
    static void starfive_rsa_free_key(struct starfive_rsa_key *key)
    {
    if (!key.key_sz)
    return;
    kfree_sensitive(key.d);
    kfree_sensitive(key.e);
    kfree_sensitive(key.n);
    memset(key, 0, sizeof(*key));
    }
#[no_mangle]
unsafe extern "C" fn starfive_rsa_get_nbit(pa: *mut u8, snum: u32, key_sz: c_int) -> c_uint {
    static unsigned int starfive_rsa_get_nbit(u8 *pa, u32 snum, int key_sz)
    {
    u32 i;
    u8 value;
    i = snum >> 3;
    value = pa[key_sz - i - 1];
    value >>= snum & 0x7;
    value &= 0x1;
    return value;
    }
    static int starfive_rsa_montgomery_form(struct starfive_cryp_ctx *ctx,
    u32 *out, u32 *in, u8 mont,
    u32 *mod, int bit_len)
    {
    struct starfive_cryp_dev *cryp = ctx.cryp;
    struct starfive_cryp_request_ctx *rctx = ctx.rctx;
    let mut count: c_int = (ALIGN(rctx.total, 4) / 4) - 1;
    int loop;
    u32 temp;
    u8 opsize;
    opsize = (bit_len - 1) >> 5;
    rctx.csr.pka.v = 0;
    writel(rctx.csr.pka.v, cryp.base + STARFIVE_PKA_CACR_OFFSET);
    for (loop = 0; loop <= opsize; loop++)
    writel(mod[opsize - loop], cryp.base + STARFIVE_PKA_CANR_OFFSET + loop * 4);
    if (mont) {
    rctx.csr.pka.v = 0;
    rctx.csr.pka.cln_done = 1;
    rctx.csr.pka.opsize = opsize;
    rctx.csr.pka.exposize = opsize;
    rctx.csr.pka.cmd = CRYPTO_CMD_PRE;
    rctx.csr.pka.start = 1;
    rctx.csr.pka.not_r2 = 1;
    rctx.csr.pka.ie = 1;
    writel(rctx.csr.pka.v, cryp.base + STARFIVE_PKA_CACR_OFFSET);
    if (starfive_pka_wait_done(ctx))
    return -ETIMEDOUT;
    for (loop = 0; loop <= opsize; loop++)
    writel(in[opsize - loop], cryp.base + STARFIVE_PKA_CAAR_OFFSET + loop * 4);
    writel(0x1000000, cryp.base + STARFIVE_PKA_CAER_OFFSET);
    for (loop = 1; loop <= opsize; loop++)
    writel(0, cryp.base + STARFIVE_PKA_CAER_OFFSET + loop * 4);
    rctx.csr.pka.v = 0;
    rctx.csr.pka.cln_done = 1;
    rctx.csr.pka.opsize = opsize;
    rctx.csr.pka.exposize = opsize;
    rctx.csr.pka.cmd = CRYPTO_CMD_AERN;
    rctx.csr.pka.start = 1;
    rctx.csr.pka.ie = 1;
    writel(rctx.csr.pka.v, cryp.base + STARFIVE_PKA_CACR_OFFSET);
    if (starfive_pka_wait_done(ctx))
    return -ETIMEDOUT;
    } else {
    rctx.csr.pka.v = 0;
    rctx.csr.pka.cln_done = 1;
    rctx.csr.pka.opsize = opsize;
    rctx.csr.pka.exposize = opsize;
    rctx.csr.pka.cmd = CRYPTO_CMD_PRE;
    rctx.csr.pka.start = 1;
    rctx.csr.pka.pre_expf = 1;
    rctx.csr.pka.ie = 1;
    writel(rctx.csr.pka.v, cryp.base + STARFIVE_PKA_CACR_OFFSET);
    if (starfive_pka_wait_done(ctx))
    return -ETIMEDOUT;
    for (loop = 0; loop <= count; loop++)
    writel(in[count - loop], cryp.base + STARFIVE_PKA_CAER_OFFSET + loop * 4);
// pad with 0 up to opsize
    for (loop = count + 1; loop <= opsize; loop++)
    writel(0, cryp.base + STARFIVE_PKA_CAER_OFFSET + loop * 4);
    rctx.csr.pka.v = 0;
    rctx.csr.pka.cln_done = 1;
    rctx.csr.pka.opsize = opsize;
    rctx.csr.pka.exposize = opsize;
    rctx.csr.pka.cmd = CRYPTO_CMD_ARN;
    rctx.csr.pka.start = 1;
    rctx.csr.pka.ie = 1;
    writel(rctx.csr.pka.v, cryp.base + STARFIVE_PKA_CACR_OFFSET);
    if (starfive_pka_wait_done(ctx))
    return -ETIMEDOUT;
    }
    for (loop = 0; loop <= opsize; loop++) {
    temp = readl(cryp.base + STARFIVE_PKA_CAAR_OFFSET + 0x4 * loop);
    out[opsize - loop] = temp;
    }
    return 0;
    }
    static int starfive_rsa_cpu_start(struct starfive_cryp_ctx *ctx, u32 *result,
    u8 *de, u32 *n, int key_sz)
    {
    struct starfive_cryp_dev *cryp = ctx.cryp;
    struct starfive_cryp_request_ctx *rctx = ctx.rctx;
    struct starfive_rsa_key *key = &ctx.rsa_key;
    u32 temp;
    let mut ret: c_int = 0;
    int opsize, mlen, loop;
    unsigned int *mta;
    opsize = (key_sz - 1) >> 2;
    mta = kmalloc(key_sz, GFP_KERNEL);
    if (!mta)
    return -ENOMEM;
    ret = starfive_rsa_montgomery_form(ctx, mta, (u32 *)rctx.rsa_data,
    0, n, key_sz << 3);
    if (ret) {
    dev_err_probe(cryp.dev, ret, "Conversion to Montgomery failed");
    goto rsa_err;
    }
    for (loop = 0; loop <= opsize; loop++)
    writel(mta[opsize - loop],
    cryp.base + STARFIVE_PKA_CAER_OFFSET + loop * 4);
    for (loop = key.bitlen - 1; loop > 0; loop--) {
    mlen = starfive_rsa_get_nbit(de, loop - 1, key_sz);
    rctx.csr.pka.v = 0;
    rctx.csr.pka.cln_done = 1;
    rctx.csr.pka.opsize = opsize;
    rctx.csr.pka.exposize = opsize;
    rctx.csr.pka.cmd = CRYPTO_CMD_AARN;
    rctx.csr.pka.start = 1;
    rctx.csr.pka.ie = 1;
    writel(rctx.csr.pka.v, cryp.base + STARFIVE_PKA_CACR_OFFSET);
    ret = -ETIMEDOUT;
    if (starfive_pka_wait_done(ctx))
    goto rsa_err;
    if (mlen) {
    rctx.csr.pka.v = 0;
    rctx.csr.pka.cln_done = 1;
    rctx.csr.pka.opsize = opsize;
    rctx.csr.pka.exposize = opsize;
    rctx.csr.pka.cmd = CRYPTO_CMD_AERN;
    rctx.csr.pka.start = 1;
    rctx.csr.pka.ie = 1;
    writel(rctx.csr.pka.v, cryp.base + STARFIVE_PKA_CACR_OFFSET);
    if (starfive_pka_wait_done(ctx))
    goto rsa_err;
    }
    }
    for (loop = 0; loop <= opsize; loop++) {
    temp = readl(cryp.base + STARFIVE_PKA_CAAR_OFFSET + 0x4 * loop);
    result[opsize - loop] = temp;
    }
    ret = starfive_rsa_montgomery_form(ctx, result, result, 1, n, key_sz << 3);
    if (ret)
    dev_err_probe(cryp.dev, ret, "Conversion from Montgomery failed");
    rsa_err:
    kfree(mta);
    return ret;
    }
    static int starfive_rsa_start(struct starfive_cryp_ctx *ctx, u8 *result,
    u8 *de, u8 *n, int key_sz)
    {
    return starfive_rsa_cpu_start(ctx, (u32 *)result, de, (u32 *)n, key_sz);
    }
#[no_mangle]
unsafe extern "C" fn starfive_rsa_enc_core(ctx: *mut starfive_cryp_ctx, enc: c_int) -> c_int {
    static int starfive_rsa_enc_core(struct starfive_cryp_ctx *ctx, int enc)
    {
    struct starfive_cryp_dev *cryp = ctx.cryp;
    struct starfive_cryp_request_ctx *rctx = ctx.rctx;
    struct starfive_rsa_key *key = &ctx.rsa_key;
    let mut ret: c_int = 0, shift = 0;
    writel(STARFIVE_RSA_RESET, cryp.base + STARFIVE_PKA_CACR_OFFSET);
    if (!IS_ALIGNED(rctx.total, sizeof(u32))) {
    shift = sizeof(u32) - (rctx.total & 0x3);
    memset(rctx.rsa_data, 0, shift);
    }
    rctx.total = sg_copy_to_buffer(rctx.in_sg, sg_nents(rctx.in_sg),
    rctx.rsa_data + shift, rctx.total);
    if (enc) {
    key.bitlen = key.e_bitlen;
    ret = starfive_rsa_start(ctx, rctx.rsa_data, key.e,
    key.n, key.key_sz);
    } else {
    key.bitlen = key.d_bitlen;
    ret = starfive_rsa_start(ctx, rctx.rsa_data, key.d,
    key.n, key.key_sz);
    }
    if (ret)
    goto err_rsa_crypt;
    sg_copy_buffer(rctx.out_sg, sg_nents(rctx.out_sg),
    rctx.rsa_data, key.key_sz, 0, 0);
    err_rsa_crypt:
    writel(STARFIVE_RSA_RESET, cryp.base + STARFIVE_PKA_CACR_OFFSET);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn starfive_rsa_enc(req: *mut akcipher_request) -> c_int {
    static int starfive_rsa_enc(struct akcipher_request *req)
    {
    struct crypto_akcipher *tfm = crypto_akcipher_reqtfm(req);
    struct starfive_cryp_ctx *ctx = akcipher_tfm_ctx(tfm);
    struct starfive_cryp_dev *cryp = ctx.cryp;
    struct starfive_rsa_key *key = &ctx.rsa_key;
    struct starfive_cryp_request_ctx *rctx = akcipher_request_ctx(req);
    int ret;
    if (!key.key_sz) {
    akcipher_request_set_tfm(req, ctx.akcipher_fbk);
    ret = crypto_akcipher_encrypt(req);
    akcipher_request_set_tfm(req, tfm);
    return ret;
    }
    if (unlikely(!key.n || !key.e))
    return -EINVAL;
    if (req.dst_len < key.key_sz)
    return dev_err_probe(cryp.dev, -EOVERFLOW,
    "Output buffer length less than parameter n\n");
    rctx.in_sg = req.src;
    rctx.out_sg = req.dst;
    rctx.total = req.src_len;
    ctx.rctx = rctx;
    return starfive_rsa_enc_core(ctx, 1);
    }
#[no_mangle]
unsafe extern "C" fn starfive_rsa_dec(req: *mut akcipher_request) -> c_int {
    static int starfive_rsa_dec(struct akcipher_request *req)
    {
    struct crypto_akcipher *tfm = crypto_akcipher_reqtfm(req);
    struct starfive_cryp_ctx *ctx = akcipher_tfm_ctx(tfm);
    struct starfive_cryp_dev *cryp = ctx.cryp;
    struct starfive_rsa_key *key = &ctx.rsa_key;
    struct starfive_cryp_request_ctx *rctx = akcipher_request_ctx(req);
    int ret;
    if (!key.key_sz) {
    akcipher_request_set_tfm(req, ctx.akcipher_fbk);
    ret = crypto_akcipher_decrypt(req);
    akcipher_request_set_tfm(req, tfm);
    return ret;
    }
    if (unlikely(!key.n || !key.d))
    return -EINVAL;
    if (req.dst_len < key.key_sz)
    return dev_err_probe(cryp.dev, -EOVERFLOW,
    "Output buffer length less than parameter n\n");
    rctx.in_sg = req.src;
    rctx.out_sg = req.dst;
    ctx.rctx = rctx;
    rctx.total = req.src_len;
    return starfive_rsa_enc_core(ctx, 0);
    }
    static int starfive_rsa_set_n(struct starfive_rsa_key *rsa_key,
    const char *value, size_t vlen)
    {
    const char *ptr = value;
    unsigned int bitslen;
    int ret;
    while (!*ptr && vlen) {
    ptr++;
    vlen--;
    }
    rsa_key.key_sz = vlen;
    bitslen = rsa_key.key_sz << 3;
// check valid key size
    if (bitslen & 0x1f)
    return -EINVAL;
    ret = -ENOMEM;
    rsa_key.n = kmemdup(ptr, rsa_key.key_sz, GFP_KERNEL);
    if (!rsa_key.n)
    goto err;
    return 0;
    err:
    rsa_key.key_sz = 0;
    rsa_key.n = core::ptr::null_mut();
    starfive_rsa_free_key(rsa_key);
    return ret;
    }
    static int starfive_rsa_set_e(struct starfive_rsa_key *rsa_key,
    const char *value, size_t vlen)
    {
    const char *ptr = value;
    unsigned char pt;
    int loop;
    while (!*ptr && vlen) {
    ptr++;
    vlen--;
    }
    pt = *ptr;
    if (!rsa_key.key_sz || !vlen || vlen > rsa_key.key_sz) {
    rsa_key.e = core::ptr::null_mut();
    return -EINVAL;
    }
    rsa_key.e = kzalloc(rsa_key.key_sz, GFP_KERNEL);
    if (!rsa_key.e)
    return -ENOMEM;
    for (loop = 8; loop > 0; loop--) {
    if (pt >> (loop - 1))
    break;
    }
    rsa_key.e_bitlen = (vlen - 1) * 8 + loop;
    memcpy(rsa_key.e + (rsa_key.key_sz - vlen), ptr, vlen);
    return 0;
    }
    static int starfive_rsa_set_d(struct starfive_rsa_key *rsa_key,
    const char *value, size_t vlen)
    {
    const char *ptr = value;
    unsigned char pt;
    int loop;
    int ret;
    while (!*ptr && vlen) {
    ptr++;
    vlen--;
    }
    pt = *ptr;
    ret = -EINVAL;
    if (!rsa_key.key_sz || !vlen || vlen > rsa_key.key_sz)
    goto err;
    ret = -ENOMEM;
    rsa_key.d = kzalloc(rsa_key.key_sz, GFP_KERNEL);
    if (!rsa_key.d)
    goto err;
    for (loop = 8; loop > 0; loop--) {
    if (pt >> (loop - 1))
    break;
    }
    rsa_key.d_bitlen = (vlen - 1) * 8 + loop;
    memcpy(rsa_key.d + (rsa_key.key_sz - vlen), ptr, vlen);
    return 0;
    err:
    rsa_key.d = core::ptr::null_mut();
    return ret;
    }
    static int starfive_rsa_setkey(struct crypto_akcipher *tfm, const void *key,
    unsigned int keylen, bool private)
    {
    struct starfive_cryp_ctx *ctx = akcipher_tfm_ctx(tfm);
    let mut raw_key: rsa_key = {core::ptr::null_mut()};
    struct starfive_rsa_key *rsa_key = &ctx.rsa_key;
    int ret;
    if (private)
    ret = rsa_parse_priv_key(&raw_key, key, keylen);
    else
    ret = rsa_parse_pub_key(&raw_key, key, keylen);
    if (ret < 0)
    goto err;
    starfive_rsa_free_key(rsa_key);
// Use fallback for mod > 256 + 1 byte prefix
    if (raw_key.n_sz > STARFIVE_RSA_MAX_KEYSZ + 1)
    return 0;
    ret = starfive_rsa_set_n(rsa_key, raw_key.n, raw_key.n_sz);
    if (ret)
    return ret;
    ret = starfive_rsa_set_e(rsa_key, raw_key.e, raw_key.e_sz);
    if (ret)
    goto err;
    if (private) {
    ret = starfive_rsa_set_d(rsa_key, raw_key.d, raw_key.d_sz);
    if (ret)
    goto err;
    }
    if (!rsa_key.n || !rsa_key.e) {
    ret = -EINVAL;
    goto err;
    }
    if (private && !rsa_key.d) {
    ret = -EINVAL;
    goto err;
    }
    return 0;
    err:
    starfive_rsa_free_key(rsa_key);
    return ret;
    }
    static int starfive_rsa_set_pub_key(struct crypto_akcipher *tfm, const void *key,
    unsigned int keylen)
    {
    struct starfive_cryp_ctx *ctx = akcipher_tfm_ctx(tfm);
    int ret;
    ret = crypto_akcipher_set_pub_key(ctx.akcipher_fbk, key, keylen);
    if (ret)
    return ret;
    return starfive_rsa_setkey(tfm, key, keylen, false);
    }
    static int starfive_rsa_set_priv_key(struct crypto_akcipher *tfm, const void *key,
    unsigned int keylen)
    {
    struct starfive_cryp_ctx *ctx = akcipher_tfm_ctx(tfm);
    int ret;
    ret = crypto_akcipher_set_priv_key(ctx.akcipher_fbk, key, keylen);
    if (ret)
    return ret;
    return starfive_rsa_setkey(tfm, key, keylen, true);
    }
#[no_mangle]
unsafe extern "C" fn starfive_rsa_max_size(tfm: *mut crypto_akcipher) -> c_uint {
    static unsigned int starfive_rsa_max_size(struct crypto_akcipher *tfm)
    {
    struct starfive_cryp_ctx *ctx = akcipher_tfm_ctx(tfm);
    if (ctx.rsa_key.key_sz)
    return ctx.rsa_key.key_sz;
    return crypto_akcipher_maxsize(ctx.akcipher_fbk);
    }
#[no_mangle]
unsafe extern "C" fn starfive_rsa_init_tfm(tfm: *mut crypto_akcipher) -> c_int {
    static int starfive_rsa_init_tfm(struct crypto_akcipher *tfm)
    {
    struct starfive_cryp_ctx *ctx = akcipher_tfm_ctx(tfm);
    ctx.cryp = starfive_cryp_find_dev(ctx);
    if (!ctx.cryp)
    return -ENODEV;
    ctx.akcipher_fbk = crypto_alloc_akcipher("rsa-generic", 0, 0);
    if (IS_ERR(ctx.akcipher_fbk))
    return PTR_ERR(ctx.akcipher_fbk);
    akcipher_set_reqsize(tfm, sizeof(struct starfive_cryp_request_ctx) +
    sizeof(struct crypto_akcipher) + 32);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn starfive_rsa_exit_tfm(tfm: *mut crypto_akcipher) {
    static void starfive_rsa_exit_tfm(struct crypto_akcipher *tfm)
    {
    struct starfive_cryp_ctx *ctx = akcipher_tfm_ctx(tfm);
    struct starfive_rsa_key *key = (struct starfive_rsa_key *)&ctx.rsa_key;
    crypto_free_akcipher(ctx.akcipher_fbk);
    starfive_rsa_free_key(key);
    }
    static struct akcipher_alg starfive_rsa = {
    .encrypt = starfive_rsa_enc,
    .decrypt = starfive_rsa_dec,
    .set_pub_key = starfive_rsa_set_pub_key,
    .set_priv_key = starfive_rsa_set_priv_key,
    .max_size = starfive_rsa_max_size,
    .init = starfive_rsa_init_tfm,
    .exit = starfive_rsa_exit_tfm,
    .base = {
    .cra_name = "rsa",
    .cra_driver_name = "starfive-rsa",
    .cra_flags = CRYPTO_ALG_TYPE_AKCIPHER |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_priority = 3000,
    .cra_module = THIS_MODULE,
    .cra_ctxsize = sizeof(struct starfive_cryp_ctx),
    },
    };
#[no_mangle]
pub unsafe extern "C" fn starfive_rsa_register_algs() -> c_int {
    int starfive_rsa_register_algs(void)
    {
    return crypto_register_akcipher(&starfive_rsa);
    }
#[no_mangle]
pub unsafe extern "C" fn starfive_rsa_unregister_algs() {
    void starfive_rsa_unregister_algs(void)
    {
    crypto_unregister_akcipher(&starfive_rsa);
    }
