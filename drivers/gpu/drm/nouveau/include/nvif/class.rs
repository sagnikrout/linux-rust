//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/class.h
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


// SPDX-License-Identifier: MIT
// these class numbers are made up by us, and not nvidia-assigned

// the below match nvidia-assigned (either in hw, or sw) class numbers
pub const NV_NULL_CLASS: c_uint = 0x00000030;

pub const NV50_TWOD: c_uint = 0x0000502d;
pub const FERMI_TWOD_A: c_uint = 0x0000902d;
pub const NV50_MEMORY_TO_MEMORY_FORMAT: c_uint = 0x00005039;
pub const FERMI_MEMORY_TO_MEMORY_FORMAT_A: c_uint = 0x00009039;
pub const KEPLER_INLINE_TO_MEMORY_A: c_uint = 0x0000a040;
pub const KEPLER_INLINE_TO_MEMORY_B: c_uint = 0x0000a140;
pub const BLACKWELL_INLINE_TO_MEMORY_A: c_uint = 0x0000cd40;

pub const VOLTA_USERMODE_A: c_uint = 0x0000c361;
pub const TURING_USERMODE_A: c_uint = 0x0000c461;
pub const AMPERE_USERMODE_A: c_uint = 0x0000c561;
pub const HOPPER_USERMODE_A: c_uint = 0x0000c661;
pub const BLACKWELL_USERMODE_A: c_uint = 0x0000c761;

pub const HOPPER_CHANNEL_GPFIFO_A: c_uint = 0x0000c86f;
pub const BLACKWELL_CHANNEL_GPFIFO_A: c_uint = 0x0000c96f;
pub const BLACKWELL_CHANNEL_GPFIFO_B: c_uint = 0x0000ca6f;

pub const GB202_DISP: c_uint = 0x0000ca70;
pub const GV100_DISP_CAPS: c_uint = 0x0000c373;
pub const GB202_DISP_CAPS: c_uint = 0x0000ca73;
pub const NV31_MPEG: c_uint = 0x00003174;
pub const G82_MPEG: c_uint = 0x00008274;
pub const NV74_VP2: c_uint = 0x00007476;

pub const GB202_DISP_CURSOR: c_uint = 0x0000ca7a;

pub const GB202_DISP_WINDOW_IMM_CHANNEL_DMA: c_uint = 0x0000ca7b;

pub const GB202_DISP_CORE_CHANNEL_DMA: c_uint = 0x0000ca7d;

pub const GB202_DISP_WINDOW_CHANNEL_DMA: c_uint = 0x0000ca7e;
pub const NV50_TESLA: c_uint = 0x00005097;
pub const G82_TESLA: c_uint = 0x00008297;
pub const GT200_TESLA: c_uint = 0x00008397;
pub const GT214_TESLA: c_uint = 0x00008597;
pub const GT21A_TESLA: c_uint = 0x00008697;

pub const AMPERE_A: c_uint = 0x0000c697;

