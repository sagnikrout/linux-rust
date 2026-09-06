//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/8xx/machine_check.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

#[no_mangle]
pub unsafe extern "C" fn machine_check_8xx(regs: *mut pt_regs) -> c_int {
    int machine_check_8xx(struct pt_regs *regs)
    {
    let mut reason: c_ulong = regs.msr;
    pr_err("Machine check in kernel mode.\n");
    pr_err("Caused by (from SRR1=%lx): ", reason);
    if (reason & 0x40000000)
    pr_cont("Fetch error at address %lx\n", regs.nip);
    else
    pr_cont("Data access error at address %lx\n", regs.dar);

// the qspan pci read routines can cause machine checks -- Cort
//
// yuck !!! that totally needs to go away ! There are better ways
// to deal with that than having a wart in the mcheck handler.
// -- BenH
//
    bad_page_fault(regs, SIGBUS);
    return 1;

    return 0;

    }
