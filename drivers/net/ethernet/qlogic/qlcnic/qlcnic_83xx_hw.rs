//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qlcnic/qlcnic_83xx_hw.h
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

pub const QLCNIC_83XX_BAR0_LENGTH: c_uint = 0x4000;
// Directly mapped registers
pub const QLC_83XX_CRB_WIN_BASE: c_uint = 0x3800;

pub const QLC_83XX_SEM_LOCK_BASE: c_uint = 0x3840;
pub const QLC_83XX_SEM_UNLOCK_BASE: c_uint = 0x3844;

pub const QLC_83XX_LINK_SPEED_FACTOR: c_int = 10;

pub const QLC_83XX_INTX_PTR: c_uint = 0x38C0;
pub const QLC_83XX_INTX_TRGR: c_uint = 0x38C4;
pub const QLC_83XX_INTX_MASK: c_uint = 0x38C8;
pub const QLC_83XX_DRV_LOCK_WAIT_COUNTER: c_int = 100;
pub const QLC_83XX_DRV_LOCK_WAIT_DELAY: c_int = 20;
pub const QLC_83XX_NEED_DRV_LOCK_RECOVERY: c_int = 1;
pub const QLC_83XX_DRV_LOCK_RECOVERY_IN_PROGRESS: c_int = 2;
pub const QLC_83XX_MAX_DRV_LOCK_RECOVERY_ATTEMPT: c_int = 3;
pub const QLC_83XX_DRV_LOCK_RECOVERY_DELAY: c_int = 200;
pub const QLC_83XX_DRV_LOCK_RECOVERY_STATUS_MASK: c_uint = 0x3;
pub const QLC_83XX_LB_WAIT_COUNT: c_int = 250;
pub const QLC_83XX_LB_MSLEEP_COUNT: c_int = 20;
pub const QLC_83XX_NO_NIC_RESOURCE: c_uint = 0x5;
pub const QLC_83XX_MAC_PRESENT: c_uint = 0xC;
pub const QLC_83XX_MAC_ABSENT: c_uint = 0xD;

// PEG status definitions
pub const QLC_83XX_CMDPEG_COMPLETE: c_uint = 0xff01;

pub const QLC_83XX_LEGACY_INTX_MAX_RETRY: c_int = 100;
pub const QLC_83XX_LEGACY_INTX_DELAY: c_int = 4;
pub const QLC_83XX_REG_DESC: c_int = 1;
pub const QLC_83XX_LRO_DESC: c_int = 2;
pub const QLC_83XX_CTRL_DESC: c_int = 3;

pub const QLC_83XX_HOST_RDS_MODE_UNIQUE: c_int = 0;
pub const QLC_83XX_HOST_SDS_MBX_IDX: c_int = 8;
pub const QLCNIC_HOST_RDS_MBX_IDX: c_int = 88;
// Pause control registers
pub const QLC_83XX_SRE_SHIM_REG: c_uint = 0x0D200284;
pub const QLC_83XX_PORT0_THRESHOLD: c_uint = 0x0B2003A4;
pub const QLC_83XX_PORT1_THRESHOLD: c_uint = 0x0B2013A4;
pub const QLC_83XX_PORT0_TC_MC_REG: c_uint = 0x0B200388;
pub const QLC_83XX_PORT1_TC_MC_REG: c_uint = 0x0B201388;
pub const QLC_83XX_PORT0_TC_STATS: c_uint = 0x0B20039C;
pub const QLC_83XX_PORT1_TC_STATS: c_uint = 0x0B20139C;
pub const QLC_83XX_PORT2_IFB_THRESHOLD: c_uint = 0x0B200704;
pub const QLC_83XX_PORT3_IFB_THRESHOLD: c_uint = 0x0B201704;
// Peg PC status registers
pub const QLC_83XX_CRB_PEG_NET_0: c_uint = 0x3400003c;
pub const QLC_83XX_CRB_PEG_NET_1: c_uint = 0x3410003c;
pub const QLC_83XX_CRB_PEG_NET_2: c_uint = 0x3420003c;
pub const QLC_83XX_CRB_PEG_NET_3: c_uint = 0x3430003c;
pub const QLC_83XX_CRB_PEG_NET_4: c_uint = 0x34b0003c;
// Firmware image definitions
pub const QLC_83XX_BOOTLOADER_FLASH_ADDR: c_uint = 0x10000;

