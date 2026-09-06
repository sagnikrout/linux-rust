//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tty_port.h
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
// struct tty_port_operations -- operations on tty_port
// @carrier_raised: return true if the carrier is raised on @port
// @dtr_rts: raise the DTR line if @active is true, otherwise lower DTR
// @shutdown: called when the last close completes or a hangup finishes IFF the
// port was initialized. Do not use to free resources. Turn off the device
// only. Called under the port mutex to serialize against @activate and
// @shutdown.
// @activate: called under the port mutex from tty_port_open(), serialized using
// the port mutex. Supposed to turn on the device.
//
// FIXME: long term getting the tty argument *out* of this would be good
// for consoles.
//
// @destruct: called on the final put of a port. Free resources, possibly incl.
// the port itself.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_port_operations {
    pub port): *mut *mut bool (carrier_raised)(struct tty_port,
    pub active): *mut *mut *mut void (dtr_rts)(struct tty_port port, bool,
    pub port): *mut *mut void (shutdown)(struct tty_port,
    pub tty): *mut *mut *mut int (activate)(struct tty_port port, struct tty_struct,
    pub port): *mut *mut void (destruct)(struct tty_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_port_client_operations {
    pub count): usize,
    pub count): *const *const u8 fp, size_t,
    pub port): *mut *mut void (write_wakeup)(struct tty_port,
}

//
// struct tty_port -- port level information
//
// @buf: buffer for this port, locked internally
// @tty: back pointer to &struct tty_struct, valid only if the tty is open. Use
// tty_port_tty_get() to obtain it (and tty_kref_put() to release).
// @itty: internal back pointer to &struct tty_struct. Avoid this. It should be
// eliminated in the long term.
// @ops: tty port operations (like activate, shutdown), see &struct
// tty_port_operations
// @client_ops: tty port client operations (like receive_buf, write_wakeup).
// By default, tty_port_default_client_ops is used.
// @lock: lock protecting @tty
// @blocked_open: # of procs waiting for open in tty_port_block_til_ready()
// @count: usage count
// @open_wait: open waiters queue (waiting e.g. for a carrier)
// @delta_msr_wait: modem status change queue (waiting for MSR changes)
// @flags: user TTY flags (%ASYNC_)
// @iflags: internal flags (%TTY_PORT_)
// @console: when set, the port is a console
// @mutex: locking, for open, shutdown and other port operations
// @buf_mutex: @xmit_buf alloc lock
// @xmit_buf: optional xmit buffer used by some drivers
// @xmit_fifo: optional xmit buffer used by some drivers
// @close_delay: delay in jiffies to wait when closing the port
// @closing_wait: delay in jiffies for output to be sent before closing
// @drain_delay: set to zero if no pure time based drain is needed else set to
// size of fifo
// @kref: references counter. Reaching zero calls @ops->destruct() if non-%NULL
// or frees the port otherwise.
// @client_data: pointer to private data, for @client_ops
//
// Each device keeps its own port level information. &struct tty_port was
// introduced as a common structure for such information. As every TTY device
// shall have a backing tty_port structure, every driver can use these members.
//
// The tty port has a different lifetime to the tty so must be kept apart.
// In addition be careful as tty -> port mappings are valid for the life
// of the tty object but in many cases port -> tty mappings are valid only
// until a hangup so don't use the wrong path.
//
// Tty port shall be initialized by tty_port_init() and shut down either by
// tty_port_destroy() (refcounting not used), or tty_port_put() (refcounting).
//
// There is a lot of helpers around &struct tty_port too. To name the most
// significant ones: tty_port_open(), tty_port_close() (or
// tty_port_close_start() and tty_port_close_end() separately if need be), and
// tty_port_hangup(). These call @ops->activate() and @ops->shutdown() as
// needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_port {
    pub buf: tty_bufhead,
    pub tty: *mut tty_struct,
    pub itty: *mut tty_struct,
    pub ops: *const tty_port_operations,
    pub client_ops: *const tty_port_client_operations,
    pub lock: spinlock_t,
    pub blocked_open: c_int,
    pub count: c_int,
    pub open_wait: wait_queue_head_t,
    pub delta_msr_wait: wait_queue_head_t,
    pub flags: c_ulong,
    pub iflags: c_ulong,
    pub console:1: c_uchar,
    pub mutex: mutex,
    pub buf_mutex: mutex,
    pub xmit_buf: *mut u8,
    pub u8): DECLARE_KFIFO_PTR(xmit_fifo,,
    pub close_delay: c_uint,
    pub closing_wait: c_uint,
    pub drain_delay: c_int,
    pub kref: kref,
    pub client_data: *mut c_void,
}

// tty_port::iflags bits -- use atomic bit ops

//
// uart drivers: use the uart_port::status field and the UPSTAT_* defines
// for s/w-based flow control steering and carrier detection status
//

extern "C" {
    pub fn tty_port_init(port: *mut tty_port);
}
extern "C" {
    pub fn tty_port_link_wq(port: *mut tty_port, flip_wq: *mut workqueue_struct);
}
extern "C" {
    pub fn tty_port_alloc_xmit_buf(port: *mut tty_port) -> c_int;
}
extern "C" {
    pub fn tty_port_free_xmit_buf(port: *mut tty_port);
}
extern "C" {
    pub fn tty_port_destroy(port: *mut tty_port);
}
extern "C" {
    pub fn tty_port_put(port: *mut tty_port);
}
//
// Never overwrite the workqueue set by tty_port_link_wq().
// No effect when %TTY_DRIVER_NO_WORKQUEUE is set, as driver->flip_wq is
// %NULL.
//
// If the cts flow control is enabled, return true.
extern "C" {
    pub fn test_bit(_arg: TTY_PORT_CTS_FLOW, _arg: &port->iflags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: TTY_PORT_ACTIVE, _arg: &port->iflags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: TTY_PORT_CHECK_CD, _arg: &port->iflags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: TTY_PORT_SUSPENDED, _arg: &port->iflags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: TTY_PORT_INITIALIZED, _arg: &port->iflags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: TTY_PORT_KOPENED, _arg: &port->iflags) -> return;
}
extern "C" {
    pub fn tty_port_tty_set(port: *mut tty_port, tty: *mut tty_struct);
}
extern "C" {
    pub fn tty_port_carrier_raised(port: *mut tty_port) -> bool;
}
extern "C" {
    pub fn tty_port_raise_dtr_rts(port: *mut tty_port);
}
extern "C" {
    pub fn tty_port_lower_dtr_rts(port: *mut tty_port);
}
extern "C" {
    pub fn tty_port_hangup(port: *mut tty_port);
}
extern "C" {
    pub fn __tty_port_tty_hangup(port: *mut tty_port, check_clocal: bool, async: bool);
}
extern "C" {
    pub fn tty_port_tty_wakeup(port: *mut tty_port);
}
extern "C" {
    pub fn tty_port_close_end(port: *mut tty_port, tty: *mut tty_struct);
}
//
// tty_port_tty_hangup - helper to hang up a tty asynchronously
// @port: tty port
// @check_clocal: hang only ttys with %CLOCAL unset?
//
// tty_port_tty_vhangup - helper to hang up a tty synchronously
// @port: tty port
//

extern "C" {
    pub fn tty_kref_put(tty: *mut tty_struct);
}

