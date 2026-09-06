//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/efi/stmm/mm_communication.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Headers for EFI variable service via StandAloneMM, EDK2 application running
// in OP-TEE. Most of the structs and defines resemble the EDK2 naming.
//
// Copyright (c) 2017, Intel Corporation. All rights reserved.
// Copyright (C) 2020 Linaro Ltd.
//
// Interface to the pseudo Trusted Application (TA), which provides a
// communication channel with the Standalone MM (Management Mode)
// Secure Partition running at Secure-EL0
//
pub const PTA_STMM_CMD_COMMUNICATE: c_int = 0;
//
// Defined in OP-TEE, this UUID is used to identify the pseudo-TA.
// OP-TEE is using big endian GUIDs while UEFI uses little endian ones
//

//
// struct efi_mm_communicate_header - Header used for SMM variable communication
//
// @header_guid:  header use for disambiguation of content
// @message_len:  length of the message. Does not include the size of the
// header
// @data:         payload of the message
//
// Defined in the PI spec as EFI_MM_COMMUNICATE_HEADER.
// To avoid confusion in interpreting frames, the communication buffer should
// always begin with efi_mm_communicate_header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_mm_communicate_header {
    pub header_guid: efi_guid_t,
    pub message_len: usize,
    pub data: [u8; ],
    pub __packed: },

// SPM return error codes
pub const ARM_SVC_SPM_RET_SUCCESS: c_int = 0;

pub const SMM_VARIABLE_FUNCTION_GET_VARIABLE: c_int = 1;
//
// The payload for this function is
// SMM_VARIABLE_COMMUNICATE_GET_NEXT_VARIABLE_NAME.
//
pub const SMM_VARIABLE_FUNCTION_GET_NEXT_VARIABLE_NAME: c_int = 2;
//
// The payload for this function is SMM_VARIABLE_COMMUNICATE_ACCESS_VARIABLE.
//
pub const SMM_VARIABLE_FUNCTION_SET_VARIABLE: c_int = 3;
//
// The payload for this function is
// SMM_VARIABLE_COMMUNICATE_QUERY_VARIABLE_INFO.
//
pub const SMM_VARIABLE_FUNCTION_QUERY_VARIABLE_INFO: c_int = 4;
//
// It is a notify event, no extra payload for this function.
//
pub const SMM_VARIABLE_FUNCTION_READY_TO_BOOT: c_int = 5;
//
// It is a notify event, no extra payload for this function.
//
pub const SMM_VARIABLE_FUNCTION_EXIT_BOOT_SERVICE: c_int = 6;
//
// The payload for this function is VARIABLE_INFO_ENTRY.
// The GUID in EFI_SMM_COMMUNICATE_HEADER is gEfiSmmVariableProtocolGuid.
//
pub const SMM_VARIABLE_FUNCTION_GET_STATISTICS: c_int = 7;
//
// The payload for this function is SMM_VARIABLE_COMMUNICATE_LOCK_VARIABLE
//
pub const SMM_VARIABLE_FUNCTION_LOCK_VARIABLE: c_int = 8;
pub const SMM_VARIABLE_FUNCTION_VAR_CHECK_VARIABLE_PROPERTY_SET: c_int = 9;
pub const SMM_VARIABLE_FUNCTION_VAR_CHECK_VARIABLE_PROPERTY_GET: c_int = 10;
pub const SMM_VARIABLE_FUNCTION_GET_PAYLOAD_SIZE: c_int = 11;
//
// The payload for this function is
// SMM_VARIABLE_COMMUNICATE_RUNTIME_VARIABLE_CACHE_CONTEXT
//
pub const SMM_VARIABLE_FUNCTION_INIT_RUNTIME_VARIABLE_CACHE_CONTEXT: c_int = 12;
pub const SMM_VARIABLE_FUNCTION_SYNC_RUNTIME_CACHE: c_int = 13;
//
// The payload for this function is
// SMM_VARIABLE_COMMUNICATE_GET_RUNTIME_CACHE_INFO
//
pub const SMM_VARIABLE_FUNCTION_GET_RUNTIME_CACHE_INFO: c_int = 14;
//
// struct smm_variable_communicate_header - Used for SMM variable communication
//
// @function:     function to call in Smm.
// @ret_status:   return status
// @data:         payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smm_variable_communicate_header {
    pub function: usize,
    pub ret_status: efi_status_t,
    pub data: [u8; ],
}

//
// struct smm_variable_access - Used to communicate with StMM by
// SetVariable and GetVariable.
//
// @guid:         vendor GUID
// @data_size:    size of EFI variable data
// @name_size:    size of EFI name
// @attr:         attributes
// @name:         variable name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smm_variable_access {
    pub guid: efi_guid_t,
    pub data_size: usize,
    pub name_size: usize,
    pub attr: u32,
    pub name: [u16; ],
}

//
// struct smm_variable_payload_size - Used to get the max allowed
// payload used in StMM.
//
// @size:  size to fill in
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smm_variable_payload_size {
    pub size: usize,
}

//
// struct smm_variable_getnext - Used to communicate with StMM for
// GetNextVariableName.
//
// @guid:       vendor GUID
// @name_size:  size of the name of the variable
// @name:       variable name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smm_variable_getnext {
    pub guid: efi_guid_t,
    pub name_size: usize,
    pub name: [u16; ],
}

//
// struct smm_variable_query_info - Used to communicate with StMM for
// QueryVariableInfo.
//
// @max_variable_storage:        max available storage
// @remaining_variable_storage:  remaining available storage
// @max_variable_size:           max variable supported size
// @attr:                        attributes to query storage for
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smm_variable_query_info {
    pub max_variable_storage: u64,
    pub remaining_variable_storage: u64,
    pub max_variable_size: u64,
    pub attr: u32,
}

pub const VAR_CHECK_VARIABLE_PROPERTY_REVISION: c_uint = 0x0001;

//
// struct var_check_property - Used to store variable properties in StMM
//
// @revision:   magic revision number for variable property checking
// @property:   properties mask for the variable used in StMM.
// Currently RO flag is supported
// @attributes: variable attributes used in StMM checking when properties
// for a variable are enabled
// @minsize:    minimum allowed size for variable payload checked against
// smm_variable_access->datasize in StMM
// @maxsize:    maximum allowed size for variable payload checked against
// smm_variable_access->datasize in StMM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct var_check_property {
    pub revision: u16,
    pub property: u16,
    pub attributes: u32,
    pub minsize: usize,
    pub maxsize: usize,
}

//
// struct smm_variable_var_check_property - Used to communicate variable
// properties with StMM
//
// @guid:       vendor GUID
// @name_size:  size of EFI name
// @property:   variable properties struct
// @name:       variable name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smm_variable_var_check_property {
    pub guid: efi_guid_t,
    pub name_size: usize,
    pub property: var_check_property,
    pub name: [u16; ],
}
