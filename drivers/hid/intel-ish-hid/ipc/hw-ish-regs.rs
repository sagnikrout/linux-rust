//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-ish-hid/ipc/hw-ish-regs.h
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
// ISH registers definitions
//
// Copyright (c) 2012-2016, Intel Corporation.
//
// IPC PCI Offsets and sizes
// ISH IPC Base Address
pub const IPC_REG_BASE: c_uint = 0x0000;
// Peripheral Interrupt Status Register

// Peripheral Interrupt Mask Register

// BXT, CHV_K0
// Peripheral Interrupt Status Register

// Peripheral Interrupt Mask Register

//
// ISH Host Firmware status Register

// Host Communication Register

// Reset register

// Inbound doorbell register Host to ISH

// Outbound doorbell register ISH to Host

// ISH to HOST message registers

// HOST to ISH message registers

// REMAP2 to enable DMA (D3 RCR)

// register bits - HISR
// bit corresponds HOST2ISH interrupt in PISR and PIMR registers

//
// CHV_A0, CHV_B0
// bit corresponds ISH2HOST interrupt in PISR and PIMR registers

// BXT, CHV_K0
// bit corresponds ISH2HOST interrupt in PISR and PIMR registers

//
// bit corresponds ISH2HOST busy clear interrupt in PIMR register

// offset of ISH2HOST busy clear interrupt in IPC_BUSY_CLR register

// bit corresponds ISH2HOST busy clear interrupt in IPC_BUSY_CLR register

// bit corresponds busy bit in doorbell registers

//
// A0: bit means that host owns MSGnn registers and is reading them.
// ISH FW may not write to them
//

//
// Host status bits (HOSTCOMM)
//
// bit corresponds host ready bit in Host Status Register (HOST_COMM)

//
// CHV_A0, CHV_B0

// BXT, CHV_K0

//
// both Host and ISH have ILUP at bit 0
// bit corresponds host ready bit in both status registers
//

//
// ISH FW status bits in ISH FW Status Register
//
pub const IPC_ISH_FWSTS_SHIFT: c_int = 12;

//
// FW status bits (relevant)
//
pub const IPC_FWSTS_ILUP: c_uint = 0x1;

// bit corresponds host ready bit in ISH FW Status Register

pub const IPC_RMP2_DMA_ENABLED: c_uint = 0x1	/* Value to enable DMA, per D3 RCR */;
pub const IPC_MSG_MAX_SIZE: c_uint = 0x80;
pub const IPC_HEADER_LENGTH_MASK: c_uint = 0x03FF;
pub const IPC_HEADER_PROTOCOL_MASK: c_uint = 0x0F;
pub const IPC_HEADER_MNG_CMD_MASK: c_uint = 0x0F;
pub const IPC_HEADER_LENGTH_OFFSET: c_int = 0;
pub const IPC_HEADER_PROTOCOL_OFFSET: c_int = 10;
pub const IPC_HEADER_MNG_CMD_OFFSET: c_int = 16;

//
// CHV_A0, CHV_B0

// BXT, CHV_K0

//

// todo - temp until PIMR HW ready
pub const IPC_HOST_BUSY_READING_OFFS: c_int = 6;
// bit corresponds host ready bit in Host Status Register (HOST_COMM)

// Macro flag: #define IPC_CLEAR_HOST_BUSY_READING(host_status)\

pub const IPC_PROTOCOL_ISHTP: c_int = 1;
pub const IPC_PROTOCOL_MNG: c_int = 3;
pub const MNG_RX_CMPL_ENABLE: c_int = 0;
pub const MNG_RX_CMPL_DISABLE: c_int = 1;
pub const MNG_RX_CMPL_INDICATION: c_int = 2;
pub const MNG_RESET_NOTIFY: c_int = 3;
pub const MNG_RESET_NOTIFY_ACK: c_int = 4;
pub const MNG_SYNC_FW_CLOCK: c_int = 5;
pub const MNG_ILLEGAL_CMD: c_uint = 0xFF;
