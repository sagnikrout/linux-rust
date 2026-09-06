//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/include/gpib_types.h
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
// copyright		   : (C) 2002 by Frank Mori Hess
//

// config parameters that are only used by driver attach functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_board_config {
// firmware blob
    pub init_data: *mut c_void,
    pub init_data_length: c_int,
// IO base address to use for non-pnp cards (set by core, driver should make local copy)
    pub ibbase: u32,
    pub mmibbase: *mut void __iomem,
// IRQ to use for non-pnp cards (set by core, driver should make local copy)
    pub ibirq: c_uint,
// dma channel to use for non-pnp cards (set by core, driver should make local copy)
    pub ibdma: c_uint,
//
// pci bus of card, useful for distinguishing multiple identical pci cards
// (negative means don't care)
//
    pub pci_bus: c_int,
//
// pci slot of card, useful for distinguishing multiple identical pci cards
// (negative means don't care)
//
    pub pci_slot: c_int,
// sysfs device path of hardware to attach
    pub device_path: *mut c_char,
// serial number of hardware to attach
    pub serial_number: *mut c_char,
}

//
// struct gpib_interface defines the interface
// between the board-specific details dealt with in the drivers
// and generic interface provided by gpib-common.
// This really should be in a different header file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_interface {
// name of board
    pub name: *mut c_char,
// attach() initializes board and allocates resources
    pub config): *const *const *const int (attach)(struct gpib_board board, struct gpib_board_config,
// detach() shuts down board and frees resources
    pub board): *mut *mut void (detach)(struct gpib_board,
//
// read() should read at most 'length' bytes from the bus into
// 'buffer'.  It should return when it fills the buffer or
// encounters an END (EOI and or EOS if appropriate).  It should set 'end'
// to be nonzero if the read was terminated by an END, otherwise 'end'
// should be zero.
// Ultimately, this will be changed into or replaced by an asynchronous
// read.  Zero return value for success, negative
// return indicates error.
// nbytes returns number of bytes read
//
    pub bytes_read): *mut usize,
//
// write() should write 'length' bytes from buffer to the bus.
// If the boolean value send_eoi is nonzero, then EOI should
// be sent along with the last byte.  Returns number of bytes
// written or negative value on error.
//
    pub bytes_written): *mut usize,
//
// command() writes the command bytes in 'buffer' to the bus
// Returns zero on success or negative value on error.
//
    pub bytes_written): *mut usize,
//
// Take control (assert ATN).  If 'asyncronous' is nonzero, take
// control asyncronously (assert ATN immediately without waiting
// for other processes to complete first).  Should not return
// until board becomes controller in charge.  Returns zero no success,
// nonzero on error.
//
    pub asyncronous): *mut *mut *mut int (take_control)(struct gpib_board board, int,
//
// De-assert ATN.  Returns zero on success, nonzer on error.
//
    pub board): *mut *mut int (go_to_standby)(struct gpib_board,
// request/release control of the IFC and REN lines (system controller)
    pub request_control): *mut *mut *mut int (request_system_control)(struct gpib_board board, int,
//
// Asserts or de-asserts 'interface clear' (IFC) depending on
// boolean value of 'assert'
//
    pub assert): *mut *mut *mut void (interface_clear)(struct gpib_board board, int,
//
// Sends remote enable command if 'enable' is nonzero, disables remote mode
// if 'enable' is zero
//
    pub enable): *mut *mut *mut void (remote_enable)(struct gpib_board board, int,
//
// enable END for reads, when byte 'eos' is received.  If
// 'compare_8_bits' is nonzero, then all 8 bits are compared
// with the eos bytes.	Otherwise only the 7 least significant
// bits are compared.
//
    pub compare_8_bits): *mut *mut *mut int (enable_eos)(struct gpib_board board, u8 eos, int,
// disable END on eos byte (END on EOI only)
    pub board): *mut *mut void (disable_eos)(struct gpib_board,
// configure parallel poll
    pub configuration): *mut *mut *mut void (parallel_poll_configure)(struct gpib_board board, u8,
// conduct parallel poll
    pub result): *mut *mut *mut int (parallel_poll)(struct gpib_board board, u8,
// set/clear ist (individual status bit)
    pub ist): *mut *mut *mut void (parallel_poll_response)(struct gpib_board board, int,
// select local parallel poll configuration mode PP2 versus remote PP1
    pub local): *mut *mut *mut void (local_parallel_poll_mode)(struct gpib_board board, int,
//
// Returns current status of the bus lines.  Should be set to
// NULL if your board does not have the ability to query the
// state of the bus lines.
//
    pub board): *const *const int (line_status)(struct gpib_board,
//
// updates and returns the board's current status.
// The meaning of the bits are specified in gpib_user.h
// in the IBSTA section.  The driver does not need to
// worry about setting the CMPL, END, TIMO, or ERR bits.
//
    pub clear_mask): *mut *mut *mut unsigned int (update_status)(struct gpib_board board, unsigned int,
//
// Sets primary address 0-30 for gpib interface card.
//
    pub address): *mut *mut *mut int (primary_address)(struct gpib_board board, unsigned int,
//
// Sets and enables, or disables secondary address 0-30
// for gpib interface card.
//
    pub enable): c_int,