pub const HOPPER_A: c_uint = 0x0000cb97;
pub const BLACKWELL_A: c_uint = 0x0000cd97;
pub const BLACKWELL_B: c_uint = 0x0000ce97;
pub const NV74_BSP: c_uint = 0x000074b0;
pub const NVB8B0_VIDEO_DECODER: c_uint = 0x0000b8b0;
pub const NVC4B0_VIDEO_DECODER: c_uint = 0x0000c4b0;
pub const NVC6B0_VIDEO_DECODER: c_uint = 0x0000c6b0;
pub const NVC7B0_VIDEO_DECODER: c_uint = 0x0000c7b0;
pub const NVC9B0_VIDEO_DECODER: c_uint = 0x0000c9b0;
pub const NVCDB0_VIDEO_DECODER: c_uint = 0x0000cdb0;
pub const NVCFB0_VIDEO_DECODER: c_uint = 0x0000cfb0;
pub const GT212_MSVLD: c_uint = 0x000085b1;
pub const IGT21A_MSVLD: c_uint = 0x000086b1;
pub const G98_MSVLD: c_uint = 0x000088b1;
pub const GF100_MSVLD: c_uint = 0x000090b1;
pub const GK104_MSVLD: c_uint = 0x000095b1;
pub const GT212_MSPDEC: c_uint = 0x000085b2;
pub const G98_MSPDEC: c_uint = 0x000088b2;
pub const GF100_MSPDEC: c_uint = 0x000090b2;
pub const GK104_MSPDEC: c_uint = 0x000095b2;
pub const GT212_MSPPP: c_uint = 0x000085b3;
pub const G98_MSPPP: c_uint = 0x000088b3;
pub const GF100_MSPPP: c_uint = 0x000090b3;
pub const G98_SEC: c_uint = 0x000088b4;
pub const GT212_DMA: c_uint = 0x000085b5;
pub const FERMI_DMA: c_uint = 0x000090b5;
pub const KEPLER_DMA_COPY_A: c_uint = 0x0000a0b5;
pub const MAXWELL_DMA_COPY_A: c_uint = 0x0000b0b5;
pub const PASCAL_DMA_COPY_A: c_uint = 0x0000c0b5;
pub const PASCAL_DMA_COPY_B: c_uint = 0x0000c1b5;
pub const VOLTA_DMA_COPY_A: c_uint = 0x0000c3b5;
pub const TURING_DMA_COPY_A: c_uint = 0x0000c5b5;
pub const AMPERE_DMA_COPY_A: c_uint = 0x0000c6b5;
pub const AMPERE_DMA_COPY_B: c_uint = 0x0000c7b5;
pub const HOPPER_DMA_COPY_A: c_uint = 0x0000c8b5;
pub const BLACKWELL_DMA_COPY_A: c_uint = 0x0000c9b5;
pub const BLACKWELL_DMA_COPY_B: c_uint = 0x0000cab5;
pub const NVC4B7_VIDEO_ENCODER: c_uint = 0x0000c4b7;
pub const NVC7B7_VIDEO_ENCODER: c_uint = 0x0000c7b7;
pub const NVC9B7_VIDEO_ENCODER: c_uint = 0x0000c9b7;
pub const NVCFB7_VIDEO_ENCODER: c_uint = 0x0000cfb7;
pub const FERMI_DECOMPRESS: c_uint = 0x000090b8;
pub const NV50_COMPUTE: c_uint = 0x000050c0;
pub const GT214_COMPUTE: c_uint = 0x000085c0;
pub const FERMI_COMPUTE_A: c_uint = 0x000090c0;
pub const FERMI_COMPUTE_B: c_uint = 0x000091c0;
pub const KEPLER_COMPUTE_A: c_uint = 0x0000a0c0;
pub const KEPLER_COMPUTE_B: c_uint = 0x0000a1c0;
pub const MAXWELL_COMPUTE_A: c_uint = 0x0000b0c0;
pub const MAXWELL_COMPUTE_B: c_uint = 0x0000b1c0;
pub const PASCAL_COMPUTE_A: c_uint = 0x0000c0c0;
pub const PASCAL_COMPUTE_B: c_uint = 0x0000c1c0;
pub const VOLTA_COMPUTE_A: c_uint = 0x0000c3c0;
pub const TURING_COMPUTE_A: c_uint = 0x0000c5c0;
pub const AMPERE_COMPUTE_A: c_uint = 0x0000c6c0;
pub const AMPERE_COMPUTE_B: c_uint = 0x0000c7c0;
pub const ADA_COMPUTE_A: c_uint = 0x0000c9c0;
pub const HOPPER_COMPUTE_A: c_uint = 0x0000cbc0;
pub const BLACKWELL_COMPUTE_A: c_uint = 0x0000cdc0;
pub const BLACKWELL_COMPUTE_B: c_uint = 0x0000cec0;
pub const NV74_CIPHER: c_uint = 0x000074c1;
pub const NVB8D1_VIDEO_NVJPG: c_uint = 0x0000b8d1;
pub const NVC4D1_VIDEO_NVJPG: c_uint = 0x0000c4d1;
pub const NVC9D1_VIDEO_NVJPG: c_uint = 0x0000c9d1;
pub const NVCDD1_VIDEO_NVJPG: c_uint = 0x0000cdd1;
pub const NVCFD1_VIDEO_NVJPG: c_uint = 0x0000cfd1;
pub const NVB8FA_VIDEO_OFA: c_uint = 0x0000b8fa;
pub const NVC6FA_VIDEO_OFA: c_uint = 0x0000c6fa;
pub const NVC7FA_VIDEO_OFA: c_uint = 0x0000c7fa;
pub const NVC9FA_VIDEO_OFA: c_uint = 0x0000c9fa;
pub const NVCDFA_VIDEO_OFA: c_uint = 0x0000cdfa;
pub const NVCFFA_VIDEO_OFA: c_uint = 0x0000cffa;
