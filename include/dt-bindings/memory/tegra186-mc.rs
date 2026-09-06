//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/memory/tegra186-mc.h
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
pub const TEGRA186_SID_INVALID: c_uint = 0x00;
pub const TEGRA186_SID_PASSTHROUGH: c_uint = 0x7f;
// host1x clients
pub const TEGRA186_SID_HOST1X: c_uint = 0x01;
pub const TEGRA186_SID_CSI: c_uint = 0x02;
pub const TEGRA186_SID_VIC: c_uint = 0x03;
pub const TEGRA186_SID_VI: c_uint = 0x04;
pub const TEGRA186_SID_ISP: c_uint = 0x05;
pub const TEGRA186_SID_NVDEC: c_uint = 0x06;
pub const TEGRA186_SID_NVENC: c_uint = 0x07;
pub const TEGRA186_SID_NVJPG: c_uint = 0x08;
pub const TEGRA186_SID_NVDISPLAY: c_uint = 0x09;
pub const TEGRA186_SID_TSEC: c_uint = 0x0a;
pub const TEGRA186_SID_TSECB: c_uint = 0x0b;
pub const TEGRA186_SID_SE: c_uint = 0x0c;
pub const TEGRA186_SID_SE1: c_uint = 0x0d;
pub const TEGRA186_SID_SE2: c_uint = 0x0e;
pub const TEGRA186_SID_SE3: c_uint = 0x0f;
// GPU clients
pub const TEGRA186_SID_GPU: c_uint = 0x10;
// other SoC clients
pub const TEGRA186_SID_AFI: c_uint = 0x11;
pub const TEGRA186_SID_HDA: c_uint = 0x12;
pub const TEGRA186_SID_ETR: c_uint = 0x13;
pub const TEGRA186_SID_EQOS: c_uint = 0x14;
pub const TEGRA186_SID_UFSHC: c_uint = 0x15;
pub const TEGRA186_SID_AON: c_uint = 0x16;
pub const TEGRA186_SID_SDMMC4: c_uint = 0x17;
pub const TEGRA186_SID_SDMMC3: c_uint = 0x18;
pub const TEGRA186_SID_SDMMC2: c_uint = 0x19;
pub const TEGRA186_SID_SDMMC1: c_uint = 0x1a;
pub const TEGRA186_SID_XUSB_HOST: c_uint = 0x1b;
pub const TEGRA186_SID_XUSB_DEV: c_uint = 0x1c;
pub const TEGRA186_SID_SATA: c_uint = 0x1d;
pub const TEGRA186_SID_APE: c_uint = 0x1e;
pub const TEGRA186_SID_SCE: c_uint = 0x1f;
// GPC DMA clients
pub const TEGRA186_SID_GPCDMA_0: c_uint = 0x20;
pub const TEGRA186_SID_GPCDMA_1: c_uint = 0x21;
pub const TEGRA186_SID_GPCDMA_2: c_uint = 0x22;
pub const TEGRA186_SID_GPCDMA_3: c_uint = 0x23;
pub const TEGRA186_SID_GPCDMA_4: c_uint = 0x24;
pub const TEGRA186_SID_GPCDMA_5: c_uint = 0x25;
pub const TEGRA186_SID_GPCDMA_6: c_uint = 0x26;
pub const TEGRA186_SID_GPCDMA_7: c_uint = 0x27;
// APE DMA clients
pub const TEGRA186_SID_APE_1: c_uint = 0x28;
pub const TEGRA186_SID_APE_2: c_uint = 0x29;
// camera RTCPU
pub const TEGRA186_SID_RCE: c_uint = 0x2a;
// camera RTCPU on host1x address space
pub const TEGRA186_SID_RCE_1X: c_uint = 0x2b;
// APE DMA clients
pub const TEGRA186_SID_APE_3: c_uint = 0x2c;
// camera RTCPU running on APE
pub const TEGRA186_SID_APE_CAM: c_uint = 0x2d;
pub const TEGRA186_SID_APE_CAM_1X: c_uint = 0x2e;
//
// The BPMP has its SID value hardcoded in the firmware. Changing it requires
// considerable effort.
//
pub const TEGRA186_SID_BPMP: c_uint = 0x32;
// for SMMU tests
pub const TEGRA186_SID_SMMU_TEST: c_uint = 0x33;
// host1x virtualization channels
pub const TEGRA186_SID_HOST1X_CTX0: c_uint = 0x38;
pub const TEGRA186_SID_HOST1X_CTX1: c_uint = 0x39;
pub const TEGRA186_SID_HOST1X_CTX2: c_uint = 0x3a;
pub const TEGRA186_SID_HOST1X_CTX3: c_uint = 0x3b;
pub const TEGRA186_SID_HOST1X_CTX4: c_uint = 0x3c;
pub const TEGRA186_SID_HOST1X_CTX5: c_uint = 0x3d;
pub const TEGRA186_SID_HOST1X_CTX6: c_uint = 0x3e;
pub const TEGRA186_SID_HOST1X_CTX7: c_uint = 0x3f;
// host1x command buffers
pub const TEGRA186_SID_HOST1X_VM0: c_uint = 0x40;
pub const TEGRA186_SID_HOST1X_VM1: c_uint = 0x41;
pub const TEGRA186_SID_HOST1X_VM2: c_uint = 0x42;
pub const TEGRA186_SID_HOST1X_VM3: c_uint = 0x43;
pub const TEGRA186_SID_HOST1X_VM4: c_uint = 0x44;
pub const TEGRA186_SID_HOST1X_VM5: c_uint = 0x45;
pub const TEGRA186_SID_HOST1X_VM6: c_uint = 0x46;
pub const TEGRA186_SID_HOST1X_VM7: c_uint = 0x47;
// SE data buffers
pub const TEGRA186_SID_SE_VM0: c_uint = 0x48;
pub const TEGRA186_SID_SE_VM1: c_uint = 0x49;
pub const TEGRA186_SID_SE_VM2: c_uint = 0x4a;
pub const TEGRA186_SID_SE_VM3: c_uint = 0x4b;
pub const TEGRA186_SID_SE_VM4: c_uint = 0x4c;
pub const TEGRA186_SID_SE_VM5: c_uint = 0x4d;
pub const TEGRA186_SID_SE_VM6: c_uint = 0x4e;
pub const TEGRA186_SID_SE_VM7: c_uint = 0x4f;
//
// memory client IDs
//
// Misses from System Memory Management Unit (SMMU) Page Table Cache (PTC)
pub const TEGRA186_MEMORY_CLIENT_PTCR: c_uint = 0x00;
// PCIE reads
pub const TEGRA186_MEMORY_CLIENT_AFIR: c_uint = 0x0e;
// High-definition audio (HDA) reads
pub const TEGRA186_MEMORY_CLIENT_HDAR: c_uint = 0x15;
// Host channel data reads
pub const TEGRA186_MEMORY_CLIENT_HOST1XDMAR: c_uint = 0x16;
pub const TEGRA186_MEMORY_CLIENT_NVENCSRD: c_uint = 0x1c;
// SATA reads
pub const TEGRA186_MEMORY_CLIENT_SATAR: c_uint = 0x1f;
// Reads from Cortex-A9 4 CPU cores via the L2 cache
pub const TEGRA186_MEMORY_CLIENT_MPCORER: c_uint = 0x27;
pub const TEGRA186_MEMORY_CLIENT_NVENCSWR: c_uint = 0x2b;
// PCIE writes
pub const TEGRA186_MEMORY_CLIENT_AFIW: c_uint = 0x31;
// High-definition audio (HDA) writes
pub const TEGRA186_MEMORY_CLIENT_HDAW: c_uint = 0x35;
// Writes from Cortex-A9 4 CPU cores via the L2 cache
pub const TEGRA186_MEMORY_CLIENT_MPCOREW: c_uint = 0x39;
// SATA writes
pub const TEGRA186_MEMORY_CLIENT_SATAW: c_uint = 0x3d;
// ISP Read client for Crossbar A
pub const TEGRA186_MEMORY_CLIENT_ISPRA: c_uint = 0x44;
// ISP Write client for Crossbar A
pub const TEGRA186_MEMORY_CLIENT_ISPWA: c_uint = 0x46;
// ISP Write client Crossbar B
pub const TEGRA186_MEMORY_CLIENT_ISPWB: c_uint = 0x47;
// XUSB reads
pub const TEGRA186_MEMORY_CLIENT_XUSB_HOSTR: c_uint = 0x4a;
// XUSB_HOST writes
pub const TEGRA186_MEMORY_CLIENT_XUSB_HOSTW: c_uint = 0x4b;
// XUSB reads
pub const TEGRA186_MEMORY_CLIENT_XUSB_DEVR: c_uint = 0x4c;
// XUSB_DEV writes
pub const TEGRA186_MEMORY_CLIENT_XUSB_DEVW: c_uint = 0x4d;
// TSEC Memory Return Data Client Description
pub const TEGRA186_MEMORY_CLIENT_TSECSRD: c_uint = 0x54;
// TSEC Memory Write Client Description
pub const TEGRA186_MEMORY_CLIENT_TSECSWR: c_uint = 0x55;
// 3D, ltcx reads instance 0
pub const TEGRA186_MEMORY_CLIENT_GPUSRD: c_uint = 0x58;
// 3D, ltcx writes instance 0
pub const TEGRA186_MEMORY_CLIENT_GPUSWR: c_uint = 0x59;
// sdmmca memory read client
pub const TEGRA186_MEMORY_CLIENT_SDMMCRA: c_uint = 0x60;
// sdmmcbmemory read client
pub const TEGRA186_MEMORY_CLIENT_SDMMCRAA: c_uint = 0x61;
// sdmmc memory read client
pub const TEGRA186_MEMORY_CLIENT_SDMMCR: c_uint = 0x62;
// sdmmcd memory read client
pub const TEGRA186_MEMORY_CLIENT_SDMMCRAB: c_uint = 0x63;
// sdmmca memory write client
pub const TEGRA186_MEMORY_CLIENT_SDMMCWA: c_uint = 0x64;
// sdmmcb memory write client
pub const TEGRA186_MEMORY_CLIENT_SDMMCWAA: c_uint = 0x65;
// sdmmc memory write client
pub const TEGRA186_MEMORY_CLIENT_SDMMCW: c_uint = 0x66;
// sdmmcd memory write client
pub const TEGRA186_MEMORY_CLIENT_SDMMCWAB: c_uint = 0x67;
pub const TEGRA186_MEMORY_CLIENT_VICSRD: c_uint = 0x6c;
pub const TEGRA186_MEMORY_CLIENT_VICSWR: c_uint = 0x6d;
// VI Write client
pub const TEGRA186_MEMORY_CLIENT_VIW: c_uint = 0x72;
pub const TEGRA186_MEMORY_CLIENT_NVDECSRD: c_uint = 0x78;
pub const TEGRA186_MEMORY_CLIENT_NVDECSWR: c_uint = 0x79;
// Audio Processing (APE) engine reads
pub const TEGRA186_MEMORY_CLIENT_APER: c_uint = 0x7a;
// Audio Processing (APE) engine writes
pub const TEGRA186_MEMORY_CLIENT_APEW: c_uint = 0x7b;
pub const TEGRA186_MEMORY_CLIENT_NVJPGSRD: c_uint = 0x7e;
pub const TEGRA186_MEMORY_CLIENT_NVJPGSWR: c_uint = 0x7f;
// SE Memory Return Data Client Description
pub const TEGRA186_MEMORY_CLIENT_SESRD: c_uint = 0x80;
// SE Memory Write Client Description
pub const TEGRA186_MEMORY_CLIENT_SESWR: c_uint = 0x81;
// ETR reads
pub const TEGRA186_MEMORY_CLIENT_ETRR: c_uint = 0x84;
// ETR writes
pub const TEGRA186_MEMORY_CLIENT_ETRW: c_uint = 0x85;
// TSECB Memory Return Data Client Description
pub const TEGRA186_MEMORY_CLIENT_TSECSRDB: c_uint = 0x86;
// TSECB Memory Write Client Description
pub const TEGRA186_MEMORY_CLIENT_TSECSWRB: c_uint = 0x87;
// 3D, ltcx reads instance 1
pub const TEGRA186_MEMORY_CLIENT_GPUSRD2: c_uint = 0x88;
// 3D, ltcx writes instance 1
pub const TEGRA186_MEMORY_CLIENT_GPUSWR2: c_uint = 0x89;
// AXI Switch read client
pub const TEGRA186_MEMORY_CLIENT_AXISR: c_uint = 0x8c;
// AXI Switch write client
pub const TEGRA186_MEMORY_CLIENT_AXISW: c_uint = 0x8d;
// EQOS read client
pub const TEGRA186_MEMORY_CLIENT_EQOSR: c_uint = 0x8e;
// EQOS write client
pub const TEGRA186_MEMORY_CLIENT_EQOSW: c_uint = 0x8f;
// UFSHC read client
pub const TEGRA186_MEMORY_CLIENT_UFSHCR: c_uint = 0x90;
// UFSHC write client
pub const TEGRA186_MEMORY_CLIENT_UFSHCW: c_uint = 0x91;
// NVDISPLAY read client
pub const TEGRA186_MEMORY_CLIENT_NVDISPLAYR: c_uint = 0x92;
// BPMP read client
pub const TEGRA186_MEMORY_CLIENT_BPMPR: c_uint = 0x93;
// BPMP write client
pub const TEGRA186_MEMORY_CLIENT_BPMPW: c_uint = 0x94;
// BPMPDMA read client
pub const TEGRA186_MEMORY_CLIENT_BPMPDMAR: c_uint = 0x95;
// BPMPDMA write client
pub const TEGRA186_MEMORY_CLIENT_BPMPDMAW: c_uint = 0x96;
// AON read client
pub const TEGRA186_MEMORY_CLIENT_AONR: c_uint = 0x97;
// AON write client
pub const TEGRA186_MEMORY_CLIENT_AONW: c_uint = 0x98;
// AONDMA read client
pub const TEGRA186_MEMORY_CLIENT_AONDMAR: c_uint = 0x99;
// AONDMA write client
pub const TEGRA186_MEMORY_CLIENT_AONDMAW: c_uint = 0x9a;
// SCE read client
pub const TEGRA186_MEMORY_CLIENT_SCER: c_uint = 0x9b;
// SCE write client
pub const TEGRA186_MEMORY_CLIENT_SCEW: c_uint = 0x9c;
// SCEDMA read client
pub const TEGRA186_MEMORY_CLIENT_SCEDMAR: c_uint = 0x9d;
// SCEDMA write client
pub const TEGRA186_MEMORY_CLIENT_SCEDMAW: c_uint = 0x9e;
// APEDMA read client
pub const TEGRA186_MEMORY_CLIENT_APEDMAR: c_uint = 0x9f;
// APEDMA write client
pub const TEGRA186_MEMORY_CLIENT_APEDMAW: c_uint = 0xa0;
// NVDISPLAY read client instance 2
pub const TEGRA186_MEMORY_CLIENT_NVDISPLAYR1: c_uint = 0xa1;
pub const TEGRA186_MEMORY_CLIENT_VICSRD1: c_uint = 0xa2;
pub const TEGRA186_MEMORY_CLIENT_NVDECSRD1: c_uint = 0xa3;
