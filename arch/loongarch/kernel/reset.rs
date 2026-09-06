//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/reset.c
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    void (*pm_power_off)(void);
    EXPORT_SYMBOL(pm_power_off);
#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    void machine_halt(void)
    {

    preempt_disable();
    smp_send_stop();

    local_irq_disable();
    clear_csr_ecfg(ECFG0_IM);
    pr_notice("\n\n** You can safely turn off the power now **\n\n");
    console_flush_on_panic(CONSOLE_FLUSH_PENDING);
    while (true) {
    __asm__ __volatile__("idle 0" : : : "memory");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    void machine_power_off(void)
    {

    preempt_disable();
    smp_send_stop();

    if (!acpi_disabled)
    enable_pci_wakeup();

    do_kernel_power_off();

    efi.reset_system(EFI_RESET_SHUTDOWN, EFI_SUCCESS, 0, core::ptr::null_mut());

    while (true) {
    __asm__ __volatile__("idle 0" : : : "memory");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn machine_restart(command: *mut c_char) {
    void machine_restart(char *command)
    {

    preempt_disable();
    smp_send_stop();

    do_kernel_restart(command);

    if (efi_capsule_pending(core::ptr::null_mut()))
    efi_reboot(REBOOT_WARM, core::ptr::null_mut());
    else
    efi_reboot(REBOOT_COLD, core::ptr::null_mut());

    if (!acpi_disabled)
    acpi_reboot();
    while (true) {
    __asm__ __volatile__("idle 0" : : : "memory");
    }
    }
