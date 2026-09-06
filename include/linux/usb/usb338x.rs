//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/usb338x.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// USB 338x super/high/full speed USB device controller.
// Unlike many such controllers, this one talks PCI.
//
// Copyright (C) 2002 NetChip Technology, Inc. (http://www.netchip.com)
// Copyright (C) 2003 David Brownell
// Copyright (C) 2014 Ricardo Ribalda - Qtechnology/AS
//

//
// Extra defined bits for net2280 registers
//
pub const SCRATCH: c_uint = 0x0b;
pub const DEFECT7374_FSM_FIELD: c_int = 28;
pub const SUPER_SPEED: c_int = 8;
pub const DMA_REQUEST_OUTSTANDING: c_int = 5;
pub const DMA_PAUSE_DONE_INTERRUPT: c_int = 26;
pub const SET_ISOCHRONOUS_DELAY: c_int = 24;
pub const SET_SEL: c_int = 22;
pub const SUPER_SPEED_MODE: c_int = 8;
// ep_cfg
pub const MAX_BURST_SIZE: c_int = 24;
pub const EP_FIFO_BYTE_COUNT: c_int = 16;
pub const IN_ENDPOINT_ENABLE: c_int = 14;
pub const IN_ENDPOINT_TYPE: c_int = 12;
pub const OUT_ENDPOINT_ENABLE: c_int = 10;
pub const OUT_ENDPOINT_TYPE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb338x_usb_ext_regs {
    pub usbclass: u32,
pub const DEVICE_PROTOCOL: c_int = 16;
pub const DEVICE_SUB_CLASS: c_int = 8;
pub const DEVICE_CLASS: c_int = 0;
    pub ss_sel: u32,
pub const U2_SYSTEM_EXIT_LATENCY: c_int = 8;
pub const U1_SYSTEM_EXIT_LATENCY: c_int = 0;
    pub ss_del: u32,
pub const U2_DEVICE_EXIT_LATENCY: c_int = 8;
pub const U1_DEVICE_EXIT_LATENCY: c_int = 0;
    pub usb2lpm: u32,
pub const USB_L1_LPM_HIRD: c_int = 2;
pub const USB_L1_LPM_REMOTE_WAKE: c_int = 1;
pub const USB_L1_LPM_SUPPORT: c_int = 0;
    pub usb3belt: u32,
pub const BELT_MULTIPLIER: c_int = 10;
pub const BEST_EFFORT_LATENCY_TOLERANCE: c_int = 0;
    pub usbctl2: u32,
pub const LTM_ENABLE: c_int = 7;
pub const U2_ENABLE: c_int = 6;
pub const U1_ENABLE: c_int = 5;
pub const FUNCTION_SUSPEND: c_int = 4;
pub const USB3_CORE_ENABLE: c_int = 3;
pub const USB2_CORE_ENABLE: c_int = 2;
pub const SERIAL_NUMBER_STRING_ENABLE: c_int = 0;
    pub in_timeout: u32,
pub const GPEP3_TIMEOUT: c_int = 19;
pub const GPEP2_TIMEOUT: c_int = 18;
pub const GPEP1_TIMEOUT: c_int = 17;
pub const GPEP0_TIMEOUT: c_int = 16;
pub const GPEP3_TIMEOUT_VALUE: c_int = 13;
pub const GPEP3_TIMEOUT_ENABLE: c_int = 12;
pub const GPEP2_TIMEOUT_VALUE: c_int = 9;
pub const GPEP2_TIMEOUT_ENABLE: c_int = 8;
pub const GPEP1_TIMEOUT_VALUE: c_int = 5;
pub const GPEP1_TIMEOUT_ENABLE: c_int = 4;
pub const GPEP0_TIMEOUT_VALUE: c_int = 1;
pub const GPEP0_TIMEOUT_ENABLE: c_int = 0;
    pub isodelay: u32,
pub const ISOCHRONOUS_DELAY: c_int = 0;
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb338x_fifo_regs {
// offset 0x0500, 0x0520, 0x0540, 0x0560, 0x0580
    pub ep_fifo_size_base: u32,
pub const IN_FIFO_BASE_ADDRESS: c_int = 22;
pub const IN_FIFO_SIZE: c_int = 16;
pub const OUT_FIFO_BASE_ADDRESS: c_int = 6;
pub const OUT_FIFO_SIZE: c_int = 0;
    pub ep_fifo_out_wrptr: u32,
    pub ep_fifo_out_rdptr: u32,
    pub ep_fifo_in_wrptr: u32,
    pub ep_fifo_in_rdptr: u32,
    pub unused: [u32; 3],
    pub __packed: },
// Link layer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb338x_ll_regs {
// offset 0x700
    pub ll_ltssm_ctrl1: u32,
    pub ll_ltssm_ctrl2: u32,
    pub ll_ltssm_ctrl3: u32,
    pub unused1: u32,
// 0x710
    pub unused2: u32,
    pub ll_general_ctrl0: u32,
    pub ll_general_ctrl1: u32,
pub const PM_U3_AUTO_EXIT: c_int = 29;
pub const PM_U2_AUTO_EXIT: c_int = 28;
pub const PM_U1_AUTO_EXIT: c_int = 27;
pub const PM_FORCE_U2_ENTRY: c_int = 26;
pub const PM_FORCE_U1_ENTRY: c_int = 25;
pub const PM_LGO_COLLISION_SEND_LAU: c_int = 24;
pub const PM_DIR_LINK_REJECT: c_int = 23;
pub const PM_FORCE_LINK_ACCEPT: c_int = 22;
pub const PM_DIR_ENTRY_U3: c_int = 20;
pub const PM_DIR_ENTRY_U2: c_int = 19;
pub const PM_DIR_ENTRY_U1: c_int = 18;
pub const PM_U2_ENABLE: c_int = 17;
pub const PM_U1_ENABLE: c_int = 16;
pub const SKP_THRESHOLD_ADJUST_FMW: c_int = 8;
pub const RESEND_DPP_ON_LRTY_FMW: c_int = 7;
pub const DL_BIT_VALUE_FMW: c_int = 6;
pub const FORCE_DL_BIT: c_int = 5;
    pub ll_general_ctrl2: u32,
pub const SELECT_INVERT_LANE_POLARITY: c_int = 7;
pub const FORCE_INVERT_LANE_POLARITY: c_int = 6;
// 0x720
    pub ll_general_ctrl3: u32,
    pub ll_general_ctrl4: u32,
    pub ll_error_gen: u32,
    pub unused3: u32,
// 0x730
    pub unused4: [u32; 4],
// 0x740
    pub unused5: [u32; 2],
    pub ll_lfps_5: u32,
pub const TIMER_LFPS_6US: c_int = 16;
    pub ll_lfps_6: u32,
pub const TIMER_LFPS_80US: c_int = 0;
// 0x750
    pub unused6: [u32; 8],
// 0x770
    pub unused7: [u32; 3],
    pub ll_tsn_counters_2: u32,
pub const HOT_TX_NORESET_TS2: c_int = 24;
// 0x780
    pub ll_tsn_counters_3: u32,
pub const HOT_RX_RESET_TS2: c_int = 0;
    pub unused8: [u32; 3],
// 0x790
    pub unused9: u32,
    pub ll_lfps_timers_2: u32,
pub const LFPS_TIMERS_2_WORKAROUND_VALUE: c_uint = 0x084d;
    pub unused10: u32,
    pub ll_tsn_chicken_bit: u32,
pub const RECOVERY_IDLE_TO_RECOVER_FMW: c_int = 3;
    pub __packed: },
