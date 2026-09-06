//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/fjes/fjes_regs.h
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
// FUJITSU Extended Socket Network Device driver
// Copyright (c) 2015 FUJITSU LIMITED
//

pub const XSCT_DEVICE_REGISTER_SIZE: c_uint = 0x1000;
// register offset
// Information registers
pub const XSCT_OWNER_EPID: c_uint = 0x0000  /* Owner EPID */;
pub const XSCT_MAX_EP: c_uint = 0x0004  /* Maximum EP */;
// Device Control registers
pub const XSCT_DCTL: c_uint = 0x0010  /* Device Control */;
// Command Control registers
pub const XSCT_CR: c_uint = 0x0020  /* Command request */;
pub const XSCT_CS: c_uint = 0x0024  /* Command status */;
pub const XSCT_SHSTSAL: c_uint = 0x0028  /* Share status address Low */;
pub const XSCT_SHSTSAH: c_uint = 0x002C  /* Share status address High */;
pub const XSCT_REQBL: c_uint = 0x0034  /* Request Buffer length */;
pub const XSCT_REQBAL: c_uint = 0x0038  /* Request Buffer Address Low */;
pub const XSCT_REQBAH: c_uint = 0x003C  /* Request Buffer Address High */;
pub const XSCT_RESPBL: c_uint = 0x0044  /* Response Buffer Length */;
pub const XSCT_RESPBAL: c_uint = 0x0048  /* Response Buffer Address Low */;
pub const XSCT_RESPBAH: c_uint = 0x004C  /* Response Buffer Address High */;
// Interrupt Control registers
pub const XSCT_IS: c_uint = 0x0080  /* Interrupt status */;
pub const XSCT_IMS: c_uint = 0x0084  /* Interrupt mask set */;
pub const XSCT_IMC: c_uint = 0x0088  /* Interrupt mask clear */;
pub const XSCT_IG: c_uint = 0x008C  /* Interrupt generator */;
pub const XSCT_ICTL: c_uint = 0x0090  /* Interrupt control */;
// register structure
// Information registers
#[repr(C)]
#[derive(Copy, Clone)]
pub union REG_OWNER_EPID {
    pub epid:16: __le32,
    pub bits: },
    pub reg: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union REG_MAX_EP {
    pub maxep:16: __le32,
    pub bits: },
    pub reg: __le32,
}

// Device Control registers
#[repr(C)]
#[derive(Copy, Clone)]
pub union REG_DCTL {
    pub reset:1: __le32,
    pub rsv0:15: __le32,
    pub rsv1:16: __le32,
    pub bits: },
    pub reg: __le32,
}

// Command Control registers
#[repr(C)]
#[derive(Copy, Clone)]
pub union REG_CR {
    pub req_code:16: __le32,
    pub err_info:14: __le32,
    pub error:1: __le32,
    pub req_start:1: __le32,
    pub bits: },
    pub reg: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union REG_CS {
    pub req_code:16: __le32,
    pub rsv0:14: __le32,
    pub busy:1: __le32,
    pub complete:1: __le32,
    pub bits: },
    pub reg: __le32,
}

// Interrupt Control registers
#[repr(C)]
#[derive(Copy, Clone)]
pub union REG_ICTL {
    pub automak:1: __le32,
    pub rsv0:31: __le32,
    pub bits: },
    pub reg: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum REG_ICTL_MASK {
    REG_ICTL_MASK_INFO_UPDATE     = 1 << 20,
    REG_ICTL_MASK_DEV_STOP_REQ    = 1 << 19,
    REG_ICTL_MASK_TXRX_STOP_REQ   = 1 << 18,
    REG_ICTL_MASK_TXRX_STOP_DONE  = 1 << 17,
    REG_ICTL_MASK_RX_DATA         = 1 << 16,
    REG_ICTL_MASK_ALL             = GENMASK(20, 16),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum REG_IS_MASK {
    REG_IS_MASK_IS_ASSERT	= 1 << 31,
    REG_IS_MASK_EPID	= GENMASK(15, 0),
}

extern "C" {
    pub fn fjes_hw_rd32(hw: *mut fjes_hw, reg: u32) -> u32;
}

