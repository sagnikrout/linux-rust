//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla4xxx/ql4_def.h
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
// QLogic iSCSI HBA Driver
// Copyright (c)  2003-2013 QLogic Corporation
//

pub const PCI_DEVICE_ID_QLOGIC_ISP4010: c_uint = 0x4010;

pub const PCI_DEVICE_ID_QLOGIC_ISP4022: c_uint = 0x4022;

pub const PCI_DEVICE_ID_QLOGIC_ISP4032: c_uint = 0x4032;

pub const PCI_DEVICE_ID_QLOGIC_ISP8022: c_uint = 0x8022;

pub const PCI_DEVICE_ID_QLOGIC_ISP8324: c_uint = 0x8032;

pub const PCI_DEVICE_ID_QLOGIC_ISP8042: c_uint = 0x8042;

pub const ISP4XXX_PCI_FN_1: c_uint = 0x1;
pub const ISP4XXX_PCI_FN_2: c_uint = 0x3;
pub const QLA_SUCCESS: c_int = 0;
pub const QLA_ERROR: c_int = 1;

//
// Data bit definitions
//
pub const BIT_0: c_uint = 0x1;
pub const BIT_1: c_uint = 0x2;
pub const BIT_2: c_uint = 0x4;
pub const BIT_3: c_uint = 0x8;
pub const BIT_4: c_uint = 0x10;
pub const BIT_5: c_uint = 0x20;
pub const BIT_6: c_uint = 0x40;
pub const BIT_7: c_uint = 0x80;
pub const BIT_8: c_uint = 0x100;
pub const BIT_9: c_uint = 0x200;
pub const BIT_10: c_uint = 0x400;
pub const BIT_11: c_uint = 0x800;
pub const BIT_12: c_uint = 0x1000;
pub const BIT_13: c_uint = 0x2000;
pub const BIT_14: c_uint = 0x4000;
pub const BIT_15: c_uint = 0x8000;
pub const BIT_16: c_uint = 0x10000;
pub const BIT_17: c_uint = 0x20000;
pub const BIT_18: c_uint = 0x40000;
pub const BIT_19: c_uint = 0x80000;
pub const BIT_20: c_uint = 0x100000;
pub const BIT_21: c_uint = 0x200000;
pub const BIT_22: c_uint = 0x400000;
pub const BIT_23: c_uint = 0x800000;
pub const BIT_24: c_uint = 0x1000000;
pub const BIT_25: c_uint = 0x2000000;
pub const BIT_26: c_uint = 0x4000000;
pub const BIT_27: c_uint = 0x8000000;
pub const BIT_28: c_uint = 0x10000000;
pub const BIT_29: c_uint = 0x20000000;
pub const BIT_30: c_uint = 0x40000000;
pub const BIT_31: c_uint = 0x80000000;
//
// Macros to help code, maintain, etc.
//

//
// Host adapter default definitions
//
pub const MAX_HBAS: c_int = 16;
pub const MAX_BUSES: c_int = 1;

pub const MAX_LUNS: c_uint = 0xffff;

pub const MAX_PDU_ENTRIES: c_int = 32;
pub const INVALID_ENTRY: c_uint = 0xFFFF;
pub const MAX_CMDS_TO_RISC: c_int = 1024;

pub const MBOX_AEN_REG_COUNT: c_int = 8;
pub const MAX_INIT_RETRIES: c_int = 5;
//
// Buffer sizes
//

pub const RESPONSE_QUEUE_DEPTH: c_int = 64;
pub const QUEUE_SIZE: c_int = 64;
pub const DMA_BUFFER_SIZE: c_int = 512;
pub const IOCB_HIWAT_CUSHION: c_int = 4;
//
// Misc
//

pub const MAX_LINKED_CMDS_PER_LUN: c_int = 3;
pub const MAX_REQS_SERVICED_PER_INTR: c_int = 1;

pub const ISCSI_NAME_SIZE: c_uint = 0xE0	/* ISCSI Name size */;

// recovery timeout

pub const DEV_DB_NON_PERSISTENT: c_int = 0;
pub const DEV_DB_PERSISTENT: c_int = 1;
pub const QL4_ISP_REG_DISCONNECT: c_uint = 0xffffffffU;

