//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/pid.c
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

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_pid(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_pid(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut pid: c_int = arg.val;
    struct trace *trace = arg.trace;
    let mut printed: usize = scnprintf(bf, size, "%d", pid);
    struct thread *thread = machine__findnew_thread(trace__host(trace), pid, pid);
    if (thread != core::ptr::null_mut()) {
    if (!thread__comm_set(thread))
    thread__set_comm_from_proc(thread);
    if (thread__comm_set(thread))
    printed += scnprintf(bf + printed, size - printed,
    " (%s)", thread__comm_str(thread));
    thread__put(thread);
    }
    return printed;
    }
