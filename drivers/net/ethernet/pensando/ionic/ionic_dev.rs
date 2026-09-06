//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/pensando/ionic/ionic_dev.h
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
// Copyright(c) 2017 - 2019 Pensando Systems, Inc

pub const IONIC_MAX_TX_DESC: c_int = 8192;
pub const IONIC_MAX_RX_DESC: c_int = 16384;
pub const IONIC_MIN_TXRX_DESC: c_int = 64;
pub const IONIC_DEF_TXRX_DESC: c_int = 1024;
pub const IONIC_RX_FILL_THRESHOLD: c_int = 16;
pub const IONIC_RX_FILL_DIV: c_int = 8;

pub const IONIC_LIFS_MAX: c_int = 1024;
pub const IONIC_WATCHDOG_SECS: c_int = 5;
pub const IONIC_ITR_COAL_USEC_DEFAULT: c_int = 64;
pub const IONIC_DEV_CMD_REG_VERSION: c_int = 1;
pub const IONIC_DEV_INFO_REG_COUNT: c_int = 32;
pub const IONIC_DEV_CMD_REG_COUNT: c_int = 32;

pub const IONIC_EXPDB_64B_WQE_LG2: c_int = 6;
pub const IONIC_EXPDB_128B_WQE_LG2: c_int = 7;
pub const IONIC_EXPDB_256B_WQE_LG2: c_int = 8;
pub const IONIC_EXPDB_512B_WQE_LG2: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev_bar {
    pub vaddr: *mut void __iomem,
    pub bus_addr: phys_addr_t,
    pub len: c_ulong,
    pub res_index: c_int,
}

