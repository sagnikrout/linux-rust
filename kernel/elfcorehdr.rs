//! Automatically rewritten from C to Rust
//! Source: kernel/elfcorehdr.c
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
// stores the physical address of elf header of crash image
//
// Note: elfcorehdr_addr is not just limited to vmcore. It is also used by
// is_kdump_kernel() to determine if we are booting after a panic. Hence put
// it under CONFIG_CRASH_DUMP and not CONFIG_PROC_VMCORE.
//
    let mut elfcorehdr_addr: c_ulonglong = ELFCORE_ADDR_MAX;
    EXPORT_SYMBOL_GPL(elfcorehdr_addr);
//
// stores the size of elf header of crash image
//
    unsigned long long elfcorehdr_size;
//
// elfcorehdr= specifies the location of elf core header stored by the crashed
// kernel. This option will be passed by kexec loader to the capture kernel.
//
// Syntax: elfcorehdr=[size[KMG]@]offset[KMG]
//
#[no_mangle]
unsafe extern "C" fn setup_elfcorehdr(arg: *mut c_char) -> int __init {
    static int __init setup_elfcorehdr(char *arg)
    {
    char *end;
    if (!arg)
    return -EINVAL;
    elfcorehdr_addr = memparse(arg, &end);
    if (*end == '@') {
    elfcorehdr_size = elfcorehdr_addr;
    elfcorehdr_addr = memparse(end + 1, &end);
    }
    return end > arg ? 0 : -EINVAL;
    }
    early_param("elfcorehdr", setup_elfcorehdr);
