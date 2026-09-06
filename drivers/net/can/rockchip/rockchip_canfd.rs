//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/rockchip/rockchip_canfd.h
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
// Copyright (c) 2023, 2024 Pengutronix,
// Marc Kleine-Budde <kernel@pengutronix.de>
//

pub const RKCANFD_REG_MODE: c_uint = 0x000;

pub const RKCANFD_REG_CMD: c_uint = 0x004;

pub const RKCANFD_REG_STATE: c_uint = 0x008;

pub const RKCANFD_REG_INT: c_uint = 0x00c;

pub const RKCANFD_REG_INT_MASK: c_uint = 0x010;
pub const RKCANFD_REG_DMA_CTL: c_uint = 0x014;

pub const RKCANFD_REG_BITTIMING: c_uint = 0x018;

pub const RKCANFD_REG_ARBITFAIL: c_uint = 0x028;

// Register seems to be clear or read
pub const RKCANFD_REG_ERROR_CODE: c_uint = 0x02c;

pub const RKCANFD_REG_ERROR_CODE_TYPE_BIT: c_uint = 0x0;
pub const RKCANFD_REG_ERROR_CODE_TYPE_STUFF: c_uint = 0x1;
pub const RKCANFD_REG_ERROR_CODE_TYPE_FORM: c_uint = 0x2;
pub const RKCANFD_REG_ERROR_CODE_TYPE_ACK: c_uint = 0x3;
pub const RKCANFD_REG_ERROR_CODE_TYPE_CRC: c_uint = 0x4;

pub const RKCANFD_REG_RXERRORCNT: c_uint = 0x034;

pub const RKCANFD_REG_TXERRORCNT: c_uint = 0x038;

pub const RKCANFD_REG_IDCODE: c_uint = 0x03c;

pub const RKCANFD_REG_IDMASK: c_uint = 0x040;
pub const RKCANFD_REG_TXFRAMEINFO: c_uint = 0x050;

pub const RKCANFD_REG_TXID: c_uint = 0x054;

pub const RKCANFD_REG_TXDATA0: c_uint = 0x058;
pub const RKCANFD_REG_TXDATA1: c_uint = 0x05C;
pub const RKCANFD_REG_RXFRAMEINFO: c_uint = 0x060;
pub const RKCANFD_REG_RXID: c_uint = 0x064;
pub const RKCANFD_REG_RXDATA0: c_uint = 0x068;
pub const RKCANFD_REG_RXDATA1: c_uint = 0x06c;
pub const RKCANFD_REG_RTL_VERSION: c_uint = 0x070;

pub const RKCANFD_REG_FD_NOMINAL_BITTIMING: c_uint = 0x100;

pub const RKCANFD_REG_FD_DATA_BITTIMING: c_uint = 0x104;

pub const RKCANFD_REG_TRANSMIT_DELAY_COMPENSATION: c_uint = 0x108;

pub const RKCANFD_REG_TIMESTAMP_CTRL: c_uint = 0x10c;
// datasheet says 6:1, which is wrong

pub const RKCANFD_REG_TIMESTAMP: c_uint = 0x110;
pub const RKCANFD_REG_TXEVENT_FIFO_CTRL: c_uint = 0x114;

pub const RKCANFD_REG_RX_FIFO_CTRL: c_uint = 0x118;

pub const RKCANFD_REG_AFC_CTRL: c_uint = 0x11c;

pub const RKCANFD_REG_IDCODE0: c_uint = 0x120;
pub const RKCANFD_REG_IDMASK0: c_uint = 0x124;
pub const RKCANFD_REG_IDCODE1: c_uint = 0x128;
pub const RKCANFD_REG_IDMASK1: c_uint = 0x12c;
pub const RKCANFD_REG_IDCODE2: c_uint = 0x130;
pub const RKCANFD_REG_IDMASK2: c_uint = 0x134;
pub const RKCANFD_REG_IDCODE3: c_uint = 0x138;
pub const RKCANFD_REG_IDMASK3: c_uint = 0x13c;
pub const RKCANFD_REG_IDCODE4: c_uint = 0x140;
pub const RKCANFD_REG_IDMASK4: c_uint = 0x144;
pub const RKCANFD_REG_FD_TXFRAMEINFO: c_uint = 0x200;

