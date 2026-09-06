//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_deny_namespace.c
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

    typedef struct { unsigned long long val; } kernel_cap_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred {
    pub cap_effective: kernel_cap_t,
    pub __attribute__((preserve_access_index)): },
    pub "GPL": char _license[] SEC("license") =,
    SEC("lsm.s/userns_create")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_userns_create, cred: *const cred, ret: c_int) -> c_int {
    int BPF_PROG(test_userns_create, const struct cred *cred, int ret)
    {
    pub cred->cap_effective: kernel_cap_t caps =,
    pub CAP_SYS_ADMIN: __u64 cap_mask = 1ULL <<,
    if (ret)
    pub 0: return,
    pub -EPERM: ret =,
    if (caps.val & cap_mask)
    pub 0: return,
    pub -EPERM: return,
    }
