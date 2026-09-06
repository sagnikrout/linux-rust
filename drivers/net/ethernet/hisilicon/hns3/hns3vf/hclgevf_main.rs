//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3vf/hclgevf_main.h
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
// Copyright (c) 2016-2017 Hisilicon Limited.

pub const HCLGEVF_MAX_VLAN_ID: c_int = 4095;
pub const HCLGEVF_MISC_VECTOR_NUM: c_int = 0;
pub const HCLGEVF_INVALID_VPORT: c_uint = 0xffff;
pub const HCLGEVF_GENERAL_TASK_INTERVAL: c_int = 5;
pub const HCLGEVF_KEEP_ALIVE_TASK_INTERVAL: c_int = 2;
// This number in actual depends upon the total number of VFs
// created by physical function. But the maximum number of
// possible vector-per-VF is {VFn(1-32), VECTn(32 + 1)}.
//

pub const HCLGEVF_VECTOR_REG_BASE: c_uint = 0x20000;
pub const HCLGEVF_MISC_VECTOR_REG_BASE: c_uint = 0x20400;
pub const HCLGEVF_VECTOR_REG_OFFSET: c_uint = 0x4;
pub const HCLGEVF_VECTOR_VF_OFFSET: c_uint = 0x100000;
// bar registers for common func
pub const HCLGEVF_GRO_EN_REG: c_uint = 0x28000;
pub const HCLGEVF_RXD_ADV_LAYOUT_EN_REG: c_uint = 0x28008;
// bar registers for rcb
pub const HCLGEVF_RING_RX_ADDR_L_REG: c_uint = 0x80000;
pub const HCLGEVF_RING_RX_ADDR_H_REG: c_uint = 0x80004;
pub const HCLGEVF_RING_RX_BD_NUM_REG: c_uint = 0x80008;
pub const HCLGEVF_RING_RX_BD_LENGTH_REG: c_uint = 0x8000C;
pub const HCLGEVF_RING_RX_MERGE_EN_REG: c_uint = 0x80014;
pub const HCLGEVF_RING_RX_TAIL_REG: c_uint = 0x80018;
pub const HCLGEVF_RING_RX_HEAD_REG: c_uint = 0x8001C;
pub const HCLGEVF_RING_RX_FBD_NUM_REG: c_uint = 0x80020;
pub const HCLGEVF_RING_RX_OFFSET_REG: c_uint = 0x80024;
pub const HCLGEVF_RING_RX_FBD_OFFSET_REG: c_uint = 0x80028;
pub const HCLGEVF_RING_RX_STASH_REG: c_uint = 0x80030;
pub const HCLGEVF_RING_RX_BD_ERR_REG: c_uint = 0x80034;
pub const HCLGEVF_RING_TX_ADDR_L_REG: c_uint = 0x80040;
pub const HCLGEVF_RING_TX_ADDR_H_REG: c_uint = 0x80044;
pub const HCLGEVF_RING_TX_BD_NUM_REG: c_uint = 0x80048;
pub const HCLGEVF_RING_TX_PRIORITY_REG: c_uint = 0x8004C;
pub const HCLGEVF_RING_TX_TC_REG: c_uint = 0x80050;
pub const HCLGEVF_RING_TX_MERGE_EN_REG: c_uint = 0x80054;
pub const HCLGEVF_RING_TX_TAIL_REG: c_uint = 0x80058;
pub const HCLGEVF_RING_TX_HEAD_REG: c_uint = 0x8005C;
pub const HCLGEVF_RING_TX_FBD_NUM_REG: c_uint = 0x80060;
pub const HCLGEVF_RING_TX_OFFSET_REG: c_uint = 0x80064;
pub const HCLGEVF_RING_TX_EBD_NUM_REG: c_uint = 0x80068;
pub const HCLGEVF_RING_TX_EBD_OFFSET_REG: c_uint = 0x80070;
pub const HCLGEVF_RING_TX_BD_ERR_REG: c_uint = 0x80074;
pub const HCLGEVF_RING_EN_REG: c_uint = 0x80090;
// bar registers for tqp interrupt
pub const HCLGEVF_TQP_INTR_CTRL_REG: c_uint = 0x20000;
pub const HCLGEVF_TQP_INTR_GL0_REG: c_uint = 0x20100;
pub const HCLGEVF_TQP_INTR_GL1_REG: c_uint = 0x20200;
pub const HCLGEVF_TQP_INTR_GL2_REG: c_uint = 0x20300;
pub const HCLGEVF_TQP_INTR_RL_REG: c_uint = 0x20900;
// CMDQ register bits for RX event(=MBX event)
pub const HCLGEVF_VECTOR0_RX_CMDQ_INT_B: c_int = 1;
// RST register bits for RESET event
pub const HCLGEVF_VECTOR0_RST_INT_B: c_int = 2;
pub const HCLGEVF_TQP_RESET_TRY_TIMES: c_int = 10;
// Reset related Registers
pub const HCLGEVF_RST_ING: c_uint = 0x20C00;

