//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/ppc4xx/dma.h
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
// 440SPe's DMA engines support header file
//
// 2006-2009 (C) DENX Software Engineering.
//
// Author: Yuri Tikhonov <yur@emcraft.com>
//

// Number of elements in the array with statical CDBs
pub const MAX_STAT_DMA_CDBS: c_int = 16;
// Number of DMA engines available on the controller
pub const DMA_ENGINES_NUM: c_int = 2;
// Maximum h/w supported number of destinations
pub const DMA_DEST_MAX_NUM: c_int = 2;
// FIFO's params
pub const DMA0_FIFO_SIZE: c_uint = 0x1000;
pub const DMA1_FIFO_SIZE: c_uint = 0x1000;

// DMA Configuration Register. Data Transfer Engine PLB Priority:

// DMA Configuration Register. DMA FIFO Manager PLB Priority:

// DMA Configuration Register. Force 64-byte Alignment

// UIC0:

// UIC1:

// I2O IOP Interrupt Mask Register

// DMA CDB fields

// DMA CDB OpCodes

pub const DMA_CUED_MULT1_OFF: c_int = 0;
pub const DMA_CUED_MULT2_OFF: c_int = 8;
pub const DMA_CUED_MULT3_OFF: c_int = 16;
pub const DMA_CUED_REGION_OFF: c_int = 24;

pub const DMA_CUED_MULT1_OFF: c_int = 2;
pub const DMA_CUED_MULT2_OFF: c_int = 10;
pub const DMA_CUED_MULT3_OFF: c_int = 18;
pub const DMA_CUED_REGION_OFF: c_int = 26;

pub const DMA_CUED_REGION_MSK: c_uint = 0x3;
pub const DMA_RXOR123: c_uint = 0x0;
pub const DMA_RXOR124: c_uint = 0x1;
pub const DMA_RXOR125: c_uint = 0x2;
pub const DMA_RXOR12: c_uint = 0x3;
// S/G addresses
pub const DMA_CDB_SG_SRC: c_int = 1;
pub const DMA_CDB_SG_DST1: c_int = 2;
pub const DMA_CDB_SG_DST2: c_int = 3;
//
// DMAx engines Command Descriptor Block Type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_cdb {
//
// Basic CDB structure (Table 20-17, p.499, 440spe_um_1_22.pdf)
//
    pub /: *mut *mut u8 pad0[2]; / reserved,
    pub /: *mut *mut u8 attr; / attributes,
    pub /: *mut *mut u8 opc; / opcode,
    pub /: *mut *mut u32 sg1u; / upper SG1 address,
    pub /: *mut *mut u32 sg1l; / lower SG1 address,
    pub /: *mut *mut u32 cnt; / SG count, 3B used,
    pub /: *mut *mut u32 sg2u; / upper SG2 address,
    pub /: *mut *mut u32 sg2l; / lower SG2 address,
    pub /: *mut *mut u32 sg3u; / upper SG3 address,
    pub /: *mut *mut u32 sg3l; / lower SG3 address,
}

//
// DMAx hardware registers (p.515 in 440SPe UM 1.22)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_regs {
    pub cpfpl: u32,
    pub cpfph: u32,
    pub csfpl: u32,
    pub csfph: u32,
    pub dsts: u32,
    pub cfg: u32,
    pub pad0: [u8; 0x8],
    pub cpfhp: u16,
    pub cpftp: u16,
    pub csfhp: u16,
    pub csftp: u16,
    pub pad1: [u8; 0x8],
    pub acpl: u32,
    pub acph: u32,
    pub s1bpl: u32,
    pub s1bph: u32,
    pub s2bpl: u32,
    pub s2bph: u32,
    pub s3bpl: u32,
    pub s3bph: u32,
    pub pad2: [u8; 0x10],
    pub earl: u32,
    pub earh: u32,
    pub pad3: [u8; 0x8],
    pub seat: u32,
    pub sead: u32,
    pub op: u32,
    pub fsiz: u32,
}

//
// I2O hardware registers (p.528 in 440SPe UM 1.22)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2o_regs {
    pub ists: u32,
    pub iseat: u32,
    pub isead: u32,
    pub pad0: [u8; 0x14],
    pub idbel: u32,
    pub pad1: [u8; 0xc],
    pub ihis: u32,
    pub ihim: u32,
    pub pad2: [u8; 0x8],
    pub ihiq: u32,
    pub ihoq: u32,
    pub pad3: [u8; 0x8],
    pub iopis: u32,
    pub iopim: u32,
    pub iopiq: u32,
    pub iopoq: u8,
    pub pad4: [u8; 3],
    pub iiflh: u16,
    pub iiflt: u16,
    pub iiplh: u16,
    pub iiplt: u16,
    pub ioflh: u16,
    pub ioflt: u16,
    pub ioplh: u16,
    pub ioplt: u16,
    pub iidc: u32,
    pub ictl: u32,
    pub ifcpp: u32,
    pub pad5: [u8; 0x4],
    pub mfac0: u16,
    pub mfac1: u16,
    pub mfac2: u16,
    pub mfac3: u16,
    pub mfac4: u16,
    pub mfac5: u16,
    pub mfac6: u16,
    pub mfac7: u16,
    pub ifcfh: u16,
    pub ifcht: u16,
    pub pad6: [u8; 0x4],
    pub iifmc: u32,
    pub iodb: u32,
    pub iodbc: u32,
    pub ifbal: u32,
    pub ifbah: u32,
    pub ifsiz: u32,
    pub ispd0: u32,
    pub ispd1: u32,
    pub ispd2: u32,
    pub ispd3: u32,
    pub ihipl: u32,
    pub ihiph: u32,
    pub ihopl: u32,
    pub ihoph: u32,
    pub iiipl: u32,
    pub iiiph: u32,
    pub iiopl: u32,
    pub iioph: u32,
    pub ifcpl: u32,
    pub ifcph: u32,
    pub pad7: [u8; 0x8],
    pub iopt: u32,
}
