//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/cec/core/cec-pin-priv.h
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
// cec-pin-priv.h - internal cec-pin header
//
// Copyright 2017 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cec_pin_state {
// CEC is off
    CEC_ST_OFF,
// CEC is idle, waiting for Rx or Tx
    CEC_ST_IDLE,

// Tx states

// Pending Tx, waiting for Signal Free Time to expire
    CEC_ST_TX_WAIT,
// Low-drive was detected, wait for bus to go high
    CEC_ST_TX_WAIT_FOR_HIGH,
// Drive CEC low for the start bit
    CEC_ST_TX_START_BIT_LOW,
// Drive CEC high for the start bit
    CEC_ST_TX_START_BIT_HIGH,
// Generate a start bit period that is too short
    CEC_ST_TX_START_BIT_HIGH_SHORT,
// Generate a start bit period that is too long
    CEC_ST_TX_START_BIT_HIGH_LONG,
// Drive CEC low for the start bit using the custom timing
    CEC_ST_TX_START_BIT_LOW_CUSTOM,
// Drive CEC high for the start bit using the custom timing
    CEC_ST_TX_START_BIT_HIGH_CUSTOM,
// Drive CEC low for the 0 bit
    CEC_ST_TX_DATA_BIT_0_LOW,
// Drive CEC high for the 0 bit
    CEC_ST_TX_DATA_BIT_0_HIGH,
// Generate a bit period that is too short
    CEC_ST_TX_DATA_BIT_0_HIGH_SHORT,
// Generate a bit period that is too long
    CEC_ST_TX_DATA_BIT_0_HIGH_LONG,
// Drive CEC low for the 1 bit
    CEC_ST_TX_DATA_BIT_1_LOW,
// Drive CEC high for the 1 bit
    CEC_ST_TX_DATA_BIT_1_HIGH,
// Generate a bit period that is too short
    CEC_ST_TX_DATA_BIT_1_HIGH_SHORT,
// Generate a bit period that is too long
    CEC_ST_TX_DATA_BIT_1_HIGH_LONG,
//
// Wait for start of sample time to check for Ack bit or first
// four initiator bits to check for Arbitration Lost.
//
    CEC_ST_TX_DATA_BIT_1_HIGH_PRE_SAMPLE,
// Wait for end of bit period after sampling
    CEC_ST_TX_DATA_BIT_1_HIGH_POST_SAMPLE,
// Generate a bit period that is too short
    CEC_ST_TX_DATA_BIT_1_HIGH_POST_SAMPLE_SHORT,
// Generate a bit period that is too long
    CEC_ST_TX_DATA_BIT_1_HIGH_POST_SAMPLE_LONG,
// Drive CEC low for a data bit using the custom timing
    CEC_ST_TX_DATA_BIT_LOW_CUSTOM,
// Drive CEC high for a data bit using the custom timing
    CEC_ST_TX_DATA_BIT_HIGH_CUSTOM,
// Drive CEC low for a standalone pulse using the custom timing
    CEC_ST_TX_PULSE_LOW_CUSTOM,
// Drive CEC high for a standalone pulse using the custom timing
    CEC_ST_TX_PULSE_HIGH_CUSTOM,
// Start low drive
    CEC_ST_TX_LOW_DRIVE,

// Rx states

// Start bit low detected
    CEC_ST_RX_START_BIT_LOW,
// Start bit high detected
    CEC_ST_RX_START_BIT_HIGH,
// Wait for bit sample time
    CEC_ST_RX_DATA_SAMPLE,
// Wait for earliest end of bit period after sampling
    CEC_ST_RX_DATA_POST_SAMPLE,
// Wait for CEC to go low (i.e. end of bit period)
    CEC_ST_RX_DATA_WAIT_FOR_LOW,
// Drive CEC low to send 0 Ack bit
    CEC_ST_RX_ACK_LOW,
// End of 0 Ack time, wait for earliest end of bit period
    CEC_ST_RX_ACK_LOW_POST,
// Wait for CEC to go high (i.e. end of bit period
    CEC_ST_RX_ACK_HIGH_POST,
