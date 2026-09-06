//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tty.h
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
// (Note: the *_driver.minor_start values 1, 64, 128, 192 are
// hardcoded at present.)
//

//
// This character is the same as _POSIX_VDISABLE: it cannot be used as
// a c_cc[] character, but indicates that a particular special character
// isn't in use (eg VINTR has no character etc)
//

//
// struct tty_struct - state associated with a tty while open
//
// @kref: reference counting by tty_kref_get() and tty_kref_put(), reaching zero
// frees the structure
// @dev: class device or %NULL (e.g. ptys, serdev)
// @driver: &struct tty_driver operating this tty
// @ops: &struct tty_operations of @driver for this tty (open, close, etc.)
// @index: index of this tty (e.g. to construct @name like tty12)
// @ldisc_sem: protects line discipline changes (@ldisc) -- lock tty not pty
// @ldisc: the current line discipline for this tty (n_tty by default)
// @atomic_write_lock: protects against concurrent writers, i.e. locks
// @write_cnt, @write_buf and similar
// @legacy_mutex: leftover from history (BKL -> BTM -> @legacy_mutex),
// protecting several operations on this tty
// @throttle_mutex: protects against concurrent tty_throttle_safe() and
// tty_unthrottle_safe() (but not tty_unthrottle())
// @termios_rwsem: protects @termios and @termios_locked
// @winsize_mutex: protects @winsize
// @termios: termios for the current tty, copied from/to @driver.termios
// @termios_locked: locked termios (by %TIOCGLCKTRMIOS and %TIOCSLCKTRMIOS
// ioctls)
// @name: name of the tty constructed by tty_line_name() (e.g. ttyS3)
// @flags: bitwise OR of %TTY_THROTTLED, %TTY_IO_ERROR, ...
// @count: count of open processes, reaching zero cancels all the work for
// this tty and drops a @kref too (but does not free this tty)
// @winsize: size of the terminal "window" (cf. @winsize_mutex)
// @flow: flow settings grouped together
// @flow.lock: lock for @flow members
// @flow.stopped: tty stopped/started by stop_tty()/start_tty()
// @flow.tco_stopped: tty stopped/started by %TCOOFF/%TCOON ioctls (it has
// precedence over @flow.stopped)
// @ctrl: control settings grouped together
// @ctrl.lock: lock for @ctrl members
// @ctrl.pgrp: process group of this tty (setpgrp(2))
// @ctrl.session: session of this tty (setsid(2)). Writes are protected by both
// @ctrl.lock and @legacy_mutex, readers must use at least one of
// them.
// @ctrl.pktstatus: packet mode status (bitwise OR of %TIOCPKT_ constants)
// @ctrl.packet: packet mode enabled
// @hw_stopped: not controlled by the tty layer, under @driver's control for CTS
// handling
// @receive_room: bytes permitted to feed to @ldisc without any being lost
// @flow_change: controls behavior of throttling, see tty_throttle_safe() and
// tty_unthrottle_safe()
// @link: link to another pty (master -> slave and vice versa)
// @fasync: state for %O_ASYNC (for %SIGIO); managed by fasync_helper()
// @write_wait: concurrent writers are waiting in this queue until they are
// allowed to write
// @read_wait: readers wait for data in this queue
// @hangup_work: normally a work to perform a hangup (do_tty_hangup()); while
// freeing the tty, (re)used to release_one_tty()
// @disc_data: pointer to @ldisc's private data (e.g. to &struct n_tty_data)
// @driver_data: pointer to @driver's private data (e.g. &struct uart_state)
// @files_lock:	protects @tty_files list
// @tty_files: list of (re)openers of this tty (i.e. linked &struct
// tty_file_private)
// @closing: when set during close, n_tty processes only START & STOP chars
// @write_buf: temporary buffer used during tty_write() to copy user data to
// @write_cnt: count of bytes written in tty_write() to @write_buf
// @SAK_work: if the tty has a pending do_SAK, it is queued here
// @port: persistent storage for this device (i.e. &struct tty_port)
//
// All of the state associated with a tty while the tty is open. Persistent
// storage for tty devices is referenced here as @port and is documented in
// &struct tty_port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_struct {
    pub kref: kref,
    pub index: c_int,
    pub dev: *mut device,
    pub driver: *mut tty_driver,
    pub port: *mut tty_port,
    pub ops: *const tty_operations,
    pub ldisc: *mut tty_ldisc,
    pub ldisc_sem: ld_semaphore,
    pub atomic_write_lock: mutex,
    pub legacy_mutex: mutex,
    pub throttle_mutex: mutex,
    pub termios_rwsem: rw_semaphore,
    pub winsize_mutex: mutex,
    pub termios_locked: ktermios termios,,
    pub name: [c_char; 64],
    pub flags: c_ulong,
    pub count: c_int,
    pub receive_room: c_uint,
    pub winsize: winsize,
    pub lock: spinlock_t,
    pub stopped: bool,
    pub tco_stopped: bool,
    pub flow: },
    pub pgrp: *mut pid,
    pub session: *mut pid,
    pub lock: spinlock_t,
    pub pktstatus: c_uchar,
    pub packet: bool,
    pub ctrl: },
    pub hw_stopped: bool,
    pub closing: bool,
    pub flow_change: c_int,
    pub link: *mut tty_struct,
    pub fasync: *mut fasync_struct,
    pub write_wait: wait_queue_head_t,
    pub read_wait: wait_queue_head_t,
    pub hangup_work: work_struct,
    pub disc_data: *mut c_void,
    pub driver_data: *mut c_void,
    pub files_lock: spinlock_t,
    pub write_cnt: c_int,
    pub write_buf: *mut u8,
    pub tty_files: list_head,
    pub SAK_work: work_struct,
    pub __randomize_layout: },
