//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firewire/nosy.h
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
// Chip register definitions for PCILynx chipset.  Based on pcilynx.h
// from the Linux 1394 drivers, but modified a bit so the names here
// match the specification exactly (even though they have weird names,
// like xxx_OVER_FLOW, or arbitrary abbreviations like SNTRJ for "sent
// reject" etc.)
//
pub const PCILYNX_MAX_REGISTER: c_uint = 0xfff;
pub const PCILYNX_MAX_MEMORY: c_uint = 0xffff;
pub const PCI_LATENCY_CACHELINE: c_uint = 0x0c;
pub const MISC_CONTROL: c_uint = 0x40;

pub const SERIAL_EEPROM_CONTROL: c_uint = 0x44;
pub const PCI_INT_STATUS: c_uint = 0x48;
pub const PCI_INT_ENABLE: c_uint = 0x4c;
// status and enable have identical bit numbers

// all DMA interrupts combined:
pub const PCI_INT_DMA_ALL: c_uint = 0x3ff;

pub const LBUS_ADDR: c_uint = 0xb4;

pub const GPIO_CTRL_A: c_uint = 0xb8;
pub const GPIO_CTRL_B: c_uint = 0xbc;
pub const GPIO_DATA_BASE: c_uint = 0xc0;

// transfer commands

// aux commands

// BRANCH condition codes

pub const DMA0_PREV_PCL: c_uint = 0x100;
pub const DMA1_PREV_PCL: c_uint = 0x120;
pub const DMA2_PREV_PCL: c_uint = 0x140;
pub const DMA3_PREV_PCL: c_uint = 0x160;
pub const DMA4_PREV_PCL: c_uint = 0x180;

pub const DMA0_CURRENT_PCL: c_uint = 0x104;
pub const DMA1_CURRENT_PCL: c_uint = 0x124;
pub const DMA2_CURRENT_PCL: c_uint = 0x144;
pub const DMA3_CURRENT_PCL: c_uint = 0x164;
pub const DMA4_CURRENT_PCL: c_uint = 0x184;

pub const DMA0_CHAN_STAT: c_uint = 0x10c;
pub const DMA1_CHAN_STAT: c_uint = 0x12c;
pub const DMA2_CHAN_STAT: c_uint = 0x14c;
pub const DMA3_CHAN_STAT: c_uint = 0x16c;
pub const DMA4_CHAN_STAT: c_uint = 0x18c;

// CHAN_STATUS registers share bits

pub const DMA0_CHAN_CTRL: c_uint = 0x110;
pub const DMA1_CHAN_CTRL: c_uint = 0x130;
pub const DMA2_CHAN_CTRL: c_uint = 0x150;
pub const DMA3_CHAN_CTRL: c_uint = 0x170;
pub const DMA4_CHAN_CTRL: c_uint = 0x190;

// CHAN_CTRL registers share bits

pub const DMA0_READY: c_uint = 0x114;
pub const DMA1_READY: c_uint = 0x134;
pub const DMA2_READY: c_uint = 0x154;
pub const DMA3_READY: c_uint = 0x174;
pub const DMA4_READY: c_uint = 0x194;

pub const DMA_GLOBAL_REGISTER: c_uint = 0x908;
pub const FIFO_SIZES: c_uint = 0xa00;
pub const FIFO_CONTROL: c_uint = 0xa10;

pub const FIFO_XMIT_THRESHOLD: c_uint = 0xa14;
pub const DMA0_WORD0_CMP_VALUE: c_uint = 0xb00;
pub const DMA1_WORD0_CMP_VALUE: c_uint = 0xb10;
pub const DMA2_WORD0_CMP_VALUE: c_uint = 0xb20;
pub const DMA3_WORD0_CMP_VALUE: c_uint = 0xb30;
pub const DMA4_WORD0_CMP_VALUE: c_uint = 0xb40;

pub const DMA0_WORD0_CMP_ENABLE: c_uint = 0xb04;
pub const DMA1_WORD0_CMP_ENABLE: c_uint = 0xb14;
pub const DMA2_WORD0_CMP_ENABLE: c_uint = 0xb24;
pub const DMA3_WORD0_CMP_ENABLE: c_uint = 0xb34;
pub const DMA4_WORD0_CMP_ENABLE: c_uint = 0xb44;

pub const DMA0_WORD1_CMP_VALUE: c_uint = 0xb08;
pub const DMA1_WORD1_CMP_VALUE: c_uint = 0xb18;
pub const DMA2_WORD1_CMP_VALUE: c_uint = 0xb28;
pub const DMA3_WORD1_CMP_VALUE: c_uint = 0xb38;
pub const DMA4_WORD1_CMP_VALUE: c_uint = 0xb48;

pub const DMA0_WORD1_CMP_ENABLE: c_uint = 0xb0c;
pub const DMA1_WORD1_CMP_ENABLE: c_uint = 0xb1c;
pub const DMA2_WORD1_CMP_ENABLE: c_uint = 0xb2c;
pub const DMA3_WORD1_CMP_ENABLE: c_uint = 0xb3c;
pub const DMA4_WORD1_CMP_ENABLE: c_uint = 0xb4c;

// word 1 compare enable flags

pub const LINK_ID: c_uint = 0xf00;

pub const LINK_CONTROL: c_uint = 0xf04;

pub const CYCLE_TIMER: c_uint = 0xf08;
pub const LINK_PHY: c_uint = 0xf0c;

pub const LINK_INT_STATUS: c_uint = 0xf14;
pub const LINK_INT_ENABLE: c_uint = 0xf18;
// status and enable have identical bit numbers

