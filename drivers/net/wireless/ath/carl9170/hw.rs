//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/carl9170/hw.h
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


//
// Shared Atheros AR9170 Header
//
// Register map, hardware-specific definitions
//
// Copyright 2008, Johannes Berg <johannes@sipsolutions.net>
// Copyright 2009-2011 Christian Lamparter <chunkeey@googlemail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; see the file COPYING.  If not, see
// http://www.gnu.org/licenses/.
//
// This file incorporates work covered by the following copyright and
// permission notice:
// Copyright (c) 2007-2008 Atheros Communications, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// High Speed UART
pub const AR9170_UART_REG_BASE: c_uint = 0x1c0000;
// Definitions of interrupt registers

pub const AR9170_UART_FIFO_CTRL_RESET_RX_FIFO: c_uint = 0x02;
pub const AR9170_UART_FIFO_CTRL_RESET_TX_FIFO: c_uint = 0x04;

pub const AR9170_UART_MODEM_CTRL_DTR_BIT: c_uint = 0x01;
pub const AR9170_UART_MODEM_CTRL_RTS_BIT: c_uint = 0x02;
pub const AR9170_UART_MODEM_CTRL_INTERNAL_LOOP_BACK: c_uint = 0x10;
pub const AR9170_UART_MODEM_CTRL_AUTO_RTS: c_uint = 0x20;
pub const AR9170_UART_MODEM_CTRL_AUTO_CTR: c_uint = 0x40;

pub const AR9170_UART_LINE_STS_RX_DATA_READY: c_uint = 0x01;
pub const AR9170_UART_LINE_STS_RX_BUFFER_OVERRUN: c_uint = 0x02;
pub const AR9170_UART_LINE_STS_RX_BREAK_IND: c_uint = 0x10;
pub const AR9170_UART_LINE_STS_TX_FIFO_NEAR_EMPTY: c_uint = 0x20;
pub const AR9170_UART_LINE_STS_TRANSMITTER_EMPTY: c_uint = 0x40;

pub const AR9170_UART_MODEM_STS_CTS_CHANGE: c_uint = 0x01;
pub const AR9170_UART_MODEM_STS_DSR_CHANGE: c_uint = 0x02;
pub const AR9170_UART_MODEM_STS_DCD_CHANGE: c_uint = 0x08;
pub const AR9170_UART_MODEM_STS_CTS_COMPL: c_uint = 0x10;
pub const AR9170_UART_MODEM_STS_DSR_COMPL: c_uint = 0x20;
pub const AR9170_UART_MODEM_STS_DCD_COMPL: c_uint = 0x80;

// Timer
pub const AR9170_TIMER_REG_BASE: c_uint = 0x1c1000;

pub const AR9170_TIMER_CTRL_DISABLE_CLOCK: c_uint = 0x100;

pub const AR9170_TIMER_INT_TIMER0: c_uint = 0x001;
pub const AR9170_TIMER_INT_TIMER1: c_uint = 0x002;
pub const AR9170_TIMER_INT_TIMER2: c_uint = 0x004;
pub const AR9170_TIMER_INT_TIMER3: c_uint = 0x008;
pub const AR9170_TIMER_INT_TIMER4: c_uint = 0x010;
pub const AR9170_TIMER_INT_TICK_TIMER: c_uint = 0x100;

pub const AR9170_MAC_REG_BASE: c_uint = 0x1c3000;

pub const AR9170_MAC_POWER_STATE_CTRL_RESET: c_uint = 0x20;

pub const AR9170_MAC_ATIM_PERIOD_S: c_int = 0;
pub const AR9170_MAC_ATIM_PERIOD: c_uint = 0x0000ffff;

pub const AR9170_MAC_BCN_PERIOD_S: c_int = 0;
pub const AR9170_MAC_BCN_PERIOD: c_uint = 0x0000ffff;
pub const AR9170_MAC_BCN_DTIM_S: c_int = 16;
pub const AR9170_MAC_BCN_DTIM: c_uint = 0x00ff0000;

pub const AR9170_MAC_PRETBTT_S: c_int = 0;
pub const AR9170_MAC_PRETBTT: c_uint = 0x0000ffff;
pub const AR9170_MAC_PRETBTT2_S: c_int = 16;
pub const AR9170_MAC_PRETBTT2: c_uint = 0xffff0000;

pub const AR9170_MAC_SNIFFER_DEFAULTS: c_uint = 0x02000000;

pub const AR9170_MAC_ENCRYPTION_DEFAULTS: c_uint = 0x70;

pub const AR9170_MAC_FTF_DEFAULTS: c_uint = 0x0500ffff;
pub const AR9170_MAC_FTF_MONITOR: c_uint = 0xff00ffff;

