//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/reg.h
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
// This file is part of wl12xx
//
// Copyright (c) 1998-2007 Texas Instruments Incorporated
// Copyright (C) 2008 Nokia Corporation
//

pub const REGISTERS_BASE: c_uint = 0x00300000;
pub const DRPW_BASE: c_uint = 0x00310000;
pub const REGISTERS_DOWN_SIZE: c_uint = 0x00008800;
pub const REGISTERS_WORK_SIZE: c_uint = 0x0000b000;
pub const HW_ACCESS_ELP_CTRL_REG_ADDR: c_uint = 0x1FFFC;
// ELP register commands
pub const ELPCTRL_WAKE_UP: c_uint = 0x1;
pub const ELPCTRL_WAKE_UP_WLAN_READY: c_uint = 0x5;
pub const ELPCTRL_SLEEP: c_uint = 0x0;
// ELP WLAN_READY bit
pub const ELPCTRL_WLAN_READY: c_uint = 0x2;
// Device Configuration registers

// EEPROM registers

pub const EE_CTL_READ: c_int = 2;

// Power Management registers

// Scratch Pad registers

// Spare registers

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl12xx_acx_int_reg {
    ACX_REG_INTERRUPT_TRIG,
    ACX_REG_INTERRUPT_TRIG_H,

// =============================================
    Host Interrupt Mask Register - 32bit (RW)
    ------------------------------------------
    Setting a bit in this register masks the
    corresponding interrupt to the host.
    0 - RX0		- Rx first dubble buffer Data Interrupt
    1 - TXD		- Tx Data Interrupt
    2 - TXXFR		- Tx Transfer Interrupt
    3 - RX1		- Rx second dubble buffer Data Interrupt
    4 - RXXFR		- Rx Transfer Interrupt
    5 - EVENT_A	- Event Mailbox interrupt
    6 - EVENT_B	- Event Mailbox interrupt
    7 - WNONHST	- Wake On Host Interrupt
    8 - TRACE_A	- Debug Trace interrupt
    9 - TRACE_B	- Debug Trace interrupt
    10 - CDCMP		- Command Complete Interrupt
    11 -
    12 -
    13 -
    14 - ICOMP		- Initialization Complete Interrupt
    16 - SG SE		- Soft Gemini - Sense enable interrupt
    17 - SG SD		- Soft Gemini - Sense disable interrupt
    18 -			-
    19 -			-
    20 -			-
    21-			-
    Default: 0x0001
// ==============================================
    ACX_REG_INTERRUPT_MASK,

// =============================================
    Host Interrupt Mask Set 16bit, (Write only)
    ------------------------------------------
    Setting a bit in this register sets
    the corresponding bin in ACX_HINT_MASK register
    without effecting the mask
    state of other bits (0 = no effect).
    ==============================================*/
    ACX_REG_HINT_MASK_SET,

// =============================================
    Host Interrupt Mask Clear 16bit,(Write only)
    ------------------------------------------
    Setting a bit in this register clears
    the corresponding bin in ACX_HINT_MASK register
    without effecting the mask
    state of other bits (0 = no effect).
    =============================================*/
    ACX_REG_HINT_MASK_CLR,

// =============================================
    Host Interrupt Status Nondestructive Read
    16bit,(Read only)
    ------------------------------------------
    The host can read this register to determine
    which interrupts are active.
    Reading this register doesn't
    effect its content.
    =============================================*/
    ACX_REG_INTERRUPT_NO_CLEAR,

// =============================================
    Host Interrupt Status Clear on Read  Register
    16bit,(Read only)
    ------------------------------------------
    The host can read this register to determine
    which interrupts are active.
    Reading this register clears it,
    thus making all interrupts inactive.
    ==============================================*/
    ACX_REG_INTERRUPT_CLEAR,

// =============================================
    Host Interrupt Acknowledge Register
    16bit,(Write only)
    ------------------------------------------
    The host can set individual bits in this
    register to clear (acknowledge) the corresp.
    interrupt status bits in the HINT_STS_CLR and
    HINT_STS_ND registers, thus making the
    assotiated interrupt inactive. (0-no effect)
    ==============================================*/
    ACX_REG_INTERRUPT_ACK,

// ===============================================
    Host Software Reset - 32bit RW
    ------------------------------------------
    [31:1] Reserved
    0  SOFT_RESET Soft Reset  - When this bit is set,
    it holds the Wlan hardware in a soft reset state.
    This reset disables all MAC and baseband processor
    clocks except the CardBus/PCI interface clock.
    It also initializes all MAC state machines except
    the host interface. It does not reload the
    contents of the EEPROM. When this bit is cleared
    (not self-clearing), the Wlan hardware
    exits the software reset state.
    ===============================================*/
    ACX_REG_SLV_SOFT_RESET,

// ===============================================
    EEPROM Burst Read Start  - 32bit RW
    ------------------------------------------
    [31:1] Reserved
    0  ACX_EE_START -  EEPROM Burst Read Start 0
    Setting this bit starts a burst read from
    the external EEPROM.
    If this bit is set (after reset) before an EEPROM read/write,
    the burst read starts at EEPROM address 0.
    Otherwise, it starts at the address
    following the address of the previous access.
    TheWlan hardware clears this bit automatically.

    Default: 0x00000000
// ================================================
    ACX_REG_EE_START,

// Embedded ARM CPU Control

// ===============================================
    Halt eCPU   - 32bit RW
    ------------------------------------------
    0 HALT_ECPU Halt Embedded CPU - This bit is the
    complement of bit 1 (MDATA2) in the SOR_CFG register.
    During a hardware reset, this bit holds
    the inverse of MDATA2.
    When downloading firmware from the host,
    set this bit (pull down MDATA2).
    The host clears this bit after downloading the firmware into
    zero-wait-state SSRAM.
    When loading firmware from Flash, clear this bit (pull up MDATA2)
    so that the eCPU can run the bootloader code in Flash
    HALT_ECPU eCPU State
    --------------------
    1 halt eCPU
    0 enable eCPU
    ===============================================*/
    ACX_REG_ECPU_CONTROL,

    ACX_REG_TABLE_LEN
}

