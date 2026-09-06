//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_ioc.h
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

//
// BFA timer declarations
//
extern "C" {
    pub fn void(: *mut *mut bfa_timer_cbfn_t)(void) -> typedef;
}
//
// BFA timer data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_timer_s {
    pub qe: list_head,
    pub timercb: bfa_timer_cbfn_t,
    pub arg: *mut c_void,
    pub /: *mut *mut int timeout; / in millisecs,
}

//
// Timer module structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_timer_mod_s {
    pub timer_q: list_head,
}

extern "C" {
    pub fn bfa_timer_beat(mod: *mut bfa_timer_mod_s);
}
extern "C" {
    pub fn bfa_timer_stop(timer: *mut bfa_timer_s);
}
//
// Generic Scatter Gather Element used by driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_sge_s {
    pub sg_len: u32,
    pub sg_addr: *mut c_void,
}

// Macro flag: #define bfa_sge_to_be(_x)

// Macro flag: #define bfa_sge_to_le(_x)

//
// BFA memory resources
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_mem_dma_s {
    pub /: *mut *mut list_head qe; / Queue of DMA elements,
    pub /: *mut *mut u32 mem_len; / Total Length in Bytes,
    pub /: *mut *mut *mut u8 kva; / kernel virtual address,
    pub /: *mut *mut u64 dma; / dma address if DMA memory,
    pub /: *mut *mut *mut u8 kva_curp; / kva allocation cursor,
    pub /: *mut *mut u64 dma_curp; / dma allocation cursor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_mem_kva_s {
    pub /: *mut *mut list_head qe; / Queue of KVA elements,
    pub /: *mut *mut u32 mem_len; / Total Length in Bytes,
    pub /: *mut *mut *mut u8 kva; / kernel virtual address,
    pub /: *mut *mut *mut u8 kva_curp; / kva allocation cursor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_meminfo_s {
    pub dma_info: bfa_mem_dma_s,
    pub kva_info: bfa_mem_kva_s,
}

// BFA memory segment setup helpers
// BFA dma memory segments iterator

// Get the corresponding dma buf kva for a req - from the tag

// Get the corresponding dma buf pa for a req - from the tag

//
// PCI device information required by IOC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_pcidev_s {
    pub pci_slot: c_int,
    pub pci_func: u8,
    pub device_id: u16,
    pub ssid: u16,
    pub pci_bar_kva: *mut void __iomem,
}

//
// Structure used to remember the DMA-able memory block's KVA and Physical
// Address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_dma_s {
    pub /: *mut *mut *mut void kva; / ! Kernel virtual address,
    pub /: *mut *mut u64 pa; / ! Physical address,
}

pub const BFA_DMA_ALIGN_SZ: c_int = 256;

//
// smem size for Crossbow and Catapult
//
pub const BFI_SMEM_CB_SIZE: c_uint = 0x200000U	/* ! 2MB for crossbow	*/;
pub const BFI_SMEM_CT_SIZE: c_uint = 0x280000U	/* ! 2.5MB for catapult	*/;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_regs_s {
    pub hfn_mbox_cmd: *mut void __iomem,
    pub hfn_mbox: *mut void __iomem,
    pub lpu_mbox_cmd: *mut void __iomem,
    pub lpu_mbox: *mut void __iomem,
    pub lpu_read_stat: *mut void __iomem,
    pub pss_ctl_reg: *mut void __iomem,
    pub pss_err_status_reg: *mut void __iomem,
    pub app_pll_fast_ctl_reg: *mut void __iomem,
    pub app_pll_slow_ctl_reg: *mut void __iomem,
    pub ioc_sem_reg: *mut void __iomem,
    pub ioc_usage_sem_reg: *mut void __iomem,
    pub ioc_init_sem_reg: *mut void __iomem,
    pub ioc_usage_reg: *mut void __iomem,
    pub host_page_num_fn: *mut void __iomem,
    pub heartbeat: *mut void __iomem,
    pub ioc_fwstate: *mut void __iomem,
    pub alt_ioc_fwstate: *mut void __iomem,
    pub ll_halt: *mut void __iomem,
    pub alt_ll_halt: *mut void __iomem,
    pub err_set: *mut void __iomem,
    pub ioc_fail_sync: *mut void __iomem,
    pub shirq_isr_next: *mut void __iomem,
    pub shirq_msk_next: *mut void __iomem,
    pub smem_page_start: *mut void __iomem,
    pub smem_pg0: u32,
}

