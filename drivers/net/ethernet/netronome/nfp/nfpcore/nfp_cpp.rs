//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_cpp.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2015-2018 Netronome Systems, Inc.
//
// nfp_cpp.h
// Interface for low-level NFP CPP access.
// Authors: Jason McMullan <jason.mcmullan@netronome.com>
// Rolf Neugebauer <rolf.neugebauer@netronome.com>
//

pub const PCI_64BIT_BAR_COUNT: c_int = 3;
pub const NFP_CPP_NUM_TARGETS: c_int = 16;
// Max size of area it should be safe to request

// NFP_MUTEX_WAIT_* are timeouts in seconds when waiting for a mutex
pub const NFP_MUTEX_WAIT_FIRST_WARN: c_int = 15;
pub const NFP_MUTEX_WAIT_NEXT_WARN: c_int = 5;
pub const NFP_MUTEX_WAIT_ERROR: c_int = 60;
// Wildcard indicating a CPP read or write action
//
// The action used will be either read or write depending on whether a
// read or write instruction/call is performed on the NFP_CPP_ID.  It
// is recomended that the RW action is used even if all actions to be
// performed on a NFP_CPP_ID are known to be only reads or writes.
// Doing so will in many cases save NFP CPP internal software
// resources.
//
pub const NFP_CPP_ACTION_RW: c_int = 32;
pub const NFP_CPP_TARGET_ID_MASK: c_uint = 0x1f;

//
// NFP_CPP_ID() - pack target, token, and action into a CPP ID.
// @target:     NFP CPP target id
// @action:     NFP CPP action id
// @token:      NFP CPP token id
//
// Create a 32-bit CPP identifier representing the access to be made.
// These identifiers are used as parameters to other NFP CPP
// functions.  Some CPP devices may allow wildcard identifiers to be
// specified.
//
// Return:      NFP CPP ID
//

//
// NFP_CPP_ISLAND_ID() - pack target, token, action, and island into a CPP ID.
// @target:     NFP CPP target id
// @action:     NFP CPP action id
// @token:      NFP CPP token id
// @island:     NFP CPP island id
//
// Create a 32-bit CPP identifier representing the access to be made.
// These identifiers are used as parameters to other NFP CPP
// functions.  Some CPP devices may allow wildcard identifiers to be
// specified.
//
// Return:      NFP CPP ID
//

//
// NFP_CPP_ID_TARGET_of() - Return the NFP CPP target of a NFP CPP ID
// @id:         NFP CPP ID
//
// Return:      NFP CPP target
//
// NFP_CPP_ID_TOKEN_of() - Return the NFP CPP token of a NFP CPP ID
// @id:         NFP CPP ID
// Return:      NFP CPP token
//
// NFP_CPP_ID_ACTION_of() - Return the NFP CPP action of a NFP CPP ID
// @id:         NFP CPP ID
//
// Return:      NFP CPP action
//
// NFP_CPP_ID_ISLAND_of() - Return the NFP CPP island of a NFP CPP ID
// @id: NFP CPP ID
//
// Return:      NFP CPP island
//
// NFP Interface types - logical interface for this CPP connection
// 4 bits are reserved for interface type.
//
pub const NFP_CPP_INTERFACE_TYPE_INVALID: c_uint = 0x0;
pub const NFP_CPP_INTERFACE_TYPE_PCI: c_uint = 0x1;
pub const NFP_CPP_INTERFACE_TYPE_ARM: c_uint = 0x2;
pub const NFP_CPP_INTERFACE_TYPE_RPC: c_uint = 0x3;
pub const NFP_CPP_INTERFACE_TYPE_ILA: c_uint = 0x4;
//
// NFP_CPP_INTERFACE() - Construct a 16-bit NFP Interface ID
// @type:       NFP Interface Type
// @unit:       Unit identifier for the interface type
// @channel:    Channel identifier for the interface unit
//
// Interface IDs consists of 4 bits of interface type,
// 4 bits of unit identifier, and 8 bits of channel identifier.
//
// The NFP Interface ID is used in the implementation of
// NFP CPP API mutexes, which use the MU Atomic CompareAndWrite
// operation - hence the limit to 16 bits to be able to
// use the NFP Interface ID as a lock owner.
//
// Return:      Interface ID
//

