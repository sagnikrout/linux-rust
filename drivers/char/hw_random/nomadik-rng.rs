//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/nomadik-rng.c
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
//
// Nomadik RNG support
// Copyright 2009 Alessandro Rubini
//

#[no_mangle]
unsafe extern "C" fn nmk_rng_read(rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int nmk_rng_read(struct hwrng *rng, void *data, size_t max, bool wait)
    {
    void __iomem *base = (void __iomem *)rng.priv;
//
// The register is 32 bits and gives 16 random bits (low half).
// A subsequent read will delay the core for 400ns, so we just read
// once and accept the very unlikely very small delay, even if wait==0.
//
// (u16 *)data = __raw_readl(base + 8) & 0xffff;
    return 2;
    }
// we have at most one RNG per machine, granted
    static struct hwrng nmk_rng = {
    .name		= "nomadik",
    .read		= nmk_rng_read,
    };
#[no_mangle]
unsafe extern "C" fn nmk_rng_probe(dev: *mut amba_device, id: *const amba_id) -> c_int {
    static int nmk_rng_probe(struct amba_device *dev, const struct amba_id *id)
    {
    struct clk *rng_clk;
    void __iomem *base;
    int ret;
    rng_clk = devm_clk_get_enabled(&dev.dev, core::ptr::null_mut());
    if (IS_ERR(rng_clk))
    return dev_err_probe(&dev.dev, PTR_ERR(rng_clk), "could not get rng clock\n");
    ret = amba_request_regions(dev, dev.dev.init_name);
    if (ret)
    return ret;
    ret = -ENOMEM;
    base = devm_ioremap(&dev.dev, dev.res.start,
    resource_size(&dev.res));
    if (!base)
    goto out_release;
    nmk_rng.priv = (unsigned long)base;
    ret = devm_hwrng_register(&dev.dev, &nmk_rng);
    if (ret)
    goto out_release;
    return 0;
    out_release:
    amba_release_regions(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nmk_rng_remove(dev: *mut amba_device) {
    static void nmk_rng_remove(struct amba_device *dev)
    {
    amba_release_regions(dev);
    }
    static const struct amba_id nmk_rng_ids[] = {
    {
    .id	= 0x000805e1,
    .mask	= 0x000fffff, /* top bits are rev and cfg: accept all */
    },
    {0, 0},
    };
    MODULE_DEVICE_TABLE(amba, nmk_rng_ids);
    static struct amba_driver nmk_rng_driver = {
    .drv = {
    .name = "rng",
    },
    .probe = nmk_rng_probe,
    .remove = nmk_rng_remove,
    .id_table = nmk_rng_ids,
    };
    module_amba_driver(nmk_rng_driver);
    MODULE_DESCRIPTION("ST-Ericsson Nomadik Random Number Generator");
    MODULE_LICENSE("GPL");