pub const RKCANFD_REG_FD_TXID: c_uint = 0x204;

pub const RKCANFD_REG_FD_TXDATA0: c_uint = 0x208;
pub const RKCANFD_REG_FD_TXDATA1: c_uint = 0x20c;
pub const RKCANFD_REG_FD_TXDATA2: c_uint = 0x210;
pub const RKCANFD_REG_FD_TXDATA3: c_uint = 0x214;
pub const RKCANFD_REG_FD_TXDATA4: c_uint = 0x218;
pub const RKCANFD_REG_FD_TXDATA5: c_uint = 0x21c;
pub const RKCANFD_REG_FD_TXDATA6: c_uint = 0x220;
pub const RKCANFD_REG_FD_TXDATA7: c_uint = 0x224;
pub const RKCANFD_REG_FD_TXDATA8: c_uint = 0x228;
pub const RKCANFD_REG_FD_TXDATA9: c_uint = 0x22c;
pub const RKCANFD_REG_FD_TXDATA10: c_uint = 0x230;
pub const RKCANFD_REG_FD_TXDATA11: c_uint = 0x234;
pub const RKCANFD_REG_FD_TXDATA12: c_uint = 0x238;
pub const RKCANFD_REG_FD_TXDATA13: c_uint = 0x23c;
pub const RKCANFD_REG_FD_TXDATA14: c_uint = 0x240;
pub const RKCANFD_REG_FD_TXDATA15: c_uint = 0x244;
pub const RKCANFD_REG_FD_RXFRAMEINFO: c_uint = 0x300;
pub const RKCANFD_REG_FD_RXID: c_uint = 0x304;
pub const RKCANFD_REG_FD_RXTIMESTAMP: c_uint = 0x308;
pub const RKCANFD_REG_FD_RXDATA0: c_uint = 0x30c;
pub const RKCANFD_REG_FD_RXDATA1: c_uint = 0x310;
pub const RKCANFD_REG_FD_RXDATA2: c_uint = 0x314;
pub const RKCANFD_REG_FD_RXDATA3: c_uint = 0x318;
pub const RKCANFD_REG_FD_RXDATA4: c_uint = 0x31c;
pub const RKCANFD_REG_FD_RXDATA5: c_uint = 0x320;
pub const RKCANFD_REG_FD_RXDATA6: c_uint = 0x320;
pub const RKCANFD_REG_FD_RXDATA7: c_uint = 0x328;
pub const RKCANFD_REG_FD_RXDATA8: c_uint = 0x32c;
pub const RKCANFD_REG_FD_RXDATA9: c_uint = 0x330;
pub const RKCANFD_REG_FD_RXDATA10: c_uint = 0x334;
pub const RKCANFD_REG_FD_RXDATA11: c_uint = 0x338;
pub const RKCANFD_REG_FD_RXDATA12: c_uint = 0x33c;
pub const RKCANFD_REG_FD_RXDATA13: c_uint = 0x340;
pub const RKCANFD_REG_FD_RXDATA14: c_uint = 0x344;
pub const RKCANFD_REG_FD_RXDATA15: c_uint = 0x348;
pub const RKCANFD_REG_RX_FIFO_RDATA: c_uint = 0x400;
pub const RKCANFD_REG_TXE_FIFO_RDATA: c_uint = 0x500;

pub const RKCANFD_NAPI_WEIGHT: c_int = 32;
pub const RKCANFD_TXFIFO_DEPTH: c_int = 2;
pub const RKCANFD_TX_STOP_THRESHOLD: c_int = 1;
pub const RKCANFD_TX_START_THRESHOLD: c_int = 1;
pub const RKCANFD_TIMESTAMP_WORK_MAX_DELAY_SEC: c_int = 60;

