//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/edac/edac_device.h
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
// Defines, structures, APIs for edac_device
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

//
// The following are the structures to provide for a generic
// or abstract 'edac_device'. This set of structures and the
// code that implements the APIs for the same, provide for
// registering EDAC type devices which are NOT standard memory.
//
// CPU caches (L1 and L2)
// DMA engines
// Core CPU switches
// Fabric switch units
// PCIe interface controllers
// other EDAC/ECC type devices that can be monitored for
// errors, etc.
//
// It allows for a 2 level set of hierarchy. For example:
//
// cache could be composed of L1, L2 and L3 levels of cache.
// Each CPU core would have its own L1 cache, while sharing
// L2 and maybe L3 caches.
//
// View them arranged, via the sysfs presentation:
// /sys/devices/system/edac/..
//
// mc/		<existing memory device directory>
// cpu/cpu0/..	<L1 and L2 block directory>
// /L1-cache/ce_count
// /ue_count
// /L2-cache/ce_count
// /ue_count
// cpu/cpu1/..	<L1 and L2 block directory>
// /L1-cache/ce_count
// /ue_count
// /L2-cache/ce_count
// /ue_count
// ...
//
// the L1 and L2 directories would be "edac_device_block's"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_device_counter {
    pub ue_count: u32,
    pub ce_count: u32,
}

// forward reference
// edac_dev_sysfs_attribute structure
// used for driver sysfs attributes in mem_ctl_info
// for extra controls and attributes:
// like high level error Injection controls
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_dev_sysfs_attribute {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct edac_device_ctl_info , char,
    pub size_t): *const *const *const *const ssize_t (store)(struct edac_device_ctl_info , char ,,
}

// edac_dev_sysfs_block_attribute structure
//
// used in leaf 'block' nodes for adding controls/attributes
//
// each block in each instance of the containing control structure can
// have an array of the following. The show function will be filled in
// with the show function in the low level driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_dev_sysfs_block_attribute {
    pub attr: attribute,
    pub ): *mut *mut *mut *mut ssize_t (show)(struct kobject , struct attribute , char,
}

// device block control structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_device_block {
    pub /: *mut *mut *mut edac_device_instance instance; / Up Pointer,
    pub 1]: char name[EDAC_DEVICE_NAME_LEN +,
    pub /: *mut *mut edac_device_counter counters; / basic UE and CE counters,
    pub /: *mut *mut int nr_attribs; / how many attributes,
// this block's attributes, could be NULL
    pub block_attributes: *mut edac_dev_sysfs_block_attribute,
// edac sysfs device control
    pub kobj: kobject,
}

// device instance control structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_device_instance {
    pub /: *mut *mut *mut edac_device_ctl_info ctl; / Up pointer,
    pub 4]: char name[EDAC_DEVICE_NAME_LEN +,
    pub /: *mut *mut edac_device_counter counters; / instance counters,
    pub /: *mut *mut u32 nr_blocks; / how many blocks,
    pub /: *mut *mut *mut edac_device_block blocks; / block array,
// edac sysfs device control
    pub kobj: kobject,
}

//
// Abstract edac_device control info structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edac_device_ctl_info {
// for global list of edac_device_ctl_info structs
    pub link: list_head,
    pub /: *mut *mut *mut module owner; / Module owner of this control struct,
    pub dev_idx: c_int,
// Per instance controls for this edac_device
    pub /: *mut *mut int log_ue; / boolean for logging UEs,
    pub /: *mut *mut int log_ce; / boolean for logging CEs,
    pub /: *mut *mut int panic_on_ue; / boolean for panic'ing on an UE,
    pub /: *mut *mut unsigned poll_msec; / number of milliseconds to poll interval,
    pub /: *mut *mut unsigned long delay; / number of jiffies for poll_msec,
// Additional top controller level attributes, but specified
// by the low level driver.
//
// Set by the low level driver to provide attributes at the
// controller level, same level as 'ue_count' and 'ce_count' above.
// An array of structures, NULL terminated
//
// If attributes are desired, then set to array of attributes
// If no attributes are desired, leave NULL
//
    pub sysfs_attributes: *const edac_dev_sysfs_attribute,
