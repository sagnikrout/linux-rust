//! Automatically rewritten from C to Rust
//! Source: drivers/misc/xilinx_tmr_manager.c
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
// Driver for Xilinx TMR Manager IP.
//
// Copyright (C) 2022 Advanced Micro Devices, Inc.
//
// Description:
// This driver is developed for TMR Manager,The Triple Modular Redundancy(TMR)
// Manager is responsible for handling the TMR subsystem state, including
// fault detection and error recovery. The core is triplicated in each of
// the sub-blocks in the TMR subsystem, and provides majority voting of
// its internal state provides soft error detection, correction and
// recovery.
//

// TMR Manager Register offsets
pub const XTMR_MANAGER_CR_OFFSET: c_uint = 0x0;
pub const XTMR_MANAGER_FFR_OFFSET: c_uint = 0x4;
pub const XTMR_MANAGER_CMR0_OFFSET: c_uint = 0x8;
pub const XTMR_MANAGER_CMR1_OFFSET: c_uint = 0xC;
pub const XTMR_MANAGER_BDIR_OFFSET: c_uint = 0x10;
pub const XTMR_MANAGER_SEMIMR_OFFSET: c_uint = 0x1C;
// Register Bitmasks/shifts

pub const XTMR_MANAGER_CR_MAGIC2_SHIFT: c_int = 4;
pub const XTMR_MANAGER_CR_RIR_SHIFT: c_int = 16;
pub const XTMR_MANAGER_CR_BB_SHIFT: c_int = 18;
pub const XTMR_MANAGER_MAGIC1_MAX_VAL: c_int = 255;
//
// struct xtmr_manager_dev - Driver data for TMR Manager
// @regs: device physical base address
// @cr_val: control register value
// @magic1: Magic 1 hardware configuration value
// @err_cnt: error statistics count
// @phys_baseaddr: Physical base address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtmr_manager_dev {
    pub regs: *mut void __iomem,
    pub cr_val: u32,
    pub magic1: u32,
    pub err_cnt: u32,
    pub phys_baseaddr: resource_size_t,
}

// IO accessors
    static inline void xtmr_manager_write(struct xtmr_manager_dev *xtmr_manager,
    u32 addr, u32 value)
    {
    iowrite32(value, xtmr_manager.regs + addr);
    }
    static inline u32 xtmr_manager_read(struct xtmr_manager_dev *xtmr_manager,
    u32 addr)
    {
    return ioread32(xtmr_manager.regs + addr);
    }
