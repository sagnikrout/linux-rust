//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/platform-quirks.c
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

#[no_mangle]
pub unsafe extern "C" fn x86_early_init_platform_quirks() -> void __init {
    void __init x86_early_init_platform_quirks(void)
    {
    x86_platform.legacy.i8042 = X86_LEGACY_I8042_EXPECTED_PRESENT;
    x86_platform.legacy.rtc = 1;
    x86_platform.legacy.warm_reset = 1;
    x86_platform.legacy.reserve_bios_regions = 0;
    x86_platform.legacy.devices.pnpbios = 1;
    switch (boot_params.hdr.hardware_subarch) {
    case X86_SUBARCH_PC:
    x86_platform.legacy.reserve_bios_regions = 1;
    break;
    case X86_SUBARCH_XEN:
    x86_platform.legacy.devices.pnpbios = 0;
    x86_platform.legacy.rtc = 0;
    break;
    case X86_SUBARCH_INTEL_MID:
    case X86_SUBARCH_CE4100:
    x86_platform.legacy.devices.pnpbios = 0;
    x86_platform.legacy.rtc = 0;
    x86_platform.legacy.i8042 = X86_LEGACY_I8042_PLATFORM_ABSENT;
    break;
    }
    if (x86_platform.set_legacy_features)
    x86_platform.set_legacy_features();
    }
#[no_mangle]
pub unsafe extern "C" fn x86_pnpbios_disabled() -> bool __init {
    bool __init x86_pnpbios_disabled(void)
    {
    return x86_platform.legacy.devices.pnpbios == 0;
    }

#[no_mangle]
pub unsafe extern "C" fn arch_pnpbios_disabled() -> bool __init {
    bool __init arch_pnpbios_disabled(void)
    {
    return x86_pnpbios_disabled();
    }
