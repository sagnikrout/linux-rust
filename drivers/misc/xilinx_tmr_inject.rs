//! Automatically rewritten from C to Rust
//! Source: drivers/misc/xilinx_tmr_inject.c
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
// Driver for Xilinx TMR Inject IP.
//
// Copyright (C) 2022 Advanced Micro Devices, Inc.
//
// Description:
// This driver is developed for TMR Inject IP,The Triple Modular Redundancy(TMR)
// Inject provides fault injection.
//

// TMR Inject Register offsets
pub const XTMR_INJECT_CR_OFFSET: c_uint = 0x0;
pub const XTMR_INJECT_AIR_OFFSET: c_uint = 0x4;
pub const XTMR_INJECT_IIR_OFFSET: c_uint = 0xC;
pub const XTMR_INJECT_EAIR_OFFSET: c_uint = 0x10;
pub const XTMR_INJECT_ERR_OFFSET: c_uint = 0x204;
// Register Bitmasks/shifts
pub const XTMR_INJECT_CR_CPUID_SHIFT: c_int = 8;
pub const XTMR_INJECT_CR_IE_SHIFT: c_int = 10;

pub const XTMR_INJECT_MAGIC_MAX_VAL: c_int = 255;
//
// struct xtmr_inject_dev - Driver data for TMR Inject
// @regs: device physical base address
// @magic: Magic hardware configuration value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtmr_inject_dev {
    pub regs: *mut void __iomem,
    pub magic: u32,
}

    static DECLARE_FAULT_ATTR(inject_fault);
    static char *inject_request;
    module_param(inject_request, charp, 0);
    MODULE_PARM_DESC(inject_request, "default fault injection attributes");
    static struct dentry *dbgfs_root;
// IO accessors
    static inline void xtmr_inject_write(struct xtmr_inject_dev *xtmr_inject,
    u32 addr, u32 value)
    {
    iowrite32(value, xtmr_inject.regs + addr);
    }
    static inline u32 xtmr_inject_read(struct xtmr_inject_dev *xtmr_inject,
    u32 addr)
    {
    return ioread32(xtmr_inject.regs + addr);
    }
#[no_mangle]
unsafe extern "C" fn xtmr_inject_set(data: *mut c_void, val: u64) -> c_int {
    static int xtmr_inject_set(void *data, u64 val)
    {
    if (val != 1)
    return -EINVAL;
    xmb_inject_err();
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(xtmr_inject_fops, core::ptr::null_mut(), xtmr_inject_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn xtmr_init_debugfs(xtmr_inject: *mut xtmr_inject_dev) {
    static void xtmr_init_debugfs(struct xtmr_inject_dev *xtmr_inject)
    {
    struct dentry *dir;
    dbgfs_root = debugfs_create_dir("xtmr_inject", core::ptr::null_mut());
    dir = fault_create_debugfs_attr("inject_fault", dbgfs_root,
    &inject_fault);
    debugfs_create_file("inject_fault", 0200, dir, core::ptr::null_mut(),
    &xtmr_inject_fops);
    }
#[no_mangle]
unsafe extern "C" fn xtmr_inject_init(xtmr_inject: *mut xtmr_inject_dev) {
    static void xtmr_inject_init(struct xtmr_inject_dev *xtmr_inject)
    {
    u32 cr_val;
    if (inject_request)
    setup_fault_attr(&inject_fault, inject_request);
// Allow fault injection
    cr_val = xtmr_inject.magic |
    (1 << XTMR_INJECT_CR_IE_SHIFT) |
    (1 << XTMR_INJECT_CR_CPUID_SHIFT);
    xtmr_inject_write(xtmr_inject, XTMR_INJECT_CR_OFFSET,
    cr_val);
// Initialize the address inject and instruction inject registers
    xtmr_inject_write(xtmr_inject, XTMR_INJECT_AIR_OFFSET,
    XMB_INJECT_ERR_OFFSET);
    xtmr_inject_write(xtmr_inject, XTMR_INJECT_IIR_OFFSET,
    XMB_INJECT_ERR_OFFSET & XTMR_INJECT_IIR_ADDR_MASK);
    }
//
// xtmr_inject_probe - Driver probe function
// @pdev: Pointer to the platform_device structure
//
// This is the driver probe routine. It does all the memory
// allocation for the device.
//
// Return: 0 on success and failure value on error
//
#[no_mangle]
unsafe extern "C" fn xtmr_inject_probe(pdev: *mut platform_device) -> c_int {
    static int xtmr_inject_probe(struct platform_device *pdev)
    {
    struct xtmr_inject_dev *xtmr_inject;
    int err;
    xtmr_inject = devm_kzalloc(&pdev.dev, sizeof(*xtmr_inject),
    GFP_KERNEL);
    if (!xtmr_inject)
    return -ENOMEM;
    xtmr_inject.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(xtmr_inject.regs))
    return PTR_ERR(xtmr_inject.regs);
    err = of_property_read_u32(pdev.dev.of_node, "xlnx,magic",
    &xtmr_inject.magic);
    if (err < 0) {
    dev_err(&pdev.dev, "unable to read xlnx,magic property");
    return err;
    }
    if (xtmr_inject.magic > XTMR_INJECT_MAGIC_MAX_VAL) {
    dev_err(&pdev.dev, "invalid xlnx,magic property value");
    return -EINVAL;
    }
// Initialize TMR Inject
    xtmr_inject_init(xtmr_inject);
    xtmr_init_debugfs(xtmr_inject);
    platform_set_drvdata(pdev, xtmr_inject);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xtmr_inject_remove(pdev: *mut platform_device) {
    static void xtmr_inject_remove(struct platform_device *pdev)
    {
    debugfs_remove_recursive(dbgfs_root);
    dbgfs_root = core::ptr::null_mut();
    }
    static const struct of_device_id xtmr_inject_of_match[] = {
    {
    .compatible = "xlnx,tmr-inject-1.0",
    },
    { /* end of table */ }
    };
    MODULE_DEVICE_TABLE(of, xtmr_inject_of_match);
    static struct platform_driver xtmr_inject_driver = {
    .driver = {
    .name = "xilinx-tmr_inject",
    .of_match_table = xtmr_inject_of_match,
    },
    .probe = xtmr_inject_probe,
    .remove = xtmr_inject_remove,
    };
    module_platform_driver(xtmr_inject_driver);
    MODULE_AUTHOR("Advanced Micro Devices, Inc");
    MODULE_DESCRIPTION("Xilinx TMR Inject Driver");
    MODULE_LICENSE("GPL");
