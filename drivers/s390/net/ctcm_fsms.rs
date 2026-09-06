//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/net/ctcm_fsms.h
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
// Copyright IBM Corp. 2001, 2007
// Authors: 	Fritz Elfert (felfert@millenux.com)
// Peter Tiedemann (ptiedem@de.ibm.com)
// MPC additions :
// Belinda Thompson (belindat@us.ibm.com)
// Andy Richter (richtera@us.ibm.com)
//

//
// Definitions for the channel statemachine(s) for ctc and ctcmpc
//
// To allow better kerntyping, prefix-less definitions for channel states
// and channel events have been replaced :
// ch_event... -> ctc_ch_event...
// CH_EVENT... -> CTC_EVENT...
// ch_state... -> ctc_ch_state...
// CH_STATE... -> CTC_STATE...
//
// Events of the channel statemachine(s) for ctc and ctcmpc
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctc_ch_events {
//
// Events, representing return code of
// I/O operations (ccw_device_start, ccw_device_halt et al.)
//
    CTC_EVENT_IO_SUCCESS,
    CTC_EVENT_IO_EBUSY,
    CTC_EVENT_IO_ENODEV,
    CTC_EVENT_IO_UNKNOWN,

    CTC_EVENT_ATTNBUSY,
    CTC_EVENT_ATTN,
    CTC_EVENT_BUSY,
//
// Events, representing unit-check
//
    CTC_EVENT_UC_RCRESET,
    CTC_EVENT_UC_RSRESET,
    CTC_EVENT_UC_TXTIMEOUT,
    CTC_EVENT_UC_TXPARITY,
    CTC_EVENT_UC_HWFAIL,
    CTC_EVENT_UC_RXPARITY,
    CTC_EVENT_UC_ZERO,
    CTC_EVENT_UC_UNKNOWN,
//
// Events, representing subchannel-check
//
    CTC_EVENT_SC_UNKNOWN,
//
// Events, representing machine checks
//
    CTC_EVENT_MC_FAIL,
    CTC_EVENT_MC_GOOD,
//
// Event, representing normal IRQ
//
    CTC_EVENT_IRQ,
    CTC_EVENT_FINSTAT,
//
// Event, representing timer expiry.
//
    CTC_EVENT_TIMER,
//
// Events, representing commands from upper levels.
//
    CTC_EVENT_START,
    CTC_EVENT_STOP,
    CTC_NR_EVENTS,
//
// additional MPC events
//
    CTC_EVENT_SEND_XID = CTC_NR_EVENTS,
    CTC_EVENT_RSWEEP_TIMER,
//
// MUST be always the last element!!
//
    CTC_MPC_NR_EVENTS,
}

//
// States of the channel statemachine(s) for ctc and ctcmpc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctc_ch_states {
//
// Channel not assigned to any device,
// initial state, direction invalid
//
    CTC_STATE_IDLE,
//
// Channel assigned but not operating
//
    CTC_STATE_STOPPED,
    CTC_STATE_STARTWAIT,
    CTC_STATE_STARTRETRY,
    CTC_STATE_SETUPWAIT,
    CTC_STATE_RXINIT,
    CTC_STATE_TXINIT,
    CTC_STATE_RX,
    CTC_STATE_TX,
    CTC_STATE_RXIDLE,
    CTC_STATE_TXIDLE,
    CTC_STATE_RXERR,
    CTC_STATE_TXERR,
    CTC_STATE_TERM,
    CTC_STATE_DTERM,
    CTC_STATE_NOTOP,
    CTC_NR_STATES,     /* MUST be the last element of non-expanded states */
//
// additional MPC states
//
    CH_XID0_PENDING = CTC_NR_STATES,
    CH_XID0_INPROGRESS,
    CH_XID7_PENDING,
    CH_XID7_PENDING1,
    CH_XID7_PENDING2,
    CH_XID7_PENDING3,
    CH_XID7_PENDING4,
    CTC_MPC_NR_STATES, /* MUST be the last element of expanded mpc states */
}

