//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/compat.h
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
// These are the type definitions for the architecture specific
// syscall compatibility layer.
//

//
// It may be useful for an architecture to override the definitions of the
// COMPAT_SYSCALL_DEFINE0 and COMPAT_SYSCALL_DEFINEx() macros, in particular
// to use a different calling convention for syscalls. To allow for that,
// if CONFIG_ARCH_HAS_SYSCALL_WRAPPER is enabled.
//

pub const COMPAT_USE_64BIT_TIME: c_int = 0;

//
// The asmlinkage stub is aliased to a function named __se_compat_sys_*() which
// sign-extends 32-bit ints to longs whenever needed. The actual work is
// done within __do_compat_sys_*().
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_iovec {
    pub iov_base: compat_uptr_t,
    pub iov_len: compat_size_t,
}

pub type compat_uid_t = __compat_uid32_t;
pub type compat_gid_t = __compat_gid32_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_tms {
    pub tms_utime: compat_clock_t,
    pub tms_stime: compat_clock_t,
    pub tms_cutime: compat_clock_t,
    pub tms_cstime: compat_clock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_sigaction {
    pub sa_handler: compat_uptr_t,
    pub sa_flags: compat_ulong_t,

    pub sa_flags: compat_uint_t,
    pub sa_handler: compat_uptr_t,

    pub sa_restorer: compat_uptr_t,

    pub __packed: compat_sigset_t sa_mask,
}

// kill()
// POSIX.1b timers
// POSIX.1b signals
// SIGCHLD

// SIGCHLD (x32 version)

// SIGILL, SIGFPE, SIGSEGV, SIGBUS, SIGTRAP, SIGEMT

// used on alpha and sparc
//
// used when si_code=BUS_MCEERR_AR or
// used when si_code=BUS_MCEERR_AO
//
// used when si_code=SEGV_BNDERR
// used when si_code=SEGV_PKUERR
// used when si_code=TRAP_PERF
// SIGPOLL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_rlimit {
    pub rlim_cur: compat_ulong_t,
    pub rlim_max: compat_ulong_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_flock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: compat_off_t,
    pub l_len: compat_off_t,

    pub l_pid: compat_pid_t,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_flock64 {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: compat_loff_t,
    pub l_len: compat_loff_t,
    pub l_pid: compat_pid_t,

    pub __ARCH_COMPAT_FLOCK64_PACK: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_rusage {
    pub ru_utime: old_timeval32,
    pub ru_stime: old_timeval32,
    pub ru_maxrss: compat_long_t,
    pub ru_ixrss: compat_long_t,
    pub ru_idrss: compat_long_t,
    pub ru_isrss: compat_long_t,
    pub ru_minflt: compat_long_t,
    pub ru_majflt: compat_long_t,
    pub ru_nswap: compat_long_t,
    pub ru_inblock: compat_long_t,
    pub ru_oublock: compat_long_t,
    pub ru_msgsnd: compat_long_t,
    pub ru_msgrcv: compat_long_t,
    pub ru_nsignals: compat_long_t,
    pub ru_nvcsw: compat_long_t,
    pub ru_nivcsw: compat_long_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_dirent {
    pub d_ino: u32,
    pub d_off: compat_off_t,
    pub d_reclen: u16,
    pub d_name: [c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ustat {
    pub f_tfree: compat_daddr_t,
    pub f_tinode: compat_ino_t,
    pub f_fname: [c_char; 6],
    pub f_fpack: [c_char; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ifmap {
    pub mem_start: compat_ulong_t,
    pub mem_end: compat_ulong_t,
    pub base_addr: c_ushort,
    pub irq: c_uchar,
    pub dma: c_uchar,
    pub port: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_if_settings {
    pub /: *mut *mut unsigned int type; / Type of physical device or protocol,
    pub /: *mut *mut unsigned int size; / Size of the data allocated by the caller,
    pub /: *mut *mut compat_uptr_t ifs_ifsu; / union of pointers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ifreq {
    pub /: *mut *mut char ifrn_name[IFNAMSIZ]; / if name, e.g. "en0",
    pub ifr_ifrn: },
    pub ifru_addr: sockaddr,
    pub ifru_dstaddr: sockaddr,
    pub ifru_broadaddr: sockaddr,
    pub ifru_netmask: sockaddr,
    pub ifru_hwaddr: sockaddr,
    pub ifru_flags: c_short,
    pub ifru_ivalue: compat_int_t,
    pub ifru_mtu: compat_int_t,
    pub ifru_map: compat_ifmap,
    pub /: *mut *mut char ifru_slave[IFNAMSIZ]; / Just fits the size,
    pub ifru_newname: [c_char; IFNAMSIZ],
    pub ifru_data: compat_caddr_t,
    pub ifru_settings: compat_if_settings,
    pub ifr_ifru: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ifconf {
    pub /: *mut *mut compat_int_t ifc_len; / size of buffer,
    pub ifcbuf: compat_caddr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_robust_list {
    pub next: compat_uptr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_robust_list_head {
    pub list: compat_robust_list,
    pub futex_offset: compat_long_t,
    pub list_op_pending: compat_uptr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_old_sigaction {
    pub sa_handler: compat_uptr_t,
    pub sa_mask: compat_old_sigset_t,
    pub sa_flags: compat_ulong_t,
    pub sa_restorer: compat_uptr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_keyctl_kdf_params {
    pub hashname: compat_uptr_t,
    pub otherinfo: compat_uptr_t,
    pub otherinfolen: __u32,
    pub __spare: [__u32; 8],
}

extern "C" {
    pub fn get_compat_sigset(set: *mut sigset_t, compat: *const compat_sigset_t __user) -> c_int;
}
//
// Defined inline such that size can be compile time constant, which avoids
// CONFIG_HARDENED_USERCOPY complaining about copies from task_struct
//
// size <= sizeof(compat_sigset_t) <= sizeof(sigset_t)

extern "C" {
    pub fn compat_restore_altstack(uss: *const compat_stack_t __user) -> c_int;
}
extern "C" {
    pub fn __compat_save_altstack(: *mut compat_stack_t __user, long: unsigned) -> c_int;
}

//
// These syscall function prototypes are kept in the same order as
// include/uapi/asm-generic/unistd.h. Deprecated or obsolete system calls
// go below.
//
// Please note that these prototypes here are only provided for information
// purposes, for static analysis, and for linking from the syscall table.
// These functions should not be called elsewhere from kernel code.
//
// As the syscall calling convention may be different from the default
// for architectures overriding the syscall calling convention, do not
// include the prototypes if CONFIG_ARCH_HAS_SYSCALL_WRAPPER is enabled.
//

extern "C" {
    pub fn compat_sys_io_setup(nr_reqs: unsigned, ctx32p: *mut u32 __user) -> asmlinkage long;
}
extern "C" {
    pub fn compat_sys_truncate(: *const char __user, _arg: compat_off_t) -> asmlinkage long;
}
extern "C" {
    pub fn compat_sys_ftruncate(int: unsigned, _arg: compat_off_t) -> asmlinkage long;
}
// No generic prototype for truncate64, ftruncate64, fallocate
extern "C" {
    pub fn compat_sys_lseek(int: unsigned, _arg: compat_off_t, int: unsigned) -> asmlinkage long;
}
// No generic prototype for pread64 and pwrite64

// No generic prototype for sync_file_range and sync_file_range2

// No generic prototype for rt_sigreturn
extern "C" {
    pub fn compat_sys_times(tbuf: *mut compat_tms __user) -> asmlinkage long;
}
extern "C" {
    pub fn compat_sys_getrusage(who: c_int, ru: *mut compat_rusage __user) -> asmlinkage long;
}
extern "C" {
    pub fn compat_sys_sysinfo(info: *mut compat_sysinfo __user) -> asmlinkage long;
}
extern "C" {
    pub fn compat_sys_msgctl(first: c_int, second: c_int, uptr: *mut void __user) -> asmlinkage long;
}
extern "C" {
    pub fn compat_sys_semctl(semid: c_int, semnum: c_int, cmd: c_int, arg: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn compat_sys_shmctl(first: c_int, second: c_int, uptr: *mut void __user) -> asmlinkage long;
}
extern "C" {
    pub fn compat_sys_shmat(shmid: c_int, shmaddr: compat_uptr_t, shmflg: c_int) -> asmlinkage long;
}
// No generic prototype for readahead
// No generic prototype for fadvise64_64
// CONFIG_MMU only

//
// Deprecated system calls which are still defined in
// include/uapi/asm-generic/unistd.h and wanted by >= 1 arch
//
// __ARCH_WANT_SYSCALL_NO_AT
// __ARCH_WANT_SYSCALL_NO_FLAGS
// __ARCH_WANT_SYSCALL_OFF_T
// __ARCH_WANT_SYSCALL_DEPRECATED
extern "C" {
    pub fn compat_sys_ustat(dev: unsigned, u32: *mut compat_ustat __user) -> asmlinkage long;
}
// obsolete
extern "C" {
    pub fn compat_sys_old_select(arg: *mut compat_sel_arg___user) -> asmlinkage long;
}
// obsolete
extern "C" {
    pub fn compat_sys_ipc(_arg: u32, _arg: c_int, _arg: c_int, _arg: u32, _arg: compat_uptr_t, _arg: u32) -> asmlinkage long;
}
// obsolete

extern "C" {
    pub fn compat_sys_sigpending(set: *mut compat_old_sigset_t __user) -> asmlinkage long;
}

// obsolete
extern "C" {
    pub fn compat_sys_socketcall(call: c_int, args: *mut u32 __user) -> asmlinkage long;
}

extern "C" {
    pub fn compat_sys_truncate64(pathname: *const char __user, _arg: compat_arg_u64(len)) -> asmlinkage long;
}

extern "C" {
    pub fn compat_sys_ftruncate64(fd: c_uint, _arg: compat_arg_u64(len)) -> asmlinkage long;
}

extern "C" {
    pub fn compat_sys_readahead(fd: c_int, _arg: compat_arg_u64(offset), count: usize) -> asmlinkage long;
}

//
// ns_to_old_timeval32 - Compat version of ns_to_timeval
// @nsec:	the nanoseconds value to be converted
//
// Returns the old_timeval32 representation of the nsec parameter.
//
// Kernel code should not call compat syscalls (i.e., compat_sys_xyzyyz())
// directly.  Instead, use one of the functions which work equivalently, such
// as the kcompat_sys_xyzyyz() functions prototyped below.
//

//
// For most but not all architectures, "am I in a compat syscall?" and
// "am I a compat task?" are the same question.  For architectures on which
// they aren't the same question, arch code can override in_compat_syscall.
//

// Ensure no one redefines in_compat_syscall() under !CONFIG_COMPAT

//
// Some legacy ABIs like the i386 one use less than natural alignment for 64-bit
// types, and will need special compat treatment for that.  Most architectures
// don't need that special handling even for compat syscalls.
//

//
// A pointer passed in from user mode. This should not
// be used for syscall parameters, just declare them
// as pointers because the syscall entry code will have
// appropriately converted them already.
//

