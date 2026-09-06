//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mei/hw-txe-regs.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (c) 2013-2014, Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
//

// SeC FW Status Register
//
// FW uses this register in order to report its status to host.
// This register resides in PCI-E config space.
//
pub const PCI_CFG_TXE_FW_STS0: c_uint = 0x40;

pub const PCI_CFG_TXE_FW_STS1: c_uint = 0x48;
pub const IPC_BASE_ADDR: c_uint = 0x80400 /* SeC IPC Base Address */;
// IPC Input Doorbell Register

// IPC Input Status Register
// This register indicates whether or not processing of
// the most recent command has been completed by the SEC
// New commands and payloads should not be written by the Host
// until this indicates that the previous command has been processed.
//

// IPC Host Interrupt Status Register

// Convenient mask for pending interrupts

// IPC Host Interrupt Mask Register

// IPC Input Payload RAM

// IPC Shared Payload RAM

// SeC Address Translation Table Entry 2 - Ctrl
//
// This register resides also in SeC's PCI-E Memory space.
//
pub const SATT2_CTRL_REG: c_uint = 0x1040;

// SATT Table Entry 2 SAP Base Address Register
pub const SATT2_SAP_BA_REG: c_uint = 0x1044;
// SATT Table Entry 2 SAP Size Register.
pub const SATT2_SAP_SIZE_REG: c_uint = 0x1048;
// SATT Table Entry 2 SAP Bridge Address - LSB Register
pub const SATT2_BRG_BA_LSB_REG: c_uint = 0x104C;
// Host High-level Interrupt Status Register
pub const HHISR_REG: c_uint = 0x2020;
// Host High-level Interrupt Enable Register
//
// Resides in PCI memory space. This is the top hierarchy for
// interrupts from SeC to host, aggregating both interrupts that
// arrive through HICR registers as well as interrupts
// that arrive via IPC.
//
pub const HHIER_REG: c_uint = 0x2024;

// Host High-level Interrupt Mask Register.
//
// Resides in PCI memory space.
// This is the top hierarchy for masking interrupts from SeC to host.
//
pub const HHIMR_REG: c_uint = 0x2028;

// Host High-level IRQ Status Register
pub const HHIRQSR_REG: c_uint = 0x202C;
// Host Interrupt Cause Register 0 - SeC IPC Readiness
//
// This register is both an ICR to Host from PCI Memory Space
// and it is also exposed in the SeC memory space.
// This register is used by SeC's IPC driver in order
// to synchronize with host about IPC interface state.
//
pub const HICR_SEC_IPC_READINESS_REG: c_uint = 0x2040;

// Host Interrupt Cause Register 1 - Aliveness Response
// This register is both an ICR to Host from PCI Memory Space
// and it is also exposed in the SeC memory space.
// The register may be used by SeC to ACK a host request for aliveness.
//
pub const HICR_HOST_ALIVENESS_RESP_REG: c_uint = 0x2044;

// Host Interrupt Cause Register 2 - SeC IPC Output Doorbell
pub const HICR_SEC_IPC_OUTPUT_DOORBELL_REG: c_uint = 0x2048;
// Host Interrupt Status Register.
//
// Resides in PCI memory space.
// This is the main register involved in generating interrupts
// from SeC to host via HICRs.
// The interrupt generation rules are as follows:
// An interrupt will be generated whenever for any i,
// there is a transition from a state where at least one of
// the following conditions did not hold, to a state where
// ALL the following conditions hold:
// A) HISR.INT[i]_STS == 1.
// B) HIER.INT[i]_EN == 1.
//
pub const HISR_REG: c_uint = 0x2060;

// Host Interrupt Enable Register. Resides in PCI memory space.
pub const HIER_REG: c_uint = 0x2064;

// SEC Memory Space IPC output payload.
//
// This register is part of the output payload which SEC provides to host.
//
pub const BRIDGE_IPC_OUTPUT_PAYLOAD_REG: c_uint = 0x20C0;
// SeC Interrupt Cause Register - Host Aliveness Request
// This register is both an ICR to SeC and it is also exposed
// in the host-visible PCI memory space.
// The register is used by host to request SeC aliveness.
//
pub const SICR_HOST_ALIVENESS_REQ_REG: c_uint = 0x214C;

// SeC Interrupt Cause Register - Host IPC Readiness
//
// This register is both an ICR to SeC and it is also exposed
// in the host-visible PCI memory space.
// This register is used by the host's SeC driver uses in order
// to synchronize with SeC about IPC interface state.
//
pub const SICR_HOST_IPC_READINESS_REQ_REG: c_uint = 0x2150;

// SeC Interrupt Cause Register - SeC IPC Output Status
//
// This register indicates whether or not processing of the most recent
// command has been completed by the Host.
// New commands and payloads should not be written by SeC until this
// register indicates that the previous command has been processed.
//
pub const SICR_SEC_IPC_OUTPUT_STATUS_REG: c_uint = 0x2154;

// MEI IPC Message payload size 64 bytes
pub const PAYLOAD_SIZE: c_int = 64;
// MAX size for SATT range 32MB