extern "C" {
    pub fn ctcm_ccw_check_rc(ch: *mut channel, rc: c_int, msg: *mut c_char);
}
extern "C" {
    pub fn ctcm_purge_skb_queue(q: *mut sk_buff_head);
}
//
// ----- non-static actions for ctcm channel statemachine -----
//
extern "C" {
    pub fn ctcm_chx_txidle(fi: *mut fsm_instance, event: c_int, arg: *mut c_void);
}
//
// ----- FSM (state/event/action) of the ctcm channel statemachine -----
//
// ----- non-static actions for ctcmpc channel statemachine ----
//
// shared :
extern "C" {
    pub fn ctcm_chx_txidle(fi: *mut *mut fsm_instance, event: c_int, arg: *mut c_void);
}
//
extern "C" {
    pub fn ctcmpc_chx_rxidle(fi: *mut fsm_instance, event: c_int, arg: *mut c_void);
}
//
// ----- FSM (state/event/action) of the ctcmpc channel statemachine -----
//
// Definitions for the device interface statemachine for ctc and mpc
//
// States of the device interface statemachine.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_states {
    DEV_STATE_STOPPED,
    DEV_STATE_STARTWAIT_RXTX,
    DEV_STATE_STARTWAIT_RX,
    DEV_STATE_STARTWAIT_TX,
    DEV_STATE_STOPWAIT_RXTX,
    DEV_STATE_STOPWAIT_RX,
    DEV_STATE_STOPWAIT_TX,
    DEV_STATE_RUNNING,
//
// MUST be always the last element!!
//
    CTCM_NR_DEV_STATES
}

//
// Events of the device interface statemachine.
// ctcm and ctcmpc
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_events {
    DEV_EVENT_START,
    DEV_EVENT_STOP,
    DEV_EVENT_RXUP,
    DEV_EVENT_TXUP,
    DEV_EVENT_RXDOWN,
    DEV_EVENT_TXDOWN,
    DEV_EVENT_RESTART,
//
// MUST be always the last element!!
//
    CTCM_NR_DEV_EVENTS
}

//
// Actions for the device interface statemachine.
// ctc and ctcmpc
//
extern "C" {
    pub fn dev_action_start(fi: *mut *mut fsm_instance, event: c_int, arg: *mut c_void) -> static void;
}
extern "C" {
    pub fn dev_action_stop(fi: *mut *mut fsm_instance, event: c_int, arg: *mut c_void) -> static void;
}
extern "C" {
    pub fn dev_action_restart(fi: *mut fsm_instance, event: c_int, arg: *mut c_void) -> static void;
}
extern "C" {
    pub fn dev_action_chup(fi: *mut *mut fsm_instance, event: c_int, arg: *mut c_void) -> static void;
}
extern "C" {
    pub fn dev_action_chdown(fi: *mut *mut fsm_instance, event: c_int, arg: *mut c_void) -> static void;
}
//
// The (state/event/action) fsm table of the device interface statemachine.
// ctcm and ctcmpc
//
// Definitions for the MPC Group statemachine
//
// MPC Group Station FSM States
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpcg_events {
    MPCG_EVENT_INOP,
    MPCG_EVENT_DISCONC,
    MPCG_EVENT_XID0DO,
    MPCG_EVENT_XID2,
    MPCG_EVENT_XID2DONE,
    MPCG_EVENT_XID7DONE,
    MPCG_EVENT_TIMER,
    MPCG_EVENT_DOIO,
    MPCG_NR_EVENTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpcg_states {
    MPCG_STATE_RESET,
    MPCG_STATE_INOP,
    MPCG_STATE_XID2INITW,
    MPCG_STATE_XID2INITX,
    MPCG_STATE_XID7INITW,
    MPCG_STATE_XID7INITX,
    MPCG_STATE_XID0IOWAIT,
    MPCG_STATE_XID0IOWAIX,
    MPCG_STATE_XID7INITI,
    MPCG_STATE_XID7INITZ,
    MPCG_STATE_XID7INITF,
    MPCG_STATE_FLOWC,
    MPCG_STATE_READY,
    MPCG_NR_STATES,
}

// --- This is the END my friend ---
