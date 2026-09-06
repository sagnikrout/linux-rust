//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mailbox/mtk-cmdq-mailbox.h
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
// Copyright (c) 2018 MediaTek Inc.
//

pub const CMDQ_SUBSYS_SHIFT: c_int = 16;
pub const CMDQ_OP_CODE_SHIFT: c_int = 24;

pub const CMDQ_WFE_WAIT_VALUE: c_uint = 0x1;
//
// WFE arg_b
// bit 0-11: wait value
// bit 15: 1 - wait, 0 - no wait
// bit 16-27: update value
// bit 31: 1 - update, 0 - no update
//

// cmdq event maximum
pub const CMDQ_MAX_EVENT: c_uint = 0x3ff;
//
// CMDQ_CODE_MASK:
// set write mask
// format: op mask
// CMDQ_CODE_WRITE:
// write value into target register
// format: op subsys address value
// CMDQ_CODE_JUMP:
// jump by offset
// format: op offset
// CMDQ_CODE_WFE:
// wait for event and clear
// it is just clear if no wait
// format: [wait]  op event update:1 to_wait:1 wait:1
// [clear] op event update:1 to_wait:0 wait:0
// CMDQ_CODE_EOC:
// end of command
// format: op irq_flag
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmdq_code {
    CMDQ_CODE_MASK = 0x02,
    CMDQ_CODE_WRITE = 0x04,
    CMDQ_CODE_POLL = 0x08,
    CMDQ_CODE_JUMP = 0x10,
    CMDQ_CODE_WFE = 0x20,
    CMDQ_CODE_EOC = 0x40,
    CMDQ_CODE_READ_S = 0x80,
    CMDQ_CODE_WRITE_S = 0x90,
    CMDQ_CODE_WRITE_S_MASK = 0x91,
    CMDQ_CODE_LOGIC = 0xa0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_cb_data {
    pub sta: c_int,
    pub pkt: *mut cmdq_pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_mbox_priv {
    pub shift_pa: u8,
    pub mminfra_offset: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_pkt {
    pub va_base: *mut c_void,
    pub pa_base: dma_addr_t,
    pub /: *mut *mut size_t cmd_buf_size; / command occupied size,
    pub /: *mut *mut size_t buf_size; / real buffer size,
    pub /: *mut *mut cmdq_mbox_priv priv; / for generating instruction,
}

//
// cmdq_get_mbox_priv() - get the private data of mailbox channel
// @chan: mailbox channel
// @priv: pointer to store the private data of mailbox channel
//
// While generating the GCE instruction to command buffer, the private data
// of GCE hardware may need to be referenced, such as the shift bits of
// physical address.
//
// This function should be called before generating the GCE instruction.
//
extern "C" {
    pub fn cmdq_get_mbox_priv(chan: *mut mbox_chan, priv: *mut cmdq_mbox_priv);
}
//
// cmdq_get_shift_pa() - get the shift bits of physical address
// @chan: mailbox channel
//
// GCE can only fetch the command buffer address from a 32-bit register.
// Some SOCs support more than 32-bit command buffer address for GCE, which
// requires some shift bits to make the address fit into the 32-bit register.
//
// Return: the shift bits of physical address
//
extern "C" {
    pub fn cmdq_get_shift_pa(chan: *mut mbox_chan) -> u8;
}
