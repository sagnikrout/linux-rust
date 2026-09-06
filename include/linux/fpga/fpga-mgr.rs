//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fpga/fpga-mgr.h
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
// FPGA Framework
//
// Copyright (C) 2013-2016 Altera Corporation
// Copyright (C) 2017 Intel Corporation
//

//
// enum fpga_mgr_states - fpga framework states
// @FPGA_MGR_STATE_UNKNOWN: can't determine state
// @FPGA_MGR_STATE_POWER_OFF: FPGA power is off
// @FPGA_MGR_STATE_POWER_UP: FPGA reports power is up
// @FPGA_MGR_STATE_RESET: FPGA in reset state
// @FPGA_MGR_STATE_FIRMWARE_REQ: firmware request in progress
// @FPGA_MGR_STATE_FIRMWARE_REQ_ERR: firmware request failed
// @FPGA_MGR_STATE_PARSE_HEADER: parse FPGA image header
// @FPGA_MGR_STATE_PARSE_HEADER_ERR: Error during PARSE_HEADER stage
// @FPGA_MGR_STATE_WRITE_INIT: preparing FPGA for programming
// @FPGA_MGR_STATE_WRITE_INIT_ERR: Error during WRITE_INIT stage
// @FPGA_MGR_STATE_WRITE: writing image to FPGA
// @FPGA_MGR_STATE_WRITE_ERR: Error while writing FPGA
// @FPGA_MGR_STATE_WRITE_COMPLETE: Doing post programming steps
// @FPGA_MGR_STATE_WRITE_COMPLETE_ERR: Error during WRITE_COMPLETE
// @FPGA_MGR_STATE_OPERATING: FPGA is programmed and operating
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fpga_mgr_states {
// default FPGA states
    FPGA_MGR_STATE_UNKNOWN,
    FPGA_MGR_STATE_POWER_OFF,
    FPGA_MGR_STATE_POWER_UP,
    FPGA_MGR_STATE_RESET,

// getting an image for loading
    FPGA_MGR_STATE_FIRMWARE_REQ,
    FPGA_MGR_STATE_FIRMWARE_REQ_ERR,

// write sequence: parse header, init, write, complete
    FPGA_MGR_STATE_PARSE_HEADER,
    FPGA_MGR_STATE_PARSE_HEADER_ERR,
    FPGA_MGR_STATE_WRITE_INIT,
    FPGA_MGR_STATE_WRITE_INIT_ERR,
    FPGA_MGR_STATE_WRITE,
    FPGA_MGR_STATE_WRITE_ERR,
    FPGA_MGR_STATE_WRITE_COMPLETE,
    FPGA_MGR_STATE_WRITE_COMPLETE_ERR,

// fpga is programmed and operating
    FPGA_MGR_STATE_OPERATING,
}

//
// DOC: FPGA Manager flags
//
// Flags used in the &fpga_image_info->flags field
//
// %FPGA_MGR_PARTIAL_RECONFIG: do partial reconfiguration if supported
//
// %FPGA_MGR_EXTERNAL_CONFIG: FPGA has been configured prior to Linux booting
//
// %FPGA_MGR_ENCRYPTED_BITSTREAM: indicates bitstream is encrypted
//
// %FPGA_MGR_BITSTREAM_LSB_FIRST: SPI bitstream bit order is LSB first
//
// %FPGA_MGR_COMPRESSED_BITSTREAM: FPGA bitstream is compressed
//

//
// struct fpga_image_info - information specific to an FPGA image
// @flags: boolean flags as defined above
// @enable_timeout_us: maximum time to enable traffic through bridge (uSec)
// @disable_timeout_us: maximum time to disable traffic through bridge (uSec)
// @config_complete_timeout_us: maximum time for FPGA to switch to operating
// status in the write_complete op.
// @firmware_name: name of FPGA image firmware file
// @sgt: scatter/gather table containing FPGA image
// @buf: contiguous buffer containing FPGA image
// @count: size of buf
// @header_size: size of image header.
// @data_size: size of image data to be sent to the device. If not specified,
// whole image will be used. Header may be skipped in either case.
// @region_id: id of target region
// @dev: device that owns this
// @overlay: Device Tree overlay
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_image_info {
    pub flags: u32,
    pub enable_timeout_us: u32,
    pub disable_timeout_us: u32,
    pub config_complete_timeout_us: u32,
    pub firmware_name: *mut c_char,
    pub sgt: *mut sg_table,
    pub buf: *const c_char,
    pub count: usize,
    pub header_size: usize,
    pub data_size: usize,
    pub region_id: c_int,
    pub dev: *mut device,

    pub overlay: *mut device_node,

}

