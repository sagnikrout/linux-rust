//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_signed_loader_map.c
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

//
// One explicit array map and no global variables, so the generated loader
// has exactly one map to create (no .rodata/.bss). prog_tests/signed_loader.c
// uses this to check that a signed loader ignores ctx-supplied max_entries:
// the map must keep its attested size (4), not whatever the host puts in
// the loader ctx.
//
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 4);
    __type(key, __u32);
    __type(value, __u64);
    } amap SEC(".maps");
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut c_void) -> c_int {
    int probe(void *ctx)
    {
    let mut key: __u32 = 0;
    __u64 *val = bpf_map_lookup_elem(&amap, &key);
    return val ? (int)*val : 0;
    }
    char _license[] SEC("license") = "GPL";
