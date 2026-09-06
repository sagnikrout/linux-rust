//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/wakeup.c
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
// wakeup.c - support wakeup devices
// Copyright (C) 2004 Li Shaohua <shaohua.li@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_wakeup_handler {
    pub list_node: list_head,
    pub context): *mut *mut bool (wakeup)(void,
    pub context: *mut c_void,
}

    static LIST_HEAD(acpi_wakeup_handler_head);
    static DEFINE_MUTEX(acpi_wakeup_handler_mutex);
//
// We didn't lock acpi_device_lock in the file, because it invokes oops in
// suspend/resume and isn't really required as this is called in S-state. At
// that time, there is no device hotplug
//
// acpi_enable_wakeup_devices - Enable wake-up device GPEs.
// @sleep_state: ACPI system sleep state.
//
// Enable wakeup device power of devices with the state.enable flag set and set
// the wakeup enable mask bits in the GPE registers that correspond to wakeup
// devices.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_enable_wakeup_devices(sleep_state: u8) {
    void acpi_enable_wakeup_devices(u8 sleep_state)
    {
    struct acpi_device *dev, *tmp;
    list_for_each_entry_safe(dev, tmp, &acpi_wakeup_device_list,
    wakeup_list) {
    if (!dev.wakeup.flags.valid
    || sleep_state > dev.wakeup.sleep_state
    || !(device_may_wakeup(&dev.dev)
    || dev.wakeup.prepare_count))
    continue;
    if (device_may_wakeup(&dev.dev))
    acpi_enable_wakeup_device_power(dev, sleep_state);
// The wake-up power should have been enabled already.
    acpi_set_gpe_wake_mask(dev.wakeup.gpe_device, dev.wakeup.gpe_number,
    ACPI_GPE_ENABLE);
    }
    }
//
// acpi_disable_wakeup_devices - Disable devices' wakeup capability.
// @sleep_state: ACPI system sleep state.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_disable_wakeup_devices(sleep_state: u8) {
    void acpi_disable_wakeup_devices(u8 sleep_state)
    {
    struct acpi_device *dev, *tmp;
    list_for_each_entry_safe(dev, tmp, &acpi_wakeup_device_list,
    wakeup_list) {
    if (!dev.wakeup.flags.valid
    || sleep_state > dev.wakeup.sleep_state
    || !(device_may_wakeup(&dev.dev)
    || dev.wakeup.prepare_count))
    continue;
    acpi_set_gpe_wake_mask(dev.wakeup.gpe_device, dev.wakeup.gpe_number,
    ACPI_GPE_DISABLE);
    if (device_may_wakeup(&dev.dev))
    acpi_disable_wakeup_device_power(dev);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_wakeup_device_init() -> int __init {
    int __init acpi_wakeup_device_init(void)
    {
    struct acpi_device *dev, *tmp;
    mutex_lock(&acpi_device_lock);
    list_for_each_entry_safe(dev, tmp, &acpi_wakeup_device_list,
    wakeup_list) {
    if (device_can_wakeup(&dev.dev)) {
// Button GPEs are supposed to be always enabled.
    acpi_enable_gpe(dev.wakeup.gpe_device,
    dev.wakeup.gpe_number);
    device_set_wakeup_enable(&dev.dev, true);
    }
    }
    mutex_unlock(&acpi_device_lock);
    return 0;
    }
//
// acpi_register_wakeup_handler - Register wakeup handler
// @wake_irq: The IRQ through which the device may receive wakeups
// @wakeup:   Wakeup-handler to call when the SCI has triggered a wakeup
// @context:  Context to pass to the handler when calling it
//
// Drivers which may share an IRQ with the SCI can use this to register
// a handler which returns true when the device they are managing wants
// to trigger a wakeup.
//
    int acpi_register_wakeup_handler(int wake_irq, bool (*wakeup)(void *context),
    void *context)
    {
    struct acpi_wakeup_handler *handler;
//
// If the device is not sharing its IRQ with the SCI, there is no
// need to register the handler.
//
    if (!acpi_sci_irq_valid() || wake_irq != acpi_sci_irq)
    return 0;
    handler = kmalloc_obj(*handler);
    if (!handler)
    return -ENOMEM;
    handler.wakeup = wakeup;
    handler.context = context;
    mutex_lock(&acpi_wakeup_handler_mutex);
    list_add(&handler.list_node, &acpi_wakeup_handler_head);
    mutex_unlock(&acpi_wakeup_handler_mutex);
    return 0;
    }
    EXPORT_SYMBOL_GPL(acpi_register_wakeup_handler);
//
// acpi_unregister_wakeup_handler - Unregister wakeup handler
// @wakeup:   Wakeup-handler passed to acpi_register_wakeup_handler()
// @context:  Context passed to acpi_register_wakeup_handler()
//
    void acpi_unregister_wakeup_handler(bool (*wakeup)(void *context),
    void *context)
    {
    struct acpi_wakeup_handler *handler;
    mutex_lock(&acpi_wakeup_handler_mutex);
    list_for_each_entry(handler, &acpi_wakeup_handler_head, list_node) {
    if (handler.wakeup == wakeup && handler.context == context) {
    list_del(&handler.list_node);
    kfree(handler);
    break;
    }
    }
    mutex_unlock(&acpi_wakeup_handler_mutex);
    }
    EXPORT_SYMBOL_GPL(acpi_unregister_wakeup_handler);
#[no_mangle]
pub unsafe extern "C" fn acpi_check_wakeup_handlers() -> bool {
    bool acpi_check_wakeup_handlers(void)
    {
    struct acpi_wakeup_handler *handler;
// No need to lock, nothing else is running when we're called.
    list_for_each_entry(handler, &acpi_wakeup_handler_head, list_node) {
    if (handler.wakeup(handler.context))
    return true;
    }
    return false;
    }