// protocol layer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb338x_pl_regs {
// offset 0x800
    pub pl_reg_1: u32,
    pub pl_reg_2: u32,
    pub pl_reg_3: u32,
    pub pl_reg_4: u32,
    pub pl_ep_ctrl: u32,
// Protocol Layer Endpoint Control
pub const PL_EP_CTRL: c_uint = 0x810;
pub const ENDPOINT_SELECT: c_int = 0;
// [4:0]
pub const EP_INITIALIZED: c_int = 16;
pub const SEQUENCE_NUMBER_RESET: c_int = 17;
pub const CLEAR_ACK_ERROR_CODE: c_int = 20;
    pub pl_reg_6: u32,
    pub pl_reg_7: u32,
    pub pl_reg_8: u32,
    pub pl_ep_status_1: u32,
// Protocol Layer Endpoint Status 1
pub const PL_EP_STATUS_1: c_uint = 0x820;
pub const STATE: c_int = 16;
pub const ACK_GOOD_NORMAL: c_uint = 0x11;
pub const ACK_GOOD_MORE_ACKS_TO_COME: c_uint = 0x16;
    pub pl_ep_status_2: u32,
    pub pl_ep_status_3: u32,
// Protocol Layer Endpoint Status 3
pub const PL_EP_STATUS_3: c_uint = 0x828;
pub const SEQUENCE_NUMBER: c_int = 0;
    pub pl_ep_status_4: u32,
// Protocol Layer Endpoint Status 4
pub const PL_EP_STATUS_4: c_uint = 0x82c;
    pub pl_ep_cfg_4: u32,
// Protocol Layer Endpoint Configuration 4
pub const PL_EP_CFG_4: c_uint = 0x830;
pub const NON_CTRL_IN_TOLERATE_BAD_DIR: c_int = 6;
    pub __packed: },
