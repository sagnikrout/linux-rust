//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_ringbuf_write.c
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

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    } ringbuf SEC(".maps");
// inputs
    let mut pid: c_int = 0;
// outputs
    let mut passed: c_long = 0;
    let mut discarded: c_long = 0;
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn test_ringbuf_write(ctx: *mut c_void) -> c_int {
    int test_ringbuf_write(void *ctx)
    {
    int *foo, cur_pid = bpf_get_current_pid_tgid() >> 32;
    void *sample1, *sample2;
    if (cur_pid != pid)
    return 0;
    sample1 = bpf_ringbuf_reserve(&ringbuf, 0x30000, 0);
    if (!sample1)
    return 0;
// first one can pass
    sample2 = bpf_ringbuf_reserve(&ringbuf, 0x30000, 0);
    if (!sample2) {
    bpf_ringbuf_discard(sample1, 0);
    __sync_fetch_and_add(&discarded, 1);
    return 0;
    }
// second one must not
    __sync_fetch_and_add(&passed, 1);
    foo = sample2 + 4084;
// foo = 256;
    bpf_ringbuf_discard(sample1, 0);
    bpf_ringbuf_discard(sample2, 0);
    return 0;
    }
