//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/elfcore-compat.h
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
// On amd64 we have two 32bit ABIs - i386 and x32.  The latter
// has bigger registers, so we use it for compat_elf_regset_t.
// The former uses i386_elf_prstatus and PRSTATUS_SIZE/SET_PR_FPVALID
// are used to choose the size and location of ->pr_fpvalid of
// the layout actually used.
//
pub type compat_elf_gregset_t = user_regs_struct;

