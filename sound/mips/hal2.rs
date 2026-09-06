//! Automatically rewritten from C Header to Rust Module
//! Source: sound/mips/hal2.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Driver for HAL2 sound processors
// Copyright (c) 1999 Ulf Carlsson <ulfc@bun.falkenberg.se>
// Copyright (c) 2001, 2002, 2003 Ladislav Michl <ladis@linux-mips.org>
//

// Indirect status register
pub const H2_ISR_TSTATUS: c_uint = 0x01	/* RO: transaction status 1=busy */;
pub const H2_ISR_USTATUS: c_uint = 0x02	/* RO: utime status bit 1=armed */;
pub const H2_ISR_QUAD_MODE: c_uint = 0x04	/* codec mode 0=indigo 1=quad */;
pub const H2_ISR_GLOBAL_RESET_N: c_uint = 0x08	/* chip global reset 0=reset */;
pub const H2_ISR_CODEC_RESET_N: c_uint = 0x10	/* codec/synth reset 0=reset  */;
// Revision register
pub const H2_REV_AUDIO_PRESENT: c_uint = 0x8000	/* RO: audio present 0=present */;
pub const H2_REV_BOARD_M: c_uint = 0x7000	/* RO: bits 14:12, board revision */;
pub const H2_REV_MAJOR_CHIP_M: c_uint = 0x00F0	/* RO: bits 7:4, major chip revision */;
pub const H2_REV_MINOR_CHIP_M: c_uint = 0x000F	/* RO: bits 3:0, minor chip revision */;
// Indirect address register
//
// Address of indirect internal register to be accessed. A write to this
// register initiates read or write access to the indirect registers in the
// HAL2. Note that there af four indirect data registers for write access to
// registers larger than 16 byte.
//
pub const H2_IAR_TYPE_M: c_uint = 0xF000	/* bits 15:12, type of functional */;
// block the register resides in
// 1=DMA Port
// 9=Global DMA Control
// 2=Bresenham
// 3=Unix Timer
pub const H2_IAR_NUM_M: c_uint = 0x0F00	/* bits 11:8 instance of the */;
// blockin which the indirect
// register resides
// If IAR_TYPE_M=DMA Port:
// 1=Synth In
// 2=AES In
// 3=AES Out
// 4=DAC Out
// 5=ADC Out
// 6=Synth Control
// If IAR_TYPE_M=Global DMA Control:
// 1=Control
// If IAR_TYPE_M=Bresenham:
// 1=Bresenham Clock Gen 1
// 2=Bresenham Clock Gen 2
// 3=Bresenham Clock Gen 3
// If IAR_TYPE_M=Unix Timer:
// 1=Unix Timer
pub const H2_IAR_ACCESS_SELECT: c_uint = 0x0080	/* 1=read 0=write */;
pub const H2_IAR_PARAM: c_uint = 0x000C	/* Parameter Select */;
pub const H2_IAR_RB_INDEX_M: c_uint = 0x0003	/* Read Back Index */;
// 00:word0
// 01:word1
// 10:word2
// 11:word3
//
// HAL2 internal addressing
//
// The HAL2 has "indirect registers" (idr) which are accessed by writing to the
// Indirect Data registers. Write the address to the Indirect Address register
// to transfer the data.
//
// We define the H2IR_* to the read address and H2IW_* to the write address and
// H2I_* to be fields in whatever register is referred to.
//
// When we write to indirect registers which are larger than one word (16 bit)
// we have to fill more than one indirect register before writing. When we read
// back however we have to read several times, each time with different Read
// Back Indexes (there are defs for doing this easily).
//
// Relay Control
//
pub const H2I_RELAY_C: c_uint = 0x9100;
pub const H2I_RELAY_C_STATE: c_uint = 0x01		/* state of RELAY pin signal */;
// DMA port enable
pub const H2I_DMA_PORT_EN: c_uint = 0x9104;
pub const H2I_DMA_PORT_EN_SY_IN: c_uint = 0x01		/* Synth_in DMA port */;
pub const H2I_DMA_PORT_EN_AESRX: c_uint = 0x02		/* AES receiver DMA port */;
pub const H2I_DMA_PORT_EN_AESTX: c_uint = 0x04		/* AES transmitter DMA port */;
pub const H2I_DMA_PORT_EN_CODECTX: c_uint = 0x08		/* CODEC transmit DMA port */;
pub const H2I_DMA_PORT_EN_CODECR: c_uint = 0x10		/* CODEC receive DMA port */;
pub const H2I_DMA_END: c_uint = 0x9108 		/* global dma endian select */;
pub const H2I_DMA_END_SY_IN: c_uint = 0x01		/* Synth_in DMA port */;
pub const H2I_DMA_END_AESRX: c_uint = 0x02		/* AES receiver DMA port */;
pub const H2I_DMA_END_AESTX: c_uint = 0x04		/* AES transmitter DMA port */;
pub const H2I_DMA_END_CODECTX: c_uint = 0x08		/* CODEC transmit DMA port */;
pub const H2I_DMA_END_CODECR: c_uint = 0x10		/* CODEC receive DMA port */;
// 0=b_end 1=l_end
pub const H2I_DMA_DRV: c_uint = 0x910C  	/* global PBUS DMA enable */;
pub const H2I_SYNTH_C: c_uint = 0x1104		/* Synth DMA control */;
pub const H2I_AESRX_C: c_uint = 0x1204	 	/* AES RX dma control */;
pub const H2I_C_TS_EN: c_uint = 0x20		/* Timestamp enable */;
pub const H2I_C_TS_FRMT: c_uint = 0x40		/* Timestamp format */;
pub const H2I_C_NAUDIO: c_uint = 0x80		/* Sign extend */;
// AESRX CTL, 16 bit
pub const H2I_AESTX_C: c_uint = 0x1304		/* AES TX DMA control */;

