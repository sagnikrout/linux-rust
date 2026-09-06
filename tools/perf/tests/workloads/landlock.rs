//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/landlock.c
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

// This workload was initially added to test enum augmentation with BTF in perf
// trace because its the only syscall that has an enum argument. Since it is
// a recent addition to the Linux kernel (at the time of the introduction of this
// 'perf test' workload) we just add the required types and defines here instead
// of including linux/landlock, that isn't available in older systems.
//
// We are not interested in the result of the syscall, just in intercepting
// its arguments.
//

pub const __NR_landlock_add_rule: c_int = 445;

pub const LANDLOCK_RULE_PATH_BENEATH: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_path_beneath_attr {
    pub allowed_access: __u64,
    pub parent_fd: __s32,
}

pub const LANDLOCK_RULE_NET_PORT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_net_port_attr {
    pub allowed_access: __u64,
    pub port: __u64,
}

#[no_mangle]
unsafe extern "C" fn landlock(__maybe_unused: int argc, __maybe_unused: *const *const *const char argv) -> c_int {
    static int landlock(int argc __maybe_unused, const char **argv __maybe_unused)
    {
    let mut fd: c_int = 11, flags = 45;
    struct landlock_path_beneath_attr path_beneath_attr = {
    .allowed_access = LANDLOCK_ACCESS_FS_READ_FILE,
    .parent_fd = 14,
    };
    struct landlock_net_port_attr net_port_attr = {
    .port = 19,
    .allowed_access = LANDLOCK_ACCESS_NET_CONNECT_TCP,
    };
    syscall(__NR_landlock_add_rule, fd, LANDLOCK_RULE_PATH_BENEATH,
    &path_beneath_attr, flags);
    syscall(__NR_landlock_add_rule, fd, LANDLOCK_RULE_NET_PORT,
    &net_port_attr, flags);
    return 0;
    }
    DEFINE_WORKLOAD(landlock);