//
// Retry & Timeout Values
//
pub const MBOX_TOV: c_int = 60;
pub const SOFT_RESET_TOV: c_int = 30;
pub const RESET_INTR_TOV: c_int = 3;
pub const SEMAPHORE_TOV: c_int = 10;
pub const ADAPTER_INIT_TOV: c_int = 30;
pub const ADAPTER_RESET_TOV: c_int = 180;
pub const EXTEND_CMD_TOV: c_int = 60;
pub const WAIT_CMD_TOV: c_int = 5;
pub const EH_WAIT_CMD_TOV: c_int = 120;
pub const FIRMWARE_UP_TOV: c_int = 60;
pub const RESET_FIRMWARE_TOV: c_int = 30;
pub const LOGOUT_TOV: c_int = 10;
pub const IOCB_TOV_MARGIN: c_int = 10;
pub const RELOGIN_TOV: c_int = 18;
pub const ISNS_DEREG_TOV: c_int = 5;
pub const HBA_ONLINE_TOV: c_int = 30;
pub const DISABLE_ACB_TOV: c_int = 30;
pub const IP_CONFIG_TOV: c_int = 30;
pub const LOGIN_TOV: c_int = 12;
pub const BOOT_LOGIN_RESP_TOV: c_int = 60;
pub const MAX_RESET_HA_RETRIES: c_int = 2;
pub const FW_ALIVE_WAIT_TOV: c_int = 3;
pub const IDC_EXTEND_TOV: c_int = 8;
pub const IDC_COMP_TOV: c_int = 5;
pub const LINK_UP_COMP_TOV: c_int = 30;
//
// Note: the data structure below does not have a struct iscsi_cmd member since
// the qla4xxx driver does not use libiscsi for SCSI I/O.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4xxx_cmd_priv {
    pub srb: *mut srb,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
//
// SCSI Request Block structure (srb) that is associated with each scsi_cmnd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct srb {
    pub /: *mut *mut list_head list; / (8),
    pub /: *mut *mut *mut scsi_qla_host ha; / HA the SP is queued on,
    pub ddb: *mut ddb_entry,
    pub /: *mut *mut uint16_t flags; / (1) Status flags.,

    pub /: *mut *mut uint8_t state; / (1) Status flags.,

pub const SRB_FREE_STATE: c_int = 1;
pub const SRB_ACTIVE_STATE: c_int = 3;
pub const SRB_ACTIVE_TIMEOUT_STATE: c_int = 4;

    pub /: *mut *mut *mut scsi_cmnd cmd; / (4) SCSI command block,
    pub /: *mut *mut dma_addr_t dma_handle; / (4) for unmap of single transfers,
    pub /: *mut *mut kref srb_ref; / reference count for this srb,
    pub /: *mut *mut uint8_t err_id; / error id,

pub const SRB_ERR_OTHER: c_int = 4;
    pub reserved: u16,
    pub iocb_tov: u16,
    pub /: *mut *mut uint16_t iocb_cnt; / Number of used iocbs,
    pub cc_stat: u16,
// Used for extended sense / status continuation
    pub req_sense_ptr: *mut u8,
    pub req_sense_len: u16,
    pub reserved2: u16,
}

// Mailbox request block structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrb {
    pub ha: *mut scsi_qla_host,
    pub mbox: *mut mbox_cmd_iocb,
    pub mbox_cmd: u32,
    pub /: *mut *mut uint16_t iocb_cnt; / Number of used iocbs,
    pub pid: u32,
}

//
// Asynchronous Event Queue structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aen {
    pub mbox_sts: [u32; MBOX_AEN_REG_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql4_aen_log {
    pub count: c_int,
    pub entry: [aen; MAX_AEN_ENTRIES],
}

//
// Device Database (DDB) structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddb_entry {
    pub ha: *mut scsi_qla_host,
    pub sess: *mut iscsi_cls_session,
    pub conn: *mut iscsi_cls_conn,
    pub /: *mut *mut uint16_t fw_ddb_index; / DDB firmware index,
    pub /: *mut *mut uint32_t fw_ddb_device_state; / F/W Device State -- see ql4_fw.h,
    pub ddb_type: u16,
pub const FLASH_DDB: c_uint = 0x01;
    pub fw_ddb_entry: dev_db_entry,
    pub cls_session): *mut *mut int (unblock_sess)(struct iscsi_cls_session,
    pub state): *mut *mut ddb_entry ddb_entry, uint32_t,
