//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-mdio.c
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

// Clause-45 mask includes the device type (5 bit) and actual register number (16 bit)

#[no_mangle]
unsafe extern "C" fn regmap_mdio_c22_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_mdio_c22_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct mdio_device *mdio_dev = context;
    int ret;
    if (unlikely(reg & ~REGNUM_C22_MASK))
    return -ENXIO;
    ret = mdiodev_read(mdio_dev, reg);
    if (ret < 0)
    return ret;
// val = ret & REGVAL_MASK;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_mdio_c22_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_mdio_c22_write(void *context, unsigned int reg, unsigned int val)
    {
    struct mdio_device *mdio_dev = context;
    if (unlikely(reg & ~REGNUM_C22_MASK))
    return -ENXIO;
    return mdiodev_write(mdio_dev, reg, val);
    }
    static const struct regmap_bus regmap_mdio_c22_bus = {
    .reg_write = regmap_mdio_c22_write,
    .reg_read = regmap_mdio_c22_read,
    };
#[no_mangle]
unsafe extern "C" fn regmap_mdio_c45_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_mdio_c45_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct mdio_device *mdio_dev = context;
    unsigned int devad;
    int ret;
    if (unlikely(reg & ~REGNUM_C45_MASK))
    return -ENXIO;
    devad = reg >> REGMAP_MDIO_C45_DEVAD_SHIFT;
    reg = reg & REGMAP_MDIO_C45_REGNUM_MASK;
    ret = mdiodev_c45_read(mdio_dev, devad, reg);
    if (ret < 0)
    return ret;
// val = ret & REGVAL_MASK;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_mdio_c45_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_mdio_c45_write(void *context, unsigned int reg, unsigned int val)
    {
    struct mdio_device *mdio_dev = context;
    unsigned int devad;
    if (unlikely(reg & ~REGNUM_C45_MASK))
    return -ENXIO;
    devad = reg >> REGMAP_MDIO_C45_DEVAD_SHIFT;
    reg = reg & REGMAP_MDIO_C45_REGNUM_MASK;
    return mdiodev_c45_write(mdio_dev, devad, reg, val);
    }
    static const struct regmap_bus regmap_mdio_c45_bus = {
    .reg_write = regmap_mdio_c45_write,
    .reg_read = regmap_mdio_c45_read,
    };
    struct regmap *__regmap_init_mdio(struct mdio_device *mdio_dev,
    const struct regmap_config *config, struct lock_class_key *lock_key,
    const char *lock_name)
    {
    const struct regmap_bus *bus;
    if (config.reg_bits == 5 && config.val_bits == 16)
    bus = &regmap_mdio_c22_bus;
#[no_mangle]
pub unsafe extern "C" fn if(16: config->reg_bits == 21 && config->val_bits ==) -> else {
    else if (config.reg_bits == 21 && config.val_bits == 16)
    bus = &regmap_mdio_c45_bus;
    else
    return ERR_PTR(-EOPNOTSUPP);
    return __regmap_init(&mdio_dev.dev, bus, mdio_dev, config, lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_mdio);
    struct regmap *__devm_regmap_init_mdio(struct mdio_device *mdio_dev,
    const struct regmap_config *config, struct lock_class_key *lock_key,
    const char *lock_name)
    {
    const struct regmap_bus *bus;
    if (config.reg_bits == 5 && config.val_bits == 16)
    bus = &regmap_mdio_c22_bus;
#[no_mangle]
pub unsafe extern "C" fn if(16: config->reg_bits == 21 && config->val_bits ==) -> else {
    else if (config.reg_bits == 21 && config.val_bits == 16)
    bus = &regmap_mdio_c45_bus;
    else
    return ERR_PTR(-EOPNOTSUPP);
    return __devm_regmap_init(&mdio_dev.dev, bus, mdio_dev, config, lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_mdio);
    MODULE_AUTHOR("Sander Vanheule <sander@svanheule.net>");
    MODULE_DESCRIPTION("regmap MDIO Module");
    MODULE_LICENSE("GPL v2");