pub const H2I_AESTX_C_CLKID_M: c_uint = 0x18;

pub const H2I_AESTX_C_DATAT_M: c_uint = 0x300;
// CODEC registers
pub const H2I_DAC_C1: c_uint = 0x1404 		/* DAC DMA control, 16 bit */;
pub const H2I_DAC_C2: c_uint = 0x1408		/* DAC DMA control, 32 bit */;
pub const H2I_ADC_C1: c_uint = 0x1504 		/* ADC DMA control, 16 bit */;
pub const H2I_ADC_C2: c_uint = 0x1508		/* ADC DMA control, 32 bit */;
// Bits in CTL1 register

pub const H2I_C1_DMA_M: c_uint = 0x7;

pub const H2I_C1_CLKID_M: c_uint = 0x18;

pub const H2I_C1_DATAT_M: c_uint = 0x300;
// Bits in CTL2 register

pub const H2I_C2_R_GAIN_M: c_uint = 0xf;

pub const H2I_C2_L_GAIN_M: c_uint = 0xf0;
pub const H2I_C2_R_SEL: c_uint = 0x100		/* right input select */;
pub const H2I_C2_L_SEL: c_uint = 0x200		/* left input select */;
pub const H2I_C2_MUTE: c_uint = 0x400		/* mute */;
pub const H2I_C2_DO1: c_uint = 0x00010000	/* digital output port bit 0 */;
pub const H2I_C2_DO2: c_uint = 0x00020000	/* digital output port bit 1 */;

pub const H2I_C2_R_ATT_M: c_uint = 0x007c0000	/* attenuation */;

pub const H2I_C2_L_ATT_M: c_uint = 0x0f800000	/* attenuation */;
pub const H2I_SYNTH_MAP_C: c_uint = 0x1104		/* synth dma handshake ctrl */;
// Clock generator CTL 1, 16 bit
pub const H2I_BRES1_C1: c_uint = 0x2104;
pub const H2I_BRES2_C1: c_uint = 0x2204;
pub const H2I_BRES3_C1: c_uint = 0x2304;

pub const H2I_BRES_C1_M: c_uint = 0x03;
// Clock generator CTL 2, 32 bit
pub const H2I_BRES1_C2: c_uint = 0x2108;
pub const H2I_BRES2_C2: c_uint = 0x2208;
pub const H2I_BRES3_C2: c_uint = 0x2308;

pub const H2I_BRES_C2_INC_M: c_uint = 0xffff;

pub const H2I_BRES_C2_MOD_M: c_uint = 0xffff0000	/* modctrl=0xffff&(modinc-1) */;
// Unix timer, 64 bit
pub const H2I_UTIME: c_uint = 0x3104;
pub const H2I_UTIME_0_LD: c_uint = 0xffff		/* microseconds, LSB's */;
pub const H2I_UTIME_1_LD0: c_uint = 0x0f		/* microseconds, MSB's */;
pub const H2I_UTIME_1_LD1: c_uint = 0xf0		/* tenths of microseconds */;
pub const H2I_UTIME_2_LD: c_uint = 0xffff		/* seconds, LSB's */;
pub const H2I_UTIME_3_LD: c_uint = 0xffff		/* seconds, MSB's */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal2_ctl_regs {
    pub _unused0: [u32; 4],
    pub /: *mut *mut u32 isr; / 0x10 Status Register,
    pub _unused1: [u32; 3],
    pub /: *mut *mut u32 rev; / 0x20 Revision Register,
    pub _unused2: [u32; 3],
    pub /: *mut *mut u32 iar; / 0x30 Indirect Address Register,
    pub _unused3: [u32; 3],
    pub /: *mut *mut u32 idr0; / 0x40 Indirect Data Register 0,
    pub _unused4: [u32; 3],
    pub /: *mut *mut u32 idr1; / 0x50 Indirect Data Register 1,
    pub _unused5: [u32; 3],
    pub /: *mut *mut u32 idr2; / 0x60 Indirect Data Register 2,
    pub _unused6: [u32; 3],
    pub /: *mut *mut u32 idr3; / 0x70 Indirect Data Register 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal2_aes_regs {
    pub /: *mut *mut u32 rx_stat[2]; / Status registers,
    pub /: *mut *mut u32 rx_cr[2]; / Control registers,
    pub /: *mut *mut u32 rx_ud[4]; / User data window,
    pub /: *mut *mut u32 rx_st[24]; / Channel status data,
    pub /: *mut *mut u32 tx_stat[1]; / Status register,
    pub /: *mut *mut u32 tx_cr[3]; / Control registers,
    pub /: *mut *mut u32 tx_ud[4]; / User data window,
    pub /: *mut *mut u32 tx_st[24]; / Channel status data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal2_vol_regs {
    pub /: *mut *mut u32 right; / Right volume,
    pub /: *mut *mut u32 left; / Left volume,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hal2_syn_regs {
    pub _unused0: [u32; 2],
    pub /: *mut *mut u32 page; / DOC Page register,
    pub /: *mut *mut u32 regsel; / DOC Register selection,
    pub /: *mut *mut u32 dlow; / DOC Data low,
    pub /: *mut *mut u32 dhigh; / DOC Data high,
    pub /: *mut *mut u32 irq; / IRQ Status,
    pub /: *mut *mut u32 dram; / DRAM Access,
}
