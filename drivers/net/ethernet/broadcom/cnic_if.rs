//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/cnic_if.h
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


// cnic_if.h: QLogic cnic core network driver.
//
// Copyright (c) 2006-2014 Broadcom Corporation
// Copyright (c) 2014-2015 QLogic Corporation
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

pub const CNIC_ULP_RDMA: c_int = 0;
pub const CNIC_ULP_ISCSI: c_int = 1;
pub const CNIC_ULP_FCOE: c_int = 2;
pub const CNIC_ULP_L4: c_int = 3;
pub const MAX_CNIC_ULP_TYPE_EXT: c_int = 3;
pub const MAX_CNIC_ULP_TYPE: c_int = 4;
// Use CPU native page size up to 16K for cnic ring sizes.

pub const CNIC_PAGE_BITS: c_int = 14;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kwqe {
    pub kwqe_op_flag: u32,
pub const KWQE_QID_SHIFT: c_int = 8;
pub const KWQE_OPCODE_MASK: c_uint = 0x00ff0000;
pub const KWQE_OPCODE_SHIFT: c_int = 16;

pub const KWQE_LAYER_MASK: c_uint = 0x70000000;
pub const KWQE_LAYER_SHIFT: c_int = 28;

    pub kwqe_info0: u32,
    pub kwqe_info1: u32,
    pub kwqe_info2: u32,
    pub kwqe_info3: u32,
    pub kwqe_info4: u32,
    pub kwqe_info5: u32,
    pub kwqe_info6: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kwqe_16 {
    pub kwqe_info0: u32,
    pub kwqe_info1: u32,
    pub kwqe_info2: u32,
    pub kwqe_info3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcqe {
    pub kcqe_info0: u32,
    pub kcqe_info1: u32,
    pub kcqe_info2: u32,
    pub kcqe_info3: u32,
    pub kcqe_info4: u32,
    pub kcqe_info5: u32,
    pub kcqe_info6: u32,
    pub kcqe_op_flag: u32,

}

pub const MAX_CNIC_CTL_DATA: c_int = 64;
pub const MAX_DRV_CTL_DATA: c_int = 64;
pub const CNIC_CTL_STOP_CMD: c_int = 1;
pub const CNIC_CTL_START_CMD: c_int = 2;
pub const CNIC_CTL_COMPLETION_CMD: c_int = 3;
pub const CNIC_CTL_STOP_ISCSI_CMD: c_int = 4;
pub const CNIC_CTL_FCOE_STATS_GET_CMD: c_int = 5;
pub const CNIC_CTL_ISCSI_STATS_GET_CMD: c_int = 6;
pub const DRV_CTL_IO_WR_CMD: c_uint = 0x101;
pub const DRV_CTL_IO_RD_CMD: c_uint = 0x102;
pub const DRV_CTL_CTX_WR_CMD: c_uint = 0x103;
pub const DRV_CTL_CTXTBL_WR_CMD: c_uint = 0x104;
pub const DRV_CTL_RET_L5_SPQ_CREDIT_CMD: c_uint = 0x105;
pub const DRV_CTL_START_L2_CMD: c_uint = 0x106;
pub const DRV_CTL_STOP_L2_CMD: c_uint = 0x107;
pub const DRV_CTL_RET_L2_SPQ_CREDIT_CMD: c_uint = 0x10c;
pub const DRV_CTL_ISCSI_STOPPED_CMD: c_uint = 0x10d;
pub const DRV_CTL_ULP_REGISTER_CMD: c_uint = 0x10e;
pub const DRV_CTL_ULP_UNREGISTER_CMD: c_uint = 0x10f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_ctl_completion {
    pub cid: u32,
    pub opcode: u8,
    pub error: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_ctl_info {
    pub cmd: c_int,
    pub comp: cnic_ctl_completion,
    pub bytes: [c_char; MAX_CNIC_CTL_DATA],
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_ctl_spq_credit {
    pub credit_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_ctl_io {
    pub cid_addr: u32,
    pub offset: u32,
    pub data: u32,
    pub dma_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_ctl_l2_ring {
    pub client_id: u32,
    pub cid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_ctl_register_data {
    pub ulp_type: c_int,
    pub fcoe_features: fcoe_capabilities,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_ctl_info {
    pub cmd: c_int,
    pub drv_state: c_int,
pub const DRV_NOP: c_int = 0;
pub const DRV_ACTIVE: c_int = 1;
pub const DRV_INACTIVE: c_int = 2;
pub const DRV_UNLOADED: c_int = 3;
    pub credit: drv_ctl_spq_credit,
    pub io: drv_ctl_io,
    pub ring: drv_ctl_l2_ring,
    pub ulp_type: c_int,
    pub register_data: drv_ctl_register_data,
    pub bytes: [c_char; MAX_DRV_CTL_DATA],
    pub data: },
}

pub const MAX_NPIV_ENTRIES: c_int = 64;
pub const FC_NPIV_WWN_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_fc_npiv_tbl {
    pub wwpn: [u8; MAX_NPIV_ENTRIES][FC_NPIV_WWN_SIZE],
    pub wwnn: [u8; MAX_NPIV_ENTRIES][FC_NPIV_WWN_SIZE],
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_ops {
    pub cnic_owner: *mut module,
// Calls to these functions are protected by RCU.  When
// unregistering, we wait for any calls to complete before
// continuing.
//
    pub ): *mut *mut *mut int (cnic_handler)(void , void,
    pub ): *mut *mut *mut int (cnic_ctl)(void , struct cnic_ctl_info,
}

pub const MAX_CNIC_VEC: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_irq {
    pub vector: c_uint,
    pub status_blk: *mut c_void,
    pub status_blk_map: dma_addr_t,
    pub status_blk_num: u32,
    pub status_blk_num2: u32,
    pub irq_flags: u32,
pub const CNIC_IRQ_FL_MSIX: c_uint = 0x00000001;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_eth_dev {
    pub drv_owner: *mut module,
    pub drv_state: u32,
pub const CNIC_DRV_STATE_REGD: c_uint = 0x00000001;
pub const CNIC_DRV_STATE_USING_MSIX: c_uint = 0x00000002;
pub const CNIC_DRV_STATE_NO_ISCSI_OOO: c_uint = 0x00000004;
pub const CNIC_DRV_STATE_NO_ISCSI: c_uint = 0x00000008;
pub const CNIC_DRV_STATE_NO_FCOE: c_uint = 0x00000010;
pub const CNIC_DRV_STATE_HANDLES_IRQ: c_uint = 0x00000020;
    pub chip_id: u32,
    pub max_kwqe_pending: u32,
    pub pdev: *mut pci_dev,
    pub io_base: *mut void __iomem,
    pub io_base2: *mut void __iomem,
    pub iro_arr: *const c_void,
    pub ctx_tbl_offset: u32,
    pub ctx_tbl_len: u32,
    pub ctx_blk_size: c_int,
    pub starting_cid: u32,
    pub max_iscsi_conn: u32,
    pub max_fcoe_conn: u32,
    pub max_rdma_conn: u32,
    pub fcoe_init_cid: u32,
    pub max_fcoe_exchanges: u32,
    pub fcoe_wwn_port_name_hi: u32,
    pub fcoe_wwn_port_name_lo: u32,
    pub fcoe_wwn_node_name_hi: u32,
    pub fcoe_wwn_node_name_lo: u32,
    pub iscsi_l2_client_id: u16,
    pub iscsi_l2_cid: u16,
    pub iscsi_mac: [u8; ETH_ALEN],
    pub num_irq: c_int,
    pub irq_arr: [cnic_irq; MAX_CNIC_VEC],
    pub ): *mut *mut cnic_ops , void,
    pub ): *mut *mut int (drv_unregister_cnic)(struct net_device,
    pub u32): *mut *mut kwqe [],,
    pub u32): *mut *mut kwqe_16 [],,
    pub ): *mut *mut *mut int (drv_ctl)(struct net_device , struct drv_ctl_info,
    pub ): *mut cnic_fc_npiv_tbl,
    pub reserved1: [c_ulong; 2],
    pub addr_drv_info_to_mcp: *mut drv_info_to_mcp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_sockaddr {
    pub v4: sockaddr_in,
    pub v6: sockaddr_in6,
    pub local: },
    pub v4: sockaddr_in,
    pub v6: sockaddr_in6,
    pub remote: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_sock {
    pub dev: *mut cnic_dev,
    pub context: *mut c_void,
    pub src_ip: [u32; 4],
    pub dst_ip: [u32; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub vlan_id: u16,
    pub old_ha: [c_uchar; ETH_ALEN],
    pub ha: [c_uchar; ETH_ALEN],
    pub mtu: u32,
    pub cid: u32,
    pub l5_cid: u32,
    pub pg_cid: u32,
    pub ulp_type: c_int,
    pub ka_timeout: u32,
    pub ka_interval: u32,
    pub ka_max_probe_count: u8,
    pub tos: u8,
    pub ttl: u8,
    pub snd_seq_scale: u8,
    pub rcv_buf: u32,
    pub snd_buf: u32,
    pub seed: u32,
    pub tcp_flags: c_ulong,
pub const SK_TCP_NO_DELAY_ACK: c_uint = 0x1;
pub const SK_TCP_KEEP_ALIVE: c_uint = 0x2;
pub const SK_TCP_NAGLE: c_uint = 0x4;
pub const SK_TCP_TIMESTAMP: c_uint = 0x8;
pub const SK_TCP_SACK: c_uint = 0x10;
pub const SK_TCP_SEG_SCALING: c_uint = 0x20;
    pub flags: c_ulong,
pub const SK_F_INUSE: c_int = 0;
pub const SK_F_OFFLD_COMPLETE: c_int = 1;
pub const SK_F_OFFLD_SCHED: c_int = 2;
pub const SK_F_PG_OFFLD_COMPLETE: c_int = 3;
pub const SK_F_CONNECT_START: c_int = 4;
pub const SK_F_IPV6: c_int = 5;
pub const SK_F_CLOSING: c_int = 7;
pub const SK_F_HW_ERR: c_int = 8;
    pub ref_count: core::sync::atomic::AtomicI32,
    pub state: u32,
    pub kwqe1: kwqe,
    pub kwqe2: kwqe,
    pub kwqe3: kwqe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_dev {
    pub netdev: *mut net_device,
    pub pcidev: *mut pci_dev,
    pub regview: *mut void __iomem,
    pub list: list_head,
    pub ulp_ctx): *mut c_void,
    pub ulp_type): *mut *mut *mut int (unregister_device)(struct cnic_dev dev, int,
    pub num_wqes): u32,
    pub num_wqes): u32,
    pub ): *mut c_void,
    pub ): *mut *mut int (cm_destroy)(struct cnic_sock,
    pub ): *mut *mut *mut int (cm_connect)(struct cnic_sock , struct cnic_sockaddr,
    pub ): *mut *mut int (cm_abort)(struct cnic_sock,
    pub ): *mut *mut int (cm_close)(struct cnic_sock,
    pub ulp_type): *mut *mut *mut *mut cnic_dev (cm_select_dev)(sockaddr_in , int,
    pub data_size): *mut *mut char data, u16,
    pub ): *mut *mut *mut int (get_fc_npiv_tbl)(struct cnic_dev , struct cnic_fc_npiv_tbl,
    pub flags: c_ulong,
pub const CNIC_F_CNIC_UP: c_int = 1;
pub const CNIC_F_BNX2_CLASS: c_int = 3;
pub const CNIC_F_BNX2X_CLASS: c_int = 4;
    pub ref_count: core::sync::atomic::AtomicI32,
    pub mac_addr: [u8; ETH_ALEN],
    pub max_iscsi_conn: c_int,
    pub max_fcoe_conn: c_int,
    pub max_rdma_conn: c_int,
    pub max_fcoe_exchanges: c_int,
    pub stats_addr: *mut drv_info_to_mcp,
    pub fcoe_cap: *mut fcoe_capabilities,
    pub cnic_priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cnic_ulp_ops {
// Calls to these functions are protected by RCU.  When
// unregistering, we wait for any calls to complete before
// continuing.
//
    pub dev): *mut *mut void (cnic_init)(struct cnic_dev,
    pub dev): *mut *mut void (cnic_exit)(struct cnic_dev,
    pub ulp_ctx): *mut *mut void (cnic_start)(void,
    pub ulp_ctx): *mut *mut void (cnic_stop)(void,
    pub num_cqes): u32,
    pub vid): *mut *mut *mut void (indicate_netevent)(void ulp_ctx, unsigned long event, u16,
    pub ): *mut *mut void (cm_connect_complete)(struct cnic_sock,
    pub ): *mut *mut void (cm_close_complete)(struct cnic_sock,
    pub ): *mut *mut void (cm_abort_complete)(struct cnic_sock,
    pub ): *mut *mut void (cm_remote_close)(struct cnic_sock,
    pub ): *mut *mut void (cm_remote_abort)(struct cnic_sock,
    pub data_size): *mut *mut char data, u16,
    pub ulp_ctx): *mut *mut int (cnic_get_stats)(void,
    pub owner: *mut module,
    pub ref_count: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn cnic_register_driver(ulp_type: c_int, ulp_ops: *mut cnic_ulp_ops) -> c_int;
}
extern "C" {
    pub fn cnic_unregister_driver(ulp_type: c_int) -> c_int;
}
