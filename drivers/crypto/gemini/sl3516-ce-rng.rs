//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/gemini/sl3516-ce-rng.c
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
// sl3516-ce-rng.c - hardware cryptographic offloader for SL3516 SoC.
//
// Copyright (C) 2021 Corentin Labbe <clabbe@baylibre.com>
//
// This file handle the RNG found in the SL3516 crypto engine
//

#[no_mangle]
unsafe extern "C" fn sl3516_ce_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int sl3516_ce_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct sl3516_ce_dev *ce;
    u32 *data = buf;
    let mut read: usize = 0;
    int err;
    ce = container_of(rng, struct sl3516_ce_dev, trng);

    ce.hwrng_stat_req++;
    ce.hwrng_stat_bytes += max;

    err = pm_runtime_get_sync(ce.dev);
    if (err < 0) {
    pm_runtime_put_noidle(ce.dev);
    return err;
    }
    while (read < max) {
// data = readl(ce->base + IPSEC_RAND_NUM_REG);
    data++;
    read += 4;
    }
    pm_runtime_put(ce.dev);
    return read;
    }
#[no_mangle]
pub unsafe extern "C" fn sl3516_ce_rng_register(ce: *mut sl3516_ce_dev) -> c_int {
    int sl3516_ce_rng_register(struct sl3516_ce_dev *ce)
    {
    int ret;
    ce.trng.name = "SL3516 Crypto Engine RNG";
    ce.trng.read = sl3516_ce_rng_read;
    ce.trng.quality = 700;
    ret = hwrng_register(&ce.trng);
    if (ret)
    dev_err(ce.dev, "Fail to register the RNG\n");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sl3516_ce_rng_unregister(ce: *mut sl3516_ce_dev) {
    void sl3516_ce_rng_unregister(struct sl3516_ce_dev *ce)
    {
    hwrng_unregister(&ce.trng);
    }
