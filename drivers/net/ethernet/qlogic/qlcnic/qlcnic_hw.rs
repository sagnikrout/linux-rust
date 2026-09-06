//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qlcnic/qlcnic_hw.h
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
// QLogic qlcnic NIC Driver
// Copyright (c) 2009-2013 QLogic Corporation
//
// Common registers in 83xx and 82xx
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlcnic_regs {
    QLCNIC_PEG_HALT_STATUS1 = 0,
    QLCNIC_PEG_HALT_STATUS2,
    QLCNIC_PEG_ALIVE_COUNTER,
    QLCNIC_FLASH_LOCK_OWNER,
    QLCNIC_FW_CAPABILITIES,
    QLCNIC_CRB_DRV_ACTIVE,
    QLCNIC_CRB_DEV_STATE,
    QLCNIC_CRB_DRV_STATE,
    QLCNIC_CRB_DRV_SCRATCH,
    QLCNIC_CRB_DEV_PARTITION_INFO,
    QLCNIC_CRB_DRV_IDC_VER,
    QLCNIC_FW_VERSION_MAJOR,
    QLCNIC_FW_VERSION_MINOR,
    QLCNIC_FW_VERSION_SUB,
    QLCNIC_CRB_DEV_NPAR_STATE,
    QLCNIC_FW_IMG_VALID,
    QLCNIC_CMDPEG_STATE,
    QLCNIC_RCVPEG_STATE,
    QLCNIC_ASIC_TEMP,
    QLCNIC_FW_API,
    QLCNIC_DRV_OP_MODE,
    QLCNIC_FLASH_LOCK,
    QLCNIC_FLASH_UNLOCK,
}

// Read from an address offset from BAR0, existing registers

// Write to an address offset from BAR0, existing registers

// Read from a direct address offset from BAR0, additional registers

// Write to a direct address offset from BAR0, additional registers