// Command/Information Mailbox Pointers
// ===============================================

// ===============================================

// Misc

//
// Rx configuration (filter) information element
// ---------------------------------------------
//

pub const RX_CFG_ENABLE_PHY_HEADER_PLCP: c_uint = 0x0002;
// promiscuous - receives all valid frames
pub const RX_CFG_PROMISCUOUS: c_uint = 0x0008;
// receives frames from any BSSID
pub const RX_CFG_BSSID: c_uint = 0x0020;
// receives frames destined to any MAC address
pub const RX_CFG_MAC: c_uint = 0x0010;
pub const RX_CFG_ENABLE_ONLY_MY_DEST_MAC: c_uint = 0x0010;
pub const RX_CFG_ENABLE_ANY_DEST_MAC: c_uint = 0x0000;
pub const RX_CFG_ENABLE_ONLY_MY_BSSID: c_uint = 0x0020;
pub const RX_CFG_ENABLE_ANY_BSSID: c_uint = 0x0000;
// discards all broadcast frames
pub const RX_CFG_DISABLE_BCAST: c_uint = 0x0200;
pub const RX_CFG_ENABLE_ONLY_MY_SSID: c_uint = 0x0400;
pub const RX_CFG_ENABLE_RX_CMPLT_FCS_ERROR: c_uint = 0x0800;
pub const RX_CFG_COPY_RX_STATUS: c_uint = 0x2000;
pub const RX_CFG_TSF: c_uint = 0x10000;

pub const RX_FILTER_OPTION_FILTER_ALL: c_int = 0;

// ===============================================

