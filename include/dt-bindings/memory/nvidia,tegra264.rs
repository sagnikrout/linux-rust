//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/nvidia,tegra264.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.

//
// SMMU stream IDs
//

//
// memory client IDs
//
// PTW read client mapped to SOC SMMU0
pub const TEGRA264_MEMORY_CLIENT_PTCR: c_uint = 0x00;
// HOST1X read client
pub const TEGRA264_MEMORY_CLIENT_HOST1XR: c_uint = 0x16;
pub const TEGRA264_MEMORY_CLIENT_MPCORER: c_uint = 0x27;
// Platform security (PSC) Read clients
pub const TEGRA264_MEMORY_CLIENT_PSCR: c_uint = 0x33;
// PSC Write clients
pub const TEGRA264_MEMORY_CLIENT_PSCW: c_uint = 0x34;
// ISP0 Read client
pub const TEGRA264_MEMORY_CLIENT_ISP0R: c_uint = 0x37;
pub const TEGRA264_MEMORY_CLIENT_MPCOREW: c_uint = 0x39;
// ISP0 Write client
pub const TEGRA264_MEMORY_CLIENT_ISP0W: c_uint = 0x44;
// ISP1 Write client
pub const TEGRA264_MEMORY_CLIENT_ISP1W: c_uint = 0x45;
// ISP FALCON Read client
pub const TEGRA264_MEMORY_CLIENT_ISPFALCONR: c_uint = 0x47;
// ISP FALCON Write client
pub const TEGRA264_MEMORY_CLIENT_ISPFALCONW: c_uint = 0x4f;
// MGBE2 Read mccif
pub const TEGRA264_MEMORY_CLIENT_MGBE2R: c_uint = 0x5c;
pub const TEGRA264_MEMORY_CLIENT_OFAR2MC: c_uint = 0x5d;
pub const TEGRA264_MEMORY_CLIENT_OFAW2MC: c_uint = 0x5e;
// MGBE2 Write mccif
pub const TEGRA264_MEMORY_CLIENT_MGBE2W: c_uint = 0x5f;
// MGBE3 Read mccif
pub const TEGRA264_MEMORY_CLIENT_MGBE3R: c_uint = 0x61;
// MGBE3 Write mccif
pub const TEGRA264_MEMORY_CLIENT_MGBE3W: c_uint = 0x65;
// SEU1 Memory Read Client
pub const TEGRA264_MEMORY_CLIENT_SEU1RD: c_uint = 0x68;
// SEU1 Memory Write Client
pub const TEGRA264_MEMORY_CLIENT_SEU1WR: c_uint = 0x69;
// VIC read client
pub const TEGRA264_MEMORY_CLIENT_VICR: c_uint = 0x6c;
// VIC Write client
pub const TEGRA264_MEMORY_CLIENT_VICW: c_uint = 0x6d;
// VI R5 Write client
pub const TEGRA264_MEMORY_CLIENT_VIW: c_uint = 0x72;
// QSPI Read Client
pub const TEGRA264_MEMORY_CLIENT_XSPI0R: c_uint = 0x75;
// QSPI Write Client
pub const TEGRA264_MEMORY_CLIENT_XSPI0W: c_uint = 0x76;
pub const TEGRA264_MEMORY_CLIENT_NVDECSRD2MC: c_uint = 0x78;
pub const TEGRA264_MEMORY_CLIENT_NVDECSWR2MC: c_uint = 0x79;
// Audio processor(APE) Read client
pub const TEGRA264_MEMORY_CLIENT_APER: c_uint = 0x7a;
// Audio processor(APE) Write client
pub const TEGRA264_MEMORY_CLIENT_APEW: c_uint = 0x7b;
// SEU0 read client
pub const TEGRA264_MEMORY_CLIENT_SER: c_uint = 0x80;
// SEU0 write client
pub const TEGRA264_MEMORY_CLIENT_SEW: c_uint = 0x81;
// AXI AP and DFD/Coresight1-AUX0/1 Read clients both share the same interface on MSS
pub const TEGRA264_MEMORY_CLIENT_AXIAPR: c_uint = 0x82;
// AXI AP and DFD/Coresight1-AUX0/1 Write clients both share the same interface on MSS
pub const TEGRA264_MEMORY_CLIENT_AXIAPW: c_uint = 0x83;
// ETR or DFD/Coresight0 Read Client
pub const TEGRA264_MEMORY_CLIENT_ETRR: c_uint = 0x84;
// ETR or DFD/Coresight0 Write Client
pub const TEGRA264_MEMORY_CLIENT_ETRW: c_uint = 0x85;
// Security(tsec) Read client
pub const TEGRA264_MEMORY_CLIENT_TSECR: c_uint = 0x86;
// Security(tsec) Write client
pub const TEGRA264_MEMORY_CLIENT_TSECW: c_uint = 0x87;
// BPMP read client
pub const TEGRA264_MEMORY_CLIENT_BPMPR: c_uint = 0x93;
// BPMP write client
pub const TEGRA264_MEMORY_CLIENT_BPMPW: c_uint = 0x94;
// AON Read Client
pub const TEGRA264_MEMORY_CLIENT_AONR: c_uint = 0x97;
// AON write client
pub const TEGRA264_MEMORY_CLIENT_AONW: c_uint = 0x98;
// GPCDMA debug Read client
pub const TEGRA264_MEMORY_CLIENT_GPCDMAR: c_uint = 0x99;
// GPCDMA debug Write client
pub const TEGRA264_MEMORY_CLIENT_GPCDMAW: c_uint = 0x9a;
// Audio DMA Read client
pub const TEGRA264_MEMORY_CLIENT_APEDMAR: c_uint = 0x9f;
// Audio DMA Write client
pub const TEGRA264_MEMORY_CLIENT_APEDMAW: c_uint = 0xa0;
// mss internal memqual MIU0 reads
pub const TEGRA264_MEMORY_CLIENT_MIU0R: c_uint = 0xa6;
// mss internal memqual MIU0 writes
pub const TEGRA264_MEMORY_CLIENT_MIU0W: c_uint = 0xa7;
// mss internal memqual MIU1 reads
pub const TEGRA264_MEMORY_CLIENT_MIU1R: c_uint = 0xa8;
// mss internal memqual MIU1 writes
pub const TEGRA264_MEMORY_CLIENT_MIU1W: c_uint = 0xa9;
// mss internal memqual MIU2 reads
pub const TEGRA264_MEMORY_CLIENT_MIU2R: c_uint = 0xae;
// mss internal memqual MIU2 writes
pub const TEGRA264_MEMORY_CLIENT_MIU2W: c_uint = 0xaf;
// mss internal memqual MIU3 reads
pub const TEGRA264_MEMORY_CLIENT_MIU3R: c_uint = 0xb0;
// mss internal memqual MIU3 writes
pub const TEGRA264_MEMORY_CLIENT_MIU3W: c_uint = 0xb1;
// mss internal memqual MIU4 reads
pub const TEGRA264_MEMORY_CLIENT_MIU4R: c_uint = 0xb2;
// mss internal memqual MIU4 writes
pub const TEGRA264_MEMORY_CLIENT_MIU4W: c_uint = 0xb3;
pub const TEGRA264_MEMORY_CLIENT_GPUR02MC: c_uint = 0xb6;
pub const TEGRA264_MEMORY_CLIENT_GPUW02MC: c_uint = 0xb7;
// VI Falcon Read client
pub const TEGRA264_MEMORY_CLIENT_VIFALCONR: c_uint = 0xbc;
// VI Falcon Write client
pub const TEGRA264_MEMORY_CLIENT_VIFALCONW: c_uint = 0xbd;
// Read Client of RCE
pub const TEGRA264_MEMORY_CLIENT_RCER: c_uint = 0xd2;
// Write client of RCE
pub const TEGRA264_MEMORY_CLIENT_RCEW: c_uint = 0xd3;
pub const TEGRA264_MEMORY_CLIENT_NVENC1SRD2MC: c_uint = 0xd6;
pub const TEGRA264_MEMORY_CLIENT_NVENC1SWR2MC: c_uint = 0xd7;
// PCIE0/MSI Write clients
pub const TEGRA264_MEMORY_CLIENT_PCIE0W: c_uint = 0xd9;
// PCIE1/RPX4 Read clients
pub const TEGRA264_MEMORY_CLIENT_PCIE1R: c_uint = 0xda;
// PCIE1/RPX4 Write clients
pub const TEGRA264_MEMORY_CLIENT_PCIE1W: c_uint = 0xdb;
// PCIE2/DMX4 Read clients
pub const TEGRA264_MEMORY_CLIENT_PCIE2AR: c_uint = 0xdc;
// PCIE2/DMX4 Write clients
pub const TEGRA264_MEMORY_CLIENT_PCIE2AW: c_uint = 0xdd;
// PCIE3/RPX4 Read clients
pub const TEGRA264_MEMORY_CLIENT_PCIE3R: c_uint = 0xde;
// PCIE3/RPX4 Write clients
pub const TEGRA264_MEMORY_CLIENT_PCIE3W: c_uint = 0xdf;
// PCIE4/DMX8 Read clients
pub const TEGRA264_MEMORY_CLIENT_PCIE4R: c_uint = 0xe0;
// PCIE4/DMX8 Write clients
pub const TEGRA264_MEMORY_CLIENT_PCIE4W: c_uint = 0xe1;
// PCIE5/DMX4 Read clients
pub const TEGRA264_MEMORY_CLIENT_PCIE5R: c_uint = 0xe2;
// PCIE5/DMX4 Write clients
pub const TEGRA264_MEMORY_CLIENT_PCIE5W: c_uint = 0xe3;
// mss internal memqual MIU5 reads
pub const TEGRA264_MEMORY_CLIENT_MIU5R: c_uint = 0xfc;
// mss internal memqual MIU5 writes
pub const TEGRA264_MEMORY_CLIENT_MIU5W: c_uint = 0xfd;
// mss internal memqual MIU6 writes
pub const TEGRA264_MEMORY_CLIENT_MIU6W: c_uint = 0xff;
pub const TEGRA264_MEMORY_CLIENT_RISTR: c_uint = 0x100;
pub const TEGRA264_MEMORY_CLIENT_RISTW: c_uint = 0x101;
// OESP (Pluton) Read client
pub const TEGRA264_MEMORY_CLIENT_OESPR: c_uint = 0x102;
// OESP (Pluton) Write client
pub const TEGRA264_MEMORY_CLIENT_OESPW: c_uint = 0x103;
// mss internal memqual MIU7 writes
pub const TEGRA264_MEMORY_CLIENT_MIU7W: c_uint = 0x105;
// mss internal memqual MIU8 reads
pub const TEGRA264_MEMORY_CLIENT_MIU8R: c_uint = 0x106;
// mss internal memqual MIU8 writes
pub const TEGRA264_MEMORY_CLIENT_MIU8W: c_uint = 0x107;
// mss internal memqual MIU9 reads
pub const TEGRA264_MEMORY_CLIENT_MIU9R: c_uint = 0x108;
// mss internal memqual MIU9 writes
pub const TEGRA264_MEMORY_CLIENT_MIU9W: c_uint = 0x109;
// HWPM Write Interface
pub const TEGRA264_MEMORY_CLIENT_PMA0AWR: c_uint = 0x122;
pub const TEGRA264_MEMORY_CLIENT_NVJPG1SRD2MC: c_uint = 0x123;
pub const TEGRA264_MEMORY_CLIENT_NVJPG1SWR2MC: c_uint = 0x124;
// CTW read client mapped to SMMU0
pub const TEGRA264_MEMORY_CLIENT_SMMU0CTWR: c_uint = 0x12e;
// CMDQV read client mapped to SMMU0
pub const TEGRA264_MEMORY_CLIENT_SMMU0CMDQVR: c_uint = 0x12f;
// CMDQV write client mapped to SMMU0
pub const TEGRA264_MEMORY_CLIENT_SMMU0CMDQVW: c_uint = 0x130;
// EVNTQ write client mapped to SMMU0
pub const TEGRA264_MEMORY_CLIENT_SMMU0EVNTQW: c_uint = 0x131;
// PTW read client mapped to SMMU1
pub const TEGRA264_MEMORY_CLIENT_SMMU1PTWR: c_uint = 0x132;
// CTW read client mapped to SMMU1
pub const TEGRA264_MEMORY_CLIENT_SMMU1CTWR: c_uint = 0x134;
// CMDQV read client mapped to SMMU1
pub const TEGRA264_MEMORY_CLIENT_SMMU1CMDQVR: c_uint = 0x135;
// CMDQV write client mapped to SMMU1
pub const TEGRA264_MEMORY_CLIENT_SMMU1CMDQVW: c_uint = 0x136;
// EVNTQ write client mapped to SMMU1
pub const TEGRA264_MEMORY_CLIENT_SMMU1EVNTQW: c_uint = 0x137;
// PTW read client mapped to SMMU2
pub const TEGRA264_MEMORY_CLIENT_SMMU2PTWR: c_uint = 0x138;
// CTW read client mapped to SMMU2
pub const TEGRA264_MEMORY_CLIENT_SMMU2CTWR: c_uint = 0x13a;
// CMDQV read client mapped to SMMU2
pub const TEGRA264_MEMORY_CLIENT_SMMU2CMDQVR: c_uint = 0x13b;
// CMDQV write client mapped to SMMU2
pub const TEGRA264_MEMORY_CLIENT_SMMU2CMDQVW: c_uint = 0x13c;
// EVNTQ write client mapped to SMMU2
pub const TEGRA264_MEMORY_CLIENT_SMMU2EVNTQW: c_uint = 0x13d;
// CMDQ read client mapped to SMMU0
pub const TEGRA264_MEMORY_CLIENT_SMMU0CMDQR: c_uint = 0x144;
// CMDQ read client mapped to SMMU1
pub const TEGRA264_MEMORY_CLIENT_SMMU1CMDQR: c_uint = 0x145;
// CMDQ read client mapped to SMMU2
pub const TEGRA264_MEMORY_CLIENT_SMMU2CMDQR: c_uint = 0x146;
// Audio processor1(APE1) Read client
pub const TEGRA264_MEMORY_CLIENT_APE1R: c_uint = 0x150;
// Audio processor1(APE1) Write client
pub const TEGRA264_MEMORY_CLIENT_APE1W: c_uint = 0x151;
// UFS Read client
pub const TEGRA264_MEMORY_CLIENT_UFSR: c_uint = 0x15c;
// UFS write client
pub const TEGRA264_MEMORY_CLIENT_UFSW: c_uint = 0x15d;
// XUSB HOST Read Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEVR: c_uint = 0x166;
// XUSB HOST Write Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEVW: c_uint = 0x167;
// XUSB SS0 Read Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV1R: c_uint = 0x168;
// XUSB SS1 Write Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV2W: c_uint = 0x169;
// XUSB SS2 Read Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV3R: c_uint = 0x16a;
// XUSB SS2 Write Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV3W: c_uint = 0x16b;
// XUSB SS3 Read Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV4R: c_uint = 0x16c;
// XUSB SS3 Write Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV4W: c_uint = 0x16d;
// XUSB DEV Read Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV5R: c_uint = 0x16e;
// XUSB DEV Write Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV5W: c_uint = 0x16f;
// DCE Read client
pub const TEGRA264_MEMORY_CLIENT_DCER: c_uint = 0x17a;
// DCE Write client
pub const TEGRA264_MEMORY_CLIENT_DCEW: c_uint = 0x17b;
// HDA Read client
pub const TEGRA264_MEMORY_CLIENT_HDAR: c_uint = 0x17c;
// HDA Write client
pub const TEGRA264_MEMORY_CLIENT_HDAW: c_uint = 0x17d;
// DISPNISO read client
pub const TEGRA264_MEMORY_CLIENT_DISPNISOR: c_uint = 0x17e;
// DISPNISO write client
pub const TEGRA264_MEMORY_CLIENT_DISPNISOW: c_uint = 0x17f;
// XUSB SS0 Write Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV1W: c_uint = 0x180;
// XUSB SS1 Read Client
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV2R: c_uint = 0x181;
// Disp ISO Read Client
pub const TEGRA264_MEMORY_CLIENT_DISPR: c_uint = 0x182;
// MSSSEQ Read Client
pub const TEGRA264_MEMORY_CLIENT_MSSSEQR: c_uint = 0x185;
// MSSSEQ Write Client
pub const TEGRA264_MEMORY_CLIENT_MSSSEQW: c_uint = 0x186;
// PTW read client mapped to SMMU3
pub const TEGRA264_MEMORY_CLIENT_SMMU3PTWR: c_uint = 0x18b;
// CTW read client mapped to SMMU3
pub const TEGRA264_MEMORY_CLIENT_SMMU3CTWR: c_uint = 0x18d;
// CMDQV read client mapped to SMMU3
pub const TEGRA264_MEMORY_CLIENT_SMMU3CMDQVR: c_uint = 0x18e;
// CMDQV write client mapped to SMMU3
pub const TEGRA264_MEMORY_CLIENT_SMMU3CMDQVW: c_uint = 0x18f;
// EVNTQ write client mapped to SMMU3
pub const TEGRA264_MEMORY_CLIENT_SMMU3EVNTQW: c_uint = 0x190;
// CMDQ read client mapped to SMMU3
pub const TEGRA264_MEMORY_CLIENT_SMMU3CMDQR: c_uint = 0x191;
// PTW read client mapped to SMMU4
pub const TEGRA264_MEMORY_CLIENT_SMMU4PTWR: c_uint = 0x192;
// CTW read client mapped to SMMU4
pub const TEGRA264_MEMORY_CLIENT_SMMU4CTWR: c_uint = 0x194;
// CMDQV read client mapped to SMMU4
pub const TEGRA264_MEMORY_CLIENT_SMMU4CMDQVR: c_uint = 0x195;
// CMDQV write client mapped to SMMU4
pub const TEGRA264_MEMORY_CLIENT_SMMU4CMDQVW: c_uint = 0x196;
// EVNTQ write client mapped to SMMU4
pub const TEGRA264_MEMORY_CLIENT_SMMU4EVNTQW: c_uint = 0x197;
// CMDQ read client mapped to SMMU4
pub const TEGRA264_MEMORY_CLIENT_SMMU4CMDQR: c_uint = 0x198;
// MGBE0 Read mccif
pub const TEGRA264_MEMORY_CLIENT_MGBE0R: c_uint = 0x1a2;
// MGBE0 Write mccif
pub const TEGRA264_MEMORY_CLIENT_MGBE0W: c_uint = 0x1a3;
// MGBE1 Read mccif
pub const TEGRA264_MEMORY_CLIENT_MGBE1R: c_uint = 0x1a4;
// MGBE1 Write mccif
pub const TEGRA264_MEMORY_CLIENT_MGBE1W: c_uint = 0x1a5;
// VI1 R5 Write client
pub const TEGRA264_MEMORY_CLIENT_VI1W: c_uint = 0x1a6;
// VI Falcon1 Read client
pub const TEGRA264_MEMORY_CLIENT_VIFALCON1R: c_uint = 0x1a7;
// VI Falcon1 Write client
pub const TEGRA264_MEMORY_CLIENT_VIFALCON1W: c_uint = 0x1a8;
// ISP FALCON1 Read client
pub const TEGRA264_MEMORY_CLIENT_ISPFALCON1R: c_uint = 0x1a9;
// ISP FALCON1 Write client
pub const TEGRA264_MEMORY_CLIENT_ISPFALCON1W: c_uint = 0x1aa;
// Read Client of RCE1
pub const TEGRA264_MEMORY_CLIENT_RCE1R: c_uint = 0x1ab;
// Write client of RCE1
pub const TEGRA264_MEMORY_CLIENT_RCE1W: c_uint = 0x1ac;
// SEU2 Read client
pub const TEGRA264_MEMORY_CLIENT_SEU2R: c_uint = 0x1ad;
// SEU2 Write client
pub const TEGRA264_MEMORY_CLIENT_SEU2W: c_uint = 0x1ae;
// SEU3 Read client
pub const TEGRA264_MEMORY_CLIENT_SEU3R: c_uint = 0x1af;
// SEU3 Write client
pub const TEGRA264_MEMORY_CLIENT_SEU3W: c_uint = 0x1b0;
// PVA0 Falcon Read mccif
pub const TEGRA264_MEMORY_CLIENT_PVA0R: c_uint = 0x1b1;
// PVA0 Falcon Write mccif
pub const TEGRA264_MEMORY_CLIENT_PVA0W: c_uint = 0x1b2;
// PVA1 Read mccif
pub const TEGRA264_MEMORY_CLIENT_PVA1R: c_uint = 0x1b3;
// PVA1 Write mccif
pub const TEGRA264_MEMORY_CLIENT_PVA1W: c_uint = 0x1b4;
// PVA2 Read mccif
pub const TEGRA264_MEMORY_CLIENT_PVA2R: c_uint = 0x1b5;
// PVA2 Write mccif
pub const TEGRA264_MEMORY_CLIENT_PVA2W: c_uint = 0x1b6;
// ISP3 Write client
pub const TEGRA264_MEMORY_CLIENT_ISP3W: c_uint = 0x1b7;
// ISP2 Read Client
pub const TEGRA264_MEMORY_CLIENT_ISP2R: c_uint = 0x1b8;
// ISP2 Write Client
pub const TEGRA264_MEMORY_CLIENT_ISP2W: c_uint = 0x1b9;
// EQOS Read mccif
pub const TEGRA264_MEMORY_CLIENT_EQOSR: c_uint = 0x1bc;
// EQOS Write mccif
pub const TEGRA264_MEMORY_CLIENT_EQOSW: c_uint = 0x1bd;
// FSI0 Read mccif
pub const TEGRA264_MEMORY_CLIENT_FSI0R: c_uint = 0x1be;
// FSI0 Write mccif
pub const TEGRA264_MEMORY_CLIENT_FSI0W: c_uint = 0x1bf;
// FSI1 Read mccif
pub const TEGRA264_MEMORY_CLIENT_FSI1R: c_uint = 0x1c0;
// FSI1 Write mccif
pub const TEGRA264_MEMORY_CLIENT_FSI1W: c_uint = 0x1c1;
// SDMMC0 Read mccif
pub const TEGRA264_MEMORY_CLIENT_SDMMC0R: c_uint = 0x1c2;
// SDMMC0 Write mccif
pub const TEGRA264_MEMORY_CLIENT_SDMMC0W: c_uint = 0x1c3;
// Strongbox (SB) read client
pub const TEGRA264_MEMORY_CLIENT_SBR: c_uint = 0x1c6;
// Strongbox (SB) write client
pub const TEGRA264_MEMORY_CLIENT_SBW: c_uint = 0x1c7;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU0R: c_uint = 0x1c8;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU0W: c_uint = 0x1c9;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU1R: c_uint = 0x1ca;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU1W: c_uint = 0x1cb;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU2R: c_uint = 0x1cc;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU2W: c_uint = 0x1cd;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU3R: c_uint = 0x1ce;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU3W: c_uint = 0x1cf;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU4R: c_uint = 0x1d0;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU4W: c_uint = 0x1d1;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU5R: c_uint = 0x1d2;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU5W: c_uint = 0x1d3;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU6R: c_uint = 0x1d4;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU6W: c_uint = 0x1d5;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU7R: c_uint = 0x1d6;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU7W: c_uint = 0x1d7;
pub const TEGRA264_MEMORY_CLIENT_GMMUR2MC: c_uint = 0x1d8;
pub const TEGRA264_MEMORY_CLIENT_UCFELAR: c_uint = 0x1d9;
pub const TEGRA264_MEMORY_CLIENT_UCFELAW: c_uint = 0x1da;
pub const TEGRA264_MEMORY_CLIENT_SLCR: c_uint = 0x1db;
pub const TEGRA264_MEMORY_CLIENT_SLCW: c_uint = 0x1dc;
pub const TEGRA264_MEMORY_CLIENT_REMOTER: c_uint = 0x1dd;
pub const TEGRA264_MEMORY_CLIENT_REMOTEW: c_uint = 0x1de;
