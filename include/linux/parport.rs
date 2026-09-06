//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/parport.h
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


//
// Any part of this program may be used in documents licensed under
// the GNU Free Documentation License, Version 1.1 or any later version
// published by the Free Software Foundation.
//

// Define this later.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pc_parport_state {
    pub ctr: c_uint,
    pub ecr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax_parport_state {
    pub ctr: c_uint,
    pub ecr: c_uint,
    pub dcsr: c_uint,
}

// used by both parport_amiga and parport_mfc3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amiga_parport_state {
    pub /: *mut *mut unsigned char data; / ciaa.prb,
    pub /: *mut *mut unsigned char datadir; / ciaa.ddrb,
    pub /: *mut *mut unsigned char status; / ciab.pra & 7,
    pub /: *mut *mut unsigned char statusdir;/ ciab.ddrb & 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip32_parport_state {
    pub dcr: c_uint,
    pub ecr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parport_state {
    pub pc: pc_parport_state,
// ARC has no state.
    pub ax: ax_parport_state,
    pub amiga: amiga_parport_state,
// Atari has not state.
    pub ip32: ip32_parport_state,
    pub misc: *mut c_void,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parport_operations {
// IBM PC-style virtual registers.
    pub char): *mut *mut *mut void (write_data)(struct parport , unsigned,
    pub ): *mut *mut unsigned char (read_data)(struct parport,
    pub char): *mut *mut *mut void (write_control)(struct parport , unsigned,
    pub ): *mut *mut unsigned char (read_control)(struct parport,
    pub val): c_uchar,
    pub ): *mut *mut unsigned char (read_status)(struct parport,
// IRQs.
    pub ): *mut *mut void (enable_irq)(struct parport,
    pub ): *mut *mut void (disable_irq)(struct parport,
// Data direction.
    pub ): *mut *mut void (data_forward) (struct parport,
    pub ): *mut *mut void (data_reverse) (struct parport,
// For core parport code.
    pub ): *mut *mut *mut void (init_state)(struct pardevice , struct parport_state,
    pub ): *mut *mut *mut void (save_state)(struct parport , struct parport_state,
    pub ): *mut *mut *mut void (restore_state)(struct parport , struct parport_state,
// Block read/write
    pub flags): size_t len, int,
    pub flags): c_int,
    pub flags): size_t len, int,
    pub flags): c_int,
    pub flags): size_t len, int,
    pub flags): c_int,
    pub flags): size_t len, int,
    pub flags): size_t len, int,
    pub flags): size_t len, int,
    pub flags): size_t len, int,
    pub owner: *mut module,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parport_device_info {
    pub class: parport_device_class,
    pub class_name: *const c_char,
    pub mfr: *const c_char,
    pub model: *const c_char,
    pub cmdset: *const c_char,
    pub description: *const c_char,
}

// Each device can have two callback functions:
// 1) a preemption function, called by the resource manager to request
// that the driver relinquish control of the port.  The driver should
// return zero if it agrees to release the port, and nonzero if it
// refuses.  Do not call parport_release() - the kernel will do this
// implicitly.
//
// 2) a wake-up function, called by the resource manager to tell drivers
// that the port is available to be claimed.  If a driver wants to use
// the port, it should call parport_claim() here.
//
// A parallel port device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pardevice {
    pub name: *const c_char,
    pub port: *mut parport,
    pub daisy: c_int,
    pub ): *mut *mut int (preempt)(void,
    pub ): *mut *mut void (wakeup)(void,
    pub private: *mut c_void,
    pub ): *mut *mut void (irq_func)(void,
    pub flags: c_uint,
    pub next: *mut pardevice,
    pub prev: *mut pardevice,
    pub dev: device,
    pub devmodel: bool,
    pub /: *mut *mut *mut parport_state state; / saved status over preemption,
    pub wait_q: wait_queue_head_t,
    pub time: unsigned long int,
    pub timeslice: unsigned long int,
    pub timeout: volatile long int,
    pub /: *mut *mut unsigned long waiting; / long req'd for set_bit --RR,
    pub waitprev: *mut pardevice,
    pub waitnext: *mut pardevice,
    pub sysctl_table: *mut *mut c_void,
}

// IEEE1284 information
// IEEE1284 phases. These are exposed to userland through ppdev IOCTL
// PP[GS]ETPHASE, so do not change existing values.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee1284_phase {
    IEEE1284_PH_FWD_DATA,
    IEEE1284_PH_FWD_IDLE,
    IEEE1284_PH_TERMINATE,
    IEEE1284_PH_NEGOTIATION,
    IEEE1284_PH_HBUSY_DNA,
    IEEE1284_PH_REV_IDLE,
    IEEE1284_PH_HBUSY_DAVAIL,
    IEEE1284_PH_REV_DATA,
    IEEE1284_PH_ECP_SETUP,
    IEEE1284_PH_ECP_FWD_TO_REV,
    IEEE1284_PH_ECP_REV_TO_FWD,
    IEEE1284_PH_ECP_DIR_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee1284_info {
    pub mode: c_int,
    pub phase: volatile enum ieee1284_phase,
    pub irq: semaphore,
}

