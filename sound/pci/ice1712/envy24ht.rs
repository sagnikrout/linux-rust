//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/envy24ht.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ALSA driver for ICEnsemble VT1724 (Envy24)
//
// Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
//

//
// Direct registers
//

pub const VT1724_REG_CONTROL: c_uint = 0x00	/* byte */;
pub const VT1724_RESET: c_uint = 0x80	/* reset whole chip */;
pub const VT1724_REG_IRQMASK: c_uint = 0x01	/* byte */;
pub const VT1724_IRQ_MPU_RX: c_uint = 0x80;
pub const VT1724_IRQ_MPU_TX: c_uint = 0x20;
pub const VT1724_IRQ_MTPCM: c_uint = 0x10;
pub const VT1724_REG_IRQSTAT: c_uint = 0x02	/* byte */;
// look to VT1724_IRQ_*
pub const VT1724_REG_SYS_CFG: c_uint = 0x04	/* byte - system configuration PCI60 on Envy24*/;
pub const VT1724_CFG_CLOCK: c_uint = 0xc0;
pub const VT1724_CFG_CLOCK512: c_uint = 0x00	/* 22.5692Mhz, 44.1kHz*512 */;
pub const VT1724_CFG_CLOCK384: c_uint = 0x40	/* 16.9344Mhz, 44.1kHz*384 */;
pub const VT1724_CFG_MPU401: c_uint = 0x20		/* MPU401 UARTs */;
pub const VT1724_CFG_ADC_MASK: c_uint = 0x0c	/* one, two or one and S/PDIF, stereo ADCs */;
pub const VT1724_CFG_ADC_NONE: c_uint = 0x0c	/* no ADCs */;
pub const VT1724_CFG_DAC_MASK: c_uint = 0x03	/* one, two, three, four stereo DACs */;
pub const VT1724_REG_AC97_CFG: c_uint = 0x05	/* byte */;
pub const VT1724_CFG_PRO_I2S: c_uint = 0x80	/* multitrack converter: I2S or AC'97 */;
pub const VT1724_CFG_AC97_PACKED: c_uint = 0x01	/* split or packed mode - AC'97 */;
pub const VT1724_REG_I2S_FEATURES: c_uint = 0x06	/* byte */;
pub const VT1724_CFG_I2S_VOLUME: c_uint = 0x80	/* volume/mute capability */;
pub const VT1724_CFG_I2S_96KHZ: c_uint = 0x40	/* supports 96kHz sampling */;
pub const VT1724_CFG_I2S_RESMASK: c_uint = 0x30	/* resolution mask, 16,18,20,24-bit */;
pub const VT1724_CFG_I2S_192KHZ: c_uint = 0x08	/* supports 192kHz sampling */;
pub const VT1724_CFG_I2S_OTHER: c_uint = 0x07	/* other I2S IDs */;
pub const VT1724_REG_SPDIF_CFG: c_uint = 0x07	/* byte */;
pub const VT1724_CFG_SPDIF_OUT_EN: c_uint = 0x80	/*Internal S/PDIF output is enabled*/;
pub const VT1724_CFG_SPDIF_OUT_INT: c_uint = 0x40	/*Internal S/PDIF output is implemented*/;
pub const VT1724_CFG_I2S_CHIPID: c_uint = 0x3c	/* I2S chip ID */;
pub const VT1724_CFG_SPDIF_IN: c_uint = 0x02	/* S/PDIF input is present */;
pub const VT1724_CFG_SPDIF_OUT: c_uint = 0x01	/* External S/PDIF output is present */;
// there is no consumer AC97 codec with the VT1724
// #define VT1724_REG_AC97_INDEX		0x08	/* byte
// #define VT1724_REG_AC97_CMD		0x09	/* byte
pub const VT1724_REG_MPU_TXFIFO: c_uint = 0x0a	/*byte ro. number of bytes in TX fifo*/;
pub const VT1724_REG_MPU_RXFIFO: c_uint = 0x0b	/*byte ro. number of bytes in RX fifo*/;
pub const VT1724_REG_MPU_DATA: c_uint = 0x0c	/* byte */;
pub const VT1724_REG_MPU_CTRL: c_uint = 0x0d	/* byte */;
pub const VT1724_MPU_UART: c_uint = 0x01;
pub const VT1724_MPU_TX_EMPTY: c_uint = 0x02;
pub const VT1724_MPU_TX_FULL: c_uint = 0x04;
pub const VT1724_MPU_RX_EMPTY: c_uint = 0x08;
pub const VT1724_MPU_RX_FULL: c_uint = 0x10;
pub const VT1724_REG_MPU_FIFO_WM: c_uint = 0x0e	/*byte set the high/low watermarks for RX/TX fifos*/;
pub const VT1724_MPU_RX_FIFO: c_uint = 0x20	//1=rx fifo watermark 0=tx fifo watermark;
pub const VT1724_MPU_FIFO_MASK: c_uint = 0x1f;
pub const VT1724_REG_I2C_DEV_ADDR: c_uint = 0x10	/* byte */;
pub const VT1724_I2C_WRITE: c_uint = 0x01	/* write direction */;
pub const VT1724_REG_I2C_BYTE_ADDR: c_uint = 0x11	/* byte */;
pub const VT1724_REG_I2C_DATA: c_uint = 0x12	/* byte */;
pub const VT1724_REG_I2C_CTRL: c_uint = 0x13	/* byte */;
pub const VT1724_I2C_EEPROM: c_uint = 0x80	/* 1 = EEPROM exists */;
pub const VT1724_I2C_BUSY: c_uint = 0x01	/* busy bit */;
pub const VT1724_REG_GPIO_DATA: c_uint = 0x14	/* word */;
pub const VT1724_REG_GPIO_WRITE_MASK: c_uint = 0x16 /* word */;
pub const VT1724_REG_GPIO_DIRECTION: c_uint = 0x18 /* dword? (3 bytes) 0=input 1=output.;
pub const VT1724_REG_POWERDOWN: c_uint = 0x1c;
pub const VT1724_REG_GPIO_DATA_22: c_uint = 0x1e /* byte direction for GPIO 16:22 */;
pub const VT1724_REG_GPIO_WRITE_MASK_22: c_uint = 0x1f /* byte write mask for GPIO 16:22 */;
//
// Professional multi-track direct control registers
//

