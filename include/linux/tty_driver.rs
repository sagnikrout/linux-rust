//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tty_driver.h
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
// enum tty_driver_flag -- TTY Driver Flags
//
// These are flags passed to tty_alloc_driver().
//
// @TTY_DRIVER_INSTALLED:
// Whether this driver was succesfully installed. This is a tty internal
// flag. Do not touch.
//
// @TTY_DRIVER_RESET_TERMIOS:
// Requests the tty layer to reset the termios setting when the last
// process has closed the device. Used for PTYs, in particular.
//
// @TTY_DRIVER_REAL_RAW:
// Indicates that the driver will guarantee not to set any special
// character handling flags if this is set for the tty:
//
// ``(IGNBRK || (!BRKINT && !PARMRK)) && (IGNPAR || !INPCK)``
//
// That is, if there is no reason for the driver to
// send notifications of parity and break characters up to the line
// driver, it won't do so.  This allows the line driver to optimize for
// this case if this flag is set.  (Note that there is also a promise, if
// the above case is true, not to signal overruns, either.)
//
// @TTY_DRIVER_DYNAMIC_DEV:
// The individual tty devices need to be registered with a call to
// tty_register_device() when the device is found in the system and
// unregistered with a call to tty_unregister_device() so the devices will
// be show up properly in sysfs.  If not set, all &tty_driver.num entries
// will be created by the tty core in sysfs when tty_register_driver() is
// called.  This is to be used by drivers that have tty devices that can
// appear and disappear while the main tty driver is registered with the
// tty core.
//
// @TTY_DRIVER_DEVPTS_MEM:
// Don't use the standard arrays (&tty_driver.ttys and
// &tty_driver.termios), instead use dynamic memory keyed through the
// devpts filesystem. This is only applicable to the PTY driver.
//
// @TTY_DRIVER_HARDWARE_BREAK:
// Hardware handles break signals. Pass the requested timeout to the
// &tty_operations.break_ctl instead of using a simple on/off interface.
//
// @TTY_DRIVER_DYNAMIC_ALLOC:
// Do not allocate structures which are needed per line for this driver
// (&tty_driver.ports) as it would waste memory. The driver will take
// care. This is only applicable to the PTY driver.
//
// @TTY_DRIVER_UNNUMBERED_NODE:
// Do not create numbered ``/dev`` nodes. For example, create
// ``/dev/ttyprintk`` and not ``/dev/ttyprintk0``. Applicable only when a
// driver for a single tty device is being allocated.
//
// @TTY_DRIVER_NO_WORKQUEUE:
// Do not create workqueue when tty_register_driver(). Whenever set, flip
// buffer workqueue can be set by tty_port_link_wq() for every port.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tty_driver_flag {
    TTY_DRIVER_INSTALLED		= BIT(0),
    TTY_DRIVER_RESET_TERMIOS	= BIT(1),
    TTY_DRIVER_REAL_RAW		= BIT(2),
    TTY_DRIVER_DYNAMIC_DEV		= BIT(3),
    TTY_DRIVER_DEVPTS_MEM		= BIT(4),
    TTY_DRIVER_HARDWARE_BREAK	= BIT(5),
    TTY_DRIVER_DYNAMIC_ALLOC	= BIT(6),
    TTY_DRIVER_UNNUMBERED_NODE	= BIT(7),
    TTY_DRIVER_NO_WORKQUEUE		= BIT(8),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tty_driver_type {
    TTY_DRIVER_TYPE_SYSTEM,
    TTY_DRIVER_TYPE_CONSOLE,
    TTY_DRIVER_TYPE_SERIAL,
    TTY_DRIVER_TYPE_PTY,
    TTY_DRIVER_TYPE_SCC,
    TTY_DRIVER_TYPE_SYSCONS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tty_driver_subtype {
    SYSTEM_TYPE_TTY = 1,
    SYSTEM_TYPE_CONSOLE,
    SYSTEM_TYPE_SYSCONS,
    SYSTEM_TYPE_SYSPTMX,

    PTY_TYPE_MASTER = 1,
    PTY_TYPE_SLAVE,

    SERIAL_TYPE_NORMAL = 1,
}

//
// struct tty_operations -- interface between driver and tty
//
// @lookup: ``struct tty_struct *()(struct tty_driver *self, struct file *,
// int idx)``
//
// Return the tty device corresponding to @idx, %NULL if there is not
// one currently in use and an %ERR_PTR value on error. Called under
// %tty_mutex (for now!)
//
// Optional method. Default behaviour is to use the @self->ttys array.
//
// @install: ``int ()(struct tty_driver *self, struct tty_struct *tty)``
//
// Install a new @tty into the @self's internal tables. Used in
// conjunction with @lookup and @remove methods.
//
// Optional method. Default behaviour is to use the @self->ttys array.
//
// @remove: ``void ()(struct tty_driver *self, struct tty_struct *tty)``
//
// Remove a closed @tty from the @self's internal tables. Used in
// conjunction with @lookup and @remove methods.
//
// Optional method. Default behaviour is to use the @self->ttys array.
//
// @open: ``int ()(struct tty_struct *tty, struct file *)``
//
// This routine is called when a particular @tty device is opened. This
// routine is mandatory; if this routine is not filled in, the attempted
// open will fail with %ENODEV.
//
// Required method. Called with tty lock held. May sleep.
//
// @close: ``void ()(struct tty_struct *tty, struct file *)``
//
// This routine is called when a particular @tty device is closed. At the
// point of return from this call the driver must make no further ldisc
// calls of any kind.
//
// Remark: called even if the corresponding @open() failed.
//
// Required method. Called with tty lock held. May sleep.
//
// @shutdown: ``void ()(struct tty_struct *tty)``
//
// This routine is called under the tty lock when a particular @tty device
// is closed for the last time. It executes before the @tty resources
// are freed so may execute while another function holds a @tty kref.
//
// @cleanup: ``void ()(struct tty_struct *tty)``
//
// This routine is called asynchronously when a particular @tty device
// is closed for the last time freeing up the resources. This is
// actually the second part of shutdown for routines that might sleep.
//
// @write: ``ssize_t ()(struct tty_struct *tty, const u8 *buf, size_t count)``
//
// This routine is called by the kernel to write a series (@count) of
// characters (@buf) to the @tty device. The characters may come from
// user space or kernel space.  This routine will return the
// number of characters actually accepted for writing.
//
// May occur in parallel in special cases. Because this includes panic
// paths drivers generally shouldn't try and do clever locking here.
//
// Optional: Required for writable devices. May not sleep.
//
// @put_char: ``int ()(struct tty_struct *tty, u8 ch)``
//
// This routine is called by the kernel to write a single character @ch to
// the @tty device. If the kernel uses this routine, it must call the
// @flush_chars() routine (if defined) when it is done stuffing characters
// into the driver. If there is no room in the queue, the character is
// ignored.
//
// Optional: Kernel will use the @write method if not provided. Do not
// call this function directly, call tty_put_char().
//
// @flush_chars: ``void ()(struct tty_struct *tty)``
//
// This routine is called by the kernel after it has written a
// series of characters to the tty device using @put_char().
//
// Optional. Do not call this function directly, call
// tty_driver_flush_chars().
//
// @write_room: ``unsigned int ()(struct tty_struct *tty)``
//
// This routine returns the numbers of characters the @tty driver
// will accept for queuing to be written.  This number is subject
// to change as output buffers get emptied, or if the output flow
// control is acted.
//
// The ldisc is responsible for being intelligent about multi-threading of
// write_room/write calls
//
// Required if @write method is provided else not needed. Do not call this
// function directly, call tty_write_room()
//
// @chars_in_buffer: ``unsigned int ()(struct tty_struct *tty)``
//
// This routine returns the number of characters in the device private
// output queue. Used in tty_wait_until_sent() and for poll()
// implementation.
//
// Optional: if not provided, it is assumed there is no queue on the
// device. Do not call this function directly, call tty_chars_in_buffer().
//
// @ioctl: ``int ()(struct tty_struct *tty, unsigned int cmd,
// unsigned long arg)``
//
// This routine allows the @tty driver to implement device-specific
// ioctls. If the ioctl number passed in @cmd is not recognized by the
// driver, it should return %ENOIOCTLCMD.
//
// Optional.
//
// @compat_ioctl: ``long ()(struct tty_struct *tty, unsigned int cmd,
// unsigned long arg)``
//
// Implement ioctl processing for 32 bit process on 64 bit system.
//
// Optional.
//
// @set_termios: ``void ()(struct tty_struct *tty, const struct ktermios *old)``
//
// This routine allows the @tty driver to be notified when device's
// termios settings have changed. New settings are in @tty->termios.
// Previous settings are passed in the @old argument.
//
// The API is defined such that the driver should return the actual modes
// selected. This means that the driver is responsible for modifying any
// bits in @tty->termios it cannot fulfill to indicate the actual modes
// being used.
//
// Optional. Called under the @tty->termios_rwsem. May sleep.
//
// @ldisc_ok: ``int ()(struct tty_struct *tty, int ldisc)``
//
// This routine allows the @tty driver to decide if it can deal
// with a particular @ldisc.
//
// Optional. Called under the @tty->ldisc_sem and @tty->termios_rwsem.
//
// @set_ldisc: ``void ()(struct tty_struct *tty)``
//
// This routine allows the @tty driver to be notified when the device's
// line discipline is being changed. At the point this is done the
// discipline is not yet usable.
//
// Optional. Called under the @tty->ldisc_sem and @tty->termios_rwsem.
//
// @throttle: ``void ()(struct tty_struct *tty)``
//
// This routine notifies the @tty driver that input buffers for the line
// discipline are close to full, and it should somehow signal that no more
// characters should be sent to the @tty.
//
// Serialization including with @unthrottle() is the job of the ldisc
// layer.
//
// Optional: Always invoke via tty_throttle_safe(). Called under the
// @tty->termios_rwsem.
//
// @unthrottle: ``void ()(struct tty_struct *tty)``
//
// This routine notifies the @tty driver that it should signal that
// characters can now be sent to the @tty without fear of overrunning the
// input buffers of the line disciplines.
//
// Optional. Always invoke via tty_unthrottle(). Called under the
// @tty->termios_rwsem.
//
// @stop: ``void ()(struct tty_struct *tty)``
//
// This routine notifies the @tty driver that it should stop outputting
// characters to the tty device.
//
// Called with @tty->flow.lock held. Serialized with @start() method.
//
// Optional. Always invoke via stop_tty().
//
// @start: ``void ()(struct tty_struct *tty)``
//
// This routine notifies the @tty driver that it resumed sending
// characters to the @tty device.
//
// Called with @tty->flow.lock held. Serialized with stop() method.
//
// Optional. Always invoke via start_tty().
//
// @hangup: ``void ()(struct tty_struct *tty)``
//
// This routine notifies the @tty driver that it should hang up the @tty
// device.
//
// Optional. Called with tty lock held.
//
// @break_ctl: ``int ()(struct tty_struct *tty, int state)``
//
// This optional routine requests the @tty driver to turn on or off BREAK
// status on the RS-232 port. If @state is -1, then the BREAK status
// should be turned on; if @state is 0, then BREAK should be turned off.
//
// If this routine is implemented, the high-level tty driver will handle
// the following ioctls: %TCSBRK, %TCSBRKP, %TIOCSBRK, %TIOCCBRK.
//
// If the driver sets %TTY_DRIVER_HARDWARE_BREAK in tty_alloc_driver(),
// then the interface will also be called with actual times and the
// hardware is expected to do the delay work itself. 0 and -1 are still
// used for on/off.
//
// Optional: Required for %TCSBRK/%BRKP/etc. handling. May sleep.
//
// @flush_buffer: ``void ()(struct tty_struct *tty)``
//
// This routine discards device private output buffer. Invoked on close,
// hangup, to implement %TCOFLUSH ioctl and similar.
//
// Optional: if not provided, it is assumed there is no queue on the
// device. Do not call this function directly, call
// tty_driver_flush_buffer().
//
// @wait_until_sent: ``void ()(struct tty_struct *tty, int timeout)``
//
// This routine waits until the device has written out all of the
// characters in its transmitter FIFO. Or until @timeout (in jiffies) is
// reached.
//
// Optional: If not provided, the device is assumed to have no FIFO.
// Usually correct to invoke via tty_wait_until_sent(). May sleep.
//
// @send_xchar: ``void ()(struct tty_struct *tty, u8 ch)``
//
// This routine is used to send a high-priority XON/XOFF character (@ch)
// to the @tty device.
//
// Optional: If not provided, then the @write method is called under
// the @tty->atomic_write_lock to keep it serialized with the ldisc.
//
// @tiocmget: ``int ()(struct tty_struct *tty)``
//
// This routine is used to obtain the modem status bits from the @tty
// driver.
//
// Optional: If not provided, then %ENOTTY is returned from the %TIOCMGET
// ioctl. Do not call this function directly, call tty_tiocmget().
//
// @tiocmset: ``int ()(struct tty_struct *tty,
// unsigned int set, unsigned int clear)``
//
// This routine is used to set the modem status bits to the @tty driver.
// First, @clear bits should be cleared, then @set bits set.
//
// Optional: If not provided, then %ENOTTY is returned from the %TIOCMSET
// ioctl. Do not call this function directly, call tty_tiocmset().
//
// @resize: ``int ()(struct tty_struct *tty, struct winsize *ws)``
//
// Called when a termios request is issued which changes the requested
// terminal geometry to @ws.
//
// Optional: the default action is to update the termios structure
// without error. This is usually the correct behaviour. Drivers should
// not force errors here if they are not resizable objects (e.g. a serial
// line). See tty_do_resize() if you need to wrap the standard method
// in your own logic -- the usual case.
//
// @get_icount: ``int ()(struct tty_struct *tty,
// struct serial_icounter *icount)``
//
// Called when the @tty device receives a %TIOCGICOUNT ioctl. Passed a
// kernel structure @icount to complete.
//
// Optional: called only if provided, otherwise %ENOTTY will be returned.
//
// @get_serial: ``int ()(struct tty_struct *tty, struct serial_struct *p)``
//
// Called when the @tty device receives a %TIOCGSERIAL ioctl. Passed a
// kernel structure @p (&struct serial_struct) to complete.
//
// Optional: called only if provided, otherwise %ENOTTY will be returned.
// Do not call this function directly, call tty_tiocgserial().
//
// @set_serial: ``int ()(struct tty_struct *tty, struct serial_struct *p)``
//
// Called when the @tty device receives a %TIOCSSERIAL ioctl. Passed a
// kernel structure @p (&struct serial_struct) to set the values from.
//
// Optional: called only if provided, otherwise %ENOTTY will be returned.
// Do not call this function directly, call tty_tiocsserial().
//
// @show_fdinfo: ``void ()(struct tty_struct *tty, struct seq_file *m)``
//
// Called when the @tty device file descriptor receives a fdinfo request
// from VFS (to show in /proc/<pid>/fdinfo/). @m should be filled with
// information.
//
// Optional: called only if provided, otherwise nothing is written to @m.
// Do not call this function directly, call tty_show_fdinfo().
//
// @poll_init: ``int ()(struct tty_driver *driver, int line, char *options)``
//
// kgdboc support (Documentation/process/debugging/kgdb.rst). This routine is
// called to initialize the HW for later use by calling @poll_get_char or
// @poll_put_char.
//
// Optional: called only if provided, otherwise skipped as a non-polling
// driver.
//
// @poll_get_char: ``int ()(struct tty_driver *driver, int line)``
//
// kgdboc support (see @poll_init). @driver should read a character from a
// tty identified by @line and return it.
//
// Optional: called only if @poll_init provided.
//
// @poll_put_char: ``void ()(struct tty_driver *driver, int line, char ch)``
//
// kgdboc support (see @poll_init). @driver should write character @ch to
// a tty identified by @line.
//
// Optional: called only if @poll_init provided.
//
// @proc_show: ``int ()(struct seq_file *m, void *driver)``
//
// Driver @driver (cast to &struct tty_driver) can show additional info in
// /proc/tty/driver/<driver_name>. It is enough to fill in the information
// into @m.
//
// Optional: called only if provided, otherwise no /proc entry created.
//
// This structure defines the interface between the low-level tty driver and
// the tty routines. These routines can be defined. Unless noted otherwise,
// they are optional, and can be filled in with a %NULL pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_operations {
    pub idx): *mut *mut file filp, int,
    pub tty): *mut *mut *mut int (install)(struct tty_driver driver, struct tty_struct,
    pub tty): *mut *mut *mut void (remove)(struct tty_driver driver, struct tty_struct,
    pub filp): *mut *mut *mut *mut int (open)(struct tty_struct  tty, struct file,
    pub filp): *mut *mut *mut *mut void (close)(struct tty_struct  tty, struct file,
    pub tty): *mut *mut void (shutdown)(struct tty_struct,
    pub tty): *mut *mut void (cleanup)(struct tty_struct,
    pub count): *const *const *const *const ssize_t (write)(struct tty_struct tty, u8 buf, size_t,
    pub ch): *mut *mut *mut int (put_char)(struct tty_struct tty, u8,
    pub tty): *mut *mut void (flush_chars)(struct tty_struct,
    pub tty): *mut *mut unsigned int (write_room)(struct tty_struct,
    pub tty): *mut *mut unsigned int (chars_in_buffer)(struct tty_struct,
    pub arg): unsigned int cmd, unsigned long,
    pub arg): unsigned int cmd, unsigned long,
    pub old): *const *const *const void (set_termios)(struct tty_struct tty, struct ktermios,
    pub tty): *mut *mut *mut void (throttle)(struct tty_struct,
    pub tty): *mut *mut *mut void (unthrottle)(struct tty_struct,
    pub tty): *mut *mut void (stop)(struct tty_struct,
    pub tty): *mut *mut void (start)(struct tty_struct,
    pub tty): *mut *mut void (hangup)(struct tty_struct,
    pub state): *mut *mut *mut int (break_ctl)(struct tty_struct tty, int,
    pub tty): *mut *mut void (flush_buffer)(struct tty_struct,
    pub ldisc): *mut *mut *mut int (ldisc_ok)(struct tty_struct tty, int,
    pub tty): *mut *mut void (set_ldisc)(struct tty_struct,
    pub timeout): *mut *mut *mut void (wait_until_sent)(struct tty_struct tty, int,
    pub ch): *mut *mut *mut void (send_xchar)(struct tty_struct tty, u8,
    pub tty): *mut *mut int (tiocmget)(struct tty_struct,
    pub clear): unsigned int set, unsigned int,
    pub ws): *mut *mut *mut int (resize)(struct tty_struct tty, struct winsize,
    pub icount): *mut serial_icounter_struct,
    pub p): *mut *mut *mut int (get_serial)(struct tty_struct tty, struct serial_struct,
    pub p): *mut *mut *mut int (set_serial)(struct tty_struct tty, struct serial_struct,
    pub m): *mut *mut *mut void (show_fdinfo)(struct tty_struct tty, struct seq_file,

    pub options): *mut *mut *mut int (poll_init)(struct tty_driver driver, int line, char,
    pub line): *mut *mut *mut int (poll_get_char)(struct tty_driver driver, int,
    pub ch): *mut *mut *mut void (poll_put_char)(struct tty_driver driver, int line, char,

    pub driver): *mut *mut *mut int (proc_show)(struct seq_file m, void,
    pub __randomize_layout: },
