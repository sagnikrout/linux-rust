//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/rockchip-rng.c
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
// rockchip-rng.c True Random Number Generator driver for Rockchip SoCs
//
// Copyright (c) 2018, Fuzhou Rockchip Electronics Co., Ltd.
// Copyright (c) 2022, Aurelien Jarno
// Copyright (c) 2025, Collabora Ltd.
// Authors:
// Lin Jinhan <troy.lin@rock-chips.com>
// Aurelien Jarno <aurelien@aurel32.net>
// Nicolas Frattaroli <nicolas.frattaroli@collabora.com>
//

pub const RK_RNG_AUTOSUSPEND_DELAY: c_int = 100;
pub const RK_RNG_MAX_BYTE: c_int = 32;
pub const RK_RNG_POLL_PERIOD_US: c_int = 100;
pub const RK_RNG_POLL_TIMEOUT_US: c_int = 10000;
//
// TRNG collects osc ring output bit every RK_RNG_SAMPLE_CNT time. The value is
// a tradeoff between speed and quality and has been adjusted to get a quality
// of ~900 (~87.5% of FIPS 140-2 successes).
//
pub const RK_RNG_SAMPLE_CNT: c_int = 1000;
// after how many bytes of output TRNGv1 implementations should be reseeded
pub const RK_TRNG_V1_AUTO_RESEED_CNT: c_int = 16000;
// TRNG registers from RK3568 TRM-Part2, section 5.4.1
pub const TRNG_RST_CTL: c_uint = 0x0004;
pub const TRNG_RNG_CTL: c_uint = 0x0400;

pub const TRNG_RNG_SAMPLE_CNT: c_uint = 0x0404;
pub const TRNG_RNG_DOUT: c_uint = 0x0410;
//
// TRNG V1 register definitions
// The TRNG V1 IP is a stand-alone TRNG implementation (not part of a crypto IP)
// and can be found in the Rockchip RK3588 SoC
//
pub const TRNG_V1_CTRL: c_uint = 0x0000;
pub const TRNG_V1_CTRL_NOP: c_uint = 0x00;
pub const TRNG_V1_CTRL_RAND: c_uint = 0x01;
pub const TRNG_V1_CTRL_SEED: c_uint = 0x02;
pub const TRNG_V1_STAT: c_uint = 0x0004;

pub const TRNG_V1_MODE: c_uint = 0x0008;

// Interrupt Enable register; unused because polling is faster
pub const TRNG_V1_IE: c_uint = 0x0010;

pub const TRNG_V1_ISTAT: c_uint = 0x0014;

// RAND0 ~ RAND7
pub const TRNG_V1_RAND0: c_uint = 0x0020;
pub const TRNG_V1_RAND7: c_uint = 0x003C;
// Auto Reseed Register
pub const TRNG_V1_AUTO_RQSTS: c_uint = 0x0060;
pub const TRNG_V1_VERSION: c_uint = 0x00F0;
pub const TRNG_v1_VERSION_CODE: c_uint = 0x46bc;
// end of TRNG_V1 register definitions
//
// RKRNG register definitions
// The RKRNG IP is a stand-alone TRNG implementation (not part of a crypto IP)
// and can be found in the Rockchip RK3576, Rockchip RK3562 and Rockchip RK3528
// SoCs. It can either output true randomness (TRNG) or "deterministic"
// randomness derived from hashing the true entropy (DRNG). This driver
// implementation uses just the true entropy, and leaves stretching the entropy
// up to Linux.
//
pub const RKRNG_CFG: c_uint = 0x0000;
pub const RKRNG_CTRL: c_uint = 0x0010;

pub const RKRNG_STATE: c_uint = 0x0014;

