//! Automatically rewritten from C Header to Rust Module
//! Source: include/pcmcia/ds.h
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
// ds.h -- 16-bit PCMCIA core support
//
// The initial developer of the original code is David A. Hinds
// <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
// are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
//
// (C) 1999		David A. Hinds
// (C) 2003 - 2008	Dominik Brodowski
//

//
// PCMCIA device drivers (16-bit cards only; 32-bit cards require CardBus
// a.k.a. PCI drivers
//
// dynamic device IDs for PCMCIA device drivers. See
// Documentation/pcmcia/driver.rst for details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmcia_dynids {
    pub lock: mutex,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmcia_driver {
    pub name: *const c_char,
    pub dev): *mut *mut int (probe) (struct pcmcia_device,
    pub dev): *mut *mut void (remove) (struct pcmcia_device,
    pub dev): *mut *mut int (suspend) (struct pcmcia_device,
    pub dev): *mut *mut int (resume) (struct pcmcia_device,
    pub owner: *mut module,
    pub id_table: *const pcmcia_device_id,
    pub drv: device_driver,
    pub dynids: pcmcia_dynids,
}

// driver registration
extern "C" {
    pub fn pcmcia_register_driver(driver: *mut pcmcia_driver) -> c_int;
}
extern "C" {
    pub fn pcmcia_unregister_driver(driver: *mut pcmcia_driver);
}
//
// module_pcmcia_driver() - Helper macro for registering a pcmcia driver
// @__pcmcia_driver: pcmcia_driver struct
//
// Helper macro for pcmcia drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only use
// this macro once, and calling it replaces module_init() and module_exit().
//

// for struct resource * array embedded in struct pcmcia_device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmcia_device {
// the socket and the device_no [for multifunction devices]
    pub socket: *mut pcmcia_socket,
    pub devname: *mut c_char,
    pub device_no: u8,
// the hardware "function" device; certain subdevices can
// share one hardware "function" device.
    pub func: u8,
    pub function_config: *mut config_t,
    pub socket_device_list: list_head,
// device setup
    pub irq: c_uint,
    pub resource: [*mut resource; PCMCIA_NUM_RESOURCES],
    pub /: *mut *mut resource_size_t card_addr; / for the 1st IOMEM resource,
    pub vpp: c_uint,
    pub /: *mut *mut unsigned int config_flags; / CONF_ENABLE_ flags below,
    pub config_base: c_uint,
    pub config_index: c_uint,
    pub /: *mut *mut unsigned int config_regs; / PRESENT_ flags below,
    pub /: *mut *mut unsigned int io_lines; / number of I/O lines,
// Is the device suspended?
    pub suspended:1: u16,
// Flags whether io, irq, win configurations were
// requested, and whether the configuration is "locked"
    pub _irq:1: u16,
    pub _io:1: u16,
    pub _win:4: u16,
    pub _locked:1: u16,
// Flag whether a "fuzzy" func_id based match is
// allowed.
    pub allow_func_id_match:1: u16,
// information about this device
    pub has_manf_id:1: u16,
    pub has_card_id:1: u16,
    pub has_func_id:1: u16,
    pub reserved:4: u16,
    pub func_id: u8,
    pub manf_id: u16,
    pub card_id: u16,
    pub prod_id: [*mut c_char; 4],
    pub dma_mask: u64,
    pub dev: device,
// data private to drivers
    pub priv: *mut c_void,
    pub open: c_uint,
}

