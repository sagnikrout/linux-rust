//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mesh.h
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
// mesh.h: definitions for the driver for the MESH SCSI bus adaptor
// (Macintosh Enhanced SCSI Hardware) found on Power Macintosh computers.
//
// Copyright (C) 1996 Paul Mackerras.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_cmd_priv {
    pub this_residual: c_int,
    pub message: c_int,
    pub status: c_int,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
//
// Registers in the MESH controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mesh_regs {
    pub count_lo: c_uchar,
    pub pad0: [c_char; 15],
    pub count_hi: c_uchar,
    pub pad1: [c_char; 15],
    pub fifo: c_uchar,
    pub pad2: [c_char; 15],
    pub sequence: c_uchar,
    pub pad3: [c_char; 15],
    pub bus_status0: c_uchar,
    pub pad4: [c_char; 15],
    pub bus_status1: c_uchar,
    pub pad5: [c_char; 15],
    pub fifo_count: c_uchar,
    pub pad6: [c_char; 15],
    pub exception: c_uchar,
    pub pad7: [c_char; 15],
    pub error: c_uchar,
    pub pad8: [c_char; 15],
    pub intr_mask: c_uchar,
    pub pad9: [c_char; 15],
    pub interrupt: c_uchar,
    pub pad10: [c_char; 15],
    pub source_id: c_uchar,
    pub pad11: [c_char; 15],
    pub dest_id: c_uchar,
    pub pad12: [c_char; 15],
    pub sync_params: c_uchar,
    pub pad13: [c_char; 15],
    pub mesh_id: c_uchar,
    pub pad14: [c_char; 15],
    pub sel_timeout: c_uchar,
    pub pad15: [c_char; 15],
}

// Bits in the sequence register.
pub const SEQ_DMA_MODE: c_uint = 0x80	/* use DMA for data transfer */;
pub const SEQ_TARGET: c_uint = 0x40	/* put the controller into target mode */;
pub const SEQ_ATN: c_uint = 0x20	/* assert ATN signal */;
pub const SEQ_ACTIVE_NEG: c_uint = 0x10	/* use active negation on REQ/ACK */;
pub const SEQ_CMD: c_uint = 0x0f	/* command bits: */;

pub const SEQ_ENBPARITY: c_uint = 0x0a	/*  enable parity checking */;
pub const SEQ_DISPARITY: c_uint = 0x0b	/*  disable parity checking */;
pub const SEQ_ENBRESEL: c_uint = 0x0c	/*  enable reselection */;
pub const SEQ_DISRESEL: c_uint = 0x0d	/*  disable reselection */;
pub const SEQ_RESETMESH: c_uint = 0x0e	/*  reset the controller */;
pub const SEQ_FLUSHFIFO: c_uint = 0x0f	/*  clear out the FIFO */;
// Bits in the bus_status0 and bus_status1 registers:
pub const BS0_REQ: c_uint = 0x20;
pub const BS0_ACK: c_uint = 0x10;
pub const BS0_ATN: c_uint = 0x08;
pub const BS0_MSG: c_uint = 0x04;
pub const BS0_CD: c_uint = 0x02;
pub const BS0_IO: c_uint = 0x01;
pub const BS1_RST: c_uint = 0x80;
pub const BS1_BSY: c_uint = 0x40;
pub const BS1_SEL: c_uint = 0x20;
// Bus phases defined by the bits in bus_status0

pub const BP_DATAOUT: c_int = 0;

// Bits in the exception register.
pub const EXC_SELWATN: c_uint = 0x20	/* (as target) we were selected with ATN */;
pub const EXC_SELECTED: c_uint = 0x10	/* (as target) we were selected w/o ATN */;
pub const EXC_RESELECTED: c_uint = 0x08	/* (as initiator) we were reselected */;
pub const EXC_ARBLOST: c_uint = 0x04	/* we lost arbitration */;
pub const EXC_PHASEMM: c_uint = 0x02	/* SCSI phase mismatch */;
pub const EXC_SELTO: c_uint = 0x01	/* selection timeout */;
// Bits in the error register
pub const ERR_UNEXPDISC: c_uint = 0x40	/* target unexpectedly disconnected */;
pub const ERR_SCSIRESET: c_uint = 0x20	/* SCSI bus got reset on us */;
pub const ERR_SEQERR: c_uint = 0x10	/* we did something the chip didn't like */;
pub const ERR_PARITY: c_uint = 0x01	/* parity error was detected */;
// Bits in the interrupt and intr_mask registers
pub const INT_ERROR: c_uint = 0x04	/* error interrupt */;
pub const INT_EXCEPTION: c_uint = 0x02	/* exception interrupt */;
pub const INT_CMDDONE: c_uint = 0x01	/* command done interrupt */;
// Fields in the sync_params register

//
// Assuming a clock frequency of 50MHz:
//
// The transfer period with SYNC_PER(sync_params) == x
// is (x + 2) * 40ns, except that x == 0 gives 100ns.
//
// The units of the sel_timeout register are 10ms.
//