pub const AR9170_MAC_CAM_IBSS: c_uint = 0xe0;
pub const AR9170_MAC_CAM_AP: c_uint = 0xa1;
pub const AR9170_MAC_CAM_STA: c_uint = 0x2;
pub const AR9170_MAC_CAM_AP_WDS: c_uint = 0x3;

pub const AR9170_MAC_CAM_HOST_PENDING: c_uint = 0x80000000;

pub const AR9170_MAC_CAM_ADDR_WRITE: c_uint = 0x80000000;

pub const AR9170_MAC_CAM_STATE_READ_PENDING: c_uint = 0x40000000;
pub const AR9170_MAC_CAM_STATE_WRITE_PENDING: c_uint = 0x80000000;

pub const AR9170_MAC_AMPDU_FACTOR: c_uint = 0x7f0000;
pub const AR9170_MAC_AMPDU_FACTOR_S: c_int = 16;

pub const AR9170_MAC_AMPDU_DENSITY: c_uint = 0x7;
pub const AR9170_MAC_AMPDU_DENSITY_S: c_int = 0;

pub const AR9170_MAC_FCS_SWFCS: c_uint = 0x1;
pub const AR9170_MAC_FCS_FIFO_PROT: c_uint = 0x4;

pub const AR9170_MAC_RX_CTRL_DEAGG: c_uint = 0x1;
pub const AR9170_MAC_RX_CTRL_SHORT_FILTER: c_uint = 0x2;
pub const AR9170_MAC_RX_CTRL_SA_DA_SEARCH: c_uint = 0x20;

pub const AR9170_MAC_TXRX_MPI_TX_MPI_MASK: c_uint = 0x0000000f;
pub const AR9170_MAC_TXRX_MPI_TX_TO_MASK: c_uint = 0x0000fff0;
pub const AR9170_MAC_TXRX_MPI_RX_MPI_MASK: c_uint = 0x000f0000;
pub const AR9170_MAC_TXRX_MPI_RX_TO_MASK: c_uint = 0xfff00000;

pub const AR9170_BCN_CTRL_READY: c_uint = 0x01;
pub const AR9170_BCN_CTRL_LOCK: c_uint = 0x02;

pub const AR9170_MAC_BCN_HT1_PWR_CTRL_S: c_int = 4;
pub const AR9170_MAC_BCN_HT1_PWR_CTRL: c_uint = 0x70;

pub const AR9170_MAC_BCN_HT1_NUM_LFT_S: c_int = 9;
pub const AR9170_MAC_BCN_HT1_NUM_LFT: c_uint = 0x600;

pub const AR9170_MAC_BCN_HT1_BF_MCS_S: c_int = 18;
pub const AR9170_MAC_BCN_HT1_BF_MCS: c_uint = 0x1c0000;
pub const AR9170_MAC_BCN_HT1_TPC_S: c_int = 21;
pub const AR9170_MAC_BCN_HT1_TPC: c_uint = 0x7e00000;
pub const AR9170_MAC_BCN_HT1_CHAIN_MASK_S: c_int = 27;
pub const AR9170_MAC_BCN_HT1_CHAIN_MASK: c_uint = 0x38000000;

pub const AR9170_MAC_BCN_HT2_MCS_S: c_int = 0;
pub const AR9170_MAC_BCN_HT2_MCS: c_uint = 0x7f;

pub const AR9170_MAC_BCN_HT2_STBC_S: c_int = 12;
pub const AR9170_MAC_BCN_HT2_STBC: c_uint = 0x3000;

pub const AR9170_MAC_BCN_HT2_LEN_S: c_int = 16;
pub const AR9170_MAC_BCN_HT2_LEN: c_uint = 0xffff0000;

// Random number generator
pub const AR9170_RAND_REG_BASE: c_uint = 0x1d0000;

pub const AR9170_RAND_MODE_MANUAL: c_uint = 0x000;
pub const AR9170_RAND_MODE_FREE: c_uint = 0x001;
// GPIO
pub const AR9170_GPIO_REG_BASE: c_uint = 0x1d0100;

pub const AR9170_GPIO_PORT_LED_0: c_int = 1;
pub const AR9170_GPIO_PORT_LED_1: c_int = 2;
// WPS Button GPIO for TP-Link TL-WN821N
pub const AR9170_GPIO_PORT_WPS_BUTTON_PRESSED: c_int = 4;
// Memory Controller
pub const AR9170_MC_REG_BASE: c_uint = 0x1d1000;

