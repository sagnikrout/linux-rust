//! Automatically rewritten from C to Rust
//! Source: tools/objtool/arch/powerpc/special.c
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

    bool arch_support_alt_relocation(struct special_alt *special_alt,
    struct instruction *insn,
    struct reloc *reloc)
    {
    exit(-1);
    }
    struct reloc *arch_find_switch_table(struct objtool_file *file,
    struct instruction *insn,
    unsigned long *table_size)
    {
    exit(-1);
    }
    const char *arch_cpu_feature_name(int feature_number)
    {
    return core::ptr::null_mut();
    }
