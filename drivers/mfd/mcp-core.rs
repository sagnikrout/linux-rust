//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/mcp-core.c
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
// linux/drivers/mfd/mcp-core.c
//
// Copyright (C) 2001 Russell King
//
// Generic MCP (Multimedia Communications Port) layer.  All MCP locking
// is solely held within this file.
//

#[no_mangle]
unsafe extern "C" fn mcp_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int mcp_bus_match(struct device *dev, const struct device_driver *drv)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn mcp_bus_probe(dev: *mut device) -> c_int {
    static int mcp_bus_probe(struct device *dev)
    {
    struct mcp *mcp = to_mcp(dev);
    struct mcp_driver *drv = to_mcp_driver(dev.driver);
    return drv.probe(mcp);
    }
#[no_mangle]
unsafe extern "C" fn mcp_bus_remove(dev: *mut device) {
    static void mcp_bus_remove(struct device *dev)
    {
    struct mcp *mcp = to_mcp(dev);
    struct mcp_driver *drv = to_mcp_driver(dev.driver);
    drv.remove(mcp);
    }
    static const struct bus_type mcp_bus_type = {
    .name		= "mcp",
    .match		= mcp_bus_match,
    .probe		= mcp_bus_probe,
    .remove		= mcp_bus_remove,
    };
//
// mcp_set_telecom_divisor - set the telecom divisor
// @mcp: MCP interface structure
// @div: SIB clock divisor
//
// Set the telecom divisor on the MCP interface.  The resulting
// sample rate is SIBCLOCK/div.
//
#[no_mangle]
pub unsafe extern "C" fn mcp_set_telecom_divisor(mcp: *mut mcp, div: c_uint) {
    void mcp_set_telecom_divisor(struct mcp *mcp, unsigned int div)
    {
    unsigned long flags;
    spin_lock_irqsave(&mcp.lock, flags);
    mcp.ops.set_telecom_divisor(mcp, div);
    spin_unlock_irqrestore(&mcp.lock, flags);
    }
    EXPORT_SYMBOL(mcp_set_telecom_divisor);
//
// mcp_set_audio_divisor - set the audio divisor
// @mcp: MCP interface structure
// @div: SIB clock divisor
//
// Set the audio divisor on the MCP interface.
//
#[no_mangle]
pub unsafe extern "C" fn mcp_set_audio_divisor(mcp: *mut mcp, div: c_uint) {
    void mcp_set_audio_divisor(struct mcp *mcp, unsigned int div)
    {
    unsigned long flags;
    spin_lock_irqsave(&mcp.lock, flags);
    mcp.ops.set_audio_divisor(mcp, div);
    spin_unlock_irqrestore(&mcp.lock, flags);
    }
    EXPORT_SYMBOL(mcp_set_audio_divisor);
//
// mcp_reg_write - write a device register
// @mcp: MCP interface structure
// @reg: 4-bit register index
// @val: 16-bit data value
//
// Write a device register.  The MCP interface must be enabled
// to prevent this function hanging.
//
#[no_mangle]
pub unsafe extern "C" fn mcp_reg_write(mcp: *mut mcp, reg: c_uint, val: c_uint) {
    void mcp_reg_write(struct mcp *mcp, unsigned int reg, unsigned int val)
    {
    unsigned long flags;
    spin_lock_irqsave(&mcp.lock, flags);
    mcp.ops.reg_write(mcp, reg, val);
    spin_unlock_irqrestore(&mcp.lock, flags);
    }
    EXPORT_SYMBOL(mcp_reg_write);
//
// mcp_reg_read - read a device register
// @mcp: MCP interface structure
// @reg: 4-bit register index
//
// Read a device register and return its value.  The MCP interface
// must be enabled to prevent this function hanging.
//
#[no_mangle]
pub unsafe extern "C" fn mcp_reg_read(mcp: *mut mcp, reg: c_uint) -> c_uint {
    unsigned int mcp_reg_read(struct mcp *mcp, unsigned int reg)
    {
    unsigned long flags;
    unsigned int val;
    spin_lock_irqsave(&mcp.lock, flags);
    val = mcp.ops.reg_read(mcp, reg);
    spin_unlock_irqrestore(&mcp.lock, flags);
    return val;
    }
    EXPORT_SYMBOL(mcp_reg_read);
//
// mcp_enable - enable the MCP interface
// @mcp: MCP interface to enable
//
// Enable the MCP interface.  Each call to mcp_enable will need
// a corresponding call to mcp_disable to disable the interface.
//
#[no_mangle]
pub unsafe extern "C" fn mcp_enable(mcp: *mut mcp) {
    void mcp_enable(struct mcp *mcp)
    {
    unsigned long flags;
    spin_lock_irqsave(&mcp.lock, flags);
    if (mcp.use_count++ == 0)
    mcp.ops.enable(mcp);
    spin_unlock_irqrestore(&mcp.lock, flags);
    }
    EXPORT_SYMBOL(mcp_enable);
//
// mcp_disable - disable the MCP interface
// @mcp: MCP interface to disable
//
// Disable the MCP interface.  The MCP interface will only be
// disabled once the number of calls to mcp_enable matches the
// number of calls to mcp_disable.
//
#[no_mangle]
pub unsafe extern "C" fn mcp_disable(mcp: *mut mcp) {
    void mcp_disable(struct mcp *mcp)
    {
    unsigned long flags;
    spin_lock_irqsave(&mcp.lock, flags);
    if (--mcp.use_count == 0)
    mcp.ops.disable(mcp);
    spin_unlock_irqrestore(&mcp.lock, flags);
    }
    EXPORT_SYMBOL(mcp_disable);
#[no_mangle]
unsafe extern "C" fn mcp_release(dev: *mut device) {
    static void mcp_release(struct device *dev)
    {
    struct mcp *mcp = container_of(dev, struct mcp, attached_device);
    kfree(mcp);
    }
    struct mcp *mcp_host_alloc(struct device *parent, size_t size)
    {
    struct mcp *mcp;
    mcp = kzalloc(sizeof(struct mcp) + size, GFP_KERNEL);
    if (mcp) {
    spin_lock_init(&mcp.lock);
    device_initialize(&mcp.attached_device);
    mcp.attached_device.parent = parent;
    mcp.attached_device.bus = &mcp_bus_type;
    mcp.attached_device.dma_mask = parent.dma_mask;
    mcp.attached_device.release = mcp_release;
    }
    return mcp;
    }
    EXPORT_SYMBOL(mcp_host_alloc);
#[no_mangle]
pub unsafe extern "C" fn mcp_host_add(mcp: *mut mcp, pdata: *mut c_void) -> c_int {
    int mcp_host_add(struct mcp *mcp, void *pdata)
    {
    mcp.attached_device.platform_data = pdata;
    dev_set_name(&mcp.attached_device, "mcp0");
    return device_add(&mcp.attached_device);
    }
    EXPORT_SYMBOL(mcp_host_add);
#[no_mangle]
pub unsafe extern "C" fn mcp_host_del(mcp: *mut mcp) {
    void mcp_host_del(struct mcp *mcp)
    {
    device_del(&mcp.attached_device);
    }
    EXPORT_SYMBOL(mcp_host_del);
#[no_mangle]
pub unsafe extern "C" fn mcp_host_free(mcp: *mut mcp) {
    void mcp_host_free(struct mcp *mcp)
    {
    put_device(&mcp.attached_device);
    }
    EXPORT_SYMBOL(mcp_host_free);
#[no_mangle]
pub unsafe extern "C" fn mcp_driver_register(mcpdrv: *mut mcp_driver) -> c_int {
    int mcp_driver_register(struct mcp_driver *mcpdrv)
    {
    mcpdrv.drv.bus = &mcp_bus_type;
    return driver_register(&mcpdrv.drv);
    }
    EXPORT_SYMBOL(mcp_driver_register);
#[no_mangle]
pub unsafe extern "C" fn mcp_driver_unregister(mcpdrv: *mut mcp_driver) {
    void mcp_driver_unregister(struct mcp_driver *mcpdrv)
    {
    driver_unregister(&mcpdrv.drv);
    }
    EXPORT_SYMBOL(mcp_driver_unregister);
#[no_mangle]
unsafe extern "C" fn mcp_init() -> int __init {
    static int __init mcp_init(void)
    {
    return bus_register(&mcp_bus_type);
    }
#[no_mangle]
unsafe extern "C" fn mcp_exit() -> void __exit {
    static void __exit mcp_exit(void)
    {
    bus_unregister(&mcp_bus_type);
    }
    module_init(mcp_init);
    module_exit(mcp_exit);
    MODULE_AUTHOR("Russell King <rmk@arm.linux.org.uk>");
    MODULE_DESCRIPTION("Core multimedia communications port driver");
    MODULE_LICENSE("GPL");
