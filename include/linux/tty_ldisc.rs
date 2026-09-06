//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tty_ldisc.h
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
// the semaphore definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ld_semaphore {
    pub count: atomic_long_t,
    pub wait_lock: raw_spinlock_t,
    pub wait_readers: c_uint,
    pub read_wait: list_head,
    pub write_wait: list_head,

    pub dep_map: lockdep_map,

}

extern "C" {
    pub fn ldsem_down_read(sem: *mut ld_semaphore, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn ldsem_down_read_trylock(sem: *mut ld_semaphore) -> c_int;
}
extern "C" {
    pub fn ldsem_down_write(sem: *mut ld_semaphore, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn ldsem_up_read(sem: *mut ld_semaphore);
}
extern "C" {
    pub fn ldsem_up_write(sem: *mut ld_semaphore);
}

//
// struct tty_ldisc_ops - ldisc operations
//
// @name: name of this ldisc rendered in /proc/tty/ldiscs
// @num: ``N_*`` number (%N_TTY, %N_HDLC, ...) reserved to this ldisc
//
// @open: [TTY] ``int ()(struct tty_struct *tty)``
//
// This function is called when the line discipline is associated with the
// @tty. No other call into the line discipline for this tty will occur
// until it completes successfully. It should initialize any state needed
// by the ldisc, and set @tty->receive_room to the maximum amount of data
// the line discipline is willing to accept from the driver with a single
// call to @receive_buf(). Returning an error will prevent the ldisc from
// being attached.
//
// Optional. Can sleep.
//
// @close: [TTY] ``void ()(struct tty_struct *tty)``
//
// This function is called when the line discipline is being shutdown,
// either because the @tty is being closed or because the @tty is being
// changed to use a new line discipline. At the point of execution no
// further users will enter the ldisc code for this tty.
//
// Optional. Can sleep.
//
// @flush_buffer: [TTY] ``void ()(struct tty_struct *tty)``
//
// This function instructs the line discipline to clear its buffers of any
// input characters it may have queued to be delivered to the user mode
// process. It may be called at any point between open and close.
//
// Optional.
//
// @read: [TTY] ``ssize_t ()(struct tty_struct *tty, struct file *file, u8 *buf,
// size_t nr)``
//
// This function is called when the user requests to read from the @tty.
// The line discipline will return whatever characters it has buffered up
// for the user. If this function is not defined, the user will receive
// an %EIO error. Multiple read calls may occur in parallel and the ldisc
// must deal with serialization issues.
//
// Optional: %EIO unless provided. Can sleep.
//
// @write: [TTY] ``ssize_t ()(struct tty_struct *tty, struct file *file,
// const u8 *buf, size_t nr)``
//
// This function is called when the user requests to write to the @tty.
// The line discipline will deliver the characters to the low-level tty
// device for transmission, optionally performing some processing on the
// characters first. If this function is not defined, the user will
// receive an %EIO error.
//
// Optional: %EIO unless provided. Can sleep.
//
// @ioctl: [TTY] ``int ()(struct tty_struct *tty, unsigned int cmd,
// unsigned long arg)``
//
// This function is called when the user requests an ioctl which is not
// handled by the tty layer or the low-level tty driver. It is intended
// for ioctls which affect line discpline operation.  Note that the search
// order for ioctls is (1) tty layer, (2) tty low-level driver, (3) line
// discpline. So a low-level driver can "grab" an ioctl request before
// the line discpline has a chance to see it.
//
// Optional.
//
// @compat_ioctl: [TTY] ``int ()(struct tty_struct *tty, unsigned int cmd,
// unsigned long arg)``
//
// Process ioctl calls from 32-bit process on 64-bit system.
//
// Note that only ioctls that are neither "pointer to compatible
// structure" nor tty-generic.  Something private that takes an integer or
// a pointer to wordsize-sensitive structure belongs here, but most of
// ldiscs will happily leave it %NULL.
//
// Optional.
//
// @set_termios: [TTY] ``void ()(struct tty_struct *tty, const struct ktermios *old)``
//
// This function notifies the line discpline that a change has been made
// to the termios structure.
//
// Optional.
//
// @poll: [TTY] ``int ()(struct tty_struct *tty, struct file *file,
// struct poll_table_struct *wait)``
//
// This function is called when a user attempts to select/poll on a @tty
// device. It is solely the responsibility of the line discipline to
// handle poll requests.
//
// Optional.
//
// @hangup: [TTY] ``void ()(struct tty_struct *tty)``
//
// Called on a hangup. Tells the discipline that it should cease I/O to
// the tty driver. The driver should seek to perform this action quickly
// but should wait until any pending driver I/O is completed. No further
// calls into the ldisc code will occur.
//
// Optional. Can sleep.
//
// @receive_buf: [DRV] ``void ()(struct tty_struct *tty, const u8 *cp,
// const u8 *fp, size_t count)``
//
// This function is called by the low-level tty driver to send characters
// received by the hardware to the line discpline for processing. @cp is
// a pointer to the buffer of input character received by the device. @fp
// is a pointer to an array of flag bytes which indicate whether a
// character was received with a parity error, etc. @fp may be %NULL to
// indicate all data received is %TTY_NORMAL.
//
// Optional.
//
// @write_wakeup: [DRV] ``void ()(struct tty_struct *tty)``
//
// This function is called by the low-level tty driver to signal that line
// discpline should try to send more characters to the low-level driver
// for transmission. If the line discpline does not have any more data to
// send, it can just return. If the line discipline does have some data to
// send, please arise a tasklet or workqueue to do the real data transfer.
// Do not send data in this hook, it may lead to a deadlock.
//
// Optional.
//
// @dcd_change: [DRV] ``void ()(struct tty_struct *tty, bool active)``
//
// Tells the discipline that the DCD pin has changed its status. Used
// exclusively by the %N_PPS (Pulse-Per-Second) line discipline.
//
// Optional.
//
// @receive_buf2: [DRV] ``ssize_t ()(struct tty_struct *tty, const u8 *cp,
// const u8 *fp, size_t count)``
//
// This function is called by the low-level tty driver to send characters
// received by the hardware to the line discpline for processing. @cp is a
// pointer to the buffer of input character received by the device.  @fp
// is a pointer to an array of flag bytes which indicate whether a
// character was received with a parity error, etc. @fp may be %NULL to
// indicate all data received is %TTY_NORMAL. If assigned, prefer this
// function for automatic flow control.
//
// Optional.
//
// @lookahead_buf: [DRV] ``void ()(struct tty_struct *tty, const u8 *cp,
// const u8 *fp, size_t count)``
//
// This function is called by the low-level tty driver for characters
// not eaten by ->receive_buf() or ->receive_buf2(). It is useful for
// processing high-priority characters such as software flow-control
// characters that could otherwise get stuck into the intermediate
// buffer until tty has room to receive them. Ldisc must be able to
// handle later a ->receive_buf() or ->receive_buf2() call for the
// same characters (e.g. by skipping the actions for high-priority
// characters already handled by ->lookahead_buf()).
//
// Optional.
//
// @owner: module containting this ldisc (for reference counting)
//
// This structure defines the interface between the tty line discipline
// implementation and the tty routines. The above routines can be defined.
// Unless noted otherwise, they are optional, and can be filled in with a %NULL
// pointer.
//
// Hooks marked [TTY] are invoked from the TTY core, the [DRV] ones from the
// tty_driver side.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_ldisc_ops {
    pub name: *mut c_char,
    pub num: c_int,
//
// The following routines are called from above.
//
    pub tty): *mut *mut int (open)(struct tty_struct,
    pub tty): *mut *mut void (close)(struct tty_struct,
    pub tty): *mut *mut void (flush_buffer)(struct tty_struct,
    pub offset): *mut *mut *mut size_t nr, void cookie, unsigned long,
    pub nr): *const *const u8 buf, size_t,
    pub arg): c_ulong,
    pub arg): c_ulong,
    pub old): *const *const *const void (set_termios)(struct tty_struct tty, struct ktermios,
    pub wait): *mut poll_table_struct,
    pub tty): *mut *mut void (hangup)(struct tty_struct,
//
// The following routines are called from below.
//
    pub count): *const *const u8 fp, size_t,
    pub tty): *mut *mut void (write_wakeup)(struct tty_struct,
    pub active): *mut *mut *mut void (dcd_change)(struct tty_struct tty, bool,
    pub count): *const *const u8 fp, size_t,
    pub count): *const *const u8 fp, size_t,
    pub owner: *mut module,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_ldisc {
    pub ops: *const tty_ldisc_ops,
    pub tty: *mut tty_struct,
}

extern "C" {
    pub fn tty_ldisc_deref(: *mut tty_ldisc);
}
extern "C" {
    pub fn tty_ldisc_flush(tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_register_ldisc(new_ldisc: *const tty_ldisc_ops) -> c_int;
}
extern "C" {
    pub fn tty_unregister_ldisc(ldisc: *const tty_ldisc_ops);
}
extern "C" {
    pub fn tty_set_ldisc(tty: *mut tty_struct, disc: c_int) -> c_int;
}