// Driver Re-login
    pub /: *mut *mut unsigned long flags; / DDB Flags,

    pub for: *mut *mut uint16_t default_relogin_timeout; / Max time to wait,
// relogin to complete
    pub relogins: *mut *mut atomic_t retry_relogin_timer; / Min Time between,
// (4000 only)
    pub for: *mut *mut atomic_t relogin_timer; / Max Time to wait,
// relogin to complete
    pub been: *mut *mut atomic_t relogin_retry_count; / Num of times relogin has,
// retried
    pub between: *mut *mut uint32_t default_time2wait; / Default Min time,
// relogins (+aens)
    pub chap_tbl_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_ddb_index {
    pub list: list_head,
    pub fw_ddb_idx: u16,
    pub flash_ddb_idx: u16,
    pub fw_ddb: dev_db_entry,
    pub flash_isid: [u8; 6],
}

pub const DDB_IPADDR_LEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql4_tuple_ddb {
    pub port: c_int,
    pub tpgt: c_int,
    pub ip_addr: [c_char; DDB_IPADDR_LEN],
    pub iscsi_name: [c_char; ISCSI_NAME_SIZE],
    pub options: u16,
pub const DDB_OPT_IPV6: c_uint = 0x0e0e;
pub const DDB_OPT_IPV4: c_uint = 0x0f0f;
    pub isid: [u8; 6],
}

//
// DDB states.
//

// this device

// commands

// to re-login
//
// DDB flags.
//

