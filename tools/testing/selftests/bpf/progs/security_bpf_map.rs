//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/security_bpf_map.c
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

    char _license[] SEC("license") = "GPL";

// From include/linux/mm.h.
pub const FMODE_WRITE: c_uint = 0x2;
    struct map;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, __u32);
    __type(value, __u32);
    __uint(max_entries, 1);
    } prot_status_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, __u32);
    __type(value, __u32);
    __uint(max_entries, 3);
    } prot_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, __u32);
    __type(value, __u32);
    __uint(max_entries, 3);
    } not_prot_map SEC(".maps");
    SEC("fmod_ret/security_bpf_map")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fmod_bpf_map, map: *mut bpf_map, fmode: c_int) -> c_int {
    int BPF_PROG(fmod_bpf_map, struct bpf_map *map, int fmode)
    {
    let mut key: __u32 = 0;
    __u32 *status_ptr = bpf_map_lookup_elem(&prot_status_map, &key);
    if (!status_ptr || !*status_ptr)
    return 0;
    if (map == &prot_map) {
// Allow read-only access
    if (fmode & FMODE_WRITE)
    return -EPERM;
    }
    return 0;
    }
//
// This program keeps references to maps. This is needed to prevent
// optimizing them out.
//
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fentry_dummy1, a: c_int) -> c_int {
    int BPF_PROG(fentry_dummy1, int a)
    {
    let mut key: __u32 = 0;
    let mut val1: __u32 = a;
    let mut val2: __u32 = a + 1;
    bpf_map_update_elem(&prot_map, &key, &val1, BPF_ANY);
    bpf_map_update_elem(&not_prot_map, &key, &val2, BPF_ANY);
    return 0;
    }
