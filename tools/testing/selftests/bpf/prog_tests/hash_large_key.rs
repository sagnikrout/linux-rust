//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/hash_large_key.c
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
pub unsafe extern "C" fn test_hash_large_key() {
    void test_hash_large_key(void)
    {
    int err, value = 21, duration = 0, hash_map_fd;
    struct test_hash_large_key *skel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bigelement {
    pub a: c_int,
    pub b: [c_char; 4096],
    pub c: c_longlong,
    pub key: },
    pub sizeof(key)): bzero(&key,,
    pub test_hash_large_key__open_and_load(): skel =,
    if (CHECK(!skel, "skel_open_and_load", "skeleton open/load failed\n"))
    pub bpf_map__fd(skel->maps.hash_map): hash_map_fd =,
    if (CHECK(hash_map_fd < 0, "bpf_map__fd", "failed\n"))
    pub cleanup: goto,
    pub test_hash_large_key__attach(skel): err =,
    if (CHECK(err, "attach_raw_tp", "err %d\n", err))
    pub cleanup: goto,
    pub BPF_ANY): err = bpf_map_update_elem(hash_map_fd, &key, &value,,
    if (CHECK(err, "bpf_map_update_elem", "errno=%d\n", errno))
    pub cleanup: goto,
    pub 1: key.c =,
    pub &value): err = bpf_map_lookup_elem(hash_map_fd, &key,,
    if (CHECK(err, "bpf_map_lookup_elem", "errno=%d\n", errno))
    pub cleanup: goto,
    pub 42): CHECK_FAIL(value !=,
    cleanup:
    }