//
// NFP_CPP_INTERFACE_TYPE_of() - Get the interface type
// @interface:  NFP Interface ID
// Return:      NFP Interface ID's type
//

//
// NFP_CPP_INTERFACE_UNIT_of() - Get the interface unit
// @interface:  NFP Interface ID
// Return:      NFP Interface ID's unit
//

//
// NFP_CPP_INTERFACE_CHANNEL_of() - Get the interface channel
// @interface:  NFP Interface ID
// Return:      NFP Interface ID's channel
//

// Implemented in nfp_cppcore.c
extern "C" {
    pub fn nfp_cpp_free(cpp: *mut nfp_cpp);
}
extern "C" {
    pub fn nfp_cpp_model(cpp: *mut nfp_cpp) -> u32;
}
extern "C" {
    pub fn nfp_cpp_interface(cpp: *mut nfp_cpp) -> u16;
}
extern "C" {
    pub fn nfp_cpp_serial(cpp: *mut nfp_cpp, serial: *const u8) -> c_int;
}
extern "C" {
    pub fn nfp_cpp_mu_locality_lsb(cpp: *mut nfp_cpp) -> c_uint;
}
extern "C" {
    pub fn nfp_cpp_area_free(area: *mut nfp_cpp_area);
}
extern "C" {
    pub fn nfp_cpp_area_acquire(area: *mut nfp_cpp_area) -> c_int;
}
extern "C" {
    pub fn nfp_cpp_area_acquire_nonblocking(area: *mut nfp_cpp_area) -> c_int;
}
extern "C" {
    pub fn nfp_cpp_area_release(area: *mut nfp_cpp_area);
}
extern "C" {
    pub fn nfp_cpp_area_release_free(area: *mut nfp_cpp_area);
}
extern "C" {
    pub fn nfp_cpp_area_size(area: *mut nfp_cpp_area) -> usize;
}
extern "C" {
    pub fn nfp_cpp_area_phys(area: *mut nfp_cpp_area) -> phys_addr_t;
}
extern "C" {
    pub fn nfp_xpb_readl(cpp: *mut nfp_cpp, xpb_tgt: u32, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn nfp_xpb_writel(cpp: *mut nfp_cpp, xpb_tgt: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn nfp_xpb_writelm(cpp: *mut nfp_cpp, xpb_tgt: u32, mask: u32, value: u32) -> c_int;
}
// Implemented in nfp_cpplib.c
extern "C" {
    pub fn nfp_cpp_mutex_free(mutex: *mut nfp_cpp_mutex);
}
extern "C" {
    pub fn nfp_cpp_mutex_lock(mutex: *mut nfp_cpp_mutex) -> c_int;
}
extern "C" {
    pub fn nfp_cpp_mutex_unlock(mutex: *mut nfp_cpp_mutex) -> c_int;
}
extern "C" {
    pub fn nfp_cpp_mutex_trylock(mutex: *mut nfp_cpp_mutex) -> c_int;
}
//
// nfp_cppcore_pcie_unit() - Get PCI Unit of a CPP handle
// @cpp:	CPP handle
//
// Return: PCI unit for the NFP CPP handle
//
extern "C" {
    pub fn NFP_CPP_INTERFACE_UNIT_of(_arg: nfp_cpp_interface(cpp)) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_cpp_explicit_command {
    pub cpp_id: u32,
    pub data_ref: u16,
    pub data_master: u8,
    pub len: u8,
    pub byte_mask: u8,
    pub signal_master: u8,
    pub signal_ref: u8,
    pub posted: u8,
    pub siga: u8,
    pub sigb: u8,
    pub siga_mode: i8,
    pub sigb_mode: i8,
}

pub const NFP_SERIAL_LEN: c_int = 6;
//
// struct nfp_cpp_operations - NFP CPP operations structure
// @area_priv_size:     Size of the nfp_cpp_area private data
// @owner:              Owner module
// @init:               Initialize the NFP CPP bus
// @free:               Free the bus
// @read_serial:	Read serial number to memory provided
// @get_interface:	Return CPP interface
// @area_init:          Initialize a new NFP CPP area (not serialized)
// @area_cleanup:       Clean up a NFP CPP area (not serialized)
// @area_acquire:       Acquire the NFP CPP area (serialized)
// @area_release:       Release area (serialized)
// @area_resource:      Get resource range of area (not serialized)
// @area_phys:          Get physical address of area (not serialized)
// @area_iomem:         Get iomem of area (not serialized)
// @area_read:          Perform a read from a NFP CPP area (serialized)
// @area_write:         Perform a write to a NFP CPP area (serialized)
// @explicit_priv_size: Size of an explicit's private area
// @explicit_acquire:   Acquire an explicit area
// @explicit_release:   Release an explicit area
// @explicit_put:       Write data to send
// @explicit_get:       Read data received
// @explicit_do:        Perform the transaction
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_cpp_operations {
    pub area_priv_size: usize,
    pub owner: *mut module,
    pub cpp): *mut *mut int (init)(struct nfp_cpp,
    pub cpp): *mut *mut void (free)(struct nfp_cpp,
    pub serial): *mut *mut *mut int (read_serial)(struct device dev, u8,
    pub dev): *mut *mut int (get_interface)(struct device,
    pub size): c_ulong,
    pub area): *mut *mut void (area_cleanup)(struct nfp_cpp_area,
    pub area): *mut *mut int (area_acquire)(struct nfp_cpp_area,
    pub area): *mut *mut void (area_release)(struct nfp_cpp_area,
    pub area): *mut *mut *mut resource (area_resource)(nfp_cpp_area,
    pub area): *mut *mut phys_addr_t (area_phys)(struct nfp_cpp_area,
    pub area): *mut *mut *mut void __iomem (area_iomem)(struct nfp_cpp_area,
    pub length): unsigned long offset, unsigned int,
    pub length): unsigned long offset, unsigned int,
    pub explicit_priv_size: usize,
    pub expl): *mut *mut int (explicit_acquire)(struct nfp_cpp_explicit,
    pub expl): *mut *mut void (explicit_release)(struct nfp_cpp_explicit,
    pub len): *const *const void buff, size_t,
    pub len): *mut *mut void buff, size_t,
    pub address): u64,
}