#[no_mangle]
unsafe extern "C" fn xmb_manager_reset_handler(xtmr_manager: *mut xtmr_manager_dev) {
    static void xmb_manager_reset_handler(struct xtmr_manager_dev *xtmr_manager)
    {
// Clear the FFR Register contents as a part of recovery process.
    xtmr_manager_write(xtmr_manager, XTMR_MANAGER_FFR_OFFSET, 0);
    }
#[no_mangle]
unsafe extern "C" fn xmb_manager_update_errcnt(xtmr_manager: *mut xtmr_manager_dev) {
    static void xmb_manager_update_errcnt(struct xtmr_manager_dev *xtmr_manager)
    {
    xtmr_manager.err_cnt++;
    }
    static ssize_t errcnt_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct xtmr_manager_dev *xtmr_manager = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%x\n", xtmr_manager.err_cnt);
    }
    static DEVICE_ATTR_RO(errcnt);
    static ssize_t dis_block_break_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    struct xtmr_manager_dev *xtmr_manager = dev_get_drvdata(dev);
    int ret;
    long value;
    ret = kstrtoul(buf, 16, &value);
    if (ret)
    return ret;
// unblock the break signal
    xtmr_manager.cr_val &= ~(1 << XTMR_MANAGER_CR_BB_SHIFT);
    xtmr_manager_write(xtmr_manager, XTMR_MANAGER_CR_OFFSET,
    xtmr_manager.cr_val);
    return size;
    }
    static DEVICE_ATTR_WO(dis_block_break);
    static struct attribute *xtmr_manager_dev_attrs[] = {
    &dev_attr_dis_block_break.attr,
    &dev_attr_errcnt.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(xtmr_manager_dev);
#[no_mangle]
unsafe extern "C" fn xtmr_manager_init(xtmr_manager: *mut xtmr_manager_dev) {
    static void xtmr_manager_init(struct xtmr_manager_dev *xtmr_manager)
    {
// Clear the SEM interrupt mask register to disable the interrupt
    xtmr_manager_write(xtmr_manager, XTMR_MANAGER_SEMIMR_OFFSET, 0);
// Allow recovery reset by default
    xtmr_manager.cr_val = (1 << XTMR_MANAGER_CR_RIR_SHIFT) |
    xtmr_manager.magic1;
    xtmr_manager_write(xtmr_manager, XTMR_MANAGER_CR_OFFSET,
    xtmr_manager.cr_val);
//
// Configure Break Delay Initialization Register to zero so that
// break occurs immediately
//
    xtmr_manager_write(xtmr_manager, XTMR_MANAGER_BDIR_OFFSET, 0);
//
// To come out of break handler need to block the break signal
// in the tmr manager, update the xtmr_manager cr_val for the same
//
    xtmr_manager.cr_val |= (1 << XTMR_MANAGER_CR_BB_SHIFT);
//
// When the break vector gets asserted because of error injection,
// the break signal must be blocked before exiting from the
// break handler, Below api updates the TMR manager address and
// control register and error counter callback arguments,
// which will be used by the break handler to block the
// break and call the callback function.
//
    xmb_manager_register(xtmr_manager.phys_baseaddr, xtmr_manager.cr_val,
    (void *)xmb_manager_update_errcnt,
    xtmr_manager, (void *)xmb_manager_reset_handler);
    }
//
// xtmr_manager_probe - Driver probe function
// @pdev: Pointer to the platform_device structure
//
// This is the driver probe routine. It does all the memory
// allocation for the device.
//
// Return: 0 on success and failure value on error
//
#[no_mangle]
unsafe extern "C" fn xtmr_manager_probe(pdev: *mut platform_device) -> c_int {
    static int xtmr_manager_probe(struct platform_device *pdev)
    {
    struct xtmr_manager_dev *xtmr_manager;
    struct resource *res;
    int err;
    xtmr_manager = devm_kzalloc(&pdev.dev, sizeof(*xtmr_manager),
    GFP_KERNEL);
    if (!xtmr_manager)
    return -ENOMEM;
    xtmr_manager.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(xtmr_manager.regs))
    return PTR_ERR(xtmr_manager.regs);
    xtmr_manager.phys_baseaddr = res.start;
    err = of_property_read_u32(pdev.dev.of_node, "xlnx,magic1",
    &xtmr_manager.magic1);
    if (err < 0) {
    dev_err(&pdev.dev, "unable to read xlnx,magic1 property");
    return err;
    }
    if (xtmr_manager.magic1 > XTMR_MANAGER_MAGIC1_MAX_VAL) {
    dev_err(&pdev.dev, "invalid xlnx,magic1 property value");
    return -EINVAL;
    }
// Initialize TMR Manager
    xtmr_manager_init(xtmr_manager);
    platform_set_drvdata(pdev, xtmr_manager);
    return 0;
    }
    static const struct of_device_id xtmr_manager_of_match[] = {
    {
    .compatible = "xlnx,tmr-manager-1.0",
    },
    { /* end of table */ }
    };
    MODULE_DEVICE_TABLE(of, xtmr_manager_of_match);
    static struct platform_driver xtmr_manager_driver = {
    .driver = {
    .name = "xilinx-tmr_manager",
    .of_match_table = xtmr_manager_of_match,
    .dev_groups = xtmr_manager_dev_groups,
    },
    .probe = xtmr_manager_probe,
    };
    module_platform_driver(xtmr_manager_driver);
    MODULE_AUTHOR("Advanced Micro Devices, Inc");
    MODULE_DESCRIPTION("Xilinx TMR Manager Driver");
    MODULE_LICENSE("GPL");