// pointer to main 'edac' subsys in sysfs
    pub edac_subsys: *const bus_type,
// the internal state of this controller instance
    pub op_state: c_int,
// work struct for this instance
    pub work: delayed_work,
// pointer to edac polling checking routine:
// If NOT NULL: points to polling check routine
// If NULL: Then assumes INTERRUPT operation, where
// MC driver will receive events
//
    pub edac_dev): *mut *mut *mut void (edac_check) (struct edac_device_ctl_info,
    pub /: *mut *mut *mut device dev; / pointer to device structure,
    pub /: *const *const *const char mod_name; / module name,
    pub /: *const *const *const char ctl_name; / edac controller name,
    pub /: *const *const *const char dev_name; / pci/platform/etc... name,
    pub /: *mut *mut *mut void pvt_info; / pointer to 'private driver' info,
    pub /: *mut *mut unsigned long start_time; / edac_device load start time (jiffies),
// sysfs top name under 'edac' directory
// and instance name:
// cpu/cpu0/...
// cpu/cpu1/...
// cpu/cpu2/...
// ...
//
    pub 1]: char name[EDAC_DEVICE_NAME_LEN +,
// Number of instances supported on this control structure
// and the array of those instances
//
    pub nr_instances: u32,
    pub instances: *mut edac_device_instance,
    pub blocks: *mut edac_device_block,
// Event counters for the this whole EDAC Device
    pub counters: edac_device_counter,
// edac sysfs device control for the 'name'
// device this structure controls
//
    pub kobj: kobject,
}

// To get from the instance's wq to the beginning of the ctl structure

//
// The alloc() and free() functions for the 'edac_device' control info
// structure. A MC driver will allocate one of these for each edac_device
// it is going to control/register with the EDAC CORE.
//
// The offset value can be:
// -1 indicating no offset value
// 0 for zero-based block numbers
// 1 for 1-based block number
// other for other-based block number
//

extern "C" {
    pub fn edac_device_free_ctl_info(ctl_info: *mut edac_device_ctl_info);
}
//
// edac_device_add_device - Insert the 'edac_dev' structure into the
// edac_device global list and create sysfs entries associated with
// edac_device structure.
//
// @edac_dev: pointer to edac_device structure to be added to the list
// 'edac_device' structure.
//
// Returns:
// 0 on Success, or an error code on failure
//
extern "C" {
    pub fn edac_device_add_device(edac_dev: *mut edac_device_ctl_info) -> c_int;
}
//
// edac_device_del_device - Remove sysfs entries for specified edac_device
// structure and then remove edac_device structure from global list
//
// @dev:
// Pointer to struct &device representing the edac device
// structure to remove.
//
// Returns:
// Pointer to removed edac_device structure,
// or %NULL if device not found.
//
// edac_device_handle_ce_count - Log correctable errors.
//
// @edac_dev: pointer to struct &edac_device_ctl_info
// @inst_nr: number of the instance where the CE error happened
// @count: Number of errors to log.
// @block_nr: number of the block where the CE error happened
// @msg: message to be printed
//
// edac_device_handle_ue_count - Log uncorrectable errors.
//
// @edac_dev: pointer to struct &edac_device_ctl_info
// @inst_nr: number of the instance where the CE error happened
// @count: Number of errors to log.
// @block_nr: number of the block where the CE error happened
// @msg: message to be printed
//
// edac_device_handle_ce(): Log a single correctable error
//
// @edac_dev: pointer to struct &edac_device_ctl_info
// @inst_nr: number of the instance where the CE error happened
// @block_nr: number of the block where the CE error happened
// @msg: message to be printed
//
// edac_device_handle_ue(): Log a single uncorrectable error
//
// @edac_dev: pointer to struct &edac_device_ctl_info
// @inst_nr: number of the instance where the UE error happened
// @block_nr: number of the block where the UE error happened
// @msg: message to be printed
//
// edac_device_alloc_index: Allocate a unique device index number
//
// Returns:
// allocated index number
//
extern "C" {
    pub fn edac_device_alloc_index() -> c_int;
}
// Free the actual struct