//
// CIS access.
//
// Please use the following functions to access CIS tuples:
// - pcmcia_get_tuple()
// - pcmcia_loop_tuple()
// - pcmcia_get_mac_from_cis()
//
// To parse a tuple_t, pcmcia_parse_tuple() exists. Its interface
// might change in future.
//
// get the very first CIS entry of type @code. Note that buf is pointer
// to u8 *buf; and that you need to kfree(buf) afterwards.
// loop over CIS entries
// get the MAC address from CISTPL_FUNCE
// parse a tuple_t
extern "C" {
    pub fn pcmcia_parse_tuple(tuple: *mut tuple_t, parse: *mut cisparse_t) -> c_int;
}
// loop CIS entries for valid configuration
// is the device still there?
// low-level interface reset
extern "C" {
    pub fn pcmcia_reset_card(skt: *mut pcmcia_socket) -> c_int;
}
// CIS config
extern "C" {
    pub fn pcmcia_read_config_byte(p_dev: *mut pcmcia_device, where: off_t, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn pcmcia_write_config_byte(p_dev: *mut pcmcia_device, where: off_t, val: u8) -> c_int;
}
// device configuration
extern "C" {
    pub fn pcmcia_request_io(p_dev: *mut pcmcia_device) -> c_int;
}
extern "C" {
    pub fn pcmcia_enable_device(p_dev: *mut pcmcia_device) -> c_int;
}
extern "C" {
    pub fn pcmcia_release_window(p_dev: *mut pcmcia_device, res: *mut resource) -> c_int;
}
extern "C" {
    pub fn pcmcia_fixup_vpp(p_dev: *mut pcmcia_device, new_vpp: c_uchar) -> c_int;
}
extern "C" {
    pub fn pcmcia_fixup_iowidth(p_dev: *mut pcmcia_device) -> c_int;
}
extern "C" {
    pub fn pcmcia_disable_device(p_dev: *mut pcmcia_device);
}
// IO ports
pub const IO_DATA_PATH_WIDTH: c_uint = 0x18;
pub const IO_DATA_PATH_WIDTH_8: c_uint = 0x00;
pub const IO_DATA_PATH_WIDTH_16: c_uint = 0x08;
pub const IO_DATA_PATH_WIDTH_AUTO: c_uint = 0x10;
// IO memory
pub const WIN_MEMORY_TYPE_CM: c_uint = 0x00 /* default */;
pub const WIN_MEMORY_TYPE_AM: c_uint = 0x20 /* MAP_ATTRIB */;
pub const WIN_DATA_WIDTH_8: c_uint = 0x00 /* default */;
pub const WIN_DATA_WIDTH_16: c_uint = 0x02 /* MAP_16BIT */;
pub const WIN_ENABLE: c_uint = 0x01 /* MAP_ACTIVE */;
pub const WIN_USE_WAIT: c_uint = 0x40 /* MAP_USE_WAIT */;
pub const WIN_FLAGS_MAP: c_uint = 0x63 /* MAP_ATTRIB | MAP_16BIT | MAP_ACTIVE |;
pub const WIN_FLAGS_REQ: c_uint = 0x1c /* mapping to socket->win[i]:;
// config_reg{ister}s present for this PCMCIA device
pub const PRESENT_OPTION: c_uint = 0x001;
pub const PRESENT_STATUS: c_uint = 0x002;
pub const PRESENT_PIN_REPLACE: c_uint = 0x004;
pub const PRESENT_COPY: c_uint = 0x008;
pub const PRESENT_EXT_STATUS: c_uint = 0x010;
pub const PRESENT_IOBASE_0: c_uint = 0x020;
pub const PRESENT_IOBASE_1: c_uint = 0x040;
pub const PRESENT_IOBASE_2: c_uint = 0x080;
pub const PRESENT_IOBASE_3: c_uint = 0x100;
pub const PRESENT_IOSIZE: c_uint = 0x200;
// flags to be passed to pcmcia_enable_device()
pub const CONF_ENABLE_IRQ: c_uint = 0x0001;
pub const CONF_ENABLE_SPKR: c_uint = 0x0002;
pub const CONF_ENABLE_PULSE_IRQ: c_uint = 0x0004;
pub const CONF_ENABLE_ESR: c_uint = 0x0008;
pub const CONF_ENABLE_IOCARD: c_uint = 0x0010 /* auto-enabled if IO resources or IRQ;
// (CONF_ENABLE_IRQ) in use
pub const CONF_ENABLE_ZVCARD: c_uint = 0x0020;
// flags used by pcmcia_loop_config() autoconfiguration
pub const CONF_AUTO_CHECK_VCC: c_uint = 0x0100 /* check for matching Vcc? */;
pub const CONF_AUTO_SET_VPP: c_uint = 0x0200 /* set Vpp? */;
pub const CONF_AUTO_AUDIO: c_uint = 0x0400 /* enable audio line? */;
pub const CONF_AUTO_SET_IO: c_uint = 0x0800 /* set ->resource[0,1] */;
pub const CONF_AUTO_SET_IOMEM: c_uint = 0x1000 /* set ->resource[2] */;

