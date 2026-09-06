//! Automatically rewritten from C to Rust
//! Source: drivers/soc/litex/litex_soc_ctrl.c
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
// LiteX SoC Controller Driver
//
// Copyright (C) 2020 Antmicro <www.antmicro.com>
//

// reset register located at the base address
pub const RESET_REG_OFF: c_uint = 0x00;
pub const RESET_REG_VALUE: c_uint = 0x00000001;
pub const SCRATCH_REG_OFF: c_uint = 0x04;
pub const SCRATCH_REG_VALUE: c_uint = 0x12345678;
pub const SCRATCH_TEST_VALUE: c_uint = 0xdeadbeef;
//
// Check LiteX CSR read/write access
//
// This function reads and writes a scratch register in order to verify if CSR
// access works.
//
// In case any problems are detected, the driver should panic.
//
// Access to the LiteX CSR is, by design, done in CPU native endianness.
// The driver should not dynamically configure access functions when
// the endianness mismatch is detected. Such situation indicates problems in
// the soft SoC design and should be solved at the LiteX generator level,
// not in the software.
//
#[no_mangle]
unsafe extern "C" fn litex_check_csr_access(reg_addr: *mut void __iomem) -> c_int {
    static int litex_check_csr_access(void __iomem *reg_addr)
    {
    unsigned long reg;
    reg = litex_read32(reg_addr + SCRATCH_REG_OFF);
    if (reg != SCRATCH_REG_VALUE) {
    panic("Scratch register read error - the system is probably broken! Expected: 0x%x but got: 0x%lx",
    SCRATCH_REG_VALUE, reg);
    return -EINVAL;
    }
    litex_write32(reg_addr + SCRATCH_REG_OFF, SCRATCH_TEST_VALUE);
    reg = litex_read32(reg_addr + SCRATCH_REG_OFF);
    if (reg != SCRATCH_TEST_VALUE) {
    panic("Scratch register write error - the system is probably broken! Expected: 0x%x but got: 0x%lx",
    SCRATCH_TEST_VALUE, reg);
    return -EINVAL;
    }
// restore original value of the SCRATCH register
    litex_write32(reg_addr + SCRATCH_REG_OFF, SCRATCH_REG_VALUE);
    pr_info("LiteX SoC Controller driver initialized");
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct litex_soc_ctrl_device {
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn litex_reset_handler(data: *mut sys_off_data) -> c_int {
    static int litex_reset_handler(struct sys_off_data *data)
    {
    struct litex_soc_ctrl_device *soc_ctrl_dev = data.cb_data;
    litex_write32(soc_ctrl_dev.base + RESET_REG_OFF, RESET_REG_VALUE);
    return NOTIFY_DONE;
    }
    static const struct of_device_id litex_soc_ctrl_of_match[] = {
    {.compatible = "litex,soc-controller"},
    {},
    };
    MODULE_DEVICE_TABLE(of, litex_soc_ctrl_of_match);
#[no_mangle]
unsafe extern "C" fn litex_soc_ctrl_probe(pdev: *mut platform_device) -> c_int {
    static int litex_soc_ctrl_probe(struct platform_device *pdev)
    {
    struct litex_soc_ctrl_device *soc_ctrl_dev;
    int error;
    soc_ctrl_dev = devm_kzalloc(&pdev.dev, sizeof(*soc_ctrl_dev), GFP_KERNEL);
    if (!soc_ctrl_dev)
    return -ENOMEM;
    soc_ctrl_dev.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(soc_ctrl_dev.base))
    return PTR_ERR(soc_ctrl_dev.base);
    error = litex_check_csr_access(soc_ctrl_dev.base);
    if (error)
    return error;
    error = devm_register_restart_handler(&pdev.dev,
    litex_reset_handler,
    soc_ctrl_dev);
    if (error) {
    dev_warn(&pdev.dev, "cannot register restart handler: %d\n",
    error);
    }
    return 0;
    }
    static struct platform_driver litex_soc_ctrl_driver = {
    .driver = {
    .name = "litex-soc-controller",
    .of_match_table = litex_soc_ctrl_of_match,
    },
    .probe = litex_soc_ctrl_probe,
    };
    module_platform_driver(litex_soc_ctrl_driver);
    MODULE_DESCRIPTION("LiteX SoC Controller driver");
    MODULE_AUTHOR("Antmicro <www.antmicro.com>");
    MODULE_LICENSE("GPL v2");
