//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-mmio.c
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
// Register map access API - MMIO support
//
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_mmio_context {
    pub regs: *mut void __iomem,
    pub val_bytes: c_uint,
    pub big_endian: bool,
    pub attached_clk: bool,
    pub clk: *mut clk,
    void (*reg_write)(struct regmap_mmio_context *ctx,
    pub val): unsigned int reg, unsigned int,
    unsigned int (*reg_read)(struct regmap_mmio_context *ctx,
    pub reg): c_uint,
}

#[no_mangle]
unsafe extern "C" fn regmap_mmio_regbits_check(reg_bits: usize) -> c_int {
    static int regmap_mmio_regbits_check(size_t reg_bits)
    {
    switch (reg_bits) {
    case 8:
    case 16:
    case 32:
    return 0;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn regmap_mmio_get_min_stride(val_bits: usize) -> c_int {
    static int regmap_mmio_get_min_stride(size_t val_bits)
    {
    int min_stride;
    switch (val_bits) {
    case 8:
// The core treats 0 as 1
    min_stride = 0;
    break;
    case 16:
    min_stride = 2;
    break;
    case 32:
    min_stride = 4;
    break;
    default:
    return -EINVAL;
    }
    return min_stride;
    }
    static void regmap_mmio_write8(struct regmap_mmio_context *ctx,
    unsigned int reg,
    unsigned int val)
    {
    writeb(val, ctx.regs + reg);
    }
    static void regmap_mmio_write8_relaxed(struct regmap_mmio_context *ctx,
    unsigned int reg,
    unsigned int val)
    {
    writeb_relaxed(val, ctx.regs + reg);
    }
    static void regmap_mmio_iowrite8(struct regmap_mmio_context *ctx,
    unsigned int reg, unsigned int val)
    {
    iowrite8(val, ctx.regs + reg);
    }
    static void regmap_mmio_write16le(struct regmap_mmio_context *ctx,
    unsigned int reg,
    unsigned int val)
    {
    writew(val, ctx.regs + reg);
    }
    static void regmap_mmio_write16le_relaxed(struct regmap_mmio_context *ctx,
    unsigned int reg,
    unsigned int val)
    {
    writew_relaxed(val, ctx.regs + reg);
    }
    static void regmap_mmio_iowrite16le(struct regmap_mmio_context *ctx,
    unsigned int reg, unsigned int val)
    {
    iowrite16(val, ctx.regs + reg);
    }
    static void regmap_mmio_write16be(struct regmap_mmio_context *ctx,
    unsigned int reg,
    unsigned int val)
    {
    writew(swab16(val), ctx.regs + reg);
    }
    static void regmap_mmio_iowrite16be(struct regmap_mmio_context *ctx,
    unsigned int reg, unsigned int val)
    {
    iowrite16be(val, ctx.regs + reg);
    }
    static void regmap_mmio_write32le(struct regmap_mmio_context *ctx,
    unsigned int reg,
    unsigned int val)
    {
    writel(val, ctx.regs + reg);
    }
    static void regmap_mmio_write32le_relaxed(struct regmap_mmio_context *ctx,
    unsigned int reg,
    unsigned int val)
    {
    writel_relaxed(val, ctx.regs + reg);
    }
    static void regmap_mmio_iowrite32le(struct regmap_mmio_context *ctx,
    unsigned int reg, unsigned int val)
    {
    iowrite32(val, ctx.regs + reg);
    }
    static void regmap_mmio_write32be(struct regmap_mmio_context *ctx,
    unsigned int reg,
    unsigned int val)
    {
    writel(swab32(val), ctx.regs + reg);
    }
    static void regmap_mmio_iowrite32be(struct regmap_mmio_context *ctx,
    unsigned int reg, unsigned int val)
    {
    iowrite32be(val, ctx.regs + reg);
    }
#[no_mangle]
unsafe extern "C" fn regmap_mmio_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_mmio_write(void *context, unsigned int reg, unsigned int val)
    {
    struct regmap_mmio_context *ctx = context;
    int ret;
    if (!IS_ERR(ctx.clk)) {
    ret = clk_enable(ctx.clk);
    if (ret < 0)
    return ret;
    }
    ctx.reg_write(ctx, reg, val);
    if (!IS_ERR(ctx.clk))
    clk_disable(ctx.clk);
    return 0;
    }
    static int regmap_mmio_noinc_write(void *context, unsigned int reg,
    const void *val, size_t val_count)
    {
    struct regmap_mmio_context *ctx = context;
    let mut ret: c_int = 0;
    int i;
    if (!IS_ERR(ctx.clk)) {
    ret = clk_enable(ctx.clk);
    if (ret < 0)
    return ret;
    }
//
// There are no native, assembly-optimized write single register
// operations for big endian, so fall back to emulation if this
// is needed. (Single bytes are fine, they are not affected by
// endianness.)
//
    if (ctx.big_endian && (ctx.val_bytes > 1)) {
    switch (ctx.val_bytes) {
    case 2:
    {
    const u16 *valp = (const u16 *)val;
    for (i = 0; i < val_count; i++)
    writew(swab16(valp[i]), ctx.regs + reg);
    goto out_clk;
    }
    case 4:
    {
    const u32 *valp = (const u32 *)val;
    for (i = 0; i < val_count; i++)
    writel(swab32(valp[i]), ctx.regs + reg);
    goto out_clk;
    }
    default:
    ret = -EINVAL;
    goto out_clk;
    }
    }
    switch (ctx.val_bytes) {
    case 1:
    writesb(ctx.regs + reg, (const u8 *)val, val_count);
    break;
    case 2:
    writesw(ctx.regs + reg, (const u16 *)val, val_count);
    break;
    case 4:
    writesl(ctx.regs + reg, (const u32 *)val, val_count);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    out_clk:
    if (!IS_ERR(ctx.clk))
    clk_disable(ctx.clk);
    return ret;
    }
    static unsigned int regmap_mmio_read8(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return readb(ctx.regs + reg);
    }
    static unsigned int regmap_mmio_read8_relaxed(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return readb_relaxed(ctx.regs + reg);
    }
    static unsigned int regmap_mmio_ioread8(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return ioread8(ctx.regs + reg);
    }
    static unsigned int regmap_mmio_read16le(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return readw(ctx.regs + reg);
    }
    static unsigned int regmap_mmio_read16le_relaxed(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return readw_relaxed(ctx.regs + reg);
    }
    static unsigned int regmap_mmio_ioread16le(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return ioread16(ctx.regs + reg);
    }
    static unsigned int regmap_mmio_read16be(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return swab16(readw(ctx.regs + reg));
    }
    static unsigned int regmap_mmio_ioread16be(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return ioread16be(ctx.regs + reg);
    }
    static unsigned int regmap_mmio_read32le(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return readl(ctx.regs + reg);
    }
    static unsigned int regmap_mmio_read32le_relaxed(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return readl_relaxed(ctx.regs + reg);
    }
    static unsigned int regmap_mmio_ioread32le(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return ioread32(ctx.regs + reg);
    }
    static unsigned int regmap_mmio_read32be(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return swab32(readl(ctx.regs + reg));
    }
    static unsigned int regmap_mmio_ioread32be(struct regmap_mmio_context *ctx,
    unsigned int reg)
    {
    return ioread32be(ctx.regs + reg);
    }
#[no_mangle]
unsafe extern "C" fn regmap_mmio_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_mmio_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct regmap_mmio_context *ctx = context;
    int ret;
    if (!IS_ERR(ctx.clk)) {
    ret = clk_enable(ctx.clk);
    if (ret < 0)
    return ret;
    }
// val = ctx->reg_read(ctx, reg);
    if (!IS_ERR(ctx.clk))
    clk_disable(ctx.clk);
    return 0;
    }
    static int regmap_mmio_noinc_read(void *context, unsigned int reg,
    void *val, size_t val_count)
    {
    struct regmap_mmio_context *ctx = context;
    let mut ret: c_int = 0;
    if (!IS_ERR(ctx.clk)) {
    ret = clk_enable(ctx.clk);
    if (ret < 0)
    return ret;
    }
    switch (ctx.val_bytes) {
    case 1:
    readsb(ctx.regs + reg, (u8 *)val, val_count);
    break;
    case 2:
    readsw(ctx.regs + reg, (u16 *)val, val_count);
    break;
    case 4:
    readsl(ctx.regs + reg, (u32 *)val, val_count);
    break;
    default:
    ret = -EINVAL;
    goto out_clk;
    }
//
// There are no native, assembly-optimized write single register
// operations for big endian, so fall back to emulation if this
// is needed. (Single bytes are fine, they are not affected by
// endianness.)
//
    if (ctx.big_endian && (ctx.val_bytes > 1)) {
    switch (ctx.val_bytes) {
    case 2:
    swab16_array(val, val_count);
    break;
    case 4:
    swab32_array(val, val_count);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    }
    out_clk:
    if (!IS_ERR(ctx.clk))
    clk_disable(ctx.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn regmap_mmio_free_context(context: *mut c_void) {
    static void regmap_mmio_free_context(void *context)
    {
    struct regmap_mmio_context *ctx = context;
    if (!IS_ERR(ctx.clk)) {
    clk_unprepare(ctx.clk);
    if (!ctx.attached_clk)
    clk_put(ctx.clk);
    }
    kfree(context);
    }
    static const struct regmap_bus regmap_mmio = {
    .fast_io = true,
    .reg_write = regmap_mmio_write,
    .reg_read = regmap_mmio_read,
    .reg_noinc_write = regmap_mmio_noinc_write,
    .reg_noinc_read = regmap_mmio_noinc_read,
    .free_context = regmap_mmio_free_context,
    .val_format_endian_default = REGMAP_ENDIAN_LITTLE,
    };
    static struct regmap_mmio_context *regmap_mmio_gen_context(struct device *dev,
    const char *clk_id,
    void __iomem *regs,
    const struct regmap_config *config)
    {
    struct regmap_mmio_context *ctx;
    int min_stride;
    int ret;
    ret = regmap_mmio_regbits_check(config.reg_bits);
    if (ret)
    return ERR_PTR(ret);
    if (config.pad_bits)
    return ERR_PTR(-EINVAL);
    min_stride = regmap_mmio_get_min_stride(config.val_bits);
    if (min_stride < 0)
    return ERR_PTR(min_stride);
    if (config.reg_stride && config.reg_stride < min_stride)
    return ERR_PTR(-EINVAL);
    if (config.use_relaxed_mmio && config.io_port)
    return ERR_PTR(-EINVAL);
    ctx = kzalloc_obj(*ctx);
    if (!ctx)
    return ERR_PTR(-ENOMEM);
    ctx.regs = regs;
    ctx.val_bytes = config.val_bits / 8;
    ctx.clk = ERR_PTR(-ENODEV);
    switch (regmap_get_val_endian(dev, &regmap_mmio, config)) {
    case REGMAP_ENDIAN_DEFAULT:
    case REGMAP_ENDIAN_LITTLE:

    case REGMAP_ENDIAN_NATIVE:

    switch (config.val_bits) {
    case 8:
    if (config.io_port) {
    ctx.reg_read = regmap_mmio_ioread8;
    ctx.reg_write = regmap_mmio_iowrite8;
    } else if (config.use_relaxed_mmio) {
    ctx.reg_read = regmap_mmio_read8_relaxed;
    ctx.reg_write = regmap_mmio_write8_relaxed;
    } else {
    ctx.reg_read = regmap_mmio_read8;
    ctx.reg_write = regmap_mmio_write8;
    }
    break;
    case 16:
    if (config.io_port) {
    ctx.reg_read = regmap_mmio_ioread16le;
    ctx.reg_write = regmap_mmio_iowrite16le;
    } else if (config.use_relaxed_mmio) {
    ctx.reg_read = regmap_mmio_read16le_relaxed;
    ctx.reg_write = regmap_mmio_write16le_relaxed;
    } else {
    ctx.reg_read = regmap_mmio_read16le;
    ctx.reg_write = regmap_mmio_write16le;
    }
    break;
    case 32:
    if (config.io_port) {
    ctx.reg_read = regmap_mmio_ioread32le;
    ctx.reg_write = regmap_mmio_iowrite32le;
    } else if (config.use_relaxed_mmio) {
    ctx.reg_read = regmap_mmio_read32le_relaxed;
    ctx.reg_write = regmap_mmio_write32le_relaxed;
    } else {
    ctx.reg_read = regmap_mmio_read32le;
    ctx.reg_write = regmap_mmio_write32le;
    }
    break;
    default:
    ret = -EINVAL;
    goto err_free;
    }
    break;
    case REGMAP_ENDIAN_BIG:

    case REGMAP_ENDIAN_NATIVE:

    ctx.big_endian = true;
    switch (config.val_bits) {
    case 8:
    if (config.io_port) {
    ctx.reg_read = regmap_mmio_ioread8;
    ctx.reg_write = regmap_mmio_iowrite8;
    } else {
    ctx.reg_read = regmap_mmio_read8;
    ctx.reg_write = regmap_mmio_write8;
    }
    break;
    case 16:
    if (config.io_port) {
    ctx.reg_read = regmap_mmio_ioread16be;
    ctx.reg_write = regmap_mmio_iowrite16be;
    } else {
    ctx.reg_read = regmap_mmio_read16be;
    ctx.reg_write = regmap_mmio_write16be;
    }
    break;
    case 32:
    if (config.io_port) {
    ctx.reg_read = regmap_mmio_ioread32be;
    ctx.reg_write = regmap_mmio_iowrite32be;
    } else {
    ctx.reg_read = regmap_mmio_read32be;
    ctx.reg_write = regmap_mmio_write32be;
    }
    break;
    default:
    ret = -EINVAL;
    goto err_free;
    }
    break;
    default:
    ret = -EINVAL;
    goto err_free;
    }
    if (clk_id == core::ptr::null_mut())
    return ctx;
    ctx.clk = clk_get(dev, clk_id);
    if (IS_ERR(ctx.clk)) {
    ret = PTR_ERR(ctx.clk);
    goto err_free;
    }
    ret = clk_prepare(ctx.clk);
    if (ret < 0) {
    clk_put(ctx.clk);
    goto err_free;
    }
    return ctx;
    err_free:
    kfree(ctx);
    return ERR_PTR(ret);
    }
    struct regmap *__regmap_init_mmio_clk(struct device *dev, const char *clk_id,
    void __iomem *regs,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    struct regmap_mmio_context *ctx;
    ctx = regmap_mmio_gen_context(dev, clk_id, regs, config);
    if (IS_ERR(ctx))
    return ERR_CAST(ctx);
    return __regmap_init(dev, &regmap_mmio, ctx, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_mmio_clk);
    struct regmap *__devm_regmap_init_mmio_clk(struct device *dev,
    const char *clk_id,
    void __iomem *regs,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    struct regmap_mmio_context *ctx;
    ctx = regmap_mmio_gen_context(dev, clk_id, regs, config);
    if (IS_ERR(ctx))
    return ERR_CAST(ctx);
    return __devm_regmap_init(dev, &regmap_mmio, ctx, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_mmio_clk);
#[no_mangle]
pub unsafe extern "C" fn regmap_mmio_attach_clk(map: *mut regmap, clk: *mut clk) -> c_int {
    int regmap_mmio_attach_clk(struct regmap *map, struct clk *clk)
    {
    struct regmap_mmio_context *ctx = map.bus_context;
    ctx.clk = clk;
    ctx.attached_clk = true;
    return clk_prepare(ctx.clk);
    }
    EXPORT_SYMBOL_GPL(regmap_mmio_attach_clk);
#[no_mangle]
pub unsafe extern "C" fn regmap_mmio_detach_clk(map: *mut regmap) {
    void regmap_mmio_detach_clk(struct regmap *map)
    {
    struct regmap_mmio_context *ctx = map.bus_context;
    clk_unprepare(ctx.clk);
    ctx.attached_clk = false;
    ctx.clk = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(regmap_mmio_detach_clk);
    MODULE_DESCRIPTION("regmap MMIO Module");
    MODULE_LICENSE("GPL v2");
