//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/llc_c_ev.h
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
// Copyright (c) 1997 by Procom Technology,Inc.
// 2001 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

// Connection component state transition event qualifiers
// Types of events (possible values in 'ev->type')
pub const LLC_CONN_EV_TYPE_SIMPLE: c_int = 1;
pub const LLC_CONN_EV_TYPE_CONDITION: c_int = 2;
pub const LLC_CONN_EV_TYPE_PRIM: c_int = 3;

pub const LLC_CONN_EV_TYPE_ACK_TMR: c_int = 5;
pub const LLC_CONN_EV_TYPE_P_TMR: c_int = 6;
pub const LLC_CONN_EV_TYPE_REJ_TMR: c_int = 7;
pub const LLC_CONN_EV_TYPE_BUSY_TMR: c_int = 8;
pub const LLC_CONN_EV_TYPE_RPT_STATUS: c_int = 9;
pub const LLC_CONN_EV_TYPE_SENDACK_TMR: c_int = 10;
pub const NBR_CONN_EV: c_int = 5;
// Connection events which cause state transitions when fully qualified
pub const LLC_CONN_EV_CONN_REQ: c_int = 1;
pub const LLC_CONN_EV_CONN_RESP: c_int = 2;
pub const LLC_CONN_EV_DATA_REQ: c_int = 3;
pub const LLC_CONN_EV_DISC_REQ: c_int = 4;
pub const LLC_CONN_EV_RESET_REQ: c_int = 5;
pub const LLC_CONN_EV_RESET_RESP: c_int = 6;
pub const LLC_CONN_EV_LOCAL_BUSY_DETECTED: c_int = 7;
pub const LLC_CONN_EV_LOCAL_BUSY_CLEARED: c_int = 8;
pub const LLC_CONN_EV_RX_BAD_PDU: c_int = 9;
pub const LLC_CONN_EV_RX_DISC_CMD_Pbit_SET_X: c_int = 10;
pub const LLC_CONN_EV_RX_DM_RSP_Fbit_SET_X: c_int = 11;
pub const LLC_CONN_EV_RX_FRMR_RSP_Fbit_SET_X: c_int = 12;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_X: c_int = 13;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_X_UNEXPD_Ns: c_int = 14;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_X_INVAL_Ns: c_int = 15;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_X: c_int = 16;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_X_UNEXPD_Ns: c_int = 17;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_X_INVAL_Ns: c_int = 18;
pub const LLC_CONN_EV_RX_REJ_CMD_Pbit_SET_X: c_int = 19;
pub const LLC_CONN_EV_RX_REJ_RSP_Fbit_SET_X: c_int = 20;
pub const LLC_CONN_EV_RX_RNR_CMD_Pbit_SET_X: c_int = 21;
pub const LLC_CONN_EV_RX_RNR_RSP_Fbit_SET_X: c_int = 22;
pub const LLC_CONN_EV_RX_RR_CMD_Pbit_SET_X: c_int = 23;
pub const LLC_CONN_EV_RX_RR_RSP_Fbit_SET_X: c_int = 24;
pub const LLC_CONN_EV_RX_SABME_CMD_Pbit_SET_X: c_int = 25;
pub const LLC_CONN_EV_RX_UA_RSP_Fbit_SET_X: c_int = 26;
pub const LLC_CONN_EV_RX_XXX_CMD_Pbit_SET_X: c_int = 27;
pub const LLC_CONN_EV_RX_XXX_RSP_Fbit_SET_X: c_int = 28;
pub const LLC_CONN_EV_RX_XXX_YYY: c_int = 29;
pub const LLC_CONN_EV_RX_ZZZ_CMD_Pbit_SET_X_INVAL_Nr: c_int = 30;
pub const LLC_CONN_EV_RX_ZZZ_RSP_Fbit_SET_X_INVAL_Nr: c_int = 31;
pub const LLC_CONN_EV_P_TMR_EXP: c_int = 32;
pub const LLC_CONN_EV_ACK_TMR_EXP: c_int = 33;
pub const LLC_CONN_EV_REJ_TMR_EXP: c_int = 34;
pub const LLC_CONN_EV_BUSY_TMR_EXP: c_int = 35;
pub const LLC_CONN_EV_RX_XXX_CMD_Pbit_SET_1: c_int = 36;
pub const LLC_CONN_EV_RX_XXX_CMD_Pbit_SET_0: c_int = 37;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_0_UNEXPD_Ns: c_int = 38;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_0_UNEXPD_Ns: c_int = 39;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_1_UNEXPD_Ns: c_int = 40;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_1_UNEXPD_Ns: c_int = 41;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_0: c_int = 42;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_0: c_int = 43;
pub const LLC_CONN_EV_RX_I_CMD_Pbit_SET_1: c_int = 44;
pub const LLC_CONN_EV_RX_RR_CMD_Pbit_SET_0: c_int = 45;
pub const LLC_CONN_EV_RX_RR_RSP_Fbit_SET_0: c_int = 46;
pub const LLC_CONN_EV_RX_RR_RSP_Fbit_SET_1: c_int = 47;
pub const LLC_CONN_EV_RX_RR_CMD_Pbit_SET_1: c_int = 48;
pub const LLC_CONN_EV_RX_RNR_CMD_Pbit_SET_0: c_int = 49;
pub const LLC_CONN_EV_RX_RNR_RSP_Fbit_SET_0: c_int = 50;
pub const LLC_CONN_EV_RX_RNR_RSP_Fbit_SET_1: c_int = 51;
pub const LLC_CONN_EV_RX_RNR_CMD_Pbit_SET_1: c_int = 52;
pub const LLC_CONN_EV_RX_REJ_CMD_Pbit_SET_0: c_int = 53;
pub const LLC_CONN_EV_RX_REJ_RSP_Fbit_SET_0: c_int = 54;
pub const LLC_CONN_EV_RX_REJ_CMD_Pbit_SET_1: c_int = 55;
pub const LLC_CONN_EV_RX_I_RSP_Fbit_SET_1: c_int = 56;
pub const LLC_CONN_EV_RX_REJ_RSP_Fbit_SET_1: c_int = 57;
pub const LLC_CONN_EV_RX_XXX_RSP_Fbit_SET_1: c_int = 58;
pub const LLC_CONN_EV_TX_BUFF_FULL: c_int = 59;
pub const LLC_CONN_EV_INIT_P_F_CYCLE: c_int = 100;
//
// Connection event qualifiers; for some events a certain combination of
// these qualifiers must be TRUE before event recognized valid for state;
// these constants act as indexes into the Event Qualifier function
// table
//
pub const LLC_CONN_EV_QFY_DATA_FLAG_EQ_1: c_int = 1;
pub const LLC_CONN_EV_QFY_DATA_FLAG_EQ_0: c_int = 2;
pub const LLC_CONN_EV_QFY_DATA_FLAG_EQ_2: c_int = 3;
pub const LLC_CONN_EV_QFY_P_FLAG_EQ_1: c_int = 4;
pub const LLC_CONN_EV_QFY_P_FLAG_EQ_0: c_int = 5;
pub const LLC_CONN_EV_QFY_P_FLAG_EQ_Fbit: c_int = 6;
pub const LLC_CONN_EV_QFY_REMOTE_BUSY_EQ_0: c_int = 7;
pub const LLC_CONN_EV_QFY_RETRY_CNT_LT_N2: c_int = 8;
pub const LLC_CONN_EV_QFY_RETRY_CNT_GTE_N2: c_int = 9;
pub const LLC_CONN_EV_QFY_S_FLAG_EQ_1: c_int = 10;
pub const LLC_CONN_EV_QFY_S_FLAG_EQ_0: c_int = 11;
pub const LLC_CONN_EV_QFY_INIT_P_F_CYCLE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_conn_state_ev {
    pub type: u8,
    pub prim: u8,
    pub prim_type: u8,
    pub reason: u8,
    pub status: u8,
    pub ind_prim: u8,
    pub cfm_prim: u8,
}