//
// Sets the byte the board should send in response to a serial poll.
// This function should also start or stop requests for service via
// IEEE 488.2 reqt/reqf, based on MSS (bit 6 of the status_byte).
// If the more flexible serial_poll_response2 is implemented by the
// driver, then this method should be left NULL since it will not
// be used.  This method can generate spurious service requests
// which are allowed by IEEE 488.2, but not ideal.
//
// This method should implement the serial poll response method described
// by IEEE 488.2 section 11.3.3.4.3 "Allowed Coupled Control of
// STB, reqt, and reqf".
//
    pub status_byte): *mut *mut *mut void (serial_poll_response)(struct gpib_board board, u8,
//
// Sets the byte the board should send in response to a serial poll.
// This function should also request service via IEEE 488.2 reqt/reqf
// based on MSS (bit 6 of the status_byte) and new_reason_for_service.
// reqt should be set true if new_reason_for_service is true,
// and reqf should be set true if MSS is false.	 This function
// will never be called with MSS false and new_reason_for_service
// true simultaneously, so don't worry about that case.
//
// This method implements the serial poll response method described
// by IEEE 488.2 section 11.3.3.4.1 "Preferred Implementation".
//
// If this method is left NULL by the driver, then the user library
// function ibrsv2 will not work.
//
    pub new_reason_for_service): c_int,
//
// returns the byte the board will send in response to a serial poll.
//
    pub board): *mut *mut u8 (serial_poll_status)(struct gpib_board,
// adjust T1 delay
    pub nano_sec): *mut *mut *mut int (t1_delay)(struct gpib_board board, unsigned int,
// go to local mode
    pub board): *mut *mut void (return_to_local)(struct gpib_board,
// board does not support 7 bit eos comparisons
    pub 1: unsigned no_7_bit_eos :,
// skip check for listeners before trying to send command bytes
    pub 1: unsigned skip_check_for_command_acceptors :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_event_queue {
    pub event_head: list_head,
    pub list: spinlock_t lock; // for access to event,
    pub num_events: c_uint,
    pub 1: unsigned dropped_event :,
}

// struct for supporting polling operation when irq is not available
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_pseudo_irq {
    pub timer: timer_list,
    pub arg): *mut *mut irqreturn_t (handler)(int irq, void,
    pub board: *mut gpib_board,
    pub active: core::sync::atomic::AtomicI32,
}

// list so we can make a linked list of drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_interface_list {
    pub list: list_head,
    pub interface: *mut gpib_interface,
    pub module: *mut module,
}

