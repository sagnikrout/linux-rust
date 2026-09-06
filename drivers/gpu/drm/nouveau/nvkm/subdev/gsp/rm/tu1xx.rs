//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/tu1xx.c
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
//
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.
//

    const struct nvkm_rm_gpu
    tu1xx_gpu = {
    .disp.class = {
    .root = TU102_DISP,
    .caps = GV100_DISP_CAPS,
    .core = TU102_DISP_CORE_CHANNEL_DMA,
    .wndw = TU102_DISP_WINDOW_CHANNEL_DMA,
    .wimm = TU102_DISP_WINDOW_IMM_CHANNEL_DMA,
    .curs = TU102_DISP_CURSOR,
    },
    .usermode.class = TURING_USERMODE_A,
    .fifo.chan = {
    .class = TURING_CHANNEL_GPFIFO_A,
    .doorbell_handle = tu102_chan_doorbell_handle,
    },
    .ce.class = TURING_DMA_COPY_A,
    .gr.class = {
    .i2m = KEPLER_INLINE_TO_MEMORY_B,
    .twod = FERMI_TWOD_A,
    .threed = TURING_A,
    .compute = TURING_COMPUTE_A,
    },
    .nvdec.class = NVC4B0_VIDEO_DECODER,
    .nvenc.class = NVC4B7_VIDEO_ENCODER,
    };
