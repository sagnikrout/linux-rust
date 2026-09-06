//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/mpc5200_dma.h
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
// Freescale MPC5200 Audio DMA driver
//
pub const PSC_STREAM_NAME_LEN: c_int = 32;
//
// psc_ac97_stream - Data specific to a single stream (playback or capture)
// @active:		flag indicating if the stream is active
// @psc_dma:		pointer back to parent psc_dma data structure
// @bcom_task:		bestcomm task structure
// @irq:		irq number for bestcomm task
// @period_end:		physical address of end of DMA region
// @period_next_pt:	physical address of next DMA buffer to enqueue
// @period_bytes:	size of DMA period in bytes
// @ac97_slot_bits:	Enable bits for turning on the correct AC97 slot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psc_dma_stream {
    pub runtime: *mut snd_pcm_runtime,
    pub active: c_int,
    pub psc_dma: *mut psc_dma,
    pub bcom_task: *mut bcom_task,
    pub irq: c_int,
    pub stream: *mut snd_pcm_substream,
    pub period_next: c_int,
    pub period_current: c_int,
    pub period_bytes: c_int,
    pub period_count: c_int,
// AC97 state
    pub ac97_slot_bits: u32,
}

//
// psc_dma - Private driver data
// @name: short name for this device ("PSC0", "PSC1", etc)
// @psc_regs: pointer to the PSC's registers
// @fifo_regs: pointer to the PSC's FIFO registers
// @irq: IRQ of this PSC
// @dev: struct device pointer
// @dai: the CPU DAI for this device
// @sicr: Base value used in serial interface control register; mode is ORed
// with this value.
// @playback: Playback stream context data
// @capture: Capture stream context data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psc_dma {
    pub name: [c_char; 32],
    pub psc_regs: *mut mpc52xx_psc __iomem,
    pub fifo_regs: *mut mpc52xx_psc_fifo __iomem,
    pub irq: c_uint,
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub sicr: u32,
    pub sysclk: c_uint,
    pub imr: c_int,
    pub id: c_int,
    pub slots: c_uint,
// per-stream data
    pub playback: psc_dma_stream,
    pub capture: psc_dma_stream,
// Statistics
    pub overrun_count: c_ulong,
    pub underrun_count: c_ulong,
    pub stats: },
}

// Utility for retrieving psc_dma_stream structure from a substream
extern "C" {
    pub fn mpc5200_audio_dma_create(op: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn mpc5200_audio_dma_destroy(op: *mut platform_device) -> c_int;
}
