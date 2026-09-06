//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/ba431-rng.c
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
// Copyright (c) 2020 Silex Insight

pub const BA431_REG_CTRL: c_uint = 0x00;
pub const BA431_REG_FIFO_LEVEL: c_uint = 0x04;
pub const BA431_REG_STATUS: c_uint = 0x30;
pub const BA431_REG_FIFODATA: c_uint = 0x80;

pub const BA431_STATUS_STATE_OFFSET: c_int = 1;
    enum ba431_state {
    BA431_STATE_RESET,
    BA431_STATE_STARTUP,
    BA431_STATE_FIFOFULLON,
    BA431_STATE_FIFOFULLOFF,
    BA431_STATE_RUNNING,
    BA431_STATE_ERROR
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ba431_trng {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub rng: hwrng,
    pub reset_pending: core::sync::atomic::AtomicI32,
    pub reset_work: work_struct,
}

#[no_mangle]
pub unsafe extern "C" fn ba431_trng_read_reg(ba431: *mut ba431_trng, reg: u32) -> u32 {
    static inline u32 ba431_trng_read_reg(struct ba431_trng *ba431, u32 reg)
    {
    return ioread32(ba431.base + reg);
    }
    static inline void ba431_trng_write_reg(struct ba431_trng *ba431, u32 reg,
    u32 val)
    {
    iowrite32(val, ba431.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn ba431_trng_get_state(ba431: *mut ba431_trng) -> enum ba431_state {
    static inline enum ba431_state ba431_trng_get_state(struct ba431_trng *ba431)
    {
    let mut status: u32 = ba431_trng_read_reg(ba431, BA431_REG_STATUS);
    return (status & BA431_STATUS_STATE_MASK) >> BA431_STATUS_STATE_OFFSET;
    }
#[no_mangle]
unsafe extern "C" fn ba431_trng_is_in_error(ba431: *mut ba431_trng) -> c_int {
    static int ba431_trng_is_in_error(struct ba431_trng *ba431)
    {
    let mut state: enum ba431_state = ba431_trng_get_state(ba431);
    if ((state < BA431_STATE_STARTUP) ||
    (state >= BA431_STATE_ERROR))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ba431_trng_reset(ba431: *mut ba431_trng) -> c_int {
    static int ba431_trng_reset(struct ba431_trng *ba431)
    {
    int ret;
// Disable interrupts, random generation and enable the softreset
    ba431_trng_write_reg(ba431, BA431_REG_CTRL, BA431_CTRL_SOFTRESET);
    udelay(BA431_RESET_DELAY);
    ba431_trng_write_reg(ba431, BA431_REG_CTRL, BA431_CTRL_ENABLE);
// Wait until the state changed
    if (readx_poll_timeout(ba431_trng_is_in_error, ba431, ret, !ret,
    BA431_RESET_READ_STATUS_INTERVAL,
    BA431_RESET_READ_STATUS_TIMEOUT)) {
    dev_err(ba431.dev, "reset failed (state: %d)\n",
    ba431_trng_get_state(ba431));
    return -ETIMEDOUT;
    }
    dev_info(ba431.dev, "reset done\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ba431_trng_reset_work(work: *mut work_struct) {
    static void ba431_trng_reset_work(struct work_struct *work)
    {
    struct ba431_trng *ba431 = container_of(work, struct ba431_trng,
    reset_work);
    ba431_trng_reset(ba431);
    atomic_set(&ba431.reset_pending, 0);
    }
#[no_mangle]
unsafe extern "C" fn ba431_trng_schedule_reset(ba431: *mut ba431_trng) {
    static void ba431_trng_schedule_reset(struct ba431_trng *ba431)
    {
    if (atomic_cmpxchg(&ba431.reset_pending, 0, 1))
    return;
    schedule_work(&ba431.reset_work);
    }
#[no_mangle]
unsafe extern "C" fn ba431_trng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int ba431_trng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct ba431_trng *ba431 = container_of(rng, struct ba431_trng, rng);
    u32 *data = buf;
    unsigned int level, i;
    let mut n: c_int = 0;
    while (max > 0) {
    level = ba431_trng_read_reg(ba431, BA431_REG_FIFO_LEVEL);
    if (!level) {
    if (ba431_trng_is_in_error(ba431)) {
    ba431_trng_schedule_reset(ba431);
    break;
    }
    if (!wait)
    break;
    udelay(BA431_READ_RETRY_INTERVAL);
    continue;
    }
    i = level;
    do {
    data[n++] = ba431_trng_read_reg(ba431,
    BA431_REG_FIFODATA);
    max -= sizeof(*data);
    } while (--i && (max > 0));
    if (ba431_trng_is_in_error(ba431)) {
    n -= (level - i);
    ba431_trng_schedule_reset(ba431);
    break;
    }
    }
    n *= sizeof(data);
    return (n || !wait) ? n : -EIO;
    }
#[no_mangle]
unsafe extern "C" fn ba431_trng_cleanup(rng: *mut hwrng) {
    static void ba431_trng_cleanup(struct hwrng *rng)
    {
    struct ba431_trng *ba431 = container_of(rng, struct ba431_trng, rng);
    ba431_trng_write_reg(ba431, BA431_REG_CTRL, 0);
    cancel_work_sync(&ba431.reset_work);
    }
#[no_mangle]
unsafe extern "C" fn ba431_trng_init(rng: *mut hwrng) -> c_int {
    static int ba431_trng_init(struct hwrng *rng)
    {
    struct ba431_trng *ba431 = container_of(rng, struct ba431_trng, rng);
    return ba431_trng_reset(ba431);
    }
#[no_mangle]
unsafe extern "C" fn ba431_trng_probe(pdev: *mut platform_device) -> c_int {
    static int ba431_trng_probe(struct platform_device *pdev)
    {
    struct ba431_trng *ba431;
    int ret;
    ba431 = devm_kzalloc(&pdev.dev, sizeof(*ba431), GFP_KERNEL);
    if (!ba431)
    return -ENOMEM;
    ba431.dev = &pdev.dev;
    ba431.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ba431.base))
    return PTR_ERR(ba431.base);
    atomic_set(&ba431.reset_pending, 0);
    INIT_WORK(&ba431.reset_work, ba431_trng_reset_work);
    ba431.rng.name = pdev.name;
    ba431.rng.init = ba431_trng_init;
    ba431.rng.cleanup = ba431_trng_cleanup;
    ba431.rng.read = ba431_trng_read;
    ret = devm_hwrng_register(&pdev.dev, &ba431.rng);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "BA431 registration failed\n");
    dev_info(&pdev.dev, "BA431 TRNG registered\n");
    return 0;
    }
    static const struct of_device_id ba431_trng_dt_ids[] = {
    { .compatible = "silex-insight,ba431-rng" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ba431_trng_dt_ids);
    static struct platform_driver ba431_trng_driver = {
    .driver = {
    .name = "ba431-rng",
    .of_match_table = ba431_trng_dt_ids,
    },
    .probe = ba431_trng_probe,
    };
    module_platform_driver(ba431_trng_driver);
    MODULE_AUTHOR("Olivier Sobrie <olivier@sobrie.be>");
    MODULE_DESCRIPTION("TRNG driver for Silex Insight BA431");
    MODULE_LICENSE("GPL");