// Each of a tty's open files has private_data pointing to tty_file_private
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_file_private {
    pub tty: *mut tty_struct,
    pub file: *mut file,
    pub list: list_head,
}

//
// enum tty_struct_flags - TTY Struct Flags
//
// These bits are used in the :c:member:`tty_struct.flags` field.
//
// So that interrupts won't be able to mess up the queues,
// copy_to_cooked must be atomic with respect to itself, as must
// tty->write.  Thus, you must use the inline functions set_bit() and
// clear_bit() to make things atomic.
//
// @TTY_THROTTLED:
// Driver input is throttled. The ldisc should call
// :c:member:`tty_driver.unthrottle()` in order to resume reception when
// it is ready to process more data (at threshold min).
//
// @TTY_IO_ERROR:
// If set, causes all subsequent userspace read/write calls on the tty to
// fail, returning -%EIO. (May be no ldisc too.)
//
// @TTY_OTHER_CLOSED:
// Device is a pty and the other side has closed.
//
// @TTY_EXCLUSIVE:
// Exclusive open mode (a single opener).
//
// @TTY_DO_WRITE_WAKEUP:
// If set, causes the driver to call the
// :c:member:`tty_ldisc_ops.write_wakeup()` method in order to resume
// transmission when it can accept more data to transmit.
//
// @TTY_LDISC_OPEN:
// Indicates that a line discipline is open. For debugging purposes only.
//
// @TTY_PTY_LOCK:
// A flag private to pty code to implement %TIOCSPTLCK/%TIOCGPTLCK logic.
//
// @TTY_NO_WRITE_SPLIT:
// Prevent driver from splitting up writes into smaller chunks (preserve
// write boundaries to driver).
//
// @TTY_HUPPED:
// The TTY was hung up. This is set post :c:member:`tty_driver.hangup()`.
//
// @TTY_HUPPING:
// The TTY is in the process of hanging up to abort potential readers.
//
// @TTY_LDISC_CHANGING:
// Line discipline for this TTY is being changed. I/O should not block
// when this is set. Use tty_io_nonblock() to check.
//
// @TTY_LDISC_HALTED:
// Line discipline for this TTY was stopped. No work should be queued to
// this ldisc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tty_struct_flags {
    TTY_THROTTLED,
    TTY_IO_ERROR,
    TTY_OTHER_CLOSED,
    TTY_EXCLUSIVE,
    TTY_DO_WRITE_WAKEUP,
    TTY_LDISC_OPEN,
    TTY_PTY_LOCK,
    TTY_NO_WRITE_SPLIT,
    TTY_HUPPED,
    TTY_HUPPING,
    TTY_LDISC_CHANGING,
    TTY_LDISC_HALTED,
}

extern "C" {
    pub fn test_bit(_arg: TTY_IO_ERROR, _arg: &tty->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: TTY_THROTTLED, _arg: &tty->flags) -> return;
}

