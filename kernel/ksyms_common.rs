//! Automatically rewritten from C to Rust
//! Source: kernel/ksyms_common.c
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
// ksyms_common.c: A split of kernel/kallsyms.c
// Contains a few generic function definations independent of config KALLSYMS.
//

#[no_mangle]
pub unsafe extern "C" fn kallsyms_for_perf() -> c_int {
    static inline int kallsyms_for_perf(void)
    {

    extern int sysctl_perf_event_paranoid;
    if (sysctl_perf_event_paranoid <= 1)
    return 1;

    return 0;
    }
//
// We show kallsyms information even to normal users if we've enabled
// kernel profiling and are explicitly not paranoid (so kptr_restrict
// is clear, and sysctl_perf_event_paranoid isn't set).
//
// Otherwise, require CAP_SYSLOG (assuming kptr_restrict isn't set to
// block even that).
//
#[no_mangle]
pub unsafe extern "C" fn kallsyms_show_value(cred: *const cred) -> bool {
    bool kallsyms_show_value(const struct cred *cred)
    {
    switch (kptr_restrict) {
    case 0:
    if (kallsyms_for_perf())
    return true;
    fallthrough;
    case 1:
    if (security_capable(cred, &init_user_ns, CAP_SYSLOG,
    CAP_OPT_NOAUDIT) == 0)
    return true;
    fallthrough;
    default:
    return false;
    }
    }
