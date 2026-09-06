//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/tegra194-mc.h
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


// special clients
pub const TEGRA194_SID_INVALID: c_uint = 0x00;
pub const TEGRA194_SID_PASSTHROUGH: c_uint = 0x7f;
// host1x clients
pub const TEGRA194_SID_HOST1X: c_uint = 0x01;
pub const TEGRA194_SID_CSI: c_uint = 0x02;
pub const TEGRA194_SID_VIC: c_uint = 0x03;
pub const TEGRA194_SID_VI: c_uint = 0x04;
pub const TEGRA194_SID_ISP: c_uint = 0x05;
pub const TEGRA194_SID_NVDEC: c_uint = 0x06;
pub const TEGRA194_SID_NVENC: c_uint = 0x07;
pub const TEGRA194_SID_NVJPG: c_uint = 0x08;
pub const TEGRA194_SID_NVDISPLAY: c_uint = 0x09;
pub const TEGRA194_SID_TSEC: c_uint = 0x0a;
pub const TEGRA194_SID_TSECB: c_uint = 0x0b;
pub const TEGRA194_SID_SE: c_uint = 0x0c;
pub const TEGRA194_SID_SE1: c_uint = 0x0d;
pub const TEGRA194_SID_SE2: c_uint = 0x0e;
pub const TEGRA194_SID_SE3: c_uint = 0x0f;
// GPU clients
pub const TEGRA194_SID_GPU: c_uint = 0x10;
// other SoC clients
pub const TEGRA194_SID_AFI: c_uint = 0x11;
pub const TEGRA194_SID_HDA: c_uint = 0x12;
pub const TEGRA194_SID_ETR: c_uint = 0x13;
pub const TEGRA194_SID_EQOS: c_uint = 0x14;
pub const TEGRA194_SID_UFSHC: c_uint = 0x15;
pub const TEGRA194_SID_AON: c_uint = 0x16;
pub const TEGRA194_SID_SDMMC4: c_uint = 0x17;
pub const TEGRA194_SID_SDMMC3: c_uint = 0x18;
pub const TEGRA194_SID_SDMMC2: c_uint = 0x19;
pub const TEGRA194_SID_SDMMC1: c_uint = 0x1a;
pub const TEGRA194_SID_XUSB_HOST: c_uint = 0x1b;
pub const TEGRA194_SID_XUSB_DEV: c_uint = 0x1c;
pub const TEGRA194_SID_SATA: c_uint = 0x1d;
pub const TEGRA194_SID_APE: c_uint = 0x1e;
pub const TEGRA194_SID_SCE: c_uint = 0x1f;
// GPC DMA clients
pub const TEGRA194_SID_GPCDMA_0: c_uint = 0x20;
pub const TEGRA194_SID_GPCDMA_1: c_uint = 0x21;
pub const TEGRA194_SID_GPCDMA_2: c_uint = 0x22;
pub const TEGRA194_SID_GPCDMA_3: c_uint = 0x23;
pub const TEGRA194_SID_GPCDMA_4: c_uint = 0x24;
pub const TEGRA194_SID_GPCDMA_5: c_uint = 0x25;
pub const TEGRA194_SID_GPCDMA_6: c_uint = 0x26;
pub const TEGRA194_SID_GPCDMA_7: c_uint = 0x27;
// APE DMA clients
pub const TEGRA194_SID_APE_1: c_uint = 0x28;
pub const TEGRA194_SID_APE_2: c_uint = 0x29;
// camera RTCPU
pub const TEGRA194_SID_RCE: c_uint = 0x2a;
// camera RTCPU on host1x address space
pub const TEGRA194_SID_RCE_1X: c_uint = 0x2b;
// APE DMA clients
pub const TEGRA194_SID_APE_3: c_uint = 0x2c;
// camera RTCPU running on APE
pub const TEGRA194_SID_APE_CAM: c_uint = 0x2d;
pub const TEGRA194_SID_APE_CAM_1X: c_uint = 0x2e;
pub const TEGRA194_SID_RCE_RM: c_uint = 0x2f;
pub const TEGRA194_SID_VI_FALCON: c_uint = 0x30;
pub const TEGRA194_SID_ISP_FALCON: c_uint = 0x31;
//
// The BPMP has its SID value hardcoded in the firmware. Changing it requires
// considerable effort.
//
pub const TEGRA194_SID_BPMP: c_uint = 0x32;
// for SMMU tests
pub const TEGRA194_SID_SMMU_TEST: c_uint = 0x33;
// host1x virtualization channels
pub const TEGRA194_SID_HOST1X_CTX0: c_uint = 0x38;
pub const TEGRA194_SID_HOST1X_CTX1: c_uint = 0x39;
pub const TEGRA194_SID_HOST1X_CTX2: c_uint = 0x3a;
pub const TEGRA194_SID_HOST1X_CTX3: c_uint = 0x3b;
pub const TEGRA194_SID_HOST1X_CTX4: c_uint = 0x3c;
pub const TEGRA194_SID_HOST1X_CTX5: c_uint = 0x3d;
pub const TEGRA194_SID_HOST1X_CTX6: c_uint = 0x3e;
pub const TEGRA194_SID_HOST1X_CTX7: c_uint = 0x3f;
// host1x command buffers
pub const TEGRA194_SID_HOST1X_VM0: c_uint = 0x40;
pub const TEGRA194_SID_HOST1X_VM1: c_uint = 0x41;
pub const TEGRA194_SID_HOST1X_VM2: c_uint = 0x42;
pub const TEGRA194_SID_HOST1X_VM3: c_uint = 0x43;
pub const TEGRA194_SID_HOST1X_VM4: c_uint = 0x44;
pub const TEGRA194_SID_HOST1X_VM5: c_uint = 0x45;
pub const TEGRA194_SID_HOST1X_VM6: c_uint = 0x46;
pub const TEGRA194_SID_HOST1X_VM7: c_uint = 0x47;
// SE data buffers
pub const TEGRA194_SID_SE_VM0: c_uint = 0x48;
pub const TEGRA194_SID_SE_VM1: c_uint = 0x49;
pub const TEGRA194_SID_SE_VM2: c_uint = 0x4a;
pub const TEGRA194_SID_SE_VM3: c_uint = 0x4b;
pub const TEGRA194_SID_SE_VM4: c_uint = 0x4c;
pub const TEGRA194_SID_SE_VM5: c_uint = 0x4d;
pub const TEGRA194_SID_SE_VM6: c_uint = 0x4e;
pub const TEGRA194_SID_SE_VM7: c_uint = 0x4f;
pub const TEGRA194_SID_MIU: c_uint = 0x50;
pub const TEGRA194_SID_NVDLA0: c_uint = 0x51;
pub const TEGRA194_SID_NVDLA1: c_uint = 0x52;
pub const TEGRA194_SID_PVA0: c_uint = 0x53;
pub const TEGRA194_SID_PVA1: c_uint = 0x54;
pub const TEGRA194_SID_NVENC1: c_uint = 0x55;
pub const TEGRA194_SID_PCIE0: c_uint = 0x56;
pub const TEGRA194_SID_PCIE1: c_uint = 0x57;
pub const TEGRA194_SID_PCIE2: c_uint = 0x58;
pub const TEGRA194_SID_PCIE3: c_uint = 0x59;
pub const TEGRA194_SID_PCIE4: c_uint = 0x5a;
pub const TEGRA194_SID_PCIE5: c_uint = 0x5b;
pub const TEGRA194_SID_NVDEC1: c_uint = 0x5c;
pub const TEGRA194_SID_XUSB_VF0: c_uint = 0x5d;
pub const TEGRA194_SID_XUSB_VF1: c_uint = 0x5e;
pub const TEGRA194_SID_XUSB_VF2: c_uint = 0x5f;
pub const TEGRA194_SID_XUSB_VF3: c_uint = 0x60;
pub const TEGRA194_SID_RCE_VM3: c_uint = 0x61;
pub const TEGRA194_SID_VI_VM2: c_uint = 0x62;
pub const TEGRA194_SID_VI_VM3: c_uint = 0x63;
pub const TEGRA194_SID_RCE_SERVER: c_uint = 0x64;
//
// memory client IDs
//
// Misses from System Memory Management Unit (SMMU) Page Table Cache (PTC)
pub const TEGRA194_MEMORY_CLIENT_PTCR: c_uint = 0x00;
// MSS internal memqual MIU7 read clients
pub const TEGRA194_MEMORY_CLIENT_MIU7R: c_uint = 0x01;
// MSS internal memqual MIU7 write clients
pub const TEGRA194_MEMORY_CLIENT_MIU7W: c_uint = 0x02;
// High-definition audio (HDA) read clients
pub const TEGRA194_MEMORY_CLIENT_HDAR: c_uint = 0x15;
// Host channel data read clients
pub const TEGRA194_MEMORY_CLIENT_HOST1XDMAR: c_uint = 0x16;
pub const TEGRA194_MEMORY_CLIENT_NVENCSRD: c_uint = 0x1c;
// SATA read clients
pub const TEGRA194_MEMORY_CLIENT_SATAR: c_uint = 0x1f;
// Reads from Cortex-A9 4 CPU cores via the L2 cache
pub const TEGRA194_MEMORY_CLIENT_MPCORER: c_uint = 0x27;
pub const TEGRA194_MEMORY_CLIENT_NVENCSWR: c_uint = 0x2b;
// High-definition audio (HDA) write clients
pub const TEGRA194_MEMORY_CLIENT_HDAW: c_uint = 0x35;
// Writes from Cortex-A9 4 CPU cores via the L2 cache
pub const TEGRA194_MEMORY_CLIENT_MPCOREW: c_uint = 0x39;
// SATA write clients
pub const TEGRA194_MEMORY_CLIENT_SATAW: c_uint = 0x3d;
// ISP read client for Crossbar A
pub const TEGRA194_MEMORY_CLIENT_ISPRA: c_uint = 0x44;
// ISP read client 1 for Crossbar A
pub const TEGRA194_MEMORY_CLIENT_ISPFALR: c_uint = 0x45;
// ISP Write client for Crossbar A
pub const TEGRA194_MEMORY_CLIENT_ISPWA: c_uint = 0x46;
// ISP Write client Crossbar B
pub const TEGRA194_MEMORY_CLIENT_ISPWB: c_uint = 0x47;
// XUSB_HOST read clients
pub const TEGRA194_MEMORY_CLIENT_XUSB_HOSTR: c_uint = 0x4a;
// XUSB_HOST write clients
pub const TEGRA194_MEMORY_CLIENT_XUSB_HOSTW: c_uint = 0x4b;
// XUSB read clients
pub const TEGRA194_MEMORY_CLIENT_XUSB_DEVR: c_uint = 0x4c;
// XUSB_DEV write clients
pub const TEGRA194_MEMORY_CLIENT_XUSB_DEVW: c_uint = 0x4d;
// sdmmca memory read client
pub const TEGRA194_MEMORY_CLIENT_SDMMCRA: c_uint = 0x60;
// sdmmc memory read client
pub const TEGRA194_MEMORY_CLIENT_SDMMCR: c_uint = 0x62;
// sdmmcd memory read client
pub const TEGRA194_MEMORY_CLIENT_SDMMCRAB: c_uint = 0x63;
// sdmmca memory write client
pub const TEGRA194_MEMORY_CLIENT_SDMMCWA: c_uint = 0x64;
// sdmmc memory write client
pub const TEGRA194_MEMORY_CLIENT_SDMMCW: c_uint = 0x66;
// sdmmcd memory write client
pub const TEGRA194_MEMORY_CLIENT_SDMMCWAB: c_uint = 0x67;
pub const TEGRA194_MEMORY_CLIENT_VICSRD: c_uint = 0x6c;
pub const TEGRA194_MEMORY_CLIENT_VICSWR: c_uint = 0x6d;
// VI Write client
pub const TEGRA194_MEMORY_CLIENT_VIW: c_uint = 0x72;
pub const TEGRA194_MEMORY_CLIENT_NVDECSRD: c_uint = 0x78;
pub const TEGRA194_MEMORY_CLIENT_NVDECSWR: c_uint = 0x79;
// Audio Processing (APE) engine read clients
pub const TEGRA194_MEMORY_CLIENT_APER: c_uint = 0x7a;
// Audio Processing (APE) engine write clients
pub const TEGRA194_MEMORY_CLIENT_APEW: c_uint = 0x7b;
pub const TEGRA194_MEMORY_CLIENT_NVJPGSRD: c_uint = 0x7e;
pub const TEGRA194_MEMORY_CLIENT_NVJPGSWR: c_uint = 0x7f;
// AXI AP and DFD-AUX0/1 read clients Both share the same interface on the on MSS
pub const TEGRA194_MEMORY_CLIENT_AXIAPR: c_uint = 0x82;
// AXI AP and DFD-AUX0/1 write clients Both sahre the same interface on MSS
pub const TEGRA194_MEMORY_CLIENT_AXIAPW: c_uint = 0x83;
// ETR read clients
pub const TEGRA194_MEMORY_CLIENT_ETRR: c_uint = 0x84;
// ETR write clients
pub const TEGRA194_MEMORY_CLIENT_ETRW: c_uint = 0x85;
// AXI Switch read client
pub const TEGRA194_MEMORY_CLIENT_AXISR: c_uint = 0x8c;
// AXI Switch write client
pub const TEGRA194_MEMORY_CLIENT_AXISW: c_uint = 0x8d;
// EQOS read client
pub const TEGRA194_MEMORY_CLIENT_EQOSR: c_uint = 0x8e;
// EQOS write client
pub const TEGRA194_MEMORY_CLIENT_EQOSW: c_uint = 0x8f;
// UFSHC read client
pub const TEGRA194_MEMORY_CLIENT_UFSHCR: c_uint = 0x90;
// UFSHC write client
pub const TEGRA194_MEMORY_CLIENT_UFSHCW: c_uint = 0x91;
// NVDISPLAY read client
pub const TEGRA194_MEMORY_CLIENT_NVDISPLAYR: c_uint = 0x92;
// BPMP read client
pub const TEGRA194_MEMORY_CLIENT_BPMPR: c_uint = 0x93;
// BPMP write client
pub const TEGRA194_MEMORY_CLIENT_BPMPW: c_uint = 0x94;
// BPMPDMA read client
pub const TEGRA194_MEMORY_CLIENT_BPMPDMAR: c_uint = 0x95;
// BPMPDMA write client
pub const TEGRA194_MEMORY_CLIENT_BPMPDMAW: c_uint = 0x96;
// AON read client
pub const TEGRA194_MEMORY_CLIENT_AONR: c_uint = 0x97;
// AON write client
pub const TEGRA194_MEMORY_CLIENT_AONW: c_uint = 0x98;
// AONDMA read client
pub const TEGRA194_MEMORY_CLIENT_AONDMAR: c_uint = 0x99;
// AONDMA write client
pub const TEGRA194_MEMORY_CLIENT_AONDMAW: c_uint = 0x9a;
// SCE read client
pub const TEGRA194_MEMORY_CLIENT_SCER: c_uint = 0x9b;
// SCE write client
pub const TEGRA194_MEMORY_CLIENT_SCEW: c_uint = 0x9c;
// SCEDMA read client
pub const TEGRA194_MEMORY_CLIENT_SCEDMAR: c_uint = 0x9d;
// SCEDMA write client
pub const TEGRA194_MEMORY_CLIENT_SCEDMAW: c_uint = 0x9e;
// APEDMA read client
pub const TEGRA194_MEMORY_CLIENT_APEDMAR: c_uint = 0x9f;
// APEDMA write client
pub const TEGRA194_MEMORY_CLIENT_APEDMAW: c_uint = 0xa0;
// NVDISPLAY read client instance 2
pub const TEGRA194_MEMORY_CLIENT_NVDISPLAYR1: c_uint = 0xa1;
pub const TEGRA194_MEMORY_CLIENT_VICSRD1: c_uint = 0xa2;
pub const TEGRA194_MEMORY_CLIENT_NVDECSRD1: c_uint = 0xa3;
// MSS internal memqual MIU0 read clients
pub const TEGRA194_MEMORY_CLIENT_MIU0R: c_uint = 0xa6;
// MSS internal memqual MIU0 write clients
pub const TEGRA194_MEMORY_CLIENT_MIU0W: c_uint = 0xa7;
// MSS internal memqual MIU1 read clients
pub const TEGRA194_MEMORY_CLIENT_MIU1R: c_uint = 0xa8;
// MSS internal memqual MIU1 write clients
pub const TEGRA194_MEMORY_CLIENT_MIU1W: c_uint = 0xa9;
// MSS internal memqual MIU2 read clients
pub const TEGRA194_MEMORY_CLIENT_MIU2R: c_uint = 0xae;
// MSS internal memqual MIU2 write clients
pub const TEGRA194_MEMORY_CLIENT_MIU2W: c_uint = 0xaf;
// MSS internal memqual MIU3 read clients
pub const TEGRA194_MEMORY_CLIENT_MIU3R: c_uint = 0xb0;
// MSS internal memqual MIU3 write clients
pub const TEGRA194_MEMORY_CLIENT_MIU3W: c_uint = 0xb1;
// MSS internal memqual MIU4 read clients
pub const TEGRA194_MEMORY_CLIENT_MIU4R: c_uint = 0xb2;
// MSS internal memqual MIU4 write clients
pub const TEGRA194_MEMORY_CLIENT_MIU4W: c_uint = 0xb3;
pub const TEGRA194_MEMORY_CLIENT_DPMUR: c_uint = 0xb4;
pub const TEGRA194_MEMORY_CLIENT_DPMUW: c_uint = 0xb5;
pub const TEGRA194_MEMORY_CLIENT_NVL0R: c_uint = 0xb6;
pub const TEGRA194_MEMORY_CLIENT_NVL0W: c_uint = 0xb7;
pub const TEGRA194_MEMORY_CLIENT_NVL1R: c_uint = 0xb8;
pub const TEGRA194_MEMORY_CLIENT_NVL1W: c_uint = 0xb9;
pub const TEGRA194_MEMORY_CLIENT_NVL2R: c_uint = 0xba;
pub const TEGRA194_MEMORY_CLIENT_NVL2W: c_uint = 0xbb;
// VI FLACON read clients
pub const TEGRA194_MEMORY_CLIENT_VIFALR: c_uint = 0xbc;
// VIFAL write clients
pub const TEGRA194_MEMORY_CLIENT_VIFALW: c_uint = 0xbd;
// DLA0ARDA read clients
pub const TEGRA194_MEMORY_CLIENT_DLA0RDA: c_uint = 0xbe;
// DLA0 Falcon read clients
pub const TEGRA194_MEMORY_CLIENT_DLA0FALRDB: c_uint = 0xbf;
// DLA0 write clients
pub const TEGRA194_MEMORY_CLIENT_DLA0WRA: c_uint = 0xc0;
// DLA0 write clients
pub const TEGRA194_MEMORY_CLIENT_DLA0FALWRB: c_uint = 0xc1;
// DLA1ARDA read clients
pub const TEGRA194_MEMORY_CLIENT_DLA1RDA: c_uint = 0xc2;
// DLA1 Falcon read clients
pub const TEGRA194_MEMORY_CLIENT_DLA1FALRDB: c_uint = 0xc3;
// DLA1 write clients
pub const TEGRA194_MEMORY_CLIENT_DLA1WRA: c_uint = 0xc4;
// DLA1 write clients
pub const TEGRA194_MEMORY_CLIENT_DLA1FALWRB: c_uint = 0xc5;
// PVA0RDA read clients
pub const TEGRA194_MEMORY_CLIENT_PVA0RDA: c_uint = 0xc6;
// PVA0RDB read clients
pub const TEGRA194_MEMORY_CLIENT_PVA0RDB: c_uint = 0xc7;
// PVA0RDC read clients
pub const TEGRA194_MEMORY_CLIENT_PVA0RDC: c_uint = 0xc8;
// PVA0WRA write clients
pub const TEGRA194_MEMORY_CLIENT_PVA0WRA: c_uint = 0xc9;
// PVA0WRB write clients
pub const TEGRA194_MEMORY_CLIENT_PVA0WRB: c_uint = 0xca;
// PVA0WRC write clients
pub const TEGRA194_MEMORY_CLIENT_PVA0WRC: c_uint = 0xcb;
// PVA1RDA read clients
pub const TEGRA194_MEMORY_CLIENT_PVA1RDA: c_uint = 0xcc;
// PVA1RDB read clients
pub const TEGRA194_MEMORY_CLIENT_PVA1RDB: c_uint = 0xcd;
// PVA1RDC read clients
pub const TEGRA194_MEMORY_CLIENT_PVA1RDC: c_uint = 0xce;
// PVA1WRA write clients
pub const TEGRA194_MEMORY_CLIENT_PVA1WRA: c_uint = 0xcf;
// PVA1WRB write clients
pub const TEGRA194_MEMORY_CLIENT_PVA1WRB: c_uint = 0xd0;
// PVA1WRC write clients
pub const TEGRA194_MEMORY_CLIENT_PVA1WRC: c_uint = 0xd1;
// RCE read client
pub const TEGRA194_MEMORY_CLIENT_RCER: c_uint = 0xd2;
// RCE write client
pub const TEGRA194_MEMORY_CLIENT_RCEW: c_uint = 0xd3;
// RCEDMA read client
pub const TEGRA194_MEMORY_CLIENT_RCEDMAR: c_uint = 0xd4;
// RCEDMA write client
pub const TEGRA194_MEMORY_CLIENT_RCEDMAW: c_uint = 0xd5;
pub const TEGRA194_MEMORY_CLIENT_NVENC1SRD: c_uint = 0xd6;
pub const TEGRA194_MEMORY_CLIENT_NVENC1SWR: c_uint = 0xd7;
// PCIE0 read clients
pub const TEGRA194_MEMORY_CLIENT_PCIE0R: c_uint = 0xd8;
// PCIE0 write clients
pub const TEGRA194_MEMORY_CLIENT_PCIE0W: c_uint = 0xd9;
// PCIE1 read clients
pub const TEGRA194_MEMORY_CLIENT_PCIE1R: c_uint = 0xda;
// PCIE1 write clients
pub const TEGRA194_MEMORY_CLIENT_PCIE1W: c_uint = 0xdb;
// PCIE2 read clients
pub const TEGRA194_MEMORY_CLIENT_PCIE2AR: c_uint = 0xdc;
// PCIE2 write clients
pub const TEGRA194_MEMORY_CLIENT_PCIE2AW: c_uint = 0xdd;
// PCIE3 read clients
pub const TEGRA194_MEMORY_CLIENT_PCIE3R: c_uint = 0xde;
// PCIE3 write clients
pub const TEGRA194_MEMORY_CLIENT_PCIE3W: c_uint = 0xdf;
// PCIE4 read clients
pub const TEGRA194_MEMORY_CLIENT_PCIE4R: c_uint = 0xe0;
// PCIE4 write clients
pub const TEGRA194_MEMORY_CLIENT_PCIE4W: c_uint = 0xe1;
// PCIE5 read clients
pub const TEGRA194_MEMORY_CLIENT_PCIE5R: c_uint = 0xe2;
// PCIE5 write clients
pub const TEGRA194_MEMORY_CLIENT_PCIE5W: c_uint = 0xe3;
// ISP read client 1 for Crossbar A
pub const TEGRA194_MEMORY_CLIENT_ISPFALW: c_uint = 0xe4;
pub const TEGRA194_MEMORY_CLIENT_NVL3R: c_uint = 0xe5;
pub const TEGRA194_MEMORY_CLIENT_NVL3W: c_uint = 0xe6;
pub const TEGRA194_MEMORY_CLIENT_NVL4R: c_uint = 0xe7;
pub const TEGRA194_MEMORY_CLIENT_NVL4W: c_uint = 0xe8;
// DLA0ARDA1 read clients
pub const TEGRA194_MEMORY_CLIENT_DLA0RDA1: c_uint = 0xe9;
// DLA1ARDA1 read clients
pub const TEGRA194_MEMORY_CLIENT_DLA1RDA1: c_uint = 0xea;
// PVA0RDA1 read clients
pub const TEGRA194_MEMORY_CLIENT_PVA0RDA1: c_uint = 0xeb;
// PVA0RDB1 read clients
pub const TEGRA194_MEMORY_CLIENT_PVA0RDB1: c_uint = 0xec;
// PVA1RDA1 read clients
pub const TEGRA194_MEMORY_CLIENT_PVA1RDA1: c_uint = 0xed;
// PVA1RDB1 read clients
pub const TEGRA194_MEMORY_CLIENT_PVA1RDB1: c_uint = 0xee;
// PCIE5r1 read clients
pub const TEGRA194_MEMORY_CLIENT_PCIE5R1: c_uint = 0xef;
pub const TEGRA194_MEMORY_CLIENT_NVENCSRD1: c_uint = 0xf0;
pub const TEGRA194_MEMORY_CLIENT_NVENC1SRD1: c_uint = 0xf1;
// ISP read client for Crossbar A
pub const TEGRA194_MEMORY_CLIENT_ISPRA1: c_uint = 0xf2;
// PCIE0 read clients
pub const TEGRA194_MEMORY_CLIENT_PCIE0R1: c_uint = 0xf3;
pub const TEGRA194_MEMORY_CLIENT_NVL0RHP: c_uint = 0xf4;
pub const TEGRA194_MEMORY_CLIENT_NVL1RHP: c_uint = 0xf5;
pub const TEGRA194_MEMORY_CLIENT_NVL2RHP: c_uint = 0xf6;
pub const TEGRA194_MEMORY_CLIENT_NVL3RHP: c_uint = 0xf7;
pub const TEGRA194_MEMORY_CLIENT_NVL4RHP: c_uint = 0xf8;
pub const TEGRA194_MEMORY_CLIENT_NVDEC1SRD: c_uint = 0xf9;
pub const TEGRA194_MEMORY_CLIENT_NVDEC1SRD1: c_uint = 0xfa;
pub const TEGRA194_MEMORY_CLIENT_NVDEC1SWR: c_uint = 0xfb;
// MSS internal memqual MIU5 read clients
pub const TEGRA194_MEMORY_CLIENT_MIU5R: c_uint = 0xfc;
// MSS internal memqual MIU5 write clients
pub const TEGRA194_MEMORY_CLIENT_MIU5W: c_uint = 0xfd;
// MSS internal memqual MIU6 read clients
pub const TEGRA194_MEMORY_CLIENT_MIU6R: c_uint = 0xfe;
// MSS internal memqual MIU6 write clients
pub const TEGRA194_MEMORY_CLIENT_MIU6W: c_uint = 0xff;