//
// struct tty_driver -- driver for TTY devices
//
// @kref: reference counting. Reaching zero frees all the internals and the
// driver.
// @cdevs: allocated/registered character /dev devices
// @owner: modules owning this driver. Used drivers cannot be rmmod'ed.
// Automatically set by tty_alloc_driver().
// @driver_name: name of the driver used in /proc/tty
// @name: used for constructing /dev node name
// @name_base: used as a number base for constructing /dev node name
// @major: major /dev device number (zero for autoassignment)
// @minor_start: the first minor /dev device number
// @num: number of devices allocated
// @type: type of tty driver (enum tty_driver_type)
// @subtype: subtype of tty driver (enum tty_driver_subtype)
// @init_termios: termios to set to each tty initially (e.g. %tty_std_termios)
// @flags: tty driver flags (%TTY_DRIVER_)
// @proc_entry: proc fs entry, used internally
// @other: driver of the linked tty; only used for the PTY driver
// @flip_wq: workqueue to queue flip buffer work on
// @ttys: array of active &struct tty_struct, set by tty_standard_install()
// @ports: array of &struct tty_port; can be set during initialization by
// tty_port_link_device() and similar
// @termios: storage for termios at each TTY close for the next open
// @driver_state: pointer to driver's arbitrary data
// @ops: driver hooks for TTYs. Set them using tty_set_operations(). Use &struct
// tty_port helpers in them as much as possible.
// @tty_drivers: used internally to link tty_drivers together
//
// The usual handling of &struct tty_driver is to allocate it by
// tty_alloc_driver(), set up all the necessary members, and register it by
// tty_register_driver(). At last, the driver is torn down by calling
// tty_unregister_driver() followed by tty_driver_kref_put().
//
// The fields required to be set before calling tty_register_driver() include
// @driver_name, @name, @type, @subtype, @init_termios, and @ops.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tty_driver {
    pub kref: kref,
    pub cdevs: *mut cdev,
    pub owner: *mut module,
    pub driver_name: *const c_char,
    pub name: *const c_char,
    pub name_base: c_int,
    pub major: c_int,
    pub minor_start: c_int,
    pub num: c_uint,
    pub type: tty_driver_type,
    pub subtype: tty_driver_subtype,
    pub init_termios: ktermios,
    pub flags: c_ulong,
    pub proc_entry: *mut proc_dir_entry,
    pub other: *mut tty_driver,
    pub flip_wq: *mut workqueue_struct,
