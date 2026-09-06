//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/tty.h
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
// TTY core internal functions
//

//
// Lock subclasses for tty locks
//
// TTY_LOCK_NORMAL is for normal ttys and master ptys.
// TTY_LOCK_SLAVE is for slave ptys only.
//
// Lock subclasses are necessary for handling nested locking with pty pairs.
// tty locks which use nested locking:
//
// legacy_mutex - Nested tty locks are necessary for releasing pty pairs.
// The stable lock order is master pty first, then slave pty.
// termios_rwsem - The stable lock order is tty_buffer lock->termios_rwsem.
// Subclassing this lock enables the slave pty to hold its
// termios_rwsem when claiming the master tty_buffer lock.
// tty_buffer lock - slave ptys can claim nested buffer lock when handling
// signal chars. The stable lock order is slave pty, then
// master.
//
// Values for tty->flow_change
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tty_flow_change {
    TTY_FLOW_NO_CHANGE,
    TTY_THROTTLE_SAFE,
    TTY_UNTHROTTLE_SAFE,
}

extern "C" {
    pub fn tty_ldisc_lock(tty: *mut tty_struct, timeout: c_ulong) -> c_int;
}
extern "C" {
    pub fn tty_ldisc_unlock(tty: *mut tty_struct);
}
extern "C" {
    pub fn __tty_check_change(tty: *mut tty_struct, sig: c_int) -> c_int;
}
extern "C" {
    pub fn tty_check_change(tty: *mut tty_struct) -> c_int;
}
extern "C" {
    pub fn __stop_tty(tty: *mut tty_struct);
}
extern "C" {
    pub fn __start_tty(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_write_unlock(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_write_lock(tty: *mut tty_struct, ndelay: bool) -> c_int;
}
extern "C" {
    pub fn tty_vhangup_session(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_open_proc_set_tty(filp: *mut file, tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_signal_session_leader(tty: *mut tty_struct, exit_session: c_int) -> c_int;
}
extern "C" {
    pub fn session_clear_tty(session: *mut pid);
}
extern "C" {
    pub fn tty_buffer_free_all(port: *mut tty_port);
}
extern "C" {
    pub fn tty_buffer_flush(tty: *mut tty_struct, ld: *mut tty_ldisc);
}
extern "C" {
    pub fn tty_buffer_init(port: *mut tty_port);
}
extern "C" {
    pub fn tty_buffer_set_lock_subclass(port: *mut tty_port);
}
extern "C" {
    pub fn tty_buffer_restart_work(port: *mut tty_port) -> bool;
}
extern "C" {
    pub fn tty_buffer_cancel_work(port: *mut tty_port) -> bool;
}
extern "C" {
    pub fn tty_buffer_flush_work(port: *mut tty_port);
}
extern "C" {
    pub fn tty_termios_input_baud_rate(termios: *const ktermios) -> speed_t;
}
extern "C" {
    pub fn tty_ldisc_hangup(tty: *mut tty_struct, reset: bool);
}
extern "C" {
    pub fn tty_ldisc_reinit(tty: *mut tty_struct, disc: c_int) -> c_int;
}
extern "C" {
    pub fn tty_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn tty_default_fops(fops: *mut file_operations);
}
extern "C" {
    pub fn tty_alloc_file(file: *mut file) -> c_int;
}
extern "C" {
    pub fn tty_add_file(tty: *mut tty_struct, file: *mut file);
}
extern "C" {
    pub fn tty_free_file(file: *mut file);
}
extern "C" {
    pub fn tty_release(inode: *mut inode, filp: *mut file) -> c_int;
}

extern "C" {
    pub fn tty_ldisc_setup(tty: *mut tty_struct, o_tty: *mut tty_struct) -> c_int;
}
extern "C" {
    pub fn tty_ldisc_release(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_ldisc_init(tty: *mut tty_struct) -> int __must_check;
}
extern "C" {
    pub fn tty_ldisc_deinit(tty: *mut tty_struct);
}
// tty_audit.c

extern "C" {
    pub fn tty_audit_tiocsti(tty: *const tty_struct, ch: u8);
}

extern "C" {
    pub fn redirected_tty_write(: *mut kiocb, : *mut iov_iter) -> isize;
}
