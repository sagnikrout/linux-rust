//! Automatically rewritten from C to Rust
//! Source: arch/x86/um/fault.c
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


//
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Licensed under the GPL
//

// These two are from asm-um/uaccess.h and linux/module.h, check them.
    struct exception_table_entry
    {
    unsigned long insn;
    unsigned long fixup;
    };
    const struct exception_table_entry *search_exception_tables(unsigned long add);
// Compare this to arch/i386/mm/extable.c:fixup_exception()
#[no_mangle]
pub unsafe extern "C" fn arch_fixup(address: c_ulong, regs: *mut uml_pt_regs) -> c_int {
    int arch_fixup(unsigned long address, struct uml_pt_regs *regs)
    {
    const struct exception_table_entry *fixup;
    fixup = search_exception_tables(address);
    if (fixup) {
    UPT_IP(regs) = fixup.fixup;
    return 1;
    }
    return 0;
    }
