//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/bugs.c
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
// Copyright (C) 2024 Rivos Inc.
//

    static enum mitigation_state ghostwrite_state;
#[no_mangle]
pub unsafe extern "C" fn ghostwrite_set_vulnerable() {
    void ghostwrite_set_vulnerable(void)
    {
    ghostwrite_state = VULNERABLE;
    }
//
// Vendor extension alternatives will use the value set at the time of boot
// alternative patching, thus this must be called before boot alternatives are
// patched (and after extension probing) to be effective.
//
// Returns true if mitgated, false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn ghostwrite_enable_mitigation() -> bool {
    bool ghostwrite_enable_mitigation(void)
    {
    if (IS_ENABLED(CONFIG_RISCV_ISA_XTHEADVECTOR) &&
    ghostwrite_state == VULNERABLE && !cpu_mitigations_off()) {
    disable_xtheadvector();
    ghostwrite_state = MITIGATED;
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn ghostwrite_get_state() -> enum mitigation_state {
    enum mitigation_state ghostwrite_get_state(void)
    {
    return ghostwrite_state;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_show_ghostwrite(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    ssize_t cpu_show_ghostwrite(struct device *dev, struct device_attribute *attr, char *buf)
    {
    if (IS_ENABLED(CONFIG_RISCV_ISA_XTHEADVECTOR)) {
    switch (ghostwrite_state) {
    case UNAFFECTED:
    return sysfs_emit(buf, "Not affected\n");
    case MITIGATED:
    return sysfs_emit(buf, "Mitigation: xtheadvector disabled\n");
    case VULNERABLE:
    fallthrough;
    default:
    return sysfs_emit(buf, "Vulnerable\n");
    }
    }
    return sysfs_emit(buf, "Not affected\n");
    }
