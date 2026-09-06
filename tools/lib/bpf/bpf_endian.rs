//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/bpf_endian.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// Isolate byte #n and put it into byte #m, for __u##b type.
// E.g., moving byte #6 (nnnnnnnn) into byte #1 (mmmmmmmm) for __u64:
// 1) xxxxxxxx nnnnnnnn xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx mmmmmmmm xxxxxxxx
// 2) nnnnnnnn xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx mmmmmmmm xxxxxxxx 00000000
// 3) 00000000 00000000 00000000 00000000 00000000 00000000 00000000 nnnnnnnn
// 4) 00000000 00000000 00000000 00000000 00000000 00000000 nnnnnnnn 00000000
//

// LLVM's BPF target selects the endianness of the CPU
// it compiles on, or the user specifies (bpfel/bpfeb),
// respectively. The used __BYTE_ORDER__ is defined by
// the compiler, we cannot rely on __BYTE_ORDER from
// libc headers, since it doesn't reflect the actual
// requested byte order.
//
// Note, LLVM's BPF target has different __builtin_bswapX()
// semantics. It does map to BPF_ALU | BPF_END | BPF_TO_BE
// in bpfel and bpfeb case, which means below, that we map
// to cpu_to_be16(). We could use it unconditionally in BPF
// case, but better not rely on it, so that this header here
// can be used from application and BPF program side, which
// use different targets.
//

