//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/cc770/cc770.h
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
// Core driver for the CC770 and AN82527 CAN controllers
//
// Copyright (C) 2009, 2011 Wolfgang Grandegger <wg@grandegger.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc770_msgobj {
    pub ctrl0: u8,
    pub ctrl1: u8,
    pub id: [u8; 4],
    pub config: u8,
    pub data: [u8; 8],
    pub /: *mut *mut u8 dontuse; / padding,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc770_regs {
    pub /: *mut *mut cc770_msgobj msgobj[16]; / Message object 1..15,
    pub /: *mut *mut u8 control; / Control Register,
    pub /: *mut *mut u8 status; / Status Register,
    pub /: *mut *mut u8 cpu_interface; / CPU Interface Register,
    pub dontuse1: u8,
    pub /: *mut *mut u8 high_speed_read[2]; / High Speed Read,
    pub /: *mut *mut u8 global_mask_std[2]; / Standard Global Mask,
    pub /: *mut *mut u8 global_mask_ext[4]; / Extended Global Mask,
    pub /: *mut *mut u8 msg15_mask[4]; / Message 15 Mask,
    pub dontuse2: [u8; 15],
    pub /: *mut *mut u8 clkout; / Clock Out Register,
    pub dontuse3: [u8; 15],
    pub /: *mut *mut u8 bus_config; / Bus Configuration Register,
    pub dontuse4: [u8; 15],
    pub /: *mut *mut u8 bit_timing_0; / Bit Timing Register byte 0,
    pub dontuse5: [u8; 15],
    pub /: *mut *mut u8 bit_timing_1; / Bit Timing Register byte 1,
    pub dontuse6: [u8; 15],
    pub /: *mut *mut u8 interrupt; / Interrupt Register,
    pub dontuse7: [u8; 15],
    pub /: *mut *mut u8 rx_error_counter; / Receive Error Counter,
    pub dontuse8: [u8; 15],
    pub /: *mut *mut u8 tx_error_counter; / Transmit Error Counter,
    pub dontuse9: [u8; 31],
    pub p1_conf: u8,
    pub dontuse10: [u8; 15],
    pub p2_conf: u8,
    pub dontuse11: [u8; 15],
    pub p1_in: u8,
    pub dontuse12: [u8; 15],
    pub p2_in: u8,
    pub dontuse13: [u8; 15],
    pub p1_out: u8,
    pub dontuse14: [u8; 15],
    pub p2_out: u8,
    pub dontuse15: [u8; 15],
    pub serial_reset_addr: u8,
}