pub const QLCNIC_CMD_CONFIGURE_IP_ADDR: c_uint = 0x1;
pub const QLCNIC_CMD_CONFIG_INTRPT: c_uint = 0x2;
pub const QLCNIC_CMD_CREATE_RX_CTX: c_uint = 0x7;
pub const QLCNIC_CMD_DESTROY_RX_CTX: c_uint = 0x8;
pub const QLCNIC_CMD_CREATE_TX_CTX: c_uint = 0x9;
pub const QLCNIC_CMD_DESTROY_TX_CTX: c_uint = 0xa;
pub const QLCNIC_CMD_CONFIGURE_LRO: c_uint = 0xC;
pub const QLCNIC_CMD_CONFIGURE_MAC_LEARNING: c_uint = 0xD;
pub const QLCNIC_CMD_GET_STATISTICS: c_uint = 0xF;
pub const QLCNIC_CMD_INTRPT_TEST: c_uint = 0x11;
pub const QLCNIC_CMD_SET_MTU: c_uint = 0x12;
pub const QLCNIC_CMD_READ_PHY: c_uint = 0x13;
pub const QLCNIC_CMD_WRITE_PHY: c_uint = 0x14;
pub const QLCNIC_CMD_READ_HW_REG: c_uint = 0x15;
pub const QLCNIC_CMD_GET_FLOW_CTL: c_uint = 0x16;
pub const QLCNIC_CMD_SET_FLOW_CTL: c_uint = 0x17;
pub const QLCNIC_CMD_READ_MAX_MTU: c_uint = 0x18;
pub const QLCNIC_CMD_READ_MAX_LRO: c_uint = 0x19;
pub const QLCNIC_CMD_MAC_ADDRESS: c_uint = 0x1f;
pub const QLCNIC_CMD_GET_PCI_INFO: c_uint = 0x20;
pub const QLCNIC_CMD_GET_NIC_INFO: c_uint = 0x21;
pub const QLCNIC_CMD_SET_NIC_INFO: c_uint = 0x22;
pub const QLCNIC_CMD_GET_ESWITCH_CAPABILITY: c_uint = 0x24;
pub const QLCNIC_CMD_TOGGLE_ESWITCH: c_uint = 0x25;
pub const QLCNIC_CMD_GET_ESWITCH_STATUS: c_uint = 0x26;
pub const QLCNIC_CMD_SET_PORTMIRRORING: c_uint = 0x27;
pub const QLCNIC_CMD_CONFIGURE_ESWITCH: c_uint = 0x28;
pub const QLCNIC_CMD_GET_ESWITCH_PORT_CONFIG: c_uint = 0x29;
pub const QLCNIC_CMD_GET_ESWITCH_STATS: c_uint = 0x2a;
pub const QLCNIC_CMD_CONFIG_PORT: c_uint = 0x2e;
pub const QLCNIC_CMD_TEMP_SIZE: c_uint = 0x2f;
pub const QLCNIC_CMD_GET_TEMP_HDR: c_uint = 0x30;
pub const QLCNIC_CMD_BC_EVENT_SETUP: c_uint = 0x31;
pub const QLCNIC_CMD_CONFIG_VPORT: c_uint = 0x32;
pub const QLCNIC_CMD_DCB_QUERY_CAP: c_uint = 0x34;
pub const QLCNIC_CMD_DCB_QUERY_PARAM: c_uint = 0x35;
pub const QLCNIC_CMD_GET_MAC_STATS: c_uint = 0x37;
pub const QLCNIC_CMD_82XX_SET_DRV_VER: c_uint = 0x38;
pub const QLCNIC_CMD_MQ_TX_CONFIG_INTR: c_uint = 0x39;
pub const QLCNIC_CMD_GET_LED_STATUS: c_uint = 0x3C;
pub const QLCNIC_CMD_CONFIGURE_RSS: c_uint = 0x41;
pub const QLCNIC_CMD_CONFIG_INTR_COAL: c_uint = 0x43;
pub const QLCNIC_CMD_CONFIGURE_LED: c_uint = 0x44;
pub const QLCNIC_CMD_CONFIG_MAC_VLAN: c_uint = 0x45;
pub const QLCNIC_CMD_GET_LINK_EVENT: c_uint = 0x48;
pub const QLCNIC_CMD_CONFIGURE_MAC_RX_MODE: c_uint = 0x49;
pub const QLCNIC_CMD_CONFIGURE_HW_LRO: c_uint = 0x4A;
pub const QLCNIC_CMD_SET_INGRESS_ENCAP: c_uint = 0x4E;
pub const QLCNIC_CMD_INIT_NIC_FUNC: c_uint = 0x60;
pub const QLCNIC_CMD_STOP_NIC_FUNC: c_uint = 0x61;
pub const QLCNIC_CMD_IDC_ACK: c_uint = 0x63;
pub const QLCNIC_CMD_SET_PORT_CONFIG: c_uint = 0x66;
pub const QLCNIC_CMD_GET_PORT_CONFIG: c_uint = 0x67;
pub const QLCNIC_CMD_GET_LINK_STATUS: c_uint = 0x68;
pub const QLCNIC_CMD_SET_LED_CONFIG: c_uint = 0x69;
pub const QLCNIC_CMD_GET_LED_CONFIG: c_uint = 0x6A;
pub const QLCNIC_CMD_83XX_SET_DRV_VER: c_uint = 0x6F;
pub const QLCNIC_CMD_ADD_RCV_RINGS: c_uint = 0x0B;
pub const QLCNIC_CMD_83XX_EXTEND_ISCSI_DUMP_CAP: c_uint = 0x37;
pub const QLCNIC_INTRPT_INTX: c_int = 1;
pub const QLCNIC_INTRPT_MSIX: c_int = 3;
pub const QLCNIC_INTRPT_ADD: c_int = 1;
pub const QLCNIC_INTRPT_DEL: c_int = 2;
pub const QLCNIC_GET_CURRENT_MAC: c_int = 1;
pub const QLCNIC_SET_STATION_MAC: c_int = 2;
pub const QLCNIC_GET_DEFAULT_MAC: c_int = 3;
pub const QLCNIC_GET_FAC_DEF_MAC: c_int = 4;
pub const QLCNIC_SET_FAC_DEF_MAC: c_int = 5;
pub const QLCNIC_MBX_LINK_EVENT: c_uint = 0x8001;
pub const QLCNIC_MBX_BC_EVENT: c_uint = 0x8002;
pub const QLCNIC_MBX_COMP_EVENT: c_uint = 0x8100;
pub const QLCNIC_MBX_REQUEST_EVENT: c_uint = 0x8101;
pub const QLCNIC_MBX_TIME_EXTEND_EVENT: c_uint = 0x8102;
pub const QLCNIC_MBX_DCBX_CONFIG_CHANGE_EVENT: c_uint = 0x8110;
pub const QLCNIC_MBX_SFP_INSERT_EVENT: c_uint = 0x8130;
pub const QLCNIC_MBX_SFP_REMOVE_EVENT: c_uint = 0x8131;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_mailbox_metadata {
    pub cmd: u32,
    pub in_args: u32,
    pub out_args: u32,
}