extern "C" {
    pub fn nfp_cpp_area_cache_add(cpp: *mut nfp_cpp, size: usize) -> c_int;
}
// The following section contains extensions to the
// NFP CPP API, to be used in a Linux kernel-space context.
//
// Use this channel ID for multiple virtual channel interfaces
// (ie ARM and PCIe) when setting up the interface field.
//
pub const NFP_CPP_INTERFACE_CHANNEL_PEROPENER: c_int = 255;
// Return code masks for nfp_cpp_explicit_do()
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_cpp_explicit_signal_mode {
    NFP_SIGNAL_NONE = 0,
    NFP_SIGNAL_PUSH = 1,
    NFP_SIGNAL_PUSH_OPTIONAL = -1,
    NFP_SIGNAL_PULL = 2,
    NFP_SIGNAL_PULL_OPTIONAL = -2,
}

extern "C" {
    pub fn nfp_cpp_explicit_do(expl: *mut nfp_cpp_explicit, address: u64) -> c_int;
}
extern "C" {
    pub fn nfp_cpp_explicit_get(expl: *mut nfp_cpp_explicit, buff: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn nfp_cpp_explicit_release(expl: *mut nfp_cpp_explicit);
}
// Implemented in nfp_cpplib.c
extern "C" {
    pub fn nfp_cpp_model_autodetect(cpp: *mut nfp_cpp, model: *mut u32) -> c_int;
}
