//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/llc_c_ac.h
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
// Connection component state transition actions
//
// Connection state transition actions
// (Fb = F bit; Pb = P bit; Xb = X bit)
//

pub const LLC_CONN_AC_CLR_REMOTE_BUSY: c_int = 1;
pub const LLC_CONN_AC_CONN_IND: c_int = 2;
pub const LLC_CONN_AC_CONN_CONFIRM: c_int = 3;
pub const LLC_CONN_AC_DATA_IND: c_int = 4;
pub const LLC_CONN_AC_DISC_IND: c_int = 5;
pub const LLC_CONN_AC_RESET_IND: c_int = 6;
pub const LLC_CONN_AC_RESET_CONFIRM: c_int = 7;
pub const LLC_CONN_AC_REPORT_STATUS: c_int = 8;
pub const LLC_CONN_AC_CLR_REMOTE_BUSY_IF_Fb_EQ_1: c_int = 9;
pub const LLC_CONN_AC_STOP_REJ_TMR_IF_DATA_FLAG_EQ_2: c_int = 10;
pub const LLC_CONN_AC_SEND_DISC_CMD_Pb_SET_X: c_int = 11;
pub const LLC_CONN_AC_SEND_DM_RSP_Fb_SET_Pb: c_int = 12;
pub const LLC_CONN_AC_SEND_DM_RSP_Fb_SET_1: c_int = 13;
pub const LLC_CONN_AC_SEND_DM_RSP_Fb_SET_F_FLAG: c_int = 14;
pub const LLC_CONN_AC_SEND_FRMR_RSP_Fb_SET_X: c_int = 15;
pub const LLC_CONN_AC_RESEND_FRMR_RSP_Fb_SET_0: c_int = 16;
pub const LLC_CONN_AC_RESEND_FRMR_RSP_Fb_SET_Pb: c_int = 17;
pub const LLC_CONN_AC_SEND_I_CMD_Pb_SET_1: c_int = 18;
pub const LLC_CONN_AC_RESEND_I_CMD_Pb_SET_1: c_int = 19;
pub const LLC_CONN_AC_RESEND_I_CMD_Pb_SET_1_OR_SEND_RR: c_int = 20;
pub const LLC_CONN_AC_SEND_I_XXX_Xb_SET_0: c_int = 21;
pub const LLC_CONN_AC_RESEND_I_XXX_Xb_SET_0: c_int = 22;
pub const LLC_CONN_AC_RESEND_I_XXX_Xb_SET_0_OR_SEND_RR: c_int = 23;
pub const LLC_CONN_AC_RESEND_I_RSP_Fb_SET_1: c_int = 24;
pub const LLC_CONN_AC_SEND_REJ_CMD_Pb_SET_1: c_int = 25;
pub const LLC_CONN_AC_SEND_REJ_RSP_Fb_SET_1: c_int = 26;
pub const LLC_CONN_AC_SEND_REJ_XXX_Xb_SET_0: c_int = 27;
pub const LLC_CONN_AC_SEND_RNR_CMD_Pb_SET_1: c_int = 28;
pub const LLC_CONN_AC_SEND_RNR_RSP_Fb_SET_1: c_int = 29;
pub const LLC_CONN_AC_SEND_RNR_XXX_Xb_SET_0: c_int = 30;
pub const LLC_CONN_AC_SET_REMOTE_BUSY: c_int = 31;
pub const LLC_CONN_AC_OPTIONAL_SEND_RNR_XXX_Xb_SET_0: c_int = 32;
pub const LLC_CONN_AC_SEND_RR_CMD_Pb_SET_1: c_int = 33;
pub const LLC_CONN_AC_SEND_ACK_CMD_Pb_SET_1: c_int = 34;
pub const LLC_CONN_AC_SEND_RR_RSP_Fb_SET_1: c_int = 35;
pub const LLC_CONN_AC_SEND_ACK_RSP_Fb_SET_1: c_int = 36;
pub const LLC_CONN_AC_SEND_RR_XXX_Xb_SET_0: c_int = 37;
pub const LLC_CONN_AC_SEND_ACK_XXX_Xb_SET_0: c_int = 38;
pub const LLC_CONN_AC_SEND_SABME_CMD_Pb_SET_X: c_int = 39;
pub const LLC_CONN_AC_SEND_UA_RSP_Fb_SET_Pb: c_int = 40;
pub const LLC_CONN_AC_SEND_UA_RSP_Fb_SET_F_FLAG: c_int = 41;
pub const LLC_CONN_AC_S_FLAG_SET_0: c_int = 42;
pub const LLC_CONN_AC_S_FLAG_SET_1: c_int = 43;
pub const LLC_CONN_AC_START_P_TMR: c_int = 44;
pub const LLC_CONN_AC_START_ACK_TMR: c_int = 45;
pub const LLC_CONN_AC_START_REJ_TMR: c_int = 46;
pub const LLC_CONN_AC_START_ACK_TMR_IF_NOT_RUNNING: c_int = 47;
pub const LLC_CONN_AC_STOP_ACK_TMR: c_int = 48;
pub const LLC_CONN_AC_STOP_P_TMR: c_int = 49;
pub const LLC_CONN_AC_STOP_REJ_TMR: c_int = 50;
pub const LLC_CONN_AC_STOP_ALL_TMRS: c_int = 51;
pub const LLC_CONN_AC_STOP_OTHER_TMRS: c_int = 52;
pub const LLC_CONN_AC_UPDATE_Nr_RECEIVED: c_int = 53;
pub const LLC_CONN_AC_UPDATE_P_FLAG: c_int = 54;
pub const LLC_CONN_AC_DATA_FLAG_SET_2: c_int = 55;
pub const LLC_CONN_AC_DATA_FLAG_SET_0: c_int = 56;
pub const LLC_CONN_AC_DATA_FLAG_SET_1: c_int = 57;
pub const LLC_CONN_AC_DATA_FLAG_SET_1_IF_DATA_FLAG_EQ_0: c_int = 58;
pub const LLC_CONN_AC_P_FLAG_SET_0: c_int = 59;
pub const LLC_CONN_AC_P_FLAG_SET_P: c_int = 60;
pub const LLC_CONN_AC_REMOTE_BUSY_SET_0: c_int = 61;
pub const LLC_CONN_AC_RETRY_CNT_SET_0: c_int = 62;
pub const LLC_CONN_AC_RETRY_CNT_INC_BY_1: c_int = 63;
pub const LLC_CONN_AC_Vr_SET_0: c_int = 64;
pub const LLC_CONN_AC_Vr_INC_BY_1: c_int = 65;
pub const LLC_CONN_AC_Vs_SET_0: c_int = 66;
pub const LLC_CONN_AC_Vs_SET_Nr: c_int = 67;
pub const LLC_CONN_AC_F_FLAG_SET_P: c_int = 68;
pub const LLC_CONN_AC_STOP_SENDACK_TMR: c_int = 70;
pub const LLC_CONN_AC_START_SENDACK_TMR_IF_NOT_RUNNING: c_int = 71;
extern "C" {
    pub fn int(sk: *mut *mut llc_conn_action_t)(struct sock, skb: *mut sk_buff) -> typedef;
}
extern "C" {
    pub fn llc_conn_ac_clear_remote_busy(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_conn_ind(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_conn_confirm(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_data_ind(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_disc_ind(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_rst_ind(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_rst_confirm(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_disc_cmd_p_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_dm_rsp_f_set_p(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_dm_rsp_f_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_frmr_rsp_f_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_resend_frmr_rsp_f_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_resend_frmr_rsp_f_set_p(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_i_cmd_p_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_i_xxx_x_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_resend_i_xxx_x_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_resend_i_rsp_f_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_rej_cmd_p_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_rej_rsp_f_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_rej_xxx_x_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_rnr_cmd_p_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_rnr_rsp_f_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_rnr_xxx_x_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_remote_busy(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_opt_send_rnr_xxx_x_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_rr_cmd_p_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_rr_rsp_f_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_ack_rsp_f_set_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_rr_xxx_x_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_ack_xxx_x_set_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_sabme_cmd_p_set_x(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_ua_rsp_f_set_p(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_s_flag_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_s_flag_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_start_p_timer(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_start_ack_timer(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_start_rej_timer(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_stop_ack_timer(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_stop_p_timer(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_stop_rej_timer(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_stop_all_timers(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_stop_other_timers(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_upd_nr_received(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_inc_tx_win_size(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_dec_tx_win_size(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_upd_p_flag(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_data_flag_2(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_data_flag_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_data_flag_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_p_flag_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_remote_busy_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_retry_cnt_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_cause_flag_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_cause_flag_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_inc_retry_cnt_by_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_vr_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_inc_vr_by_1(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_vs_0(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_set_vs_nr(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_rst_vs(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_upd_vs(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_disc(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_reset(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_disc_confirm(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_circular_between(a: u8, b: u8, c: u8) -> u8;
}
extern "C" {
    pub fn llc_conn_ac_send_ack_if_needed(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_adjust_npta_by_rr(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_adjust_npta_by_rnr(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_rst_sendack_flag(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_ac_send_i_as_ack(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn llc_conn_busy_tmr_cb(t: *mut timer_list);
}
extern "C" {
    pub fn llc_conn_pf_cycle_tmr_cb(t: *mut timer_list);
}
extern "C" {
    pub fn llc_conn_ack_tmr_cb(t: *mut timer_list);
}
extern "C" {
    pub fn llc_conn_rej_tmr_cb(t: *mut timer_list);
}
extern "C" {
    pub fn llc_conn_set_p_flag(sk: *mut sock, value: u8);
}
