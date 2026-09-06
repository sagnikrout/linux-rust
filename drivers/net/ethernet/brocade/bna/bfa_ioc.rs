//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/brocade/bna/bfa_ioc.h
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

// PCI device information required by IOC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_pcidev {
    pub pci_slot: c_int,
    pub pci_func: u8,
    pub device_id: u16,
    pub ssid: u16,
    pub pci_bar_kva: *mut void __iomem,
}

// Structure used to remember the DMA-able memory block's KVA and Physical
// Address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_dma {
    pub /: *mut *mut *mut void kva; / ! Kernel virtual address,
    pub /: *mut *mut u64 pa; / ! Physical address,
}

pub const BFA_DMA_ALIGN_SZ: c_int = 256;
// smem size for Crossbow and Catapult
pub const BFI_SMEM_CB_SIZE: c_uint = 0x200000U	/* ! 2MB for crossbow	*/;
pub const BFI_SMEM_CT_SIZE: c_uint = 0x280000U	/* ! 2.5MB for catapult	*/;
// BFA dma address assignment macro. (big endian format)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_regs {
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

// IOC Mailbox structures
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_mbox_cmd_cbfn_t)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_mbox_cmd {
    pub qe: list_head,
    pub cbfn: bfa_mbox_cmd_cbfn_t,
    pub cbarg: *mut c_void,
    pub msg: [u32; BFI_IOC_MSGSZ],
}

// IOC mailbox module
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_ioc_mbox_mcfunc_t)(void, m: *mut bfi_mbmsg) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_mbox_mod {
    pub /: *mut *mut list_head cmd_q; /!< pending mbox queue,
    pub /: *mut *mut int nmclass; /!< number of handlers,
    pub /: *mut *mut bfa_ioc_mbox_mcfunc_t cbfn; /!< message handlers,
    pub cbarg: *mut c_void,
    pub mbhdlr: [}; BFI_MC_MAX],
}

// IOC callback function interfaces
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
pub struct bfa_ioc_cbfn {
    pub enable_cbfn: bfa_ioc_enable_cbfn_t,
    pub disable_cbfn: bfa_ioc_disable_cbfn_t,
    pub hbfail_cbfn: bfa_ioc_hbfail_cbfn_t,
    pub reset_cbfn: bfa_ioc_reset_cbfn_t,
}

// IOC event notification mechanism.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_ioc_event {
    BFA_IOC_E_ENABLED	= 1,
    BFA_IOC_E_DISABLED	= 2,
    BFA_IOC_E_FAILED	= 3,
}

extern "C" {
    pub fn void(: *mut *mut bfa_ioc_notify_cbfn_t)(void, bfa_ioc_event: enum) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_notify {
    pub qe: list_head,
    pub cbfn: bfa_ioc_notify_cbfn_t,
    pub cbarg: *mut c_void,
}

// Initialize a IOC event notification structure

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_iocpf {
    pub e): *mut *mut *mut void (fsm)(struct bfa_iocpf s, enum iocpf_event,
    pub ioc: *mut bfa_ioc,
    pub fw_mismatch_notified: bool,
    pub auto_recover: bool,
    pub poll_time: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc {
    pub e): *mut *mut *mut void (fsm)(struct bfa_ioc s, enum ioc_event,
    pub bfa: *mut bfa,
    pub pcidev: bfa_pcidev,
    pub ioc_timer: timer_list,
    pub iocpf_timer: timer_list,
    pub sem_timer: timer_list,
    pub hb_timer: timer_list,
    pub hb_count: u32,
    pub notify_q: list_head,
    pub dbg_fwsave: *mut c_void,
    pub dbg_fwsave_len: c_int,
    pub dbg_fwsave_once: bool,
    pub clscode: bfi_pcifn_class,
    pub ioc_regs: bfa_ioc_regs,
    pub stats: bfa_ioc_drv_stats,
    pub fcmode: bool,
    pub pllinit: bool,
    pub /: *mut *mut bool stats_busy; /!< outstanding stats,
    pub port_id: u8,
    pub attr_dma: bfa_dma,
    pub attr: *mut bfi_ioc_attr,
    pub cbfn: *mut bfa_ioc_cbfn,
    pub mbox_mod: bfa_ioc_mbox_mod,
    pub ioc_hwif: *const bfa_ioc_hwif,
    pub iocpf: bfa_iocpf,
    pub asic_gen: bfi_asic_gen,
    pub asic_mode: bfi_asic_mode,
    pub port0_mode: bfi_port_mode,
    pub port1_mode: bfi_port_mode,
    pub port_mode: bfa_mode,
    pub /: *mut *mut u8 ad_cap_bm; /!< adapter cap bit mask,
    pub /: *mut *mut u8 port_mode_cfg; /!< config port mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_ioc_hwif {
    pub m): bfi_asic_mode,
    pub ioc): *mut *mut bool (ioc_firmware_lock) (struct bfa_ioc,
    pub ioc): *mut *mut void (ioc_firmware_unlock) (struct bfa_ioc,
    pub ioc): *mut *mut void (ioc_reg_init) (struct bfa_ioc,
    pub ioc): *mut *mut void (ioc_map_port) (struct bfa_ioc,
    pub msix): bool,
    pub ioc): *mut *mut void (ioc_notify_fail) (struct bfa_ioc,
    pub ioc): *mut *mut void (ioc_ownership_reset) (struct bfa_ioc,
    pub ioc): *mut *mut bool (ioc_sync_start) (struct bfa_ioc,
    pub ioc): *mut *mut void (ioc_sync_join) (struct bfa_ioc,
    pub ioc): *mut *mut void (ioc_sync_leave) (struct bfa_ioc,
    pub ioc): *mut *mut void (ioc_sync_ack) (struct bfa_ioc,
    pub ioc): *mut *mut bool (ioc_sync_complete) (struct bfa_ioc,
    pub ioc): *mut *mut bool (ioc_lpu_read_stat) (struct bfa_ioc,
    pub fwstate): bfi_ioc_state,
    pub ioc): *mut *mut bfi_ioc_state (ioc_get_fwstate) (struct bfa_ioc,
    pub fwstate): bfi_ioc_state,
    pub ioc): *mut *mut bfi_ioc_state (ioc_get_alt_fwstate) (struct bfa_ioc,
}

