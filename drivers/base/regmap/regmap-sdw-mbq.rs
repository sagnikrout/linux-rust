//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-sdw-mbq.c
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
// Copyright(c) 2020 Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_mbq_context {
    pub dev: *mut device,
    pub sdw: *mut sdw_slave,
    pub reg): *mut *mut *mut bool (readable_reg)(struct device dev, unsigned int,
    pub cfg: regmap_sdw_mbq_cfg,
    pub val_size: c_int,
}

#[no_mangle]
unsafe extern "C" fn regmap_sdw_mbq_size(ctx: *mut regmap_mbq_context, reg: c_uint) -> c_int {
    static int regmap_sdw_mbq_size(struct regmap_mbq_context *ctx, unsigned int reg)
    {
    let mut size: c_int = ctx.val_size;
    if (ctx.cfg.mbq_size) {
    size = ctx.cfg.mbq_size(ctx.dev, reg);
    if (!size || size > ctx.val_size)
    return -EINVAL;
    }
    return size;
    }
#[no_mangle]
unsafe extern "C" fn regmap_sdw_mbq_deferrable(ctx: *mut regmap_mbq_context, reg: c_uint) -> bool {
    static bool regmap_sdw_mbq_deferrable(struct regmap_mbq_context *ctx, unsigned int reg)
    {
    if (ctx.cfg.deferrable)
    return ctx.cfg.deferrable(ctx.dev, reg);
    return false;
    }
    static int regmap_sdw_mbq_poll_busy(struct sdw_slave *slave, unsigned int reg,
    struct regmap_mbq_context *ctx)
    {
    struct device *dev = ctx.dev;
    int val, ret = 0;
    dev_dbg(dev, "Deferring transaction for 0x%x\n", reg);
    reg = SDW_SDCA_CTL(SDW_SDCA_CTL_FUNC(reg), 0,
    SDCA_CTL_ENTITY_0_FUNCTION_STATUS, 0);
    if (!ctx.readable_reg || ctx.readable_reg(dev, reg)) {
    ret = read_poll_timeout(sdw_read_no_pm, val,
    val < 0 || !(val & SDCA_CTL_ENTITY_0_FUNCTION_BUSY),
    ctx.cfg.retry_us, ctx.cfg.timeout_us,
    false, slave, reg);
    if (val < 0)
    return val;
    if (ret)
    dev_err(dev, "Function busy timed out 0x%x: %d\n", reg, val);
    } else {
    fsleep(ctx.cfg.timeout_us);
    }
    return ret;
    }
    static int regmap_sdw_mbq_write_impl(struct sdw_slave *slave,
    unsigned int reg, unsigned int val,
    int mbq_size)
    {
    let mut shift: c_int = mbq_size * BITS_PER_BYTE;
    int ret;
    while (--mbq_size > 0) {
    shift -= BITS_PER_BYTE;
    ret = sdw_write_no_pm(slave, SDW_SDCA_MBQ_CTL(reg),
    (val >> shift) & 0xff);
    if (ret < 0)
    return ret;
    }
    return sdw_write_no_pm(slave, reg, val & 0xff);
    }
#[no_mangle]
unsafe extern "C" fn regmap_sdw_mbq_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_sdw_mbq_write(void *context, unsigned int reg, unsigned int val)
    {
    struct regmap_mbq_context *ctx = context;
    struct sdw_slave *slave = ctx.sdw;
    struct device *dev = ctx.dev;
    let mut deferrable: bool = regmap_sdw_mbq_deferrable(ctx, reg);
    let mut mbq_size: c_int = regmap_sdw_mbq_size(ctx, reg);
    int ret;
    if (mbq_size < 0)
    return mbq_size;
//
// Technically the spec does allow a device to set itself to busy for
// internal reasons, but since it doesn't provide any information on
// how to handle timeouts in that case, for now the code will only
// process a single wait/timeout on function busy and a single retry
// of the transaction.
//
    ret = regmap_sdw_mbq_write_impl(slave, reg, val, mbq_size);
    if (ret == -ENODATA) {
    if (!deferrable)
    dev_warn(dev, "Defer on undeferrable control: %x\n", reg);
    ret = regmap_sdw_mbq_poll_busy(slave, reg, ctx);
    if (ret)
    return ret;
    ret = regmap_sdw_mbq_write_impl(slave, reg, val, mbq_size);
    }
    return ret;
    }
    static int regmap_sdw_mbq_read_impl(struct sdw_slave *slave,
    unsigned int reg, unsigned int *val,
    int mbq_size)
    {
    let mut shift: c_int = BITS_PER_BYTE;
    int read;
    read = sdw_read_no_pm(slave, reg);
    if (read < 0)
    return read;
// val = read;
    while (--mbq_size > 0) {
    read = sdw_read_no_pm(slave, SDW_SDCA_MBQ_CTL(reg));
    if (read < 0)
    return read;
// val |= read << shift;
    shift += BITS_PER_BYTE;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_sdw_mbq_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_sdw_mbq_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct regmap_mbq_context *ctx = context;
    struct sdw_slave *slave = ctx.sdw;
    struct device *dev = ctx.dev;
    let mut deferrable: bool = regmap_sdw_mbq_deferrable(ctx, reg);
    let mut mbq_size: c_int = regmap_sdw_mbq_size(ctx, reg);
    int ret;
    if (mbq_size < 0)
    return mbq_size;
//
// Technically the spec does allow a device to set itself to busy for
// internal reasons, but since it doesn't provide any information on
// how to handle timeouts in that case, for now the code will only
// process a single wait/timeout on function busy and a single retry
// of the transaction.
//
    ret = regmap_sdw_mbq_read_impl(slave, reg, val, mbq_size);
    if (ret == -ENODATA) {
    if (!deferrable)
    dev_warn(dev, "Defer on undeferrable control: %x\n", reg);
    ret = regmap_sdw_mbq_poll_busy(slave, reg, ctx);
    if (ret)
    return ret;
    ret = regmap_sdw_mbq_read_impl(slave, reg, val, mbq_size);
    }
    return ret;
    }
    static const struct regmap_bus regmap_sdw_mbq = {
    .reg_read = regmap_sdw_mbq_read,
    .reg_write = regmap_sdw_mbq_write,
    .reg_format_endian_default = REGMAP_ENDIAN_LITTLE,
    .val_format_endian_default = REGMAP_ENDIAN_LITTLE,
    };
#[no_mangle]
unsafe extern "C" fn regmap_sdw_mbq_config_check(config: *const regmap_config) -> c_int {
    static int regmap_sdw_mbq_config_check(const struct regmap_config *config)
    {
    if (config.val_bits > (sizeof(unsigned int) * BITS_PER_BYTE))
    return -ENOTSUPP;
// Registers are 32 bits wide
    if (config.reg_bits != 32)
    return -ENOTSUPP;
    if (config.pad_bits != 0)
    return -ENOTSUPP;
    return 0;
    }
    static struct regmap_mbq_context *
    regmap_sdw_mbq_gen_context(struct device *dev,
    struct sdw_slave *sdw,
    const struct regmap_config *config,
    const struct regmap_sdw_mbq_cfg *mbq_config)
    {
    struct regmap_mbq_context *ctx;
    ctx = devm_kzalloc(dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return ERR_PTR(-ENOMEM);
    ctx.dev = dev;
    ctx.sdw = sdw;
    if (mbq_config)
    ctx.cfg = *mbq_config;
    ctx.val_size = config.val_bits / BITS_PER_BYTE;
    ctx.readable_reg = config.readable_reg;
    return ctx;
    }
    struct regmap *__regmap_init_sdw_mbq(struct device *dev, struct sdw_slave *sdw,
    const struct regmap_config *config,
    const struct regmap_sdw_mbq_cfg *mbq_config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    struct regmap_mbq_context *ctx;
    int ret;
    ret = regmap_sdw_mbq_config_check(config);
    if (ret)
    return ERR_PTR(ret);
    ctx = regmap_sdw_mbq_gen_context(dev, sdw, config, mbq_config);
    if (IS_ERR(ctx))
    return ERR_CAST(ctx);
    return __regmap_init(dev, &regmap_sdw_mbq, ctx,
    config, lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_sdw_mbq);
    struct regmap *__devm_regmap_init_sdw_mbq(struct device *dev, struct sdw_slave *sdw,
    const struct regmap_config *config,
    const struct regmap_sdw_mbq_cfg *mbq_config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    struct regmap_mbq_context *ctx;
    int ret;
    ret = regmap_sdw_mbq_config_check(config);
    if (ret)
    return ERR_PTR(ret);
    ctx = regmap_sdw_mbq_gen_context(dev, sdw, config, mbq_config);
    if (IS_ERR(ctx))
    return ERR_CAST(ctx);
    return __devm_regmap_init(dev, &regmap_sdw_mbq, ctx,
    config, lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_sdw_mbq);
    MODULE_DESCRIPTION("regmap SoundWire MBQ Module");
    MODULE_LICENSE("GPL");
