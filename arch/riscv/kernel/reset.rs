//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/reset.c
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
// Copyright (C) 2012 Regents of the University of California
//

#[no_mangle]
unsafe extern "C" fn default_power_off() -> void __noreturn {
    static void __noreturn default_power_off(void)
    {
    while (1)
    wait_for_interrupt();
    }
    void (*pm_power_off)(void) = core::ptr::null_mut();
    EXPORT_SYMBOL(pm_power_off);
#[no_mangle]
pub unsafe extern "C" fn machine_restart(cmd: *mut c_char) {
    void machine_restart(char *cmd)
    {
//
// UpdateCapsule() depends on the system being reset via ResetSystem().
//
    if (efi_enabled(EFI_RUNTIME_SERVICES))
    efi_reboot(reboot_mode, core::ptr::null_mut());
    do_kernel_restart(cmd);
    while (1);
    }
#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    void machine_halt(void)
    {
    do_kernel_power_off();
    default_power_off();
    }
#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    void machine_power_off(void)
    {
    do_kernel_power_off();
    default_power_off();
    }