//
// struct fpga_compat_id - id for compatibility check
//
// @id_h: high 64bit of the compat_id
// @id_l: low 64bit of the compat_id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_compat_id {
    pub id_h: u64,
    pub id_l: u64,
}

//
// struct fpga_manager_info - collection of parameters for an FPGA Manager
// @name: fpga manager name
// @compat_id: FPGA manager id for compatibility check.
// @mops: pointer to structure of fpga manager ops
// @priv: fpga manager private data
//
// fpga_manager_info contains parameters for the register_full function.
// These are separated into an info structure because they some are optional
// others could be added to in the future. The info structure facilitates
// maintaining a stable API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_manager_info {
    pub name: *const c_char,
    pub compat_id: *mut fpga_compat_id,
    pub mops: *const fpga_manager_ops,
    pub priv: *mut c_void,
}

//
// struct fpga_manager_ops - ops for low level fpga manager drivers
// @initial_header_size: minimum number of bytes that should be passed into
// parse_header and write_init.
// @skip_header: bool flag to tell fpga-mgr core whether it should skip
// info->header_size part at the beginning of the image when invoking
// write callback.
// @state: returns an enum value of the FPGA's state
// @status: returns status of the FPGA, including reconfiguration error code
// @parse_header: parse FPGA image header to set info->header_size and
// info->data_size. In case the input buffer is not large enough, set
// required size to info->header_size and return -EAGAIN.
// @write_init: prepare the FPGA to receive configuration data
// @write: write count bytes of configuration data to the FPGA
// @write_sg: write the scatter list of configuration data to the FPGA
// @write_complete: set FPGA to operating state after writing is done
// @fpga_remove: optional: Set FPGA into a specific state during driver remove
// @groups: optional attribute groups.
//
// fpga_manager_ops are the low level functions implemented by a specific
// fpga manager driver.  The optional ones are tested for NULL before being
// called, so leaving them out is fine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_manager_ops {
    pub initial_header_size: usize,
    pub skip_header: bool,
    pub mgr): *mut *mut fpga_mgr_states (state)(struct fpga_manager,
    pub mgr): *mut *mut u64 (status)(struct fpga_manager,
    pub count): *const *const char buf, size_t,
    pub count): *const *const char buf, size_t,
    pub count): *const *const *const *const int (write)(struct fpga_manager mgr, char buf, size_t,
    pub sgt): *mut *mut *mut int (write_sg)(struct fpga_manager mgr, struct sg_table,
    pub info): *mut fpga_image_info,
    pub mgr): *mut *mut void (fpga_remove)(struct fpga_manager,
    pub groups: *const attribute_group,
}

// FPGA manager status: Partial/Full Reconfiguration errors

//
// struct fpga_manager - fpga manager structure
// @name: name of low level fpga manager
// @dev: fpga manager device
// @ref_mutex: only allows one reference to fpga manager
// @state: state of fpga manager
// @compat_id: FPGA manager id for compatibility check.
// @mops: pointer to struct of fpga manager ops
// @mops_owner: module containing the mops
// @priv: low level driver private date
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_manager {
    pub name: *const c_char,
    pub dev: device,
    pub ref_mutex: mutex,
    pub state: fpga_mgr_states,
    pub compat_id: *mut fpga_compat_id,
    pub mops: *const fpga_manager_ops,
    pub mops_owner: *mut module,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn fpga_image_info_free(info: *mut fpga_image_info);
}
extern "C" {
    pub fn fpga_mgr_load(mgr: *mut fpga_manager, info: *mut fpga_image_info) -> c_int;
}
extern "C" {
    pub fn fpga_mgr_lock(mgr: *mut fpga_manager) -> c_int;
}
extern "C" {
    pub fn fpga_mgr_unlock(mgr: *mut fpga_manager);
}
extern "C" {
    pub fn fpga_mgr_put(mgr: *mut fpga_manager);
}

extern "C" {
    pub fn fpga_mgr_unregister(mgr: *mut fpga_manager);
}