extern "C" {
    pub fn tty_kref_put(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_vhangup_self();
}
extern "C" {
    pub fn disassociate_ctty(priv: c_int);
}
extern "C" {
    pub fn tty_devnum(tty: *mut tty_struct) -> dev_t;
}
extern "C" {
    pub fn proc_clear_tty(p: *mut task_struct);
}
// tty_io.c
extern "C" {
    pub fn tty_init() -> int __init;
}
extern "C" {
    pub fn tty_kclose(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_dev_name_to_number(name: *const c_char, number: *mut dev_t) -> c_int;
}

// tty_io.c

extern "C" {
    pub fn vcs_init() -> c_int;
}
//
// tty_kref_get - get a tty reference
// @tty: tty device
//
// Returns: a new reference to a tty object
//
// Locking: The caller must hold sufficient locks/counts to ensure that their
// existing reference cannot go away.
//
extern "C" {
    pub fn tty_wait_until_sent(tty: *mut tty_struct, timeout: c_long);
}
extern "C" {
    pub fn stop_tty(tty: *mut tty_struct);
}
extern "C" {
    pub fn start_tty(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_write_message(tty: *mut tty_struct, msg: *mut c_char);
}
extern "C" {
    pub fn tty_send_xchar(tty: *mut tty_struct, ch: u8) -> c_int;
}
extern "C" {
    pub fn tty_put_char(tty: *mut tty_struct, c: u8) -> c_int;
}
extern "C" {
    pub fn tty_chars_in_buffer(tty: *mut tty_struct) -> c_uint;
}
extern "C" {
    pub fn tty_write_room(tty: *mut tty_struct) -> c_uint;
}
extern "C" {
    pub fn tty_driver_flush_buffer(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_unthrottle(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_throttle_safe(tty: *mut tty_struct) -> bool;
}
extern "C" {
    pub fn tty_unthrottle_safe(tty: *mut tty_struct) -> bool;
}
extern "C" {
    pub fn tty_do_resize(tty: *mut tty_struct, ws: *mut winsize) -> c_int;
}
extern "C" {
    pub fn tty_get_tiocm(tty: *mut tty_struct) -> c_int;
}
extern "C" {
    pub fn is_current_pgrp_orphaned() -> c_int;
}
extern "C" {
    pub fn tty_hangup(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_vhangup(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_hung_up_p(filp: *mut file) -> c_int;
}
extern "C" {
    pub fn do_SAK(tty: *mut tty_struct);
}
extern "C" {
    pub fn __do_SAK(tty: *mut tty_struct);
}
extern "C" {
    pub fn no_tty();
}
extern "C" {
    pub fn tty_termios_baud_rate(termios: *const ktermios) -> speed_t;
}
//
// tty_get_baud_rate - get tty bit rates
// @tty: tty to query
//
// Returns: the baud rate as an integer for this terminal
//
// Locking: The termios lock must be held by the caller.
//
extern "C" {
    pub fn tty_termios_baud_rate(_arg: &tty->termios) -> return;
}
extern "C" {
    pub fn tty_get_char_size(cflag: c_uint) -> c_uchar;
}
extern "C" {
    pub fn tty_get_frame_size(cflag: c_uint) -> c_uchar;
}
extern "C" {
    pub fn tty_termios_copy_hw(new: *mut ktermios, old: *const ktermios);
}
extern "C" {
    pub fn tty_termios_hw_change(a: *const ktermios, b: *const ktermios) -> bool;
}
extern "C" {
    pub fn tty_set_termios(tty: *mut tty_struct, kt: *mut ktermios) -> c_int;
}
extern "C" {
    pub fn tty_wakeup(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_mode_ioctl(tty: *mut tty_struct, cmd: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn tty_perform_flush(tty: *mut tty_struct, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn tty_release_struct(tty: *mut tty_struct, idx: c_int);
}
extern "C" {
    pub fn tty_init_termios(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_save_termios(tty: *mut tty_struct);
}
// n_tty.c
extern "C" {
    pub fn n_tty_inherit_ops(ops: *mut tty_ldisc_ops);
}

extern "C" {
    pub fn n_tty_init() -> void __init;
}

// tty_audit.c

extern "C" {
    pub fn tty_audit_exit();
}
extern "C" {
    pub fn tty_audit_fork(sig: *mut signal_struct);
}
extern "C" {
    pub fn tty_audit_push() -> c_int;
}

// tty_ioctl.c
// vt.c
extern "C" {
    pub fn vt_ioctl(tty: *mut tty_struct, cmd: c_uint, arg: c_ulong) -> c_int;
}
// tty_mutex.c
// functions for preparation of BKL removal
extern "C" {
    pub fn tty_lock(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_lock_interruptible(tty: *mut tty_struct) -> c_int;
}
extern "C" {
    pub fn tty_unlock(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_lock_slave(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_unlock_slave(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_set_lock_subclass(tty: *mut tty_struct);
}
