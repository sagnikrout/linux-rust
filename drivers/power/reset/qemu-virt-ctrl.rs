//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/qemu-virt-ctrl.c
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
// QEMU Virt Machine System Controller Driver
//
// Copyright (C) 2026 Kuan-Wei Chiu <visitorckw@gmail.com>
//

// Registers
pub const VIRT_CTRL_REG_FEATURES: c_uint = 0x00;
pub const VIRT_CTRL_REG_CMD: c_uint = 0x04;
// Commands
pub const CMD_NOOP: c_int = 0;
pub const CMD_RESET: c_int = 1;
pub const CMD_HALT: c_int = 2;
pub const CMD_PANIC: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qemu_virt_ctrl {
    pub base: *mut void __iomem,
    pub reboot_nb: notifier_block,
}

#[no_mangle]
pub unsafe extern "C" fn virt_ctrl_write32(val: u32, addr: *mut void __iomem) {
    static inline void virt_ctrl_write32(u32 val, void __iomem *addr)
    {
    if (IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    iowrite32be(val, addr);
    else
    iowrite32(val, addr);
    }
#[no_mangle]
unsafe extern "C" fn qemu_virt_ctrl_power_off(data: *mut sys_off_data) -> c_int {
    static int qemu_virt_ctrl_power_off(struct sys_off_data *data)
    {
    struct qemu_virt_ctrl *ctrl = data.cb_data;
    virt_ctrl_write32(CMD_HALT, ctrl.base + VIRT_CTRL_REG_CMD);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn qemu_virt_ctrl_restart(data: *mut sys_off_data) -> c_int {
    static int qemu_virt_ctrl_restart(struct sys_off_data *data)
    {
    struct qemu_virt_ctrl *ctrl = data.cb_data;
    virt_ctrl_write32(CMD_RESET, ctrl.base + VIRT_CTRL_REG_CMD);
    return NOTIFY_DONE;
    }
    static int qemu_virt_ctrl_reboot_notify(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct qemu_virt_ctrl *ctrl = container_of(nb, struct qemu_virt_ctrl, reboot_nb);
    if (action == SYS_HALT)
    virt_ctrl_write32(CMD_HALT, ctrl.base + VIRT_CTRL_REG_CMD);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn qemu_virt_ctrl_probe(pdev: *mut platform_device) -> c_int {
    static int qemu_virt_ctrl_probe(struct platform_device *pdev)
    {
    struct qemu_virt_ctrl *ctrl;
    int ret;
    ctrl = devm_kzalloc(&pdev.dev, sizeof(*ctrl), GFP_KERNEL);
    if (!ctrl)
    return -ENOMEM;
    ctrl.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctrl.base))
    return PTR_ERR(ctrl.base);
    ret = devm_register_sys_off_handler(&pdev.dev,
    SYS_OFF_MODE_RESTART,
    SYS_OFF_PRIO_DEFAULT,
    qemu_virt_ctrl_restart,
    ctrl);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "cannot register restart handler\n");
    ret = devm_register_sys_off_handler(&pdev.dev,
    SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_DEFAULT,
    qemu_virt_ctrl_power_off,
    ctrl);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "cannot register power-off handler\n");
    ctrl.reboot_nb.notifier_call = qemu_virt_ctrl_reboot_notify;
    ret = devm_register_reboot_notifier(&pdev.dev, &ctrl.reboot_nb);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "cannot register reboot notifier\n");
    return 0;
    }
    static const struct platform_device_id qemu_virt_ctrl_id[] = {
    { .name = "qemu-virt-ctrl" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, qemu_virt_ctrl_id);
    static struct platform_driver qemu_virt_ctrl_driver = {
    .probe = qemu_virt_ctrl_probe,
    .driver = {
    .name = "qemu-virt-ctrl",
    },
    .id_table = qemu_virt_ctrl_id,
    };
    module_platform_driver(qemu_virt_ctrl_driver);
    MODULE_AUTHOR("Kuan-Wei Chiu <visitorckw@gmail.com>");
    MODULE_DESCRIPTION("QEMU Virt Machine System Controller Driver");
    MODULE_LICENSE("GPL");
