//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/ks-sa-rng.c
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
// Random Number Generator driver for the Keystone SOC
//
// Copyright (C) 2016 Texas Instruments Incorporated - https://www.ti.com
//
// Authors:	Sandeep Nair
// Vitaly Andrianov
//

pub const SA_CMD_STATUS_OFS: c_uint = 0x8;
// TRNG enable control in SA System module

// TRNG start control in TRNG module

// Data ready indicator in STATUS register

// Data ready clear control in INTACK register

//
// Number of samples taken to gather entropy during startup.
// If value is 0, the number of samples is 2^24 else
// equals value times 2^8.
//
pub const TRNG_DEF_STARTUP_CYCLES: c_int = 0;
pub const TRNG_CNTL_REG_STARTUP_CYCLES_SHIFT: c_int = 16;
//
// Minimum number of samples taken to regenerate entropy
// If value is 0, the number of samples is 2^24 else
// equals value times 2^6.
//
pub const TRNG_DEF_MIN_REFILL_CYCLES: c_int = 1;
pub const TRNG_CFG_REG_MIN_REFILL_CYCLES_SHIFT: c_int = 0;
//
// Maximum number of samples taken to regenerate entropy
// If value is 0, the number of samples is 2^24 else
// equals value times 2^8.
//
pub const TRNG_DEF_MAX_REFILL_CYCLES: c_int = 0;
pub const TRNG_CFG_REG_MAX_REFILL_CYCLES_SHIFT: c_int = 16;
// Number of CLK input cycles between samples
pub const TRNG_DEF_CLK_DIV_CYCLES: c_int = 0;
pub const TRNG_CFG_REG_SAMPLE_DIV_SHIFT: c_int = 8;
// Maximum retries to get rng data
pub const SA_MAX_RNG_DATA_RETRIES: c_int = 5;
// Delay between retries (in usecs)
pub const SA_RNG_DATA_RETRY_DELAY: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trng_regs {
    pub output_l: u32,
    pub output_h: u32,
    pub status: u32,
    pub intmask: u32,
    pub intack: u32,
    pub control: u32,
    pub config: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ks_sa_rng {
    pub rng: hwrng,
    pub clk: *mut clk,
    pub regmap_cfg: *mut regmap,
    pub reg_rng: *mut trng_regs __iomem,
    pub ready_ts: u64,
    pub refill_delay_ns: c_uint,
}

#[no_mangle]
unsafe extern "C" fn cycles_to_ns(clk_rate: c_ulong, cycles: c_uint) -> c_uint {
    static unsigned int cycles_to_ns(unsigned long clk_rate, unsigned int cycles)
    {
    return DIV_ROUND_UP_ULL((TRNG_DEF_CLK_DIV_CYCLES + 1) * 1000000000ull *
    cycles, clk_rate);
    }
#[no_mangle]
unsafe extern "C" fn startup_delay_ns(clk_rate: c_ulong) -> c_uint {
    static unsigned int startup_delay_ns(unsigned long clk_rate)
    {
    if (!TRNG_DEF_STARTUP_CYCLES)
    return cycles_to_ns(clk_rate, BIT(24));
    return cycles_to_ns(clk_rate, 256 * TRNG_DEF_STARTUP_CYCLES);
    }
#[no_mangle]
unsafe extern "C" fn refill_delay_ns(clk_rate: c_ulong) -> c_uint {
    static unsigned int refill_delay_ns(unsigned long clk_rate)
    {
    if (!TRNG_DEF_MAX_REFILL_CYCLES)
    return cycles_to_ns(clk_rate, BIT(24));
    return cycles_to_ns(clk_rate, 256 * TRNG_DEF_MAX_REFILL_CYCLES);
    }
#[no_mangle]
unsafe extern "C" fn ks_sa_rng_init(rng: *mut hwrng) -> c_int {
    static int ks_sa_rng_init(struct hwrng *rng)
    {
    u32 value;
    struct ks_sa_rng *ks_sa_rng = container_of(rng, struct ks_sa_rng, rng);
    let mut clk_rate: c_ulong = clk_get_rate(ks_sa_rng.clk);
// Enable RNG module
    regmap_write_bits(ks_sa_rng.regmap_cfg, SA_CMD_STATUS_OFS,
    SA_CMD_STATUS_REG_TRNG_ENABLE,
    SA_CMD_STATUS_REG_TRNG_ENABLE);
// Configure RNG module
    writel(0, &ks_sa_rng.reg_rng.control);
    value = TRNG_DEF_STARTUP_CYCLES << TRNG_CNTL_REG_STARTUP_CYCLES_SHIFT;
    writel(value, &ks_sa_rng.reg_rng.control);
    value =	(TRNG_DEF_MIN_REFILL_CYCLES <<
    TRNG_CFG_REG_MIN_REFILL_CYCLES_SHIFT) |
    (TRNG_DEF_MAX_REFILL_CYCLES <<
    TRNG_CFG_REG_MAX_REFILL_CYCLES_SHIFT) |
    (TRNG_DEF_CLK_DIV_CYCLES <<
    TRNG_CFG_REG_SAMPLE_DIV_SHIFT);
    writel(value, &ks_sa_rng.reg_rng.config);
// Disable all interrupts from TRNG
    writel(0, &ks_sa_rng.reg_rng.intmask);
// Enable RNG
    value = readl(&ks_sa_rng.reg_rng.control);
    value |= TRNG_CNTL_REG_TRNG_ENABLE;
    writel(value, &ks_sa_rng.reg_rng.control);
    ks_sa_rng.refill_delay_ns = refill_delay_ns(clk_rate);
    ks_sa_rng.ready_ts = ktime_get_ns() +
    startup_delay_ns(clk_rate);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ks_sa_rng_cleanup(rng: *mut hwrng) {
    static void ks_sa_rng_cleanup(struct hwrng *rng)
    {
    struct ks_sa_rng *ks_sa_rng = container_of(rng, struct ks_sa_rng, rng);
// Disable RNG
    writel(0, &ks_sa_rng.reg_rng.control);
    regmap_write_bits(ks_sa_rng.regmap_cfg, SA_CMD_STATUS_OFS,
    SA_CMD_STATUS_REG_TRNG_ENABLE, 0);
    }
#[no_mangle]
unsafe extern "C" fn ks_sa_rng_data_read(rng: *mut hwrng, data: *mut u32) -> c_int {
    static int ks_sa_rng_data_read(struct hwrng *rng, u32 *data)
    {
    struct ks_sa_rng *ks_sa_rng = container_of(rng, struct ks_sa_rng, rng);
// Read random data
    data[0] = readl(&ks_sa_rng.reg_rng.output_l);
    data[1] = readl(&ks_sa_rng.reg_rng.output_h);
    writel(TRNG_INTACK_REG_READY, &ks_sa_rng.reg_rng.intack);
    ks_sa_rng.ready_ts = ktime_get_ns() + ks_sa_rng.refill_delay_ns;
    return sizeof(u32) * 2;
    }
#[no_mangle]
unsafe extern "C" fn ks_sa_rng_data_present(rng: *mut hwrng, wait: c_int) -> c_int {
    static int ks_sa_rng_data_present(struct hwrng *rng, int wait)
    {
    struct ks_sa_rng *ks_sa_rng = container_of(rng, struct ks_sa_rng, rng);
    let mut now: u64 = ktime_get_ns();
    u32	ready;
    int	j;
    if (wait && now < ks_sa_rng.ready_ts) {
// Max delay expected here is 81920000 ns
    unsigned long min_delay =
    DIV_ROUND_UP((u32)(ks_sa_rng.ready_ts - now), 1000);
    usleep_range(min_delay, min_delay + SA_RNG_DATA_RETRY_DELAY);
    }
    for (j = 0; j < SA_MAX_RNG_DATA_RETRIES; j++) {
    ready = readl(&ks_sa_rng.reg_rng.status);
    ready &= TRNG_STATUS_REG_READY;
    if (ready || !wait)
    break;
    udelay(SA_RNG_DATA_RETRY_DELAY);
    }
    return ready;
    }
#[no_mangle]
unsafe extern "C" fn ks_sa_rng_probe(pdev: *mut platform_device) -> c_int {
    static int ks_sa_rng_probe(struct platform_device *pdev)
    {
    struct ks_sa_rng	*ks_sa_rng;
    struct device		*dev = &pdev.dev;
    int			ret;
    ks_sa_rng = devm_kzalloc(dev, sizeof(*ks_sa_rng), GFP_KERNEL);
    if (!ks_sa_rng)
    return -ENOMEM;
    ks_sa_rng.rng = (struct hwrng) {
    .name = "ks_sa_hwrng",
    .init = ks_sa_rng_init,
    .data_read = ks_sa_rng_data_read,
    .data_present = ks_sa_rng_data_present,
    .cleanup = ks_sa_rng_cleanup,
    };
    ks_sa_rng.reg_rng = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ks_sa_rng.reg_rng))
    return PTR_ERR(ks_sa_rng.reg_rng);
    ks_sa_rng.regmap_cfg =
    syscon_regmap_lookup_by_phandle(dev.of_node,
    "ti,syscon-sa-cfg");
    if (IS_ERR(ks_sa_rng.regmap_cfg))
    return dev_err_probe(dev, -EINVAL, "syscon_node_to_regmap failed\n");
    ks_sa_rng.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(ks_sa_rng.clk))
    return dev_err_probe(dev, PTR_ERR(ks_sa_rng.clk), "Failed to get clock\n");
    pm_runtime_enable(dev);
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0) {
    pm_runtime_disable(dev);
    return dev_err_probe(dev, ret, "Failed to enable SA power-domain\n");
    }
    ret = devm_hwrng_register(dev, &ks_sa_rng.rng);
    if (ret) {
    pm_runtime_put_sync(dev);
    pm_runtime_disable(dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ks_sa_rng_remove(pdev: *mut platform_device) {
    static void ks_sa_rng_remove(struct platform_device *pdev)
    {
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
    static const struct of_device_id ks_sa_rng_dt_match[] = {
    {
    .compatible = "ti,keystone-rng",
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, ks_sa_rng_dt_match);
    static struct platform_driver ks_sa_rng_driver = {
    .driver		= {
    .name	= "ks-sa-rng",
    .of_match_table = ks_sa_rng_dt_match,
    },
    .probe		= ks_sa_rng_probe,
    .remove		= ks_sa_rng_remove,
    };
    module_platform_driver(ks_sa_rng_driver);
    MODULE_DESCRIPTION("Keystone NETCP SA H/W Random Number Generator driver");
    MODULE_AUTHOR("Vitaly Andrianov <vitalya@ti.com>");
    MODULE_LICENSE("GPL");
