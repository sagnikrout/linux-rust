//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ras/amd/atl/reg_fields.h
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
// AMD Address Translation Library
//
// reg_fields.h : Register field definitions
//
// Copyright (c) 2023, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Yazen Ghannam <Yazen.Ghannam@amd.com>
//
// Notes on naming:
// 1) Use "DF_" prefix for fields that are the same for all revisions.
// 2) Use "DFx_" prefix for fields that differ between revisions.
// a) "x" is the first major revision where the new field appears.
// b) E.g., if DF2 and DF3 have the same field, then call it DF2.
// c) E.g., if DF3p5 and DF4 have the same field, then call it DF4.
//
// Coherent Station Fabric ID
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x50 [Fabric Block Instance Information 3]
// DF2	BlockFabricId	[19:8]
// DF3	BlockFabricId	[19:8]
// DF3p5	BlockFabricId	[19:8]
// DF4	BlockFabricId	[19:8]
// DF4p5	BlockFabricId	[15:8]
//

//
// Component ID Mask
//
// Access type: Broadcast
//
// Register
// Rev	Fieldname	Bits
//
// DF2	N/A
//
// D18F1x208 [System Fabric ID Mask 0]
// DF3	ComponentIdMask	[9:0]
//
// D18F1x150 [System Fabric ID Mask 0]
// DF3p5	ComponentIdMask	[15:0]
//
// D18F4x1B0 [System Fabric ID Mask 0]
// DF4	ComponentIdMask	[15:0]
// DF4p5	ComponentIdMask	[15:0]
//

//
// Destination Fabric ID
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x114 [DRAM Limit Address]
// DF2	DstFabricID	[7:0]
// DF3	DstFabricID	[9:0]
// DF3	DstFabricID	[11:0]
//
// D18F7xE08 [DRAM Address Control]
// DF4	DstFabricID	[27:16]
//
// D18F7x208 [DRAM Address Control]
// DF4p5	DstFabricID	[23:16]
//

//
// Die ID Mask
//
// Access type: Broadcast
//
// Register
// Rev	Fieldname	Bits
//
// D18F1x208 [System Fabric ID Mask]
// DF2	DieIdMask	[15:8]
//
// D18F1x20C [System Fabric ID Mask 1]
// DF3	DieIdMask	[18:16]
//
// D18F1x158 [System Fabric ID Mask 2]
// DF3p5	DieIdMask	[15:0]
//
// D18F4x1B8 [System Fabric ID Mask 2]
// DF4	DieIdMask	[15:0]
// DF4p5	DieIdMask	[15:0]
//

//
// Die ID Shift
//
// Access type: Broadcast
//
// Register
// Rev	Fieldname	Bits
//
// D18F1x208 [System Fabric ID Mask]
// DF2	DieIdShift	[27:24]
//
// DF3	N/A
// DF3p5	N/A
// DF4	N/A
// DF4p5	N/A
//

//
// DRAM Address Range Valid
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x110 [DRAM Base Address]
// DF2	AddrRngVal	[0]
// DF3	AddrRngVal	[0]
// DF3p5	AddrRngVal	[0]
//
// D18F7xE08 [DRAM Address Control]
// DF4	AddrRngVal	[0]
//
// D18F7x208 [DRAM Address Control]
// DF4p5	AddrRngVal	[0]
//

//
// DRAM Base Address
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x110 [DRAM Base Address]
// DF2	DramBaseAddr	[31:12]
// DF3	DramBaseAddr	[31:12]
// DF3p5	DramBaseAddr	[31:12]
//
// D18F7xE00 [DRAM Base Address]
// DF4	DramBaseAddr	[27:0]
//
// D18F7x200 [DRAM Base Address]
// DF4p5	DramBaseAddr	[27:0]
//

//
// DRAM Hole Base
//
// Access type: Broadcast
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x104 [DRAM Hole Control]
// DF2	DramHoleBase	[31:24]
// DF3	DramHoleBase	[31:24]
// DF3p5	DramHoleBase	[31:24]
//
// D18F7x104 [DRAM Hole Control]
// DF4	DramHoleBase	[31:24]
// DF4p5	DramHoleBase	[31:24]
//

