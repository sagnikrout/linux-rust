//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/imx-ssi.h
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
pub const SSI_STX0: c_uint = 0x00;
pub const SSI_STX1: c_uint = 0x04;
pub const SSI_SRX0: c_uint = 0x08;
pub const SSI_SRX1: c_uint = 0x0c;
pub const SSI_SCR: c_uint = 0x10;

pub const SSI_SCR_CLK_IST_SHIFT: c_int = 9;

pub const SSI_SISR: c_uint = 0x14;

pub const SSI_SIER: c_uint = 0x18;

pub const SSI_STCR: c_uint = 0x1c;

pub const SSI_FIFO_ENABLE_0_SHIFT: c_int = 7;

pub const SSI_SRCR: c_uint = 0x20;

pub const SSI_FIFO_ENABLE_0_SHIFT: c_int = 7;

pub const SSI_SRCCR: c_uint = 0x28;

pub const SSI_STCCR: c_uint = 0x24;

pub const SSI_SFCSR: c_uint = 0x2c;

pub const SSI_RX_FIFO_1_COUNT_SHIFT: c_int = 28;

pub const SSI_TX_FIFO_1_COUNT_SHIFT: c_int = 24;

pub const SSI_RX_FIFO_0_COUNT_SHIFT: c_int = 12;

pub const SSI_TX_FIFO_0_COUNT_SHIFT: c_int = 8;

pub const SSI_STR: c_uint = 0x30;

pub const SSI_SOR: c_uint = 0x34;

pub const SSI_SACNT: c_uint = 0x38;

pub const SSI_SACADD: c_uint = 0x3c;
pub const SSI_SACDAT: c_uint = 0x40;
pub const SSI_SATAG: c_uint = 0x44;
pub const SSI_STMSK: c_uint = 0x48;
pub const SSI_SRMSK: c_uint = 0x4c;
pub const SSI_SACCST: c_uint = 0x50;
pub const SSI_SACCEN: c_uint = 0x54;
pub const SSI_SACCDIS: c_uint = 0x58;
// SSI clock sources
pub const IMX_SSP_SYS_CLK: c_int = 0;
// SSI audio dividers
pub const IMX_SSI_TX_DIV_2: c_int = 0;
pub const IMX_SSI_TX_DIV_PSR: c_int = 1;
pub const IMX_SSI_TX_DIV_PM: c_int = 2;
pub const IMX_SSI_RX_DIV_2: c_int = 3;
pub const IMX_SSI_RX_DIV_PSR: c_int = 4;
pub const IMX_SSI_RX_DIV_PM: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_ssi {
    pub ac97_dev: *mut platform_device,
    pub imx_ac97: *mut snd_soc_dai,
    pub clk: *mut clk,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub fiq_enable: c_int,
    pub offset: c_uint,
    pub flags: c_uint,
    pub ac97): *mut *mut void (ac97_reset) (struct snd_ac97,
    pub ac97): *mut *mut void (ac97_warm_reset)(struct snd_ac97,
    pub dma_params_rx: snd_dmaengine_dai_dma_data,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
    pub filter_data_tx: imx_dma_data,
    pub filter_data_rx: imx_dma_data,
    pub fiq_params: imx_pcm_fiq_params,
    pub fiq_init: c_int,
    pub dma_init: c_int,
}
