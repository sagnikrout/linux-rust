//! Automatically rewritten from C to Rust
//! Source: drivers/soc/apple/tunable.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple Silicon hardware tunable support
//
// Each tunable is a list with each entry containing a offset into the MMIO
// region, a mask of bits to be cleared and a set of bits to be set. These
// tunables are passed along by the previous boot stages and vary from device
// to device such that they cannot be hardcoded in the individual drivers.
//
// Copyright (C) The Asahi Linux Contributors
//

    struct apple_tunable *devm_apple_tunable_parse(struct device *dev,
    struct device_node *np,
    const char *name,
    struct resource *res)
    {
    struct apple_tunable *tunable;
    struct property *prop;
    const __be32 *p;
    size_t sz;
    int i;
    if (resource_size(res) < 4)
    return ERR_PTR(-EINVAL);
    prop = of_find_property(np, name, core::ptr::null_mut());
    if (!prop)
    return ERR_PTR(-ENOENT);
    if (prop.length % (3 * sizeof(u32)))
    return ERR_PTR(-EINVAL);
    sz = prop.length / (3 * sizeof(u32));
    tunable = devm_kzalloc(dev, struct_size(tunable, values, sz), GFP_KERNEL);
    if (!tunable)
    return ERR_PTR(-ENOMEM);
    tunable.sz = sz;
    for (i = 0, p = core::ptr::null_mut(); i < tunable.sz; ++i) {
    p = of_prop_next_u32(prop, p, &tunable.values[i].offset);
    p = of_prop_next_u32(prop, p, &tunable.values[i].mask);
    p = of_prop_next_u32(prop, p, &tunable.values[i].value);
// Sanity checks to catch bugs in our bootloader
    if (tunable.values[i].offset % 4)
    return ERR_PTR(-EINVAL);
    if (tunable.values[i].offset > (resource_size(res) - 4))
    return ERR_PTR(-EINVAL);
    }
    return tunable;
    }
    EXPORT_SYMBOL(devm_apple_tunable_parse);
#[no_mangle]
pub unsafe extern "C" fn apple_tunable_apply(regs: *mut void __iomem, tunable: *mut apple_tunable) {
    void apple_tunable_apply(void __iomem *regs, struct apple_tunable *tunable)
    {
    size_t i;
    for (i = 0; i < tunable.sz; ++i) {
    u32 val, old_val;
    old_val = readl(regs + tunable.values[i].offset);
    val = old_val & ~tunable.values[i].mask;
    val |= tunable.values[i].value;
    if (val != old_val)
    writel(val, regs + tunable.values[i].offset);
    }
    }
    EXPORT_SYMBOL(apple_tunable_apply);
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_AUTHOR("Sven Peter <sven@kernel.org>");
    MODULE_DESCRIPTION("Apple Silicon hardware tunable support");
