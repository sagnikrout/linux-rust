//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/trace.h
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
// Portions of this file
// Copyright(c) 2016-2017 Intel Deutschland GmbH
// Copyright (C) 2018-2024, 2026 Intel Corporation
//

pub const MAXNAME: c_int = 32;

//
// Tracing for driver callbacks.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_vif_entry {
    pub vif_type: nl80211_iftype,
    pub p2p: bool,
    pub vif_name: [c_char; IFNAMSIZ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_chandef_entry {
    pub control_freq: u32,
    pub freq_offset: u32,
    pub chan_width: u32,
    pub center_freq1: u32,
    pub freq1_offset: u32,
    pub center_freq2: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_switch_entry {
    pub vif: trace_vif_entry,
    pub link_id: c_uint,
    pub old_chandef: trace_chandef_entry,
    pub new_chandef: trace_chandef_entry,
    pub __packed: },

    pub n_vifs: __entry->n_vifs =,
    pub mode: __entry->mode =,
    pub i: c_int,
    pub {: for (i = 0; i < n_vifs; i++),
    pub sdata: *mut ieee80211_sub_if_data,
    pub vif->type): SWITCH_ENTRY_ASSIGN(vif.vif_type,,
    pub vif->p2p): SWITCH_ENTRY_ASSIGN(vif.p2p,,
    pub link_conf->link_id): SWITCH_ENTRY_ASSIGN(link_id,,
    pub link_conf->link_id: __entry->link_id =,
    pub link_conf->link_id: __entry->link_id =,
    pub link_conf->dtim_period: __entry->dtimper =,
    pub link_conf->beacon_int: __entry->bcnint =,
    pub link_conf->hidden_ssid: __entry->hidden_ssid =,
    pub link_conf->link_id: __entry->link_id =,
    pub reconfig_type: __entry->reconfig_type =,

    pub info->dtim_period: __entry->dtimper =,
    pub info->beacon_int: __entry->bcnint =,
    pub conf->master_pref: __entry->master_pref =,
    pub conf->bands: __entry->bands =,
    pub conf->master_pref: __entry->master_pref =,
    pub conf->bands: __entry->bands =,
    pub changes: __entry->changes =,
    pub func->type: __entry->type =,
    pub func->instance_id: __entry->inst_id =,
    pub instance_id: __entry->instance_id =,
    pub key_idx: __entry->key_idx =,
    pub link_id: __entry->link_id =,
    pub dbm: __entry->dbm =,
    pub ret: __entry->ret =,
    pub oper_class: __entry->oper_class =,
    pub params->sta->tdls_initiator: __entry->peer_initiator =,
    pub params->action_code: __entry->action_code =,
    pub params->status: __entry->status =,
    pub params->timestamp: __entry->timestamp =,
    pub params->switch_time: __entry->switch_time =,
    pub params->switch_timeout: __entry->switch_timeout =,
    pub txq->txq.sta: *mut *mut ieee80211_sta sta =,
    pub txq->txq.ac: __entry->ac =,
    pub txq->txq.tid: __entry->tid =,
    pub enabled: __entry->enabled =,
    pub twt->dialog_token: __entry->dialog_token =,
    pub twt->control: __entry->control =,
    pub twt_agrt->req_type: __entry->req_type =,
    pub twt_agrt->twt: __entry->twt =,
    pub twt_agrt->min_twt_dur: __entry->duration =,
    pub twt_agrt->mantissa: __entry->mantissa =,
    pub twt_agrt->channel: __entry->channel =,
    pub flowid: __entry->flowid =,
    pub type: __entry->type =,
    pub active_links: __entry->active_links =,
    pub old_links: __entry->old_links =,
    pub new_links: __entry->new_links =,
    pub old_links: __entry->old_links =,
    pub new_links: __entry->new_links =,
//
// Tracing for API calls that drivers call.
//
    pub result: __entry->result =,
    pub tid: __entry->tid =,
    pub ETH_ALEN): memcpy(__entry->ra, ra,,
    pub tid: __entry->tid =,
    pub tid: __entry->tid =,
    pub ETH_ALEN): memcpy(__entry->ra, ra,,
    pub tid: __entry->tid =,
    pub reconnect: __entry->reconnect =,
    pub rssi_event: __entry->rssi_event =,
    pub rssi_level: __entry->rssi_level =,
    pub aborted: __entry->aborted =,
    pub block: __entry->block =,
    pub success: __entry->success =,
    pub link_id: __entry->link_id =,
    pub ETH_ALEN): memcpy(__entry->bssid, bssid,,
    pub NL80211_REPLAY_CTR_LEN): memcpy(__entry->replay_ctr, replay_ctr,,
    pub rssi_min_thold: __entry->rssi_min_thold =,
    pub rssi_max_thold: __entry->rssi_max_thold =,
    pub tid: __entry->tid =,
    pub tid: __entry->tid =,
    pub buffered: __entry->buffered =,
    pub smps_mode: __entry->smps_mode =,
    pub link_sta->link_id: __entry->link_id =,
    pub bw: __entry->bw =,
    pub link_sta->link_id: __entry->link_id =,
//
// Tracing for internal functions
// (which may also be called in response to driver calls)
//
    pub queue: __entry->queue =,
    pub reason: __entry->reason =,
    pub refcount: __entry->refcount =,
    pub queue: __entry->queue =,
    pub reason: __entry->reason =,
    pub refcount: __entry->refcount =,
    pub res: __entry->res =,
    pub type: __entry->type =,
    pub link_id: __entry->link_id =,
    pub control: __entry->control =,
    pub link_bitmap: __entry->link_bitmap =,
    pub i++): for (int i = 0; i < CFG80211_NAN_MAX_PEER_MAPS;,

