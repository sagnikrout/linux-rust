//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/cirrus/cs_dsp.h
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
// cs_dsp.h  --  Cirrus Logic DSP firmware support
//
// Based on sound/soc/codecs/wm_adsp.h
//
// Copyright 2012 Wolfson Microelectronics plc
// Copyright (C) 2015-2021 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

pub const CS_DSP_DATA_WORD_SIZE: c_int = 3;

pub const CS_DSP_ACKED_CTL_TIMEOUT_MS: c_int = 100;
pub const CS_DSP_ACKED_CTL_N_QUICKPOLLS: c_int = 10;
pub const CS_DSP_ACKED_CTL_MIN_VALUE: c_int = 0;
pub const CS_DSP_ACKED_CTL_MAX_VALUE: c_uint = 0xFFFFFF;
//
// Write sequence operation codes
//
pub const CS_DSP_WSEQ_FULL: c_uint = 0x00;
pub const CS_DSP_WSEQ_ADDR8: c_uint = 0x02;
pub const CS_DSP_WSEQ_L16: c_uint = 0x04;
pub const CS_DSP_WSEQ_H16: c_uint = 0x05;
pub const CS_DSP_WSEQ_UNLOCK: c_uint = 0xFD;
pub const CS_DSP_WSEQ_END: c_uint = 0xFF;
//
// struct cs_dsp_region - Describes a logical memory region in DSP address space
// @type:	Memory region type
// @base:	Address of region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_region {
    pub type: c_int,
    pub base: c_uint,
}

//
// struct cs_dsp_alg_region - Describes a logical algorithm region in DSP address space
// @alg:	Algorithm id
// @ver:	Expected algorithm version
// @type:	Memory region type
// @base:	Address of region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_alg_region {
    pub alg: c_uint,
    pub ver: c_uint,
    pub type: c_int,
    pub base: c_uint,
}

//
// struct cs_dsp_coeff_ctl - Describes a coefficient control
// @list:		List node for internal use
// @dsp:		DSP instance associated with this control
// @cache:		Cached value of the control
// @fw_name:		Name of the firmware
// @subname:		Name of the control parsed from the WMFW
// @subname_len:	Length of subname
// @offset:		Offset of control within alg_region in words
// @len:		Length of the cached value in bytes
// @type:		One of the WMFW_CTL_TYPE_ control types defined in wmfw.h
// @flags:		Bitfield of WMFW_CTL_FLAG_ control flags defined in wmfw.h
// @set:		Flag indicating the value has been written by the user
// @enabled:		Flag indicating whether control is enabled
// @alg_region:		Logical region associated with this control
// @priv:		For use by the client
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_coeff_ctl {
    pub list: list_head,
    pub dsp: *mut cs_dsp,
    pub cache: *mut c_void,
    pub fw_name: *const c_char,
// Subname is needed to match with firmware
    pub subname: *const c_char,
    pub subname_len: c_uint,
    pub offset: c_uint,
    pub len: c_uint,
    pub type: c_uint,
    pub flags: c_uint,
    pub set:1: c_uint,
    pub enabled:1: c_uint,
    pub alg_region: cs_dsp_alg_region,
    pub priv: *mut c_void,
}

//
// struct cs_dsp - Configuration and state of a Cirrus Logic DSP
// @name:		The name of the DSP instance
// @rev:		Revision of the DSP
// @num:		DSP instance number
// @type:		Type of DSP
// @dev:		Driver model representation of the device
// @regmap:		Register map of the device
// @ops:		Function pointers for internal callbacks
// @client_ops:		Function pointers for client callbacks
// @base:		Address of the DSP registers
// @base_sysinfo:	Address of the sysinfo register (Halo only)
// @sysclk_reg:		Address of the sysclk register (ADSP1 only)
// @sysclk_mask:	Mask of frequency bits within sysclk register (ADSP1 only)
// @sysclk_shift:	Shift of frequency bits within sysclk register (ADSP1 only)
// @alg_regions:	List of currently loaded algorithm regions
// @fw_name:		Name of the current firmware
// @fw_id:		ID of the current firmware, obtained from the wmfw
// @fw_id_version:	Version of the firmware, obtained from the wmfw
// @fw_vendor_id:	Vendor of the firmware, obtained from the wmfw
// @mem:		DSP memory region descriptions
// @num_mems:		Number of memory regions in this DSP
// @fw_ver:		Version of the wmfw file format
// @booted:		Flag indicating DSP has been configured
// @running:		Flag indicating DSP is executing firmware
// @ctl_list:		Controls defined within the loaded DSP firmware
// @lock_regions:	Enable MPU traps on specified memory regions
// @pwr_lock:		Lock used to serialize accesses
// @debugfs_root:	Debugfs directory for this DSP instance
// @wmfw_file_name:	Filename of the currently loaded firmware
// @bin_file_name:	Filename of the currently loaded coefficients
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp {
    pub name: *const c_char,
    pub rev: c_int,
    pub num: c_int,
    pub type: c_int,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub ops: *const cs_dsp_ops,
    pub client_ops: *const cs_dsp_client_ops,
    pub base: c_uint,
    pub base_sysinfo: c_uint,
    pub sysclk_reg: c_uint,
    pub sysclk_mask: c_uint,
    pub sysclk_shift: c_uint,
    pub no_core_startstop: bool,
    pub alg_regions: list_head,
    pub fw_name: *const c_char,
    pub fw_id: c_uint,
    pub fw_id_version: c_uint,
    pub fw_vendor_id: c_uint,
    pub mem: *const cs_dsp_region,
    pub num_mems: c_int,
    pub wmfw_ver: c_int,
    pub booted: bool,
    pub running: bool,
    pub hibernating: bool,
    pub ctl_list: list_head,
    pub pwr_lock: mutex,
    pub lock_regions: c_uint,

    pub debugfs_root: *mut dentry,
    pub wmfw_file_name: *const c_char,
    pub bin_file_name: *const c_char,

}

