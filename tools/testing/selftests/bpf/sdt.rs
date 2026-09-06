//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/sdt.h
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


// <sys/sdt.h> - Systemtap static probe definition macros.
//
pub const _SYS_SDT_H: c_int = 1;
//

//

extern "C" {
    pub fn __volatile__((provider##_##name##_semaphore): "" :: "m") -> __asm__;
}

// _SDT_S encodes the size and type as 0xSSTT which is decoded by the assembler

// __SDT_COND_SIGNED(char16_t)
// __SDT_COND_SIGNED(char32_t)

// __CHAR_BIT__ - 1))) == 0)	\

// NB: gdb PR24541 highlighted an unspecified corner of the sdt.h
//

// The ia64 and s390 nop instructions take an argument.

pub const _SDT_NOTE_TYPE: c_int = 3;
// If the assembler supports the necessary feature, then we can play

// These macros can be used in C, C++, or assembly code.

// Macro flag: #define STAP_PROBE9(provider,name,arg1,arg2,arg3,arg4,arg5,arg6,arg7,arg8,arg9)\

// This STAP_PROBEV macro can be used in variadic scenarios, where the

// These macros are for use in asm statements.  You must compile

// DTrace compatible macro names.

