//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/int340x_thermal/processor_thermal_mbox.c
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
// processor thermal device mailbox driver for Workload type hints
// Copyright (c) 2020, Intel Corporation.
//

pub const MBOX_OFFSET_DATA: c_uint = 0x5810;
pub const MBOX_OFFSET_INTERFACE: c_uint = 0x5818;
pub const MBOX_BUSY_BIT: c_int = 31;
pub const MBOX_RETRY_COUNT: c_int = 100;
    static DEFINE_MUTEX(mbox_lock);
#[no_mangle]
unsafe extern "C" fn wait_for_mbox_ready(proc_priv: *mut proc_thermal_device) -> c_int {
    static int wait_for_mbox_ready(struct proc_thermal_device *proc_priv)
    {
    u32 retries, data;
    int ret;
// Poll for rb bit == 0
    retries = MBOX_RETRY_COUNT;
    do {
    data = readl(proc_priv.mmio_base + MBOX_OFFSET_INTERFACE);
    if (data & BIT_ULL(MBOX_BUSY_BIT)) {
    ret = -EBUSY;
    continue;
    }
    ret = 0;
    break;
    } while (--retries);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn send_mbox_write_cmd(pdev: *mut pci_dev, id: u16, data: u32) -> c_int {
    static int send_mbox_write_cmd(struct pci_dev *pdev, u16 id, u32 data)
    {
    struct proc_thermal_device *proc_priv;
    u32 reg_data;
    int ret;
    proc_priv = pci_get_drvdata(pdev);
    ret = wait_for_mbox_ready(proc_priv);
    if (ret)
    return ret;
    writel(data, (proc_priv.mmio_base + MBOX_OFFSET_DATA));
// Write command register
    reg_data = BIT_ULL(MBOX_BUSY_BIT) | id;
    writel(reg_data, (proc_priv.mmio_base + MBOX_OFFSET_INTERFACE));
    return wait_for_mbox_ready(proc_priv);
    }
#[no_mangle]
unsafe extern "C" fn send_mbox_read_cmd(pdev: *mut pci_dev, id: u16, resp: *mut u64) -> c_int {
    static int send_mbox_read_cmd(struct pci_dev *pdev, u16 id, u64 *resp)
    {
    struct proc_thermal_device *proc_priv;
    u32 reg_data;
    int ret;
    proc_priv = pci_get_drvdata(pdev);
    ret = wait_for_mbox_ready(proc_priv);
    if (ret)
    return ret;
// Write command register
    reg_data = BIT_ULL(MBOX_BUSY_BIT) | id;
    writel(reg_data, (proc_priv.mmio_base + MBOX_OFFSET_INTERFACE));
    ret = wait_for_mbox_ready(proc_priv);
    if (ret)
    return ret;
    if (id == MBOX_CMD_WORKLOAD_TYPE_READ)
// resp = readl(proc_priv->mmio_base + MBOX_OFFSET_DATA);
    else
// resp = readq(proc_priv->mmio_base + MBOX_OFFSET_DATA);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn processor_thermal_send_mbox_read_cmd(pdev: *mut pci_dev, id: u16, resp: *mut u64) -> c_int {
    int processor_thermal_send_mbox_read_cmd(struct pci_dev *pdev, u16 id, u64 *resp)
    {
    int ret;
    mutex_lock(&mbox_lock);
    ret = send_mbox_read_cmd(pdev, id, resp);
    mutex_unlock(&mbox_lock);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(processor_thermal_send_mbox_read_cmd, "INT340X_THERMAL");
#[no_mangle]
pub unsafe extern "C" fn processor_thermal_send_mbox_write_cmd(pdev: *mut pci_dev, id: u16, data: u32) -> c_int {
    int processor_thermal_send_mbox_write_cmd(struct pci_dev *pdev, u16 id, u32 data)
    {
    int ret;
    mutex_lock(&mbox_lock);
    ret = send_mbox_write_cmd(pdev, id, data);
    mutex_unlock(&mbox_lock);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(processor_thermal_send_mbox_write_cmd, "INT340X_THERMAL");
pub const MBOX_CAMARILLO_RD_INTR_CONFIG: c_uint = 0x1E;
pub const MBOX_CAMARILLO_WR_INTR_CONFIG: c_uint = 0x1F;

pub const SOC_PREDICTION_TW_SHIFT: c_int = 24;
    int processor_thermal_mbox_interrupt_config(struct pci_dev *pdev, bool enable,
    int enable_bit, int time_window)
    {
    u64 data;
    int ret;
    if (!pdev)
    return -ENODEV;
    mutex_lock(&mbox_lock);
// Do read modify write for MBOX_CAMARILLO_RD_INTR_CONFIG
    ret = send_mbox_read_cmd(pdev, MBOX_CAMARILLO_RD_INTR_CONFIG,  &data);
    if (ret) {
    dev_err(&pdev.dev, "MBOX_CAMARILLO_RD_INTR_CONFIG failed\n");
    goto unlock;
    }
    if (time_window >= 0) {
    data &= ~WLT_TW_MASK;
// Program notification delay
    data |= ((u64)time_window << SOC_PREDICTION_TW_SHIFT) & WLT_TW_MASK;
    }
    if (enable)
    data |= BIT(enable_bit);
    else
    data &= ~BIT(enable_bit);
    ret = send_mbox_write_cmd(pdev, MBOX_CAMARILLO_WR_INTR_CONFIG, data);
    if (ret)
    dev_err(&pdev.dev, "MBOX_CAMARILLO_WR_INTR_CONFIG failed\n");
    unlock:
    mutex_unlock(&mbox_lock);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(processor_thermal_mbox_interrupt_config, "INT340X_THERMAL");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Processor Thermal Mail Box Interface");