//
// DRAM Limit Address
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x114 [DRAM Limit Address]
// DF2	DramLimitAddr	[31:12]
// DF3	DramLimitAddr	[31:12]
// DF3p5	DramLimitAddr	[31:12]
//
// D18F7xE04 [DRAM Limit Address]
// DF4	DramLimitAddr	[27:0]
//
// D18F7x204 [DRAM Limit Address]
// DF4p5	DramLimitAddr	[27:0]
//

//
// Hash Interleave Controls
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// DF2	N/A
//
// D18F0x3F8 [DF Global Control]
// DF3	GlbHashIntlvCtl64K	[20]
// GlbHashIntlvCtl2M	[21]
// GlbHashIntlvCtl1G	[22]
//
// DF3p5	GlbHashIntlvCtl64K	[20]
// GlbHashIntlvCtl2M	[21]
// GlbHashIntlvCtl1G	[22]
//
// D18F7xE08 [DRAM Address Control]
// DF4	HashIntlvCtl64K		[8]
// HashIntlvCtl2M		[9]
// HashIntlvCtl1G		[10]
//
// D18F7x208 [DRAM Address Control]
// DF4p5	HashIntlvCtl4K		[7]
// HashIntlvCtl64K		[8]
// HashIntlvCtl2M		[9]
// HashIntlvCtl1G		[10]
// HashIntlvCtl1T		[15]
//

//
// High Address Offset
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x1B4 [DRAM Offset]
// DF2	HiAddrOffset	[31:20]
// DF3	HiAddrOffset	[31:12]
// DF3p5	HiAddrOffset	[31:12]
//
// D18F7x140 [DRAM Offset]
// DF4	HiAddrOffset	[24:1]
// DF4p5	HiAddrOffset	[24:1]
// MI300	HiAddrOffset	[31:1]
//

// Follow reference code by including reserved bits for simplicity.

//
// High Address Offset Enable
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x1B4 [DRAM Offset]
// DF2	HiAddrOffsetEn	[0]
// DF3	HiAddrOffsetEn	[0]
// DF3p5	HiAddrOffsetEn	[0]
//
// D18F7x140 [DRAM Offset]
// DF4	HiAddrOffsetEn	[0]
// DF4p5	HiAddrOffsetEn	[0]
//

//
// Interleave Address Select
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x110 [DRAM Base Address]
// DF2	IntLvAddrSel	[10:8]
// DF3	IntLvAddrSel	[11:9]
// DF3p5	IntLvAddrSel	[11:9]
//
// D18F7xE0C [DRAM Address Interleave]
// DF4	IntLvAddrSel	[2:0]
//
// D18F7x20C [DRAM Address Interleave]
// DF4p5	IntLvAddrSel	[2:0]
//

//
// Interleave Number of Channels
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x110 [DRAM Base Address]
// DF2	IntLvNumChan	[7:4]
// DF3	IntLvNumChan	[5:2]
// DF3p5	IntLvNumChan	[6:2]
//
// D18F7xE0C [DRAM Address Interleave]
// DF4	IntLvNumChan	[8:4]
//
// D18F7x20C [DRAM Address Interleave]
// DF4p5	IntLvNumChan	[9:4]
//

//
// Interleave Number of Dies
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x114 [DRAM Limit Address]
// DF2	IntLvNumDies	[11:10]
//
// D18F0x110 [DRAM Base Address]
// DF3	IntLvNumDies	[7:6]
// DF3p5	IntLvNumDies	[7]
//
// D18F7xE0C [DRAM Address Interleave]
// DF4	IntLvNumDies	[13:12]
//
// D18F7x20C [DRAM Address Interleave]
// DF4p5	IntLvNumDies	[13:12]
//

//
// Interleave Number of Sockets
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x114 [DRAM Limit Address]
// DF2	IntLvNumSockets	[8]
//
// D18F0x110 [DRAM Base Address]
// DF3	IntLvNumSockets	[8]
// DF3p5	IntLvNumSockets	[8]
//
// D18F7xE0C [DRAM Address Interleave]
// DF4	IntLvNumSockets	[18]
//
// D18F7x20C [DRAM Address Interleave]
// DF4p5	IntLvNumSockets	[18]
//

//
// Legacy MMIO Hole Enable
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// D18F0x110 [DRAM Base Address]
// DF2	LgcyMmioHoleEn	[1]
// DF3	LgcyMmioHoleEn	[1]
// DF3p5	LgcyMmioHoleEn	[1]
//
// D18F7xE08 [DRAM Address Control]
// DF4	LgcyMmioHoleEn	[1]
//
// D18F7x208 [DRAM Address Control]
// DF4p5	LgcyMmioHoleEn	[1]
//