pub const QLC_83XX_BOOT_FROM_FLASH: c_int = 0;
pub const QLC_83XX_BOOT_FROM_FILE: c_uint = 0x12345678;
pub const QLC_FW_FILE_NAME_LEN: c_int = 20;
pub const QLC_83XX_MAX_RESET_SEQ_ENTRIES: c_int = 16;
pub const QLC_83XX_MBX_POST_BC_OP: c_uint = 0x1;
pub const QLC_83XX_MBX_COMPLETION: c_uint = 0x0;
pub const QLC_83XX_MBX_REQUEST: c_uint = 0x1;

pub const QLC_83XX_MBX_CMD_LOOP: c_int = 5000000;
// status descriptor mailbox data
// @phy_addr_{low|high}: physical address of buffer
// @sds_ring_size: buffer size
// @intrpt_id: interrupt id
// @intrpt_val: source of interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_sds_mbx {
    pub phy_addr_low: u32,
    pub phy_addr_high: u32,
    pub rsvd1: [u32; 4],
    pub sds_ring_size: u16,
    pub rsvd2: u16,
    pub rsvd3: [u16; 2],
    pub intrpt_id: u16,
    pub intrpt_val: u8,
    pub rsvd4: u8,

    pub rsvd2: u16,
    pub sds_ring_size: u16,
    pub rsvd3: [u16; 2],
    pub rsvd4: u8,
    pub intrpt_val: u8,
    pub intrpt_id: u16,

    pub rsvd5: u32,
    pub __packed: },
// receive descriptor buffer data
// phy_addr_reg_{low|high}: physical address of regular buffer
// phy_addr_jmb_{low|high}: physical address of jumbo buffer
// reg_ring_sz: size of regular buffer
// reg_ring_len: no. of entries in regular buffer
// jmb_ring_len: no. of entries in jumbo buffer
// jmb_ring_sz: size of jumbo buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_rds_mbx {
    pub phy_addr_reg_low: u32,
    pub phy_addr_reg_high: u32,
    pub phy_addr_jmb_low: u32,
    pub phy_addr_jmb_high: u32,

    pub reg_ring_sz: u16,
    pub reg_ring_len: u16,
    pub jmb_ring_sz: u16,
    pub jmb_ring_len: u16,

    pub reg_ring_len: u16,
    pub reg_ring_sz: u16,
    pub jmb_ring_len: u16,
    pub jmb_ring_sz: u16,

    pub __packed: },
// host producers for regular and jumbo rings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __host_producer_mbx {
    pub reg_buf: u32,
    pub jmb_buf: u32,
    pub __packed: },
// Receive context mailbox data outbox registers
// @state: state of the context
// @vport_id: virtual port id
// @context_id: receive context id
// @num_pci_func: number of pci functions of the port
// @phy_port: physical port id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_rcv_mbx_out {

    pub rcv_num: u8,
    pub sts_num: u8,
    pub ctx_id: u16,
    pub state: u8,
    pub num_pci_func: u8,
    pub phy_port: u8,
    pub vport_id: u8,

    pub ctx_id: u16,
    pub sts_num: u8,
    pub rcv_num: u8,
    pub vport_id: u8,
    pub phy_port: u8,
    pub num_pci_func: u8,
    pub state: u8,
    pub host_csmr: [u32; QLCNIC_MAX_SDS_RINGS],
    pub host_prod: [__host_producer_mbx; QLCNIC_MAX_SDS_RINGS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_add_rings_mbx_out {

    pub rcv_num: u8,
    pub sts_num: u8,
    pub ctx_id: u16,

    pub ctx_id: u16,
    pub sts_num: u8,
    pub rcv_num: u8,
    pub host_csmr: [u32; QLCNIC_MAX_SDS_RINGS],
    pub host_prod: [__host_producer_mbx; QLCNIC_MAX_SDS_RINGS],
    pub __packed: },
// Transmit context mailbox inbox registers
// @phys_addr_{low|high}: DMA address of the transmit buffer
// @cnsmr_index_{low|high}: host consumer index
// @size: legth of transmit buffer ring
// @intr_id: interrupt id
// @src: src of interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_tx_mbx {
    pub phys_addr_low: u32,
    pub phys_addr_high: u32,
    pub cnsmr_index_low: u32,
    pub cnsmr_index_high: u32,

    pub size: u16,
    pub intr_id: u16,
    pub src: u8,
    pub rsvd: [u8; 3],
    pub intr_id: u16,
    pub size: u16,
    pub rsvd: [u8; 3],
    pub src: u8,

    pub __packed: },
