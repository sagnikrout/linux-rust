//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/reboot.c
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
// Copyright (C) 2014 Intel Corporation; author Matt Fleming
// Copyright (c) 2014 Red Hat, Inc., Mark Salter <msalter@redhat.com>
//

    static struct sys_off_handler *efi_sys_off_handler;
    let mut efi_reboot_quirk_mode: c_int = -1;
#[no_mangle]
pub unsafe extern "C" fn efi_reboot(reboot_mode: enum reboot_mode, __unused: *const c_char) {
    void efi_reboot(enum reboot_mode reboot_mode, const char *__unused)
    {
    const char *str[] = { "cold", "warm", "shutdown", "platform" };
    int efi_mode, cap_reset_mode;
    if (!efi_rt_services_supported(EFI_RT_SUPPORTED_RESET_SYSTEM))
    return;
    switch (reboot_mode) {
    case REBOOT_WARM:
    case REBOOT_SOFT:
    efi_mode = EFI_RESET_WARM;
    break;
    default:
    efi_mode = EFI_RESET_COLD;
    break;
    }
//
// If a quirk forced an EFI reset mode, always use that.
//
    if (efi_reboot_quirk_mode != -1)
    efi_mode = efi_reboot_quirk_mode;
    if (efi_capsule_pending(&cap_reset_mode)) {
    if (efi_mode != cap_reset_mode)
    printk(KERN_CRIT "efi: %s reset requested but pending "
    "capsule update requires %s reset... Performing "
    "%s reset.\n", str[efi_mode], str[cap_reset_mode],
    str[cap_reset_mode]);
    efi_mode = cap_reset_mode;
    }
    efi.reset_system(efi_mode, EFI_SUCCESS, 0, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn efi_poweroff_required() -> bool __weak {
    bool __weak efi_poweroff_required(void)
    {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn efi_power_off(data: *mut sys_off_data) -> c_int {
    static int efi_power_off(struct sys_off_data *data)
    {
    efi.reset_system(EFI_RESET_SHUTDOWN, EFI_SUCCESS, 0, core::ptr::null_mut());
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn efi_shutdown_init() -> int __init {
    static int __init efi_shutdown_init(void)
    {
    if (!efi_rt_services_supported(EFI_RT_SUPPORTED_RESET_SYSTEM))
    return -ENODEV;
    if (efi_poweroff_required()) {
// SYS_OFF_PRIO_FIRMWARE + 1 so that it runs before acpi_power_off
    efi_sys_off_handler =
    register_sys_off_handler(SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_FIRMWARE + 1,
    efi_power_off, core::ptr::null_mut());
    if (IS_ERR(efi_sys_off_handler))
    return PTR_ERR(efi_sys_off_handler);
    }
    return 0;
    }
    late_initcall(efi_shutdown_init);