// Mailbox ownership

pub const QLCNIC_SET_OWNER: c_int = 1;
pub const QLCNIC_CLR_OWNER: c_int = 0;
pub const QLCNIC_MBX_TIMEOUT: c_int = 5000;
pub const QLCNIC_MBX_RSP_OK: c_int = 1;
pub const QLCNIC_MBX_PORT_RSP_OK: c_uint = 0x1a;

// Set HW Tx ring limit for 82xx adapter.
pub const QLCNIC_MAX_HW_TX_RINGS: c_int = 8;
pub const QLCNIC_MAX_HW_VNIC_TX_RINGS: c_int = 4;
pub const QLCNIC_MAX_TX_RINGS: c_int = 8;
pub const QLCNIC_MAX_SDS_RINGS: c_int = 8;
extern "C" {
    pub fn qlcnic_82xx_hw_read_wx_2M(adapter: *mut qlcnic_adapter, _arg: c_ulong, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_hw_write_wx_2M(: *mut qlcnic_adapter, _arg: c_ulong, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_config_hw_lro(adapter: *mut qlcnic_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_nic_set_promisc(adapter: *mut qlcnic_adapter, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_get_beacon_state(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_82xx_set_rx_coalesce(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_config_rss(adapter: *mut qlcnic_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_linkevent_request(adapter: *mut qlcnic_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_process_rcv_ring_diag(sds_ring: *mut qlcnic_host_sds_ring);
}
extern "C" {
    pub fn qlcnic_82xx_clear_lb_mode(adapter: *mut qlcnic_adapter, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_set_lb_mode(: *mut qlcnic_adapter, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_write_crb(: *mut qlcnic_adapter, : *mut c_char, _arg: loff_t, _arg: usize);
}
extern "C" {
    pub fn qlcnic_82xx_read_crb(: *mut qlcnic_adapter, : *mut c_char, _arg: loff_t, _arg: usize);
}
extern "C" {
    pub fn qlcnic_82xx_mq_intrpt(: *mut qlcnic_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_config_intrpt(: *mut qlcnic_adapter, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_fw_cmd_create_rx_ctx(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_fw_cmd_del_rx_ctx(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_82xx_sre_macaddr_change(: *mut qlcnic_adapter, : *mut u8, _arg: u16, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_get_mac_address(: *mut qlcnic_adapter, _arg: *mut u8, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_get_nic_info(: *mut qlcnic_adapter, : *mut qlcnic_info, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_set_nic_info(: *mut qlcnic_adapter, : *mut qlcnic_info) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_get_pci_info(: *mut qlcnic_adapter, qlcnic_pci_info*: *mut struct) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_get_board_info(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_config_led(: *mut qlcnic_adapter, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_get_func_no(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_82xx_api_lock(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_api_unlock(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_82xx_napi_enable(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_82xx_napi_disable(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_82xx_napi_del(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_82xx_shutdown(: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn qlcnic_82xx_resume(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_clr_all_drv_state(adapter: *mut qlcnic_adapter, failed: u8);
}
extern "C" {
    pub fn qlcnic_fw_poll_work(work: *mut work_struct);
}
extern "C" {
    pub fn qlcnic_82xx_get_saved_state(: *mut c_void, _arg: u32) -> u32;
}
extern "C" {
    pub fn qlcnic_82xx_set_saved_state(: *mut c_void, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn qlcnic_82xx_cache_tmpl_hdr_values(: *mut qlcnic_fw_dump);
}
extern "C" {
    pub fn qlcnic_82xx_get_cap_size(: *mut c_void, _arg: c_int) -> u32;
}
extern "C" {
    pub fn qlcnic_82xx_set_sys_info(: *mut c_void, _arg: c_int, _arg: u32);
}
extern "C" {
    pub fn qlcnic_82xx_store_cap_mask(: *mut c_void, _arg: u32);
}
