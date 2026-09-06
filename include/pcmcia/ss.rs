//! Automatically rewritten from C Header to Rust Module
//! Source: include/pcmcia/ss.h
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
// ss.h
//
// The initial developer of the original code is David A. Hinds
// <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
// are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
//
// (C) 1999             David A. Hinds
//

// Definitions for card status flags for GetStatus
pub const SS_WRPROT: c_uint = 0x0001;
pub const SS_CARDLOCK: c_uint = 0x0002;
pub const SS_EJECTION: c_uint = 0x0004;
pub const SS_INSERTION: c_uint = 0x0008;
pub const SS_BATDEAD: c_uint = 0x0010;
pub const SS_BATWARN: c_uint = 0x0020;
pub const SS_READY: c_uint = 0x0040;
pub const SS_DETECT: c_uint = 0x0080;
pub const SS_POWERON: c_uint = 0x0100;
pub const SS_GPI: c_uint = 0x0200;
pub const SS_STSCHG: c_uint = 0x0400;
pub const SS_CARDBUS: c_uint = 0x0800;
pub const SS_3VCARD: c_uint = 0x1000;
pub const SS_XVCARD: c_uint = 0x2000;
pub const SS_PENDING: c_uint = 0x4000;
pub const SS_ZVCARD: c_uint = 0x8000;
// InquireSocket capabilities
pub const SS_CAP_PAGE_REGS: c_uint = 0x0001;
pub const SS_CAP_VIRTUAL_BUS: c_uint = 0x0002;
pub const SS_CAP_MEM_ALIGN: c_uint = 0x0004;
pub const SS_CAP_STATIC_MAP: c_uint = 0x0008;
pub const SS_CAP_PCCARD: c_uint = 0x4000;
pub const SS_CAP_CARDBUS: c_uint = 0x8000;
// for GetSocket, SetSocket
// Socket configuration flags
pub const SS_PWR_AUTO: c_uint = 0x0010;
pub const SS_IOCARD: c_uint = 0x0020;
pub const SS_RESET: c_uint = 0x0040;
pub const SS_DMA_MODE: c_uint = 0x0080;
pub const SS_SPKR_ENA: c_uint = 0x0100;
pub const SS_OUTPUT_ENA: c_uint = 0x0200;
// Flags for I/O port and memory windows
pub const MAP_ACTIVE: c_uint = 0x01;
pub const MAP_16BIT: c_uint = 0x02;
pub const MAP_AUTOSZ: c_uint = 0x04;
pub const MAP_0WS: c_uint = 0x08;
pub const MAP_WRPROT: c_uint = 0x10;
pub const MAP_ATTRIB: c_uint = 0x20;
pub const MAP_USE_WAIT: c_uint = 0x40;
pub const MAP_PREFETCH: c_uint = 0x80;
// Use this just for bridge windows
pub const MAP_IOSPACE: c_uint = 0x20;
// power hook operations
pub const HOOK_POWER_PRE: c_uint = 0x01;
pub const HOOK_POWER_POST: c_uint = 0x02;
// Maximum number of IO windows per socket
pub const MAX_IO_WIN: c_int = 2;
// Maximum number of memory windows per socket
pub const MAX_WIN: c_int = 4;
//
// Socket operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pccard_operations {
    pub s): *mut *mut int (init)(struct pcmcia_socket,
    pub s): *mut *mut int (suspend)(struct pcmcia_socket,
    pub value): *mut *mut *mut int (get_status)(struct pcmcia_socket s, u_int,
    pub state): *mut *mut *mut int (set_socket)(struct pcmcia_socket s, socket_state_t,
    pub io): *mut *mut *mut int (set_io_map)(struct pcmcia_socket s, struct pccard_io_map,
    pub mem): *mut *mut *mut int (set_mem_map)(struct pcmcia_socket s, struct pccard_mem_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmcia_socket {
    pub owner: *mut module,
    pub socket: socket_state_t,
    pub state: u_int,
    pub /: *mut *mut u_int suspended_state; / state before suspend,
    pub functions: u_short,
    pub lock_count: u_short,
    pub cis_mem: pccard_mem_map,
    pub cis_virt: *mut void __iomem,
    pub io: [io_window_t; MAX_IO_WIN],
    pub win: [pccard_mem_map; MAX_WIN],
    pub cis_cache: list_head,
    pub fake_cis_len: usize,
    pub fake_cis: *mut u8,
    pub socket_list: list_head,
    pub socket_released: completion,
// deprecated
    pub /: *mut *mut unsigned int sock; / socket number,
// socket capabilities
    pub features: u_int,
    pub irq_mask: u_int,
    pub map_size: u_int,
    pub io_offset: u_int,
    pub pci_irq: u_int,
    pub cb_dev: *mut pci_dev,
// socket setup is done so resources should be able to be allocated.
// Only if set to 1, calls to find_{io,mem}_region are handled, and
// insertio events are actually managed by the PCMCIA layer.
    pub resource_setup_done: u8,
// socket operations
    pub ops: *mut pccard_operations,
    pub resource_ops: *mut pccard_resource_ops,
    pub resource_data: *mut c_void,
// Zoom video behaviour is so chip specific its not worth adding
// so is power hook
    pub operation): *mut *mut *mut int (power_hook)(struct pcmcia_socket sock, int,
// allows tuning the CB bridge before loading driver for the CB card

    pub bus): *mut *mut *mut void (tune_bridge)(struct pcmcia_socket sock, struct pci_bus,

// state thread
    pub thread: *mut task_struct,
    pub thread_done: completion,
    pub thread_events: c_uint,
    pub sysfs_events: c_uint,
// For the non-trivial interaction between these locks,
// see Documentation/pcmcia/locking.rst
    pub skt_mutex: mutex,
    pub ops_mutex: mutex,
// protects thread_events and sysfs_events
    pub thread_lock: spinlock_t,
// pcmcia (16-bit)
    pub callback: *mut pcmcia_callback,

// The following elements refer to 16-bit PCMCIA devices inserted
// into the socket
    pub devices_list: list_head,
// the number of devices, used only internally and subject to
// incorrectness and change
    pub device_count: u8,
// does the PCMCIA card consist of two pseudo devices?
    pub pcmcia_pfc: u8,
// non-zero if PCMCIA card is present
    pub present: core::sync::atomic::AtomicI32,
// IRQ to be used by PCMCIA devices. May not be IRQ 0.
    pub pcmcia_irq: c_uint,

// socket device
    pub dev: device,
// data internal to the socket driver
    pub driver_data: *mut c_void,
// status of the card during resume from a system sleep state
    pub resume_status: c_int,
}

// socket drivers must define the resource operations type they use. There
// are two options:
// - pccard_static_ops		iomem and ioport areas are assigned statically
// - pccard_nonstatic_ops	iomem and ioport areas are assigned dynamically.
// If this option is selected, use
// "select PCCARD_NONSTATIC" in Kconfig.
//

// If PCMCIA is not used, but only CARDBUS, these functions are not used
// at all. Therefore, do not use the large (240K!) rsrc_nonstatic module
//

// socket drivers use this callback in their IRQ handler
// to register and unregister a socket
extern "C" {
    pub fn pcmcia_register_socket(socket: *mut pcmcia_socket) -> c_int;
}
extern "C" {
    pub fn pcmcia_unregister_socket(socket: *mut pcmcia_socket);
}
