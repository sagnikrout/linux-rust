//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl2/hw_atl2_llh.h
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
// Atlantic Network Driver
// Copyright (C) 2020 Marvell International Ltd.
//

// Get Enable usage of extended tags from 32-255.
extern "C" {
    pub fn hw_atl2_phi_ext_tag_get(aq_hw: *mut aq_hw_s) -> u32;
}
// Set TX Interrupt Moderation Control Register
// Set Redirection Table 2 Select
// Set RSS HASH type
extern "C" {
    pub fn hw_atl2_rpf_rss_hash_type_set(aq_hw: *mut aq_hw_s, rss_hash_type: u32);
}
// set new RPF enable
extern "C" {
    pub fn hw_atl2_rpf_new_enable_set(aq_hw: *mut aq_hw_s, enable: u32);
}
// set l2 unicast filter tag
extern "C" {
    pub fn hw_atl2_rpfl2_uc_flr_tag_set(aq_hw: *mut aq_hw_s, tag: u32, filter: u32);
}
// set l2 broadcast filter tag
extern "C" {
    pub fn hw_atl2_rpfl2_bc_flr_tag_set(aq_hw: *mut aq_hw_s, tag: u32);
}
// set new rss redirection table
// Set VLAN filter tag
extern "C" {
    pub fn hw_atl2_rpf_vlan_flr_tag_set(aq_hw: *mut aq_hw_s, tag: u32, filter: u32);
}
// set ethertype filter tag
extern "C" {
    pub fn hw_atl2_rpf_etht_flr_tag_set(aq_hw: *mut aq_hw_s, tag: u32, filter: u32);
}
// get ethertype filter tag
extern "C" {
    pub fn hw_atl2_rpf_etht_flr_tag_get(aq_hw: *mut aq_hw_s, filter: u32) -> u32;
}
// set L3 v4 dest address
// set L3 v4 src address
extern "C" {
    pub fn hw_atl2_rpf_l3_v4_src_addr_set(aq_hw: *mut aq_hw_s, filter: u32, val: u32);
}
// set L3 v4 cmd
extern "C" {
    pub fn hw_atl2_rpf_l3_v4_cmd_set(aq_hw: *mut aq_hw_s, val: u32, filter: u32);
}
// set L3 v6 cmd
extern "C" {
    pub fn hw_atl2_rpf_l3_v6_cmd_set(aq_hw: *mut aq_hw_s, val: u32, filter: u32);
}
// set L3 v6 dest address
// set L3 v6 src address
// set L3 v6 v4 select
extern "C" {
    pub fn hw_atl2_rpf_l3_v6_v4_select_set(aq_hw: *mut aq_hw_s, val: u32);
}
// set L3 v4 tag
extern "C" {
    pub fn hw_atl2_rpf_l3_v4_tag_set(aq_hw: *mut aq_hw_s, val: u32, filter: u32);
}
// set L3 v6 tag
extern "C" {
    pub fn hw_atl2_rpf_l3_v6_tag_set(aq_hw: *mut aq_hw_s, val: u32, filter: u32);
}
// set L4 cmd
extern "C" {
    pub fn hw_atl2_rpf_l4_cmd_set(aq_hw: *mut aq_hw_s, val: u32, filter: u32);
}
// set L4 tag
extern "C" {
    pub fn hw_atl2_rpf_l4_tag_set(aq_hw: *mut aq_hw_s, val: u32, filter: u32);
}
// set tx random TC-queue mapping enable bit
// set tx buffer clock gate enable
extern "C" {
    pub fn hw_atl2_tpb_tx_buf_clk_gate_en_set(aq_hw: *mut aq_hw_s, clk_gate_en: u32);
}
// tsg
extern "C" {
    pub fn hw_atl2_tsg_clock_reset(aq_hw: *mut aq_hw_s, clock_sel: u32);
}
extern "C" {
    pub fn hw_atl2_tsg_clock_read(aq_hw: *mut aq_hw_s, clock_sel: u32) -> u64;
}
extern "C" {
    pub fn hw_atl2_tsg_gpio_clear_status(aq_hw: *mut aq_hw_s);
}
// Set Rx Descriptor0 Timestamp request
// Set Tx Descriptor Timestamp writeback Enable
// Set Tx Descriptor Timestamp enable
// set tx packet scheduler tc data max credit
// set tx packet scheduler tc data weight
// Set Tx Descriptor AVB enable
extern "C" {
    pub fn hw_atl2_tdm_tx_data_read_req_limit_set(aq_hw: *mut aq_hw_s, limit: u32);
}
extern "C" {
    pub fn hw_atl2_tdm_tx_desc_read_req_limit_set(aq_hw: *mut aq_hw_s, limit: u32);
}
extern "C" {
    pub fn hw_atl2_get_hw_version(aq_hw: *mut aq_hw_s) -> u32;
}
extern "C" {
    pub fn hw_atl2_init_launchtime(aq_hw: *mut aq_hw_s);
}
// set action resolver record
// set enable action resolver section
extern "C" {
    pub fn hw_atl2_rpf_act_rslvr_section_en_set(aq_hw: *mut aq_hw_s, sections: u32);
}
// get enable action resolver section
extern "C" {
    pub fn hw_atl2_rpf_act_rslvr_section_en_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get data from firmware shared input buffer
// set data into firmware shared input buffer
// get data from firmware shared output buffer
// set host finished write shared buffer indication
extern "C" {
    pub fn hw_atl2_mif_host_finished_write_set(aq_hw: *mut aq_hw_s, finish: u32);
}
// get mcp finished read shared buffer indication
extern "C" {
    pub fn hw_atl2_mif_mcp_finished_read_get(aq_hw: *mut aq_hw_s) -> u32;
}
// get mcp boot register
extern "C" {
    pub fn hw_atl2_mif_mcp_boot_reg_get(aq_hw: *mut aq_hw_s) -> u32;
}
// set mcp boot register
extern "C" {
    pub fn hw_atl2_mif_mcp_boot_reg_set(aq_hw: *mut aq_hw_s, val: u32);
}
// get host interrupt request
extern "C" {
    pub fn hw_atl2_mif_host_req_int_get(aq_hw: *mut aq_hw_s) -> u32;
}
// clear host interrupt request
extern "C" {
    pub fn hw_atl2_mif_host_req_int_clr(aq_hw: *mut aq_hw_s, val: u32);
}
// Set GPIO Special Mode
