//! Automatically rewritten from C to Rust
//! Source: tools/build/feature/test-bpf.c
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

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    union bpf_attr attr;
// Check fields in attr
    attr.prog_type = BPF_PROG_TYPE_KPROBE;
    attr.insn_cnt = 0;
    attr.insns = 0;
    attr.license = 0;
    attr.log_buf = 0;
    attr.log_size = 0;
    attr.log_level = 0;
    attr.kern_version = 0;
    attr.prog_flags = 0;
//
// Test existence of __NR_bpf and BPF_PROG_LOAD.
// This call should fail if we run the testcase.
//
    return syscall(__NR_bpf, BPF_PROG_LOAD, &attr, sizeof(attr)) == 0;
    }