pub const EE_WRITE: c_uint = 0x00000001ul;
pub const EE_READ: c_uint = 0x00000002ul;
// ===============================================

// ===============================================

pub const START_EEPROM_MGR: c_uint = 0x00000001;
// ===============================================

// ===============================================

pub const ACX_MAX_GPIO_LINES: c_int = 15;
// ===============================================

pub const ACX_CONT_WIND_MIN_MASK: c_uint = 0x0000007f;
pub const ACX_CONT_WIND_MAX: c_uint = 0x03ff0000;
// ===============================================
pub const HI_CFG_UART_ENABLE: c_uint = 0x00000004;
pub const HI_CFG_RST232_ENABLE: c_uint = 0x00000008;
pub const HI_CFG_CLOCK_REQ_SELECT: c_uint = 0x00000010;
pub const HI_CFG_HOST_INT_ENABLE: c_uint = 0x00000020;
pub const HI_CFG_VLYNQ_OUTPUT_ENABLE: c_uint = 0x00000040;
pub const HI_CFG_HOST_INT_ACTIVE_LOW: c_uint = 0x00000080;
pub const HI_CFG_UART_TX_OUT_GPIO_15: c_uint = 0x00000100;
pub const HI_CFG_UART_TX_OUT_GPIO_14: c_uint = 0x00000200;
pub const HI_CFG_UART_TX_OUT_GPIO_7: c_uint = 0x00000400;
//
// NOTE: USE_ACTIVE_HIGH compilation flag should be defined in makefile
// for platforms using active high interrupt level
//

pub const REF_FREQ_19_2: c_int = 0;
pub const REF_FREQ_26_0: c_int = 1;
pub const REF_FREQ_38_4: c_int = 2;
pub const REF_FREQ_40_0: c_int = 3;
pub const REF_FREQ_33_6: c_int = 4;
pub const REF_FREQ_NUM: c_int = 5;
pub const LUT_PARAM_INTEGER_DIVIDER: c_int = 0;
pub const LUT_PARAM_FRACTIONAL_DIVIDER: c_int = 1;
pub const LUT_PARAM_ATTN_BB: c_int = 2;
pub const LUT_PARAM_ALPHA_BB: c_int = 3;
pub const LUT_PARAM_STOP_TIME_BB: c_int = 4;
pub const LUT_PARAM_BB_PLL_LOOP_FILTER: c_int = 5;
pub const LUT_PARAM_NUM: c_int = 6;

pub const USE_EEPROM: c_int = 0;
pub const SOFT_RESET_MAX_TIME: c_int = 1000000;
pub const SOFT_RESET_STALL_TIME: c_int = 1000;
pub const NVS_DATA_BUNDARY_ALIGNMENT: c_int = 4;
// Firmware image load chunk size
pub const CHUNK_SIZE: c_int = 512;
// Firmware image header size
pub const FW_HDR_SIZE: c_int = 8;
pub const ECPU_CONTROL_HALT: c_uint = 0x00000101;
//

//
// Hardware to Embedded CPU Interrupts - first 32-bit register set
//
// Host Command Interrupt. Setting this bit masks
// the interrupt that the host issues to inform
// the FW that it has sent a command
// to the Wlan hardware Command Mailbox.
//

//
// Host Event Acknowlegde Interrupt. The host
// sets this bit to acknowledge that it received
// the unsolicited information from the event
// mailbox.
//

//
// The host sets this bit to inform the Wlan
// FW that a TX packet is in the XFER
// Buffer #0.
//

//
// The host sets this bit to inform the FW
// that it read a packet from RX XFER
// Buffer #0.
//

// Hardware to Embedded CPU Interrupts - second 32-bit register set
//
// The host sets this bit to inform the FW
// that it read a packet from RX XFER
// Buffer #1.
//

//
// The host sets this bit to inform the Wlan
// hardware that a TX packet is in the XFER
// Buffer #1.
//