// Control Register (0x00)
pub const CTRL_INI: c_uint = 0x01	/* Initialization */;
pub const CTRL_IE: c_uint = 0x02	/* Interrupt Enable */;
pub const CTRL_SIE: c_uint = 0x04	/* Status Interrupt Enable */;
pub const CTRL_EIE: c_uint = 0x08	/* Error Interrupt Enable */;
pub const CTRL_EAF: c_uint = 0x20	/* Enable additional functions */;
pub const CTRL_CCE: c_uint = 0x40	/* Change Configuration Enable */;
// Status Register (0x01)
pub const STAT_LEC_STUFF: c_uint = 0x01	/* Stuff error */;
pub const STAT_LEC_FORM: c_uint = 0x02	/* Form error */;
pub const STAT_LEC_ACK: c_uint = 0x03	/* Acknowledgement error */;
pub const STAT_LEC_BIT1: c_uint = 0x04	/* Bit1 error */;
pub const STAT_LEC_BIT0: c_uint = 0x05	/* Bit0 error */;
pub const STAT_LEC_CRC: c_uint = 0x06	/* CRC error */;
pub const STAT_LEC_MASK: c_uint = 0x07	/* Last Error Code mask */;
pub const STAT_TXOK: c_uint = 0x08	/* Transmit Message Successfully */;
pub const STAT_RXOK: c_uint = 0x10	/* Receive Message Successfully */;
pub const STAT_WAKE: c_uint = 0x20	/* Wake Up Status */;
pub const STAT_WARN: c_uint = 0x40	/* Warning Status */;
pub const STAT_BOFF: c_uint = 0x80	/* Bus Off Status */;
//
// CPU Interface Register (0x02)
// Clock Out Register (0x1f)
// Bus Configuration Register (0x2f)
//
// see include/linux/can/platform/cc770.h
//
// Message Control Register 0 (Base Address + 0x0)
pub const INTPND_RES: c_uint = 0x01	/* No Interrupt pending */;
pub const INTPND_SET: c_uint = 0x02	/* Interrupt pending */;
pub const INTPND_UNC: c_uint = 0x03;
pub const RXIE_RES: c_uint = 0x04	/* Receive Interrupt Disable */;
pub const RXIE_SET: c_uint = 0x08	/* Receive Interrupt Enable */;
pub const RXIE_UNC: c_uint = 0x0c;
pub const TXIE_RES: c_uint = 0x10	/* Transmit Interrupt Disable */;
pub const TXIE_SET: c_uint = 0x20	/* Transmit Interrupt Enable */;
pub const TXIE_UNC: c_uint = 0x30;
pub const MSGVAL_RES: c_uint = 0x40	/* Message Invalid */;
pub const MSGVAL_SET: c_uint = 0x80	/* Message Valid */;
pub const MSGVAL_UNC: c_uint = 0xc0;
// Message Control Register 1 (Base Address + 0x01)
pub const NEWDAT_RES: c_uint = 0x01	/* No New Data */;
pub const NEWDAT_SET: c_uint = 0x02	/* New Data */;
pub const NEWDAT_UNC: c_uint = 0x03;
pub const MSGLST_RES: c_uint = 0x04	/* No Message Lost */;
pub const MSGLST_SET: c_uint = 0x08	/* Message Lost */;
pub const MSGLST_UNC: c_uint = 0x0c;
pub const CPUUPD_RES: c_uint = 0x04	/* No CPU Updating */;
pub const CPUUPD_SET: c_uint = 0x08	/* CPU Updating */;
pub const CPUUPD_UNC: c_uint = 0x0c;
pub const TXRQST_RES: c_uint = 0x10	/* No Transmission Request */;
pub const TXRQST_SET: c_uint = 0x20	/* Transmission Request */;
pub const TXRQST_UNC: c_uint = 0x30;
pub const RMTPND_RES: c_uint = 0x40	/* No Remote Request Pending */;
pub const RMTPND_SET: c_uint = 0x80	/* Remote Request Pending */;
pub const RMTPND_UNC: c_uint = 0xc0;
// Message Configuration Register (Base Address + 0x06)
pub const MSGCFG_XTD: c_uint = 0x04	/* Extended Identifier */;
pub const MSGCFG_DIR: c_uint = 0x08	/* Direction is Transmit */;
pub const MSGOBJ_FIRST: c_int = 1;
pub const MSGOBJ_LAST: c_int = 15;
pub const CC770_IO_SIZE: c_uint = 0x100;

pub const CC770_ECHO_SKB_MAX: c_int = 1;

//
// Message objects and flags used by this driver
//
pub const CC770_OBJ_FLAG_RX: c_uint = 0x01;
pub const CC770_OBJ_FLAG_RTR: c_uint = 0x02;
pub const CC770_OBJ_FLAG_EFF: c_uint = 0x04;

//
// CC770 private data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc770_priv {
    pub /: *mut *mut can_priv can; / must be the first member,
    pub echo_skb: *mut sk_buff,
// the lower-layer is responsible for appropriate locking
    pub reg): *const *const *const u8 (read_reg)(struct cc770_priv priv, int,
    pub val): *const *const *const void (write_reg)(struct cc770_priv priv, int reg, u8,
    pub priv): *const *const void (pre_irq)(struct cc770_priv,
    pub priv): *const *const void (post_irq)(struct cc770_priv,
    pub /: *mut *mut *mut void priv; / for board-specific data,
    pub dev: *mut net_device,
    pub /: *mut *mut *mut void __iomem reg_base; / ioremap'ed address to registers,
    pub /: *mut *mut unsigned long irq_flags; / for request_irq(),
    pub obj_flags: [c_uchar; CC770_OBJ_MAX],
    pub /: *mut *mut u8 control_normal_mode; / Control register for normal mode,
    pub /: *mut *mut u8 cpu_interface; / CPU interface register,
    pub /: *mut *mut u8 clkout; / Clock out register,
    pub /: *mut *mut u8 bus_config; / Bus configuration register,
    pub tx_skb: *mut sk_buff,
}

extern "C" {
    pub fn free_cc770dev(dev: *mut net_device);
}
extern "C" {
    pub fn register_cc770dev(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn unregister_cc770dev(dev: *mut net_device);
}
