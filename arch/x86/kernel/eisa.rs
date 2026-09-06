//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/eisa.c
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
// EISA specific code
//

#[no_mangle]
unsafe extern "C" fn eisa_bus_probe() -> __init int {
    static __init int eisa_bus_probe(void)
    {
    u32 *p;
    if ((xen_pv_domain() && !xen_initial_domain()) || cc_platform_has(CC_ATTR_GUEST_SEV_SNP))
    return 0;
    p = memremap(0x0FFFD9, 4, MEMREMAP_WB);
    if (p && *p == 'E' + ('I' << 8) + ('S' << 16) + ('A' << 24))
    EISA_bus = 1;
    memunmap(p);
    return 0;
    }
    subsys_initcall(eisa_bus_probe);