//
// One struct gpib_board is allocated for each physical board in the computer.
// It provides storage for variables local to each board, and interface
// functions for performing operations on the board
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_board {
// functions used by this board
    pub interface: *mut gpib_interface,
//
// Pointer to module whose use count we should increment when
// interface is in use
//
    pub provider_module: *mut module,
// buffer used to store read/write data for this board
    pub buffer: *mut u8,
// length of buffer
    pub buffer_length: c_uint,
//
// Used to hold the board's current status (see update_status() above)
//
    pub status: c_ulong,
//
// Driver should only sleep on this wait queue.	 It is special in that the
// core will wake this queue and set the TIMO bit in 'status' when the
// watchdog timer times out.
//
    pub wait: wait_queue_head_t,
//
// Lock that only allows one process to access this board at a time.
// Has to be first in any locking order, since it can be locked over
// multiple ioctls.
//
    pub user_mutex: mutex,
//
// Mutex which compensates for removal of "big kernel lock" from kernel.
// Should not be held for extended waits.
//
    pub big_gpib_mutex: mutex,
// pid of last process to lock the board mutex
    pub locking_pid: pid_t,
// lock for setting locking pid
    pub locking_pid_spinlock: spinlock_t,
// Spin lock for dealing with races with the interrupt handler
    pub spinlock: spinlock_t,
// Watchdog timer to enable timeouts
    pub timer: timer_list,
// device of attached driver if any
    pub dev: *mut device,
// gpib_common device gpibN
    pub gpib_dev: *mut device,
//
// 'private_data' can be used as seen fit by the driver to
// store additional variables for this board
//
    pub private_data: *mut c_void,
// Number of open file descriptors using this board
    pub use_count: c_uint,
// list of open devices connected to this board
    pub device_list: list_head,
// primary address
    pub pad: c_uint,
// secondary address
    pub sad: c_int,
// timeout for io operations, in microseconds
    pub usec_timeout: c_uint,
// board's parallel poll configuration byte
    pub parallel_poll_configuration: u8,
// t1 delay we are using
    pub t1_nano_sec: c_uint,
// Count that keeps track of whether board is up and running or not
    pub online: c_uint,
// number of processes trying to autopoll
    pub autospollers: c_int,
// autospoll kernel thread
    pub autospoll_task: *mut task_struct,
// queue for recording received trigger/clear/ifc events
    pub event_queue: gpib_event_queue,
// minor number for this board's device file
    pub minor: c_int,
// struct to deal with polling mode
    pub pseudo_irq: gpib_pseudo_irq,
// error dong autopoll
    pub stuck_srq: core::sync::atomic::AtomicI32,
    pub config: gpib_board_config,
// Flag that indicates whether board is system controller of the bus
    pub 1: unsigned master :,
// individual status bit
    pub 1: unsigned ist :,
//
// one means local parallel poll mode ieee 488.1 PP2 (or no parallel poll PP0),
// zero means remote parallel poll configuration mode ieee 488.1 PP1
//
    pub 1: unsigned local_ppoll_mode :,
}

// element of event queue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_event {
    pub list: list_head,
    pub event_type: c_short,
}

//
// Each board has a list of gpib_status_queue to keep track of all open devices
// on the bus, so we know what address to poll when we get a service request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_status_queue {
// list_head so we can make a linked list of devices
    pub list: list_head,
    pub /: *mut *mut unsigned int pad; / primary gpib address,
    pub /: *mut *mut int sad; / secondary gpib address (negative means disabled),
// stores serial poll bytes for this device
    pub status_bytes: list_head,
    pub num_status_bytes: c_uint,
// number of times this address is opened
    pub reference_count: c_uint,
// flags loss of status byte error due to limit on size of queue
    pub 1: unsigned dropped_byte :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_status_byte {
    pub list: list_head,
    pub poll_byte: u8,
}

extern "C" {
    pub fn init_gpib_status_queue(device: *mut gpib_status_queue);
}
// Used to store device-descriptor-specific information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_descriptor {
    pub /: *mut *mut unsigned int pad; / primary gpib address,
    pub /: *mut *mut int sad; / secondary gpib address (negative means disabled),
    pub io_in_progress: core::sync::atomic::AtomicI32,
//
// Kernel-only reference count to prevent descriptor from being
// freed while IO handlers hold a pointer to it.  Incremented
// before each IO operation, decremented when done.  Unlike
// io_in_progress, this cannot be modified from userspace via
// general_ibstatus().
//
    pub descriptor_busy: core::sync::atomic::AtomicI32,
    pub 1: unsigned is_board :,
    pub 1: unsigned autopoll_enabled :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpib_file_private {
    pub holding_mutex: core::sync::atomic::AtomicI32,
    pub descriptors: [*mut gpib_descriptor; GPIB_MAX_NUM_DESCRIPTORS],
// locked while descriptors are being allocated/deallocated
    pub descriptors_mutex: mutex,
    pub 1: unsigned got_module :,
}

