//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tpm_svsm.h
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
// Copyright (C) 2023 James.Bottomley@HansenPartnership.com
// Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
//
// Helpers for the SVSM_VTPM_CMD calls used by the vTPM protocol defined by the
// AMD SVSM spec [1].
//
// The vTPM protocol follows the Official TPM 2.0 Reference Implementation
// (originally by Microsoft, now part of the TCG) simulator protocol.
//
// [1] "Secure VM Service Module for SEV-SNP Guests"
// Publication # 58019 Revision: 1.00
//

//
// struct svsm_vtpm_request - Generic request for single word command
// @cmd:	The command to send
//
// Defined by AMD SVSM spec [1] in section "8.2 SVSM_VTPM_CMD Call" -
// Table 15: vTPM Common Request/Response Structure
// Byte      Size       In/Out    Description
// Offset    (Bytes)
// 0x000     4          In        Platform command
// Out       Platform command response size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svsm_vtpm_request {
    pub cmd: u32,
}

//
// struct svsm_vtpm_response - Generic response
// @size:	The response size (zero if nothing follows)
//
// Defined by AMD SVSM spec [1] in section "8.2 SVSM_VTPM_CMD Call" -
// Table 15: vTPM Common Request/Response Structure
// Byte      Size       In/Out    Description
// Offset    (Bytes)
// 0x000     4          In        Platform command
// Out       Platform command response size
//
// Note: most TCG Simulator commands simply return zero here with no indication
// of success or failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svsm_vtpm_response {
    pub size: u32,
}

//
// struct svsm_vtpm_cmd_request - Structure for a TPM_SEND_COMMAND request
// @cmd:	The command to send (must be TPM_SEND_COMMAND)
// @locality:	The locality
// @buf_size:	The size of the input buffer following
// @buf:	A buffer of size buf_size
//
// Defined by AMD SVSM spec [1] in section "8.2 SVSM_VTPM_CMD Call" -
// Table 16: TPM_SEND_COMMAND Request Structure
// Byte      Size       Meaning
// Offset    (Bytes)
// 0x000     4          Platform command (8)
// 0x004     1          Locality (must-be-0)
// 0x005     4          TPM Command size (in bytes)
// 0x009     Variable   TPM Command
//
// Note: the TCG Simulator expects @buf_size to be equal to the size of the
// specific TPM command, otherwise an TPM_RC_COMMAND_SIZE error is returned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svsm_vtpm_cmd_request {
    pub cmd: u32,
    pub locality: u8,
    pub buf_size: u32,
    pub buf: [u8; ],
    pub __packed: },
//
// struct svsm_vtpm_cmd_response - Structure for a TPM_SEND_COMMAND response
// @buf_size:	The size of the output buffer following
// @buf:	A buffer of size buf_size
//
// Defined by AMD SVSM spec [1] in section "8.2 SVSM_VTPM_CMD Call" -
// Table 17: TPM_SEND_COMMAND Response Structure
// Byte      Size       Meaning
// Offset    (Bytes)
// 0x000     4          Response size (in bytes)
// 0x004     Variable   Response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svsm_vtpm_cmd_response {
    pub buf_size: u32,
    pub buf: [u8; ],
}

//
// svsm_vtpm_cmd_request_fill() - Fill a TPM_SEND_COMMAND request to be sent to SVSM
// @req: The struct svsm_vtpm_cmd_request to fill
// @locality: The locality
// @buf: The buffer from where to copy the payload of the command
// @len: The size of the buffer
//
// Return: 0 on success, negative error code on failure.
//
// svsm_vtpm_cmd_response_parse() - Parse a TPM_SEND_COMMAND response received from SVSM
// @resp: The struct svsm_vtpm_cmd_response to parse
// @buf: The buffer where to copy the response
// @len: The size of the buffer
//
// Return: buffer size filled with the response on success, negative error
// code on failure.
//