//
// Pointer to the tty data structures
//
    pub ttys: *mut tty_struct,
    pub ports: *mut tty_port,
    pub termios: *mut ktermios,
    pub driver_state: *mut c_void,
//
// Driver methods
//
    pub ops: *const tty_operations,
    pub tty_drivers: list_head,
    pub __randomize_layout: },
    pub tty_drivers: extern struct list_head,
    pub flags): c_ulong,
    pub line): *mut *mut *mut tty_driver tty_find_polling_driver(char name, int,
    pub driver): *mut void tty_driver_kref_put(struct tty_driver,
//
// tty_alloc_driver - allocate tty driver
// @lines: count of lines this driver can handle at most
// @flags: some of enum tty_driver_flag, will be set in driver->flags
//
// Returns: struct tty_driver or a PTR-encoded error (use IS_ERR() and friends).
//

    pub d: return,
    pub op: driver->ops =,
    pub driver): *mut int tty_register_driver(struct tty_driver,
    pub driver): *mut void tty_unregister_driver(struct tty_driver,
    pub dev): *mut device,
    pub attr_grp): *const attribute_group,
    pub index): *mut *mut void tty_unregister_device(struct tty_driver driver, unsigned,

    pub ): *mut void proc_tty_register_driver(struct tty_driver,
    pub ): *mut void proc_tty_unregister_driver(struct tty_driver,

