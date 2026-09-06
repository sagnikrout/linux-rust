//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfad_bsg.h
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
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014- QLogic Corporation.
// All rights reserved
// www.qlogic.com
//
// Linux driver for QLogic BR-series Fibre Channel Host Bus Adapter.
//

// Definitions of vendor unique structures and command codes passed in
// using FC_BSG_HST_VENDOR message code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_gen_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_portlogctl_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub ctl: bfa_boolean_t,
    pub inst_no: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fcpim_profile_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_itnim_ioprofile_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub lpwwn: wwn_t,
    pub rpwwn: wwn_t,
    pub ioprofile: bfa_itnim_ioprofile_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fcport_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub stats: bfa_fcport_stats_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_ioc_name_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub name: [c_char; BFA_ADAPTER_SYM_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_ioc_info_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub serialnum: [c_char; 64],
    pub hwpath: [c_char; BFA_STRING_32],
    pub adapter_hwpath: [c_char; BFA_STRING_32],
    pub guid: [*mut c_char; BFA_ADAPTER_SYM_NAME_LEN*2],
    pub name: [c_char; BFA_ADAPTER_SYM_NAME_LEN],
    pub port_name: [c_char; BFA_ADAPTER_SYM_NAME_LEN],
    pub eth_name: [c_char; BFA_ADAPTER_SYM_NAME_LEN],
    pub pwwn: wwn_t,
    pub nwwn: wwn_t,
    pub factorypwwn: wwn_t,
    pub factorynwwn: wwn_t,
    pub mac: mac_t,
    pub /: *mut *mut mac_t factory_mac; / Factory mac address,
    pub /: *mut *mut mac_t current_mac; / Currently assigned mac address,
    pub ioc_type: bfa_ioc_type_e,
    pub /: *mut *mut u16 pvid; / Port vlan id,
    pub rsvd1: u16,
    pub host: u32,
    pub /: *mut *mut u32 bandwidth; / For PF support,
    pub rsvd2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_ioc_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub ioc_attr: bfa_ioc_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_ioc_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub ioc_stats: bfa_ioc_stats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_ioc_fwstats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub buf_size: u32,
    pub rsvd1: u32,
    pub buf_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_iocfc_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub iocfc_attr: bfa_iocfc_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_iocfc_intr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub attr: bfa_iocfc_intr_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_port_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub attr: bfa_port_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_port_cfg_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub param: u32,
    pub rsvd1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_port_cfg_maxfrsize_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub maxfrsize: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_port_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub buf_size: u32,
    pub rsvd1: u32,
    pub buf_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_lport_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub pwwn: wwn_t,
    pub port_attr: bfa_lport_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_lport_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub pwwn: wwn_t,
    pub port_stats: bfa_lport_stats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_lport_iostats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub pwwn: wwn_t,
    pub iostats: bfa_itnim_iostats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_lport_get_rports_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub pwwn: wwn_t,
    pub rbuf_ptr: u64,
    pub nrports: u32,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_rport_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub pwwn: wwn_t,
    pub rpwwn: wwn_t,
    pub pid: u32,
    pub rsvd: u32,
    pub attr: bfa_rport_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_rport_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub pwwn: wwn_t,
    pub rpwwn: wwn_t,
    pub stats: bfa_rport_stats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_rport_scsi_addr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub pwwn: wwn_t,
    pub rpwwn: wwn_t,
    pub host: u32,
    pub bus: u32,
    pub target: u32,
    pub lun: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_rport_reset_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub pwwn: wwn_t,
    pub rpwwn: wwn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_rport_set_speed_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub speed: bfa_port_speed,
    pub rsvd: u32,
    pub pwwn: wwn_t,
    pub rpwwn: wwn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_vport_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub vpwwn: wwn_t,
    pub vport_attr: bfa_vport_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_vport_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub vpwwn: wwn_t,
    pub vport_stats: bfa_vport_stats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_reset_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub vpwwn: wwn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fabric_get_lports_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub buf_ptr: u64,
    pub nports: u32,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_trl_speed_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub speed: bfa_port_speed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fcpim_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub param: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fcpim_modstats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub modstats: bfa_itnim_iostats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fcpim_del_itn_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub modstats: bfa_fcpim_del_itn_stats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fcpim_modstatsclr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_itnim_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub lpwwn: wwn_t,
    pub rpwwn: wwn_t,
    pub attr: bfa_itnim_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_itnim_iostats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub lpwwn: wwn_t,
    pub rpwwn: wwn_t,
    pub iostats: bfa_itnim_iostats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_itnim_itnstats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub lpwwn: wwn_t,
    pub rpwwn: wwn_t,
    pub itnstats: bfa_itnim_stats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_pcifn_cfg_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub pcifn_cfg: bfa_ablk_cfg_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_pcifn_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub pcifn_id: u16,
    pub bw_min: u16,
    pub bw_max: u16,
    pub port: u8,
    pub pcifn_class: bfi_pcifn_class,
    pub rsvd: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_adapter_cfg_mode_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub cfg: bfa_adapter_cfg_mode_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_port_cfg_mode_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub instance: u16,
    pub cfg: bfa_port_cfg_mode_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_bbcr_enable_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub bb_scn: u8,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_bbcr_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub attr: bfa_bbcr_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_faa_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub faa_attr: bfa_faa_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_cee_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub buf_size: u32,
    pub rsvd1: u32,
    pub buf_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_cee_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub buf_size: u32,
    pub rsvd1: u32,
    pub buf_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_sfp_media_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub media: bfa_defs_sfp_media_e,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_sfp_speed_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub speed: bfa_port_speed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_flash_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub attr: bfa_flash_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_flash_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub instance: u8,
    pub rsvd: u8,
    pub type: bfa_flash_part_type,
    pub bufsz: c_int,
    pub buf_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_diag_get_temp_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub result: bfa_diag_results_tempsensor_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_diag_memtest_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: [u16; 3],
    pub pat: u32,
    pub result: bfa_diag_memtest_result,
    pub memtest: bfa_diag_memtest_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_diag_loopback_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub opmode: bfa_port_opmode,
    pub speed: bfa_port_speed,
    pub lpcnt: u32,
    pub pat: u32,
    pub result: bfa_diag_loopback_result_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_diag_dport_show_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub result: bfa_diag_dport_result_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_dport_enable_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub lpcnt: u16,
    pub pat: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_diag_fwping_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub cnt: u32,
    pub pattern: u32,
    pub result: bfa_diag_results_fwping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_diag_qtest_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub force: u32,
    pub queue: u32,
    pub result: bfa_diag_qtest_result_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_sfp_show_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub sfp: sfp_mem_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_diag_led_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub ledtest: bfa_diag_ledtest_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_diag_beacon_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub beacon: bfa_boolean_t,
    pub link_e2e_beacon: bfa_boolean_t,
    pub second: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_diag_lb_stat_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_phy_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub instance: u16,
    pub attr: bfa_phy_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_phy_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub instance: u16,
    pub bufsz: u64,
    pub buf_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_debug_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub bufsz: u32,
    pub inst_no: c_int,
    pub buf_ptr: u64,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_phy_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub instance: u16,
    pub stats: bfa_phy_stats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_vhba_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub pcifn_id: u16,
    pub attr: bfa_vhba_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_boot_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub cfg: bfa_boot_cfg_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_preboot_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub cfg: bfa_boot_pbc_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_ethboot_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub cfg: bfa_ethboot_cfg_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_trunk_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub attr: bfa_trunk_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_qos_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub attr: bfa_qos_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_qos_vc_attr_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub attr: bfa_qos_vc_attr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_qos_bw_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub qos_bw: bfa_qos_bw_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_vf_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub stats: bfa_vf_stats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_vf_reset_stats_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fcpim_lunmask_query_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub lun_mask: bfa_lunmask_cfg_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fcpim_lunmask_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub pwwn: wwn_t,
    pub rpwwn: wwn_t,
    pub lun: scsi_lun,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fcpim_throttle_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub vf_id: u16,
    pub throttle: bfa_defs_fcpim_throttle_s,
}

pub const BFA_TFRU_DATA_SIZE: c_int = 64;
pub const BFA_MAX_FRUVPD_TRANSFER_SIZE: c_uint = 0x1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_tfru_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub offset: u32,
    pub len: u32,
    pub data: [u8; BFA_TFRU_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fruvpd_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd1: u16,
    pub offset: u32,
    pub len: u32,
    pub data: [u8; BFA_MAX_FRUVPD_TRANSFER_SIZE],
    pub trfr_cmpl: u8,
    pub rsvd2: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fruvpd_max_size_s {
    pub status: bfa_status_t,
    pub bfad_num: u16,
    pub rsvd: u16,
    pub max_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_fcpt_s {
    pub status: bfa_status_t,
    pub vf_id: u16,
    pub lpwwn: wwn_t,
    pub dpwwn: wwn_t,
    pub tsecs: u32,
    pub cts: c_int,
    pub cos: fc_cos,
    pub fchs: fchs_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bsg_data {
    pub payload_len: c_int,
    pub payload: u64,
}

