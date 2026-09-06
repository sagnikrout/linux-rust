//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/xilinx/zynqmp-sha.c
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
// Xilinx ZynqMP SHA Driver.
// Copyright (c) 2022 Xilinx Inc.
//

pub const ZYNQMP_DMA_ALLOC_FIXED_SIZE: c_uint = 0x1000U;
    enum zynqmp_sha_op {
    ZYNQMP_SHA3_INIT = 1,
    ZYNQMP_SHA3_UPDATE = 2,
    ZYNQMP_SHA3_FINAL = 4,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_sha_drv_ctx {
    pub sha3_384: shash_alg,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_sha_tfm_ctx {
    pub dev: *mut device,
    pub fbk_tfm: *mut crypto_shash,
}

    static dma_addr_t update_dma_addr, final_dma_addr;
    static char *ubuf, *fbuf;
    static DEFINE_SPINLOCK(zynqmp_sha_lock);
#[no_mangle]
unsafe extern "C" fn zynqmp_sha_init_tfm(hash: *mut crypto_shash) -> c_int {
    static int zynqmp_sha_init_tfm(struct crypto_shash *hash)
    {
    const char *fallback_driver_name = crypto_shash_alg_name(hash);
    struct zynqmp_sha_tfm_ctx *tfm_ctx = crypto_shash_ctx(hash);
    struct shash_alg *alg = crypto_shash_alg(hash);
    struct crypto_shash *fallback_tfm;
    struct zynqmp_sha_drv_ctx *drv_ctx;
    drv_ctx = container_of(alg, struct zynqmp_sha_drv_ctx, sha3_384);
    tfm_ctx.dev = drv_ctx.dev;
// Allocate a fallback and abort if it failed.
    fallback_tfm = crypto_alloc_shash(fallback_driver_name, 0,
    CRYPTO_ALG_NEED_FALLBACK);
    if (IS_ERR(fallback_tfm))
    return PTR_ERR(fallback_tfm);
    if (crypto_shash_descsize(hash) <
    crypto_shash_statesize(tfm_ctx.fbk_tfm)) {
    crypto_free_shash(fallback_tfm);
    return -EINVAL;
    }
    tfm_ctx.fbk_tfm = fallback_tfm;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zynqmp_sha_exit_tfm(hash: *mut crypto_shash) {
    static void zynqmp_sha_exit_tfm(struct crypto_shash *hash)
    {
    struct zynqmp_sha_tfm_ctx *tfm_ctx = crypto_shash_ctx(hash);
    crypto_free_shash(tfm_ctx.fbk_tfm);
    }
    static int zynqmp_sha_continue(struct shash_desc *desc,
    struct shash_desc *fbdesc, int err)
    {
    err = err ?: crypto_shash_export(fbdesc, shash_desc_ctx(desc));
    shash_desc_zero(fbdesc);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn zynqmp_sha_init(desc: *mut shash_desc) -> c_int {
    static int zynqmp_sha_init(struct shash_desc *desc)
    {
    struct zynqmp_sha_tfm_ctx *tctx = crypto_shash_ctx(desc.tfm);
    struct crypto_shash *fbtfm = tctx.fbk_tfm;
    SHASH_DESC_ON_STACK(fbdesc, fbtfm);
    int err;
    fbdesc.tfm = fbtfm;
    err = crypto_shash_init(fbdesc);
    return zynqmp_sha_continue(desc, fbdesc, err);
    }
#[no_mangle]
unsafe extern "C" fn zynqmp_sha_update(desc: *mut shash_desc, data: *const u8, length: c_uint) -> c_int {
    static int zynqmp_sha_update(struct shash_desc *desc, const u8 *data, unsigned int length)
    {
    struct zynqmp_sha_tfm_ctx *tctx = crypto_shash_ctx(desc.tfm);
    struct crypto_shash *fbtfm = tctx.fbk_tfm;
    SHASH_DESC_ON_STACK(fbdesc, fbtfm);
    int err;
    fbdesc.tfm = fbtfm;
    err = crypto_shash_import(fbdesc, shash_desc_ctx(desc)) ?:
    crypto_shash_update(fbdesc, data, length);
    return zynqmp_sha_continue(desc, fbdesc, err);
    }
#[no_mangle]
unsafe extern "C" fn zynqmp_sha_finup(desc: *mut shash_desc, data: *const u8, length: c_uint, out: *mut u8) -> c_int {
    static int zynqmp_sha_finup(struct shash_desc *desc, const u8 *data, unsigned int length, u8 *out)
    {
    struct zynqmp_sha_tfm_ctx *tctx = crypto_shash_ctx(desc.tfm);
    struct crypto_shash *fbtfm = tctx.fbk_tfm;
    SHASH_DESC_ON_STACK(fbdesc, fbtfm);
    fbdesc.tfm = fbtfm;
    return crypto_shash_import(fbdesc, shash_desc_ctx(desc)) ?:
    crypto_shash_finup(fbdesc, data, length, out);
    }
    static int __zynqmp_sha_digest(struct shash_desc *desc, const u8 *data,
    unsigned int len, u8 *out)
    {
    let mut remaining_len: c_uint = len;
    int update_size;
    int ret;
    ret = zynqmp_pm_sha_hash(0, 0, ZYNQMP_SHA3_INIT);
    if (ret)
    return ret;
    while (remaining_len != 0) {
    memzero_explicit(ubuf, ZYNQMP_DMA_ALLOC_FIXED_SIZE);
    if (remaining_len >= ZYNQMP_DMA_ALLOC_FIXED_SIZE) {
    update_size = ZYNQMP_DMA_ALLOC_FIXED_SIZE;
    remaining_len -= ZYNQMP_DMA_ALLOC_FIXED_SIZE;
    } else {
    update_size = remaining_len;
    remaining_len = 0;
    }
    memcpy(ubuf, data, update_size);
    flush_icache_range((unsigned long)ubuf, (unsigned long)ubuf + update_size);
    ret = zynqmp_pm_sha_hash(update_dma_addr, update_size, ZYNQMP_SHA3_UPDATE);
    if (ret)
    return ret;
    data += update_size;
    }
    ret = zynqmp_pm_sha_hash(final_dma_addr, SHA3_384_DIGEST_SIZE, ZYNQMP_SHA3_FINAL);
    memcpy(out, fbuf, SHA3_384_DIGEST_SIZE);
    memzero_explicit(fbuf, SHA3_384_DIGEST_SIZE);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn zynqmp_sha_digest(desc: *mut shash_desc, data: *const u8, len: c_uint, out: *mut u8) -> c_int {
    static int zynqmp_sha_digest(struct shash_desc *desc, const u8 *data, unsigned int len, u8 *out)
    {
    scoped_guard(spinlock_bh, &zynqmp_sha_lock)
    return __zynqmp_sha_digest(desc, data, len, out);
    }
    static struct zynqmp_sha_drv_ctx sha3_drv_ctx = {
    .sha3_384 = {
    .init = zynqmp_sha_init,
    .update = zynqmp_sha_update,
    .finup = zynqmp_sha_finup,
    .digest = zynqmp_sha_digest,
    .init_tfm = zynqmp_sha_init_tfm,
    .exit_tfm = zynqmp_sha_exit_tfm,
    .descsize = SHA3_384_EXPORT_SIZE,
    .digestsize = SHA3_384_DIGEST_SIZE,
    .base = {
    .cra_name = "sha3-384",
    .cra_driver_name = "zynqmp-sha3-384",
    .cra_priority = 300,
    .cra_flags = CRYPTO_ALG_KERN_DRIVER_ONLY |
    CRYPTO_ALG_NEED_FALLBACK,
    .cra_blocksize = SHA3_384_BLOCK_SIZE,
    .cra_ctxsize = sizeof(struct zynqmp_sha_tfm_ctx),
    .cra_module = THIS_MODULE,
    }
    }
    };
#[no_mangle]
unsafe extern "C" fn zynqmp_sha_probe(pdev: *mut platform_device) -> c_int {
    static int zynqmp_sha_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    int err;
    u32 v;
// Verify the hardware is present
    err = zynqmp_pm_get_api_version(&v);
    if (err)
    return err;
    err = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(ZYNQMP_DMA_BIT_MASK));
    if (err < 0) {
    dev_err(dev, "No usable DMA configuration\n");
    return err;
    }
    err = crypto_register_shash(&sha3_drv_ctx.sha3_384);
    if (err < 0) {
    dev_err(dev, "Failed to register shash alg.\n");
    return err;
    }
    sha3_drv_ctx.dev = dev;
    platform_set_drvdata(pdev, &sha3_drv_ctx);
    ubuf = dma_alloc_coherent(dev, ZYNQMP_DMA_ALLOC_FIXED_SIZE, &update_dma_addr, GFP_KERNEL);
    if (!ubuf) {
    err = -ENOMEM;
    goto err_shash;
    }
    fbuf = dma_alloc_coherent(dev, SHA3_384_DIGEST_SIZE, &final_dma_addr, GFP_KERNEL);
    if (!fbuf) {
    err = -ENOMEM;
    goto err_mem;
    }
    return 0;
    err_mem:
    dma_free_coherent(sha3_drv_ctx.dev, ZYNQMP_DMA_ALLOC_FIXED_SIZE, ubuf, update_dma_addr);
    err_shash:
    crypto_unregister_shash(&sha3_drv_ctx.sha3_384);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn zynqmp_sha_remove(pdev: *mut platform_device) {
    static void zynqmp_sha_remove(struct platform_device *pdev)
    {
    sha3_drv_ctx.dev = platform_get_drvdata(pdev);
    dma_free_coherent(sha3_drv_ctx.dev, ZYNQMP_DMA_ALLOC_FIXED_SIZE, ubuf, update_dma_addr);
    dma_free_coherent(sha3_drv_ctx.dev, SHA3_384_DIGEST_SIZE, fbuf, final_dma_addr);
    crypto_unregister_shash(&sha3_drv_ctx.sha3_384);
    }
    static struct platform_driver zynqmp_sha_driver = {
    .probe = zynqmp_sha_probe,
    .remove = zynqmp_sha_remove,
    .driver = {
    .name = "zynqmp-sha3-384",
    },
    };
    module_platform_driver(zynqmp_sha_driver);
    MODULE_DESCRIPTION("ZynqMP SHA3 hardware acceleration support.");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Harsha <harsha.harsha@xilinx.com>");
