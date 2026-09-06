//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/acbuffer.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: acbuffer.h - Support for buffers returned by ACPI predefined names
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Contains buffer structures for these predefined names:
// _FDE, _GRT, _GTM, _PLD, _SRT
//
// Note: C bitfields are not used for this reason:
//
// "Bitfields are great and easy to read, but unfortunately the C language
// does not specify the layout of bitfields in memory, which means they are
// essentially useless for dealing with packed data in on-disk formats or
// binary wire protocols." (Or ACPI tables and buffers.) "If you ask me,
// this decision was a design error in C. Ritchie could have picked an order
// and stuck with it." Norman Ramsey.
// See http://stackoverflow.com/a/1053662/41661
//
// _FDE return value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_fde_info {
    pub floppy0: u32,
    pub floppy1: u32,
    pub floppy2: u32,
    pub floppy3: u32,
    pub tape: u32,
}

//
// _GRT return value
// _SRT input value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_grt_info {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub valid: u8,
    pub milliseconds: u16,
    pub timezone: u16,
    pub daylight: u8,
    pub reserved: [u8; 3],
}

// _GTM return value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gtm_info {
    pub pio_speed0: u32,
    pub dma_speed0: u32,
    pub pio_speed1: u32,
    pub dma_speed1: u32,
    pub flags: u32,
}

//
// Formatted _PLD return value. The minimum size is a package containing
// one buffer.
// Revision 1: Buffer is 16 bytes (128 bits)
// Revision 2: Buffer is 20 bytes (160 bits)
//
// Note: This structure is returned from the acpi_decode_pld_buffer
// interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pld_info {
    pub revision: u8,
    pub ignore_color: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub width: u16,
    pub height: u16,
    pub user_visible: u8,
    pub dock: u8,
    pub lid: u8,
    pub panel: u8,
    pub vertical_position: u8,
    pub horizontal_position: u8,
    pub shape: u8,
    pub group_orientation: u8,
    pub group_token: u8,
    pub group_position: u8,
    pub bay: u8,
    pub ejectable: u8,
    pub ospm_eject_required: u8,
    pub cabinet_number: u8,
    pub card_cage_number: u8,
    pub reference: u8,
    pub rotation: u8,
    pub order: u8,
    pub reserved: u8,
    pub vertical_offset: u16,
    pub horizontal_offset: u16,
}

//
// Macros to:
// 1) Convert a _PLD buffer to internal struct acpi_pld_info format - ACPI_PLD_GET
// (Used by acpi_decode_pld_buffer)
// 2) Construct a _PLD buffer - ACPI_PLD_SET
// (Intended for BIOS use only)
//

// First 32-bit dword, bits 0:32

// Second 32-bit dword, bits 33:63

// Third 32-bit dword, bits 64:95

// Fourth 32-bit dword, bits 96:127

// Fifth 32-bit dword, bits 128:159 (Revision 2 of _PLD only)

// Panel position defined in _PLD section of ACPI Specification 6.3
pub const ACPI_PLD_PANEL_TOP: c_int = 0;
pub const ACPI_PLD_PANEL_BOTTOM: c_int = 1;
pub const ACPI_PLD_PANEL_LEFT: c_int = 2;
pub const ACPI_PLD_PANEL_RIGHT: c_int = 3;
pub const ACPI_PLD_PANEL_FRONT: c_int = 4;
pub const ACPI_PLD_PANEL_BACK: c_int = 5;
pub const ACPI_PLD_PANEL_UNKNOWN: c_int = 6;