// rk3568 CAN-FD Errata, as of Tue 07 Nov 2023 11:25:31 +08:00
// Erratum 1: The error frame sent by the CAN controller has an
// abnormal format.
//

// Erratum 2: The error frame sent after detecting a CRC error has an
// abnormal position.
//

// Erratum 3: Intermittent CRC calculation errors.

// Erratum 4: Intermittent occurrence of stuffing errors.

// Erratum 5: Counters related to the TXFIFO and RXFIFO exhibit
// abnormal counting behavior.
//
// The rk3568 CAN-FD errata sheet as of Tue 07 Nov 2023 11:25:31 +08:00
// states that only the rk3568v2 is affected by this erratum, but
// tests with the rk3568v2 and rk3568v3 show that the RX_FIFO_CNT is
// sometimes too high. This leads to CAN frames being read from the
// FIFO, which is then already empty.
//
// Further tests on the rk3568v2 and rk3568v3 show that in this
// situation (i.e. empty FIFO) all elements of the FIFO header
// (frameinfo, id, ts) contain the same data.
//
// On the rk3568v2 and rk3568v3, this problem only occurs extremely
// rarely with the standard clock of 300 MHz, but almost immediately
// at 80 MHz.
//
// Tests on the rk3588 show the same empty FIFO condition.
// In that setup rx_fifo_empty_errors increments when the bus
// transitions from idle to high CAN-FD load and stops growing once
// the bus reaches a steady state.
//
// To workaround this problem, check for empty FIFO with
// rkcanfd_fifo_header_empty() in rkcanfd_handle_rx_int_one() and exit
// early.
//
// To reproduce:
// assigned-clocks = <&cru CLK_CANx>;
// assigned-clock-rates = <80000000>;
//

// Erratum 6: The CAN controller's transmission of extended frames may
// intermittently change into standard frames
//
// Tests on the rk3588 show the same problem.
//
// Work around this issue by activating self reception (RXSTX). If we
// have pending TX CAN frames, check all RX'ed CAN frames in
// rkcanfd_rxstx_filter().
//
// If it's a frame we've send and it's OK, call the TX complete
// handler: rkcanfd_handle_tx_done_one(). Mask the TX complete IRQ.
//
// If it's a frame we've send, but the CAN-ID is mangled, resend the
// original extended frame.
//
// To reproduce:
// host:
// canfdtest -evx -g can0
// candump any,0:80000000 -cexdtA
// dut:
// canfdtest -evx can0
// ethtool -S can0
//

// Erratum 7: In the passive error state, the CAN controller's
// interframe space segment counting is inaccurate.
//

// Erratum 8: The Format-Error error flag is transmitted one bit
// later.
//

// Erratum 9: In the arbitration segment, the CAN controller will
// identify stuffing errors as arbitration failures.
//

// Erratum 10: Does not support the BUSOFF slow recovery mechanism.

// Erratum 11: Arbitration error.

// Erratum 12: A dominant bit at the third bit of the intermission may
// cause a transmission error.
//

// Tests on the rk3568v2 and rk3568v3 show that receiving certain
// CAN-FD frames trigger an Error Interrupt.
//
// - Form Error in RX Arbitration Phase: TX_IDLE RX_STUFF_COUNT (0x0a010100) CMD=0 RX=0 TX=0
// Error-Warning=1 Bus-Off=0
// To reproduce:
// host:
// cansend can0 002##01f
// DUT:
// candump any,0:0,#FFFFFFFF -cexdHtA
//
// - Form Error in RX Arbitration Phase: TX_IDLE RX_CRC (0x0a010200) CMD=0 RX=0 TX=0
// Error-Warning=1 Bus-Off=0
// To reproduce:
// host:
// cansend can0 002##07217010000000000
// DUT:
// candump any,0:0,#FFFFFFFF -cexdHtA
//