// A parallel port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parport {
    pub /: *mut *mut unsigned long base; / base address,
    pub /: *mut *mut unsigned long base_hi; / base address (hi - ECR),
    pub /: *mut *mut unsigned int size; / IO extent,
    pub name: *const c_char,
    pub modes: c_uint,
    pub /: *mut *mut int irq; / interrupt (or -1 for none),
    pub dma: c_int,
    pub /: *mut *mut int muxport; / which muxport (if any) this is,
    pub /: *mut *mut int portnum; / which physical parallel port (not mux),
    pub IO/DMA.: *mut *mut *mut device dev; / Physical device associated with,
// This may unfortulately be null if the
// port has a legacy driver.
//
    pub /: *mut *mut device bus_dev; / to link with the bus,
    pub physport: *mut parport,
// If this is a non-default mux
    pub devices: *mut pardevice,
    pub /: *mut *mut *mut pardevice cad; / port owner,
    pub /: *mut *mut int daisy; / currently selected daisy addr,
    pub /: *mut *mut int muxsel; / currently selected mux port,
    pub waithead: *mut pardevice,
    pub waittail: *mut pardevice,
    pub list: list_head,
    pub timer: timer_list,
    pub flags: c_uint,
    pub sysctl_table: *mut c_void,
    pub /: *mut *mut parport_device_info probe_info[5]; / 0-3 + non-IEEE1284.3,
    pub ieee1284: ieee1284_info,
    pub ops: *mut parport_operations,
    pub /: *mut *mut *mut void private_data; / for lowlevel driver,
    pub /: *mut *mut int number; / port index - the `n' in `parportn',
    pub pardevice_lock: spinlock_t,
    pub waitlist_lock: spinlock_t,
    pub cad_lock: rwlock_t,
    pub spintime: c_int,
    pub ref_count: core::sync::atomic::AtomicI32,
    pub devflags: c_ulong,
