//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/softing/softing.h
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
// softing common interfaces
//
// by Kurt Van Dijck, 2008-2010
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct softing_priv {
    pub /: *mut *mut can_priv can; / must be the first member!,
    pub netdev: *mut net_device,
    pub card: *mut softing,
    pub pending: c_int,
// variables which hold the circular buffer
    pub echo_put: c_int,
    pub echo_get: c_int,
    pub tx: },
    pub btr_const: can_bittiming_const,
    pub index: c_int,
    pub output: u8,
    pub chip: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct softing {
    pub pdat: *const softing_platform_data,
    pub pdev: *mut platform_device,
    pub net: [*mut net_device; 2],
    pub /: *mut *mut spinlock_t spin; / protect this structure & DPRAM access,
    pub ts_ref: ktime_t,
    pub /: *mut *mut ktime_t ts_overflow; / timestamp overflow value, in ktime,
// indication of firmware status
    pub up: c_int,
// protection of the 'up' variable
    pub lock: mutex,
    pub fw: },
    pub nr: c_int,
    pub requested: c_int,
    pub svc_count: c_int,
    pub dpram_position: c_uint,
    pub irq: },
    pub pending: c_int,
    pub last_bus: c_int,
//
// keep the bus that last tx'd a message,
// in order to let every netdev queue resume
//
    pub tx: },
    pub dpram: *mut __iomem uint8_t,
    pub dpram_phys: c_ulong,
    pub dpram_size: c_ulong,
    pub serial: uint16_t fw_version, hw_version, license,,
    pub chip: [u16; 2],
    pub /: *mut *mut unsigned int freq; / remote cpu's operating frequency,
    pub id: },
}

extern "C" {
    pub fn softing_default_output(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn softing_raw2ktime(card: *mut softing, raw: u32) -> ktime_t;
}
extern "C" {
    pub fn softing_chip_poweron(card: *mut softing) -> c_int;
}
// Load firmware after reset
// Load final application firmware after bootloader
extern "C" {
    pub fn softing_load_app_fw(file: *const c_char, card: *mut softing) -> c_int;
}
//
// enable or disable irq
// only called with fw.lock locked
//
extern "C" {
    pub fn softing_enable_irq(card: *mut softing, enable: c_int) -> c_int;
}
// start/stop 1 bus on card
extern "C" {
    pub fn softing_startstop(netdev: *mut net_device, up: c_int) -> c_int;
}
// netif_rx()
// SOFTING DPRAM mappings
pub const DPRAM_RX: c_uint = 0x0000;
pub const DPRAM_RX_SIZE: c_int = 32;
pub const DPRAM_RX_CNT: c_int = 16;
pub const DPRAM_RX_RD: c_uint = 0x0201	/* uint8_t */;
pub const DPRAM_RX_WR: c_uint = 0x0205	/* uint8_t */;
pub const DPRAM_RX_LOST: c_uint = 0x0207	/* uint8_t */;
pub const DPRAM_FCT_PARAM: c_uint = 0x0300	/* int16_t [20] */;
pub const DPRAM_FCT_RESULT: c_uint = 0x0328	/* int16_t */;
pub const DPRAM_FCT_HOST: c_uint = 0x032b	/* uint16_t */;
pub const DPRAM_INFO_BUSSTATE: c_uint = 0x0331	/* uint16_t */;
pub const DPRAM_INFO_BUSSTATE2: c_uint = 0x0335	/* uint16_t */;
pub const DPRAM_INFO_ERRSTATE: c_uint = 0x0339	/* uint16_t */;
pub const DPRAM_INFO_ERRSTATE2: c_uint = 0x033d	/* uint16_t */;
pub const DPRAM_RESET: c_uint = 0x0341	/* uint16_t */;
pub const DPRAM_CLR_RECV_FIFO: c_uint = 0x0345	/* uint16_t */;
pub const DPRAM_RESET_TIME: c_uint = 0x034d	/* uint16_t */;
pub const DPRAM_TIME: c_uint = 0x0350	/* uint64_t */;
pub const DPRAM_WR_START: c_uint = 0x0358	/* uint8_t */;
pub const DPRAM_WR_END: c_uint = 0x0359	/* uint8_t */;
pub const DPRAM_RESET_RX_FIFO: c_uint = 0x0361	/* uint16_t */;
pub const DPRAM_RESET_TX_FIFO: c_uint = 0x0364	/* uint8_t */;
pub const DPRAM_READ_FIFO_LEVEL: c_uint = 0x0365	/* uint8_t */;
pub const DPRAM_RX_FIFO_LEVEL: c_uint = 0x0366	/* uint16_t */;
pub const DPRAM_TX_FIFO_LEVEL: c_uint = 0x0366	/* uint16_t */;
pub const DPRAM_TX: c_uint = 0x0400	/* uint16_t */;
pub const DPRAM_TX_SIZE: c_int = 16;
pub const DPRAM_TX_CNT: c_int = 32;
pub const DPRAM_TX_RD: c_uint = 0x0601	/* uint8_t */;
pub const DPRAM_TX_WR: c_uint = 0x0605	/* uint8_t */;
pub const DPRAM_COMMAND: c_uint = 0x07e0	/* uint16_t */;
pub const DPRAM_RECEIPT: c_uint = 0x07f0	/* uint16_t */;
pub const DPRAM_IRQ_TOHOST: c_uint = 0x07fe	/* uint8_t */;
pub const DPRAM_IRQ_TOCARD: c_uint = 0x07ff	/* uint8_t */;
pub const DPRAM_V2_RESET: c_uint = 0x0e00	/* uint8_t */;
pub const DPRAM_V2_IRQ_TOHOST: c_uint = 0x0e02	/* uint8_t */;

// DPRAM return codes
pub const RES_NONE: c_int = 0;
pub const RES_OK: c_int = 1;
pub const RES_NOK: c_int = 2;
pub const RES_UNKNOWN: c_int = 3;
// DPRAM flags
pub const CMD_TX: c_uint = 0x01;
pub const CMD_ACK: c_uint = 0x02;
pub const CMD_XTD: c_uint = 0x04;
pub const CMD_RTR: c_uint = 0x08;
pub const CMD_ERR: c_uint = 0x10;
pub const CMD_BUS2: c_uint = 0x80;
// returned fifo entry bus state masks
pub const SF_MASK_BUSOFF: c_uint = 0x80;
pub const SF_MASK_EPASSIVE: c_uint = 0x60;
// bus states
pub const STATE_BUSOFF: c_int = 2;
pub const STATE_EPASSIVE: c_int = 1;
pub const STATE_EACTIVE: c_int = 0;