//
// IOC Mailbox structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_mbox_cmd_s {
    pub qe: list_head,
    pub msg: [u32; BFI_IOC_MSGSZ],
}

//
// IOC mailbox module
//
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_ioc_mbox_mcfunc_t)(void, m: *mut bfi_mbmsg_s) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_mbox_mod_s {
    pub /: *mut *mut list_head cmd_q; / pending mbox queue,
    pub /: *mut *mut int nmclass; / number of handlers,
    pub /: *mut *mut bfa_ioc_mbox_mcfunc_t cbfn; / message handlers,
    pub cbarg: *mut c_void,
    pub mbhdlr: [}; BFI_MC_MAX],
}

//
// IOC callback function interfaces
//
extern "C" {
    pub fn void(bfa: *mut *mut bfa_ioc_enable_cbfn_t)(void, status: bfa_status) -> typedef;
}
extern "C" {
    pub fn void(bfa: *mut *mut bfa_ioc_disable_cbfn_t)(void) -> typedef;
}
extern "C" {
    pub fn void(bfa: *mut *mut bfa_ioc_hbfail_cbfn_t)(void) -> typedef;
}
extern "C" {
    pub fn void(bfa: *mut *mut bfa_ioc_reset_cbfn_t)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_cbfn_s {
    pub enable_cbfn: bfa_ioc_enable_cbfn_t,
    pub disable_cbfn: bfa_ioc_disable_cbfn_t,
    pub hbfail_cbfn: bfa_ioc_hbfail_cbfn_t,
    pub reset_cbfn: bfa_ioc_reset_cbfn_t,
}

//
// IOC event notification mechanism.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ioc_event {
    IOC_E_RESET		= 1,	/*  IOC reset request		*/
    IOC_E_ENABLE		= 2,	/*  IOC enable request		*/
    IOC_E_DISABLE		= 3,	/*  IOC disable request	*/
    IOC_E_DETACH		= 4,	/*  driver detach cleanup	*/
    IOC_E_ENABLED		= 5,	/*  f/w enabled		*/
    IOC_E_FWRSP_GETATTR	= 6,	/*  IOC get attribute response	*/
    IOC_E_DISABLED		= 7,	/*  f/w disabled		*/
    IOC_E_PFFAILED		= 8,	/*  failure notice by iocpf sm	*/
    IOC_E_HBFAIL		= 9,	/*  heartbeat failure		*/
    IOC_E_HWERROR		= 10,	/*  hardware error interrupt	*/
    IOC_E_TIMEOUT		= 11,	/*  timeout			*/
    IOC_E_HWFAILED		= 12,	/*  PCI mapping failure notice	*/
}

