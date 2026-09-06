//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/int340x_thermal/processor_thermal_power_floor.c
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
// Processor thermal device module for registering and processing
// power floor. When the hardware reduces the power to the minimum
// possible, the power floor is notified via an interrupt.
//
// Operation:
// When user space enables power floor reporting:
// - Use mailbox to:
// Enable processor thermal device interrupt
//
// - Current status of power floor is read from offset 0x5B18
// bit 39.
//
// Two interface functions are provided to call when there is a
// thermal device interrupt:
// - proc_thermal_power_floor_intr():
// Check if the interrupt is for change in power floor.
// Called from interrupt context.
//
// - proc_thermal_power_floor_intr_callback():
// Callback for interrupt processing in thread context. This involves
// sending notification to user space that there is a change in the
// power floor status.
//
// Copyright (c) 2023, Intel Corporation.
//

pub const SOC_POWER_FLOOR_SHIFT: c_int = 39;
pub const SOC_POWER_FLOOR_INT_ENABLE_BIT: c_int = 31;

#[no_mangle]
pub unsafe extern "C" fn proc_thermal_read_power_floor_status(proc_priv: *mut proc_thermal_device) -> c_int {
    int proc_thermal_read_power_floor_status(struct proc_thermal_device *proc_priv)
    {
    let mut status: u64 = 0;
    status = readq(proc_priv.mmio_base + SOC_WT_RES_INT_STATUS_OFFSET);
    return (status & SOC_POWER_FLOOR_STATUS) >> SOC_POWER_FLOOR_SHIFT;
    }
    EXPORT_SYMBOL_NS_GPL(proc_thermal_read_power_floor_status, "INT340X_THERMAL");
    static bool enable_state;
    static DEFINE_MUTEX(pf_lock);
#[no_mangle]
pub unsafe extern "C" fn proc_thermal_power_floor_set_state(proc_priv: *mut proc_thermal_device, enable: bool) -> c_int {
    int proc_thermal_power_floor_set_state(struct proc_thermal_device *proc_priv, bool enable)
    {
    let mut ret: c_int = 0;
    mutex_lock(&pf_lock);
    if (enable_state == enable)
    goto pf_unlock;
//
// Time window parameter is not applicable to power floor interrupt configuration.
// Hence use -1 for time window.
//
    ret = processor_thermal_mbox_interrupt_config(to_pci_dev(proc_priv.dev), enable,
    SOC_POWER_FLOOR_INT_ENABLE_BIT, -1);
    if (!ret)
    enable_state = enable;
    pf_unlock:
    mutex_unlock(&pf_lock);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(proc_thermal_power_floor_set_state, "INT340X_THERMAL");
#[no_mangle]
pub unsafe extern "C" fn proc_thermal_power_floor_get_state(proc_priv: *mut proc_thermal_device) -> bool {
    bool proc_thermal_power_floor_get_state(struct proc_thermal_device *proc_priv)
    {
    return enable_state;
    }
    EXPORT_SYMBOL_NS_GPL(proc_thermal_power_floor_get_state, "INT340X_THERMAL");
//
// proc_thermal_check_power_floor_intr() - Check power floor interrupt.
// @proc_priv: Processor thermal device instance.
//
// Callback to check if the interrupt for power floor is active.
//
// Context: Called from interrupt context.
//
// Return: true if power floor is active, false when not active.
//
#[no_mangle]
pub unsafe extern "C" fn proc_thermal_check_power_floor_intr(proc_priv: *mut proc_thermal_device) -> bool {
    bool proc_thermal_check_power_floor_intr(struct proc_thermal_device *proc_priv)
    {
    u64 int_status;
    int_status = readq(proc_priv.mmio_base + SOC_WT_RES_INT_STATUS_OFFSET);
    return !!(int_status & SOC_POWER_FLOOR_INT_ACTIVE);
    }
    EXPORT_SYMBOL_NS_GPL(proc_thermal_check_power_floor_intr, "INT340X_THERMAL");
//
// proc_thermal_power_floor_intr_callback() - Process power floor notification
// @pdev:	PCI device instance
// @proc_priv: Processor thermal device instance.
//
// Check if the power floor interrupt is active, if active send notification to
// user space for the attribute "power_limits", so that user can read the attribute
// and take action.
//
// Context: Called from interrupt thread context.
//
// Return: None.
//
    void proc_thermal_power_floor_intr_callback(struct pci_dev *pdev,
    struct proc_thermal_device *proc_priv)
    {
    u64 status;
    status = readq(proc_priv.mmio_base + SOC_WT_RES_INT_STATUS_OFFSET);
    if (!(status & SOC_POWER_FLOOR_INT_ACTIVE))
    return;
    sysfs_notify(&pdev.dev.kobj, "power_limits", "power_floor_status");
    }
    EXPORT_SYMBOL_NS_GPL(proc_thermal_power_floor_intr_callback, "INT340X_THERMAL");
    MODULE_IMPORT_NS("INT340X_THERMAL");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Processor Thermal power floor notification Interface");
