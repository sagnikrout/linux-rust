//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/asm-offsets.c
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
// Macro flag: #define COMPILE_OFFSETS

// workaround for a warning with -Wmissing-prototypes
    void foo(void);
#[no_mangle]
pub unsafe extern "C" fn foo() {
    void foo(void)
    {
    DEFINE(KERNEL_MADV_REMOVE, MADV_REMOVE);
    DEFINE(UM_KERN_PAGE_SIZE, PAGE_SIZE);
    DEFINE(UM_KERN_PAGE_MASK, PAGE_MASK);
    DEFINE(UM_KERN_PAGE_SHIFT, PAGE_SHIFT);
    DEFINE(UM_GFP_KERNEL, GFP_KERNEL);
    DEFINE(UM_GFP_ATOMIC, GFP_ATOMIC);
    DEFINE(UM_THREAD_SIZE, THREAD_SIZE);
    DEFINE(UM_NSEC_PER_SEC, NSEC_PER_SEC);
    DEFINE(UM_NSEC_PER_USEC, NSEC_PER_USEC);
    DEFINE(UM_KERN_GDT_ENTRY_TLS_ENTRIES, GDT_ENTRY_TLS_ENTRIES);
    DEFINE(UM_SECCOMP_ARCH_NATIVE, SECCOMP_ARCH_NATIVE);
    DEFINE(HOSTFS_ATTR_MODE, ATTR_MODE);
    DEFINE(HOSTFS_ATTR_UID, ATTR_UID);
    DEFINE(HOSTFS_ATTR_GID, ATTR_GID);
    DEFINE(HOSTFS_ATTR_SIZE, ATTR_SIZE);
    DEFINE(HOSTFS_ATTR_ATIME, ATTR_ATIME);
    DEFINE(HOSTFS_ATTR_MTIME, ATTR_MTIME);
    DEFINE(HOSTFS_ATTR_CTIME, ATTR_CTIME);
    DEFINE(HOSTFS_ATTR_ATIME_SET, ATTR_ATIME_SET);
    DEFINE(HOSTFS_ATTR_MTIME_SET, ATTR_MTIME_SET);
    DEFINE(ALT_INSTR_SIZE, sizeof(struct alt_instr));
    DEFINE(EXTABLE_SIZE,   sizeof(struct exception_table_entry));
    }
