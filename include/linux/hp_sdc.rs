//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hp_sdc.h
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
// HP i8042 System Device Controller -- header
//
// Copyright (c) 2001 Brian S. Julin
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. The name of the author may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL").
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
//
// References:
//
// HP-HIL Technical Reference Manual.  Hewlett Packard Product No. 45918A
//
// System Device Controller Microprocessor Firmware Theory of Operation
// for Part Number 1820-4784 Revision B.  Dwg No. A-1820-4784-2
//

// No 4X status reads take longer than this (in usec).
//
pub const HP_SDC_MAX_REG_DELAY: c_int = 20000;
extern "C" {
    pub fn hp_sdc_request_timer_irq(callback: *mut hp_sdc_irqhook) -> c_int;
}
extern "C" {
    pub fn hp_sdc_request_hil_irq(callback: *mut hp_sdc_irqhook) -> c_int;
}
extern "C" {
    pub fn hp_sdc_request_cooked_irq(callback: *mut hp_sdc_irqhook) -> c_int;
}
extern "C" {
    pub fn hp_sdc_release_timer_irq(callback: *mut hp_sdc_irqhook) -> c_int;
}
extern "C" {
    pub fn hp_sdc_release_hil_irq(callback: *mut hp_sdc_irqhook) -> c_int;
}
extern "C" {
    pub fn hp_sdc_release_cooked_irq(callback: *mut hp_sdc_irqhook) -> c_int;
}
extern "C" {
    pub fn __hp_sdc_enqueue_transaction(this: *mut hp_sdc_transaction) -> c_int;
}
extern "C" {
    pub fn hp_sdc_enqueue_transaction(this: *mut hp_sdc_transaction) -> c_int;
}
extern "C" {
    pub fn hp_sdc_dequeue_transaction(this: *mut hp_sdc_transaction) -> c_int;
}
// The HP_SDC_ACT* values are peculiar to this driver.
// Nuance: never HP_SDC_ACT_DATAIN | HP_SDC_ACT_DEALLOC, use another
// act to perform the dealloc.
//
pub const HP_SDC_ACT_PRECMD: c_uint = 0x01		/* Send a command first */;
pub const HP_SDC_ACT_DATAREG: c_uint = 0x02		/* Set data registers */;
pub const HP_SDC_ACT_DATAOUT: c_uint = 0x04		/* Send data bytes */;
pub const HP_SDC_ACT_POSTCMD: c_uint = 0x08            /* Send command after */;
pub const HP_SDC_ACT_DATAIN: c_uint = 0x10		/* Collect data after */;
pub const HP_SDC_ACT_DURING: c_uint = 0x1f;
pub const HP_SDC_ACT_SEMAPHORE: c_uint = 0x20            /* Raise semaphore after */;
pub const HP_SDC_ACT_CALLBACK: c_uint = 0x40		/* Pass data to IRQ handler */;
pub const HP_SDC_ACT_DEALLOC: c_uint = 0x80		/* Destroy transaction after */;
pub const HP_SDC_ACT_AFTER: c_uint = 0xe0;
pub const HP_SDC_ACT_DEAD: c_uint = 0x60		/* Act timed out. */;
// Rest of the flags are straightforward representation of the SDC interface
pub const HP_SDC_STATUS_IBF: c_uint = 0x02	/* Input buffer full */;
pub const HP_SDC_STATUS_IRQMASK: c_uint = 0xf0	/* Bits containing "level 1" irq */;
pub const HP_SDC_STATUS_PERIODIC: c_uint = 0x10    /* Periodic 10ms timer */;
pub const HP_SDC_STATUS_USERTIMER: c_uint = 0x20    /* "Special purpose" timer */;
pub const HP_SDC_STATUS_TIMER: c_uint = 0x30    /* Both PERIODIC and USERTIMER */;
pub const HP_SDC_STATUS_REG: c_uint = 0x40	/* Data from an i8042 register */;
pub const HP_SDC_STATUS_HILCMD: c_uint = 0x50	/* Command from HIL MLC */;
pub const HP_SDC_STATUS_HILDATA: c_uint = 0x60	/* Data from HIL MLC */;
pub const HP_SDC_STATUS_PUP: c_uint = 0x70	/* Successful power-up self test */;
pub const HP_SDC_STATUS_KCOOKED: c_uint = 0x80	/* Key from cooked kbd */;
pub const HP_SDC_STATUS_KRPG: c_uint = 0xc0	/* Key from Repeat Gen */;
pub const HP_SDC_STATUS_KMOD_SUP: c_uint = 0x10	/* Shift key is up */;
pub const HP_SDC_STATUS_KMOD_CUP: c_uint = 0x20	/* Control key is up */;
pub const HP_SDC_NMISTATUS_FHS: c_uint = 0x40	/* NMI is a fast handshake irq */;
// Internal i8042 registers (there are more, but they are not too useful).
pub const HP_SDC_USE: c_uint = 0x02	/* Resource usage (including OB bit) */;
pub const HP_SDC_IM: c_uint = 0x04	/* Interrupt mask */;
pub const HP_SDC_CFG: c_uint = 0x11	/* Configuration register */;
pub const HP_SDC_KBLANGUAGE: c_uint = 0x12	/* Keyboard language */;
pub const HP_SDC_D0: c_uint = 0x70	/* General purpose data buffer 0 */;
pub const HP_SDC_D1: c_uint = 0x71	/* General purpose data buffer 1 */;
pub const HP_SDC_D2: c_uint = 0x72	/* General purpose data buffer 2 */;
pub const HP_SDC_D3: c_uint = 0x73	/* General purpose data buffer 3 */;
pub const HP_SDC_VT1: c_uint = 0x74	/* Timer for voice 1 */;
pub const HP_SDC_VT2: c_uint = 0x75	/* Timer for voice 2 */;
pub const HP_SDC_VT3: c_uint = 0x76	/* Timer for voice 3 */;
pub const HP_SDC_VT4: c_uint = 0x77	/* Timer for voice 4 */;
pub const HP_SDC_KBN: c_uint = 0x78	/* Which HIL devs are Nimitz */;
pub const HP_SDC_KBC: c_uint = 0x79	/* Which HIL devs are cooked kbds */;
pub const HP_SDC_LPS: c_uint = 0x7a	/* i8042's view of HIL status */;
pub const HP_SDC_LPC: c_uint = 0x7b	/* i8042's view of HIL "control" */;
pub const HP_SDC_RSV: c_uint = 0x7c	/* Reserved "for testing" */;
pub const HP_SDC_LPR: c_uint = 0x7d    /* i8042 count of HIL reconfigs */;
pub const HP_SDC_XTD: c_uint = 0x7e    /* "Extended Configuration" register */;
pub const HP_SDC_STR: c_uint = 0x7f    /* i8042 self-test result */;
// Bitfields for above registers
pub const HP_SDC_USE_LOOP: c_uint = 0x04	/* Command is currently on the loop. */;
pub const HP_SDC_IM_MASK: c_uint = 0x1f    /* these bits not part of cmd/status */;
pub const HP_SDC_IM_FH: c_uint = 0x10	/* Mask the fast handshake irq */;
pub const HP_SDC_IM_PT: c_uint = 0x08	/* Mask the periodic timer irq */;
pub const HP_SDC_IM_TIMERS: c_uint = 0x04	/* Mask the MT/DT/CT irq */;
pub const HP_SDC_IM_RESET: c_uint = 0x02	/* Mask the reset key irq */;
pub const HP_SDC_IM_HIL: c_uint = 0x01	/* Mask the HIL MLC irq */;
pub const HP_SDC_CFG_ROLLOVER: c_uint = 0x08	/* WTF is "N-key rollover"? */;
pub const HP_SDC_CFG_KBD: c_uint = 0x10	/* There is a keyboard */;
pub const HP_SDC_CFG_NEW: c_uint = 0x20	/* Supports/uses HIL MLC */;
pub const HP_SDC_CFG_KBD_OLD: c_uint = 0x03	/* keyboard code for non-HIL */;
pub const HP_SDC_CFG_KBD_NEW: c_uint = 0x07	/* keyboard code from HIL autoconfig */;
pub const HP_SDC_CFG_REV: c_uint = 0x40	/* Code revision bit */;
pub const HP_SDC_CFG_IDPROM: c_uint = 0x80	/* IDPROM present in kbd (not HIL) */;
pub const HP_SDC_LPS_NDEV: c_uint = 0x07	/* # devices autoconfigured on HIL */;
pub const HP_SDC_LPS_ACSUCC: c_uint = 0x08	/* loop autoconfigured successfully */;
pub const HP_SDC_LPS_ACFAIL: c_uint = 0x80	/* last loop autoconfigure failed */;
pub const HP_SDC_LPC_APE_IPF: c_uint = 0x01	/* HIL MLC APE/IPF (autopoll) set */;
pub const HP_SDC_LPC_ARCONERR: c_uint = 0x02	/* i8042 autoreconfigs loop on err */;
pub const HP_SDC_LPC_ARCQUIET: c_uint = 0x03	/* i8042 doesn't report autoreconfigs*/;
pub const HP_SDC_LPC_COOK: c_uint = 0x10	/* i8042 cooks devices in _KBN */;
pub const HP_SDC_LPC_RC: c_uint = 0x80	/* causes autoreconfig */;
pub const HP_SDC_XTD_REV: c_uint = 0x07	/* contains revision code */;

