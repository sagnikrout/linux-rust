//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/fifo/regsnv04.h
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
pub const NV04_PFIFO_DELAY_0: c_uint = 0x00002040;
pub const NV04_PFIFO_DMA_TIMESLICE: c_uint = 0x00002044;
pub const NV04_PFIFO_NEXT_CHANNEL: c_uint = 0x00002050;
pub const NV03_PFIFO_INTR_0: c_uint = 0x00002100;
pub const NV03_PFIFO_INTR_EN_0: c_uint = 0x00002140;

pub const NV03_PFIFO_RAMHT: c_uint = 0x00002210;
pub const NV03_PFIFO_RAMFC: c_uint = 0x00002214;
pub const NV03_PFIFO_RAMRO: c_uint = 0x00002218;
pub const NV40_PFIFO_RAMFC: c_uint = 0x00002220;
pub const NV03_PFIFO_CACHES: c_uint = 0x00002500;
pub const NV04_PFIFO_MODE: c_uint = 0x00002504;
pub const NV04_PFIFO_DMA: c_uint = 0x00002508;
pub const NV04_PFIFO_SIZE: c_uint = 0x0000250c;

pub const NV50_PFIFO_CTX_TABLE__SIZE: c_int = 128;

pub const NV50_PFIFO_CTX_TABLE_INSTANCE_MASK_G80: c_uint = 0x0FFFFFFF;
pub const NV50_PFIFO_CTX_TABLE_INSTANCE_MASK_G84: c_uint = 0x00FFFFFF;
pub const NV03_PFIFO_CACHE0_PUSH0: c_uint = 0x00003000;
pub const NV03_PFIFO_CACHE0_PULL0: c_uint = 0x00003040;
pub const NV04_PFIFO_CACHE0_PULL0: c_uint = 0x00003050;
pub const NV04_PFIFO_CACHE0_PULL1: c_uint = 0x00003054;
pub const NV03_PFIFO_CACHE1_PUSH0: c_uint = 0x00003200;
pub const NV03_PFIFO_CACHE1_PUSH1: c_uint = 0x00003204;

pub const NV03_PFIFO_CACHE1_PUSH1_CHID_MASK: c_uint = 0x0000000f;
pub const NV10_PFIFO_CACHE1_PUSH1_CHID_MASK: c_uint = 0x0000001f;
pub const NV50_PFIFO_CACHE1_PUSH1_CHID_MASK: c_uint = 0x0000007f;
pub const NV03_PFIFO_CACHE1_PUT: c_uint = 0x00003210;
pub const NV04_PFIFO_CACHE1_DMA_PUSH: c_uint = 0x00003220;
pub const NV04_PFIFO_CACHE1_DMA_FETCH: c_uint = 0x00003224;

pub const NV04_PFIFO_CACHE1_DMA_STATE: c_uint = 0x00003228;
pub const NV04_PFIFO_CACHE1_DMA_INSTANCE: c_uint = 0x0000322c;
pub const NV04_PFIFO_CACHE1_DMA_CTL: c_uint = 0x00003230;
pub const NV04_PFIFO_CACHE1_DMA_PUT: c_uint = 0x00003240;
pub const NV04_PFIFO_CACHE1_DMA_GET: c_uint = 0x00003244;
pub const NV10_PFIFO_CACHE1_REF_CNT: c_uint = 0x00003248;
pub const NV10_PFIFO_CACHE1_DMA_SUBROUTINE: c_uint = 0x0000324C;
pub const NV03_PFIFO_CACHE1_PULL0: c_uint = 0x00003240;
pub const NV04_PFIFO_CACHE1_PULL0: c_uint = 0x00003250;

pub const NV03_PFIFO_CACHE1_PULL1: c_uint = 0x00003250;
pub const NV04_PFIFO_CACHE1_PULL1: c_uint = 0x00003254;
pub const NV04_PFIFO_CACHE1_HASH: c_uint = 0x00003258;
pub const NV10_PFIFO_CACHE1_ACQUIRE_TIMEOUT: c_uint = 0x00003260;
pub const NV10_PFIFO_CACHE1_ACQUIRE_TIMESTAMP: c_uint = 0x00003264;
pub const NV10_PFIFO_CACHE1_ACQUIRE_VALUE: c_uint = 0x00003268;
pub const NV10_PFIFO_CACHE1_SEMAPHORE: c_uint = 0x0000326C;
pub const NV03_PFIFO_CACHE1_GET: c_uint = 0x00003270;
pub const NV04_PFIFO_CACHE1_ENGINE: c_uint = 0x00003280;
pub const NV04_PFIFO_CACHE1_DMA_DCOUNT: c_uint = 0x000032A0;
pub const NV40_PFIFO_GRCTX_INSTANCE: c_uint = 0x000032E0;
pub const NV40_PFIFO_UNK32E4: c_uint = 0x000032E4;