// Wait for earliest end of bit period and end of message
    CEC_ST_RX_ACK_FINISH,
// Start low drive
    CEC_ST_RX_LOW_DRIVE,

// Monitor pin using interrupts
    CEC_ST_RX_IRQ,

// Total number of pin states
    CEC_PIN_STATES
}

// Error Injection
// Error injection modes
pub const CEC_ERROR_INJ_MODE_OFF: c_int = 0;
pub const CEC_ERROR_INJ_MODE_ONCE: c_int = 1;
pub const CEC_ERROR_INJ_MODE_ALWAYS: c_int = 2;
pub const CEC_ERROR_INJ_MODE_TOGGLE: c_int = 3;

// Receive error injection options
pub const CEC_ERROR_INJ_RX_NACK_OFFSET: c_int = 0;
pub const CEC_ERROR_INJ_RX_LOW_DRIVE_OFFSET: c_int = 2;
pub const CEC_ERROR_INJ_RX_ADD_BYTE_OFFSET: c_int = 4;
pub const CEC_ERROR_INJ_RX_REMOVE_BYTE_OFFSET: c_int = 6;
pub const CEC_ERROR_INJ_RX_ARB_LOST_OFFSET: c_int = 8;
pub const CEC_ERROR_INJ_RX_MASK: c_uint = 0xffffULL;
// Transmit error injection options
pub const CEC_ERROR_INJ_TX_NO_EOM_OFFSET: c_int = 16;
pub const CEC_ERROR_INJ_TX_EARLY_EOM_OFFSET: c_int = 18;
pub const CEC_ERROR_INJ_TX_SHORT_BIT_OFFSET: c_int = 20;
pub const CEC_ERROR_INJ_TX_LONG_BIT_OFFSET: c_int = 22;
pub const CEC_ERROR_INJ_TX_CUSTOM_BIT_OFFSET: c_int = 24;
pub const CEC_ERROR_INJ_TX_SHORT_START_OFFSET: c_int = 26;
pub const CEC_ERROR_INJ_TX_LONG_START_OFFSET: c_int = 28;
pub const CEC_ERROR_INJ_TX_CUSTOM_START_OFFSET: c_int = 30;
pub const CEC_ERROR_INJ_TX_LAST_BIT_OFFSET: c_int = 32;
pub const CEC_ERROR_INJ_TX_ADD_BYTES_OFFSET: c_int = 34;
pub const CEC_ERROR_INJ_TX_REMOVE_BYTE_OFFSET: c_int = 36;
pub const CEC_ERROR_INJ_TX_LOW_DRIVE_OFFSET: c_int = 38;
pub const CEC_ERROR_INJ_TX_MASK: c_uint = 0xffffffffffff0000ULL;
pub const CEC_ERROR_INJ_RX_LOW_DRIVE_ARG_IDX: c_int = 0;
pub const CEC_ERROR_INJ_RX_ARB_LOST_ARG_IDX: c_int = 1;
pub const CEC_ERROR_INJ_TX_ADD_BYTES_ARG_IDX: c_int = 2;
pub const CEC_ERROR_INJ_TX_SHORT_BIT_ARG_IDX: c_int = 3;
pub const CEC_ERROR_INJ_TX_LONG_BIT_ARG_IDX: c_int = 4;
pub const CEC_ERROR_INJ_TX_CUSTOM_BIT_ARG_IDX: c_int = 5;
pub const CEC_ERROR_INJ_TX_LAST_BIT_ARG_IDX: c_int = 6;
pub const CEC_ERROR_INJ_TX_LOW_DRIVE_ARG_IDX: c_int = 7;
pub const CEC_ERROR_INJ_NUM_ARGS: c_int = 8;
// Special CEC op values
pub const CEC_ERROR_INJ_OP_ANY: c_uint = 0x00000100;
// The default for the low/high time of the custom pulse
pub const CEC_TIM_CUSTOM_DEFAULT: c_int = 1000;
// The default for the low/high time of the glitch pulse
pub const CEC_TIM_GLITCH_DEFAULT: c_int = 1;
pub const CEC_NUM_PIN_EVENTS: c_int = 128;

