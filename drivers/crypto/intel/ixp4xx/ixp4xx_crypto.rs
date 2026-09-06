//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/ixp4xx/ixp4xx_crypto.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Intel IXP4xx NPE-C crypto driver
//
// Copyright (C) 2008 Christian Hohnstaedt <chohnstaedt@innominate.com>
//

// Intermittent includes, delete this after v5.14-rc1

pub const MAX_KEYLEN: c_int = 32;
// hash: cfgword + 2 * digestlen; crypt: keylen + cfgword
pub const NPE_CTX_LEN: c_int = 80;
pub const AES_BLOCK128: c_int = 16;
pub const NPE_OP_HASH_VERIFY: c_uint = 0x01;
pub const NPE_OP_CCM_ENABLE: c_uint = 0x04;
pub const NPE_OP_CRYPT_ENABLE: c_uint = 0x08;
pub const NPE_OP_HASH_ENABLE: c_uint = 0x10;
pub const NPE_OP_NOT_IN_PLACE: c_uint = 0x20;
pub const NPE_OP_HMAC_DISABLE: c_uint = 0x40;
pub const NPE_OP_CRYPT_ENCRYPT: c_uint = 0x80;
pub const NPE_OP_CCM_GEN_MIC: c_uint = 0xcc;
pub const NPE_OP_HASH_GEN_ICV: c_uint = 0x50;
pub const NPE_OP_ENC_GEN_KEY: c_uint = 0xc9;
pub const MOD_ECB: c_uint = 0x0000;
pub const MOD_CTR: c_uint = 0x1000;
pub const MOD_CBC_ENC: c_uint = 0x2000;
pub const MOD_CBC_DEC: c_uint = 0x3000;
pub const MOD_CCM_ENC: c_uint = 0x4000;
pub const MOD_CCM_DEC: c_uint = 0x5000;
pub const KEYLEN_128: c_int = 4;
pub const KEYLEN_192: c_int = 6;
pub const KEYLEN_256: c_int = 8;
pub const CIPH_DECR: c_uint = 0x0000;
pub const CIPH_ENCR: c_uint = 0x0400;
pub const MOD_DES: c_uint = 0x0000;
pub const MOD_TDEA2: c_uint = 0x0100;
pub const MOD_3DES: c_uint = 0x0200;
pub const MOD_AES: c_uint = 0x0800;

pub const MAX_IVLEN: c_int = 16;
pub const NPE_QLEN: c_int = 16;
// Space for registering when the first
// NPE_QLEN crypt_ctl are busy
pub const NPE_QLEN_TOTAL: c_int = 64;
pub const CTL_FLAG_UNUSED: c_uint = 0x0000;
pub const CTL_FLAG_USED: c_uint = 0x1000;
pub const CTL_FLAG_PERFORM_ABLK: c_uint = 0x0001;
pub const CTL_FLAG_GEN_ICV: c_uint = 0x0002;
pub const CTL_FLAG_GEN_REVAES: c_uint = 0x0004;
pub const CTL_FLAG_PERFORM_AEAD: c_uint = 0x0008;
pub const CTL_FLAG_MASK: c_uint = 0x000f;