pub const PARPORT_DEVPROC_REGISTERED: c_int = 0;
pub const PARPORT_ANNOUNCED: c_int = 1;
    pub /: *mut *mut *mut pardevice proc_device; / Currently register proc device,
    pub full_list: list_head,
    pub slaves: [*mut parport; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parport_driver {
    pub name: *const c_char,
    pub ): *mut *mut void (detach) (struct parport,
    pub ): *mut *mut void (match_port)(struct parport,
    pub ): *mut *mut int (probe)(struct pardevice,
    pub driver: device_driver,
}

extern "C" {
    pub fn parport_bus_init() -> c_int;
}
extern "C" {
    pub fn parport_bus_exit();
}
// parport_register_port registers a new parallel port at the given
// Once a registered port is ready for high-level drivers to use, the
extern "C" {
    pub fn parport_announce_port(port: *mut parport);
}
// Unregister a port.
extern "C" {
    pub fn parport_remove_port(port: *mut parport);
}
// Register a new high-level driver.
//
// parport_register_driver must be a macro so that KBUILD_MODNAME can
// be expanded
//
// parport_register_driver - register a parallel port device driver
// @driver: structure describing the driver
//
// This can be called by a parallel port device driver in order
// to receive notifications about ports being found in the
// system, as well as ports no longer available.
//
// The @driver structure is allocated by the caller and must not be
// deallocated until after calling parport_unregister_driver().
//
// If using the non device model:
// The driver's attach() function may block.  The port that
// attach() is given will be valid for the duration of the
// callback, but if the driver wants to take a copy of the
// pointer it must call parport_get_port() to do so.  Calling
// parport_register_device() on that port will do this for you.
//
// The driver's detach() function may block.  The port that
// detach() is given will be valid for the duration of the
// callback, but if the driver wants to take a copy of the
// pointer it must call parport_get_port() to do so.
//
// Returns 0 on success. The non device model will always succeeds.
// but the new device model can fail and will return the error code.
//

// Unregister a high-level driver.
extern "C" {
    pub fn parport_unregister_driver(: *mut parport_driver);
}
//
// module_parport_driver() - Helper macro for registering a modular parport driver
// @__parport_driver: struct parport_driver to be used
//
// Helper macro for parport drivers which do not do anything special in module
// init and exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit().
//

// If parport_register_driver doesn't fit your needs, perhaps
// parport_find_xxx does.
// generic irq handler, if it suits your needs
extern "C" {
    pub fn parport_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
// Reference counting for ports.
extern "C" {
    pub fn parport_put_port(: *mut parport);
}
extern "C" {
    pub fn parport_del_port(: *mut parport);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pardev_cb {
    pub ): *mut *mut int (preempt)(void,
    pub ): *mut *mut void (wakeup)(void,
    pub private: *mut c_void,
    pub ): *mut *mut void (irq_func)(void,
    pub flags: c_uint,
}

//
// parport_register_dev_model declares that a device is connected to a
// port, and tells the kernel all it needs to know.
//
// parport_unregister unlinks a device from the chain.
extern "C" {
    pub fn parport_unregister_device(dev: *mut pardevice);
}
// parport_claim tries to gain ownership of the port for a particular
extern "C" {
    pub fn parport_claim(dev: *mut pardevice) -> c_int;
}
// parport_claim_or_block is the same, but sleeps if the port cannot
extern "C" {
    pub fn parport_claim_or_block(dev: *mut pardevice) -> c_int;
}
// parport_release reverses a previous parport_claim.  This can never
extern "C" {
    pub fn parport_release(dev: *mut pardevice);
}
//
// parport_yield - relinquish a parallel port temporarily
// @dev: a device on the parallel port
//
// This function relinquishes the port if it would be helpful to other
// drivers to do so.  Afterwards it tries to reclaim the port using
// parport_claim(), and the return value is the same as for
// parport_claim().  If it fails, the port is left unclaimed and it is
// the driver's responsibility to reclaim the port.
//
// The parport_yield() and parport_yield_blocking() functions are for
// marking points in the driver at which other drivers may claim the
// port and use their devices.  Yielding the port is similar to
// releasing it and reclaiming it, but is more efficient because no
// action is taken if there are no other devices needing the port.  In
// fact, nothing is done even if there are other devices waiting but
// the current device is still within its "timeslice".  The default
// timeslice is half a second, but it can be adjusted via the /proc
// interface.
//
extern "C" {
    pub fn parport_claim(_arg: dev) -> return;
}
//
// parport_yield_blocking - relinquish a parallel port temporarily
// @dev: a device on the parallel port
//
// This function relinquishes the port if it would be helpful to other
// drivers to do so.  Afterwards it tries to reclaim the port using
// parport_claim_or_block(), and the return value is the same as for
// parport_claim_or_block().
//
extern "C" {
    pub fn parport_claim_or_block(_arg: dev) -> return;
}
// Flags used to identify what a device does.

// IEEE1284 functions
extern "C" {
    pub fn parport_ieee1284_interrupt(: *mut c_void);
}
extern "C" {
    pub fn parport_negotiate(: *mut parport, mode: c_int) -> c_int;
}
extern "C" {
    pub fn parport_write(: *mut parport, buf: *const c_void, len: usize) -> isize;
}
extern "C" {
    pub fn parport_read(: *mut parport, buf: *mut c_void, len: usize) -> isize;
}
pub const PARPORT_INACTIVITY_O_NONBLOCK: c_int = 1;
extern "C" {
    pub fn parport_set_timeout(: *mut pardevice, inactivity: c_long) -> c_long;
}
extern "C" {
    pub fn parport_wait_event(: *mut parport, timeout: c_long) -> c_int;
}
// For architectural drivers
// IEEE1284.3 functions

extern "C" {
    pub fn parport_daisy_init(port: *mut parport) -> c_int;
}
extern "C" {
    pub fn parport_daisy_fini(port: *mut parport);
}
extern "C" {
    pub fn parport_close(dev: *mut pardevice);
}
extern "C" {
    pub fn parport_device_id(devnum: c_int, buffer: *mut c_char, len: usize) -> isize;
}
extern "C" {
    pub fn parport_daisy_deselect_all(port: *mut parport);
}
extern "C" {
    pub fn parport_daisy_select(port: *mut parport, daisy: c_int, mode: c_int) -> c_int;
}
// Lowlevel drivers _can_ call this support function to handle irqs.
// Prototypes from parport_procfs
extern "C" {
    pub fn parport_proc_register(pp: *mut parport) -> c_int;
}
extern "C" {
    pub fn parport_proc_unregister(pp: *mut parport) -> c_int;
}
extern "C" {
    pub fn parport_device_proc_register(device: *mut pardevice) -> c_int;
}
extern "C" {
    pub fn parport_device_proc_unregister(device: *mut pardevice) -> c_int;
}
// If PC hardware is the only type supported, we can optimise a bit.

// Generic operations vector through the dispatch table.