pub const CEC_PIN_IRQ_UNCHANGED: c_int = 0;
pub const CEC_PIN_IRQ_DISABLE: c_int = 1;
pub const CEC_PIN_IRQ_ENABLE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_pin {
    pub adap: *mut cec_adapter,
    pub ops: *const cec_pin_ops,
    pub kthread: *mut task_struct,
    pub kthread_waitq: wait_queue_head_t,
    pub timer: hrtimer,
    pub ts: ktime_t,
    pub wait_usecs: c_uint,
    pub la_mask: u16,
    pub monitor_all: bool,
    pub rx_eom: bool,
    pub enabled_irq: bool,
    pub enable_irq_failed: bool,
    pub state: cec_pin_state,
    pub tx_msg: cec_msg,
    pub tx_bit: u32,
    pub tx_nacked: bool,
    pub tx_signal_free_time: u32,
    pub tx_toggle: bool,
    pub rx_msg: cec_msg,
    pub rx_bit: u32,
    pub rx_toggle: bool,
    pub rx_start_bit_low_too_short_cnt: u32,
    pub rx_start_bit_low_too_short_ts: u64,
    pub rx_start_bit_low_too_short_delta: u32,
    pub rx_start_bit_too_short_cnt: u32,
    pub rx_start_bit_too_short_ts: u64,
    pub rx_start_bit_too_short_delta: u32,
    pub rx_start_bit_too_long_cnt: u32,
    pub rx_data_bit_too_short_cnt: u32,
    pub rx_data_bit_too_short_ts: u64,
    pub rx_data_bit_too_short_delta: u32,
    pub rx_data_bit_too_long_cnt: u32,
    pub rx_low_drive_cnt: u32,
    pub work_rx_msg: cec_msg,
    pub work_tx_status: u8,
    pub work_tx_ts: ktime_t,
    pub work_irq_change: core::sync::atomic::AtomicI32,
    pub work_pin_num_events: core::sync::atomic::AtomicI32,
    pub work_pin_events_wr: c_uint,
    pub work_pin_events_rd: c_uint,
    pub work_pin_ts: [ktime_t; CEC_NUM_PIN_EVENTS],
    pub work_pin_events: [u8; CEC_NUM_PIN_EVENTS],
    pub work_pin_events_dropped: bool,
    pub work_pin_events_dropped_cnt: u32,
    pub timer_ts: ktime_t,
    pub timer_cnt: u32,
    pub timer_100us_overruns: u32,
    pub timer_300us_overruns: u32,
    pub timer_max_overrun: u32,
    pub timer_sum_overrun: u32,
    pub rx_no_low_drive: bool,
    pub tx_custom_low_usecs: u32,
    pub tx_custom_high_usecs: u32,
    pub tx_glitch_low_usecs: u32,
    pub tx_glitch_high_usecs: u32,
    pub tx_ignore_nack_until_eom: bool,
    pub tx_custom_pulse: bool,
    pub tx_generated_poll: bool,
    pub tx_post_eom: bool,
    pub tx_glitch_falling_edge: bool,
    pub tx_glitch_rising_edge: bool,
    pub tx_extra_bytes: u8,
    pub tx_low_drive_cnt: u32,

    pub 1]: u64 error_inj[CEC_ERROR_INJ_OP_ANY +,
    pub 1][CEC_ERROR_INJ_NUM_ARGS]: u8 error_inj_args[CEC_ERROR_INJ_OP_ANY +,

}

extern "C" {
    pub fn cec_pin_start_timer(pin: *mut cec_pin);
}

extern "C" {
    pub fn cec_pin_error_inj_parse_line(adap: *mut cec_adapter, line: *mut c_char) -> bool;
}
extern "C" {
    pub fn cec_pin_error_inj_show(adap: *mut cec_adapter, sf: *mut seq_file) -> c_int;
}
extern "C" {
    pub fn cec_pin_rx_error_inj(pin: *mut cec_pin) -> u16;
}
extern "C" {
    pub fn cec_pin_tx_error_inj(pin: *mut cec_pin) -> u16;
}