pub const AR9170_SPI_CONTROL0_CMD_LEN_S: c_int = 8;
pub const AR9170_SPI_CONTROL0_CMD_LEN: c_uint = 0x00000f00;
pub const AR9170_SPI_CONTROL0_RD_LEN_S: c_int = 12;
pub const AR9170_SPI_CONTROL0_RD_LEN: c_uint = 0x00007000;

pub const AR9170_SPI_CONTROL1_MODE_SEL_S: c_int = 2;
pub const AR9170_SPI_CONTROL1_MODE_SEL: c_uint = 0x000000c0;

pub const AR9170_SPI_COMMAND_PORT0_CMD0_S: c_int = 0;
pub const AR9170_SPI_COMMAND_PORT0_CMD0: c_uint = 0x000000ff;
pub const AR9170_SPI_COMMAND_PORT0_CMD1_S: c_int = 8;
pub const AR9170_SPI_COMMAND_PORT0_CMD1: c_uint = 0x0000ff00;
pub const AR9170_SPI_COMMAND_PORT0_CMD2_S: c_int = 16;
pub const AR9170_SPI_COMMAND_PORT0_CMD2: c_uint = 0x00ff0000;
pub const AR9170_SPI_COMMAND_PORT0_CMD3_S: c_int = 24;
pub const AR9170_SPI_COMMAND_PORT0_CMD3: c_uint = 0xff000000;

pub const AR9170_SPI_COMMAND_PORT1_CMD4_S: c_int = 0;
pub const AR9170_SPI_COMMAND_PORT1_CMD4: c_uint = 0x000000ff;
pub const AR9170_SPI_COMMAND_PORT1_CMD5_S: c_int = 8;
pub const AR9170_SPI_COMMAND_PORT1_CMD5: c_uint = 0x0000ff00;
pub const AR9170_SPI_COMMAND_PORT1_CMD6_S: c_int = 16;
pub const AR9170_SPI_COMMAND_PORT1_CMD6: c_uint = 0x00ff0000;
pub const AR9170_SPI_COMMAND_PORT1_CMD7_S: c_int = 24;
pub const AR9170_SPI_COMMAND_PORT1_CMD7: c_uint = 0xff000000;

pub const AR9170_EEPROM_WP_MAGIC1: c_uint = 0x12345678;

pub const AR9170_EEPROM_WP_MAGIC2: c_uint = 0x55aa00ff;

pub const AR9170_EEPROM_WP_MAGIC3: c_uint = 0x13579ace;

pub const AR9170_EEPROM_CLOCK_DIV_FAC_S: c_int = 0;
pub const AR9170_EEPROM_CLOCK_DIV_FAC: c_uint = 0x000001ff;
pub const AR9170_EEPROM_CLOCK_DIV_FAC_39KHZ: c_uint = 0xff;
pub const AR9170_EEPROM_CLOCK_DIV_FAC_78KHZ: c_uint = 0x7f;
pub const AR9170_EEPROM_CLOCK_DIV_FAC_312KHZ: c_uint = 0x1f;
pub const AR9170_EEPROM_CLOCK_DIV_FAC_10MHZ: c_uint = 0x0;

// Interrupt Controller
pub const AR9170_MAX_INT_SRC: c_int = 9;
pub const AR9170_INT_REG_BASE: c_uint = 0x1d2000;

// INT_REG_FLAG, INT_REG_FIQ_MASK and INT_REG_IRQ_MASK
pub const AR9170_INT_FLAG_WLAN: c_uint = 0x001;
pub const AR9170_INT_FLAG_PTAB_BIT: c_uint = 0x002;
pub const AR9170_INT_FLAG_SE_BIT: c_uint = 0x004;
pub const AR9170_INT_FLAG_UART_BIT: c_uint = 0x008;
pub const AR9170_INT_FLAG_TIMER_BIT: c_uint = 0x010;
pub const AR9170_INT_FLAG_EXT_BIT: c_uint = 0x020;
pub const AR9170_INT_FLAG_SW_BIT: c_uint = 0x040;
pub const AR9170_INT_FLAG_USB_BIT: c_uint = 0x080;
pub const AR9170_INT_FLAG_ETHERNET_BIT: c_uint = 0x100;

pub const AR9170_INT_SW_INT_ENABLE: c_uint = 0x1;

// Power Management
pub const AR9170_PWR_REG_BASE: c_uint = 0x1d4000;

pub const AR9170_PWR_CLK_AHB_40MHZ: c_int = 0;
pub const AR9170_PWR_CLK_AHB_20_22MHZ: c_int = 1;
pub const AR9170_PWR_CLK_AHB_40_44MHZ: c_int = 2;
pub const AR9170_PWR_CLK_AHB_80_88MHZ: c_int = 3;
pub const AR9170_PWR_CLK_DAC_160_INV_DLY: c_uint = 0x70;

