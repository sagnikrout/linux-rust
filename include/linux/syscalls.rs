//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/syscalls.h
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
// syscalls.h - Linux syscall interfaces (non-arch-specific)
//
// Copyright (c) 2004 Randy Dunlap
// Copyright (c) 2004 Open Source Development Labs
//

//
// It may be useful for an architecture to override the definitions of the
// SYSCALL_DEFINE0() and __SYSCALL_DEFINEx() macros, in particular to use a
// different calling convention for syscalls. To allow for that, the prototypes
// for the sys_*() functions below will *not* be included if
// CONFIG_ARCH_HAS_SYSCALL_WRAPPER is enabled.
//

//
// __MAP - apply a macro to syscall arguments
// __MAP(n, m, t1, a1, t2, a2, ..., tn, an) will expand to
// m(t1, a1), m(t2, a2), ..., m(tn, an)
// The first argument must be equal to the amount of type/name
// pairs given.  Note that this list of pairs (i.e. the arguments
// of __MAP starting at the third one) is in the same format as
// for SYSCALL_DEFINE<n>/COMPAT_SYSCALL_DEFINE<n>
//
// Macro flag: #define __MAP0(m,...)

// __event_enter_##sname = &event_enter_##sname;

// __event_exit_##sname = &event_exit_##sname;

// __p_syscall_meta_##sname = &__syscall_meta_##sname;

pub const SYSCALL_DEFINE_MAXARGS: c_int = 6;

//
// The asmlinkage stub is aliased to a function named __se_sys_*() which
// sign-extends 32-bit ints to longs whenever needed. The actual work is
// done within __do_sys_*().
//

// For split 64-bit arguments on 32-bit architectures