pub const RKRNG_TRNG_DATA0: c_uint = 0x0050;
pub const RKRNG_TRNG_DATA1: c_uint = 0x0054;
pub const RKRNG_TRNG_DATA2: c_uint = 0x0058;
pub const RKRNG_TRNG_DATA3: c_uint = 0x005C;
pub const RKRNG_TRNG_DATA4: c_uint = 0x0060;
pub const RKRNG_TRNG_DATA5: c_uint = 0x0064;
pub const RKRNG_TRNG_DATA6: c_uint = 0x0068;
pub const RKRNG_TRNG_DATA7: c_uint = 0x006C;
pub const RKRNG_READ_LEN: c_int = 32;
// Before removing this assert, give rk3588_rng_read an upper bound of 32
    static_assert(RK_RNG_MAX_BYTE <= (TRNG_V1_RAND7 + 4 - TRNG_V1_RAND0),
    "You raised RK_RNG_MAX_BYTE and broke rk3588-rng, congrats.");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_rng {
    pub rng: hwrng,
    pub base: *mut void __iomem,
    pub clk_num: c_int,
    pub clk_bulks: *mut clk_bulk_data,
    pub soc_data: *const rk_rng_soc_data,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk_rng_soc_data {
    pub rng): *mut *mut int (rk_rng_init)(struct hwrng,
    pub wait): *mut *mut *mut *mut int (rk_rng_read)(struct hwrng rng, void buf, size_t max, bool,
    pub rng): *mut *mut void (rk_rng_cleanup)(struct hwrng,
    pub quality: c_ushort,
    pub reset_optional: bool,
}

// The mask in the upper 16 bits determines the bits that are updated
#[no_mangle]
unsafe extern "C" fn rk_rng_write_ctl(rng: *mut rk_rng, val: u32, mask: u32) {
    static void rk_rng_write_ctl(struct rk_rng *rng, u32 val, u32 mask)
    {
    writel((mask << 16) | val, rng.base + TRNG_RNG_CTL);
    }
#[no_mangle]
pub unsafe extern "C" fn rk_rng_writel(rng: *mut rk_rng, val: u32, offset: u32) {
    static inline void rk_rng_writel(struct rk_rng *rng, u32 val, u32 offset)
    {
    writel(val, rng.base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn rk_rng_readl(rng: *mut rk_rng, offset: u32) -> u32 {
    static inline u32 rk_rng_readl(struct rk_rng *rng, u32 offset)
    {
    return readl(rng.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn rk_rng_enable_clks(rk_rng: *mut rk_rng) -> c_int {
    static int rk_rng_enable_clks(struct rk_rng *rk_rng)
    {
    int ret;
// start clocks
    ret = clk_bulk_prepare_enable(rk_rng.clk_num, rk_rng.clk_bulks);
    if (ret < 0) {
    dev_err(rk_rng.dev, "Failed to enable clocks: %d\n", ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk3568_rng_init(rng: *mut hwrng) -> c_int {
    static int rk3568_rng_init(struct hwrng *rng)
    {
    struct rk_rng *rk_rng = container_of(rng, struct rk_rng, rng);
    int ret;
    ret = rk_rng_enable_clks(rk_rng);
    if (ret < 0)
    return ret;
// set the sample period
    writel(RK_RNG_SAMPLE_CNT, rk_rng.base + TRNG_RNG_SAMPLE_CNT);
// set osc ring speed and enable it
    rk_rng_write_ctl(rk_rng, TRNG_RNG_CTL_LEN_256_BIT |
    TRNG_RNG_CTL_OSC_RING_SPEED_0 |
    TRNG_RNG_CTL_ENABLE,
    TRNG_RNG_CTL_MASK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk3568_rng_cleanup(rng: *mut hwrng) {
    static void rk3568_rng_cleanup(struct hwrng *rng)
    {
    struct rk_rng *rk_rng = container_of(rng, struct rk_rng, rng);
// stop TRNG
    rk_rng_write_ctl(rk_rng, 0, TRNG_RNG_CTL_MASK);
// stop clocks
    clk_bulk_disable_unprepare(rk_rng.clk_num, rk_rng.clk_bulks);
    }
#[no_mangle]
unsafe extern "C" fn rk3568_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int rk3568_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct rk_rng *rk_rng = container_of(rng, struct rk_rng, rng);
    let mut to_read: usize = min_t(size_t, max, RK_RNG_MAX_BYTE);
    u32 reg;
    let mut ret: c_int = 0;
    ret = pm_runtime_resume_and_get(rk_rng.dev);
    if (ret < 0)
    return ret;
// Start collecting random data
    rk_rng_write_ctl(rk_rng, TRNG_RNG_CTL_START, TRNG_RNG_CTL_START);
    ret = readl_poll_timeout(rk_rng.base + TRNG_RNG_CTL, reg,
    !(reg & TRNG_RNG_CTL_START),
    RK_RNG_POLL_PERIOD_US,
    RK_RNG_POLL_TIMEOUT_US);
    if (ret < 0)
    goto out;
// Read random data stored in the registers
    memcpy_fromio(buf, rk_rng.base + TRNG_RNG_DOUT, to_read);
    out:
    pm_runtime_put_sync_autosuspend(rk_rng.dev);
    return (ret < 0) ? ret : to_read;
    }
#[no_mangle]
unsafe extern "C" fn rk3576_rng_init(rng: *mut hwrng) -> c_int {
    static int rk3576_rng_init(struct hwrng *rng)
    {
    struct rk_rng *rk_rng = container_of(rng, struct rk_rng, rng);
    return rk_rng_enable_clks(rk_rng);
    }
#[no_mangle]
unsafe extern "C" fn rk3576_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int rk3576_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct rk_rng *rk_rng = container_of(rng, struct rk_rng, rng);
    let mut to_read: usize = min_t(size_t, max, RKRNG_READ_LEN);
    let mut ret: c_int = 0;
    u32 val;
    ret = pm_runtime_resume_and_get(rk_rng.dev);
    if (ret < 0)
    return ret;
    rk_rng_writel(rk_rng, RKRNG_CTRL_REQ_TRNG | (RKRNG_CTRL_REQ_TRNG << 16),
    RKRNG_CTRL);
    if (readl_poll_timeout(rk_rng.base + RKRNG_STATE, val,
    (val & RKRNG_STATE_TRNG_RDY), RK_RNG_POLL_PERIOD_US,
    RK_RNG_POLL_TIMEOUT_US)) {
    dev_err(rk_rng.dev, "timed out waiting for data\n");
    ret = -ETIMEDOUT;
    goto out;
    }
    rk_rng_writel(rk_rng, RKRNG_STATE_TRNG_RDY, RKRNG_STATE);
    memcpy_fromio(buf, rk_rng.base + RKRNG_TRNG_DATA0, to_read);
    out:
    pm_runtime_put_sync_autosuspend(rk_rng.dev);
    return (ret < 0) ? ret : to_read;
    }
#[no_mangle]
unsafe extern "C" fn rk3588_rng_init(rng: *mut hwrng) -> c_int {
    static int rk3588_rng_init(struct hwrng *rng)
    {
    struct rk_rng *rk_rng = container_of(rng, struct rk_rng, rng);
    u32 version, status, mask, istat;
    int ret;
    ret = rk_rng_enable_clks(rk_rng);
    if (ret < 0)
    return ret;
    version = rk_rng_readl(rk_rng, TRNG_V1_VERSION);
    if (version != TRNG_v1_VERSION_CODE) {
    dev_err(rk_rng.dev,
    "wrong trng version, expected = %08x, actual = %08x\n",
    TRNG_V1_VERSION, version);
    ret = -EFAULT;
    goto err_disable_clk;
    }
    mask = TRNG_V1_STAT_SEEDED | TRNG_V1_STAT_GENERATING |
    TRNG_V1_STAT_RESEEDING;
    if (readl_poll_timeout(rk_rng.base + TRNG_V1_STAT, status,
    (status & mask) == TRNG_V1_STAT_SEEDED,
    RK_RNG_POLL_PERIOD_US, RK_RNG_POLL_TIMEOUT_US) < 0) {
    dev_err(rk_rng.dev, "timed out waiting for hwrng to reseed\n");
    ret = -ETIMEDOUT;
    goto err_disable_clk;
    }
//
// clear ISTAT flag, downstream advises to do this to avoid
// auto-reseeding "on power on"
//
    istat = rk_rng_readl(rk_rng, TRNG_V1_ISTAT);
    rk_rng_writel(rk_rng, istat, TRNG_V1_ISTAT);
// auto reseed after RK_TRNG_V1_AUTO_RESEED_CNT bytes
    rk_rng_writel(rk_rng, RK_TRNG_V1_AUTO_RESEED_CNT / 16, TRNG_V1_AUTO_RQSTS);
    return 0;
    err_disable_clk:
    clk_bulk_disable_unprepare(rk_rng.clk_num, rk_rng.clk_bulks);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rk3588_rng_cleanup(rng: *mut hwrng) {
    static void rk3588_rng_cleanup(struct hwrng *rng)
    {
    struct rk_rng *rk_rng = container_of(rng, struct rk_rng, rng);
    clk_bulk_disable_unprepare(rk_rng.clk_num, rk_rng.clk_bulks);
    }
#[no_mangle]
unsafe extern "C" fn rk3588_rng_read(rng: *mut hwrng, buf: *mut c_void, max: usize, wait: bool) -> c_int {
    static int rk3588_rng_read(struct hwrng *rng, void *buf, size_t max, bool wait)
    {
    struct rk_rng *rk_rng = container_of(rng, struct rk_rng, rng);
    let mut to_read: usize = min_t(size_t, max, RK_RNG_MAX_BYTE);
    let mut ret: c_int = 0;
    u32 reg;
    ret = pm_runtime_resume_and_get(rk_rng.dev);
    if (ret < 0)
    return ret;
// Clear ISTAT, even without interrupts enabled, this will be updated
    reg = rk_rng_readl(rk_rng, TRNG_V1_ISTAT);
    rk_rng_writel(rk_rng, reg, TRNG_V1_ISTAT);
// generate 256 bits of random data
    rk_rng_writel(rk_rng, TRNG_V1_MODE_256_BIT, TRNG_V1_MODE);
    rk_rng_writel(rk_rng, TRNG_V1_CTRL_RAND, TRNG_V1_CTRL);
    ret = readl_poll_timeout_atomic(rk_rng.base + TRNG_V1_ISTAT, reg,
    (reg & TRNG_V1_ISTAT_RAND_RDY), 0,
    RK_RNG_POLL_TIMEOUT_US);
    if (ret < 0)
    goto out;
// Read random data that's in registers TRNG_V1_RAND0 through RAND7
    memcpy_fromio(buf, rk_rng.base + TRNG_V1_RAND0, to_read);
    out:
// Clear ISTAT
    rk_rng_writel(rk_rng, reg, TRNG_V1_ISTAT);
// close the TRNG
    rk_rng_writel(rk_rng, TRNG_V1_CTRL_NOP, TRNG_V1_CTRL);
    pm_runtime_put_sync_autosuspend(rk_rng.dev);
    return (ret < 0) ? ret : to_read;
    }
    static const struct rk_rng_soc_data rk3568_soc_data = {
    .rk_rng_init = rk3568_rng_init,
    .rk_rng_read = rk3568_rng_read,
    .rk_rng_cleanup = rk3568_rng_cleanup,
    .quality = 900,
    .reset_optional = false,
    };
    static const struct rk_rng_soc_data rk3576_soc_data = {
    .rk_rng_init = rk3576_rng_init,
    .rk_rng_read = rk3576_rng_read,
    .rk_rng_cleanup = rk3588_rng_cleanup,
    .quality = 999,		/* as determined by actual testing */
    .reset_optional = true,
    };
    static const struct rk_rng_soc_data rk3588_soc_data = {
    .rk_rng_init = rk3588_rng_init,
    .rk_rng_read = rk3588_rng_read,
    .rk_rng_cleanup = rk3588_rng_cleanup,
    .quality = 999,		/* as determined by actual testing */
    .reset_optional = true,
    };
#[no_mangle]
unsafe extern "C" fn rk_rng_probe(pdev: *mut platform_device) -> c_int {
    static int rk_rng_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct reset_control *rst;
    struct rk_rng *rk_rng;
    int ret;
    rk_rng = devm_kzalloc(dev, sizeof(*rk_rng), GFP_KERNEL);
    if (!rk_rng)
    return -ENOMEM;
    rk_rng.soc_data = of_device_get_match_data(dev);
    rk_rng.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(rk_rng.base))
    return PTR_ERR(rk_rng.base);
    rk_rng.clk_num = devm_clk_bulk_get_all(dev, &rk_rng.clk_bulks);
    if (rk_rng.clk_num < 0)
    return dev_err_probe(dev, rk_rng.clk_num,
    "Failed to get clks property\n");
    if (rk_rng.soc_data.reset_optional)
    rst = devm_reset_control_array_get_optional_exclusive(dev);
    else
    rst = devm_reset_control_array_get_exclusive(dev);
    if (rst) {
    if (IS_ERR(rst))
    return dev_err_probe(dev, PTR_ERR(rst), "Failed to get reset property\n");
    reset_control_assert(rst);
    udelay(2);
    reset_control_deassert(rst);
    }
    platform_set_drvdata(pdev, rk_rng);
    rk_rng.rng.name = dev_driver_string(dev);
    if (!IS_ENABLED(CONFIG_PM)) {
    rk_rng.rng.init = rk_rng.soc_data.rk_rng_init;
    rk_rng.rng.cleanup = rk_rng.soc_data.rk_rng_cleanup;
    }
    rk_rng.rng.read = rk_rng.soc_data.rk_rng_read;
    rk_rng.dev = dev;
    rk_rng.rng.quality = rk_rng.soc_data.quality;
    pm_runtime_set_autosuspend_delay(dev, RK_RNG_AUTOSUSPEND_DELAY);
    pm_runtime_use_autosuspend(dev);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return dev_err_probe(dev, ret, "Runtime pm activation failed.\n");
    ret = devm_hwrng_register(dev, &rk_rng.rng);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register Rockchip hwrng\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk_rng_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused rk_rng_runtime_suspend(struct device *dev)
    {
    struct rk_rng *rk_rng = dev_get_drvdata(dev);
    rk_rng.soc_data.rk_rng_cleanup(&rk_rng.rng);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rk_rng_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused rk_rng_runtime_resume(struct device *dev)
    {
    struct rk_rng *rk_rng = dev_get_drvdata(dev);
    return rk_rng.soc_data.rk_rng_init(&rk_rng.rng);
    }
    static const struct dev_pm_ops rk_rng_pm_ops = {
    SET_RUNTIME_PM_OPS(rk_rng_runtime_suspend,
    rk_rng_runtime_resume, core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    };
    static const struct of_device_id rk_rng_dt_match[] = {
    { .compatible = "rockchip,rk3568-rng", .data = (void *)&rk3568_soc_data },
    { .compatible = "rockchip,rk3576-rng", .data = (void *)&rk3576_soc_data },
    { .compatible = "rockchip,rk3588-rng", .data = (void *)&rk3588_soc_data },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, rk_rng_dt_match);
    static struct platform_driver rk_rng_driver = {
    .driver	= {
    .name	= "rockchip-rng",
    .pm	= &rk_rng_pm_ops,
    .of_match_table = rk_rng_dt_match,
    },
    .probe	= rk_rng_probe,
    };
    module_platform_driver(rk_rng_driver);
    MODULE_DESCRIPTION("Rockchip True Random Number Generator driver");
    MODULE_AUTHOR("Lin Jinhan <troy.lin@rock-chips.com>");
    MODULE_AUTHOR("Aurelien Jarno <aurelien@aurel32.net>");
    MODULE_AUTHOR("Daniel Golle <daniel@makrotopia.org>");
    MODULE_AUTHOR("Nicolas Frattaroli <nicolas.frattaroli@collabora.com>");
    MODULE_LICENSE("GPL");
