//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/arc/emac.h
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
// Copyright (C) 2004-2013 Synopsys, Inc. (www.synopsys.com)
//
// Registers and bits definitions of ARC EMAC
//

// STATUS and ENABLE Register bit masks

// CONTROL Register bit masks

// Buffer descriptor INFO bit masks

pub const LEN_MASK: c_uint = 0x000007FF	/* last 11 bits */;

pub const FOR_CPU: c_int = 0;
// ARC EMAC register set combines entries for MAC and MDIO

//
// struct arc_emac_bd - EMAC buffer descriptor (BD).
//
// @info:	Contains status information on the buffer itself.
// @data:	32-bit byte addressable pointer to the packet data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_emac_bd {
    pub info: __le32,
    pub data: dma_addr_t,
}

// Number of Rx/Tx BD's
pub const RX_BD_NUM: c_int = 128;
pub const TX_BD_NUM: c_int = 128;

//
// struct buffer_state - Stores Rx/Tx buffer state.
// @sk_buff:	Pointer to socket buffer.
// @addr:	Start address of DMA-mapped memory region.
// @len:	Length of DMA-mapped memory region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer_state {
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_emac_mdio_bus_data {
    pub reset_gpio: *mut gpio_desc,
    pub msec: c_int,
}

//
// struct arc_emac_priv - Storage of EMAC's private information.
// @dev:	Pointer to the current device.
// @phy_dev:	Pointer to attached PHY device.
// @bus:	Pointer to the current MII bus.
// @regs:	Base address of EMAC memory-mapped control registers.
// @napi:	Structure for NAPI.
// @rxbd:	Pointer to Rx BD ring.
// @txbd:	Pointer to Tx BD ring.
// @rxbd_dma:	DMA handle for Rx BD ring.
// @txbd_dma:	DMA handle for Tx BD ring.
// @rx_buff:	Storage for Rx buffers states.
// @tx_buff:	Storage for Tx buffers states.
// @txbd_curr:	Index of Tx BD to use on the next "ndo_start_xmit".
// @txbd_dirty:	Index of Tx BD to free on the next Tx interrupt.
// @last_rx_bd:	Index of the last Rx BD we've got from EMAC.
// @link:	PHY's last seen link state.
// @duplex:	PHY's last set duplex mode.
// @speed:	PHY's last set speed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arc_emac_priv {
    pub drv_name: *const c_char,
    pub speed): *mut *mut *mut void (set_mac_speed)(void priv, unsigned int,
// Devices
    pub dev: *mut device,
    pub bus: *mut mii_bus,
    pub bus_data: arc_emac_mdio_bus_data,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub napi: napi_struct,
    pub rxbd: *mut arc_emac_bd,
    pub txbd: *mut arc_emac_bd,
    pub rxbd_dma: dma_addr_t,
    pub txbd_dma: dma_addr_t,
    pub rx_buff: [buffer_state; RX_BD_NUM],
    pub tx_buff: [buffer_state; TX_BD_NUM],
    pub txbd_curr: c_uint,
    pub txbd_dirty: c_uint,
    pub last_rx_bd: c_uint,
    pub link: c_uint,
    pub duplex: c_uint,
    pub speed: c_uint,
    pub rx_missed_errors: c_uint,
}

//
// arc_reg_set - Sets EMAC register with provided value.
// @priv:	Pointer to ARC EMAC private data structure.
// @reg:	Register offset from base address.
// @value:	Value to set in register.
//
// arc_reg_get - Gets value of specified EMAC register.
// @priv:	Pointer to ARC EMAC private data structure.
// @reg:	Register offset from base address.
//
// returns:	Value of requested register.
//
extern "C" {
    pub fn ioread32(sizeof(int): *mut *mut priv->regs + reg) -> return;
}
//
// arc_reg_or - Applies mask to specified EMAC register - ("reg" | "mask").
// @priv:	Pointer to ARC EMAC private data structure.
// @reg:	Register offset from base address.
// @mask:	Mask to apply to specified register.
//
// This function reads initial register value, then applies provided mask
// to it and then writes register back.
//
// arc_reg_clr - Applies mask to specified EMAC register - ("reg" & ~"mask").
// @priv:	Pointer to ARC EMAC private data structure.
// @reg:	Register offset from base address.
// @mask:	Mask to apply to specified register.
//
// This function reads initial register value, then applies provided mask
// to it and then writes register back.
//
extern "C" {
    pub fn arc_mdio_probe(priv: *mut arc_emac_priv) -> c_int;
}
extern "C" {
    pub fn arc_mdio_remove(priv: *mut arc_emac_priv) -> c_int;
}
extern "C" {
    pub fn arc_emac_probe(ndev: *mut net_device, interface: c_int) -> c_int;
}
extern "C" {
    pub fn arc_emac_remove(ndev: *mut net_device);
}