//
// These syscall function prototypes are kept in the same order as
// include/uapi/asm-generic/unistd.h. Architecture specific entries go below,
// followed by deprecated or obsolete system calls.
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
    pub fn sys_io_setup(nr_reqs: unsigned, ctx: *mut aio_context_t __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_io_destroy(ctx: aio_context_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_flistxattr(fd: c_int, list: *mut char __user, size: usize) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fremovexattr(fd: c_int, name: *const char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getcwd(buf: *mut char __user, size: c_ulong) -> asmlinkage long;
}
extern "C" {
    pub fn sys_eventfd2(count: c_uint, flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_epoll_create1(flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_dup(fildes: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_dup3(oldfd: c_uint, newfd: c_uint, flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fcntl(fd: c_uint, cmd: c_uint, arg: c_ulong) -> asmlinkage long;
}

extern "C" {
    pub fn sys_inotify_init1(flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_inotify_rm_watch(fd: c_int, wd: __s32) -> asmlinkage long;
}
extern "C" {
    pub fn sys_ioprio_set(which: c_int, who: c_int, ioprio: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_ioprio_get(which: c_int, who: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_flock(fd: c_uint, cmd: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mkdirat(dfd: c_int, pathname: *const *const char __user, mode: umode_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_unlinkat(dfd: c_int, pathname: *const *const char __user, flag: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_umount(name: *mut char __user, flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fstatfs(fd: c_uint, buf: *mut statfs __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_truncate(path: *const char __user, length: c_long) -> asmlinkage long;
}
extern "C" {
    pub fn sys_ftruncate(fd: c_uint, length: off_t) -> asmlinkage long;
}

extern "C" {
    pub fn sys_truncate64(path: *const char __user, length: loff_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_ftruncate64(fd: c_uint, length: loff_t) -> asmlinkage long;
}

extern "C" {
    pub fn sys_fallocate(fd: c_int, mode: c_int, offset: loff_t, len: loff_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_faccessat(dfd: c_int, filename: *const char __user, mode: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_chdir(filename: *const char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fchdir(fd: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_chroot(filename: *const char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fchroot(fd: c_int, flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fchmod(fd: c_uint, mode: umode_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fchown(fd: c_uint, user: uid_t, group: gid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_close(fd: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_vhangup() -> asmlinkage long;
}
extern "C" {
    pub fn sys_pipe2(fildes: *mut int __user, flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_read(fd: c_uint, buf: *mut char __user, count: usize) -> asmlinkage long;
}
extern "C" {
    pub fn sys_signalfd4(ufd: c_int, user_mask: *mut sigset_t __user, sizemask: usize, flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_tee(fdin: c_int, fdout: c_int, len: usize, flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_newfstat(fd: c_uint, statbuf: *mut stat __user) -> asmlinkage long;
}

extern "C" {
    pub fn sys_fstat64(fd: c_ulong, statbuf: *mut stat64 __user) -> asmlinkage long;
}

extern "C" {
    pub fn sys_sync() -> asmlinkage long;
}
extern "C" {
    pub fn sys_fsync(fd: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fdatasync(fd: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_timerfd_create(clockid: c_int, flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_timerfd_gettime(ufd: c_int, otmr: *mut __kernel_itimerspec __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_acct(name: *const char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_personality(personality: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_exit(error_code: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_exit_group(error_code: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_set_tid_address(tidptr: *mut int __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_unshare(unshare_flags: c_ulong) -> asmlinkage long;
}
extern "C" {
    pub fn sys_futex_wake(uaddr: *mut void __user, mask: c_ulong, nr: c_int, flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getitimer(which: c_int, value: *mut __kernel_old_itimerval __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_timer_getoverrun(timer_id: timer_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_timer_delete(timer_id: timer_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_syslog(type: c_int, buf: *mut char __user, len: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_sched_getscheduler(pid: pid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_sched_yield() -> asmlinkage long;
}
extern "C" {
    pub fn sys_sched_get_priority_max(policy: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_sched_get_priority_min(policy: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_restart_syscall() -> asmlinkage long;
}
extern "C" {
    pub fn sys_kill(pid: pid_t, sig: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_tkill(pid: pid_t, sig: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_tgkill(tgid: pid_t, pid: pid_t, sig: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_rt_sigsuspend(unewset: *mut sigset_t __user, sigsetsize: usize) -> asmlinkage long;
}

extern "C" {
    pub fn sys_rt_sigpending(set: *mut sigset_t __user, sigsetsize: usize) -> asmlinkage long;
}
extern "C" {
    pub fn sys_rt_sigqueueinfo(pid: pid_t, sig: c_int, uinfo: *mut siginfo_t __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setpriority(which: c_int, who: c_int, niceval: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getpriority(which: c_int, who: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setregid(rgid: gid_t, egid: gid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setgid(gid: gid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setreuid(ruid: uid_t, euid: uid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setuid(uid: uid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getresuid(ruid: *mut uid_t __user, euid: *mut uid_t __user, suid: *mut uid_t __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getresgid(rgid: *mut gid_t __user, egid: *mut gid_t __user, sgid: *mut gid_t __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setfsuid(uid: uid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setfsgid(gid: gid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_times(tbuf: *mut tms __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setpgid(pid: pid_t, pgid: pid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getpgid(pid: pid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getsid(pid: pid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setsid() -> asmlinkage long;
}
extern "C" {
    pub fn sys_getgroups(gidsetsize: c_int, grouplist: *mut gid_t __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setgroups(gidsetsize: c_int, grouplist: *mut gid_t __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_newuname(name: *mut new_utsname __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_sethostname(name: *mut char __user, len: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setdomainname(name: *mut char __user, len: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getrusage(who: c_int, ru: *mut rusage __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_umask(mask: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getcpu(cpu: *mut unsigned __user, node: *mut unsigned __user, cache: *mut void __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_adjtimex(txc_p: *mut __kernel_timex __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_adjtimex_time32(txc_p: *mut old_timex32 __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getpid() -> asmlinkage long;
}
extern "C" {
    pub fn sys_getppid() -> asmlinkage long;
}
extern "C" {
    pub fn sys_getuid() -> asmlinkage long;
}
extern "C" {
    pub fn sys_geteuid() -> asmlinkage long;
}
extern "C" {
    pub fn sys_getgid() -> asmlinkage long;
}
extern "C" {
    pub fn sys_getegid() -> asmlinkage long;
}
extern "C" {
    pub fn sys_gettid() -> asmlinkage long;
}
extern "C" {
    pub fn sys_sysinfo(info: *mut sysinfo __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mq_open(name: *const char __user, oflag: c_int, mode: umode_t, attr: *mut mq_attr __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mq_unlink(name: *const char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mq_timedsend(mqdes: mqd_t, msg_ptr: *const char __user, msg_len: usize, msg_prio: c_uint, abs_timeout: *const __kernel_timespec __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mq_timedreceive(mqdes: mqd_t, msg_ptr: *mut char __user, msg_len: usize, msg_prio: *mut unsigned int __user, abs_timeout: *const __kernel_timespec __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mq_notify(mqdes: mqd_t, notification: *const sigevent __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mq_getsetattr(mqdes: mqd_t, mqstat: *const mq_attr __user, omqstat: *mut mq_attr __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_msgget(key: key_t, msgflg: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_old_msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_semget(key: key_t, nsems: c_int, semflg: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_semctl(semid: c_int, semnum: c_int, cmd: c_int, arg: c_ulong) -> asmlinkage long;
}
extern "C" {
    pub fn sys_old_semctl(semid: c_int, semnum: c_int, cmd: c_int, arg: c_ulong) -> asmlinkage long;
}
extern "C" {
    pub fn sys_shmget(key: key_t, size: usize, flag: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_old_shmctl(shmid: c_int, cmd: c_int, buf: *mut shmid_ds __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_shmctl(shmid: c_int, cmd: c_int, buf: *mut shmid_ds __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_shmat(shmid: c_int, shmaddr: *mut char __user, shmflg: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_shmdt(shmaddr: *mut char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_socket(_arg: c_int, _arg: c_int, _arg: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_socketpair(_arg: c_int, _arg: c_int, _arg: c_int, : *mut int __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_bind(_arg: c_int, : *mut sockaddr __user, _arg: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_listen(_arg: c_int, _arg: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_accept(_arg: c_int, : *mut sockaddr __user, : *mut int __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_connect(_arg: c_int, : *mut sockaddr __user, _arg: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getsockname(_arg: c_int, : *mut sockaddr __user, : *mut int __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getpeername(_arg: c_int, : *mut sockaddr __user, : *mut int __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_shutdown(_arg: c_int, _arg: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_sendmsg(fd: c_int, msg: *mut user_msghdr __user, flags: unsigned) -> asmlinkage long;
}
extern "C" {
    pub fn sys_recvmsg(fd: c_int, msg: *mut user_msghdr __user, flags: unsigned) -> asmlinkage long;
}
extern "C" {
    pub fn sys_readahead(fd: c_int, offset: loff_t, count: usize) -> asmlinkage long;
}
extern "C" {
    pub fn sys_brk(brk: c_ulong) -> asmlinkage long;
}
extern "C" {
    pub fn sys_munmap(addr: c_ulong, len: usize) -> asmlinkage long;
}

extern "C" {
    pub fn sys_clone3(uargs: *mut clone_args __user, size: usize) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fadvise64_64(fd: c_int, offset: loff_t, len: loff_t, advice: c_int) -> asmlinkage long;
}
// CONFIG_MMU only
extern "C" {
    pub fn sys_swapon(specialfile: *const char __user, swap_flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_swapoff(specialfile: *const char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_msync(start: c_ulong, len: usize, flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mlock(start: c_ulong, len: usize) -> asmlinkage long;
}
extern "C" {
    pub fn sys_munlock(start: c_ulong, len: usize) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mlockall(flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_munlockall() -> asmlinkage long;
}
extern "C" {
    pub fn sys_madvise(start: c_ulong, len: usize, behavior: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_process_mrelease(pidfd: c_int, flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mseal(start: c_ulong, len: usize, flags: c_ulong) -> asmlinkage long;
}
extern "C" {
    pub fn sys_accept4(_arg: c_int, : *mut sockaddr __user, : *mut int __user, _arg: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fanotify_init(flags: c_uint, event_f_flags: c_uint) -> asmlinkage long;
}

extern "C" {
    pub fn sys_syncfs(fd: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setns(fd: c_int, nstype: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_pidfd_open(pid: pid_t, flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_finit_module(fd: c_int, uargs: *const char __user, flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_memfd_create(uname_ptr: *const char __user, flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_userfaultfd(flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_membarrier(cmd: c_int, flags: c_uint, cpu_id: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mlock2(start: c_ulong, len: usize, flags: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_pkey_alloc(flags: c_ulong, init_val: c_ulong) -> asmlinkage long;
}
extern "C" {
    pub fn sys_pkey_free(pkey: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_rseq_slice_yield() -> asmlinkage long;
}
extern "C" {
    pub fn sys_open_tree(dfd: c_int, path: *const char __user, flags: unsigned) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fsopen(fs_name: *const char __user, flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fsmount(fs_fd: c_int, flags: c_uint, ms_flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fspick(dfd: c_int, path: *const char __user, flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_pidfd_getfd(pidfd: c_int, fd: c_int, flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_landlock_restrict_self(ruleset_fd: c_int, flags: __u32) -> asmlinkage long;
}
extern "C" {
    pub fn sys_memfd_secret(flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_map_shadow_stack(addr: c_ulong, size: c_ulong, flags: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_lsm_list_modules(ids: *mut u64 __user, size: *mut u32 __user, flags: u32) -> asmlinkage long;
}
//
// Architecture-specific system calls
//
// x86
extern "C" {
    pub fn sys_ioperm(from: c_ulong, num: c_ulong, on: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_uretprobe() -> asmlinkage long;
}
extern "C" {
    pub fn sys_uprobe() -> asmlinkage long;
}
// pciconfig: alpha, arm, arm64, ia64, sparc
extern "C" {
    pub fn sys_pciconfig_iobase(which: c_long, bus: c_ulong, devfn: c_ulong) -> asmlinkage long;
}
// powerpc
//
// Deprecated system calls which are still defined in
// include/uapi/asm-generic/unistd.h and wanted by >= 1 arch
//
// __ARCH_WANT_SYSCALL_NO_AT
extern "C" {
    pub fn sys_unlink(pathname: *const char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_chmod(filename: *const char __user, mode: umode_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_mkdir(pathname: *const char __user, mode: umode_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_rmdir(pathname: *const char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_access(filename: *const char __user, mode: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_symlink(old: *const char __user, new: *const char __user) -> asmlinkage long;
}

// __ARCH_WANT_SYSCALL_NO_FLAGS
extern "C" {
    pub fn sys_pipe(fildes: *mut int __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_dup2(oldfd: c_uint, newfd: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_epoll_create(size: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_inotify_init() -> asmlinkage long;
}
extern "C" {
    pub fn sys_eventfd(count: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_signalfd(ufd: c_int, user_mask: *mut sigset_t __user, sizemask: usize) -> asmlinkage long;
}
// __ARCH_WANT_SYSCALL_OFF_T
extern "C" {
    pub fn sys_fadvise64(fd: c_int, offset: loff_t, len: usize, advice: c_int) -> asmlinkage long;
}
// __ARCH_WANT_SYSCALL_DEPRECATED
extern "C" {
    pub fn sys_alarm(seconds: c_uint) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getpgrp() -> asmlinkage long;
}
extern "C" {
    pub fn sys_pause() -> asmlinkage long;
}
extern "C" {
    pub fn sys_time(tloc: *mut __kernel_old_time_t __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_time32(tloc: *mut old_time32_t __user) -> asmlinkage long;
}

extern "C" {
    pub fn sys_creat(pathname: *const char __user, mode: umode_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_ustat(dev: unsigned, ubuf: *mut ustat __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_vfork() -> asmlinkage long;
}
extern "C" {
    pub fn sys_recv(_arg: c_int, : *mut void __user, _arg: usize, _arg: unsigned) -> asmlinkage long;
}
extern "C" {
    pub fn sys_send(_arg: c_int, : *mut void __user, _arg: usize, _arg: unsigned) -> asmlinkage long;
}
extern "C" {
    pub fn sys_oldumount(name: *mut char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_uselib(library: *const char __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_fork() -> asmlinkage long;
}
// obsolete
extern "C" {
    pub fn sys_stime(tptr: *mut __kernel_old_time_t __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_stime32(tptr: *mut old_time32_t __user) -> asmlinkage long;
}
// obsolete
extern "C" {
    pub fn sys_sigpending(uset: *mut old_sigset_t __user) -> asmlinkage long;
}

extern "C" {
    pub fn sys_sigsuspend(mask: old_sigset_t) -> asmlinkage long;
}

extern "C" {
    pub fn sys_sigsuspend(unused1: c_int, unused2: c_int, mask: old_sigset_t) -> asmlinkage long;
}

extern "C" {
    pub fn sys_sgetmask() -> asmlinkage long;
}
extern "C" {
    pub fn sys_ssetmask(newmask: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_signal(sig: c_int, handler: __sighandler_t) -> asmlinkage long;
}
// obsolete
extern "C" {
    pub fn sys_nice(increment: c_int) -> asmlinkage long;
}
// obsolete
extern "C" {
    pub fn sys_waitpid(pid: pid_t, stat_addr: *mut int __user, options: c_int) -> asmlinkage long;
}
// obsolete

extern "C" {
    pub fn sys_fchown16(fd: c_uint, user: old_uid_t, group: old_gid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setregid16(rgid: old_gid_t, egid: old_gid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setgid16(gid: old_gid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setreuid16(ruid: old_uid_t, euid: old_uid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setuid16(uid: old_uid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setresuid16(ruid: old_uid_t, euid: old_uid_t, suid: old_uid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setresgid16(rgid: old_gid_t, egid: old_gid_t, sgid: old_gid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setfsuid16(uid: old_uid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setfsgid16(gid: old_gid_t) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getgroups16(gidsetsize: c_int, grouplist: *mut old_gid_t __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_setgroups16(gidsetsize: c_int, grouplist: *mut old_gid_t __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_getuid16() -> asmlinkage long;
}
extern "C" {
    pub fn sys_geteuid16() -> asmlinkage long;
}
extern "C" {
    pub fn sys_getgid16() -> asmlinkage long;
}
extern "C" {
    pub fn sys_getegid16() -> asmlinkage long;
}

// obsolete
extern "C" {
    pub fn sys_socketcall(call: c_int, args: *mut unsigned long __user) -> asmlinkage long;
}
// obsolete
extern "C" {
    pub fn sys_old_select(arg: *mut sel_arg___user) -> asmlinkage long;
}
// obsolete
extern "C" {
    pub fn sys_old_readdir(int: unsigned, : *mut old_linux_dirent __user, int: unsigned) -> asmlinkage long;
}
// obsolete
extern "C" {
    pub fn sys_gethostname(name: *mut char __user, len: c_int) -> asmlinkage long;
}
extern "C" {
    pub fn sys_uname(: *mut old_utsname __user) -> asmlinkage long;
}
extern "C" {
    pub fn sys_olduname(: *mut oldold_utsname __user) -> asmlinkage long;
}

extern "C" {
    pub fn sys_old_getrlimit(resource: c_uint, rlim: *mut rlimit __user) -> asmlinkage long;
}

// obsolete
extern "C" {
    pub fn sys_old_mmap(arg: *mut mmap_arg___user) -> asmlinkage long;
}
//
// Not a real system call, but a placeholder for syscalls which are
// not implemented -- see kernel/sys_ni.c
//
extern "C" {
    pub fn sys_ni_syscall() -> asmlinkage long;
}

extern "C" {
    pub fn sys_ni_posix_timers() -> asmlinkage long;
}
//
// Kernel code should not call syscalls (i.e., sys_xyzyyz()) directly.
// Instead, use one of the functions which work equivalently, such as
// the ksys_xyzyyz() functions prototyped below.
//
extern "C" {
    pub fn ksys_write(fd: c_uint, buf: *const char __user, count: usize) -> isize;
}
extern "C" {
    pub fn ksys_fchown(fd: c_uint, user: uid_t, group: gid_t) -> c_int;
}
extern "C" {
    pub fn ksys_read(fd: c_uint, buf: *mut char __user, count: usize) -> isize;
}
extern "C" {
    pub fn ksys_sync();
}
extern "C" {
    pub fn ksys_unshare(unshare_flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn ksys_setsid() -> c_int;
}
extern "C" {
    pub fn ksys_fallocate(fd: c_int, mode: c_int, offset: loff_t, len: loff_t) -> c_int;
}

extern "C" {
    pub fn ksys_fadvise64_64(fd: c_int, offset: loff_t, len: loff_t, advice: c_int) -> c_int;
}

extern "C" {
    pub fn ksys_readahead(fd: c_int, offset: loff_t, count: usize) -> isize;
}
//
// The following kernel syscall equivalents are just wrappers to fs-internal
// functions. Therefore, provide stubs to be inlined at the callsites.
//
extern "C" {
    pub fn do_fchownat(_arg: AT_FDCWD, _arg: filename, _arg: user, _arg: group, _arg: 0) -> return;
}

extern "C" {
    pub fn ksys_ftruncate(fd: c_uint, length: loff_t, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn ksys_truncate(pathname: *const char __user, length: loff_t) -> c_int;
}
// for __ARCH_WANT_SYS_IPC
extern "C" {
    pub fn ksys_semget(key: key_t, nsems: c_int, semflg: c_int) -> c_long;
}
extern "C" {
    pub fn ksys_old_semctl(semid: c_int, semnum: c_int, cmd: c_int, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn ksys_msgget(key: key_t, msgflg: c_int) -> c_long;
}
extern "C" {
    pub fn ksys_old_msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds __user) -> c_long;
}
extern "C" {
    pub fn ksys_shmget(key: key_t, size: usize, shmflg: c_int) -> c_long;
}
extern "C" {
    pub fn ksys_shmdt(shmaddr: *mut char __user) -> c_long;
}
extern "C" {
    pub fn ksys_old_shmctl(shmid: c_int, cmd: c_int, buf: *mut shmid_ds __user) -> c_long;
}
