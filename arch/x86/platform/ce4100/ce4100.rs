//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/ce4100/ce4100.c
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
// Intel CE4100  platform specific setup code
//
// (C) Copyright 2010 Intel Corporation
//

//
// The CE4100 platform has an internal 8051 Microcontroller which is
// responsible for signaling to the external Power Management Unit the
// intention to reset, reboot or power off the system. This 8051 device has
// its command register mapped at I/O port 0xcf9 and the value 0x4 is used
// to power off the system.
//
#[no_mangle]
unsafe extern "C" fn ce4100_power_off() {
    static void ce4100_power_off(void)
    {
    outb(0x4, 0xcf9);
    }
#[no_mangle]
unsafe extern "C" fn sdv_arch_setup() -> void __init {
    static void __init sdv_arch_setup(void)
    {
    sdv_serial_fixup();
    }
#[no_mangle]
unsafe extern "C" fn sdv_pci_init() {
    static void sdv_pci_init(void)
    {
    x86_of_pci_init();
    }
//
// CE4100 specific x86_init function overrides and early setup
// calls.
//
#[no_mangle]
pub unsafe extern "C" fn x86_ce4100_early_setup() -> void __init {
    void __init x86_ce4100_early_setup(void)
    {
    x86_init.oem.arch_setup			= sdv_arch_setup;
    x86_init.resources.probe_roms		= x86_init_noop;
    x86_init.mpparse.find_mptable		= x86_init_noop;
    x86_init.mpparse.early_parse_smp_cfg	= x86_init_noop;
    x86_init.pci.init			= ce4100_pci_init;
    x86_init.pci.init_irq			= sdv_pci_init;
//
// By default, the reboot method is ACPI which is supported by the
// CE4100 bootloader CEFDK using FADT.ResetReg Address and ResetValue
// the bootloader will however issue a system power off instead of
// reboot. By using BOOT_KBD we ensure proper system reboot as
// expected.
//
    reboot_type = BOOT_KBD;
    pm_power_off = ce4100_power_off;
    }
