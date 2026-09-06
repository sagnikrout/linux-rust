//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/perf_event_open.c
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


// SPDX-License-Identifier: LGPL-2.1

    size_t syscall_arg__scnprintf_perf_flags(char *bf, size_t size,
    struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "PERF_";
    let mut printed: c_int = 0, flags = arg.val;
    if (flags == 0)
    return 0;

    if (flags & PERF_FLAG_##n) { \
    printed += scnprintf(bf + printed, size - printed, "%s%s%s", printed ? "|" : "", show_prefix ? prefix : "", #n); \
    flags &= ~PERF_FLAG_##n; \
    }
    P_FLAG(FD_NO_GROUP);
    P_FLAG(FD_OUTPUT);
    P_FLAG(PID_CGROUP);
    P_FLAG(FD_CLOEXEC);

    if (flags)
    printed += scnprintf(bf + printed, size - printed, "%s%#x", printed ? "|" : "", flags);
    return printed;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct attr_fprintf_args {
    pub printed: size_t size,,
    pub bf: *mut c_char,
    pub first: bool,
}

#[no_mangle]
unsafe extern "C" fn attr__fprintf(__maybe_unused: *mut *mut FILE fp, name: *const c_char, val: *const c_char, priv: *mut c_void) -> c_int {
    static int attr__fprintf(FILE *fp __maybe_unused, const char *name, const char *val, void *priv)
    {
    struct attr_fprintf_args *args = priv;
    let mut printed: usize = scnprintf(args.bf + args.printed , args.size - args.printed, "%s%s: %s", args.first ? "" : ", ", name, val);
    args.first = false;
    args.printed += printed;
    return printed;
    }
#[no_mangle]
unsafe extern "C" fn perf_event_attr___scnprintf(attr: *mut perf_event_attr, bf: *mut c_char, size: usize, __maybe_unused: bool show_zeros) -> usize {
    static size_t perf_event_attr___scnprintf(struct perf_event_attr *attr, char *bf, size_t size, bool show_zeros __maybe_unused)
    {
    struct attr_fprintf_args args = {
    .printed = scnprintf(bf, size, "{ "),
    .size    = size,
    .first   = true,
    .bf	 = bf,
    };
    perf_event_attr__fprintf(stdout, attr, attr__fprintf, &args);
    return args.printed + scnprintf(bf + args.printed, size - args.printed, " }");
    }
#[no_mangle]
unsafe extern "C" fn syscall_arg__scnprintf_augmented_perf_event_attr(arg: *mut syscall_arg, bf: *mut c_char, size: usize) -> usize {
    static size_t syscall_arg__scnprintf_augmented_perf_event_attr(struct syscall_arg *arg, char *bf, size_t size)
    {
    struct perf_event_attr *attr = (void *)arg.augmented.args.value;
    struct perf_event_attr local_attr;
//
// augmented_raw_syscalls.bpf.c (shipped with perf) copies
// PERF_ATTR_SIZE_VER0 bytes when the tracee passes size=0,
// but leaves the size field as 0.  The payload size is
// guaranteed by perf's own BPF program, not externally
// controllable.  Copy to a local so we can fix up size
// without writing to the potentially read-only augmented
// args buffer.
//
    if (!attr.size) {
    memcpy(&local_attr, attr, PERF_ATTR_SIZE_VER0);
    memset((void *)&local_attr + PERF_ATTR_SIZE_VER0, 0,
    sizeof(local_attr) - PERF_ATTR_SIZE_VER0);
    local_attr.size = PERF_ATTR_SIZE_VER0;
    attr = &local_attr;
    }
    return perf_event_attr___scnprintf(attr, bf, size,
    trace__show_zeros(arg.trace));
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_perf_event_attr(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_perf_event_attr(char *bf, size_t size, struct syscall_arg *arg)
    {
    if (arg.augmented.args)
    return syscall_arg__scnprintf_augmented_perf_event_attr(arg, bf, size);
    return scnprintf(bf, size, "%#lx", arg.val);
    }
