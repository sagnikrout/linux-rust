//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/jeq_infer_not_null_fail.c
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
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, u64);
    __type(value, u64);
    } m_hash SEC(".maps");
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __msg('map_value_or_null": "R8 invalid mem access) -> __failure {
    __failure __msg("R8 invalid mem access 'map_value_or_null")
#[no_mangle]
pub unsafe extern "C" fn jeq_infer_not_null_ptr_to_btfid(ctx: *mut c_void) -> c_int {
    int jeq_infer_not_null_ptr_to_btfid(void *ctx)
    {
    struct bpf_map *map = (struct bpf_map *)&m_hash;
    struct bpf_map *inner_map = map.inner_map_meta;
    let mut key: u64 = 0, ret = 0, *val;
    val = bpf_map_lookup_elem(map, &key);
// Do not mark ptr as non-null if one of them is
// PTR_TO_BTF_ID (R9), reject because of invalid
// access to map value (R8).
//
// Here, we need to inline those insns to access
// R8 directly, since compiler may use other reg
// once it figures out val==inner_map.
//
    asm volatile("r8 = %[val];\n"
    "r9 = %[inner_map];\n"
    "if r8 != r9 goto +1;\n"
    "%[ret] = *(u64 *)(r8 +0);\n"
    : [ret] "+r"(ret)
    : [inner_map] "r"(inner_map), [val] "r"(val)
    : "r8", "r9");
    return ret;
    }