pub const HP_SDC_XTD_BEEPER: c_uint = 0x08	/* TI SN76494 beeper available */;
pub const HP_SDC_XTD_BBRTC: c_uint = 0x20	/* OKI MSM-58321 BBRTC present */;
pub const HP_SDC_CMD_LOAD_RT: c_uint = 0x31	/* Load real time (from 8042) */;
pub const HP_SDC_CMD_LOAD_FHS: c_uint = 0x36	/* Load the fast handshake timer */;
pub const HP_SDC_CMD_LOAD_MT: c_uint = 0x38	/* Load the match timer */;
pub const HP_SDC_CMD_LOAD_DT: c_uint = 0x3B	/* Load the delay timer */;
pub const HP_SDC_CMD_LOAD_CT: c_uint = 0x3E	/* Load the cycle timer */;
pub const HP_SDC_CMD_SET_IM: c_uint = 0x40    /* 010xxxxx == set irq mask */;
// The documents provided do not explicitly state that all registers between
// 0x01 and 0x1f inclusive can be read by sending their register index as a
// command, but this is implied and appears to be the case.
//
pub const HP_SDC_CMD_READ_RAM: c_uint = 0x00	/* Load from i8042 RAM (autoinc) */;
pub const HP_SDC_CMD_READ_USE: c_uint = 0x02	/* Undocumented! Load from usage reg */;
pub const HP_SDC_CMD_READ_IM: c_uint = 0x04	/* Load current interrupt mask */;
pub const HP_SDC_CMD_READ_KCC: c_uint = 0x11	/* Load primary kbd config code */;
pub const HP_SDC_CMD_READ_KLC: c_uint = 0x12	/* Load primary kbd language code */;
pub const HP_SDC_CMD_READ_T1: c_uint = 0x13	/* Load timer output buffer byte 1 */;
pub const HP_SDC_CMD_READ_T2: c_uint = 0x14	/* Load timer output buffer byte 1 */;
pub const HP_SDC_CMD_READ_T3: c_uint = 0x15	/* Load timer output buffer byte 1 */;
pub const HP_SDC_CMD_READ_T4: c_uint = 0x16	/* Load timer output buffer byte 1 */;
pub const HP_SDC_CMD_READ_T5: c_uint = 0x17	/* Load timer output buffer byte 1 */;
pub const HP_SDC_CMD_READ_D0: c_uint = 0xf0	/* Load from i8042 RAM location 0x70 */;
pub const HP_SDC_CMD_READ_D1: c_uint = 0xf1	/* Load from i8042 RAM location 0x71 */;
pub const HP_SDC_CMD_READ_D2: c_uint = 0xf2	/* Load from i8042 RAM location 0x72 */;
pub const HP_SDC_CMD_READ_D3: c_uint = 0xf3	/* Load from i8042 RAM location 0x73 */;
pub const HP_SDC_CMD_READ_VT1: c_uint = 0xf4	/* Load from i8042 RAM location 0x74 */;
pub const HP_SDC_CMD_READ_VT2: c_uint = 0xf5	/* Load from i8042 RAM location 0x75 */;
pub const HP_SDC_CMD_READ_VT3: c_uint = 0xf6	/* Load from i8042 RAM location 0x76 */;
pub const HP_SDC_CMD_READ_VT4: c_uint = 0xf7	/* Load from i8042 RAM location 0x77 */;
pub const HP_SDC_CMD_READ_KBN: c_uint = 0xf8	/* Load from i8042 RAM location 0x78 */;
pub const HP_SDC_CMD_READ_KBC: c_uint = 0xf9	/* Load from i8042 RAM location 0x79 */;
pub const HP_SDC_CMD_READ_LPS: c_uint = 0xfa	/* Load from i8042 RAM location 0x7a */;
pub const HP_SDC_CMD_READ_LPC: c_uint = 0xfb	/* Load from i8042 RAM location 0x7b */;
pub const HP_SDC_CMD_READ_RSV: c_uint = 0xfc	/* Load from i8042 RAM location 0x7c */;
pub const HP_SDC_CMD_READ_LPR: c_uint = 0xfd	/* Load from i8042 RAM location 0x7d */;
pub const HP_SDC_CMD_READ_XTD: c_uint = 0xfe	/* Load from i8042 RAM location 0x7e */;
pub const HP_SDC_CMD_READ_STR: c_uint = 0xff	/* Load from i8042 RAM location 0x7f */;
pub const HP_SDC_CMD_SET_ARD: c_uint = 0xA0	/* Set emulated autorepeat delay */;
pub const HP_SDC_CMD_SET_ARR: c_uint = 0xA2	/* Set emulated autorepeat rate */;
pub const HP_SDC_CMD_SET_BELL: c_uint = 0xA3	/* Set voice 3 params for "beep" cmd */;
pub const HP_SDC_CMD_SET_RPGR: c_uint = 0xA6	/* Set "RPG" irq rate (doesn't work) */;
pub const HP_SDC_CMD_SET_RTMS: c_uint = 0xAD	/* Set the RTC time (milliseconds) */;
pub const HP_SDC_CMD_SET_RTD: c_uint = 0xAF	/* Set the RTC time (days) */;
pub const HP_SDC_CMD_SET_FHS: c_uint = 0xB2	/* Set fast handshake timer */;
pub const HP_SDC_CMD_SET_MT: c_uint = 0xB4	/* Set match timer */;
pub const HP_SDC_CMD_SET_DT: c_uint = 0xB7	/* Set delay timer */;
pub const HP_SDC_CMD_SET_CT: c_uint = 0xBA	/* Set cycle timer */;
pub const HP_SDC_CMD_SET_RAMP: c_uint = 0xC1	/* Reset READ_RAM autoinc counter */;
pub const HP_SDC_CMD_SET_D0: c_uint = 0xe0	/* Load to i8042 RAM location 0x70 */;
pub const HP_SDC_CMD_SET_D1: c_uint = 0xe1	/* Load to i8042 RAM location 0x71 */;
pub const HP_SDC_CMD_SET_D2: c_uint = 0xe2	/* Load to i8042 RAM location 0x72 */;
pub const HP_SDC_CMD_SET_D3: c_uint = 0xe3	/* Load to i8042 RAM location 0x73 */;
pub const HP_SDC_CMD_SET_VT1: c_uint = 0xe4	/* Load to i8042 RAM location 0x74 */;
pub const HP_SDC_CMD_SET_VT2: c_uint = 0xe5	/* Load to i8042 RAM location 0x75 */;
pub const HP_SDC_CMD_SET_VT3: c_uint = 0xe6	/* Load to i8042 RAM location 0x76 */;
pub const HP_SDC_CMD_SET_VT4: c_uint = 0xe7	/* Load to i8042 RAM location 0x77 */;
pub const HP_SDC_CMD_SET_KBN: c_uint = 0xe8	/* Load to i8042 RAM location 0x78 */;
pub const HP_SDC_CMD_SET_KBC: c_uint = 0xe9	/* Load to i8042 RAM location 0x79 */;
pub const HP_SDC_CMD_SET_LPS: c_uint = 0xea	/* Load to i8042 RAM location 0x7a */;
pub const HP_SDC_CMD_SET_LPC: c_uint = 0xeb	/* Load to i8042 RAM location 0x7b */;
pub const HP_SDC_CMD_SET_RSV: c_uint = 0xec	/* Load to i8042 RAM location 0x7c */;
pub const HP_SDC_CMD_SET_LPR: c_uint = 0xed	/* Load to i8042 RAM location 0x7d */;
pub const HP_SDC_CMD_SET_XTD: c_uint = 0xee	/* Load to i8042 RAM location 0x7e */;
pub const HP_SDC_CMD_SET_STR: c_uint = 0xef	/* Load to i8042 RAM location 0x7f */;
pub const HP_SDC_CMD_DO_RTCW: c_uint = 0xc2	/* i8042 RAM 0x70 --> RTC */;
pub const HP_SDC_CMD_DO_RTCR: c_uint = 0xc3	/* RTC[0x70 0:3] --> irq/status/data */;
pub const HP_SDC_CMD_DO_BEEP: c_uint = 0xc4	/* i8042 RAM 0x70-74  --> beeper,VT3 */;
pub const HP_SDC_CMD_DO_HIL: c_uint = 0xc5	/* i8042 RAM 0x70-73 -->;
// Values used to (de)mangle input/output to/from the HIL MLC
pub const HP_SDC_DATA: c_uint = 0x40	/* Data from an 8042 register */;
pub const HP_SDC_HIL_CMD: c_uint = 0x50	/* Data from HIL MLC R1/8042 */;
pub const HP_SDC_HIL_R1MASK: c_uint = 0x0f	/* Contents of HIL MLC R1 0:3 */;
pub const HP_SDC_HIL_AUTO: c_uint = 0x10	/* Set if POL results from i8042 */;
pub const HP_SDC_HIL_ISERR: c_uint = 0x80	/* Has meaning as in next 4 values */;
pub const HP_SDC_HIL_RC_DONE: c_uint = 0x80	/* i8042 auto-configured loop */;
pub const HP_SDC_HIL_ERR: c_uint = 0x81	/* HIL MLC R2 had a bit set */;
pub const HP_SDC_HIL_TO: c_uint = 0x82	/* i8042 HIL watchdog expired */;
pub const HP_SDC_HIL_RC: c_uint = 0x84	/* i8042 is auto-configuring loop */;
pub const HP_SDC_HIL_DAT: c_uint = 0x60	/* Data from HIL MLC R0 */;
pub const HP_SDC_QUEUE_LEN: c_int = 16;

