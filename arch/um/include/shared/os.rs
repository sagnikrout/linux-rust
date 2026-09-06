//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/shared/os.h
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
// Copyright (C) 2015 Anton Ivanov (aivanov@{brocade.com,kot-begemot.co.uk})
// Copyright (C) 2015 Thomas Meyer (thomas@m3y3r.de)
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

// This is to get size_t

pub const OS_TYPE_FILE: c_int = 1;
pub const OS_TYPE_DIR: c_int = 2;
pub const OS_TYPE_SYMLINK: c_int = 3;
pub const OS_TYPE_CHARDEV: c_int = 4;
pub const OS_TYPE_BLOCKDEV: c_int = 5;
pub const OS_TYPE_FIFO: c_int = 6;
pub const OS_TYPE_SOCK: c_int = 7;
// os_access() flags

pub const OS_SENDMSG_MAX_FDS: c_int = 8;
//
// types taken from stat_file() in hostfs_user.c
// (if they are wrong here, they are wrong there...).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uml_stat {
    pub /: *mut *mut int ust_dev; / device,
    pub /: *mut *mut unsigned long long ust_ino; / inode,
    pub /: *mut *mut int ust_mode; / protection,
    pub /: *mut *mut int ust_nlink; / number of hard links,
    pub /: *mut *mut int ust_uid; / user ID of owner,
    pub /: *mut *mut int ust_gid; / group ID of owner,
    pub /: *mut *mut unsigned long long ust_size; / total size, in bytes,
    pub /: *mut *mut int ust_blksize; / blocksize for filesystem I/O,
    pub /: *mut *mut unsigned long long ust_blocks; / number of blocks allocated,
    pub /: *mut *mut unsigned long ust_atime; / time of last access,
    pub /: *mut *mut unsigned long ust_mtime; / time of last modification,
    pub /: *mut *mut unsigned long ust_ctime; / time of last change,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct openflags {
    pub 1: unsigned int r :,
    pub 1: unsigned int w :,
    pub /: *mut *mut unsigned int s : 1; / O_SYNC,
    pub /: *mut *mut unsigned int c : 1; / O_CREAT,
    pub /: *mut *mut unsigned int t : 1; / O_TRUNC,
    pub /: *mut *mut unsigned int a : 1; / O_APPEND,
    pub /: *mut *mut unsigned int e : 1; / O_EXCL,
    pub /: *mut *mut unsigned int cl : 1; / FD_CLOEXEC,
}

