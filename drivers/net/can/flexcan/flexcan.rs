//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/flexcan/flexcan.h
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
// flexcan.c - FLEXCAN CAN controller driver
//
// Copyright (c) 2005-2006 Varma Electronics Oy
// Copyright (c) 2009 Sascha Hauer, Pengutronix
// Copyright (c) 2010-2017 Pengutronix, Marc Kleine-Budde <kernel@pengutronix.de>
// Copyright (c) 2014 David Jander, Protonic Holland
// Copyright (C) 2022 Amarula Solutions, Dario Binacchi <dario.binacchi@amarulasolutions.com>
//
// Based on code originally by Andrey Volkov <avolkov@varma-el.com>
//

// FLEXCAN hardware feature flags
//
// Below is some version info we got:
// SOC   Version   IP-Version  Glitch- [TR]WRN_INT IRQ Err Memory err RTR rece-   FD Mode     MB
// Filter? connected?  Passive detection  ption in MB Supported?
// MCF5441X FlexCAN2  ?               no       yes        no       no        no           no     16
// MX25  FlexCAN2  03.00.00.00     no        no        no       no        no           no     64
// MX28  FlexCAN2  03.00.04.00    yes       yes        no       no        no           no     64
// MX35  FlexCAN2  03.00.00.00     no        no        no       no        no           no     64
// MX53  FlexCAN2  03.00.00.00    yes        no        no       no        no           no     64
// MX6s  FlexCAN3  10.00.12.00    yes       yes        no       no       yes           no     64
// MX8QM FlexCAN3  03.00.23.00    yes       yes        no       no       yes          yes     64
// MX8MP FlexCAN3  03.00.17.01    yes       yes        no      yes       yes          yes     64
// VF610 FlexCAN3  ?               no       yes        no      yes       yes?          no     64
// LS1021A FlexCAN2  03.00.04.00     no       yes        no       no       yes           no     64
// LX2160A FlexCAN3  03.00.23.00     no       yes        no      yes       yes          yes     64
//
// Some SOCs do not have the RX_WARN & TX_WARN interrupt line connected.
//
// [TR]WRN_INT not connected

// Disable RX FIFO Global mask

// Enable EACEN and RRS bit in ctrl2

// Disable non-correctable errors interrupt and freeze mode

// Use mailboxes (not FIFO) for RX path

// No interrupt for error passive

// default to BE register access

// Setup stop mode with GPR to support wakeup

// Support CAN-FD mode

// support memory detection and correction

// Setup stop mode with SCU firmware to support wakeup

// Setup 3 separate interrupts, main, boff and err

// Setup 16 mailboxes

// Device supports RX via mailboxes

// Device supports RTR reception via mailboxes

// Device supports RX via FIFO

// Setup stop mode with ATF SCMI protocol to support wakeup

// Device has two separate interrupt lines for two mailbox ranges, which
// both need to have an interrupt handler registered.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flexcan_devtype_data {
    pub /: *mut *mut u32 quirks; / quirks needed for different IP cores,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flexcan_stop_mode {
    pub gpr: *mut regmap,
    pub req_gpr: u8,
    pub req_bit: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flexcan_priv {
    pub can: can_priv,
    pub offload: can_rx_offload,
    pub dev: *mut device,
    pub regs: *mut flexcan_regs __iomem,
    pub tx_mb: *mut flexcan_mb __iomem,
    pub tx_mb_reserved: *mut flexcan_mb __iomem,
    pub tx_mb_idx: u8,
    pub mb_count: u8,
    pub mb_size: u8,
    pub /: *mut *mut u8 clk_src; / clock source of CAN Protocol Engine,
    pub scu_idx: u8,
    pub rx_mask: u64,
    pub tx_mask: u64,
    pub reg_ctrl_default: u32,
    pub clk_ipg: *mut clk,
    pub clk_per: *mut clk,
    pub devtype_data: flexcan_devtype_data,
    pub reg_xceiver: *mut regulator,
    pub transceiver: *mut phy,
    pub stm: flexcan_stop_mode,
    pub irq_boff: c_int,
    pub irq_err: c_int,
    pub irq_secondary_mb: c_int,
// IPC handle when setup stop mode by System Controller firmware(scfw)
    pub sc_ipc_handle: *mut imx_sc_ipc,
// Read and Write APIs
    pub addr): *mut *mut u32 (read)(void __iomem,
    pub addr): *mut *mut void (write)(u32 val, void __iomem,
}

// RX-FIFO is always RTR capable
