//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/maccess.c
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

#[no_mangle]
pub unsafe extern "C" fn copy_from_kernel_nofault_allowed(unsafe_src: *const c_void, size: usize) -> bool {
    bool copy_from_kernel_nofault_allowed(const void *unsafe_src, size_t size)
    {
    let mut vaddr: c_ulong = (unsigned long)unsafe_src;
//
// Do not allow userspace addresses.  This disallows
// normal userspace and the userspace guard page:
//
    if (vaddr < TASK_SIZE_MAX + PAGE_SIZE)
    return false;
//
// Reading from the vsyscall page may cause an unhandled fault in
// certain cases.  Though it is at an address above TASK_SIZE_MAX, it is
// usually considered as a user space address.
//
    if (is_vsyscall_vaddr(vaddr))
    return false;
//
// Allow everything during early boot before 'x86_virt_bits'
// is initialized.  Needed for instruction decoding in early
// exception handlers.
//
    if (!boot_cpu_data.x86_virt_bits)
    return true;
    return __is_canonical_address(vaddr, boot_cpu_data.x86_virt_bits);
    }

#[no_mangle]
pub unsafe extern "C" fn copy_from_kernel_nofault_allowed(unsafe_src: *const c_void, size: usize) -> bool {
    bool copy_from_kernel_nofault_allowed(const void *unsafe_src, size_t size)
    {
    return (unsigned long)unsafe_src >= TASK_SIZE_MAX;
    }