pub const MD5_DIGEST_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer_desc {
    pub phys_next: u32,

    pub buf_len: u16,
    pub pkt_len: u16,

    pub pkt_len: u16,
    pub buf_len: u16,

    pub phys_addr: dma_addr_t,
    pub __reserved: [u32; 4],
    pub next: *mut buffer_desc,
    pub dir: enum dma_data_direction,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypt_ctl {

    pub /: *mut *mut *mut u8 mode; / NPE_OP_ operation mode,
    pub init_len: u8,
    pub reserved: u16,

    pub reserved: u16,
    pub init_len: u8,
    pub /: *mut *mut *mut u8 mode; / NPE_OP_ operation mode,

    pub /: *mut *mut u8 iv[MAX_IVLEN]; / IV for CBC mode or CTR IV for CTR mode,
    pub /: *mut *mut u32 icv_rev_aes; / icv or rev aes,
    pub src_buf: u32,
    pub dst_buf: u32,

    pub /: *mut *mut u16 auth_offs; / Authentication start offset,
    pub /: *mut *mut u16 auth_len; / Authentication data length,
    pub /: *mut *mut u16 crypt_offs; / Cryption start offset,
    pub /: *mut *mut u16 crypt_len; / Cryption data length,

    pub /: *mut *mut u16 auth_len; / Authentication data length,
    pub /: *mut *mut u16 auth_offs; / Authentication start offset,
    pub /: *mut *mut u16 crypt_len; / Cryption data length,
    pub /: *mut *mut u16 crypt_offs; / Cryption start offset,

    pub /: *mut *mut u32 aadAddr; / Additional Auth Data Addr for CCM mode,
    pub /: *mut *mut u32 crypto_ctx; / NPE Crypto Param structure address,
// Used by Host: 4*4 bytes
    pub ctl_flags: c_uint,
    union {
    pub ablk_req: *mut skcipher_request,
    pub aead_req: *mut aead_request,
    pub tfm: *mut crypto_tfm,
    pub data: },
    pub regist_buf: *mut buffer_desc,
    pub regist_ptr: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ablk_ctx {
    pub src: *mut buffer_desc,
    pub dst: *mut buffer_desc,
    pub iv: [u8; MAX_IVLEN],
    pub encrypt: bool,
    pub end: skcipher_request fallback_req; // keep at the,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_ctx {
    pub src: *mut buffer_desc,
    pub dst: *mut buffer_desc,
    pub ivlist: scatterlist,
// used when the hmac is not on one sg entry
    pub hmac_virt: *mut u8,
    pub encrypt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ix_hash_algo {
    pub cfgword: u32,
    pub icv: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ix_sa_dir {
    pub npe_ctx: *mut c_uchar,
    pub npe_ctx_phys: dma_addr_t,
    pub npe_ctx_idx: c_int,
    pub npe_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixp_ctx {
    pub encrypt: ix_sa_dir,
    pub decrypt: ix_sa_dir,
    pub authkey_len: c_int,
    pub authkey: [u8; MAX_KEYLEN],
    pub enckey_len: c_int,
    pub enckey: [u8; MAX_KEYLEN],
    pub salt: [u8; MAX_IVLEN],
    pub nonce: [u8; CTR_RFC3686_NONCE_SIZE],
    pub salted: c_uint,
    pub configuring: core::sync::atomic::AtomicI32,
    pub completion: completion,
    pub fallback_tfm: *mut crypto_skcipher,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixp_alg {
    pub crypto: skcipher_alg,
    pub hash: *const ix_hash_algo,
    pub cfg_enc: u32,
    pub cfg_dec: u32,
    pub registered: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixp_aead_alg {
    pub crypto: aead_alg,
    pub hash: *const ix_hash_algo,
    pub cfg_enc: u32,
    pub cfg_dec: u32,
    pub registered: c_int,
}

    static const struct ix_hash_algo hash_alg_md5 = {
    .cfgword	= 0xAA010004,
    .icv		= "\x01\x23\x45\x67\x89\xAB\xCD\xEF"
    "\xFE\xDC\xBA\x98\x76\x54\x32\x10",
    };
    static const struct ix_hash_algo hash_alg_sha1 = {
    .cfgword	= 0x00000005,
    .icv		= "\x67\x45\x23\x01\xEF\xCD\xAB\x89\x98\xBA"
    "\xDC\xFE\x10\x32\x54\x76\xC3\xD2\xE1\xF0",
    };
    static struct npe *npe_c;
    static unsigned int send_qid;
    static unsigned int recv_qid;
    static struct dma_pool *buffer_pool;
    static struct dma_pool *ctx_pool;
    static struct crypt_ctl *crypt_virt;
    static dma_addr_t crypt_phys;
    let mut support_aes: static int = 1;
    static struct platform_device *pdev;
#[no_mangle]
pub unsafe extern "C" fn crypt_virt2phys(virt: *mut crypt_ctl) -> dma_addr_t {
    static inline dma_addr_t crypt_virt2phys(struct crypt_ctl *virt)
    {
    return crypt_phys + (virt - crypt_virt) * sizeof(struct crypt_ctl);
    }
    static inline struct crypt_ctl *crypt_phys2virt(dma_addr_t phys)
    {
    return crypt_virt + (phys - crypt_phys) / sizeof(struct crypt_ctl);
    }
#[no_mangle]
pub unsafe extern "C" fn cipher_cfg_enc(tfm: *mut crypto_tfm) -> u32 {
    static inline u32 cipher_cfg_enc(struct crypto_tfm *tfm)
    {
    return container_of(tfm.__crt_alg, struct ixp_alg, crypto.base).cfg_enc;
    }
#[no_mangle]
pub unsafe extern "C" fn cipher_cfg_dec(tfm: *mut crypto_tfm) -> u32 {
    static inline u32 cipher_cfg_dec(struct crypto_tfm *tfm)
    {
    return container_of(tfm.__crt_alg, struct ixp_alg, crypto.base).cfg_dec;
    }
    static inline const struct ix_hash_algo *ix_hash(struct crypto_tfm *tfm)
    {
    return container_of(tfm.__crt_alg, struct ixp_alg, crypto.base).hash;
    }
#[no_mangle]
unsafe extern "C" fn setup_crypt_desc() -> c_int {
    static int setup_crypt_desc(void)
    {
    struct device *dev = &pdev.dev;
    BUILD_BUG_ON(!(IS_ENABLED(CONFIG_COMPILE_TEST) &&
    IS_ENABLED(CONFIG_64BIT)) &&
    sizeof(struct crypt_ctl) != 64);
    crypt_virt = dma_alloc_coherent(dev,
    NPE_QLEN * sizeof(struct crypt_ctl),
    &crypt_phys, GFP_ATOMIC);
    if (!crypt_virt)
    return -ENOMEM;
    return 0;
    }
    static DEFINE_SPINLOCK(desc_lock);
    static struct crypt_ctl *get_crypt_desc(void)
    {
    int i;
    static int idx;
    unsigned long flags;
    spin_lock_irqsave(&desc_lock, flags);
    if (unlikely(!crypt_virt))
    setup_crypt_desc();
    if (unlikely(!crypt_virt)) {
    spin_unlock_irqrestore(&desc_lock, flags);
    return core::ptr::null_mut();
    }
    i = idx;
    if (crypt_virt[i].ctl_flags == CTL_FLAG_UNUSED) {
    if (++idx >= NPE_QLEN)
    idx = 0;
    crypt_virt[i].ctl_flags = CTL_FLAG_USED;
    spin_unlock_irqrestore(&desc_lock, flags);
    return crypt_virt + i;
    } else {
    spin_unlock_irqrestore(&desc_lock, flags);
    return core::ptr::null_mut();
    }
    }
    static DEFINE_SPINLOCK(emerg_lock);
    static struct crypt_ctl *get_crypt_desc_emerg(void)
    {
    int i;
    let mut idx: static int = NPE_QLEN;
    struct crypt_ctl *desc;
    unsigned long flags;
    desc = get_crypt_desc();
    if (desc)
    return desc;
    if (unlikely(!crypt_virt))
    return core::ptr::null_mut();
    spin_lock_irqsave(&emerg_lock, flags);
    i = idx;
    if (crypt_virt[i].ctl_flags == CTL_FLAG_UNUSED) {
    if (++idx >= NPE_QLEN_TOTAL)
    idx = NPE_QLEN;
    crypt_virt[i].ctl_flags = CTL_FLAG_USED;
    spin_unlock_irqrestore(&emerg_lock, flags);
    return crypt_virt + i;
    } else {
    spin_unlock_irqrestore(&emerg_lock, flags);
    return core::ptr::null_mut();
    }
    }
    static void free_buf_chain(struct device *dev, struct buffer_desc *buf,
    dma_addr_t phys)
    {
    while (buf) {
    struct buffer_desc *buf1;
    u32 phys1;
    buf1 = buf.next;
    phys1 = buf.phys_next;
    dma_unmap_single(dev, buf.phys_addr, buf.buf_len, buf.dir);
    dma_pool_free(buffer_pool, buf, phys);
    buf = buf1;
    phys = phys1;
    }
    }
    static struct tasklet_struct crypto_done_tasklet;
#[no_mangle]
unsafe extern "C" fn finish_scattered_hmac(crypt: *mut crypt_ctl) {
    static void finish_scattered_hmac(struct crypt_ctl *crypt)
    {
    struct aead_request *req = crypt.data.aead_req;
    struct aead_ctx *req_ctx = aead_request_ctx(req);
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    let mut authsize: c_int = crypto_aead_authsize(tfm);
    let mut decryptlen: c_int = req.assoclen + req.cryptlen - authsize;
    if (req_ctx.encrypt) {
    scatterwalk_map_and_copy(req_ctx.hmac_virt, req.dst,
    decryptlen, authsize, 1);
    }
    dma_pool_free(buffer_pool, req_ctx.hmac_virt, crypt.icv_rev_aes);
    }
#[no_mangle]
unsafe extern "C" fn one_packet(phys: dma_addr_t) {
    static void one_packet(dma_addr_t phys)
    {
    struct device *dev = &pdev.dev;
    struct crypt_ctl *crypt;
    struct ixp_ctx *ctx;
    int failed;
    failed = phys & 0x1 ? -EBADMSG : 0;
    phys &= ~0x3;
    crypt = crypt_phys2virt(phys);
    switch (crypt.ctl_flags & CTL_FLAG_MASK) {
    case CTL_FLAG_PERFORM_AEAD: {
    struct aead_request *req = crypt.data.aead_req;
    struct aead_ctx *req_ctx = aead_request_ctx(req);
    free_buf_chain(dev, req_ctx.src, crypt.src_buf);
    free_buf_chain(dev, req_ctx.dst, crypt.dst_buf);
    if (req_ctx.hmac_virt)
    finish_scattered_hmac(crypt);
    aead_request_complete(req, failed);
    break;
    }
    case CTL_FLAG_PERFORM_ABLK: {
    struct skcipher_request *req = crypt.data.ablk_req;
    struct ablk_ctx *req_ctx = skcipher_request_ctx(req);
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    let mut ivsize: c_uint = crypto_skcipher_ivsize(tfm);
    unsigned int offset;
    if (ivsize > 0) {
    offset = req.cryptlen - ivsize;
    if (req_ctx.encrypt) {
    scatterwalk_map_and_copy(req.iv, req.dst,
    offset, ivsize, 0);
    } else {
    memcpy(req.iv, req_ctx.iv, ivsize);
    memzero_explicit(req_ctx.iv, ivsize);
    }
    }
    if (req_ctx.dst)
    free_buf_chain(dev, req_ctx.dst, crypt.dst_buf);
    free_buf_chain(dev, req_ctx.src, crypt.src_buf);
    skcipher_request_complete(req, failed);
    break;
    }
    case CTL_FLAG_GEN_ICV:
    ctx = crypto_tfm_ctx(crypt.data.tfm);
    dma_pool_free(ctx_pool, crypt.regist_ptr,
    crypt.regist_buf.phys_addr);
    dma_pool_free(buffer_pool, crypt.regist_buf, crypt.src_buf);
    if (atomic_dec_and_test(&ctx.configuring))
    complete(&ctx.completion);
    break;
    case CTL_FLAG_GEN_REVAES:
    ctx = crypto_tfm_ctx(crypt.data.tfm);
// (__be32 *)ctx->decrypt.npe_ctx &= cpu_to_be32(~CIPH_ENCR);
    if (atomic_dec_and_test(&ctx.configuring))
    complete(&ctx.completion);
    break;
    default:
    BUG();
    }
    crypt.ctl_flags = CTL_FLAG_UNUSED;
    }
#[no_mangle]
unsafe extern "C" fn irqhandler(_unused: *mut c_void) {
    static void irqhandler(void *_unused)
    {
    tasklet_schedule(&crypto_done_tasklet);
    }
#[no_mangle]
unsafe extern "C" fn crypto_done_action(arg: c_ulong) {
    static void crypto_done_action(unsigned long arg)
    {
    int i;
    for (i = 0; i < 4; i++) {
    let mut phys: dma_addr_t = qmgr_get_entry(recv_qid);
    if (!phys)
    return;
    one_packet(phys);
    }
    tasklet_schedule(&crypto_done_tasklet);
    }
#[no_mangle]
unsafe extern "C" fn init_ixp_crypto(dev: *mut device) -> c_int {
    static int init_ixp_crypto(struct device *dev)
    {
    struct device_node *np = dev.of_node;
    u32 msg[2] = { 0, 0 };
    let mut ret: c_int = -ENODEV;
    u32 npe_id;
    dev_info(dev, "probing...\n");
// Locate the NPE and queue manager to use from device tree
    if (IS_ENABLED(CONFIG_OF) && np) {
    struct of_phandle_args queue_spec;
    struct of_phandle_args npe_spec;
    ret = of_parse_phandle_with_fixed_args(np, "intel,npe-handle",
    1, 0, &npe_spec);
    if (ret) {
    dev_err(dev, "no NPE engine specified\n");
    return -ENODEV;
    }
    npe_id = npe_spec.args[0];
    of_node_put(npe_spec.np);
    ret = of_parse_phandle_with_fixed_args(np, "queue-rx", 1, 0,
    &queue_spec);
    if (ret) {
    dev_err(dev, "no rx queue phandle\n");
    return -ENODEV;
    }
    recv_qid = queue_spec.args[0];
    of_node_put(queue_spec.np);
    ret = of_parse_phandle_with_fixed_args(np, "queue-txready", 1, 0,
    &queue_spec);
    if (ret) {
    dev_err(dev, "no txready queue phandle\n");
    return -ENODEV;
    }
    send_qid = queue_spec.args[0];
    of_node_put(queue_spec.np);
    } else {
//
// Hardcoded engine when using platform data, this goes away
// when we switch to using DT only.
//
    npe_id = 2;
    send_qid = 29;
    recv_qid = 30;
    }
    npe_c = npe_request(npe_id);
    if (!npe_c)
    return ret;
    if (!npe_running(npe_c)) {
    ret = npe_load_firmware(npe_c, npe_name(npe_c), dev);
    if (ret)
    goto npe_release;
    if (npe_recv_message(npe_c, msg, "STATUS_MSG"))
    goto npe_error;
    } else {
    if (npe_send_message(npe_c, msg, "STATUS_MSG"))
    goto npe_error;
    if (npe_recv_message(npe_c, msg, "STATUS_MSG"))
    goto npe_error;
    }
    switch ((msg[1] >> 16) & 0xff) {
    case 3:
    dev_warn(dev, "Firmware of %s lacks AES support\n", npe_name(npe_c));
    support_aes = 0;
    break;
    case 4:
    case 5:
    support_aes = 1;
    break;
    default:
    dev_err(dev, "Firmware of %s lacks crypto support\n", npe_name(npe_c));
    ret = -ENODEV;
    goto npe_release;
    }
// buffer_pool will also be used to sometimes store the hmac,
// so assure it is large enough
//
    BUILD_BUG_ON(SHA1_DIGEST_SIZE > sizeof(struct buffer_desc));
    buffer_pool = dma_pool_create("buffer", dev, sizeof(struct buffer_desc),
    32, 0);
    ret = -ENOMEM;
    if (!buffer_pool)
    goto err;
    ctx_pool = dma_pool_create("context", dev, NPE_CTX_LEN, 16, 0);
    if (!ctx_pool)
    goto err;
    ret = qmgr_request_queue(send_qid, NPE_QLEN_TOTAL, 0, 0,
    "ixp_crypto:out", core::ptr::null_mut());
    if (ret)
    goto err;
    ret = qmgr_request_queue(recv_qid, NPE_QLEN, 0, 0,
    "ixp_crypto:in", core::ptr::null_mut());
    if (ret) {
    qmgr_release_queue(send_qid);
    goto err;
    }
    qmgr_set_irq(recv_qid, QUEUE_IRQ_SRC_NOT_EMPTY, irqhandler, core::ptr::null_mut());
    tasklet_init(&crypto_done_tasklet, crypto_done_action, 0);
    qmgr_enable_irq(recv_qid);
    return 0;
    npe_error:
    dev_err(dev, "%s not responding\n", npe_name(npe_c));
    ret = -EIO;
    err:
    dma_pool_destroy(ctx_pool);
    dma_pool_destroy(buffer_pool);
    npe_release:
    npe_release(npe_c);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn release_ixp_crypto(dev: *mut device) {
    static void release_ixp_crypto(struct device *dev)
    {
    qmgr_disable_irq(recv_qid);
    tasklet_kill(&crypto_done_tasklet);
    qmgr_release_queue(send_qid);
    qmgr_release_queue(recv_qid);
    dma_pool_destroy(ctx_pool);
    dma_pool_destroy(buffer_pool);
    npe_release(npe_c);
    if (crypt_virt)
    dma_free_coherent(dev, NPE_QLEN * sizeof(struct crypt_ctl),
    crypt_virt, crypt_phys);
    }
#[no_mangle]
unsafe extern "C" fn reset_sa_dir(dir: *mut ix_sa_dir) {
    static void reset_sa_dir(struct ix_sa_dir *dir)
    {
    memset(dir.npe_ctx, 0, NPE_CTX_LEN);
    dir.npe_ctx_idx = 0;
    dir.npe_mode = 0;
    }
#[no_mangle]
unsafe extern "C" fn init_sa_dir(dir: *mut ix_sa_dir) -> c_int {
    static int init_sa_dir(struct ix_sa_dir *dir)
    {
    dir.npe_ctx = dma_pool_alloc(ctx_pool, GFP_KERNEL, &dir.npe_ctx_phys);
    if (!dir.npe_ctx)
    return -ENOMEM;
    reset_sa_dir(dir);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_sa_dir(dir: *mut ix_sa_dir) {
    static void free_sa_dir(struct ix_sa_dir *dir)
    {
    memset(dir.npe_ctx, 0, NPE_CTX_LEN);
    dma_pool_free(ctx_pool, dir.npe_ctx, dir.npe_ctx_phys);
    }
#[no_mangle]
unsafe extern "C" fn init_tfm(tfm: *mut crypto_tfm) -> c_int {
    static int init_tfm(struct crypto_tfm *tfm)
    {
    struct ixp_ctx *ctx = crypto_tfm_ctx(tfm);
    int ret;
    atomic_set(&ctx.configuring, 0);
    ret = init_sa_dir(&ctx.encrypt);
    if (ret)
    return ret;
    ret = init_sa_dir(&ctx.decrypt);
    if (ret)
    free_sa_dir(&ctx.encrypt);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn init_tfm_ablk(tfm: *mut crypto_skcipher) -> c_int {
    static int init_tfm_ablk(struct crypto_skcipher *tfm)
    {
    struct crypto_tfm *ctfm = crypto_skcipher_tfm(tfm);
    struct ixp_ctx *ctx = crypto_tfm_ctx(ctfm);
    const char *name = crypto_tfm_alg_name(ctfm);
    ctx.fallback_tfm = crypto_alloc_skcipher(name, 0, CRYPTO_ALG_NEED_FALLBACK);
    if (IS_ERR(ctx.fallback_tfm)) {
    pr_err("ERROR: Cannot allocate fallback for %s %ld\n",
    name, PTR_ERR(ctx.fallback_tfm));
    return PTR_ERR(ctx.fallback_tfm);
    }
    pr_info("Fallback for %s is %s\n",
    crypto_tfm_alg_driver_name(&tfm.base),
    crypto_tfm_alg_driver_name(crypto_skcipher_tfm(ctx.fallback_tfm))
    );
    crypto_skcipher_set_reqsize(tfm, sizeof(struct ablk_ctx) + crypto_skcipher_reqsize(ctx.fallback_tfm));
    return init_tfm(crypto_skcipher_tfm(tfm));
    }
#[no_mangle]
unsafe extern "C" fn init_tfm_aead(tfm: *mut crypto_aead) -> c_int {
    static int init_tfm_aead(struct crypto_aead *tfm)
    {
    crypto_aead_set_reqsize(tfm, sizeof(struct aead_ctx));
    return init_tfm(crypto_aead_tfm(tfm));
    }
#[no_mangle]
unsafe extern "C" fn exit_tfm(tfm: *mut crypto_tfm) {
    static void exit_tfm(struct crypto_tfm *tfm)
    {
    struct ixp_ctx *ctx = crypto_tfm_ctx(tfm);
    free_sa_dir(&ctx.encrypt);
    free_sa_dir(&ctx.decrypt);
    }
#[no_mangle]
unsafe extern "C" fn exit_tfm_ablk(tfm: *mut crypto_skcipher) {
    static void exit_tfm_ablk(struct crypto_skcipher *tfm)
    {
    struct crypto_tfm *ctfm = crypto_skcipher_tfm(tfm);
    struct ixp_ctx *ctx = crypto_tfm_ctx(ctfm);
    crypto_free_skcipher(ctx.fallback_tfm);
    exit_tfm(crypto_skcipher_tfm(tfm));
    }
#[no_mangle]
unsafe extern "C" fn exit_tfm_aead(tfm: *mut crypto_aead) {
    static void exit_tfm_aead(struct crypto_aead *tfm)
    {
    exit_tfm(crypto_aead_tfm(tfm));
    }
    static int register_chain_var(struct crypto_tfm *tfm, u8 xpad, u32 target,
    int init_len, u32 ctx_addr, const u8 *key,
    int key_len)
    {
    struct ixp_ctx *ctx = crypto_tfm_ctx(tfm);
    struct crypt_ctl *crypt;
    struct buffer_desc *buf;
    int i;
    u8 *pad;
    dma_addr_t pad_phys, buf_phys;
    BUILD_BUG_ON(NPE_CTX_LEN < HMAC_PAD_BLOCKLEN);
    pad = dma_pool_alloc(ctx_pool, GFP_KERNEL, &pad_phys);
    if (!pad)
    return -ENOMEM;
    buf = dma_pool_alloc(buffer_pool, GFP_KERNEL, &buf_phys);
    if (!buf) {
    dma_pool_free(ctx_pool, pad, pad_phys);
    return -ENOMEM;
    }
    crypt = get_crypt_desc_emerg();
    if (!crypt) {
    dma_pool_free(ctx_pool, pad, pad_phys);
    dma_pool_free(buffer_pool, buf, buf_phys);
    return -EAGAIN;
    }
    memcpy(pad, key, key_len);
    memset(pad + key_len, 0, HMAC_PAD_BLOCKLEN - key_len);
    for (i = 0; i < HMAC_PAD_BLOCKLEN; i++)
    pad[i] ^= xpad;
    crypt.data.tfm = tfm;
    crypt.regist_ptr = pad;
    crypt.regist_buf = buf;
    crypt.auth_offs = 0;
    crypt.auth_len = HMAC_PAD_BLOCKLEN;
    crypt.crypto_ctx = ctx_addr;
    crypt.src_buf = buf_phys;
    crypt.icv_rev_aes = target;
    crypt.mode = NPE_OP_HASH_GEN_ICV;
    crypt.init_len = init_len;
    crypt.ctl_flags |= CTL_FLAG_GEN_ICV;
    buf.next = core::ptr::null_mut();
    buf.buf_len = HMAC_PAD_BLOCKLEN;
    buf.pkt_len = 0;
    buf.phys_addr = pad_phys;
    atomic_inc(&ctx.configuring);
    qmgr_put_entry(send_qid, crypt_virt2phys(crypt));
    BUG_ON(qmgr_stat_overflow(send_qid));
    return 0;
    }
    static int setup_auth(struct crypto_tfm *tfm, int encrypt, unsigned int authsize,
    const u8 *key, int key_len, unsigned int digest_len)
    {
    u32 itarget, otarget, npe_ctx_addr;
    unsigned char *cinfo;
    int init_len, ret = 0;
    u32 cfgword;
    struct ix_sa_dir *dir;
    struct ixp_ctx *ctx = crypto_tfm_ctx(tfm);
    const struct ix_hash_algo *algo;
    dir = encrypt ? &ctx.encrypt : &ctx.decrypt;
    cinfo = dir.npe_ctx + dir.npe_ctx_idx;
    algo = ix_hash(tfm);
// write cfg word to cryptinfo
    cfgword = algo.cfgword | (authsize << 6); /* (authsize/4) << 8 */

    cfgword ^= 0xAA000000; /* change the "byte swap" flags */

// (__be32 *)cinfo = cpu_to_be32(cfgword);
    cinfo += sizeof(cfgword);
// write ICV to cryptinfo
    memcpy(cinfo, algo.icv, digest_len);
    cinfo += digest_len;
    itarget = dir.npe_ctx_phys + dir.npe_ctx_idx
    + sizeof(algo.cfgword);
    otarget = itarget + digest_len;
    init_len = cinfo - (dir.npe_ctx + dir.npe_ctx_idx);
    npe_ctx_addr = dir.npe_ctx_phys + dir.npe_ctx_idx;
    dir.npe_ctx_idx += init_len;
    dir.npe_mode |= NPE_OP_HASH_ENABLE;
    if (!encrypt)
    dir.npe_mode |= NPE_OP_HASH_VERIFY;
    ret = register_chain_var(tfm, HMAC_OPAD_VALUE, otarget,
    init_len, npe_ctx_addr, key, key_len);
    if (ret)
    return ret;
    return register_chain_var(tfm, HMAC_IPAD_VALUE, itarget,
    init_len, npe_ctx_addr, key, key_len);
    }
#[no_mangle]
unsafe extern "C" fn gen_rev_aes_key(tfm: *mut crypto_tfm) -> c_int {
    static int gen_rev_aes_key(struct crypto_tfm *tfm)
    {
    struct crypt_ctl *crypt;
    struct ixp_ctx *ctx = crypto_tfm_ctx(tfm);
    struct ix_sa_dir *dir = &ctx.decrypt;
    crypt = get_crypt_desc_emerg();
    if (!crypt)
    return -EAGAIN;
// (__be32 *)dir->npe_ctx |= cpu_to_be32(CIPH_ENCR);
    crypt.data.tfm = tfm;
    crypt.crypt_offs = 0;
    crypt.crypt_len = AES_BLOCK128;
    crypt.src_buf = 0;
    crypt.crypto_ctx = dir.npe_ctx_phys;
    crypt.icv_rev_aes = dir.npe_ctx_phys + sizeof(u32);
    crypt.mode = NPE_OP_ENC_GEN_KEY;
    crypt.init_len = dir.npe_ctx_idx;
    crypt.ctl_flags |= CTL_FLAG_GEN_REVAES;
    atomic_inc(&ctx.configuring);
    qmgr_put_entry(send_qid, crypt_virt2phys(crypt));
    BUG_ON(qmgr_stat_overflow(send_qid));
    return 0;
    }
    static int setup_cipher(struct crypto_tfm *tfm, int encrypt, const u8 *key,
    int key_len)
    {
    u8 *cinfo;
    u32 cipher_cfg;
    let mut keylen_cfg: u32 = 0;
    struct ix_sa_dir *dir;
    struct ixp_ctx *ctx = crypto_tfm_ctx(tfm);
    int err;
    dir = encrypt ? &ctx.encrypt : &ctx.decrypt;
    cinfo = dir.npe_ctx;
    if (encrypt) {
    cipher_cfg = cipher_cfg_enc(tfm);
    dir.npe_mode |= NPE_OP_CRYPT_ENCRYPT;
    } else {
    cipher_cfg = cipher_cfg_dec(tfm);
    }
    if (cipher_cfg & MOD_AES) {
    switch (key_len) {
    case 16:
    keylen_cfg = MOD_AES128;
    break;
    case 24:
    keylen_cfg = MOD_AES192;
    break;
    case 32:
    keylen_cfg = MOD_AES256;
    break;
    default:
    return -EINVAL;
    }
    cipher_cfg |= keylen_cfg;
    } else {
    err = crypto_des_verify_key(tfm, key);
    if (err)
    return err;
    }
// write cfg word to cryptinfo
// (__be32 *)cinfo = cpu_to_be32(cipher_cfg);
    cinfo += sizeof(cipher_cfg);
// write cipher key to cryptinfo
    memcpy(cinfo, key, key_len);
// NPE wants keylen set to DES3_EDE_KEY_SIZE even for single DES
    if (key_len < DES3_EDE_KEY_SIZE && !(cipher_cfg & MOD_AES)) {
    memset(cinfo + key_len, 0, DES3_EDE_KEY_SIZE - key_len);
    key_len = DES3_EDE_KEY_SIZE;
    }
    dir.npe_ctx_idx = sizeof(cipher_cfg) + key_len;
    dir.npe_mode |= NPE_OP_CRYPT_ENABLE;
    if ((cipher_cfg & MOD_AES) && !encrypt)
    return gen_rev_aes_key(tfm);
    return 0;
    }
    static struct buffer_desc *chainup_buffers(struct device *dev,
    struct scatterlist *sg,	unsigned int nbytes,
    struct buffer_desc *buf, gfp_t flags,
    enum dma_data_direction dir)
    {
    for (; nbytes > 0; sg = sg_next(sg)) {
    let mut len: c_uint = min(nbytes, sg.length);
    struct buffer_desc *next_buf;
    dma_addr_t next_buf_phys;
    void *ptr;
    nbytes -= len;
    ptr = sg_virt(sg);
    next_buf = dma_pool_alloc(buffer_pool, flags, &next_buf_phys);
    if (!next_buf) {
    buf.next = core::ptr::null_mut();
    buf.phys_next = 0;
    return core::ptr::null_mut();
    }
    sg_dma_address(sg) = dma_map_single(dev, ptr, len, dir);
    buf.next = next_buf;
    buf.phys_next = next_buf_phys;
    buf = next_buf;
    buf.phys_addr = sg_dma_address(sg);
    buf.buf_len = len;
    buf.dir = dir;
    }
    buf.next = core::ptr::null_mut();
    buf.phys_next = 0;
    return buf;
    }
    static int ablk_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int key_len)
    {
    struct ixp_ctx *ctx = crypto_skcipher_ctx(tfm);
    int ret;
    init_completion(&ctx.completion);
    atomic_inc(&ctx.configuring);
    reset_sa_dir(&ctx.encrypt);
    reset_sa_dir(&ctx.decrypt);
    ctx.encrypt.npe_mode = NPE_OP_HMAC_DISABLE;
    ctx.decrypt.npe_mode = NPE_OP_HMAC_DISABLE;
    ret = setup_cipher(&tfm.base, 0, key, key_len);
    if (ret)
    goto out;
    ret = setup_cipher(&tfm.base, 1, key, key_len);
    out:
    if (!atomic_dec_and_test(&ctx.configuring))
    wait_for_completion(&ctx.completion);
    if (ret)
    return ret;
    crypto_skcipher_clear_flags(ctx.fallback_tfm, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(ctx.fallback_tfm, tfm.base.crt_flags & CRYPTO_TFM_REQ_MASK);
    return crypto_skcipher_setkey(ctx.fallback_tfm, key, key_len);
    }
    static int ablk_des3_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int key_len)
    {
    return verify_skcipher_des3_key(tfm, key) ?:
    ablk_setkey(tfm, key, key_len);
    }
    static int ablk_rfc3686_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int key_len)
    {
    struct ixp_ctx *ctx = crypto_skcipher_ctx(tfm);
// the nonce is stored in bytes at end of key
    if (key_len < CTR_RFC3686_NONCE_SIZE)
    return -EINVAL;
    memcpy(ctx.nonce, key + (key_len - CTR_RFC3686_NONCE_SIZE),
    CTR_RFC3686_NONCE_SIZE);
    key_len -= CTR_RFC3686_NONCE_SIZE;
    return ablk_setkey(tfm, key, key_len);
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_cipher_fallback(areq: *mut skcipher_request, encrypt: c_int) -> c_int {
    static int ixp4xx_cipher_fallback(struct skcipher_request *areq, int encrypt)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(areq);
    struct ixp_ctx *op = crypto_skcipher_ctx(tfm);
    struct ablk_ctx *rctx = skcipher_request_ctx(areq);
    int err;
    skcipher_request_set_tfm(&rctx.fallback_req, op.fallback_tfm);
    skcipher_request_set_callback(&rctx.fallback_req, areq.base.flags,
    areq.base.complete, areq.base.data);
    skcipher_request_set_crypt(&rctx.fallback_req, areq.src, areq.dst,
    areq.cryptlen, areq.iv);
    if (encrypt)
    err = crypto_skcipher_encrypt(&rctx.fallback_req);
    else
    err = crypto_skcipher_decrypt(&rctx.fallback_req);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ablk_perform(req: *mut skcipher_request, encrypt: c_int) -> c_int {
    static int ablk_perform(struct skcipher_request *req, int encrypt)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct ixp_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut ivsize: c_uint = crypto_skcipher_ivsize(tfm);
    struct ix_sa_dir *dir;
    struct crypt_ctl *crypt;
    let mut nbytes: c_uint = req.cryptlen;
    let mut src_direction: enum dma_data_direction = DMA_BIDIRECTIONAL;
    struct ablk_ctx *req_ctx = skcipher_request_ctx(req);
    struct buffer_desc *buf, src_hook;
    struct device *dev = &pdev.dev;
    unsigned int offset;
    gfp_t flags = req.base.flags & CRYPTO_TFM_REQ_MAY_SLEEP ?
    GFP_KERNEL : GFP_ATOMIC;
    if (sg_nents(req.src) > 1 || sg_nents(req.dst) > 1)
    return ixp4xx_cipher_fallback(req, encrypt);
    if (qmgr_stat_full(send_qid))
    return -EAGAIN;
    if (atomic_read(&ctx.configuring))
    return -EAGAIN;
    dir = encrypt ? &ctx.encrypt : &ctx.decrypt;
    req_ctx.encrypt = encrypt;
    crypt = get_crypt_desc();
    if (!crypt)
    return -ENOMEM;
    crypt.data.ablk_req = req;
    crypt.crypto_ctx = dir.npe_ctx_phys;
    crypt.mode = dir.npe_mode;
    crypt.init_len = dir.npe_ctx_idx;
    crypt.crypt_offs = 0;
    crypt.crypt_len = nbytes;
    BUG_ON(ivsize && !req.iv);
    memcpy(crypt.iv, req.iv, ivsize);
    if (ivsize > 0 && !encrypt) {
    offset = req.cryptlen - ivsize;
    scatterwalk_map_and_copy(req_ctx.iv, req.src, offset, ivsize, 0);
    }
    if (req.src != req.dst) {
    struct buffer_desc dst_hook;
    crypt.mode |= NPE_OP_NOT_IN_PLACE;
// This was never tested by Intel
// for more than one dst buffer, I think.
    req_ctx.dst = core::ptr::null_mut();
    buf = chainup_buffers(dev, req.dst, nbytes, &dst_hook,
    flags, DMA_FROM_DEVICE);
    req_ctx.dst = dst_hook.next;
    crypt.dst_buf = dst_hook.phys_next;
    if (!buf)
    goto free_buf_dest;
    src_direction = DMA_TO_DEVICE;
    } else {
    req_ctx.dst = core::ptr::null_mut();
    }
    req_ctx.src = core::ptr::null_mut();
    buf = chainup_buffers(dev, req.src, nbytes, &src_hook, flags,
    src_direction);
    req_ctx.src = src_hook.next;
    crypt.src_buf = src_hook.phys_next;
    if (!buf)
    goto free_buf_src;
    crypt.ctl_flags |= CTL_FLAG_PERFORM_ABLK;
    qmgr_put_entry(send_qid, crypt_virt2phys(crypt));
    BUG_ON(qmgr_stat_overflow(send_qid));
    return -EINPROGRESS;
    free_buf_src:
    free_buf_chain(dev, req_ctx.src, crypt.src_buf);
    free_buf_dest:
    if (req.src != req.dst)
    free_buf_chain(dev, req_ctx.dst, crypt.dst_buf);
    crypt.ctl_flags = CTL_FLAG_UNUSED;
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn ablk_encrypt(req: *mut skcipher_request) -> c_int {
    static int ablk_encrypt(struct skcipher_request *req)
    {
    return ablk_perform(req, 1);
    }
#[no_mangle]
unsafe extern "C" fn ablk_decrypt(req: *mut skcipher_request) -> c_int {
    static int ablk_decrypt(struct skcipher_request *req)
    {
    return ablk_perform(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn ablk_rfc3686_crypt(req: *mut skcipher_request) -> c_int {
    static int ablk_rfc3686_crypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct ixp_ctx *ctx = crypto_skcipher_ctx(tfm);
    u8 iv[CTR_RFC3686_BLOCK_SIZE];
    u8 *info = req.iv;
    int ret;
// set up counter block
    memcpy(iv, ctx.nonce, CTR_RFC3686_NONCE_SIZE);
    memcpy(iv + CTR_RFC3686_NONCE_SIZE, info, CTR_RFC3686_IV_SIZE);
// initialize counter portion of counter block
// (__be32 *)(iv + CTR_RFC3686_NONCE_SIZE + CTR_RFC3686_IV_SIZE) =
    cpu_to_be32(1);
    req.iv = iv;
    ret = ablk_perform(req, 1);
    req.iv = info;
    return ret;
    }
    static int aead_perform(struct aead_request *req, int encrypt,
    int cryptoffset, int eff_cryptlen, u8 *iv)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    struct ixp_ctx *ctx = crypto_aead_ctx(tfm);
    let mut ivsize: c_uint = crypto_aead_ivsize(tfm);
    let mut authsize: c_uint = crypto_aead_authsize(tfm);
    struct ix_sa_dir *dir;
    struct crypt_ctl *crypt;
    unsigned int cryptlen;
    struct buffer_desc *buf, src_hook;
    struct aead_ctx *req_ctx = aead_request_ctx(req);
    struct device *dev = &pdev.dev;
    gfp_t flags = req.base.flags & CRYPTO_TFM_REQ_MAY_SLEEP ?
    GFP_KERNEL : GFP_ATOMIC;
    let mut src_direction: enum dma_data_direction = DMA_BIDIRECTIONAL;
    unsigned int lastlen;
    if (qmgr_stat_full(send_qid))
    return -EAGAIN;
    if (atomic_read(&ctx.configuring))
    return -EAGAIN;
    if (encrypt) {
    dir = &ctx.encrypt;
    cryptlen = req.cryptlen;
    } else {
    dir = &ctx.decrypt;
// req->cryptlen includes the authsize when decrypting
    cryptlen = req.cryptlen - authsize;
    eff_cryptlen -= authsize;
    }
    crypt = get_crypt_desc();
    if (!crypt)
    return -ENOMEM;
    crypt.data.aead_req = req;
    crypt.crypto_ctx = dir.npe_ctx_phys;
    crypt.mode = dir.npe_mode;
    crypt.init_len = dir.npe_ctx_idx;
    crypt.crypt_offs = cryptoffset;
    crypt.crypt_len = eff_cryptlen;
    crypt.auth_offs = 0;
    crypt.auth_len = req.assoclen + cryptlen;
    BUG_ON(ivsize && !req.iv);
    memcpy(crypt.iv, req.iv, ivsize);
    buf = chainup_buffers(dev, req.src, crypt.auth_len,
    &src_hook, flags, src_direction);
    req_ctx.src = src_hook.next;
    crypt.src_buf = src_hook.phys_next;
    if (!buf)
    goto free_buf_src;
    lastlen = buf.buf_len;
    if (lastlen >= authsize)
    crypt.icv_rev_aes = buf.phys_addr +
    buf.buf_len - authsize;
    req_ctx.dst = core::ptr::null_mut();
    if (req.src != req.dst) {
    struct buffer_desc dst_hook;
    crypt.mode |= NPE_OP_NOT_IN_PLACE;
    src_direction = DMA_TO_DEVICE;
    buf = chainup_buffers(dev, req.dst, crypt.auth_len,
    &dst_hook, flags, DMA_FROM_DEVICE);
    req_ctx.dst = dst_hook.next;
    crypt.dst_buf = dst_hook.phys_next;
    if (!buf)
    goto free_buf_dst;
    if (encrypt) {
    lastlen = buf.buf_len;
    if (lastlen >= authsize)
    crypt.icv_rev_aes = buf.phys_addr +
    buf.buf_len - authsize;
    }
    }
    if (unlikely(lastlen < authsize)) {
    dma_addr_t dma;
// The 12 hmac bytes are scattered,
// we need to copy them into a safe buffer
    req_ctx.hmac_virt = dma_pool_alloc(buffer_pool, flags, &dma);
    if (unlikely(!req_ctx.hmac_virt))
    goto free_buf_dst;
    crypt.icv_rev_aes = dma;
    if (!encrypt) {
    scatterwalk_map_and_copy(req_ctx.hmac_virt,
    req.src, cryptlen, authsize, 0);
    }
    req_ctx.encrypt = encrypt;
    } else {
    req_ctx.hmac_virt = core::ptr::null_mut();
    }
    crypt.ctl_flags |= CTL_FLAG_PERFORM_AEAD;
    qmgr_put_entry(send_qid, crypt_virt2phys(crypt));
    BUG_ON(qmgr_stat_overflow(send_qid));
    return -EINPROGRESS;
    free_buf_dst:
    free_buf_chain(dev, req_ctx.dst, crypt.dst_buf);
    free_buf_src:
    free_buf_chain(dev, req_ctx.src, crypt.src_buf);
    crypt.ctl_flags = CTL_FLAG_UNUSED;
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn aead_setup(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    static int aead_setup(struct crypto_aead *tfm, unsigned int authsize)
    {
    struct ixp_ctx *ctx = crypto_aead_ctx(tfm);
    let mut digest_len: c_uint = crypto_aead_maxauthsize(tfm);
    int ret;
    if (!ctx.enckey_len && !ctx.authkey_len)
    return 0;
    init_completion(&ctx.completion);
    atomic_inc(&ctx.configuring);
    reset_sa_dir(&ctx.encrypt);
    reset_sa_dir(&ctx.decrypt);
    ret = setup_cipher(&tfm.base, 0, ctx.enckey, ctx.enckey_len);
    if (ret)
    goto out;
    ret = setup_cipher(&tfm.base, 1, ctx.enckey, ctx.enckey_len);
    if (ret)
    goto out;
    ret = setup_auth(&tfm.base, 0, authsize, ctx.authkey,
    ctx.authkey_len, digest_len);
    if (ret)
    goto out;
    ret = setup_auth(&tfm.base, 1, authsize,  ctx.authkey,
    ctx.authkey_len, digest_len);
    out:
    if (!atomic_dec_and_test(&ctx.configuring))
    wait_for_completion(&ctx.completion);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aead_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    static int aead_setauthsize(struct crypto_aead *tfm, unsigned int authsize)
    {
    let mut max: c_int = crypto_aead_maxauthsize(tfm) >> 2;
    if ((authsize >> 2) < 1 || (authsize >> 2) > max || (authsize & 3))
    return -EINVAL;
    return aead_setup(tfm, authsize);
    }
    static int aead_setkey(struct crypto_aead *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct ixp_ctx *ctx = crypto_aead_ctx(tfm);
    struct crypto_authenc_keys keys;
    if (crypto_authenc_extractkeys(&keys, key, keylen) != 0)
    goto badkey;
    if (keys.authkeylen > sizeof(ctx.authkey))
    goto badkey;
    if (keys.enckeylen > sizeof(ctx.enckey))
    goto badkey;
    memcpy(ctx.authkey, keys.authkey, keys.authkeylen);
    memcpy(ctx.enckey, keys.enckey, keys.enckeylen);
    ctx.authkey_len = keys.authkeylen;
    ctx.enckey_len = keys.enckeylen;
    memzero_explicit(&keys, sizeof(keys));
    return aead_setup(tfm, crypto_aead_authsize(tfm));
    badkey:
    memzero_explicit(&keys, sizeof(keys));
    return -EINVAL;
    }
    static int des3_aead_setkey(struct crypto_aead *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct ixp_ctx *ctx = crypto_aead_ctx(tfm);
    struct crypto_authenc_keys keys;
    int err;
    err = crypto_authenc_extractkeys(&keys, key, keylen);
    if (unlikely(err))
    goto badkey;
    err = -EINVAL;
    if (keys.authkeylen > sizeof(ctx.authkey))
    goto badkey;
    err = verify_aead_des3_key(tfm, keys.enckey, keys.enckeylen);
    if (err)
    goto badkey;
    memcpy(ctx.authkey, keys.authkey, keys.authkeylen);
    memcpy(ctx.enckey, keys.enckey, keys.enckeylen);
    ctx.authkey_len = keys.authkeylen;
    ctx.enckey_len = keys.enckeylen;
    memzero_explicit(&keys, sizeof(keys));
    return aead_setup(tfm, crypto_aead_authsize(tfm));
    badkey:
    memzero_explicit(&keys, sizeof(keys));
    return err;
    }
#[no_mangle]
unsafe extern "C" fn aead_encrypt(req: *mut aead_request) -> c_int {
    static int aead_encrypt(struct aead_request *req)
    {
    return aead_perform(req, 1, req.assoclen, req.cryptlen, req.iv);
    }
#[no_mangle]
unsafe extern "C" fn aead_decrypt(req: *mut aead_request) -> c_int {
    static int aead_decrypt(struct aead_request *req)
    {
    return aead_perform(req, 0, req.assoclen, req.cryptlen, req.iv);
    }
    static struct ixp_alg ixp4xx_algos[] = {
    {
    .crypto	= {
    .base.cra_name		= "cbc(des)",
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .min_keysize		= DES_KEY_SIZE,
    .max_keysize		= DES_KEY_SIZE,
    .ivsize			= DES_BLOCK_SIZE,
    },
    .cfg_enc = CIPH_ENCR | MOD_DES | MOD_CBC_ENC | KEYLEN_192,
    .cfg_dec = CIPH_DECR | MOD_DES | MOD_CBC_DEC | KEYLEN_192,
    }, {
    .crypto	= {
    .base.cra_name		= "ecb(des)",
    .base.cra_blocksize	= DES_BLOCK_SIZE,
    .min_keysize		= DES_KEY_SIZE,
    .max_keysize		= DES_KEY_SIZE,
    },
    .cfg_enc = CIPH_ENCR | MOD_DES | MOD_ECB | KEYLEN_192,
    .cfg_dec = CIPH_DECR | MOD_DES | MOD_ECB | KEYLEN_192,
    }, {
    .crypto	= {
    .base.cra_name		= "cbc(des3_ede)",
    .base.cra_blocksize	= DES3_EDE_BLOCK_SIZE,
    .min_keysize		= DES3_EDE_KEY_SIZE,
    .max_keysize		= DES3_EDE_KEY_SIZE,
    .ivsize			= DES3_EDE_BLOCK_SIZE,
    .setkey			= ablk_des3_setkey,
    },
    .cfg_enc = CIPH_ENCR | MOD_3DES | MOD_CBC_ENC | KEYLEN_192,
    .cfg_dec = CIPH_DECR | MOD_3DES | MOD_CBC_DEC | KEYLEN_192,
    }, {
    .crypto	= {
    .base.cra_name		= "ecb(des3_ede)",
    .base.cra_blocksize	= DES3_EDE_BLOCK_SIZE,
    .min_keysize		= DES3_EDE_KEY_SIZE,
    .max_keysize		= DES3_EDE_KEY_SIZE,
    .setkey			= ablk_des3_setkey,
    },
    .cfg_enc = CIPH_ENCR | MOD_3DES | MOD_ECB | KEYLEN_192,
    .cfg_dec = CIPH_DECR | MOD_3DES | MOD_ECB | KEYLEN_192,
    }, {
    .crypto	= {
    .base.cra_name		= "cbc(aes)",
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .ivsize			= AES_BLOCK_SIZE,
    },
    .cfg_enc = CIPH_ENCR | MOD_AES | MOD_CBC_ENC,
    .cfg_dec = CIPH_DECR | MOD_AES | MOD_CBC_DEC,
    }, {
    .crypto	= {
    .base.cra_name		= "ecb(aes)",
    .base.cra_blocksize	= AES_BLOCK_SIZE,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    },
    .cfg_enc = CIPH_ENCR | MOD_AES | MOD_ECB,
    .cfg_dec = CIPH_DECR | MOD_AES | MOD_ECB,
    }, {
    .crypto	= {
    .base.cra_name		= "ctr(aes)",
    .base.cra_blocksize	= 1,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .ivsize			= AES_BLOCK_SIZE,
    },
    .cfg_enc = CIPH_ENCR | MOD_AES | MOD_CTR,
    .cfg_dec = CIPH_ENCR | MOD_AES | MOD_CTR,
    }, {
    .crypto	= {
    .base.cra_name		= "rfc3686(ctr(aes))",
    .base.cra_blocksize	= 1,
    .min_keysize		= AES_MIN_KEY_SIZE,
    .max_keysize		= AES_MAX_KEY_SIZE,
    .ivsize			= AES_BLOCK_SIZE,
    .setkey			= ablk_rfc3686_setkey,
    .encrypt		= ablk_rfc3686_crypt,
    .decrypt		= ablk_rfc3686_crypt,
    },
    .cfg_enc = CIPH_ENCR | MOD_AES | MOD_CTR,
    .cfg_dec = CIPH_ENCR | MOD_AES | MOD_CTR,
    } };
    static struct ixp_aead_alg ixp4xx_aeads[] = {
    {
    .crypto	= {
    .base = {
    .cra_name	= "authenc(hmac(md5),cbc(des))",
    .cra_blocksize	= DES_BLOCK_SIZE,
    },
    .ivsize		= DES_BLOCK_SIZE,
    .maxauthsize	= MD5_DIGEST_SIZE,
    },
    .hash = &hash_alg_md5,
    .cfg_enc = CIPH_ENCR | MOD_DES | MOD_CBC_ENC | KEYLEN_192,
    .cfg_dec = CIPH_DECR | MOD_DES | MOD_CBC_DEC | KEYLEN_192,
    }, {
    .crypto	= {
    .base = {
    .cra_name	= "authenc(hmac(md5),cbc(des3_ede))",
    .cra_blocksize	= DES3_EDE_BLOCK_SIZE,
    },
    .ivsize		= DES3_EDE_BLOCK_SIZE,
    .maxauthsize	= MD5_DIGEST_SIZE,
    .setkey		= des3_aead_setkey,
    },
    .hash = &hash_alg_md5,
    .cfg_enc = CIPH_ENCR | MOD_3DES | MOD_CBC_ENC | KEYLEN_192,
    .cfg_dec = CIPH_DECR | MOD_3DES | MOD_CBC_DEC | KEYLEN_192,
    }, {
    .crypto	= {
    .base = {
    .cra_name	= "authenc(hmac(sha1),cbc(des))",
    .cra_blocksize	= DES_BLOCK_SIZE,
    },
    .ivsize		= DES_BLOCK_SIZE,
    .maxauthsize	= SHA1_DIGEST_SIZE,
    },
    .hash = &hash_alg_sha1,
    .cfg_enc = CIPH_ENCR | MOD_DES | MOD_CBC_ENC | KEYLEN_192,
    .cfg_dec = CIPH_DECR | MOD_DES | MOD_CBC_DEC | KEYLEN_192,
    }, {
    .crypto	= {
    .base = {
    .cra_name	= "authenc(hmac(sha1),cbc(des3_ede))",
    .cra_blocksize	= DES3_EDE_BLOCK_SIZE,
    },
    .ivsize		= DES3_EDE_BLOCK_SIZE,
    .maxauthsize	= SHA1_DIGEST_SIZE,
    .setkey		= des3_aead_setkey,
    },
    .hash = &hash_alg_sha1,
    .cfg_enc = CIPH_ENCR | MOD_3DES | MOD_CBC_ENC | KEYLEN_192,
    .cfg_dec = CIPH_DECR | MOD_3DES | MOD_CBC_DEC | KEYLEN_192,
    }, {
    .crypto	= {
    .base = {
    .cra_name	= "authenc(hmac(md5),cbc(aes))",
    .cra_blocksize	= AES_BLOCK_SIZE,
    },
    .ivsize		= AES_BLOCK_SIZE,
    .maxauthsize	= MD5_DIGEST_SIZE,
    },
    .hash = &hash_alg_md5,
    .cfg_enc = CIPH_ENCR | MOD_AES | MOD_CBC_ENC,
    .cfg_dec = CIPH_DECR | MOD_AES | MOD_CBC_DEC,
    }, {
    .crypto	= {
    .base = {
    .cra_name	= "authenc(hmac(sha1),cbc(aes))",
    .cra_blocksize	= AES_BLOCK_SIZE,
    },
    .ivsize		= AES_BLOCK_SIZE,
    .maxauthsize	= SHA1_DIGEST_SIZE,
    },
    .hash = &hash_alg_sha1,
    .cfg_enc = CIPH_ENCR | MOD_AES | MOD_CBC_ENC,
    .cfg_dec = CIPH_DECR | MOD_AES | MOD_CBC_DEC,
    } };

#[no_mangle]
unsafe extern "C" fn ixp_crypto_probe(_pdev: *mut platform_device) -> c_int {
    static int ixp_crypto_probe(struct platform_device *_pdev)
    {
    struct device *dev = &_pdev.dev;
    let mut num: c_int = ARRAY_SIZE(ixp4xx_algos);
    int i, err;
    pdev = _pdev;
    err = init_ixp_crypto(dev);
    if (err)
    return err;
    for (i = 0; i < num; i++) {
    struct skcipher_alg *cra = &ixp4xx_algos[i].crypto;
    if (snprintf(cra.base.cra_driver_name, CRYPTO_MAX_ALG_NAME,
    "%s"IXP_POSTFIX, cra.base.cra_name) >=
    CRYPTO_MAX_ALG_NAME)
    continue;
    if (!support_aes && (ixp4xx_algos[i].cfg_enc & MOD_AES))
    continue;
// block ciphers
    cra.base.cra_flags = CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_ALLOCATES_MEMORY |
    CRYPTO_ALG_NEED_FALLBACK;
    if (!cra.setkey)
    cra.setkey = ablk_setkey;
    if (!cra.encrypt)
    cra.encrypt = ablk_encrypt;
    if (!cra.decrypt)
    cra.decrypt = ablk_decrypt;
    cra.init = init_tfm_ablk;
    cra.exit = exit_tfm_ablk;
    cra.base.cra_ctxsize = sizeof(struct ixp_ctx);
    cra.base.cra_module = THIS_MODULE;
    cra.base.cra_alignmask = 3;
    cra.base.cra_priority = 300;
    if (crypto_register_skcipher(cra))
    dev_err(&pdev.dev, "Failed to register '%s'\n",
    cra.base.cra_name);
    else
    ixp4xx_algos[i].registered = 1;
    }
    for (i = 0; i < ARRAY_SIZE(ixp4xx_aeads); i++) {
    struct aead_alg *cra = &ixp4xx_aeads[i].crypto;
    if (snprintf(cra.base.cra_driver_name, CRYPTO_MAX_ALG_NAME,
    "%s"IXP_POSTFIX, cra.base.cra_name) >=
    CRYPTO_MAX_ALG_NAME)
    continue;
    if (!support_aes && (ixp4xx_algos[i].cfg_enc & MOD_AES))
    continue;
// authenc
    cra.base.cra_flags = CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_ASYNC |
    CRYPTO_ALG_ALLOCATES_MEMORY;
    cra.setkey = cra.setkey ?: aead_setkey;
    cra.setauthsize = aead_setauthsize;
    cra.encrypt = aead_encrypt;
    cra.decrypt = aead_decrypt;
    cra.init = init_tfm_aead;
    cra.exit = exit_tfm_aead;
    cra.base.cra_ctxsize = sizeof(struct ixp_ctx);
    cra.base.cra_module = THIS_MODULE;
    cra.base.cra_alignmask = 3;
    cra.base.cra_priority = 300;
    if (crypto_register_aead(cra))
    dev_err(&pdev.dev, "Failed to register '%s'\n",
    cra.base.cra_driver_name);
    else
    ixp4xx_aeads[i].registered = 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixp_crypto_remove(pdev: *mut platform_device) {
    static void ixp_crypto_remove(struct platform_device *pdev)
    {
    let mut num: c_int = ARRAY_SIZE(ixp4xx_algos);
    int i;
    for (i = 0; i < ARRAY_SIZE(ixp4xx_aeads); i++) {
    if (ixp4xx_aeads[i].registered)
    crypto_unregister_aead(&ixp4xx_aeads[i].crypto);
    }
    for (i = 0; i < num; i++) {
    if (ixp4xx_algos[i].registered)
    crypto_unregister_skcipher(&ixp4xx_algos[i].crypto);
    }
    release_ixp_crypto(&pdev.dev);
    }
    static const struct of_device_id ixp4xx_crypto_of_match[] = {
    {
    .compatible = "intel,ixp4xx-crypto",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, ixp4xx_crypto_of_match);
    static struct platform_driver ixp_crypto_driver = {
    .probe = ixp_crypto_probe,
    .remove = ixp_crypto_remove,
    .driver = {
    .name = "ixp4xx_crypto",
    .of_match_table = ixp4xx_crypto_of_match,
    },
    };
    module_platform_driver(ixp_crypto_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Christian Hohnstaedt <chohnstaedt@innominate.com>");
    MODULE_DESCRIPTION("IXP4xx hardware crypto");
