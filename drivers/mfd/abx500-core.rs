//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/abx500-core.c
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
// Copyright (C) 2007-2010 ST-Ericsson
// Register access functions for the ABX500 Mixed Signal IC family.
// Author: Mattias Wallin <mattias.wallin@stericsson.com>
//

    static LIST_HEAD(abx500_list);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct abx500_device_entry {
    pub list: list_head,
    pub ops: abx500_ops,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn lookup_ops(dev: *mut device, ops: *mut abx500_ops) {
    static void lookup_ops(struct device *dev, struct abx500_ops **ops)
    {
    struct abx500_device_entry *dev_entry;
// ops = NULL;
    list_for_each_entry(dev_entry, &abx500_list, list) {
    if (dev_entry.dev == dev) {
// ops = &dev_entry->ops;
    return;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn abx500_register_ops(dev: *mut device, ops: *mut abx500_ops) -> c_int {
    int abx500_register_ops(struct device *dev, struct abx500_ops *ops)
    {
    struct abx500_device_entry *dev_entry;
    dev_entry = devm_kzalloc(dev, sizeof(*dev_entry), GFP_KERNEL);
    if (!dev_entry)
    return -ENOMEM;
    dev_entry.dev = dev;
    memcpy(&dev_entry.ops, ops, sizeof(*ops));
    list_add_tail(&dev_entry.list, &abx500_list);
    return 0;
    }
    EXPORT_SYMBOL(abx500_register_ops);
#[no_mangle]
pub unsafe extern "C" fn abx500_remove_ops(dev: *mut device) {
    void abx500_remove_ops(struct device *dev)
    {
    struct abx500_device_entry *dev_entry, *tmp;
    list_for_each_entry_safe(dev_entry, tmp, &abx500_list, list)
    if (dev_entry.dev == dev)
    list_del(&dev_entry.list);
    }
    EXPORT_SYMBOL(abx500_remove_ops);
    int abx500_set_register_interruptible(struct device *dev, u8 bank, u8 reg,
    u8 value)
    {
    struct abx500_ops *ops;
    lookup_ops(dev.parent, &ops);
    if (ops && ops.set_register)
    return ops.set_register(dev, bank, reg, value);
    else
    return -ENOTSUPP;
    }
    EXPORT_SYMBOL(abx500_set_register_interruptible);
    int abx500_get_register_interruptible(struct device *dev, u8 bank, u8 reg,
    u8 *value)
    {
    struct abx500_ops *ops;
    lookup_ops(dev.parent, &ops);
    if (ops && ops.get_register)
    return ops.get_register(dev, bank, reg, value);
    else
    return -ENOTSUPP;
    }
    EXPORT_SYMBOL(abx500_get_register_interruptible);
    int abx500_get_register_page_interruptible(struct device *dev, u8 bank,
    u8 first_reg, u8 *regvals, u8 numregs)
    {
    struct abx500_ops *ops;
    lookup_ops(dev.parent, &ops);
    if (ops && ops.get_register_page)
    return ops.get_register_page(dev, bank,
    first_reg, regvals, numregs);
    else
    return -ENOTSUPP;
    }
    EXPORT_SYMBOL(abx500_get_register_page_interruptible);
    int abx500_mask_and_set_register_interruptible(struct device *dev, u8 bank,
    u8 reg, u8 bitmask, u8 bitvalues)
    {
    struct abx500_ops *ops;
    lookup_ops(dev.parent, &ops);
    if (ops && ops.mask_and_set_register)
    return ops.mask_and_set_register(dev, bank,
    reg, bitmask, bitvalues);
    else
    return -ENOTSUPP;
    }
    EXPORT_SYMBOL(abx500_mask_and_set_register_interruptible);
#[no_mangle]
pub unsafe extern "C" fn abx500_get_chip_id(dev: *mut device) -> c_int {
    int abx500_get_chip_id(struct device *dev)
    {
    struct abx500_ops *ops;
    lookup_ops(dev.parent, &ops);
    if (ops && ops.get_chip_id)
    return ops.get_chip_id(dev);
    else
    return -ENOTSUPP;
    }
    EXPORT_SYMBOL(abx500_get_chip_id);
#[no_mangle]
pub unsafe extern "C" fn abx500_event_registers_startup_state_get(dev: *mut device, event: *mut u8) -> c_int {
    int abx500_event_registers_startup_state_get(struct device *dev, u8 *event)
    {
    struct abx500_ops *ops;
    lookup_ops(dev.parent, &ops);
    if (ops && ops.event_registers_startup_state_get)
    return ops.event_registers_startup_state_get(dev, event);
    else
    return -ENOTSUPP;
    }
    EXPORT_SYMBOL(abx500_event_registers_startup_state_get);
#[no_mangle]
pub unsafe extern "C" fn abx500_startup_irq_enabled(dev: *mut device, irq: c_uint) -> c_int {
    int abx500_startup_irq_enabled(struct device *dev, unsigned int irq)
    {
    struct abx500_ops *ops;
    lookup_ops(dev.parent, &ops);
    if (ops && ops.startup_irq_enabled)
    return ops.startup_irq_enabled(dev, irq);
    else
    return -ENOTSUPP;
    }
    EXPORT_SYMBOL(abx500_startup_irq_enabled);