extern "C" {
    pub fn int(sk: *mut *mut llc_conn_ev_t)(struct sock, skb: *mut sk_buff) -> typedef;
}
extern "C" {
    pub fn int(sk: *mut *mut llc_conn_ev_qfyr_t)(struct sock, skb: *mut sk_buff) -> typedef;
}
extern "C" {
    pub fn llc_conn_ev_conn_req(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_data_req(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_disc_req(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rst_req(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_local_busy_detected(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_local_busy_cleared(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_bad_pdu(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_disc_cmd_pbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_dm_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_frmr_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_i_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rej_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_sabme_cmd_pbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_ua_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_xxx_cmd_pbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_xxx_rsp_fbit_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_p_tmr_exp(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_ack_tmr_exp(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rej_tmr_exp(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_busy_tmr_exp(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
// NOT_USED functions and their variations
extern "C" {
    pub fn llc_conn_ev_rx_xxx_cmd_pbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_xxx_rsp_fbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_i_cmd_pbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_i_cmd_pbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_i_rsp_fbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_i_rsp_fbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rr_cmd_pbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rr_cmd_pbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rr_rsp_fbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rr_rsp_fbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rnr_cmd_pbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rnr_cmd_pbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rnr_rsp_fbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rnr_rsp_fbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rej_cmd_pbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rej_cmd_pbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rej_rsp_fbit_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_rej_rsp_fbit_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_rx_any_frame(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_tx_buffer_full(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_init_p_f_cycle(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
// Available connection action qualifiers
extern "C" {
    pub fn llc_conn_ev_qlfy_data_flag_eq_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_data_flag_eq_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_data_flag_eq_2(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_p_flag_eq_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_last_frame_eq_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_last_frame_eq_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_p_flag_eq_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_p_flag_eq_f(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_remote_busy_eq_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_remote_busy_eq_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_retry_cnt_lt_n2(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_retry_cnt_gte_n2(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_s_flag_eq_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_s_flag_eq_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_cause_flag_eq_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_cause_flag_eq_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_set_status_conn(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_set_status_disc(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_set_status_failed(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_set_status_refuse(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_set_status_conflict(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ev_qlfy_set_status_rst_done(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