extern "C" {
    pub fn of_read(_arg: of_write(flags)) -> return;
}
// file.c
extern "C" {
    pub fn os_stat_file(file_name: *const c_char, buf: *mut uml_stat) -> c_int;
}
extern "C" {
    pub fn os_stat_fd(fd: c_int, buf: *mut uml_stat) -> c_int;
}
extern "C" {
    pub fn os_access(file: *const c_char, mode: c_int) -> c_int;
}
extern "C" {
    pub fn os_set_exec_close(fd: c_int) -> c_int;
}
extern "C" {
    pub fn os_ioctl_generic(fd: c_int, cmd: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn os_get_ifname(fd: c_int, namebuf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn os_mode_fd(fd: c_int, mode: c_int) -> c_int;
}
extern "C" {
    pub fn os_seek_file(fd: c_int, offset: c_ulonglong) -> c_int;
}
extern "C" {
    pub fn os_open_file(file: *const c_char, flags: openflags, mode: c_int) -> c_int;
}
extern "C" {
    pub fn os_read_file(fd: c_int, buf: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn os_write_file(fd: c_int, buf: *const c_void, count: c_int) -> c_int;
}
extern "C" {
    pub fn os_sync_file(fd: c_int) -> c_int;
}
extern "C" {
    pub fn os_file_size(file: *const c_char, size_out: *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn os_pread_file(fd: c_int, buf: *mut c_void, len: c_int, offset: c_ulonglong) -> c_int;
}
extern "C" {
    pub fn os_pwrite_file(fd: c_int, buf: *const c_void, count: c_int, offset: c_ulonglong) -> c_int;
}
extern "C" {
    pub fn os_file_modtime(file: *const c_char, modtime: *mut c_longlong) -> c_int;
}
extern "C" {
    pub fn os_pipe(fd: *mut c_int, stream: c_int, close_on_exec: c_int) -> c_int;
}
extern "C" {
    pub fn os_set_fd_async(fd: c_int) -> c_int;
}
extern "C" {
    pub fn os_clear_fd_async(fd: c_int) -> c_int;
}
extern "C" {
    pub fn os_set_fd_block(fd: c_int, blocking: c_int) -> c_int;
}
extern "C" {
    pub fn os_accept_connection(fd: c_int) -> c_int;
}
extern "C" {
    pub fn os_create_unix_socket(file: *const c_char, len: c_int, close_on_exec: c_int) -> c_int;
}
extern "C" {
    pub fn os_shutdown_socket(fd: c_int, r: c_int, w: c_int) -> c_int;
}
extern "C" {
    pub fn os_dup_file(fd: c_int) -> c_int;
}
extern "C" {
    pub fn os_close_file(fd: c_int);
}
extern "C" {
    pub fn os_connect_socket(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn os_file_type(file: *mut c_char) -> c_int;
}
extern "C" {
    pub fn os_file_mode(file: *const c_char, mode_out: *mut openflags) -> c_int;
}
extern "C" {
    pub fn os_lock_file(fd: c_int, excl: c_int) -> c_int;
}
extern "C" {
    pub fn os_flush_stdout();
}
extern "C" {
    pub fn os_major(dev: c_ulonglong) -> unsigned;
}
extern "C" {
    pub fn os_minor(dev: c_ulonglong) -> unsigned;
}
extern "C" {
    pub fn os_makedev(major: unsigned, minor: unsigned) -> c_ulonglong;
}
extern "C" {
    pub fn os_falloc_punch(fd: c_int, offset: c_ulonglong, count: c_int) -> c_int;
}
extern "C" {
    pub fn os_falloc_zeroes(fd: c_int, offset: c_ulonglong, count: c_int) -> c_int;
}
extern "C" {
    pub fn os_eventfd(initval: c_uint, flags: c_int) -> c_int;
}
extern "C" {
    pub fn os_poll(n: c_uint, fds: *const c_int) -> c_int;
}
// start_up.c
extern "C" {
    pub fn os_early_checks();
}
extern "C" {
    pub fn os_check_bugs();
}
extern "C" {
    pub fn check_host_supports_tls(supports_tls: *mut c_int, tls_min: *mut c_int);
}
// mem.c
extern "C" {
    pub fn create_mem_file(len: c_ulonglong) -> c_int;
}
// tlb.c
extern "C" {
    pub fn report_enomem();
}
// process.c
extern "C" {
    pub fn os_reap_child() -> pid_t;
}
extern "C" {
    pub fn os_alarm_process(pid: c_int);
}
extern "C" {
    pub fn os_kill_process(pid: c_int, reap_child: c_int);
}
extern "C" {
    pub fn os_kill_ptraced_process(pid: c_int, reap_child: c_int);
}
extern "C" {
    pub fn os_getpid() -> c_int;
}
extern "C" {
    pub fn init_new_thread_signals();
}
extern "C" {
    pub fn os_unmap_memory(addr: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn os_drop_memory(addr: *mut c_void, length: c_int) -> c_int;
}
extern "C" {
    pub fn can_drop_memory() -> c_int;
}
extern "C" {
    pub fn os_set_pdeathsig();
}
extern "C" {
    pub fn os_futex_wait(uaddr: *mut c_void, val: c_uint) -> c_int;
}
extern "C" {
    pub fn os_futex_wake(uaddr: *mut c_void) -> c_int;
}
// execvp.c
extern "C" {
    pub fn execvp_noalloc(buf: *mut c_char, file: *const c_char, argv[]: *const *const c_char) -> c_int;
}
// helper.c
extern "C" {
    pub fn run_helper(): *mut *mut void (pre_exec)(void, pre_data: *mut c_void, argv: *mut c_char) -> c_int;
}
extern "C" {
    pub fn helper_wait(pid: c_int) -> c_int;
}
extern "C" {
    pub fn os_kill_helper_thread(td: *mut os_helper_thread);
}
extern "C" {
    pub fn os_fix_helper_thread_signals();
}
// umid.c
extern "C" {
    pub fn umid_file_name(name: *mut c_char, buf: *mut c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn set_umid(name: *mut c_char) -> c_int;
}
// signal.c
extern "C" {
    pub fn timer_set_signal_handler();
}
extern "C" {
    pub fn set_sigstack(sig_stack: *mut c_void, size: c_int);
}
extern "C" {
    pub fn set_handler(sig: c_int);
}
extern "C" {
    pub fn send_sigio_to_self();
}
extern "C" {
    pub fn change_sig(signal: c_int, on: c_int) -> c_int;
}
extern "C" {
    pub fn block_signals();
}
extern "C" {
    pub fn unblock_signals();
}
extern "C" {
    pub fn um_get_signals() -> c_int;
}
extern "C" {
    pub fn um_set_signals(enable: c_int) -> c_int;
}
extern "C" {
    pub fn um_set_signals_trace(enable: c_int) -> c_int;
}
extern "C" {
    pub fn deliver_alarm();
}
extern "C" {
    pub fn register_pm_wake_signal();
}
extern "C" {
    pub fn block_signals_hard();
}
extern "C" {
    pub fn unblock_signals_hard();
}
extern "C" {
    pub fn mark_sigio_pending();
}
// util.c
extern "C" {
    pub fn stack_protections(address: c_ulong);
}
extern "C" {
    pub fn raw(fd: c_int) -> c_int;
}
extern "C" {
    pub fn setup_machinename(machine_out: *mut c_char);
}
extern "C" {
    pub fn setup_hostinfo(buf: *mut c_char, len: c_int);
}
extern "C" {
    pub fn os_getrandom(buf: *mut c_void, len: usize, flags: c_uint) -> isize;
}
extern "C" {
    pub fn os_dump_core(((noreturn): void) __attribute__);
}
extern "C" {
    pub fn um_early_printk(s: *const c_char, n: c_uint);
}
extern "C" {
    pub fn os_fix_helper_signals();
}
// time.c
extern "C" {
    pub fn os_idle_prepare();
}
extern "C" {
    pub fn os_idle_sleep();
}
extern "C" {
    pub fn os_timer_create() -> c_int;
}
extern "C" {
    pub fn os_timer_set_interval(cpu: c_int, nsecs: c_ulonglong) -> c_int;
}
extern "C" {
    pub fn os_timer_one_shot(cpu: c_int, nsecs: c_ulonglong) -> c_int;
}
extern "C" {
    pub fn os_timer_disable(cpu: c_int);
}
extern "C" {
    pub fn os_persistent_clock_emulation() -> c_longlong;
}
extern "C" {
    pub fn os_nsecs() -> c_longlong;
}
// skas/mem.c
extern "C" {
    pub fn syscall_stub_flush(mm_idp: *mut mm_id) -> c_int;
}
extern "C" {
    pub fn syscall_stub_dump_error(mm_idp: *mut mm_id);
}
extern "C" {
    pub fn unmap(mm_idp: *mut mm_id, addr: c_ulong, len: c_ulong) -> c_int;
}
// skas/process.c
extern "C" {
    pub fn is_skas_winch(pid: c_int, fd: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn start_userspace(mm_id: *mut mm_id) -> c_int;
}
extern "C" {
    pub fn userspace(regs: *mut uml_pt_regs);
}
extern "C" {
    pub fn new_thread(stack: *mut c_void, buf: *mut jmp_buf, (*handler)(void): *mut c_void);
}
extern "C" {
    pub fn switch_threads(me: *mut jmp_buf, you: *mut jmp_buf);
}
extern "C" {
    pub fn start_idle_thread(stack: *mut c_void, switch_buf: *mut jmp_buf) -> c_int;
}
extern "C" {
    pub fn halt_skas();
}
extern "C" {
    pub fn reboot_skas();
}
// irq.c
extern "C" {
    pub fn os_waiting_for_events_epoll() -> c_int;
}
extern "C" {
    pub fn os_epoll_triggered(index: c_int, events: c_int) -> c_int;
}
extern "C" {
    pub fn os_event_mask(irq_type: um_irq_type) -> c_int;
}
extern "C" {
    pub fn os_setup_epoll() -> c_int;
}
extern "C" {
    pub fn os_add_epoll_fd(events: c_int, fd: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn os_mod_epoll_fd(events: c_int, fd: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn os_del_epoll_fd(fd: c_int) -> c_int;
}
extern "C" {
    pub fn os_set_ioignore();
}
extern "C" {
    pub fn os_close_epoll_fd();
}
extern "C" {
    pub fn um_irqs_suspend();
}
extern "C" {
    pub fn um_irqs_resume();
}
// sigio.c
extern "C" {
    pub fn add_sigio_fd(fd: c_int) -> c_int;
}
extern "C" {
    pub fn ignore_sigio_fd(fd: c_int) -> c_int;
}
extern "C" {
    pub fn maybe_sigio_broken(fd: c_int);
}
extern "C" {
    pub fn sigio_broken();
}
//
// unlocked versions for IRQ controller code.
//
// This is safe because it's used at suspend/resume and nothing
// else is running.
//
extern "C" {
    pub fn __add_sigio_fd(fd: c_int) -> c_int;
}
extern "C" {
    pub fn __ignore_sigio_fd(fd: c_int) -> c_int;
}
// tty.c
extern "C" {
    pub fn get_pty() -> c_int;
}
extern "C" {
    pub fn syscall(number: c_long, ...) -> c_long;
}
// irqflags tracing
extern "C" {
    pub fn block_signals_trace();
}
extern "C" {
    pub fn unblock_signals_trace();
}
extern "C" {
    pub fn um_trace_signals_on();
}
extern "C" {
    pub fn um_trace_signals_off();
}
// time-travel
extern "C" {
    pub fn deliver_time_travel_irqs();
}
// smp.c

extern "C" {
    pub fn os_init_smp();
}
extern "C" {
    pub fn os_start_cpu_thread(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn os_start_secondary(arg: *mut c_void, switch_buf: *mut jmp_buf);
}
extern "C" {
    pub fn os_send_ipi(cpu: c_int, vector: c_int) -> c_int;
}
extern "C" {
    pub fn os_local_ipi_enable();
}
extern "C" {
    pub fn os_local_ipi_disable();
}