// Registers
// Device commands
// Port commands
// LIF commands
// RDMA commands
// Events
// I/O
// SR/IOV

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_devinfo {
    pub asic_type: u8,
    pub asic_rev: u8,
    pub 1]: char fw_version[IONIC_DEVINFO_FWVERS_BUFLEN +,
    pub 1]: char serial_num[IONIC_DEVINFO_SERIAL_BUFLEN +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_dev {
    pub dev_info_regs: *mut ionic_dev_info_regs __iomem,
    pub dev_cmd_regs: *mut ionic_dev_cmd_regs __iomem,
    pub hwstamp_regs: *mut ionic_hwstamp_regs __iomem,
    pub last_check_time: atomic_long_t,
    pub last_hb_time: c_ulong,
    pub last_fw_hb: u32,
    pub fw_hb_ready: bool,
    pub fw_status_ready: bool,
    pub fw_generation: u8,
    pub opcode: u8,
    pub db_pages: *mut u64 __iomem,
    pub phy_db_pages: dma_addr_t,
    pub intr_ctrl: *mut ionic_intr __iomem,
    pub intr_status: *mut u64 __iomem,
    pub /: *mut *mut mutex cmb_inuse_lock; / for cmb_inuse,
    pub cmb_inuse: *mut c_ulong,
    pub phy_cmb_pages: dma_addr_t,
    pub cmb_npages: u32,
    pub phy_cmb_expdb64_pages: dma_addr_t,
    pub phy_cmb_expdb128_pages: dma_addr_t,
    pub phy_cmb_expdb256_pages: dma_addr_t,
    pub phy_cmb_expdb512_pages: dma_addr_t,
    pub port_info_sz: u32,
    pub port_info: *mut ionic_port_info,
    pub port_info_pa: dma_addr_t,
    pub port_extra_stats_cache: ionic_port_extra_stats,
    pub link_down_count_init: bool,
    pub link_down_count_last: u16,
    pub link_down_count_total: u32,
    pub dev_info: ionic_devinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_buf_info {
    pub page: *mut page,
    pub dma_addr: dma_addr_t,
    pub page_offset: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_tx_desc_info {
    pub bytes: c_uint,
    pub nbufs: c_uint,
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
    pub act: xdp_action,
    pub 1]: ionic_buf_info bufs[MAX_SKB_FRAGS +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rx_desc_info {
    pub nbufs: c_uint,
    pub bufs: [ionic_buf_info; IONIC_RX_MAX_FRAGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_admin_desc_info {
    pub ctx: *mut c_void,
}

pub const IONIC_QUEUE_NAME_MAX_SZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_queue {
    pub dev: *mut device,
    pub lif: *mut ionic_lif,
    pub info: *mut c_void,
    pub tx_info: *mut ionic_tx_desc_info,
    pub rx_info: *mut ionic_rx_desc_info,
    pub admin_info: *mut ionic_admin_desc_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_cq {
    pub lif: *mut ionic_lif,
    pub bound_q: *mut ionic_queue,
    pub bound_intr: *mut ionic_intr_info,
    pub tail_idx: u16,
    pub done_color: bool,
    pub num_descs: c_uint,
    pub desc_size: c_uint,
    pub base: *mut c_void,
    pub base_pa: dma_addr_t,
    pub idev: *mut ionic_dev,
    pub ____cacheline_aligned_in_smp: },
    pub ionic: struct,
    pub index): ionic_intr_clean(idev->intr_ctrl,,
    pub index: intr->index =,
    pub q->tail_idx: unsigned int avail =,
    pub 1: avail += q->num_descs - q->head_idx -,
    pub 1: avail -= q->head_idx +,
    pub avail: return,
    pub want: return ionic_q_space_avail(q) >=,
    pub ionic): *mut void ionic_init_devinfo(struct ionic,
    pub ionic): *mut int ionic_dev_setup(struct ionic,
    pub ionic): *mut void ionic_dev_teardown(struct ionic,
    pub cmd): *mut *mut void ionic_dev_cmd_go(struct ionic_dev idev, union ionic_dev_cmd,
    pub idev): *mut u8 ionic_dev_cmd_status(struct ionic_dev,
    pub idev): *mut bool ionic_dev_cmd_done(struct ionic_dev,
    pub comp): *mut *mut void ionic_dev_cmd_comp(struct ionic_dev idev, union ionic_dev_cmd_comp,
    pub ver): *mut *mut void ionic_dev_cmd_identify(struct ionic_dev idev, u8,
    pub idev): *mut void ionic_dev_cmd_init(struct ionic_dev,
    pub idev): *mut void ionic_dev_cmd_reset(struct ionic_dev,
    pub idev): *mut void ionic_dev_cmd_port_identify(struct ionic_dev,
    pub idev): *mut void ionic_dev_cmd_port_init(struct ionic_dev,
    pub idev): *mut void ionic_dev_cmd_port_reset(struct ionic_dev,
    pub state): *mut *mut void ionic_dev_cmd_port_state(struct ionic_dev idev, u8,
    pub speed): *mut *mut void ionic_dev_cmd_port_speed(struct ionic_dev idev, u32,
    pub an_enable): *mut *mut void ionic_dev_cmd_port_autoneg(struct ionic_dev idev, u8,
    pub fec_type): *mut *mut void ionic_dev_cmd_port_fec(struct ionic_dev idev, u8,
    pub pause_type): *mut *mut void ionic_dev_cmd_port_pause(struct ionic_dev idev, u8,
    pub vfc): *mut ionic_vf_setattr_cmd,
    pub qver): u16 lif_type, u8 qtype, u8,
    pub ionic): *mut void ionic_vf_start(struct ionic,
    pub ver): *mut *mut void ionic_dev_cmd_lif_identify(struct ionic_dev idev, u8 type, u8,
    pub addr): dma_addr_t,
    pub lif_index): *mut *mut void ionic_dev_cmd_lif_reset(struct ionic_dev idev, u16,
    pub intr_index): u16 lif_index, u16,
    pub pid): *mut *mut int ionic_db_page_num(struct ionic_lif lif, int,
    pub idev): *mut void ionic_dev_cmd_discover_cmb(struct ionic_dev,
    pub ionic): *mut void ionic_map_cmb(struct ionic,
    pub desc_size): unsigned int num_descs, size_t,
    pub base_pa): *mut *mut *mut void ionic_cq_map(struct ionic_cq cq, void base, dma_addr_t,
    pub q): *mut *mut void ionic_cq_bind(struct ionic_cq cq, struct ionic_queue,
    pub cq): *mut *mut typedef bool (ionic_cq_cb)(struct ionic_cq,
    pub done_arg): *mut *mut typedef void (ionic_cq_done_cb)(void,
    pub done_arg): *mut c_void,
    pub in_napi): bool,
    pub pid): size_t sg_desc_size, unsigned int,
    pub ring_doorbell): *mut *mut void ionic_q_post(struct ionic_queue q, bool,
    pub pos): *mut *mut bool ionic_q_is_posted(struct ionic_queue q, unsigned int,
    pub ionic): *mut int ionic_heartbeat_check(struct ionic,
    pub idev): *mut bool ionic_is_fw_running(struct ionic_dev,
    pub work): *mut void ionic_doorbell_napi_work(struct work_struct,
    pub delay): *mut *mut void ionic_queue_doorbell_check(struct ionic ionic, int,
    pub q): *mut bool ionic_adminq_poke_doorbell(struct ionic_queue,
    pub q): *mut bool ionic_txq_poke_doorbell(struct ionic_queue,
    pub q): *mut bool ionic_rxq_poke_doorbell(struct ionic_queue,
    pub idev): *mut void ionic_reset_link_down_count(struct ionic_dev,