pub const AR9170_PWR_PLL_ADDAC_DIV_S: c_int = 2;
pub const AR9170_PWR_PLL_ADDAC_DIV: c_uint = 0xffc;

// Faraday USB Controller
pub const AR9170_USB_REG_BASE: c_uint = 0x1e1000;

pub const AR9170_USB_EP_IN_STALL: c_uint = 0x8;
pub const AR9170_USB_EP_IN_TOGGLE: c_uint = 0x10;

pub const AR9170_USB_EP_OUT_STALL: c_uint = 0x8;
pub const AR9170_USB_EP_OUT_TOGGLE: c_uint = 0x10;

pub const AR9170_USB_DMA_CTL_UP_STREAM_S: c_int = 4;

// PCI/USB to AHB Bridge
pub const AR9170_PTA_REG_BASE: c_uint = 0x1e2000;

//
// PCI to AHB Bridge
//

pub const AR9170_PTA_INT_FLAG_DN: c_uint = 0x01;
pub const AR9170_PTA_INT_FLAG_UP: c_uint = 0x02;
pub const AR9170_PTA_INT_FLAG_CMD: c_uint = 0x04;

pub const AR9170_PTA_CTRL_4_BEAT_BURST: c_uint = 0x00;
pub const AR9170_PTA_CTRL_8_BEAT_BURST: c_uint = 0x01;
pub const AR9170_PTA_CTRL_16_BEAT_BURST: c_uint = 0x02;
pub const AR9170_PTA_CTRL_LOOPBACK_MODE: c_uint = 0x10;

// Protocol Controller Module

pub const AR9170_NUM_LEDS: c_int = 2;
// CAM
pub const AR9170_CAM_MAX_USER: c_int = 64;
pub const AR9170_CAM_MAX_KEY_LENGTH: c_int = 16;
pub const AR9170_SRAM_OFFSET: c_uint = 0x100000;
pub const AR9170_SRAM_SIZE: c_uint = 0x18000;
pub const AR9170_PRAM_OFFSET: c_uint = 0x200000;
pub const AR9170_PRAM_SIZE: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpu_clock {
    AHB_STATIC_40MHZ = 0,
    AHB_GMODE_22MHZ = 1,
    AHB_AMODE_20MHZ = 1,
    AHB_GMODE_44MHZ = 2,
    AHB_AMODE_40MHZ = 2,
    AHB_GMODE_88MHZ = 3,
    AHB_AMODE_80MHZ = 3
}

// USB endpoints
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ar9170_usb_ep {
//
// Control EP is always EP 0 (USB SPEC)
//
// The weird thing is: the original firmware has a few
// comments that suggest that the actual EP numbers
// are in the 1 to 10 range?!
//
    AR9170_USB_EP_CTRL		= 0,

    AR9170_USB_EP_TX,
    AR9170_USB_EP_RX,
    AR9170_USB_EP_IRQ,
    AR9170_USB_EP_CMD,
    AR9170_USB_NUM_EXTRA_EP		= 4,

    __AR9170_USB_NUM_EP,

    __AR9170_USB_NUM_MAX_EP		= 10
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ar9170_usb_fifo {
    __AR9170_USB_NUM_MAX_FIFO	= 10
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ar9170_tx_queues {
    AR9170_TXQ0	= 0,
    AR9170_TXQ1,
    AR9170_TXQ2,
    AR9170_TXQ3,
    AR9170_TXQ_SPECIAL,

// keep last
    __AR9170_NUM_TX_QUEUES = 5
}

pub const AR9170_TX_STREAM_TAG: c_uint = 0x697e;
pub const AR9170_RX_STREAM_TAG: c_uint = 0x4e00;
pub const AR9170_RX_STREAM_MAX_SIZE: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_stream {
    pub length: __le16,
    pub tag: __le16,
    pub payload: [u8; ],
    pub __aligned(4): } __packed,
pub const AR9170_STREAM_LEN: c_int = 4;
pub const AR9170_MAX_ACKTABLE_ENTRIES: c_int = 8;
pub const AR9170_MAX_VIRTUAL_MAC: c_int = 7;
pub const AR9170_USB_EP_CTRL_MAX: c_int = 64;
pub const AR9170_USB_EP_TX_MAX: c_int = 512;
pub const AR9170_USB_EP_RX_MAX: c_int = 512;
pub const AR9170_USB_EP_IRQ_MAX: c_int = 64;
pub const AR9170_USB_EP_CMD_MAX: c_int = 64;
// Trigger PRETBTT interrupt 6 Kus earlier
pub const CARL9170_PRETBTT_KUS: c_int = 6;
pub const AR5416_MAX_RATE_POWER: c_int = 63;