extern "C" {
    pub fn void(fsm: *mut *mut bfa_ioc_sm_t)(struct bfa_ioc_s, ioc_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_ioc_event_e {
    BFA_IOC_E_ENABLED	= 1,
    BFA_IOC_E_DISABLED	= 2,
    BFA_IOC_E_FAILED	= 3,
}

extern "C" {
    pub fn void(: *mut *mut bfa_ioc_notify_cbfn_t)(void, bfa_ioc_event_e: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_notify_s {
    pub qe: list_head,
    pub cbfn: bfa_ioc_notify_cbfn_t,
    pub cbarg: *mut c_void,
}

//
// Initialize a IOC event notification structure
//

//
// IOCPF state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iocpf_event {
    IOCPF_E_ENABLE		= 1,	/*  IOCPF enable request	*/
    IOCPF_E_DISABLE		= 2,	/*  IOCPF disable request	*/
    IOCPF_E_STOP		= 3,	/*  stop on driver detach	*/
    IOCPF_E_FWREADY		= 4,	/*  f/w initialization done	*/
    IOCPF_E_FWRSP_ENABLE	= 5,	/*  enable f/w response	*/
    IOCPF_E_FWRSP_DISABLE	= 6,	/*  disable f/w response	*/
    IOCPF_E_FAIL		= 7,	/*  failure notice by ioc sm	*/
    IOCPF_E_INITFAIL	= 8,	/*  init fail notice by ioc sm	*/
    IOCPF_E_GETATTRFAIL	= 9,	/*  init fail notice by ioc sm	*/
    IOCPF_E_SEMLOCKED	= 10,	/*  h/w semaphore is locked	*/
    IOCPF_E_TIMEOUT		= 11,	/*  f/w response timeout	*/
    IOCPF_E_SEM_ERROR	= 12,	/*  h/w sem mapping error	*/
}

extern "C" {
    pub fn void(fsm: *mut *mut bfa_iocpf_sm_t)(struct bfa_iocpf_s, iocpf_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_iocpf_s {
    pub fsm: bfa_iocpf_sm_t,
    pub ioc: *mut bfa_ioc_s,
    pub fw_mismatch_notified: bfa_boolean_t,
    pub auto_recover: bfa_boolean_t,
    pub poll_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_s {
    pub fsm: bfa_ioc_sm_t,
    pub bfa: *mut bfa_s,
    pub pcidev: bfa_pcidev_s,
    pub timer_mod: *mut bfa_timer_mod_s,
    pub ioc_timer: bfa_timer_s,
    pub sem_timer: bfa_timer_s,
    pub hb_timer: bfa_timer_s,
    pub hb_count: u32,
    pub notify_q: list_head,
    pub dbg_fwsave: *mut c_void,
    pub dbg_fwsave_len: c_int,
    pub dbg_fwsave_once: bfa_boolean_t,
    pub clscode: bfi_pcifn_class,
    pub ioc_regs: bfa_ioc_regs_s,
    pub trcmod: *mut bfa_trc_mod_s,
    pub stats: bfa_ioc_drv_stats_s,
    pub fcmode: bfa_boolean_t,
    pub pllinit: bfa_boolean_t,
    pub /: *mut *mut bfa_boolean_t stats_busy; / outstanding stats,
    pub port_id: u8,
    pub attr_dma: bfa_dma_s,
    pub attr: *mut bfi_ioc_attr_s,
    pub cbfn: *mut bfa_ioc_cbfn_s,
    pub mbox_mod: bfa_ioc_mbox_mod_s,
    pub ioc_hwif: *mut bfa_ioc_hwif_s,
    pub iocpf: bfa_iocpf_s,
    pub asic_gen: bfi_asic_gen,
    pub asic_mode: bfi_asic_mode,
    pub port0_mode: bfi_port_mode,
    pub port1_mode: bfi_port_mode,
    pub port_mode: bfa_mode_s,
    pub /: *mut *mut u8 ad_cap_bm; / adapter cap bit mask,
    pub /: *mut *mut u8 port_mode_cfg; / config port mode,
    pub ioc_aen_seq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_hwif_s {
    pub m): *mut *mut *mut bfa_status_t (ioc_pll_init) (void __iomem rb, enum bfi_asic_mode,
    pub ioc): *mut *mut bfa_boolean_t (ioc_firmware_lock) (struct bfa_ioc_s,
    pub ioc): *mut *mut void (ioc_firmware_unlock) (struct bfa_ioc_s,
    pub ioc): *mut *mut void (ioc_reg_init) (struct bfa_ioc_s,
    pub ioc): *mut *mut void (ioc_map_port) (struct bfa_ioc_s,
    pub msix): bfa_boolean_t,
    pub ioc): *mut *mut void (ioc_notify_fail) (struct bfa_ioc_s,
    pub ioc): *mut *mut void (ioc_ownership_reset) (struct bfa_ioc_s,
    pub ioc): *mut *mut bfa_boolean_t (ioc_sync_start) (struct bfa_ioc_s,
    pub ioc): *mut *mut void (ioc_sync_join) (struct bfa_ioc_s,
    pub ioc): *mut *mut void (ioc_sync_leave) (struct bfa_ioc_s,
    pub ioc): *mut *mut void (ioc_sync_ack) (struct bfa_ioc_s,
    pub ioc): *mut *mut bfa_boolean_t (ioc_sync_complete) (struct bfa_ioc_s,
    pub ioc): *mut *mut bfa_boolean_t (ioc_lpu_read_stat) (struct bfa_ioc_s,
    pub fwstate): bfi_ioc_state,
    pub ioc): *mut *mut bfi_ioc_state (ioc_get_fwstate) (struct bfa_ioc_s,
    pub fwstate): bfi_ioc_state,
    pub ioc): *mut *mut bfi_ioc_state (ioc_get_alt_fwstate) (struct bfa_ioc_s,
}

//
// Queue element to wait for room in request queue. FIFO order is
// maintained when fullfilling requests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_reqq_wait_s {
    pub qe: list_head,
    pub cbarg): *mut *mut void (qresume) (void,
    pub cbarg: *mut c_void,
}

extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_cbfn_t) (void, complete: bfa_boolean_t) -> typedef;
}
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_cbfn_status_t) (void, status: bfa_status_t) -> typedef;
}
//
// Generic BFA callback element.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cb_qe_s {
    pub qe: list_head,
    pub cbfn_status: bfa_cb_cbfn_status_t,
    pub cbfn: bfa_cb_cbfn_t,
}