pub const VT1724_MT_IRQ: c_uint = 0x00	/* byte - interrupt mask */;
pub const VT1724_MULTI_PDMA4: c_uint = 0x80	/* SPDIF Out / PDMA4 */;
pub const VT1724_MULTI_PDMA3: c_uint = 0x40	/* PDMA3 */;
pub const VT1724_MULTI_PDMA2: c_uint = 0x20	/* PDMA2 */;
pub const VT1724_MULTI_PDMA1: c_uint = 0x10	/* PDMA1 */;
pub const VT1724_MULTI_FIFO_ERR: c_uint = 0x08	/* DMA FIFO underrun/overrun. */;
pub const VT1724_MULTI_RDMA1: c_uint = 0x04	/* RDMA1 (S/PDIF input) */;
pub const VT1724_MULTI_RDMA0: c_uint = 0x02	/* RMDA0 */;
pub const VT1724_MULTI_PDMA0: c_uint = 0x01	/* MC Interleave/PDMA0 */;
pub const VT1724_MT_RATE: c_uint = 0x01	/* byte - sampling rate select */;
pub const VT1724_SPDIF_MASTER: c_uint = 0x10	/* S/PDIF input is master clock */;
pub const VT1724_MT_I2S_FORMAT: c_uint = 0x02	/* byte - I2S data format */;
pub const VT1724_MT_I2S_MCLK_128X: c_uint = 0x08;
pub const VT1724_MT_I2S_FORMAT_MASK: c_uint = 0x03;
pub const VT1724_MT_I2S_FORMAT_I2S: c_uint = 0x00;
pub const VT1724_MT_DMA_INT_MASK: c_uint = 0x03	/* byte -DMA Interrupt Mask */;
// lool to VT1724_MULTI_*
pub const VT1724_MT_AC97_INDEX: c_uint = 0x04	/* byte - AC'97 index */;
pub const VT1724_MT_AC97_CMD: c_uint = 0x05	/* byte - AC'97 command & status */;
pub const VT1724_AC97_COLD: c_uint = 0x80	/* cold reset */;
pub const VT1724_AC97_WARM: c_uint = 0x40	/* warm reset */;
pub const VT1724_AC97_WRITE: c_uint = 0x20	/* W: write, R: write in progress */;
pub const VT1724_AC97_READ: c_uint = 0x10	/* W: read, R: read in progress */;
pub const VT1724_AC97_READY: c_uint = 0x08	/* codec ready status bit */;
pub const VT1724_AC97_ID_MASK: c_uint = 0x03	/* codec id mask */;
pub const VT1724_MT_AC97_DATA: c_uint = 0x06	/* word - AC'97 data */;
pub const VT1724_MT_PLAYBACK_ADDR: c_uint = 0x10	/* dword - playback address */;
pub const VT1724_MT_PLAYBACK_SIZE: c_uint = 0x14	/* dword - playback size */;
pub const VT1724_MT_DMA_CONTROL: c_uint = 0x18	/* byte - control */;
pub const VT1724_PDMA4_START: c_uint = 0x80	/* SPDIF out / PDMA4 start */;
pub const VT1724_PDMA3_START: c_uint = 0x40	/* PDMA3 start */;
pub const VT1724_PDMA2_START: c_uint = 0x20	/* PDMA2 start */;
pub const VT1724_PDMA1_START: c_uint = 0x10	/* PDMA1 start */;
pub const VT1724_RDMA1_START: c_uint = 0x04	/* RDMA1 start */;
pub const VT1724_RDMA0_START: c_uint = 0x02	/* RMDA0 start */;
pub const VT1724_PDMA0_START: c_uint = 0x01	/* MC Interleave / PDMA0 start */;
pub const VT1724_MT_BURST: c_uint = 0x19	/* Interleaved playback DMA Active streams / PCI burst size */;
pub const VT1724_MT_DMA_FIFO_ERR: c_uint = 0x1a	/*Global playback and record DMA FIFO Underrun/Overrun */;
pub const VT1724_PDMA4_UNDERRUN: c_uint = 0x80;
pub const VT1724_PDMA2_UNDERRUN: c_uint = 0x40;
pub const VT1724_PDMA3_UNDERRUN: c_uint = 0x20;
pub const VT1724_PDMA1_UNDERRUN: c_uint = 0x10;
pub const VT1724_RDMA1_UNDERRUN: c_uint = 0x04;
pub const VT1724_RDMA0_UNDERRUN: c_uint = 0x02;
pub const VT1724_PDMA0_UNDERRUN: c_uint = 0x01;
pub const VT1724_MT_DMA_PAUSE: c_uint = 0x1b	/*Global playback and record DMA FIFO pause/resume */;
pub const VT1724_PDMA4_PAUSE: c_uint = 0x80;
pub const VT1724_PDMA3_PAUSE: c_uint = 0x40;
pub const VT1724_PDMA2_PAUSE: c_uint = 0x20;
pub const VT1724_PDMA1_PAUSE: c_uint = 0x10;
pub const VT1724_RDMA1_PAUSE: c_uint = 0x04;
pub const VT1724_RDMA0_PAUSE: c_uint = 0x02;
pub const VT1724_PDMA0_PAUSE: c_uint = 0x01;
pub const VT1724_MT_PLAYBACK_COUNT: c_uint = 0x1c	/* word - playback count */;
pub const VT1724_MT_CAPTURE_ADDR: c_uint = 0x20	/* dword - capture address */;
pub const VT1724_MT_CAPTURE_SIZE: c_uint = 0x24	/* word - capture size */;
pub const VT1724_MT_CAPTURE_COUNT: c_uint = 0x26	/* word - capture count */;
pub const VT1724_MT_ROUTE_PLAYBACK: c_uint = 0x2c	/* word */;
pub const VT1724_MT_RDMA1_ADDR: c_uint = 0x30	/* dword - RDMA1 capture address */;
pub const VT1724_MT_RDMA1_SIZE: c_uint = 0x34	/* word - RDMA1 capture size */;
pub const VT1724_MT_RDMA1_COUNT: c_uint = 0x36	/* word - RDMA1 capture count */;
pub const VT1724_MT_SPDIF_CTRL: c_uint = 0x3c	/* word */;
pub const VT1724_MT_MONITOR_PEAKINDEX: c_uint = 0x3e	/* byte */;
pub const VT1724_MT_MONITOR_PEAKDATA: c_uint = 0x3f	/* byte */;
// concurrent stereo channels
pub const VT1724_MT_PDMA4_ADDR: c_uint = 0x40	/* dword */;
pub const VT1724_MT_PDMA4_SIZE: c_uint = 0x44	/* word */;
pub const VT1724_MT_PDMA4_COUNT: c_uint = 0x46	/* word */;
pub const VT1724_MT_PDMA3_ADDR: c_uint = 0x50	/* dword */;
pub const VT1724_MT_PDMA3_SIZE: c_uint = 0x54	/* word */;
pub const VT1724_MT_PDMA3_COUNT: c_uint = 0x56	/* word */;
pub const VT1724_MT_PDMA2_ADDR: c_uint = 0x60	/* dword */;
pub const VT1724_MT_PDMA2_SIZE: c_uint = 0x64	/* word */;
pub const VT1724_MT_PDMA2_COUNT: c_uint = 0x66	/* word */;
pub const VT1724_MT_PDMA1_ADDR: c_uint = 0x70	/* dword */;
pub const VT1724_MT_PDMA1_SIZE: c_uint = 0x74	/* word */;
pub const VT1724_MT_PDMA1_COUNT: c_uint = 0x76	/* word */;
extern "C" {
    pub fn snd_vt1724_read_i2c(ice: *mut snd_ice1712, dev: c_uchar, addr: c_uchar) -> c_uchar;
}
extern "C" {
    pub fn snd_vt1724_write_i2c(ice: *mut snd_ice1712, dev: c_uchar, addr: c_uchar, data: c_uchar);
}