pub const DF_FO_MASKED: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qla4_work_type {
    QLA4_EVENT_AEN,
    QLA4_EVENT_PING_STATUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_work_evt {
    pub list: list_head,
    pub type: qla4_work_type,
    pub code: iscsi_host_event_code,
    pub data_size: u32,
    pub data: [u8; ],
    pub aen: },
    pub status: u32,
    pub pid: u32,
    pub data_size: u32,
    pub data: [u8; ],
    pub ping: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql82xx_hw_data {
// Offsets for flash/nvram access (set to ~0 if not used).
    pub flash_conf_off: u32,
    pub flash_data_off: u32,
    pub fdt_wrt_disable: u32,
    pub fdt_erase_cmd: u32,
    pub fdt_block_size: u32,
    pub fdt_unprotect_sec_cmd: u32,
    pub fdt_protect_sec_cmd: u32,
    pub flt_region_flt: u32,
    pub flt_region_fdt: u32,
    pub flt_region_boot: u32,
    pub flt_region_bootload: u32,
    pub flt_region_fw: u32,
    pub flt_iscsi_param: u32,
    pub flt_region_chap: u32,
    pub flt_chap_size: u32,
    pub flt_region_ddb: u32,
    pub flt_ddb_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla4_8xxx_legacy_intr_set {
    pub int_vec_bit: u32,
    pub tgt_status_reg: u32,
    pub tgt_mask_reg: u32,
    pub pci_int_reg: u32,
}

// MSI-X Support
pub const QLA_MSIX_ENTRIES: c_int = 2;
//
// ISP Operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_operations {
    pub ha): *mut *mut int (iospace_config) (struct scsi_qla_host,
    pub ): *mut *mut void (pci_config) (struct scsi_qla_host,
    pub ): *mut *mut void (disable_intrs) (struct scsi_qla_host,
    pub ): *mut *mut void (enable_intrs) (struct scsi_qla_host,
    pub ): *mut *mut int (start_firmware) (struct scsi_qla_host,
    pub ): *mut *mut int (restart_firmware) (struct scsi_qla_host,
    pub ): *mut *mut irqreturn_t (intr_handler) (int , void,
    pub uint32_t): *mut *mut *mut void (interrupt_service_routine) (struct scsi_qla_host ,,
    pub ): *mut *mut int (need_reset) (struct scsi_qla_host,
    pub ): *mut *mut int (reset_chip) (struct scsi_qla_host,
    pub ): *mut *mut int (reset_firmware) (struct scsi_qla_host,
    pub ): *mut *mut void (queue_iocb) (struct scsi_qla_host,
    pub ): *mut *mut void (complete_iocb) (struct scsi_qla_host,
    pub ): *mut *mut uint16_t (rd_shdw_req_q_out) (struct scsi_qla_host,
    pub ): *mut *mut uint16_t (rd_shdw_rsp_q_in) (struct scsi_qla_host,
    pub ): *mut *mut int (get_sys_info) (struct scsi_qla_host,
    pub ulong): *mut *mut *mut uint32_t (rd_reg_direct) (struct scsi_qla_host ,,
    pub uint32_t): *mut *mut *mut void (wr_reg_direct) (struct scsi_qla_host , ulong,,
    pub ): *mut *mut *mut int (rd_reg_indirect) (struct scsi_qla_host , uint32_t, uint32_t,
    pub uint32_t): *mut *mut *mut int (wr_reg_indirect) (struct scsi_qla_host , uint32_t,,
    pub /: *mut *mut *mut *mut int (idc_lock) (struct scsi_qla_host ); / Context: task, can sleep,
    pub ): *mut *mut void (idc_unlock) (struct scsi_qla_host,
    pub /: *mut *mut *mut *mut void (rom_lock_recovery) (struct scsi_qla_host ); / Context: task, can sleep,
    pub int): *mut *mut *mut *mut void (queue_mailbox_command) (struct scsi_qla_host , uint32_t ,,
    pub int): *mut *mut *mut void (process_mailbox_interrupt) (struct scsi_qla_host ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql4_mdump_size_table {
    pub size: u32,
    pub size_cmask_02: u32,
    pub size_cmask_04: u32,
    pub size_cmask_08: u32,
    pub size_cmask_10: u32,
    pub size_cmask_FF: u32,
    pub version: u32,
}

// qla4xxx ipaddress configuration details
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipaddress_config {
    pub ipv4_options: u16,
    pub tcp_options: u16,
    pub ipv4_vlan_tag: u16,
    pub ipv4_addr_state: u8,
    pub ip_address: [u8; IP_ADDR_LEN],
    pub subnet_mask: [u8; IP_ADDR_LEN],
    pub gateway: [u8; IP_ADDR_LEN],
    pub ipv6_options: u32,
    pub ipv6_addl_options: u32,
    pub ipv6_link_local_state: u8,
    pub ipv6_addr0_state: u8,
    pub ipv6_addr1_state: u8,
    pub ipv6_default_router_state: u8,
    pub ipv6_vlan_tag: u16,
    pub ipv6_link_local_addr: in6_addr,
    pub ipv6_addr0: in6_addr,
    pub ipv6_addr1: in6_addr,
    pub ipv6_default_router_addr: in6_addr,
    pub eth_mtu_size: u16,
    pub ipv4_port: u16,
    pub ipv6_port: u16,
    pub control: u8,
    pub ipv6_tcp_options: u16,
    pub tcp_wsf: u8,
    pub ipv6_tcp_wsf: u8,
    pub ipv4_tos: u8,
    pub ipv4_cache_id: u8,
    pub ipv6_cache_id: u8,
    pub ipv4_alt_cid_len: u8,
    pub ipv4_alt_cid: [u8; 11],
    pub ipv4_vid_len: u8,
    pub ipv4_vid: [u8; 11],
    pub ipv4_ttl: u8,
    pub ipv6_flow_lbl: u16,
    pub ipv6_traffic_class: u8,
    pub ipv6_hop_limit: u8,
    pub ipv6_nd_reach_time: u32,
    pub ipv6_nd_rexmit_timer: u32,
    pub ipv6_nd_stale_timeout: u32,
    pub ipv6_dup_addr_detect_count: u8,
    pub ipv6_gw_advrt_mtu: u32,
    pub def_timeout: u16,
    pub abort_timer: u8,
    pub iscsi_options: u16,
    pub iscsi_max_pdu_size: u16,
    pub iscsi_first_burst_len: u16,
    pub iscsi_max_outstnd_r2t: u16,
    pub iscsi_max_burst_len: u16,
    pub iscsi_name: [u8; 224],
}

pub const QL4_CHAP_MAX_NAME_LEN: c_int = 256;
pub const QL4_CHAP_MAX_SECRET_LEN: c_int = 100;
pub const LOCAL_CHAP: c_int = 0;
pub const BIDI_CHAP: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql4_chap_format {
    pub intr_chap_name: [u8; QL4_CHAP_MAX_NAME_LEN],
    pub intr_secret: [u8; QL4_CHAP_MAX_SECRET_LEN],
    pub target_chap_name: [u8; QL4_CHAP_MAX_NAME_LEN],
    pub target_secret: [u8; QL4_CHAP_MAX_SECRET_LEN],
    pub intr_chap_name_length: u16,
    pub intr_secret_length: u16,
    pub target_chap_name_length: u16,
    pub target_secret_length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_address_format {
    pub ip_type: u8,
    pub ip_address: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql4_conn_info {
    pub dest_port: u16,
    pub dest_ipaddr: ip_address_format,
    pub chap: ql4_chap_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql4_boot_session_info {
    pub target_name: [u8; 224],
    pub conn_list: [ql4_conn_info; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql4_boot_tgt_info {
    pub boot_pri_sess: ql4_boot_session_info,
    pub boot_sec_sess: ql4_boot_session_info,
}

//
// Linux Host Adapter structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_qla_host {
// Linux adapter configuration data
    pub flags: c_ulong,

    pub dpc_flags: c_ulong,

    pub /: *mut *mut *mut Scsi_Host host; / pointer to host data,
    pub tot_ddbs: u32,
    pub iocb_cnt: u16,
    pub iocb_hiwat: u16,
// SRB cache.
pub const SRB_MIN_REQ: c_int = 128;
    pub srb_mempool: *mut mempool_t,
// pci information
    pub pdev: *mut pci_dev,
    pub /: *mut *mut *mut isp_reg __iomem reg; / Base I/O address,
    pub pio_address: c_ulong,
    pub pio_length: c_ulong,
pub const MIN_IOBASE_LEN: c_uint = 0x100;
    pub req_q_count: u16,
    pub host_no: c_ulong,
// NVRAM registers
    pub nvram: *mut eeprom_data,
    pub ____cacheline_aligned: spinlock_t hardware_lock,
    pub eeprom_cmd_data: u32,
// Counters for general statistics
    pub isr_count: u64,
    pub adapter_error_count: u64,
    pub device_error_count: u64,
    pub total_io_count: u64,
    pub total_mbytes_xferred: u64,
    pub link_failure_count: u64,
    pub invalid_crc_count: u64,
    pub bytes_xfered: u32,
    pub spurious_int_count: u32,
    pub aborted_io_count: u32,
    pub io_timeout_count: u32,
    pub mailbox_timeout_count: u32,
    pub seconds_since_last_intr: u32,
    pub seconds_since_last_heartbeat: u32,
    pub mac_index: u32,
// Info Needed for Management App
// --- From GetFwVersion ---
    pub firmware_version: [u32; 2],
    pub patch_number: u32,
    pub build_number: u32,
    pub board_id: u32,
// --- From Init_FW ---
// init_cb_t *init_cb;
    pub firmware_options: u16,
    pub alias: [u8; 32],
    pub name_string: [u8; 256],
    pub heartbeat_interval: u8,
// --- From FlashSysInfo ---
    pub my_mac: [u8; MAC_ADDR_LEN],
    pub serial_number: [u8; 16],
    pub port_num: u16,
// --- From GetFwState ---
    pub firmware_state: u32,
    pub addl_fw_state: u32,
// Linux kernel thread
    pub dpc_thread: *mut workqueue_struct,
    pub dpc_work: work_struct,
// Linux timer thread
    pub timer: timer_list,
    pub timer_active: u32,
// Recovery Timers
    pub check_relogin_timeouts: core::sync::atomic::AtomicI32,
    pub retry_reset_ha_cnt: u32,
    pub /: *mut *mut uint32_t isp_reset_timer; / reset test timer,
    pub /: *mut *mut uint32_t nic_reset_timer; / simulated nic reset test timer,
    pub eh_start: c_int,
    pub free_srb_q: list_head,
    pub free_srb_q_count: u16,
    pub num_srbs_allocated: u16,
// DMA Memory Block
    pub queues: *mut c_void,
    pub queues_dma: dma_addr_t,
    pub queues_len: c_ulong,

// request and response queue variables
    pub request_dma: dma_addr_t,
    pub request_ring: *mut queue_entry,
    pub request_ptr: *mut queue_entry,
    pub response_dma: dma_addr_t,
    pub response_ring: *mut queue_entry,
    pub response_ptr: *mut queue_entry,
    pub shadow_regs_dma: dma_addr_t,
    pub shadow_regs: *mut shadow_regs,
    pub /: *mut *mut uint16_t request_in; / Current indexes.,
    pub request_out: u16,
    pub response_in: u16,
    pub response_out: u16,
// aen queue variables
    pub /: *mut *mut uint16_t aen_q_count; / Number of available aen_q entries,
    pub /: *mut *mut uint16_t aen_in; / Current indexes,
    pub aen_out: u16,
    pub aen_q: [aen; MAX_AEN_ENTRIES],
    pub /: *mut *mut ql4_aen_log aen_log;/ tracks all aens,
// This mutex protects several threads to do mailbox commands
// concurrently.
//
    pub mbox_sem: mutex,
// temporary mailbox status registers
    pub mbox_status_count: volatile uint8_t,
    pub mbox_status: [volatile uint32_t; MBOX_REG_COUNT],
// FW ddb index map
    pub fw_ddb_index_map: [*mut ddb_entry; MAX_DDB_ENTRIES],
// Saved srb for status continuation entry processing
    pub status_srb: *mut srb,
    pub acb_version: u8,
// qla82xx specific fields
    pub /: *mut *mut *mut device_reg_82xx __iomem qla4_82xx_reg; / Base I/O address,
    pub /: *mut *mut unsigned long nx_pcibase; / Base I/O address,
    pub /: *mut *mut *mut uint8_t nx_db_rd_ptr; / Doorbell read pointer,
    pub /: *mut *mut unsigned long nx_db_wr_ptr; / Door bell write pointer,
    pub first_page_group_start: c_ulong,
    pub first_page_group_end: c_ulong,
    pub crb_win: u32,
    pub curr_window: u32,
    pub ddr_mn_window: u32,
    pub mn_win_crb: c_ulong,
    pub ms_win_crb: c_ulong,
    pub qdr_sn_window: c_int,
    pub hw_lock: rwlock_t,
    pub func_num: u16,
    pub link_width: c_int,
    pub nx_legacy_intr: qla4_8xxx_legacy_intr_set,
    pub nx_crb_mask: u32,
    pub revision_id: u8,
    pub fw_heartbeat_counter: u32,
    pub isp_ops: *mut isp_operations,
    pub hw: ql82xx_hw_data,
    pub nx_dev_init_timeout: u32,
    pub nx_reset_timeout: u32,
    pub fw_dump: *mut c_void,
    pub fw_dump_size: u32,
    pub fw_dump_capture_mask: u32,
    pub fw_dump_tmplt_hdr: *mut c_void,
    pub fw_dump_tmplt_size: u32,
    pub fw_dump_skip_size: u32,
    pub mbx_intr_comp: completion,
    pub ip_config: ipaddress_config,
    pub iface_ipv4: *mut iscsi_iface,
    pub iface_ipv6_0: *mut iscsi_iface,
    pub iface_ipv6_1: *mut iscsi_iface,
// --- From About Firmware ---
    pub fw_info: about_fw_info,
    pub /: *mut *mut uint32_t fw_uptime_secs; / seconds elapsed since fw bootup,
    pub /: *mut *mut uint32_t fw_uptime_msecs; / milliseconds beyond elapsed seconds,
    pub /: *mut *mut uint16_t def_timeout; / Default login timeout,
    pub flash_state: u32,
pub const QLFLASH_WAITING: c_int = 0;
pub const QLFLASH_READING: c_int = 1;
pub const QLFLASH_WRITING: c_int = 2;
    pub chap_dma_pool: *mut dma_pool,
    pub /: *mut *mut *mut uint8_t chap_list; / CHAP table cache,
    pub chap_sem: mutex,
pub const CHAP_DMA_BLOCK_SIZE: c_int = 512;
    pub task_wq: *mut workqueue_struct,
    pub BITS_PER_LONG]: unsigned long ddb_idx_map[MAX_DDB_ENTRIES /,
pub const SYSFS_FLAG_FW_SEL_BOOT: c_int = 2;
    pub boot_kset: *mut iscsi_boot_kset,
    pub boot_tgt: ql4_boot_tgt_info,
    pub phy_port_num: u16,
    pub phy_port_cnt: u16,
    pub iscsi_pci_func_cnt: u16,
    pub model_name: [u8; 16],
    pub disable_acb_comp: completion,
    pub fw_ddb_dma_pool: *mut dma_pool,
pub const DDB_DMA_BLOCK_SIZE: c_int = 512;
    pub pri_ddb_idx: u16,
    pub sec_ddb_idx: u16,
    pub is_reset: c_int,
    pub temperature: u16,
// event work list
    pub work_list: list_head,
    pub work_lock: spinlock_t,
// mbox iocb
pub const MAX_MRB: c_int = 128;
    pub active_mrb_array: [*mut mrb; MAX_MRB],
    pub mrb_index: u32,
    pub reg_tbl: *mut u32,
    pub reset_tmplt: qla4_83xx_reset_template,
    pub address: *mut *mut *mut device_reg_83xx __iomem qla4_83xx_reg; / Base I/O,
    pub pf_bit: u32,
    pub idc_info: qla4_83xx_idc_information,
    pub saved_acb: *mut addr_ctrl_blk,
    pub notify_idc_comp: c_int,
    pub notify_link_up_comp: c_int,
    pub idc_extend_tmo: c_int,
    pub idc_comp: completion,
    pub link_up_comp: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql4_task_data {
    pub ha: *mut scsi_qla_host,
    pub iocb_req_cnt: u8,
    pub data_dma: dma_addr_t,
    pub req_buffer: *mut c_void,
    pub req_dma: dma_addr_t,
    pub req_len: u32,
    pub resp_buffer: *mut c_void,
    pub resp_dma: dma_addr_t,
    pub resp_len: u32,
    pub task: *mut iscsi_task,
    pub sts: passthru_status,
    pub task_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_endpoint {
    pub host: *mut Scsi_Host,
    pub dst_addr: sockaddr_storage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qla_conn {
    pub qla_ep: *mut qla_endpoint,
}

extern "C" {
    pub fn is_qla4032(is_qla4010(ha: ha) || is_qla4022(ha) ||) -> return;
}
extern "C" {
    pub fn is_qla8022(is_qla8042(ha: ha) || is_qla8032(ha) ||) -> return;
}
extern "C" {
    pub fn ql4xxx_sem_spinlock(ha: *mut *mut scsi_qla_host, sem_mask: u32, sem_bits: u32) -> c_int;
}
extern "C" {
    pub fn ql4xxx_sem_unlock(ha: *mut *mut scsi_qla_host, sem_mask: u32);
}
extern "C" {
    pub fn ql4xxx_sem_lock(ha: *mut *mut scsi_qla_host, sem_mask: u32, sem_bits: u32) -> c_int;
}
// ---------------------------------------------------------------------------
// Defines for qla4xxx_initialize_adapter() and qla4xxx_recover_adapter()
pub const INIT_ADAPTER: c_int = 0;
pub const RESET_ADAPTER: c_int = 1;
pub const PRESERVE_DDB_LIST: c_int = 0;
pub const REBUILD_DDB_LIST: c_int = 1;
// Defines for process_aen()
pub const PROCESS_ALL_AENS: c_int = 0;
pub const FLUSH_DDB_CHANGED_AENS: c_int = 1;
// Defines for udev events
pub const QL4_UEVENT_CODE_FW_DUMP: c_int = 0;
