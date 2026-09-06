//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/w1.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2004 Evgeniy Polyakov <zbr@ioremap.net>
//

//
// struct w1_reg_num - broken out slave device id
//
// @family: identifies the type of device
// @id: along with family is the unique device id
// @crc: checksum of the other bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_reg_num {

}

pub const W1_MAXNAMELEN: c_int = 32;
pub const W1_SEARCH: c_uint = 0xF0;
pub const W1_ALARM_SEARCH: c_uint = 0xEC;
pub const W1_CONVERT_TEMP: c_uint = 0x44;
pub const W1_SKIP_ROM: c_uint = 0xCC;
pub const W1_COPY_SCRATCHPAD: c_uint = 0x48;
pub const W1_WRITE_SCRATCHPAD: c_uint = 0x4E;
pub const W1_READ_SCRATCHPAD: c_uint = 0xBE;
pub const W1_READ_ROM: c_uint = 0x33;
pub const W1_READ_PSUPPLY: c_uint = 0xB4;
pub const W1_MATCH_ROM: c_uint = 0x55;
pub const W1_RESUME_CMD: c_uint = 0xA5;
//
// struct w1_slave - holds a single slave device on the bus
//
// @owner: Points to the one wire "wire" kernel module.
// @name: Device id is ascii.
// @w1_slave_entry: data for the linked list
// @reg_num: the slave id in binary
// @refcnt: reference count, delete when 0
// @flags: bit flags for W1_SLAVE_ACTIVE W1_SLAVE_DETACH
// @ttl: decrement per search this slave isn't found, deatch at 0
// @master: bus which this slave is on
// @family: module for device family type
// @family_data: pointer for use by the family module
// @dev: kernel device identifier
// @hwmon: pointer to hwmon device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_slave {
    pub owner: *mut module,
    pub name: [c_uchar; W1_MAXNAMELEN],
    pub w1_slave_entry: list_head,
    pub reg_num: w1_reg_num,
    pub refcnt: core::sync::atomic::AtomicI32,
    pub ttl: c_int,
    pub flags: c_ulong,
    pub master: *mut w1_master,
    pub family: *mut w1_family,
    pub family_data: *mut c_void,
    pub dev: device,
    pub hwmon: *mut device,
}

extern "C" {
    pub fn void(: *mut *mut w1_slave_found_callback)(struct w1_master, _arg: u64) -> typedef;
}
//
// struct w1_bus_master - operations available on a bus master
//
// @data: the first parameter in all the functions below
//
// @read_bit: Sample the line level
// @return the level read (0 or 1)
//
// @write_bit: Sets the line level
//
// @touch_bit: the lowest-level function for devices that really support the
// 1-wire protocol.
// touch_bit(0) = write-0 cycle
// touch_bit(1) = write-1 / read cycle
// @return the bit read (0 or 1)
//
// @read_byte: Reads a byte. Same as 8 touch_bit(1) calls.
// @return the byte read
//
// @write_byte: Writes a byte. Same as 8 touch_bit(x) calls.
//
// @read_block: Same as a series of read_byte() calls
// @return the number of bytes read
//
// @write_block: Same as a series of write_byte() calls
//
// @triplet: Combines two reads and a smart write for ROM searches
// @return bit0=Id bit1=comp_id bit2=dir_taken
//
// @reset_bus: long write-0 with a read for the presence pulse detection
// @return -1=Error, 0=Device present, 1=No device present
//
// @set_pullup: Put out a strong pull-up pulse of the specified duration.
// @return -1=Error, 0=completed
//
// @search: Really nice hardware can handle the different types of ROM search
// w1_master* is passed to the slave found callback.
// u8 is search_type, W1_SEARCH or W1_ALARM_SEARCH
//
// @dev_id: Optional device id string, which w1 slaves could use for
// creating names, which then give a connection to the w1 master
//
// Note: read_bit and write_bit are very low level functions and should only
// be used with hardware that doesn't really support 1-wire operations,
// like a parallel/serial port.
// Either define read_bit and write_bit OR define, at minimum, touch_bit and
// reset_bus.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_bus_master {
    pub data: *mut c_void,
    pub ): *mut *mut u8 (read_bit)(void,
    pub u8): *mut *mut *mut void (write_bit)(void ,,
    pub u8): *mut *mut *mut u8 (touch_bit)(void ,,
    pub ): *mut *mut u8 (read_byte)(void,
    pub u8): *mut *mut *mut void (write_byte)(void ,,
    pub int): *mut *mut *mut *mut u8 (read_block)(void , u8 ,,
    pub int): *const *const *const *const void (write_block)(void , u8 ,,
    pub u8): *mut *mut *mut u8 (triplet)(void ,,
    pub ): *mut *mut u8 (reset_bus)(void,
    pub int): *mut *mut *mut u8 (set_pullup)(void ,,
    pub w1_slave_found_callback): u8,,
    pub dev_id: *mut c_char,
}

//
// enum w1_master_flags - bitfields used in w1_master.flags
// @W1_ABORT_SEARCH: abort searching early on shutdown
// @W1_WARN_MAX_COUNT: limit warning when the maximum count is reached
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum w1_master_flags {
    W1_ABORT_SEARCH = 0,
    W1_WARN_MAX_COUNT = 1,
}

