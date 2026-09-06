//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/arc/mcip.h
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
// ARConnect IP Support (Multi core enabler: Cross core IPI, RTC ...)
//
// Copyright (C) 2014-15 Synopsys, Inc. (www.synopsys.com)
//

pub const ARC_REG_MCIP_BCR: c_uint = 0x0d0;
pub const ARC_REG_MCIP_IDU_BCR: c_uint = 0x0D5;
pub const ARC_REG_GFRC_BUILD: c_uint = 0x0D6;
pub const ARC_REG_MCIP_CMD: c_uint = 0x600;
pub const ARC_REG_MCIP_WDATA: c_uint = 0x601;
pub const ARC_REG_MCIP_READBACK: c_uint = 0x602;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcip_cmd {

    pub cmd:8: unsigned int pad:8, param:16,,

    pub pad:8: unsigned int cmd:8, param:16,,

pub const CMD_INTRPT_GENERATE_IRQ: c_uint = 0x01;
pub const CMD_INTRPT_GENERATE_ACK: c_uint = 0x02;
pub const CMD_INTRPT_READ_STATUS: c_uint = 0x03;
pub const CMD_INTRPT_CHECK_SOURCE: c_uint = 0x04;
// Semaphore Commands
pub const CMD_SEMA_CLAIM_AND_READ: c_uint = 0x11;
pub const CMD_SEMA_RELEASE: c_uint = 0x12;
pub const CMD_DEBUG_SET_MASK: c_uint = 0x34;
pub const CMD_DEBUG_READ_MASK: c_uint = 0x35;
pub const CMD_DEBUG_SET_SELECT: c_uint = 0x36;
pub const CMD_DEBUG_READ_SELECT: c_uint = 0x37;
pub const CMD_GFRC_READ_LO: c_uint = 0x42;
pub const CMD_GFRC_READ_HI: c_uint = 0x43;
pub const CMD_GFRC_SET_CORE: c_uint = 0x47;
pub const CMD_GFRC_READ_CORE: c_uint = 0x48;
pub const CMD_IDU_ENABLE: c_uint = 0x71;
pub const CMD_IDU_DISABLE: c_uint = 0x72;
pub const CMD_IDU_SET_MODE: c_uint = 0x74;
pub const CMD_IDU_READ_MODE: c_uint = 0x75;
pub const CMD_IDU_SET_DEST: c_uint = 0x76;
pub const CMD_IDU_ACK_CIRQ: c_uint = 0x79;
pub const CMD_IDU_SET_MASK: c_uint = 0x7C;
pub const IDU_M_TRIG_LEVEL: c_uint = 0x0;
pub const IDU_M_TRIG_EDGE: c_uint = 0x1;
pub const IDU_M_DISTRI_RR: c_uint = 0x0;
pub const IDU_M_DISTRI_DEST: c_uint = 0x2;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcip_bcr {

    pub pad4:6: pad3:1, pw_dom:1,,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcip_idu_bcr {

    pub ver:8: unsigned int pad:21, cirqnum:3,,

    pub pad:21: unsigned int ver:8, cirqnum:3,,

}

//
// Build register for IDU contains not an actual number of supported common
// interrupts but an exponent of 2 which must be multiplied by 4 to
// get a number of supported common interrupts.
//

//
// MCIP programming model
//
// - Simple commands write {cmd:8,param:16} to MCIP_CMD aux reg
// (param could be irq, common_irq, core_id ...)
// - More involved commands setup MCIP_WDATA with cmd specific data
// before invoking the simple command
//
// Setup additional data for a cmd
// Callers need to lock to ensure atomicity
//
// Read MCIP register
//
extern "C" {
    pub fn read_aux_reg(_arg: ARC_REG_MCIP_READBACK) -> return;
}