// IOC mailbox interface
extern "C" {
    pub fn bfa_nw_ioc_mbox_isr(ioc: *mut bfa_ioc);
}
// IOC interfaces

extern "C" {
    pub fn bfa_nw_ioc_set_ct_hwif(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_ioc_set_ct2_hwif(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_ioc_ct2_poweron(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_ioc_auto_recover(auto_recover: bool);
}
extern "C" {
    pub fn bfa_nw_ioc_detach(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_ioc_meminfo() -> u32;
}
extern "C" {
    pub fn bfa_nw_ioc_mem_claim(ioc: *mut bfa_ioc, dm_kva: *mut u8, dm_pa: u64);
}
extern "C" {
    pub fn bfa_nw_ioc_enable(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_ioc_disable(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_ioc_error_isr(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_ioc_is_disabled(ioc: *mut bfa_ioc) -> bool;
}
extern "C" {
    pub fn bfa_nw_ioc_is_operational(ioc: *mut bfa_ioc) -> bool;
}
extern "C" {
    pub fn bfa_nw_ioc_get_attr(ioc: *mut bfa_ioc, ioc_attr: *mut bfa_ioc_attr);
}
extern "C" {
    pub fn bfa_nw_ioc_fwsig_invalidate(ioc: *mut bfa_ioc) -> bfa_status;
}
extern "C" {
    pub fn bfa_nw_ioc_sem_get(sem_reg: *mut void __iomem) -> bool;
}
extern "C" {
    pub fn bfa_nw_ioc_sem_release(sem_reg: *mut void __iomem);
}
extern "C" {
    pub fn bfa_nw_ioc_hw_sem_release(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_ioc_get_mac(ioc: *mut bfa_ioc, mac: *mut u8);
}
extern "C" {
    pub fn bfa_nw_ioc_debug_memclaim(ioc: *mut bfa_ioc, dbg_fwsave: *mut c_void);
}
extern "C" {
    pub fn bfa_nw_ioc_debug_fwtrc(ioc: *mut bfa_ioc, trcdata: *mut c_void, trclen: *mut c_int) -> c_int;
}
extern "C" {
    pub fn bfa_nw_ioc_debug_fwsave(ioc: *mut bfa_ioc, trcdata: *mut c_void, trclen: *mut c_int) -> c_int;
}
//
// Timeout APIs
//
extern "C" {
    pub fn bfa_nw_ioc_timeout(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_ioc_hb_check(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_iocpf_timeout(ioc: *mut bfa_ioc);
}
extern "C" {
    pub fn bfa_nw_iocpf_sem_timeout(ioc: *mut bfa_ioc);
}
//
// F/W Image Size & Chunk
//
extern "C" {
    pub fn bfa_cb_image_get_size(asic_gen: bfi_asic_gen) -> u32;
}
//
// Flash module specific
//
extern "C" {
    pub fn void(cbarg: *mut *mut bfa_cb_flash) (void, status: bfa_status) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_flash {
    pub /: *mut *mut *mut bfa_ioc ioc; / back pointer to ioc,
    pub /: *mut *mut u32 type; / partition type,
    pub /: *mut *mut u8 instance; / partition instance,
    pub rsv: [u8; 3],
    pub /: *mut *mut u32 op_busy; / operation busy flag,
    pub /: *mut *mut u32 residue; / residual length,
    pub /: *mut *mut u32 offset; / offset,
    pub /: *mut *mut bfa_status status; / status,
    pub /: *mut *mut *mut u8 dbuf_kva; / dma buf virtual address,
    pub /: *mut *mut u64 dbuf_pa; / dma buf physical address,
    pub /: *mut *mut bfa_cb_flash cbfn; / user callback function,
    pub /: *mut *mut *mut void cbarg; / user callback arg,
    pub /: *mut *mut *mut u8 ubuf; / user supplied buffer,
    pub /: *mut *mut u32 addr_off; / partition address offset,
    pub /: *mut *mut bfa_mbox_cmd mb; / mailbox,
    pub /: *mut *mut bfa_ioc_notify ioc_notify; / ioc event notify,
}

extern "C" {
    pub fn bfa_nw_flash_meminfo() -> u32;
}
extern "C" {
    pub fn bfa_nw_flash_memclaim(flash: *mut bfa_flash, dm_kva: *mut u8, dm_pa: u64);
}
