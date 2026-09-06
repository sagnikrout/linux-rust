//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aeroflex/greth.h
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

// Register bits and masks
pub const GRETH_RESET: c_uint = 0x40;
pub const GRETH_MII_BUSY: c_uint = 0x8;
pub const GRETH_MII_NVALID: c_uint = 0x10;
pub const GRETH_CTRL_FD: c_uint = 0x10;
pub const GRETH_CTRL_PR: c_uint = 0x20;
pub const GRETH_CTRL_SP: c_uint = 0x80;
pub const GRETH_CTRL_GB: c_uint = 0x100;
pub const GRETH_CTRL_PSTATIEN: c_uint = 0x400;
pub const GRETH_CTRL_MCEN: c_uint = 0x800;
pub const GRETH_CTRL_DISDUPLEX: c_uint = 0x1000;
pub const GRETH_STATUS_PHYSTAT: c_uint = 0x100;
pub const GRETH_BD_EN: c_uint = 0x800;
pub const GRETH_BD_WR: c_uint = 0x1000;
pub const GRETH_BD_IE: c_uint = 0x2000;
pub const GRETH_BD_LEN: c_uint = 0x7FF;
pub const GRETH_TXEN: c_uint = 0x1;
pub const GRETH_INT_TE: c_uint = 0x2;
pub const GRETH_INT_TX: c_uint = 0x8;
pub const GRETH_TXI: c_uint = 0x4;
pub const GRETH_TXBD_STATUS: c_uint = 0x0001C000;
pub const GRETH_TXBD_MORE: c_uint = 0x20000;
pub const GRETH_TXBD_IPCS: c_uint = 0x40000;
pub const GRETH_TXBD_TCPCS: c_uint = 0x80000;
pub const GRETH_TXBD_UDPCS: c_uint = 0x100000;

pub const GRETH_TXBD_ERR_LC: c_uint = 0x10000;
pub const GRETH_TXBD_ERR_UE: c_uint = 0x4000;
pub const GRETH_TXBD_ERR_AL: c_uint = 0x8000;
pub const GRETH_INT_RE: c_uint = 0x1;
pub const GRETH_INT_RX: c_uint = 0x4;
pub const GRETH_RXEN: c_uint = 0x2;
pub const GRETH_RXI: c_uint = 0x8;
pub const GRETH_RXBD_STATUS: c_uint = 0xFFFFC000;
pub const GRETH_RXBD_ERR_AE: c_uint = 0x4000;
pub const GRETH_RXBD_ERR_FT: c_uint = 0x8000;
pub const GRETH_RXBD_ERR_CRC: c_uint = 0x10000;
pub const GRETH_RXBD_ERR_OE: c_uint = 0x20000;
pub const GRETH_RXBD_ERR_LE: c_uint = 0x40000;
pub const GRETH_RXBD_IP: c_uint = 0x80000;
pub const GRETH_RXBD_IP_CSERR: c_uint = 0x100000;
pub const GRETH_RXBD_UDP: c_uint = 0x200000;
pub const GRETH_RXBD_UDP_CSERR: c_uint = 0x400000;
pub const GRETH_RXBD_TCP: c_uint = 0x800000;
pub const GRETH_RXBD_TCP_CSERR: c_uint = 0x1000000;
pub const GRETH_RXBD_IP_FRAG: c_uint = 0x2000000;
pub const GRETH_RXBD_MCAST: c_uint = 0x4000000;
// Descriptor parameters
pub const GRETH_TXBD_NUM: c_int = 128;

pub const GRETH_TX_BUF_SIZE: c_int = 2048;
pub const GRETH_RXBD_NUM: c_int = 128;

pub const GRETH_RX_BUF_SIZE: c_int = 2048;
// Buffers per page

// How many pages are needed for buffers

// Buffer size.
// Gbit MAC uses tagged maximum frame size which is 1518 excluding CRC.
// Set to 1520 to make all buffers word aligned for non-gbit MAC.
//
pub const MAX_FRAME_SIZE: c_int = 1520;
// GRETH APB registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct greth_regs {
    pub control: u32,
    pub status: u32,
    pub esa_msb: u32,
    pub esa_lsb: u32,
    pub mdio: u32,
    pub tx_desc_p: u32,
    pub rx_desc_p: u32,
    pub edclip: u32,
    pub hash_msb: u32,
    pub hash_lsb: u32,
}

// GRETH buffer descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct greth_bd {
    pub stat: u32,
    pub addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct greth_private {
    pub rx_skbuff: [*mut sk_buff; GRETH_RXBD_NUM],
    pub tx_skbuff: [*mut sk_buff; GRETH_TXBD_NUM],
    pub tx_bufs: [*mut c_uchar; GRETH_TXBD_NUM],
    pub rx_bufs: [*mut c_uchar; GRETH_RXBD_NUM],
    pub tx_bufs_length: [u16; GRETH_TXBD_NUM],
    pub tx_next: u16,
    pub tx_last: u16,
    pub /: *mut *mut u16 tx_free; / only used on 10/100Mbit,
    pub rx_cur: u16,
    pub /: *mut *mut *mut greth_regs regs; / Address of controller registers.,
    pub /: *mut *mut *mut greth_bd rx_bd_base; / Address of Rx BDs.,
    pub /: *mut *mut *mut greth_bd tx_bd_base; / Address of Tx BDs.,
    pub rx_bd_base_phys: dma_addr_t,
    pub tx_bd_base_phys: dma_addr_t,
    pub irq: c_int,
    pub /: *mut *mut *mut device dev; / Pointer to platform_device->dev,
    pub netdev: *mut net_device,
    pub napi: napi_struct,
    pub devlock: spinlock_t,
    pub mdio: *mut mii_bus,
    pub link: c_uint,
    pub speed: c_uint,
    pub duplex: c_uint,
    pub msg_enable: u32,
    pub phyaddr: u8,
    pub multicast: u8,
    pub gbit_mac: u8,
    pub mdio_int_en: u8,
    pub edcl: u8,
}