pub const HCLGEVF_VF_RST_ING: c_uint = 0x07008;

pub const HCLGEVF_WAIT_RESET_DONE: c_int = 100;
pub const HCLGEVF_RSS_IND_TBL_SIZE: c_int = 512;
pub const HCLGEVF_TQP_MEM_SIZE: c_uint = 0x10000;
pub const HCLGEVF_MEM_BAR: c_int = 4;
// in the bar4, the first half is for roce, and the second half is for nic

pub const HCLGEVF_MAC_MAX_FRAME: c_int = 9728;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclgevf_evt_cause {
    HCLGEVF_VECTOR0_EVENT_RST,
    HCLGEVF_VECTOR0_EVENT_MBX,
    HCLGEVF_VECTOR0_EVENT_OTHER,
}

// states of hclgevf device & tasks
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclgevf_states {
// device states
    HCLGEVF_STATE_DOWN,
    HCLGEVF_STATE_DISABLED,
    HCLGEVF_STATE_IRQ_INITED,
    HCLGEVF_STATE_REMOVING,
    HCLGEVF_STATE_NIC_REGISTERED,
    HCLGEVF_STATE_ROCE_REGISTERED,
    HCLGEVF_STATE_SERVICE_INITED,
// task states
    HCLGEVF_STATE_RST_SERVICE_SCHED,
    HCLGEVF_STATE_RST_HANDLING,
    HCLGEVF_STATE_MBX_SERVICE_SCHED,
    HCLGEVF_STATE_MBX_HANDLING,
    HCLGEVF_STATE_LINK_UPDATING,
    HCLGEVF_STATE_PROMISC_CHANGED,
    HCLGEVF_STATE_RST_FAIL,
    HCLGEVF_STATE_PF_PUSH_LINK_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_mac {
    pub media_type: u8,
    pub module_type: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub link: c_int,
    pub duplex: u8,
    pub speed: u32,
    pub supported: u64,
    pub advertising: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_hw {
    pub hw: hclge_comm_hw,
    pub num_vec: c_int,
    pub mac: hclgevf_mac,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_cfg {
    pub tc_num: u8,
    pub tqp_desc_num: u16,
    pub rx_buf_len: u16,
    pub phy_addr: u8,
    pub media_type: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub numa_node_map: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_misc_vector {
    pub addr: *mut u8 __iomem,
    pub vector_irq: c_int,
    pub name: [c_char; HNAE3_INT_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_rst_stats {
    pub /: *mut *mut u32 rst_cnt; / the number of reset,
    pub /: *mut *mut u32 vf_func_rst_cnt; / the number of VF function reset,
    pub /: *mut *mut u32 flr_rst_cnt; / the number of FLR,
    pub /: *mut *mut u32 vf_rst_cnt; / the number of VF reset,
    pub /: *mut *mut u32 rst_done_cnt; / the number of reset completed,
    pub /: *mut *mut u32 hw_rst_done_cnt; / the number of HW reset completed,
    pub /: *mut *mut u32 rst_fail_cnt; / the number of VF reset fail,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGEVF_MAC_ADDR_TYPE {
    HCLGEVF_MAC_ADDR_UC,
    HCLGEVF_MAC_ADDR_MC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGEVF_MAC_NODE_STATE {
    HCLGEVF_MAC_TO_ADD,
    HCLGEVF_MAC_TO_DEL,
    HCLGEVF_MAC_ACTIVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_mac_addr_node {
    pub node: list_head,
    pub state: HCLGEVF_MAC_NODE_STATE,
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_mac_table_cfg {
    pub /: *mut *mut spinlock_t mac_list_lock; / protect mac address need to add/detele,
    pub uc_mac_list: list_head,
    pub mc_mac_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclgevf_dev {
    pub pdev: *mut pci_dev,
    pub ae_dev: *mut hnae3_ae_dev,
    pub hw: hclgevf_hw,
    pub misc_vector: hclgevf_misc_vector,
    pub rss_cfg: hclge_comm_rss_cfg,
    pub state: c_ulong,
    pub flr_state: c_ulong,
    pub default_reset_request: c_ulong,
    pub last_reset_time: c_ulong,
    pub reset_level: hnae3_reset_type,
    pub reset_pending: c_ulong,
    pub reset_type: hnae3_reset_type,
    pub reset_timer: timer_list,
pub const HCLGEVF_RESET_REQUESTED: c_int = 0;
pub const HCLGEVF_RESET_PENDING: c_int = 1;
    pub /: *mut *mut unsigned long reset_state; / requested, pending,
    pub rst_stats: hclgevf_rst_stats,
    pub reset_attempts: u32,
    pub /: *mut *mut semaphore reset_sem; / protect reset process,
    pub fw_version: u32,
    pub mbx_api_version: u16,
    pub /: *mut *mut u16 num_tqps; / num task queue pairs of this VF,
    pub /: *mut *mut u16 alloc_rss_size; / allocated RSS task queue,
    pub /: *mut *mut u16 rss_size_max; / HW defined max RSS task queue,
    pub /: *mut *mut u16 num_alloc_vport; / num vports this driver supports,
    pub numa_node_mask: nodemask_t,
    pub rx_buf_len: u16,
    pub /: *mut *mut u16 num_tx_desc; / desc num of per tx queue,
    pub /: *mut *mut u16 num_rx_desc; / desc num of per rx queue,
    pub hw_tc_map: u8,
    pub has_pf_mac: u8,
    pub num_msi: u16,
    pub num_msi_left: u16,
    pub num_msi_used: u16,
    pub /: *mut *mut u16 num_nic_msix; / Num of nic vectors for this VF,
    pub /: *mut *mut u16 num_roce_msix; / Num of roce vectors for this VF,
    pub roce_base_msix_offset: u16,
    pub vector_status: *mut u16,
    pub vector_irq: *mut c_int,
    pub gro_en: bool,
    pub rxvtag_strip_en: bool,
    pub vlan_del_fail_bmap: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub mac_table: hclgevf_mac_table_cfg,
    pub /: *mut *mut hclgevf_mbx_resp_status mbx_resp; / mailbox response,
    pub /: *mut *mut hclgevf_mbx_arq_ring arq; / mailbox async rx queue,
    pub service_task: delayed_work,
    pub htqp: *mut hclge_comm_tqp,
    pub nic: hnae3_handle,
    pub roce: hnae3_handle,
    pub nic_client: *mut hnae3_client,
    pub roce_client: *mut hnae3_client,
    pub flag: u32,
    pub serv_processed_cnt: c_ulong,
    pub last_serv_processed: c_ulong,
    pub devlink: *mut devlink,
}

extern "C" {
    pub fn hclgevf_mbx_handler(hdev: *mut hclgevf_dev);
}
extern "C" {
    pub fn hclgevf_mbx_async_handler(hdev: *mut hclgevf_dev);
}
extern "C" {
    pub fn hclgevf_update_link_status(hdev: *mut hclgevf_dev, link_state: c_int);
}
extern "C" {
    pub fn hclgevf_reset_task_schedule(hdev: *mut hclgevf_dev);
}
extern "C" {
    pub fn hclgevf_mbx_task_schedule(hdev: *mut hclgevf_dev);
}