//
// ASIC block configurtion related
//
extern "C" {
    pub fn void(: *mut *mut bfa_ablk_cbfn_t)(void, bfa_status: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ablk_s {
    pub ioc: *mut bfa_ioc_s,
    pub cfg: *mut bfa_ablk_cfg_s,
    pub pcifn: *mut u16,
    pub dma_addr: bfa_dma_s,
    pub busy: bfa_boolean_t,
    pub mb: bfa_mbox_cmd_s,
    pub cbfn: bfa_ablk_cbfn_t,
    pub cbarg: *mut c_void,
    pub ioc_notify: bfa_ioc_notify_s,
    pub ablk_dma: bfa_mem_dma_s,
}

//
// SFP module specific
//
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_sfp_t) (void, status: bfa_status_t) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_sfp_s {
    pub dev: *mut c_void,
    pub ioc: *mut bfa_ioc_s,
    pub trcmod: *mut bfa_trc_mod_s,
    pub sfpmem: *mut sfp_mem_s,
    pub cbfn: bfa_cb_sfp_t,
    pub cbarg: *mut c_void,
    pub /: *mut *mut bfi_sfp_mem_e memtype; / mem access type,
    pub status: u32,
    pub mbcmd: bfa_mbox_cmd_s,
    pub /: *mut *mut *mut u8 dbuf_kva; / dma buf virtual address,
    pub /: *mut *mut u64 dbuf_pa; / dma buf physical address,
    pub ioc_notify: bfa_ioc_notify_s,
    pub media: *mut bfa_defs_sfp_media_e,
    pub portspeed: bfa_port_speed,
    pub state_query_cbfn: bfa_cb_sfp_t,
    pub state_query_cbarg: *mut c_void,
    pub lock: u8,
    pub /: *mut *mut u8 data_valid; / data in dbuf is valid,
    pub /: *mut *mut u8 state; / sfp state,
    pub state_query_lock: u8,
    pub sfp_dma: bfa_mem_dma_s,
    pub /: *mut *mut u8 is_elb; / eloopback,
}

extern "C" {
    pub fn bfa_sfp_meminfo() -> u32;
}
extern "C" {
    pub fn bfa_sfp_memclaim(diag: *mut bfa_sfp_s, dm_kva: *mut u8, dm_pa: u64);
}
extern "C" {
    pub fn bfa_sfp_intr(bfaarg: *mut c_void, msg: *mut bfi_mbmsg_s);
}
//
// Flash module specific
//
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_flash_t) (void, status: bfa_status_t) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_flash_s {
    pub /: *mut *mut *mut bfa_ioc_s ioc; / back pointer to ioc,
    pub trcmod: *mut bfa_trc_mod_s,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub rsv: [u8; 3],
    pub /: *mut *mut u32 op_busy; / operation busy flag,
    pub /: *mut *mut u32 residue; / residual length,
    pub /: *mut *mut u32 offset; / offset,
    pub /: *mut *mut bfa_status_t status; / status,
    pub /: *mut *mut *mut u8 dbuf_kva; / dma buf virtual address,
    pub /: *mut *mut u64 dbuf_pa; / dma buf physical address,
    pub /: *mut *mut bfa_reqq_wait_s reqq_wait; / to wait for room in reqq,
    pub /: *mut *mut bfa_cb_flash_t cbfn; / user callback function,
    pub /: *mut *mut *mut void cbarg; / user callback arg,
    pub /: *mut *mut *mut u8 ubuf; / user supplied buffer,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / comp: BFA callback qelem,
    pub /: *mut *mut u32 addr_off; / partition address offset,
    pub /: *mut *mut bfa_mbox_cmd_s mb; / mailbox,
    pub /: *mut *mut bfa_ioc_notify_s ioc_notify; / ioc event notify,
    pub flash_dma: bfa_mem_dma_s,
}

