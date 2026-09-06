//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_stack_var_off.c
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

    int probe_res;
    char input[4] = {};
    int test_pid;
    SEC("tracepoint/syscalls/sys_enter_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut c_void) -> c_int {
    int probe(void *ctx)
    {
// This BPF program performs variable-offset reads and writes on a
// stack-allocated buffer.
//
    char stack_buf[16];
    unsigned long len;
    unsigned long last;
    if ((bpf_get_current_pid_tgid() >> 32) != test_pid)
    return 0;
// Copy the input to the stack.
    __builtin_memcpy(stack_buf, input, 4);
// The first byte in the buffer indicates the length.
    len = stack_buf[0] & 0xf;
    last = (len - 1) & 0xf;
// Append something to the buffer. The offset where we write is not
// statically known; this is a variable-offset stack write.
//
    stack_buf[len] = 42;
// Index into the buffer at an unknown offset. This is a
// variable-offset stack read.
//
// Note that if it wasn't for the preceding variable-offset write, this
// read would be rejected because the stack slot cannot be verified as
// being initialized. With the preceding variable-offset write, the
// stack slot still cannot be verified, but the write inhibits the
// respective check on the reasoning that, if there was a
// variable-offset to a higher-or-equal spot, we're probably reading
// what we just wrote.
//
    probe_res = stack_buf[last];
    return 0;
    }
    char _license[] SEC("license") = "GPL";