// known issues with rk3568v3:
//
// - Overload situation during high bus load
// To reproduce:
// host:
// # add a 2nd CAN adapter to the CAN bus
// cangen can0 -I 1 -Li -Di -p10 -g 0.3
// cansequence -rve
// DUT:
// cangen can0 -I2 -L1 -Di -p10 -c10 -g 1 -e
// cansequence -rv -i 1
//
// - TX starvation after repeated Bus-Off
// Tests on the rk3588 show the same problem. In a
// 10-cycle Bus-Off recovery test, 9 cycles failed to send after the
// controller restarted.
// To reproduce:
// host:
// sleep 3 && cangen can0 -I2 -Li -Di -p10 -g 0.0
// DUT:
// cangen can0 -I2 -Li -Di -p10 -g 0.05
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcanfd_model {
    RKCANFD_MODEL_RK3568V2 = 0x35682,
    RKCANFD_MODEL_RK3568V3 = 0x35683,
    RKCANFD_MODEL_RK3588 = 0x3588,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcanfd_devtype_data {
    pub model: rkcanfd_model,
    pub quirks: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcanfd_fifo_header {
    pub frameinfo: u32,
    pub id: u32,
    pub ts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcanfd_stats {
    pub syncp: u64_stats_sync,
// Erratum 5
    pub rx_fifo_empty_errors: u64_stats_t,
// Erratum 6
    pub tx_extended_as_standard_errors: u64_stats_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcanfd_priv {
    pub can: can_priv,
    pub offload: can_rx_offload,
    pub ndev: *mut net_device,
    pub regs: *mut void __iomem,
    pub tx_head: c_uint,
    pub tx_tail: c_uint,
    pub reg_mode_default: u32,
    pub reg_int_mask_default: u32,
    pub devtype_data: rkcanfd_devtype_data,
    pub cc: cyclecounter,
    pub tc: timecounter,
    pub timestamp: delayed_work,
    pub work_delay_jiffies: c_ulong,
    pub bec: can_berr_counter,
    pub stats: rkcanfd_stats,
    pub reset: *mut reset_control,
    pub clks: *mut clk_bulk_data,
    pub clks_num: c_int,
}

extern "C" {
    pub fn readl(reg: priv->regs +) -> return;
}
extern "C" {
    pub fn rkcanfd_read(_arg: priv, _arg: RKCANFD_REG_TIMESTAMP) -> return;
}
extern "C" {
    pub fn READ_ONCE(1: priv->tx_head) & (RKCANFD_TXFIFO_DEPTH -) -> return;
}
extern "C" {
    pub fn READ_ONCE(1: priv->tx_tail) & (RKCANFD_TXFIFO_DEPTH -) -> return;
}
extern "C" {
    pub fn READ_ONCE(READ_ONCE(priv->tx_tail: priv->tx_head) -) -> return;
}
extern "C" {
    pub fn rkcanfd_ethtool_init(priv: *mut rkcanfd_priv);
}
extern "C" {
    pub fn rkcanfd_handle_rx_int(priv: *mut rkcanfd_priv) -> c_int;
}
extern "C" {
    pub fn rkcanfd_timestamp_init(priv: *mut rkcanfd_priv);
}
extern "C" {
    pub fn rkcanfd_timestamp_start(priv: *mut rkcanfd_priv);
}
extern "C" {
    pub fn rkcanfd_timestamp_stop(priv: *mut rkcanfd_priv);
}
extern "C" {
    pub fn rkcanfd_timestamp_stop_sync(priv: *mut rkcanfd_priv);
}
extern "C" {
    pub fn rkcanfd_get_effective_tx_free(priv: *const rkcanfd_priv) -> c_uint;
}
extern "C" {
    pub fn rkcanfd_xmit_retry(priv: *mut rkcanfd_priv);
}
extern "C" {
    pub fn rkcanfd_start_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t;
}