extern "C" {
    pub fn bfa_flash_meminfo(mincfg: bfa_boolean_t) -> u32;
}
//
// DIAG module specific
//
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_diag_t) (void, status: bfa_status_t) -> typedef;
}
//
// Firmware ping test results
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_results_fwping {
    pub /: *mut *mut u32 data; / store the corrupted data,
    pub status: u32,
    pub dmastatus: u32,
    pub rsvd: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_qtest_result_s {
    pub status: u32,
    pub /: *mut *mut u16 count; / successful queue test count,
    pub queue: u8,
    pub /: *mut *mut u8 rsvd; / 64-bit align,
}

//
// Firmware ping test results
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_fwping_s {
    pub result: *mut bfa_diag_results_fwping,
    pub cbfn: bfa_cb_diag_t,
    pub cbarg: *mut c_void,
    pub data: u32,
    pub lock: u8,
    pub rsv: [u8; 3],
    pub status: u32,
    pub count: u32,
    pub mbcmd: bfa_mbox_cmd_s,
    pub /: *mut *mut *mut u8 dbuf_kva; / dma buf virtual address,
    pub /: *mut *mut u64 dbuf_pa; / dma buf physical address,
}

//
// Temperature sensor query results
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_results_tempsensor_s {
    pub status: u32,
    pub /: *mut *mut u16 temp; / 10-bit A/D value,
    pub /: *mut *mut u16 brd_temp; / 9-bit board temp,
    pub /: *mut *mut u8 ts_junc; / show junction tempsensor,
    pub /: *mut *mut u8 ts_brd; / show board tempsensor,
    pub /: *mut *mut u8 rsvd[6]; / keep 8 bytes alignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_tsensor_s {
    pub cbfn: bfa_cb_diag_t,
    pub cbarg: *mut c_void,
    pub temp: *mut bfa_diag_results_tempsensor_s,
    pub lock: u8,
    pub rsv: [u8; 3],
    pub status: u32,
    pub mbcmd: bfa_mbox_cmd_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_sfpshow_s {
    pub sfpmem: *mut sfp_mem_s,
    pub cbfn: bfa_cb_diag_t,
    pub cbarg: *mut c_void,
    pub lock: u8,
    pub static_data: u8,
    pub rsv: [u8; 2],
    pub status: u32,
    pub mbcmd: bfa_mbox_cmd_s,
    pub /: *mut *mut *mut u8 dbuf_kva; / dma buf virtual address,
    pub /: *mut *mut u64 dbuf_pa; / dma buf physical address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_led_s {
    pub mbcmd: bfa_mbox_cmd_s,
    pub /: *mut *mut bfa_boolean_t lock; / 1: ledtest is operating,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_beacon_s {
    pub mbcmd: bfa_mbox_cmd_s,
    pub /: *mut *mut bfa_boolean_t state; / port beacon state,
    pub /: *mut *mut bfa_boolean_t link_e2e; / link beacon state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_diag_s {
    pub dev: *mut c_void,
    pub ioc: *mut bfa_ioc_s,
    pub trcmod: *mut bfa_trc_mod_s,
    pub fwping: bfa_diag_fwping_s,
    pub tsensor: bfa_diag_tsensor_s,
    pub sfpshow: bfa_diag_sfpshow_s,
    pub ledtest: bfa_diag_led_s,
    pub beacon: bfa_diag_beacon_s,
    pub result: *mut c_void,
    pub timer: bfa_timer_s,
    pub cbfn_beacon: bfa_cb_diag_beacon_t,
    pub cbfn: bfa_cb_diag_t,
    pub cbarg: *mut c_void,
    pub block: u8,
    pub timer_active: u8,
    pub rsvd: [u8; 2],
    pub status: u32,
    pub ioc_notify: bfa_ioc_notify_s,
    pub diag_dma: bfa_mem_dma_s,
}

extern "C" {
    pub fn bfa_diag_meminfo() -> u32;
}
extern "C" {
    pub fn bfa_diag_memclaim(diag: *mut bfa_diag_s, dm_kva: *mut u8, dm_pa: u64);
}
//
// PHY module specific
//
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_phy_t) (void, status: bfa_status_t) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_phy_s {
    pub /: *mut *mut *mut bfa_ioc_s ioc; / back pointer to ioc,
    pub /: *mut *mut *mut bfa_trc_mod_s trcmod; / trace module,
    pub /: *mut *mut u8 instance; / port instance,
    pub /: *mut *mut u8 op_busy; / operation busy flag,
    pub rsv: [u8; 2],
    pub /: *mut *mut u32 residue; / residual length,
    pub /: *mut *mut u32 offset; / offset,
    pub /: *mut *mut bfa_status_t status; / status,
    pub /: *mut *mut *mut u8 dbuf_kva; / dma buf virtual address,
    pub /: *mut *mut u64 dbuf_pa; / dma buf physical address,
    pub /: *mut *mut bfa_reqq_wait_s reqq_wait; / to wait for room in reqq,
    pub /: *mut *mut bfa_cb_phy_t cbfn; / user callback function,
    pub /: *mut *mut *mut void cbarg; / user callback arg,
    pub /: *mut *mut *mut u8 ubuf; / user supplied buffer,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / comp: BFA callback qelem,
    pub /: *mut *mut u32 addr_off; / phy address offset,
    pub /: *mut *mut bfa_mbox_cmd_s mb; / mailbox,
    pub /: *mut *mut bfa_ioc_notify_s ioc_notify; / ioc event notify,
    pub phy_dma: bfa_mem_dma_s,
}

extern "C" {
    pub fn bfa_phy_busy(ioc: *mut bfa_ioc_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_phy_meminfo(mincfg: bfa_boolean_t) -> u32;
}
extern "C" {
    pub fn bfa_phy_intr(phyarg: *mut c_void, msg: *mut bfi_mbmsg_s);
}
//
// FRU module specific
//
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_fru_t) (void, status: bfa_status_t) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fru_s {
    pub /: *mut *mut *mut bfa_ioc_s ioc; / back pointer to ioc,
    pub /: *mut *mut *mut bfa_trc_mod_s trcmod; / trace module,
    pub /: *mut *mut u8 op_busy; / operation busy flag,
    pub rsv: [u8; 3],
    pub /: *mut *mut u32 residue; / residual length,
    pub /: *mut *mut u32 offset; / offset,
    pub /: *mut *mut bfa_status_t status; / status,
    pub /: *mut *mut *mut u8 dbuf_kva; / dma buf virtual address,
    pub /: *mut *mut u64 dbuf_pa; / dma buf physical address,
    pub /: *mut *mut bfa_reqq_wait_s reqq_wait; / to wait for room in reqq,
    pub /: *mut *mut bfa_cb_fru_t cbfn; / user callback function,
    pub /: *mut *mut *mut void cbarg; / user callback arg,
    pub /: *mut *mut *mut u8 ubuf; / user supplied buffer,
    pub /: *mut *mut bfa_cb_qe_s hcb_qe; / comp: BFA callback qelem,
    pub /: *mut *mut u32 addr_off; / fru address offset,
    pub /: *mut *mut bfa_mbox_cmd_s mb; / mailbox,
    pub /: *mut *mut bfa_ioc_notify_s ioc_notify; / ioc event notify,
    pub fru_dma: bfa_mem_dma_s,
    pub trfr_cmpl: u8,
}

extern "C" {
    pub fn bfa_fruvpd_get_max_size(fru: *mut bfa_fru_s, max_size: *mut u32) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_fru_meminfo(mincfg: bfa_boolean_t) -> u32;
}
extern "C" {
    pub fn bfa_fru_intr(fruarg: *mut c_void, msg: *mut bfi_mbmsg_s);
}
//
// Driver Config( dconf) specific
//
pub const BFI_DCONF_SIGNATURE: c_uint = 0xabcdabcd;
pub const BFI_DCONF_VERSION: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_dconf_hdr_s {
    pub signature: u32,
    pub version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_dconf_s {
    pub hdr: bfa_dconf_hdr_s,
    pub lun_mask: bfa_lunmask_cfg_s,
    pub throttle_cfg: bfa_throttle_cfg_s,
}

//
// DCONF state machine events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_dconf_event {
    BFA_DCONF_SM_INIT		= 1,	/* dconf Init */
    BFA_DCONF_SM_FLASH_COMP		= 2,	/* read/write to flash */
    BFA_DCONF_SM_WR			= 3,	/* binding change, map */
    BFA_DCONF_SM_TIMEOUT		= 4,	/* Start timer */
    BFA_DCONF_SM_EXIT		= 5,	/* exit dconf module */
    BFA_DCONF_SM_IOCDISABLE		= 6,	/* IOC disable event */
}

extern "C" {
    pub fn void(fsm: *mut *mut bfa_dconf_sm_t)(struct bfa_dconf_mod_s, bfa_dconf_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_dconf_mod_s {
    pub sm: bfa_dconf_sm_t,
    pub instance: u8,
    pub read_data_valid: bfa_boolean_t,
    pub min_cfg: bfa_boolean_t,
    pub timer: bfa_timer_s,
    pub bfa: *mut bfa_s,
    pub bfad: *mut c_void,
    pub trcmod: *mut c_void,
    pub dconf: *mut bfa_dconf_s,
    pub kva_seg: bfa_mem_kva_s,
}

extern "C" {
    pub fn bfa_dconf_modinit(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_dconf_modexit(bfa: *mut bfa_s);
}
extern "C" {
    pub fn bfa_dconf_update(bfa: *mut bfa_s) -> bfa_status_t;
}
//
// IOC specfic macros
//

//
// IOC mailbox interface
//
extern "C" {
    pub fn bfa_ioc_mbox_queue(ioc: *mut bfa_ioc_s, cmd: *mut bfa_mbox_cmd_s);
}
extern "C" {
    pub fn bfa_ioc_mbox_isr(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_mbox_send(ioc: *mut bfa_ioc_s, ioc_msg: *mut c_void, len: c_int);
}
extern "C" {
    pub fn bfa_ioc_msgget(ioc: *mut bfa_ioc_s, mbmsg: *mut c_void) -> bfa_boolean_t;
}
//
// IOC interfaces
//

extern "C" {
    pub fn bfa_ioc_pll_init(ioc: *mut bfa_ioc_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_ioc_cb_pll_init(rb: *mut void __iomem, mode: bfi_asic_mode) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_ioc_ct_pll_init(rb: *mut void __iomem, mode: bfi_asic_mode) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_ioc_ct2_pll_init(rb: *mut void __iomem, mode: bfi_asic_mode) -> bfa_status_t;
}

extern "C" {
    pub fn bfa_ioc_set_cb_hwif(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_set_ct_hwif(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_set_ct2_hwif(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_ct2_poweron(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_detach(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_suspend(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_mem_claim(ioc: *mut bfa_ioc_s, dm_kva: *mut u8, dm_pa: u64);
}
extern "C" {
    pub fn bfa_ioc_enable(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_disable(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_intx_claim(ioc: *mut bfa_ioc_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_ioc_isr(ioc: *mut bfa_ioc_s, msg: *mut bfi_mbmsg_s);
}
extern "C" {
    pub fn bfa_ioc_error_isr(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_is_operational(ioc: *mut bfa_ioc_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_ioc_is_disabled(ioc: *mut bfa_ioc_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_ioc_is_acq_addr(ioc: *mut bfa_ioc_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_ioc_fw_mismatch(ioc: *mut bfa_ioc_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_ioc_adapter_is_disabled(ioc: *mut bfa_ioc_s) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_ioc_reset_fwstate(ioc: *mut bfa_ioc_s);
}
extern "C" {
    pub fn bfa_ioc_get_type(ioc: *mut bfa_ioc_s) -> bfa_ioc_type_e;
}
extern "C" {
    pub fn bfa_ioc_get_adapter_serial_num(ioc: *mut bfa_ioc_s, serial_num: *mut c_char);
}
extern "C" {
    pub fn bfa_ioc_get_adapter_fw_ver(ioc: *mut bfa_ioc_s, fw_ver: *mut c_char);
}
extern "C" {
    pub fn bfa_ioc_get_adapter_optrom_ver(ioc: *mut bfa_ioc_s, optrom_ver: *mut c_char);
}
extern "C" {
    pub fn bfa_ioc_get_adapter_model(ioc: *mut bfa_ioc_s, model: *mut c_char);
}
extern "C" {
    pub fn bfa_ioc_get_pci_chip_rev(ioc: *mut bfa_ioc_s, chip_rev: *mut c_char);
}
extern "C" {
    pub fn bfa_ioc_get_state(ioc: *mut bfa_ioc_s) -> bfa_ioc_state;
}
extern "C" {
    pub fn bfa_ioc_get_attr(ioc: *mut bfa_ioc_s, ioc_attr: *mut bfa_ioc_attr_s);
}
extern "C" {
    pub fn bfa_ioc_debug_memclaim(ioc: *mut bfa_ioc_s, dbg_fwsave: *mut c_void);
}
extern "C" {
    pub fn bfa_ioc_fwsig_invalidate(ioc: *mut bfa_ioc_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_ioc_sem_get(sem_reg: *mut void __iomem) -> bfa_boolean_t;
}
extern "C" {
    pub fn bfa_ioc_aen_post(ioc: *mut bfa_ioc_s, event: bfa_ioc_aen_event);
}
extern "C" {
    pub fn bfa_ioc_fw_stats_get(ioc: *mut bfa_ioc_s, stats: *mut c_void) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_ioc_fw_stats_clear(ioc: *mut bfa_ioc_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfa_ioc_debug_save_ftrc(ioc: *mut bfa_ioc_s);
}
//
// asic block configuration related APIs
//
extern "C" {
    pub fn bfa_ablk_meminfo() -> u32;
}
extern "C" {
    pub fn bfa_ablk_memclaim(ablk: *mut bfa_ablk_s, dma_kva: *mut u8, dma_pa: u64);
}
extern "C" {
    pub fn bfa_ablk_attach(ablk: *mut bfa_ablk_s, ioc: *mut bfa_ioc_s);
}
//
// bfa mfg wwn API functions
//
extern "C" {
    pub fn bfa_ioc_get_mac(ioc: *mut bfa_ioc_s) -> mac_t;
}
extern "C" {
    pub fn bfa_ioc_get_mfg_mac(ioc: *mut bfa_ioc_s) -> mac_t;
}
//
// F/W Image Size & Chunk
//
extern "C" {
    pub fn bfi_image_cb_get_chunk(_arg: off) -> return;
}
extern "C" {
    pub fn bfi_image_ct_get_chunk(_arg: off) -> return;
}
extern "C" {
    pub fn bfi_image_ct2_get_chunk(_arg: off) -> return;
}
//
// CNA TRCMOD declaration
//
// !!! Only append to the enums defined here to avoid any versioning
// !!! needed between trace utility and driver version
//