//
// Log2 Address 64K Space 0
//
// Access type: Instance
//
// Register
// Rev	Fieldname		Bits
//
// DF2	N/A
//
// D18F2x90 [Non-power-of-2 channel Configuration Register for COH_ST DRAM Address Maps]
// DF3	Log2Addr64KSpace0	[5:0]
//
// DF3p5	N/A
// DF4	N/A
// DF4p5	N/A
//

//
// Major Revision
//
// Access type: Broadcast
//
// Register
// Rev	Fieldname	Bits
//
// DF2	N/A
// DF3	N/A
// DF3p5	N/A
//
// D18F0x040 [Fabric Block Instance Count]
// DF4	MajorRevision	[27:24]
// DF4p5	MajorRevision	[27:24]
//

//
// Minor Revision
//
// Access type: Broadcast
//
// Register
// Rev	Fieldname	Bits
//
// DF2	N/A
// DF3	N/A
// DF3p5	N/A
//
// D18F0x040 [Fabric Block Instance Count]
// DF4	MinorRevision	[23:16]
// DF4p5	MinorRevision	[23:16]
//

//
// Node ID Mask
//
// Access type: Broadcast
//
// Register
// Rev	Fieldname	Bits
//
// DF2	N/A
//
// D18F1x208 [System Fabric ID Mask 0]
// DF3	NodeIdMask	[25:16]
//
// D18F1x150 [System Fabric ID Mask 0]
// DF3p5	NodeIdMask	[31:16]
//
// D18F4x1B0 [System Fabric ID Mask 0]
// DF4	NodeIdMask	[31:16]
// DF4p5	NodeIdMask	[31:16]
//

//
// Node ID Shift
//
// Access type: Broadcast
//
// Register
// Rev	Fieldname	Bits
//
// DF2	N/A
//
// D18F1x20C [System Fabric ID Mask 1]
// DF3	NodeIdShift	[3:0]
//
// D18F1x154 [System Fabric ID Mask 1]
// DF3p5	NodeIdShift	[3:0]
//
// D18F4x1B4 [System Fabric ID Mask 1]
// DF4	NodeIdShift	[3:0]
// DF4p5	NodeIdShift	[3:0]
//

//
// Remap Enable
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// DF2	N/A
// DF3	N/A
// DF3p5	N/A
//
// D18F7xE08 [DRAM Address Control]
// DF4	RemapEn		[4]
//
// D18F7x208 [DRAM Address Control]
// DF4p5	RemapEn		[4]
//

//
// Remap Select
//
// Access type: Instance
//
// Register
// Rev	Fieldname	Bits
//
// DF2	N/A
// DF3	N/A
// DF3p5	N/A
//
// D18F7xE08 [DRAM Address Control]
// DF4	RemapSel	[7:5]
//
// D18F7x208 [DRAM Address Control]
// DF4p5	RemapSel	[6:5]
//

//
// Socket ID Mask
//
// Access type: Broadcast
//
// Register
// Rev	Fieldname	Bits
//
// D18F1x208 [System Fabric ID Mask]
// DF2	SocketIdMask	[23:16]
//
// D18F1x20C [System Fabric ID Mask 1]
// DF3	SocketIdMask	[26:24]
//
// D18F1x158 [System Fabric ID Mask 2]
// DF3p5	SocketIdMask	[31:16]
//
// D18F4x1B8 [System Fabric ID Mask 2]
// DF4	SocketIdMask	[31:16]
// DF4p5	SocketIdMask	[31:16]
//

//
// Socket ID Shift
//
// Access type: Broadcast
//
// Register
// Rev	Fieldname	Bits
//
// D18F1x208 [System Fabric ID Mask]
// DF2	SocketIdShift	[31:28]
//
// D18F1x20C [System Fabric ID Mask 1]
// DF3	SocketIdShift	[9:8]
//
// D18F1x158 [System Fabric ID Mask 2]
// DF3p5	SocketIdShift	[11:8]
//
// D18F4x1B4 [System Fabric ID Mask 1]
// DF4	SocketIdShift	[11:8]
// DF4p5	SocketIdShift	[11:8]
//

