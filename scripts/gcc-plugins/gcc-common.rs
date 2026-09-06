//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/gcc-plugins/gcc-common.h
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

// Macro flag: #define GCC_COMMON_H_INCLUDED

// missing from basic_block.h...
extern "C" {
    pub fn debug_dominance_info(dir: cdi_direction);
}
extern "C" {
    pub fn debug_dominance_tree(dir: cdi_direction, root: basic_block);
}

// should come from c-tree.h if only it were installed for gcc 4.5...

// Macro flag: #define add_referenced_var(var)
// Macro flag: #define mark_sym_for_renaming(var)
// Macro flag: #define varpool_mark_needed_node(node)
// Macro flag: #define create_var_ann(var)
pub const TODO_dump_func: c_int = 0;
pub const TODO_dump_cgraph: c_int = 0;
pub const TODO_ggc_collect: c_int = 0;

pub const TODO_verify_ssa: c_int = 0;
pub const TODO_verify_flow: c_int = 0;
pub const TODO_verify_stmts: c_int = 0;
pub const TODO_verify_rtl_sharing: c_int = 0;

extern "C" {
    pub fn DECL_SECTION_NAME(_arg: decl) -> return;
}
// symtab/cgraph related

// gimple related
extern "C" {
    pub fn gimple_build_assign(_arg: lhs, _arg: subcode, _arg: op1, PASS_MEM_STAT: op2) -> return;
}

// IPA/LTO related