// Transmit context mailbox outbox registers
// @host_prod: host producer index
// @ctx_id: transmit context id
// @state: state of the transmit context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_tx_mbx_out {
    pub host_prod: u32,

    pub ctx_id: u16,
    pub state: u8,
    pub rsvd: u8,

    pub rsvd: u8,
    pub state: u8,
    pub ctx_id: u16,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_intrpt_config {
    pub type: u8,
    pub enabled: u8,
    pub id: u16,
    pub src: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlcnic_macvlan_mbx {

    pub mac_addr0: u8,
    pub mac_addr1: u8,
    pub mac_addr2: u8,
    pub mac_addr3: u8,
    pub mac_addr4: u8,
    pub mac_addr5: u8,
    pub vlan: u16,

    pub mac_addr3: u8,
    pub mac_addr2: u8,
    pub mac_addr1: u8,
    pub mac_addr0: u8,
    pub vlan: u16,
    pub mac_addr5: u8,
    pub mac_addr4: u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlc_83xx_fw_info {
    pub fw: *const firmware,
    pub fw_file_name: [c_char; QLC_FW_FILE_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlc_83xx_reset {
    pub hdr: *mut qlc_83xx_reset_hdr,
    pub seq_index: c_int,
    pub seq_error: c_int,
    pub array_index: c_int,
    pub array: [u32; QLC_83XX_MAX_RESET_SEQ_ENTRIES],
    pub buff: *mut u8,
    pub stop_offset: *mut u8,
    pub start_offset: *mut u8,
    pub init_offset: *mut u8,
    pub seq_end: u8,
    pub template_end: u8,
}

pub const QLC_83XX_IDC_DISABLE_FW_RESET_RECOVERY: c_uint = 0x1;
pub const QLC_83XX_IDC_GRACEFULL_RESET: c_uint = 0x2;
pub const QLC_83XX_IDC_DISABLE_FW_DUMP: c_uint = 0x4;
pub const QLC_83XX_IDC_TIMESTAMP: c_int = 0;
pub const QLC_83XX_IDC_DURATION: c_int = 1;
pub const QLC_83XX_IDC_INIT_TIMEOUT_SECS: c_int = 30;
pub const QLC_83XX_IDC_RESET_ACK_TIMEOUT_SECS: c_int = 10;
pub const QLC_83XX_IDC_RESET_TIMEOUT_SECS: c_int = 10;
pub const QLC_83XX_IDC_QUIESCE_ACK_TIMEOUT_SECS: c_int = 20;

pub const QLC_83XX_IDC_FW_FAIL_THRESH: c_int = 2;
pub const QLC_83XX_IDC_MAX_FUNC_PER_PARTITION_INFO: c_int = 8;
pub const QLC_83XX_IDC_MAX_CNA_FUNCTIONS: c_int = 16;
pub const QLC_83XX_IDC_MAJOR_VERSION: c_int = 1;
pub const QLC_83XX_IDC_MINOR_VERSION: c_int = 0;
pub const QLC_83XX_IDC_FLASH_PARAM_ADDR: c_uint = 0x3e8020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qlc_83xx_idc {
    pub ): *mut *mut int (state_entry) (struct qlcnic_adapter,
    pub sec_counter: u64,
    pub delay: u64,
    pub status: c_ulong,
    pub err_code: c_int,
    pub collect_dump: c_int,
    pub curr_state: u8,
    pub prev_state: u8,
    pub vnic_state: u8,
    pub vnic_wait_limit: u8,
    pub quiesce_req: u8,
    pub delay_reset: u8,
    pub name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlcnic_vlan_operations {
    QLC_VLAN_ADD = 0,
    QLC_VLAN_DELETE
}

// Device States
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlcnic_83xx_states {
    QLC_83XX_IDC_DEV_UNKNOWN,
    QLC_83XX_IDC_DEV_COLD,
    QLC_83XX_IDC_DEV_INIT,
    QLC_83XX_IDC_DEV_READY,
    QLC_83XX_IDC_DEV_NEED_RESET,
    QLC_83XX_IDC_DEV_NEED_QUISCENT,
    QLC_83XX_IDC_DEV_FAILED,
    QLC_83XX_IDC_DEV_QUISCENT
}

// Mailbox process AEN count
pub const QLC_83XX_IDC_COMP_AEN: c_int = 3;
pub const QLC_83XX_MBX_AEN_CNT: c_int = 5;
pub const QLC_83XX_MODULE_LOADED: c_int = 1;
pub const QLC_83XX_MBX_READY: c_int = 2;
pub const QLC_83XX_MBX_AEN_ACK: c_int = 3;

pub const QLC_83XX_TX_PAUSE: c_uint = 0x10;
pub const QLC_83XX_RX_PAUSE: c_uint = 0x20;
pub const QLC_83XX_TX_RX_PAUSE: c_uint = 0x30;

// LED configuration settings
pub const QLC_83XX_ENABLE_BEACON: c_uint = 0xe;
pub const QLC_83XX_BEACON_ON: c_int = 1;
pub const QLC_83XX_BEACON_OFF: c_int = 0;
pub const QLC_83XX_LED_RATE: c_uint = 0xff;

pub const QLC_83XX_10M_LINK: c_int = 1;
pub const QLC_83XX_100M_LINK: c_int = 2;
pub const QLC_83XX_1G_LINK: c_int = 3;
pub const QLC_83XX_10G_LINK: c_int = 4;
pub const QLC_83XX_STAT_TX: c_int = 3;
pub const QLC_83XX_STAT_RX: c_int = 2;
pub const QLC_83XX_STAT_MAC: c_int = 1;
pub const QLC_83XX_TX_STAT_REGS: c_int = 14;
pub const QLC_83XX_RX_STAT_REGS: c_int = 40;
pub const QLC_83XX_MAC_STAT_REGS: c_int = 94;

pub const QLC_83XX_DEFAULT_OPMODE: c_uint = 0x55555555;
pub const QLC_83XX_PRIVLEGED_FUNC: c_uint = 0x1;
pub const QLC_83XX_VIRTUAL_FUNC: c_uint = 0x2;
pub const QLC_83XX_LB_MAX_FILTERS: c_int = 2048;
pub const QLC_83XX_LB_BUCKET_SIZE: c_int = 256;
pub const QLC_83XX_MINIMUM_VECTOR: c_int = 3;
pub const QLC_83XX_MAX_MC_COUNT: c_int = 38;
pub const QLC_83XX_MAX_UC_COUNT: c_int = 4096;

pub const QLC_83XX_SRIOV_MODE: c_uint = 0x1;
pub const QLCNIC_BRDTYPE_83XX_10G: c_uint = 0x0083;
pub const QLC_83XX_FLASH_SPI_STATUS: c_uint = 0x2808E010;
pub const QLC_83XX_FLASH_SPI_CONTROL: c_uint = 0x2808E014;
pub const QLC_83XX_FLASH_STATUS: c_uint = 0x42100004;
pub const QLC_83XX_FLASH_CONTROL: c_uint = 0x42110004;
pub const QLC_83XX_FLASH_ADDR: c_uint = 0x42110008;
pub const QLC_83XX_FLASH_WRDATA: c_uint = 0x4211000C;
pub const QLC_83XX_FLASH_RDDATA: c_uint = 0x42110018;
pub const QLC_83XX_FLASH_DIRECT_WINDOW: c_uint = 0x42110030;

pub const QLC_83XX_FLASH_SECTOR_ERASE_CMD: c_uint = 0xdeadbeef;
pub const QLC_83XX_FLASH_WRITE_CMD: c_uint = 0xdacdacda;
pub const QLC_83XX_FLASH_BULK_WRITE_CMD: c_uint = 0xcadcadca;
pub const QLC_83XX_FLASH_READ_RETRY_COUNT: c_int = 5000;
pub const QLC_83XX_FLASH_STATUS_READY: c_uint = 0x6;
pub const QLC_83XX_FLASH_WRITE_MIN: c_int = 2;
pub const QLC_83XX_FLASH_WRITE_MAX: c_int = 64;
pub const QLC_83XX_FLASH_STATUS_REG_POLL_DELAY: c_int = 1;
pub const QLC_83XX_ERASE_MODE: c_int = 1;
pub const QLC_83XX_WRITE_MODE: c_int = 2;
pub const QLC_83XX_BULK_WRITE_MODE: c_int = 3;
pub const QLC_83XX_FLASH_FDT_WRITE_DEF_SIG: c_uint = 0xFD0100;
pub const QLC_83XX_FLASH_FDT_ERASE_DEF_SIG: c_uint = 0xFD0300;
pub const QLC_83XX_FLASH_FDT_READ_MFG_ID_VAL: c_uint = 0xFD009F;
pub const QLC_83XX_FLASH_OEM_ERASE_SIG: c_uint = 0xFD03D8;
pub const QLC_83XX_FLASH_OEM_WRITE_SIG: c_uint = 0xFD0101;
pub const QLC_83XX_FLASH_OEM_READ_SIG: c_uint = 0xFD0005;
pub const QLC_83XX_FLASH_ADDR_TEMP_VAL: c_uint = 0x00800000;
pub const QLC_83XX_FLASH_ADDR_SECOND_TEMP_VAL: c_uint = 0x00800001;
pub const QLC_83XX_FLASH_WRDATA_DEF: c_uint = 0x0;
pub const QLC_83XX_FLASH_READ_CTRL: c_uint = 0x3F;
pub const QLC_83XX_FLASH_SPI_CTRL: c_uint = 0x4;
pub const QLC_83XX_FLASH_FIRST_ERASE_MS_VAL: c_uint = 0x2;
pub const QLC_83XX_FLASH_SECOND_ERASE_MS_VAL: c_uint = 0x5;
pub const QLC_83XX_FLASH_LAST_ERASE_MS_VAL: c_uint = 0x3D;
pub const QLC_83XX_FLASH_FIRST_MS_PATTERN: c_uint = 0x43;
pub const QLC_83XX_FLASH_SECOND_MS_PATTERN: c_uint = 0x7F;
pub const QLC_83XX_FLASH_LAST_MS_PATTERN: c_uint = 0x7D;
pub const QLC_83xx_FLASH_MAX_WAIT_USEC: c_int = 100;
pub const QLC_83XX_FLASH_LOCK_TIMEOUT: c_int = 10000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlc_83xx_mbx_cmd_type {
    QLC_83XX_MBX_CMD_WAIT = 0,
    QLC_83XX_MBX_CMD_NO_WAIT,
    QLC_83XX_MBX_CMD_BUSY_WAIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlc_83xx_mbx_response_states {
    QLC_83XX_MBX_RESPONSE_WAIT = 0,
    QLC_83XX_MBX_RESPONSE_ARRIVED,
}

pub const QLC_83XX_MBX_RESPONSE_FAILED: c_uint = 0x2;
pub const QLC_83XX_MBX_RESPONSE_UNKNOWN: c_uint = 0x3;
// Additional registers in 83xx
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qlc_83xx_ext_regs {
    QLCNIC_GLOBAL_RESET = 0,
    QLCNIC_WILDCARD,
    QLCNIC_INFORMANT,
    QLCNIC_HOST_MBX_CTRL,
    QLCNIC_FW_MBX_CTRL,
    QLCNIC_BOOTLOADER_ADDR,
    QLCNIC_BOOTLOADER_SIZE,
    QLCNIC_FW_IMAGE_ADDR,
    QLCNIC_MBX_INTR_ENBL,
    QLCNIC_DEF_INT_MASK,
    QLCNIC_DEF_INT_ID,
    QLC_83XX_IDC_MAJ_VERSION,
    QLC_83XX_IDC_DEV_STATE,
    QLC_83XX_IDC_DRV_PRESENCE,
    QLC_83XX_IDC_DRV_ACK,
    QLC_83XX_IDC_CTRL,
    QLC_83XX_IDC_DRV_AUDIT,
    QLC_83XX_IDC_MIN_VERSION,
    QLC_83XX_RECOVER_DRV_LOCK,
    QLC_83XX_IDC_PF_0,
    QLC_83XX_IDC_PF_1,
    QLC_83XX_IDC_PF_2,
    QLC_83XX_IDC_PF_3,
    QLC_83XX_IDC_PF_4,
    QLC_83XX_IDC_PF_5,
    QLC_83XX_IDC_PF_6,
    QLC_83XX_IDC_PF_7,
    QLC_83XX_IDC_PF_8,
    QLC_83XX_IDC_PF_9,
    QLC_83XX_IDC_PF_10,
    QLC_83XX_IDC_PF_11,
    QLC_83XX_IDC_PF_12,
    QLC_83XX_IDC_PF_13,
    QLC_83XX_IDC_PF_14,
    QLC_83XX_IDC_PF_15,
    QLC_83XX_IDC_DEV_PARTITION_INFO_1,
    QLC_83XX_IDC_DEV_PARTITION_INFO_2,
    QLC_83XX_DRV_OP_MODE,
    QLC_83XX_VNIC_STATE,
    QLC_83XX_DRV_LOCK,
    QLC_83XX_DRV_UNLOCK,
    QLC_83XX_DRV_LOCK_ID,
    QLC_83XX_ASIC_TEMP,
}

// Initialize/Stop NIC command bit definitions

// 83xx funcitons
extern "C" {
    pub fn qlcnic_83xx_get_fw_version(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_issue_cmd(: *mut qlcnic_adapter, : *mut qlcnic_cmd_args) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_setup_intr(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_get_func_no(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_cam_lock(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_cam_unlock(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_add_sysfs(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_remove_sysfs(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_write_crb(: *mut qlcnic_adapter, : *mut c_char, _arg: loff_t, _arg: usize);
}
extern "C" {
    pub fn qlcnic_83xx_read_crb(: *mut qlcnic_adapter, : *mut c_char, _arg: loff_t, _arg: usize);
}
extern "C" {
    pub fn qlcnic_83xx_rd_reg_indirect(: *mut qlcnic_adapter, _arg: c_ulong, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_wrt_reg_indirect(: *mut qlcnic_adapter, _arg: c_ulong, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_nic_set_promisc(: *mut qlcnic_adapter, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_config_hw_lro(: *mut qlcnic_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_config_rss(: *mut qlcnic_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_get_pci_info(: *mut qlcnic_adapter, : *mut qlcnic_pci_info) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_set_nic_info(: *mut qlcnic_adapter, : *mut qlcnic_info) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_initialize_nic(: *mut qlcnic_adapter, _arg: c_int);
}
extern "C" {
    pub fn qlcnic_83xx_napi_add(: *mut qlcnic_adapter, : *mut net_device) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_napi_del(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_napi_enable(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_napi_disable(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_config_led(: *mut qlcnic_adapter, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_ind_wr(: *mut qlcnic_adapter, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_ind_rd(: *mut qlcnic_adapter, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_create_rx_ctx(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_del_rx_ctx(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_get_nic_info(: *mut qlcnic_adapter, : *mut qlcnic_info, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_setup_link_event(: *mut qlcnic_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_process_rcv_ring_diag(: *mut qlcnic_host_sds_ring);
}
extern "C" {
    pub fn qlcnic_83xx_config_intrpt(: *mut qlcnic_adapter, _arg: bool) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_sre_macaddr_change(: *mut qlcnic_adapter, : *mut u8, _arg: u16, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_get_mac_address(: *mut qlcnic_adapter, : *mut u8, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_free_mbx_args(: *mut qlcnic_cmd_args);
}
extern "C" {
    pub fn qlcnic_83xx_set_rx_tx_intr_coal(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_get_port_info(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_enable_mbx_interrupt(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_disable_mbx_intr(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_clear_legacy_intr(: *mut qlcnic_adapter) -> irqreturn_t;
}
extern "C" {
    pub fn qlcnic_83xx_intr(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qlcnic_83xx_tmp_intr(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qlcnic_83xx_setup_mbx_intr(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_free_mbx_intr(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_register_map(: *mut qlcnic_hardware_context);
}
extern "C" {
    pub fn qlcnic_83xx_idc_aen_work(: *mut work_struct);
}
extern "C" {
    pub fn qlcnic_83xx_config_ipaddr(: *mut qlcnic_adapter, _arg: __be32, _arg: c_int);
}
extern "C" {
    pub fn qlcnic_83xx_erase_flash_sector(: *mut qlcnic_adapter, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_flash_bulk_write(: *mut qlcnic_adapter, _arg: u32, : *mut u32, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_flash_write32(: *mut qlcnic_adapter, _arg: u32, : *mut u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_lock_flash(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_unlock_flash(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_read_flash_mfg_id(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_read_flash_descriptor_table(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_flash_read32(: *mut qlcnic_adapter, _arg: u32, : *mut u8, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_init(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_idc_ready_state_entry(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_idc_poll_dev_state(: *mut work_struct);
}
extern "C" {
    pub fn qlcnic_83xx_idc_exit(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_idc_request_reset(: *mut qlcnic_adapter, _arg: u32);
}
extern "C" {
    pub fn qlcnic_83xx_lock_driver(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_unlock_driver(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_idc_vnic_pf_entry(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_disable_vnic_mode(: *mut qlcnic_adapter, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_config_vnic_opmode(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_set_port_eswitch_status(: *mut qlcnic_adapter, _arg: c_int, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_get_minidump_template(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_get_stats(adapter: *mut qlcnic_adapter, data: *mut u64);
}
extern "C" {
    pub fn qlcnic_83xx_extend_md_capab(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_test_link(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_get_port_type(adapter: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_reg_test(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_get_regs_len(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_get_registers(: *mut qlcnic_adapter, : *mut u32) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_loopback_test(: *mut net_device, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_interrupt_test(: *mut net_device) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_set_led(: *mut net_device, ethtool_phys_id_state: enum) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_flash_test(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_enable_flash_write(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_disable_flash_write(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_enable_mbx_poll(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_disable_mbx_poll(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_idc_init(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_idc_reattach_driver(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_set_vnic_opmode(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_check_vnic_state(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_aer_stop_poll_work(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_aer_reset(: *mut qlcnic_adapter) -> c_int;
}
extern "C" {
    pub fn qlcnic_83xx_aer_start_poll_work(: *mut qlcnic_adapter);
}
extern "C" {
    pub fn qlcnic_83xx_get_saved_state(: *mut c_void, _arg: u32) -> u32;
}
extern "C" {
    pub fn qlcnic_83xx_set_saved_state(: *mut c_void, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn qlcnic_83xx_cache_tmpl_hdr_values(: *mut qlcnic_fw_dump);
}
extern "C" {
    pub fn qlcnic_83xx_get_cap_size(: *mut c_void, _arg: c_int) -> u32;
}
extern "C" {
    pub fn qlcnic_83xx_set_sys_info(: *mut c_void, _arg: c_int, _arg: u32);
}
extern "C" {
    pub fn qlcnic_83xx_store_cap_mask(: *mut c_void, _arg: u32);
}
extern "C" {
    pub fn qlcnic_ms_mem_write128(: *mut qlcnic_adapter, _arg: u64, : *mut u32, _arg: u32) -> c_int;
}
