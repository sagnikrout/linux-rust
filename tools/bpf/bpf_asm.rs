//! Automatically rewritten from C to Rust
//! Source: tools/bpf/bpf_asm.c
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
// Minimal BPF assembler
//
// Instead of libpcap high-level filter expressions, it can be quite
// useful to define filters in low-level BPF assembler (that is kept
// close to Steven McCanne and Van Jacobson's original BPF paper).
// In particular for BPF JIT implementors, JIT security auditors, or
// just for defining BPF expressions that contain extensions which are
// not supported by compilers.
//
// How to get into it:
//
// 1) read Documentation/networking/filter.rst
// 2) Run `bpf_asm [-c] <filter-prog file>` to translate into binary
// blob that is loadable with xt_bpf, cls_bpf et al. Note: -c will
// pretty print a C-like construct.
//
// Copyright 2013 Daniel Borkmann <borkmann@redhat.com>
//

    extern void bpf_asm_compile(FILE *fp, bool cstyle);
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    FILE *fp = stdin;
    let mut cstyle: bool = false;
    int i;
    for (i = 1; i < argc; i++) {
    if (!strncmp("-c", argv[i], 2)) {
    cstyle = true;
    continue;
    }
    fp = fopen(argv[i], "r");
    if (!fp) {
    fp = stdin;
    continue;
    }
    break;
    }
    bpf_asm_compile(fp, cstyle);
    return 0;
    }
