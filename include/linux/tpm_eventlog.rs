//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tpm_eventlog.h
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

pub const TCG_EVENT_NAME_LEN_MAX: c_int = 255;

pub const EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2: c_uint = 0x1;
pub const EFI_TCG2_EVENT_LOG_FORMAT_TCG_2: c_uint = 0x2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bios_platform_class {
    BIOS_CLIENT = 0x00,
    BIOS_SERVER = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpa_event {
    pub pcr_index: u32,
    pub event_type: u32,
    pub /: *mut *mut u8 pcr_value[20]; / SHA1,
    pub event_size: u32,
    pub event_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcpa_event_types {
    PREBOOT = 0,
    POST_CODE,
    UNUSED,
    NO_ACTION,
    SEPARATOR,
    ACTION,
    EVENT_TAG,
    SCRTM_CONTENTS,
    SCRTM_VERSION,
    CPU_MICROCODE,
    PLATFORM_CONFIG_FLAGS,
    TABLE_OF_DEVICES,
    COMPACT_HASH,
    IPL,
    IPL_PARTITION_DATA,
    NONHOST_CODE,
    NONHOST_CONFIG,
    NONHOST_INFO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpa_pc_event {
    pub event_id: u32,
    pub event_size: u32,
    pub event_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcpa_pc_event_ids {
    SMBIOS = 1,
    BIS_CERT,
    POST_BIOS_ROM,
    ESCD,
    CMOS,
    NVRAM,
    OPTION_ROM_EXEC,
    OPTION_ROM_CONFIG,
    OPTION_ROM_MICROCODE = 10,
    S_CRTM_VERSION,
    S_CRTM_CONTENTS,
    POST_CONTENTS,
    HOST_TABLE_OF_DEVICES,
}

// http://www.trustedcomputinggroup.org/tcg-efi-protocol-specification/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcg_efi_specid_event_algs {
    pub alg_id: u16,
    pub digest_size: u16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcg_efi_specid_event_head {
    pub signature: [u8; 16],
    pub platform_class: u32,
    pub spec_version_minor: u8,
    pub spec_version_major: u8,
    pub spec_errata: u8,
    pub uintnsize: u8,
    pub num_algs: u32,
    pub digest_sizes: [tcg_efi_specid_event_algs; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcg_pcr_event {
    pub pcr_idx: u32,
    pub event_type: u32,
    pub digest: [u8; 20],
    pub event_size: u32,
    pub event: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcg_event_field {
    pub event_size: u32,
    pub event: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcg_pcr_event2_head {
    pub pcr_idx: u32,
    pub event_type: u32,
    pub count: u32,
    pub digests: [tpm_digest; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcg_algorithm_size {
    pub algorithm_id: u16,
    pub algorithm_size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcg_algorithm_info {
    pub signature: [u8; 16],
    pub platform_class: u32,
    pub spec_version_minor: u8,
    pub spec_version_major: u8,
    pub spec_errata: u8,
    pub uintn_size: u8,
    pub number_of_algorithms: u32,
    pub digest_sizes: [tcg_algorithm_size; ],
}

//
// __calc_tpm2_event_size - calculate the size of a TPM2 event log entry
// @event:        Pointer to the event whose size should be calculated
// @event_header: Pointer to the initial event containing the digest lengths
// @do_mapping:   Whether or not the event needs to be mapped
//
// The TPM2 event log format can contain multiple digests corresponding to
// separate PCR banks, and also contains a variable length of the data that
// was measured. This requires knowledge of how long each digest type is,
// and this information is contained within the first event in the log.
//
// We calculate the length by examining the number of events, and then looking
// at each event in turn to determine how much space is used for events in
// total. Once we've done this we know the offset of the data length field,
// and can calculate the total size of the event.
//
// Return: size of the event on success, 0 on failure
//
// Map the event header
//
// The loop below will unmap these fields if the log is larger than
// one page, so save them here for reference:
//
// Verify that it's the log header
//
// Perform validation of the event in order to identify malformed
// events. This function may be asked to parse arbitrary byte sequences
// immediately following a valid event log. The caller expects this
// function to recognize that the byte sequence is not a valid event
// and to return an event size of 0.
//
// Map the digest's algorithm identifier
// Algorithm without known length. Such event is unparseable.
//
// Map the event size - we don't read from the event itself, so
// we don't need to map it
//
