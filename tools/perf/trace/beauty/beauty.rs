//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/trace/beauty/beauty.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strarray {
    pub offset: u64,
    pub nr_entries: c_int,
    pub prefix: *const c_char,
    pub entries: *const *const c_char,
}

extern "C" {
    pub fn strarray__scnprintf(sa: *mut strarray, bf: *mut c_char, size: usize, intfmt: *const c_char, show_prefix: bool, val: c_int) -> usize;
}
extern "C" {
    pub fn strarray__scnprintf_suffix(sa: *mut strarray, bf: *mut c_char, size: usize, intfmt: *const c_char, show_suffix: bool, val: c_int) -> usize;
}
extern "C" {
    pub fn strarray__scnprintf_flags(sa: *mut strarray, bf: *mut c_char, size: usize, show_prefix: bool, flags: c_ulong) -> usize;
}
extern "C" {
    pub fn strarray__strtoul(sa: *mut strarray, bf: *mut c_char, size: usize, ret: *mut u64) -> bool;
}
extern "C" {
    pub fn strarray__strtoul_flags(sa: *mut strarray, bf: *mut c_char, size: usize, ret: *mut u64) -> bool;
}
extern "C" {
    pub fn trace__show_zeros(trace: *const trace) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file {
    pub pathname: *mut c_char,
    pub dev_maj: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strarrays {
    pub nr_entries: c_int,
    pub entries: *mut strarray,
}

extern "C" {
    pub fn strarrays__scnprintf(sas: *mut strarrays, bf: *mut c_char, size: usize, intfmt: *const c_char, show_prefix: bool, val: c_int) -> usize;
}
extern "C" {
    pub fn strarrays__strtoul(sas: *mut strarrays, bf: *mut c_char, size: usize, ret: *mut u64) -> bool;
}
extern "C" {
    pub fn pid__scnprintf_fd(trace: *mut trace, pid: pid_t, fd: c_int, bf: *mut c_char, size: usize) -> usize;
}
//
// augmented_arg: extra payload for syscall pointer arguments
// If perf_sample->raw_size is more than what a syscall sys_enter_FOO puts, then
// its the arguments contents, so that we can show more than just a
// pointer. This will be done initially with eBPF, the start of that is at the
// tools/perf/util/bpf_skel/augmented_syscalls.bpf.c that will eventually be
// done automagically caching the running kernel tracefs events data into an
// eBPF C script, that then gets compiled and its .o file cached for subsequent
// use. For char pointers like the ones for 'open' like syscalls its easy, for
// the rest we should use DWARF or better, BTF, much more compact.
//
// @size: 8 if all we need is an integer, otherwise all of the augmented arg.
// @int_arg: will be used for integer like pointer contents, like 'accept's 'upeer_addrlen'
// @value: u64 aligned, for structs, pathnames
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct augmented_arg {
    pub size: c_int,
    pub int_arg: c_int,
    pub value: [u64; ],
}

//
// @val: value of syscall argument being formatted
// @len: for tracepoint dynamic arrays, if fmt->nr_entries == 0, then its not a fixed array, look at arg->len
// @args: All the args, use syscall_args__val(arg, nth) to access one
// @augmented_args: Extra data that can be collected, for instance, with eBPF for expanding the pathname for open, etc
// @augmented_args_size: augmented_args total payload size
// @thread: tid state (maps, pid, tid, etc)
// @trace: 'perf trace' internals: all threads, etc
// @parm: private area, may be an strarray, for instance
// @idx: syscall arg idx (is this the first?)
// @mask: a syscall arg may mask another arg, see syscall_arg__scnprintf_futex_op
// @show_string_prefix: When there is a common prefix in a string table, show it or not
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub args: *mut c_uchar,
    pub fmt: *mut syscall_arg_fmt,
    pub args: *mut augmented_arg,
    pub size: c_int,
    pub augmented: },
    pub thread: *mut thread,
    pub trace: *mut trace,
    pub parm: *mut c_void,
    pub type_name: *mut c_char,
    pub len: u16,
    pub idx: u8,
    pub mask: u8,
    pub show_string_prefix: bool,
}

extern "C" {
    pub fn syscall_arg__val(arg: *mut syscall_arg, idx: u8) -> c_ulong;
}
extern "C" {
    pub fn syscall_arg__scnprintf_strarray_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__strtoul_strarray(bf: *mut c_char, size: usize, arg: *mut syscall_arg, ret: *mut u64) -> bool;
}

extern "C" {
    pub fn syscall_arg__strtoul_strarray_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg, ret: *mut u64) -> bool;
}

extern "C" {
    pub fn syscall_arg__strtoul_strarrays(bf: *mut c_char, size: usize, arg: *mut syscall_arg, ret: *mut u64) -> bool;
}

extern "C" {
    pub fn syscall_arg__scnprintf_x86_irq_vectors(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__strtoul_x86_irq_vectors(bf: *mut c_char, size: usize, arg: *mut syscall_arg, ret: *mut u64) -> bool;
}

extern "C" {
    pub fn syscall_arg__scnprintf_x86_MSR(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__strtoul_x86_MSR(bf: *mut c_char, size: usize, arg: *mut syscall_arg, ret: *mut u64) -> bool;
}

extern "C" {
    pub fn syscall_arg__scnprintf_strarrays(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_fd(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_hex(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_ptr(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_int(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_long(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_pid(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_clone_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_fcntl_cmd(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_fcntl_arg(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_flock(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_fsmount_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_fsmount_attr_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_fspick_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_ioctl_cmd(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_kcmp_type(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_kcmp_idx(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__mask_val_mount_flags(arg: *mut syscall_arg, flags: c_ulong) -> c_ulong;
}

extern "C" {
    pub fn syscall_arg__scnprintf_mount_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_move_mount_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_pkey_alloc_access_rights(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_open_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_x86_arch_prctl_code(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_prctl_option(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_prctl_arg2(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_prctl_arg3(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_renameat2_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_sockaddr(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

// 'argname' is just documentational at this point, to remove the previous comment with that info

extern "C" {
    pub fn syscall_arg__scnprintf_socket_protocol(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_socket_level(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_fs_at_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_faccessat2_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_statx_mask(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_sync_file_range_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_timespec(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

// 'argname' is just documentational at this point, to remove the previous comment with that info

extern "C" {
    pub fn open__scnprintf_flags(flags: c_ulong, bf: *mut c_char, size: usize, show_prefix: bool) -> usize;
}
extern "C" {
    pub fn syscall_arg__scnprintf_eventfd_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_futex_op(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_futex_val3(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_mmap_prot(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_mmap_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_mremap_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_madvise_behavior(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_mode_t(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_msg_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_perf_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_perf_event_attr(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_sched_policy(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_seccomp_op(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_seccomp_flags(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_signum(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_socket_type(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

extern "C" {
    pub fn syscall_arg__scnprintf_waitid_options(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize;
}