//
// struct w1_master - one per bus master
// @w1_master_entry:	master linked list
// @owner:		module owner
// @name:		dynamically allocate bus name
// @list_mutex:		protect slist and async_list
// @slist:		linked list of slaves
// @async_list:		linked list of netlink commands to execute
// @max_slave_count:	maximum number of slaves to search for at a time
// @slave_count:	current number of slaves known
// @attempts:		number of searches ran
// @slave_ttl:		number of searches before a slave is timed out
// @initialized:	prevent init/removal race conditions
// @id:			w1 bus number
// @search_count:	number of automatic searches to run, -1 unlimited
// @search_id:		allows continuing a search
// @refcnt:		reference count
// @priv:		private data storage
// @enable_pullup:	allows a strong pullup
// @pullup_duration:	time for the next strong pullup
// @flags:		one of w1_master_flags
// @thread:		thread for bus search and netlink commands
// @mutex:		protect most of w1_master
// @bus_mutex:		pretect concurrent bus access
// @driver:		sysfs driver
// @dev:		sysfs device
// @bus_master:		io operations available
// @seq:		sequence number used for netlink broadcasts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_master {
    pub w1_master_entry: list_head,
    pub owner: *mut module,
    pub name: [c_uchar; W1_MAXNAMELEN],
// list_mutex protects just slist and async_list so slaves can be
// searched for and async commands added while the master has
// w1_master.mutex locked and is operating on the bus.
// lock order w1_mlock, w1_master.mutex, w1_master.list_mutex
//
    pub list_mutex: mutex,
    pub slist: list_head,
    pub async_list: list_head,
    pub slave_count: int max_slave_count,,
    pub attempts: c_ulong,
    pub slave_ttl: c_int,
    pub initialized: c_int,
    pub id: u32,
    pub search_count: c_int,
// id to start searching on, to continue a search or 0 to restart
    pub search_id: u64,
    pub refcnt: core::sync::atomic::AtomicI32,
    pub priv: *mut c_void,
// 5V strong pullup enabled flag, 1 enabled, zero disabled.
    pub enable_pullup: c_int,
// 5V strong pullup duration in milliseconds, zero disabled.
    pub pullup_duration: c_int,
    pub flags: c_long,
    pub thread: *mut task_struct,
    pub mutex: mutex,
    pub bus_mutex: mutex,
    pub driver: *mut device_driver,
    pub dev: device,
    pub bus_master: *mut w1_bus_master,
    pub seq: u32,
}

extern "C" {
    pub fn w1_add_master_device(master: *mut w1_bus_master) -> c_int;
}
extern "C" {
    pub fn w1_remove_master_device(master: *mut w1_bus_master);
}
//
// struct w1_family_ops - operations for a family type
// @add_slave: add_slave
// @remove_slave: remove_slave
// @groups: sysfs group
// @chip_info: pointer to struct hwmon_chip_info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_family_ops {
    pub sl): *mut *mut int (add_slave)(struct w1_slave,
    pub sl): *mut *mut void (remove_slave)(struct w1_slave,
    pub groups: *const attribute_group,
    pub chip_info: *const hwmon_chip_info,
}

//
// struct w1_family - reference counted family structure.
// @family_entry:	family linked list
// @fid:		8 bit family identifier
// @fops:		operations for this family
// @of_match_table: open firmware match table
// @refcnt:		reference counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_family {
    pub family_entry: list_head,
    pub fid: u8,
    pub fops: *const w1_family_ops,
    pub of_match_table: *const of_device_id,
    pub refcnt: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn w1_register_family(family: *mut w1_family) -> c_int;
}
extern "C" {
    pub fn w1_unregister_family(family: *mut w1_family);
}
//
// module_w1_family() - Helper macro for registering a 1-Wire families
// @__w1_family: w1_family struct
//
// Helper macro for 1-Wire families which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

extern "C" {
    pub fn w1_triplet(dev: *mut w1_master, bdir: c_int) -> u8;
}
extern "C" {
    pub fn w1_touch_bit(dev: *mut w1_master, bit: c_int) -> u8;
}
extern "C" {
    pub fn w1_write_8(: *mut w1_master, _arg: u8);
}
extern "C" {
    pub fn w1_read_8(: *mut w1_master) -> u8;
}
extern "C" {
    pub fn w1_reset_bus(: *mut w1_master) -> c_int;
}
extern "C" {
    pub fn w1_calc_crc8(: *mut u8, _arg: c_int) -> u8;
}
extern "C" {
    pub fn w1_write_block(: *mut w1_master, : *const u8, _arg: c_int);
}
extern "C" {
    pub fn w1_touch_block(: *mut w1_master, : *mut u8, _arg: c_int);
}
extern "C" {
    pub fn w1_read_block(: *mut w1_master, : *mut u8, _arg: c_int) -> u8;
}
extern "C" {
    pub fn w1_reset_select_slave(sl: *mut w1_slave) -> c_int;
}
extern "C" {
    pub fn w1_reset_resume_command(: *mut w1_master) -> c_int;
}
extern "C" {
    pub fn w1_next_pullup(: *mut w1_master, _arg: c_int);
}
extern "C" {
    pub fn container_of(_arg: dev, w1_slave: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn dev_to_w1_slave(_arg: container_of(kobj, device: struct, _arg: kobj)) -> return;
}
extern "C" {
    pub fn container_of(_arg: dev, w1_master: struct, _arg: dev) -> return;
}

