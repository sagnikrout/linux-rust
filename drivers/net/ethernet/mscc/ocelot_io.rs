//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/mscc/ocelot_io.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Microsemi Ocelot Switch driver
//
// Copyright (c) 2017 Microsemi Corporation
//

    int __ocelot_bulk_read_ix(struct ocelot *ocelot, enum ocelot_reg reg,
    u32 offset, void *buf, int count)
    {
    enum ocelot_target target;
    u32 addr;
    ocelot_reg_to_target_addr(ocelot, reg, &target, &addr);
    WARN_ON(!target);
    return regmap_bulk_read(ocelot.targets[target], addr + offset,
    buf, count);
    }
    EXPORT_SYMBOL_GPL(__ocelot_bulk_read_ix);
#[no_mangle]
pub unsafe extern "C" fn __ocelot_read_ix(ocelot: *mut ocelot, reg: enum ocelot_reg, offset: u32) -> u32 {
    u32 __ocelot_read_ix(struct ocelot *ocelot, enum ocelot_reg reg, u32 offset)
    {
    enum ocelot_target target;
    u32 addr, val;
    ocelot_reg_to_target_addr(ocelot, reg, &target, &addr);
    WARN_ON(!target);
    regmap_read(ocelot.targets[target], addr + offset, &val);
    return val;
    }
    EXPORT_SYMBOL_GPL(__ocelot_read_ix);
    void __ocelot_write_ix(struct ocelot *ocelot, u32 val, enum ocelot_reg reg,
    u32 offset)
    {
    enum ocelot_target target;
    u32 addr;
    ocelot_reg_to_target_addr(ocelot, reg, &target, &addr);
    WARN_ON(!target);
    regmap_write(ocelot.targets[target], addr + offset, val);
    }
    EXPORT_SYMBOL_GPL(__ocelot_write_ix);
    void __ocelot_rmw_ix(struct ocelot *ocelot, u32 val, u32 mask,
    enum ocelot_reg reg, u32 offset)
    {
    enum ocelot_target target;
    u32 addr;
    ocelot_reg_to_target_addr(ocelot, reg, &target, &addr);
    WARN_ON(!target);
    regmap_update_bits(ocelot.targets[target], addr + offset, mask, val);
    }
    EXPORT_SYMBOL_GPL(__ocelot_rmw_ix);
#[no_mangle]
pub unsafe extern "C" fn ocelot_port_readl(port: *mut ocelot_port, reg: enum ocelot_reg) -> u32 {
    u32 ocelot_port_readl(struct ocelot_port *port, enum ocelot_reg reg)
    {
    struct ocelot *ocelot = port.ocelot;
    let mut target: u16 = reg >> TARGET_OFFSET;
    u32 val;
    WARN_ON(!target);
    regmap_read(port.target, ocelot.map[target][reg & REG_MASK], &val);
    return val;
    }
    EXPORT_SYMBOL_GPL(ocelot_port_readl);
#[no_mangle]
pub unsafe extern "C" fn ocelot_port_writel(port: *mut ocelot_port, val: u32, reg: enum ocelot_reg) {
    void ocelot_port_writel(struct ocelot_port *port, u32 val, enum ocelot_reg reg)
    {
    struct ocelot *ocelot = port.ocelot;
    let mut target: u16 = reg >> TARGET_OFFSET;
    WARN_ON(!target);
    regmap_write(port.target, ocelot.map[target][reg & REG_MASK], val);
    }
    EXPORT_SYMBOL_GPL(ocelot_port_writel);
    void ocelot_port_rmwl(struct ocelot_port *port, u32 val, u32 mask,
    enum ocelot_reg reg)
    {
    let mut cur: u32 = ocelot_port_readl(port, reg);
    ocelot_port_writel(port, (cur & (~mask)) | val, reg);
    }
    EXPORT_SYMBOL_GPL(ocelot_port_rmwl);
    u32 __ocelot_target_read_ix(struct ocelot *ocelot, enum ocelot_target target,
    u32 reg, u32 offset)
    {
    u32 val;
    regmap_read(ocelot.targets[target],
    ocelot.map[target][reg] + offset, &val);
    return val;
    }
    void __ocelot_target_write_ix(struct ocelot *ocelot, enum ocelot_target target,
    u32 val, u32 reg, u32 offset)
    {
    regmap_write(ocelot.targets[target],
    ocelot.map[target][reg] + offset, val);
    }
    int ocelot_regfields_init(struct ocelot *ocelot,
    const struct reg_field *const regfields)
    {
    unsigned int i;
    u16 target;
    for (i = 0; i < REGFIELD_MAX; i++) {
    let mut regfield: reg_field = {};
    let mut reg: u32 = regfields[i].reg;
    if (!reg)
    continue;
    target = regfields[i].reg >> TARGET_OFFSET;
    regfield.reg = ocelot.map[target][reg & REG_MASK];
    regfield.lsb = regfields[i].lsb;
    regfield.msb = regfields[i].msb;
    regfield.id_size = regfields[i].id_size;
    regfield.id_offset = regfields[i].id_offset;
    ocelot.regfields[i] =
    devm_regmap_field_alloc(ocelot.dev,
    ocelot.targets[target],
    regfield);
    if (IS_ERR(ocelot.regfields[i]))
    return PTR_ERR(ocelot.regfields[i]);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(ocelot_regfields_init);
    static struct regmap_config ocelot_regmap_config = {
    .reg_bits	= 32,
    .val_bits	= 32,
    .reg_stride	= 4,
    };
    struct regmap *ocelot_regmap_init(struct ocelot *ocelot, struct resource *res)
    {
    void __iomem *regs;
    regs = devm_ioremap_resource(ocelot.dev, res);
    if (IS_ERR(regs))
    return ERR_CAST(regs);
    ocelot_regmap_config.name = res.name;
    return devm_regmap_init_mmio(ocelot.dev, regs, &ocelot_regmap_config);
    }
    EXPORT_SYMBOL_GPL(ocelot_regmap_init);
