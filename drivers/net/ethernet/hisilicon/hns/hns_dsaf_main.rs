//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns/hns_dsaf_main.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2014-2015 Hisilicon Limited.
//

pub const HNS_DSAF_DEBUG_NW_REG_OFFSET: c_uint = 0x100000;

pub const DSAF_DEFAUTL_QUEUE_NUM_PER_PPE: c_int = 22;
pub const HNS_DSAF_MAX_DESC_CNT: c_int = 1024;
pub const HNS_DSAF_MIN_DESC_CNT: c_int = 16;
pub const DSAF_INVALID_ENTRY_IDX: c_uint = 0xffff;
pub const DSAF_CFG_READ_CNT: c_int = 30;
pub const DSAF_DUMP_REGS_NUM: c_int = 504;
pub const DSAF_STATIC_NUM: c_int = 28;
pub const DSAF_V2_STATIC_NUM: c_int = 44;
pub const DSAF_PRIO_NR: c_int = 8;
pub const DSAF_REG_PER_ZONE: c_int = 3;
pub const DSAF_ROCE_CREDIT_CHN: c_int = 8;
pub const DSAF_ROCE_CHAN_MODE: c_int = 3;
pub const HNS_MAX_WAIT_CNT: c_int = 10000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_dsaf_mode {
    HRD_DSAF_NO_DSAF_MODE	= 0x0,
    HRD_DSAF_MODE		= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hal_dsaf_tc_mode {
    HRD_DSAF_4TC_MODE		= 0X0,
    HRD_DSAF_8TC_MODE		= 0X1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_vm_def_vlan {
    pub vm_def_vlan_id: u32,
    pub vm_def_vlan_cfi: u32,
    pub vm_def_vlan_pri: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_tbl_tcam_data {
    pub tbl_tcam_data_high: u32,
    pub tbl_tcam_data_low: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_tbl_tcam_mcast_cfg {
    pub tbl_mcast_old_en: u8,
    pub tbl_mcast_item_vld: u8,
    pub tbl_mcast_port_msk: [u32; DSAF_PORT_MSK_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_tbl_tcam_ucast_cfg {
    pub tbl_ucast_old_en: u32,
    pub tbl_ucast_item_vld: u32,
    pub tbl_ucast_mac_discard: u32,
    pub tbl_ucast_dvc: u32,
    pub tbl_ucast_out_port: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_tbl_line_cfg {
    pub tbl_line_mac_discard: u32,
    pub tbl_line_dvc: u32,
    pub tbl_line_out_port: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsaf_port_rate_mode {
    DSAF_PORT_RATE_1000 = 0,
    DSAF_PORT_RATE_2500,
    DSAF_PORT_RATE_10000
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsaf_stp_port_type {
    DSAF_STP_PORT_TYPE_DISCARD = 0,
    DSAF_STP_PORT_TYPE_BLOCK = 1,
    DSAF_STP_PORT_TYPE_LISTEN = 2,
    DSAF_STP_PORT_TYPE_LEARN = 3,
    DSAF_STP_PORT_TYPE_FORWARD = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsaf_sw_port_type {
    DSAF_SW_PORT_TYPE_NON_VLAN = 0,
    DSAF_SW_PORT_TYPE_ACCESS = 1,
    DSAF_SW_PORT_TYPE_TRUNK = 2,
}

// dsaf mode define
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsaf_mode {
    DSAF_MODE_INVALID = 0,	/**< Invalid dsaf mode */
    DSAF_MODE_ENABLE_FIX,	/**< en DSAF-mode, fixed to queue*/
    DSAF_MODE_ENABLE_0VM,	/**< en DSAF-mode, support 0 VM */
    DSAF_MODE_ENABLE_8VM,	/**< en DSAF-mode, support 8 VM */
    DSAF_MODE_ENABLE_16VM,	/**< en DSAF-mode, support 16 VM */
    DSAF_MODE_ENABLE_32VM,	/**< en DSAF-mode, support 32 VM */
    DSAF_MODE_ENABLE_128VM,	/**< en DSAF-mode, support 128 VM */
    DSAF_MODE_ENABLE,		/**< before is enable DSAF mode*/
    DSAF_MODE_DISABLE_SP,	/* <non-dsaf, single port mode */
    DSAF_MODE_DISABLE_FIX,	/**< non-dasf, fixed to queue*/
    DSAF_MODE_DISABLE_2PORT_8VM,	/**< non-dasf, 2port 8VM */
    DSAF_MODE_DISABLE_2PORT_16VM,	/**< non-dasf, 2port 16VM */
    DSAF_MODE_DISABLE_2PORT_64VM,	/**< non-dasf, 2port 64VM */
    DSAF_MODE_DISABLE_6PORT_0VM,	/**< non-dasf, 6port 0VM */
    DSAF_MODE_DISABLE_6PORT_2VM,	/**< non-dasf, 6port 2VM */
    DSAF_MODE_DISABLE_6PORT_4VM,	/**< non-dasf, 6port 4VM */
    DSAF_MODE_DISABLE_6PORT_16VM,	/**< non-dasf, 6port 16VM */
    DSAF_MODE_MAX		/**< the last one, use as the num */
}

// mac entry, mc or uc entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_drv_mac_single_dest_entry {
// mac addr, match the entry
    pub addr: [u8; ETH_ALEN],
    pub /: *mut *mut u16 in_vlan_id; / value of VlanId,
// the vld input port num, dsaf-mode fix 0,
// non-dasf is the entry whitch port vld
    pub in_port_num: u8,
    pub num*/: *mut *mut u8 port_num; /output port,
    pub rsv: [u8; 6],
}

// only mc entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_drv_mac_multi_dest_entry {
// mac addr, match the entry
    pub addr: [u8; ETH_ALEN],
    pub in_vlan_id: u16,
// this mac addr output port,
// bit0-bit5 means Port0-Port5(1bit is vld)
    pub DSAF_WORD_BIT_CNT]: u32 port_mask[DSAF_DEST_PORT_NUM /,
// the vld input port num, dsaf-mode fix 0,
// non-dasf is the entry whitch port vld
    pub in_port_num: u8,
    pub rsv: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_hw_stats {
    pub pad_drop: u64,
    pub man_pkts: u64,
    pub rx_pkts: u64,
    pub rx_pkt_id: u64,
    pub rx_pause_frame: u64,
    pub release_buf_num: u64,
    pub sbm_drop: u64,
    pub crc_false: u64,
    pub bp_drop: u64,
    pub rslt_drop: u64,
    pub local_addr_false: u64,
    pub vlan_drop: u64,
    pub stp_drop: u64,
    pub rx_pfc: [u64; DSAF_PRIO_NR],
    pub tx_pfc: [u64; DSAF_PRIO_NR],
    pub tx_pkts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hnae_vf_cb {
    pub port_index: u8,
    pub mac_cb: *mut hns_mac_cb,
    pub dsaf_dev: *mut dsaf_device,
    pub /: *mut *mut hnae_handle ae_handle; / must be the last member,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_int_xge_src {
    pub xid_xge_ecc_err_int_src: u32,
    pub xid_xge_fsm_timout_int_src: u32,
    pub sbm_xge_lnk_fsm_timout_int_src: u32,
    pub sbm_xge_lnk_ecc_2bit_int_src: u32,
    pub sbm_xge_mib_req_failed_int_src: u32,
    pub sbm_xge_mib_req_fsm_timout_int_src: u32,
    pub sbm_xge_mib_rels_fsm_timout_int_src: u32,
    pub sbm_xge_sram_ecc_2bit_int_src: u32,
    pub sbm_xge_mib_buf_sum_err_int_src: u32,
    pub sbm_xge_mib_req_extra_int_src: u32,
    pub sbm_xge_mib_rels_extra_int_src: u32,
    pub voq_xge_start_to_over_0_int_src: u32,
    pub voq_xge_start_to_over_1_int_src: u32,
    pub voq_xge_ecc_err_int_src: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_int_ppe_src {
    pub xid_ppe_fsm_timout_int_src: u32,
    pub sbm_ppe_lnk_fsm_timout_int_src: u32,
    pub sbm_ppe_lnk_ecc_2bit_int_src: u32,
    pub sbm_ppe_mib_req_failed_int_src: u32,
    pub sbm_ppe_mib_req_fsm_timout_int_src: u32,
    pub sbm_ppe_mib_rels_fsm_timout_int_src: u32,
    pub sbm_ppe_sram_ecc_2bit_int_src: u32,
    pub sbm_ppe_mib_buf_sum_err_int_src: u32,
    pub sbm_ppe_mib_req_extra_int_src: u32,
    pub sbm_ppe_mib_rels_extra_int_src: u32,
    pub voq_ppe_start_to_over_0_int_src: u32,
    pub voq_ppe_ecc_err_int_src: u32,
    pub xod_ppe_fifo_rd_empty_int_src: u32,
    pub xod_ppe_fifo_wr_full_int_src: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_int_rocee_src {
    pub xid_rocee_fsm_timout_int_src: u32,
    pub sbm_rocee_lnk_fsm_timout_int_src: u32,
    pub sbm_rocee_lnk_ecc_2bit_int_src: u32,
    pub sbm_rocee_mib_req_failed_int_src: u32,
    pub sbm_rocee_mib_req_fsm_timout_int_src: u32,
    pub sbm_rocee_mib_rels_fsm_timout_int_src: u32,
    pub sbm_rocee_sram_ecc_2bit_int_src: u32,
    pub sbm_rocee_mib_buf_sum_err_int_src: u32,
    pub sbm_rocee_mib_req_extra_int_src: u32,
    pub sbm_rocee_mib_rels_extra_int_src: u32,
    pub voq_rocee_start_to_over_0_int_src: u32,
    pub voq_rocee_ecc_err_int_src: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_int_tbl_src {
    pub tbl_da0_mis_src: u32,
    pub tbl_da1_mis_src: u32,
    pub tbl_da2_mis_src: u32,
    pub tbl_da3_mis_src: u32,
    pub tbl_da4_mis_src: u32,
    pub tbl_da5_mis_src: u32,
    pub tbl_da6_mis_src: u32,
    pub tbl_da7_mis_src: u32,
    pub tbl_sa_mis_src: u32,
    pub tbl_old_sech_end_src: u32,
    pub lram_ecc_err1_src: u32,
    pub lram_ecc_err2_src: u32,
    pub tram_ecc_err1_src: u32,
    pub tram_ecc_err2_src: u32,
    pub tbl_ucast_bcast_xge0_src: u32,
    pub tbl_ucast_bcast_xge1_src: u32,
    pub tbl_ucast_bcast_xge2_src: u32,
    pub tbl_ucast_bcast_xge3_src: u32,
    pub tbl_ucast_bcast_xge4_src: u32,
    pub tbl_ucast_bcast_xge5_src: u32,
    pub tbl_ucast_bcast_ppe_src: u32,
    pub tbl_ucast_bcast_rocee_src: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_int_stat {
    pub dsaf_int_xge_stat: [dsaf_int_xge_src; DSAF_COMM_CHN],
    pub dsaf_int_ppe_stat: [dsaf_int_ppe_src; DSAF_COMM_CHN],
    pub dsaf_int_rocee_stat: [dsaf_int_rocee_src; DSAF_COMM_CHN],
    pub dsaf_int_tbl_stat: [dsaf_int_tbl_src; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_misc_op {
    pub data): u16 speed, int,
    pub mac_cb): *mut *mut void (cpld_reset_led)(struct hns_mac_cb,
    pub status): hnae_led_state,
// reset series function, it will be reset if the dereset is 0
    pub dereset): *mut *mut *mut void (dsaf_reset)(struct dsaf_device dsaf_dev, bool,
    pub dereset): *mut *mut *mut void (xge_srst)(struct dsaf_device dsaf_dev, u32 port, bool,
    pub dereset): *mut *mut *mut void (ge_srst)(struct dsaf_device dsaf_dev, u32 port, bool,
    pub dereset): *mut *mut *mut void (ppe_srst)(struct dsaf_device dsaf_dev, u32 port, bool,
    pub dereset): *mut *mut *mut void (ppe_comm_srst)(struct dsaf_device dsaf_dev, bool,
    pub mac_cb): *mut *mut phy_interface_t (get_phy_if)(struct hns_mac_cb,
    pub sfp_prsnt): *mut *mut *mut int (get_sfp_prsnt)(struct hns_mac_cb mac_cb, int,
    pub en): *mut *mut *mut int (cfg_serdes_loopback)(struct hns_mac_cb mac_cb, bool,
}

// Dsaf device struct define ,and mac ->  dsaf
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_device {
    pub dev: *mut device,
    pub ae_dev: hnae_ae_dev,
    pub sc_base: *mut u8 __iomem,
    pub sds_base: *mut u8 __iomem,
    pub ppe_base: *mut u8 __iomem,
    pub io_base: *mut u8 __iomem,
    pub sub_ctrl: *mut regmap,
    pub ppe_paddr: phys_addr_t,
    pub queue*/: *mut *mut u32 desc_num; / desc num per,
    pub /: *mut *mut u32 buf_size; / ring buffer size,
    pub /: *mut *mut u32 reset_offset; / reset field offset in sub sysctrl,
    pub /: *mut *mut int buf_size_type; / ring buffer size-type,
    pub /: *mut *mut dsaf_mode dsaf_mode; / dsaf mode,
    pub dsaf_en: hal_dsaf_mode,
    pub dsaf_tc_mode: hal_dsaf_tc_mode,
    pub dsaf_ver: u32,
    pub /: *mut *mut u16 tcam_max_num; / max TCAM entry for user except promisc,
    pub ppe_common: [*mut ppe_common_cb; DSAF_COMM_DEV_NUM],
    pub rcb_common: [*mut rcb_common_cb; DSAF_COMM_DEV_NUM],
    pub mac_cb: [*mut hns_mac_cb; DSAF_MAX_PORT_NUM],
    pub misc_op: *mut dsaf_misc_op,
    pub hw_stats: [dsaf_hw_stats; DSAF_NODE_NUM],
    pub int_stat: dsaf_int_stat,
// make sure tcam table config spinlock
    pub tcam_lock: spinlock_t,
}

pub const DSAF_TBL_TCAM_KEY_PORT_S: c_int = 0;

pub const DSAF_TBL_TCAM_KEY_VLAN_S: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_drv_tbl_tcam_key {
    pub mac_3: u8,
    pub mac_2: u8,
    pub mac_1: u8,
    pub mac_0: u8,
    pub bits: },
    pub val: u32,
    pub high: },
    pub port_vlan: u16,
    pub mac_5: u8,
    pub mac_4: u8,
    pub bits: },
    pub val: u32,
    pub low: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_drv_soft_mac_tbl {
    pub tcam_key: dsaf_drv_tbl_tcam_key,
    pub tab*/: *mut *mut u16 index; /the entry's index in tcam,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsaf_drv_priv {
// soft tab Mac key, for hardware tab
    pub soft_mac_tbl: *mut dsaf_drv_soft_mac_tbl,
}

extern "C" {
    pub fn container_of(_arg: handle, hnae_vf_cb: struct, _arg: ae_handle) -> return;
}
extern "C" {
    pub fn hns_dsaf_fix_mac_mode(mac_cb: *mut hns_mac_cb);
}
extern "C" {
    pub fn hns_dsaf_ae_init(dsaf_dev: *mut dsaf_device) -> c_int;
}
extern "C" {
    pub fn hns_dsaf_ae_uninit(dsaf_dev: *mut dsaf_device);
}
extern "C" {
    pub fn hns_dsaf_update_stats(dsaf_dev: *mut dsaf_device, inode_num: u32);
}
extern "C" {
    pub fn hns_dsaf_get_sset_count(dsaf_dev: *mut dsaf_device, stringset: c_int) -> c_int;
}
extern "C" {
    pub fn hns_dsaf_get_stats(ddev: *mut dsaf_device, data: *mut u64, port: c_int);
}
extern "C" {
    pub fn hns_dsaf_get_regs(ddev: *mut dsaf_device, port: u32, data: *mut c_void);
}
extern "C" {
    pub fn hns_dsaf_get_regs_count() -> c_int;
}
extern "C" {
    pub fn hns_dsaf_set_promisc_mode(dsaf_dev: *mut dsaf_device, en: u32);
}
extern "C" {
    pub fn hns_dsaf_wait_pkt_clean(dsaf_dev: *mut dsaf_device, port: c_int) -> c_int;
}
