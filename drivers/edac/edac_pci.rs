//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/edac/edac_pci.h
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
// Defines, structures, APIs for edac_pci and edac_pci_sysfs
//
// (C) 2007 Linux Networx (http://lnxi.com)
// This file may be distributed under the terms of the
// GNU General Public License.
//
// Written by Thayne Harbaugh
// Based on work by Dan Hollis <goemon at anime dot net> and others.
// http://www.anime.net/~goemon/linux-ecc
//
// NMI handling support added by
// Dave Peterson <dsp@llnl.gov> <dave_peterson@pobox.com>
//
// Refactored for multi-source files:
// Doug Thompson <norsk5@xmission.com>
//
// Please look at Documentation/driver-api/edac.rst for more info about
// EDAC core structs and functions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_pci_counter {
    pub pe_count: core::sync::atomic::AtomicI32,
    pub npe_count: core::sync::atomic::AtomicI32,
}

//
// Abstract edac_pci control info structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_pci_ctl_info {
// for global list of edac_pci_ctl_info structs
    pub link: list_head,
    pub pci_idx: c_int,
// the internal state of this controller instance
    pub op_state: c_int,
// work struct for this instance
    pub work: delayed_work,
// pointer to edac polling checking routine:
// If NOT NULL: points to polling check routine
// If NULL: Then assumes INTERRUPT operation, where
// MC driver will receive events
//
    pub edac_dev): *mut *mut *mut void (edac_check) (struct edac_pci_ctl_info,
    pub /: *mut *mut *mut device dev; / pointer to device structure,
    pub /: *const *const *const char mod_name; / module name,
    pub /: *const *const *const char ctl_name; / edac controller name,
    pub /: *const *const *const char dev_name; / pci/platform/etc... name,
    pub /: *mut *mut *mut void pvt_info; / pointer to 'private driver' info,
    pub /: *mut *mut unsigned long start_time; / edac_pci load start time (jiffies),
// sysfs top name under 'edac' directory
// and instance name:
// cpu/cpu0/...
// cpu/cpu1/...
// cpu/cpu2/...
// ...
//
    pub 1]: char name[EDAC_DEVICE_NAME_LEN +,
// Event counters for the this whole EDAC Device
    pub counters: edac_pci_counter,
// edac sysfs device control for the 'name'
// device this structure controls
//
    pub kobj: kobject,
}

// write all or some bits in a byte-register
// write all or some bits in a word-register
//
// pci_write_bits32
//
// edac local routine to do pci_write_config_dword, but adds
// a mask parameter. If mask is all ones, ignore the mask.
// Otherwise utilize the mask to isolate specified bits
//
// write all or some bits in a dword-register
//

//
// edac_pci APIs
//
// edac_pci_alloc_ctl_info:
// The alloc() function for the 'edac_pci' control info
// structure.
//
// @sz_pvt: size of the private info at struct &edac_pci_ctl_info
// @edac_pci_name: name of the PCI device
//
// The chip driver will allocate one of these for each
// edac_pci it is going to control/register with the EDAC CORE.
//
// Returns: a pointer to struct &edac_pci_ctl_info on success; %NULL otherwise.
//
// edac_pci_free_ctl_info():
// Last action on the pci control structure.
//
// @pci: pointer to struct &edac_pci_ctl_info
//
// Calls the remove sysfs information, which will unregister
// this control struct's kobj. When that kobj's ref count
// goes to zero, its release function will be call and then
// kfree() the memory.
//
extern "C" {
    pub fn edac_pci_free_ctl_info(pci: *mut edac_pci_ctl_info);
}
//
// edac_pci_alloc_index: Allocate a unique PCI index number
//
// Returns:
// allocated index number
//
extern "C" {
    pub fn edac_pci_alloc_index() -> c_int;
}
//
// edac_pci_add_device(): Insert the 'edac_dev' structure into the
// edac_pci global list and create sysfs entries associated with
// edac_pci structure.
//
// @pci: pointer to the edac_device structure to be added to the list
// @edac_idx: A unique numeric identifier to be assigned to the
// 'edac_pci' structure.
//
// Returns:
// 0 on Success, or an error code on failure
//
extern "C" {
    pub fn edac_pci_add_device(pci: *mut edac_pci_ctl_info, edac_idx: c_int) -> c_int;
}
//
// edac_pci_del_device()
// Remove sysfs entries for specified edac_pci structure and
// then remove edac_pci structure from global list
//
// @dev:
// Pointer to 'struct device' representing edac_pci structure
// to remove
//
// Returns:
// Pointer to removed edac_pci structure,
// or %NULL if device not found
//
// edac_pci_create_generic_ctl()
// A generic constructor for a PCI parity polling device
// Some systems have more than one domain of PCI busses.
// For systems with one domain, then this API will
// provide for a generic poller.
//
// @dev: pointer to struct &device;
// @mod_name: name of the PCI device
//
// This routine calls the edac_pci_alloc_ctl_info() for
// the generic device, with default values
//
// Returns: Pointer to struct &edac_pci_ctl_info on success, %NULL on
// failure.
//
// edac_pci_release_generic_ctl
// The release function of a generic EDAC PCI polling device
//
// @pci: pointer to struct &edac_pci_ctl_info
//
extern "C" {
    pub fn edac_pci_release_generic_ctl(pci: *mut edac_pci_ctl_info);
}
//
// edac_pci_create_sysfs
// Create the controls/attributes for the specified EDAC PCI device
//
// @pci: pointer to struct &edac_pci_ctl_info
//
extern "C" {
    pub fn edac_pci_create_sysfs(pci: *mut edac_pci_ctl_info) -> c_int;
}
//
// edac_pci_remove_sysfs()
// remove the controls and attributes for this EDAC PCI device
//
// @pci: pointer to struct &edac_pci_ctl_info
//
extern "C" {
    pub fn edac_pci_remove_sysfs(pci: *mut edac_pci_ctl_info);
}