//
// struct cs_dsp_client_ops - client callbacks
// @control_add:	Called under the pwr_lock when a control is created
// @control_remove:	Called under the pwr_lock when a control is destroyed
// @pre_run:		Called under the pwr_lock by cs_dsp_run() before the core is started
// @post_run:		Called under the pwr_lock by cs_dsp_run() after the core is started
// @pre_stop:		Called under the pwr_lock by cs_dsp_stop() before the core is stopped
// @post_stop:		Called under the pwr_lock by cs_dsp_stop() after the core is stopped
// @watchdog_expired:	Called when a watchdog expiry is detected
//
// These callbacks give the cs_dsp client an opportunity to respond to events
// or to perform actions atomically.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_client_ops {
    pub ctl): *mut *mut int (control_add)(struct cs_dsp_coeff_ctl,
    pub ctl): *mut *mut void (control_remove)(struct cs_dsp_coeff_ctl,
    pub dsp): *mut *mut int (pre_run)(struct cs_dsp,
    pub dsp): *mut *mut int (post_run)(struct cs_dsp,
    pub dsp): *mut *mut void (pre_stop)(struct cs_dsp,
    pub dsp): *mut *mut void (post_stop)(struct cs_dsp,
    pub dsp): *mut *mut void (watchdog_expired)(struct cs_dsp,
}

extern "C" {
    pub fn cs_dsp_adsp1_init(dsp: *mut cs_dsp) -> c_int;
}
extern "C" {
    pub fn cs_dsp_adsp2_init(dsp: *mut cs_dsp) -> c_int;
}
extern "C" {
    pub fn cs_dsp_halo_init(dsp: *mut cs_dsp) -> c_int;
}
extern "C" {
    pub fn cs_dsp_adsp1_power_down(dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs_dsp_power_down(dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs_dsp_run(dsp: *mut cs_dsp) -> c_int;
}
extern "C" {
    pub fn cs_dsp_stop(dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs_dsp_remove(dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs_dsp_set_dspclk(dsp: *mut cs_dsp, freq: c_uint) -> c_int;
}
extern "C" {
    pub fn cs_dsp_adsp2_bus_error(dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs_dsp_halo_bus_error(dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs_dsp_halo_wdt_expire(dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs_dsp_init_debugfs(dsp: *mut cs_dsp, debugfs_root: *mut dentry);
}
extern "C" {
    pub fn cs_dsp_cleanup_debugfs(dsp: *mut cs_dsp);
}
extern "C" {
    pub fn cs_dsp_coeff_write_acked_control(ctl: *mut cs_dsp_coeff_ctl, event_id: c_uint) -> c_int;
}
extern "C" {
    pub fn cs_dsp_read_data_word(dsp: *mut cs_dsp, mem_type: c_int, mem_addr: c_uint, data: *mut u32) -> c_int;
}
extern "C" {
    pub fn cs_dsp_write_data_word(dsp: *mut cs_dsp, mem_type: c_int, mem_addr: c_uint, data: u32) -> c_int;
}
extern "C" {
    pub fn cs_dsp_remove_padding(buf: *mut u32, nwords: c_int);
}
//
// struct cs_dsp_wseq - Describes a write sequence
// @ctl:	Write sequence cs_dsp control
// @ops:	Operations contained within
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_wseq {
    pub ctl: *mut cs_dsp_coeff_ctl,
    pub ops: list_head,
}

extern "C" {
    pub fn cs_dsp_wseq_init(dsp: *mut cs_dsp, wseqs: *mut cs_dsp_wseq, num_wseqs: c_uint) -> c_int;
}
//
// struct cs_dsp_chunk - Describes a buffer holding data formatted for the DSP
// @data:	Pointer to underlying buffer memory
// @max:	Pointer to end of the buffer memory
// @bytes:	Number of bytes read/written into the memory chunk
// @cache:	Temporary holding data as it is formatted
// @cachebits:	Number of bits of data currently in cache
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_dsp_chunk {
    pub data: *mut u8,
    pub max: *mut u8,
    pub bytes: c_int,
    pub cache: u32,
    pub cachebits: c_int,
}

//
// cs_dsp_chunk() - Create a DSP memory chunk
// @data: Pointer to the buffer that will be used to store data
// @size: Size of the buffer in bytes
//
// Return: A cs_dsp_chunk structure
//
// cs_dsp_chunk_end() - Check if a DSP memory chunk is full
// @ch: Pointer to the chunk structure
//
// Return: True if the whole buffer has been read/written
//
// cs_dsp_chunk_bytes() - Number of bytes written/read from a DSP memory chunk
// @ch: Pointer to the chunk structure
//
// Return: Number of bytes read/written to the buffer
//
// cs_dsp_chunk_valid_addr() - Check if an address is in a DSP memory chunk
// @ch: Pointer to the chunk structure
//
// Return: True if the given address is within the buffer
//
extern "C" {
    pub fn cs_dsp_chunk_write(ch: *mut cs_dsp_chunk, nbits: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn cs_dsp_chunk_flush(ch: *mut cs_dsp_chunk) -> c_int;
}
extern "C" {
    pub fn cs_dsp_chunk_read(ch: *mut cs_dsp_chunk, nbits: c_int) -> c_int;
}
extern "C" {
    pub fn cs_dsp_hibernate(dsp: *mut cs_dsp, hibernating: bool);
}
