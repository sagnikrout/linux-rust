//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_mmio.h
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
// Copyright (C) 2020-21 Intel Corporation.
//
// Minimal IOSM CP VERSION which has valid CP_CAPABILITIES field
pub const IOSM_CP_VERSION: c_uint = 0x0100UL;
// DL dir Aggregation support mask

// UL dir Aggregation support mask

// UL flow credit support mask

// Possible states of the IPC finite state machine.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mem_device_ipc_state {
    IPC_MEM_DEVICE_IPC_UNINIT,
    IPC_MEM_DEVICE_IPC_INIT,
    IPC_MEM_DEVICE_IPC_RUNNING,
    IPC_MEM_DEVICE_IPC_RECOVERY,
    IPC_MEM_DEVICE_IPC_ERROR,
    IPC_MEM_DEVICE_IPC_DONT_CARE,
    IPC_MEM_DEVICE_IPC_INVALID = -1
}

// Boot ROM exit status.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rom_exit_code {
    IMEM_ROM_EXIT_OPEN_EXT = 0x01,
    IMEM_ROM_EXIT_OPEN_MEM = 0x02,
    IMEM_ROM_EXIT_CERT_EXT = 0x10,
    IMEM_ROM_EXIT_CERT_MEM = 0x20,
    IMEM_ROM_EXIT_FAIL = 0xFF
}

// Boot stages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mem_exec_stage {
    IPC_MEM_EXEC_STAGE_RUN = 0x600DF00D,
    IPC_MEM_EXEC_STAGE_CRASH = 0x8BADF00D,
    IPC_MEM_EXEC_STAGE_CD_READY = 0xBADC0DED,
    IPC_MEM_EXEC_STAGE_BOOT = 0xFEEDB007,
    IPC_MEM_EXEC_STAGE_PSI = 0xFEEDBEEF,
    IPC_MEM_EXEC_STAGE_EBL = 0xFEEDCAFE,
    IPC_MEM_EXEC_STAGE_INVALID = 0xFFFFFFFF
}

// mmio scratchpad info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmio_offset {
    pub exec_stage: c_int,
    pub chip_info: c_int,
    pub rom_exit_code: c_int,
    pub psi_address: c_int,
    pub psi_size: c_int,
    pub ipc_status: c_int,
    pub context_info: c_int,
    pub ap_win_base: c_int,
    pub ap_win_end: c_int,
    pub cp_version: c_int,
    pub cp_capability: c_int,
}

//
// struct iosm_mmio - MMIO region mapped to the doorbell scratchpad.
// @base:		Base address of MMIO region
// @dev:		Pointer to device structure
// @offset:		Start offset
// @context_info_addr:	Physical base address of context info structure
// @chip_info_version:	Version of chip info structure
// @chip_info_size:	Size of chip info structure
// @mux_protocol:	mux protocol
// @has_ul_flow_credit:	Ul flow credit support
// @has_slp_no_prot:	Device sleep no protocol support
// @has_mcr_support:	Usage of mcr support
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_mmio {
    pub base: *mut unsigned char __iomem,
    pub dev: *mut device,
    pub offset: mmio_offset,
    pub context_info_addr: phys_addr_t,
    pub chip_info_version: c_uint,
    pub chip_info_size: c_uint,
    pub mux_protocol: u32,
}

//
// ipc_mmio_init - Allocate mmio instance data
// @mmio_addr:	Mapped AP base address of the MMIO area.
// @dev:	Pointer to device structure
//
// Returns: address of mmio instance data or NULL if fails.
//
// ipc_mmio_set_psi_addr_and_size - Set start address and size of the
// primary system image (PSI) for the
// FW dowload.
// @ipc_mmio:	Pointer to mmio instance
// @addr:	PSI address
// @size:	PSI immage size
//
// ipc_mmio_set_contex_info_addr - Stores the Context Info Address in
// MMIO instance to share it with CP during
// mmio_init.
// @ipc_mmio:	Pointer to mmio instance
// @addr:	64-bit address of AP context information.
//
// ipc_mmio_get_cp_version - Get the CP IPC version
// @ipc_mmio:	Pointer to mmio instance
//
// Returns: version number on success and failure value on error.
//
extern "C" {
    pub fn ipc_mmio_get_cp_version(ipc_mmio: *mut iosm_mmio) -> c_int;
}
//
// ipc_mmio_get_rom_exit_code - Get exit code from CP boot rom download app
// @ipc_mmio:	Pointer to mmio instance
//
// Returns: exit code from CP boot rom download APP
//
extern "C" {
    pub fn ipc_mmio_get_rom_exit_code(ipc_mmio: *mut iosm_mmio) -> rom_exit_code;
}
//
// ipc_mmio_get_exec_stage - Query CP execution stage
// @ipc_mmio:	Pointer to mmio instance
//
// Returns: CP execution stage
//
extern "C" {
    pub fn ipc_mmio_get_exec_stage(ipc_mmio: *mut iosm_mmio) -> ipc_mem_exec_stage;
}
//
// ipc_mmio_get_ipc_state - Query CP IPC state
// @ipc_mmio:	Pointer to mmio instance
//
// Returns: CP IPC state
//
// ipc_mmio_copy_chip_info - Copy size bytes of CP chip info structure
// into caller provided buffer
// @ipc_mmio:	Pointer to mmio instance
// @dest:	Pointer to caller provided buff
// @size:	Number of bytes to copy
//
// ipc_mmio_config - Write context info and AP memory range addresses.
// This needs to be called when CP is in
// IPC_MEM_DEVICE_IPC_INIT state
//
// @ipc_mmio:	Pointer to mmio instance
//
extern "C" {
    pub fn ipc_mmio_config(ipc_mmio: *mut iosm_mmio);
}
//
// ipc_mmio_update_cp_capability - Read and update modem capability, from mmio
// capability offset
//
// @ipc_mmio:	Pointer to mmio instance
//
extern "C" {
    pub fn ipc_mmio_update_cp_capability(ipc_mmio: *mut iosm_mmio);
}
