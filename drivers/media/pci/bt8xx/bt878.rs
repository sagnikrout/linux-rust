//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/bt8xx/bt878.h
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

pub const BT878_VERSION_CODE: c_uint = 0x000000;
pub const BT878_AINT_STAT: c_uint = 0x100;

pub const BT878_AINT_MASK: c_uint = 0x104;
pub const BT878_AGPIO_DMA_CTL: c_uint = 0x10c;

pub const BT878_FIFO_EN: c_int = 1;
pub const BT878_APACK_LEN: c_uint = 0x110;

pub const BT878_ALP_LEN: c_uint = 0xfff;
pub const BT878_ARISC_START: c_uint = 0x114;
pub const BT878_ARISC_PC: c_uint = 0x120;
// BT878 FUNCTION 0 REGISTERS
pub const BT878_GPIO_DMA_CTL: c_uint = 0x10c;
// Interrupt register
pub const BT878_INT_STAT: c_uint = 0x100;
pub const BT878_INT_MASK: c_uint = 0x104;

pub const BT878_MAX: c_int = 4;

pub const BTTV_BOARD_UNKNOWN: c_uint = 0x00;
pub const BTTV_BOARD_PINNACLESAT: c_uint = 0x5e;
pub const BTTV_BOARD_NEBULA_DIGITV: c_uint = 0x68;
pub const BTTV_BOARD_PC_HDTV: c_uint = 0x70;
pub const BTTV_BOARD_TWINHAN_DST: c_uint = 0x71;
pub const BTTV_BOARD_AVDVBT_771: c_uint = 0x7b;
pub const BTTV_BOARD_AVDVBT_761: c_uint = 0x7c;
pub const BTTV_BOARD_DVICO_DVBT_LITE: c_uint = 0x80;
pub const BTTV_BOARD_DVICO_FUSIONHDTV_5_LITE: c_uint = 0x87;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt878 {
    pub gpio_lock: mutex,
    pub nr: c_uint,
    pub bttv_nr: c_uint,
    pub adapter: *mut i2c_adapter,
    pub dev: *mut pci_dev,
    pub id: c_uint,
    pub TS_Size: c_uint,
    pub revision: c_uchar,
    pub irq: c_uint,
    pub bt878_adr: c_ulong,
    pub /: *mut *mut *mut volatile void __iomem bt878_mem; / function 1,
    pub finished_block: volatile u32,
    pub last_block: volatile u32,
    pub block_count: u32,
    pub block_bytes: u32,
    pub line_bytes: u32,
    pub line_count: u32,
    pub buf_size: u32,
    pub buf_cpu: *mut u8,
    pub buf_dma: dma_addr_t,
    pub risc_size: u32,
    pub risc_cpu: *mut __le32,
    pub risc_dma: dma_addr_t,
    pub risc_pos: u32,
    pub bh_work: work_struct,
    pub shutdown: c_int,
}

extern "C" {
    pub fn bt878_stop(bt: *mut bt878);
}

