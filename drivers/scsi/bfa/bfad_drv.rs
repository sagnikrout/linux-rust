//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfad_drv.h
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
// Contains base driver definitions.
//
// bfa_drv.h Linux driver data structures.
//

pub const FC_PORTSPEED_8GBIT: c_uint = 0x10;

//
// BFAD flags
//
pub const BFAD_MSIX_ON: c_uint = 0x00000001;
pub const BFAD_HAL_INIT_DONE: c_uint = 0x00000002;
pub const BFAD_DRV_INIT_DONE: c_uint = 0x00000004;
pub const BFAD_CFG_PPORT_DONE: c_uint = 0x00000008;
pub const BFAD_HAL_START_DONE: c_uint = 0x00000010;
pub const BFAD_PORT_ONLINE: c_uint = 0x00000020;
pub const BFAD_RPORT_ONLINE: c_uint = 0x00000040;
pub const BFAD_FCS_INIT_DONE: c_uint = 0x00000080;
pub const BFAD_HAL_INIT_FAIL: c_uint = 0x00000100;
pub const BFAD_FC4_PROBE_DONE: c_uint = 0x00000200;
pub const BFAD_PORT_DELETE: c_uint = 0x00000001;
pub const BFAD_INTX_ON: c_uint = 0x00000400;
pub const BFAD_EEH_BUSY: c_uint = 0x00000800;
pub const BFAD_EEH_PCI_CHANNEL_IO_PERM_FAILURE: c_uint = 0x00001000;
//
// BFAD related definition
//

pub const BFAD_STOP_TIMEOUT: c_int = 30;

//
// BFAD configuration parameter default values
//
pub const BFAD_LUN_QUEUE_DEPTH: c_int = 32;

pub const BFAD_MAX_SECTORS: c_uint = 0xFFFF  /* 32 MB */;

pub const MAX_MSIX_ENTRY: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_msix_s {
    pub bfad: *mut bfad_s,
    pub msix: msix_entry,
    pub name: [c_char; 32],
}

//
// Only append to the enums defined here to avoid any versioning
// needed between trace utility and driver version
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfad_port_pvb_type {
    BFAD_PORT_PHYS_BASE = 0,
    BFAD_PORT_PHYS_VPORT = 1,
    BFAD_PORT_VF_BASE = 2,
    BFAD_PORT_VF_VPORT = 3,
}

//
// PORT data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_port_s {
    pub list_entry: list_head,
    pub bfad: *mut bfad_s,
    pub fcs_port: *mut bfa_fcs_lport_s,
    pub roles: u32,
    pub flags: i32,
    pub supported_fc4s: u32,
    pub pvb_type: bfad_port_pvb_type,
    pub /: *mut *mut *mut bfad_im_port_s im_port; / IM specific data,
// port debugfs specific data
    pub port_debugfs_root: *mut dentry,
}

//
// VPORT data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_vport_s {
    pub drv_port: bfad_port_s,
    pub fcs_vport: bfa_fcs_vport_s,
    pub comp_del: *mut completion,
    pub list_entry: list_head,
}

//
// VF data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_vf_s {
    pub fcs_vf: bfa_fcs_vf_t,
    pub /: *mut *mut bfad_port_s base_port; / base port for vf,
    pub bfad: *mut bfad_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_cfg_param_s {
    pub rport_del_timeout: u32,
    pub ioc_queue_depth: u32,
    pub lun_queue_depth: u32,
    pub io_max_sge: u32,
    pub binding_method: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfad_tmp_buf {
// From struct bfa_adapter_attr_s
    pub manufacturer: [c_char; BFA_ADAPTER_MFG_NAME_LEN],
    pub serial_num: [c_char; BFA_ADAPTER_SERIAL_NUM_LEN],
    pub model: [c_char; BFA_ADAPTER_MODEL_NAME_LEN],
    pub fw_ver: [c_char; BFA_VERSION_LEN],
    pub optrom_ver: [c_char; BFA_VERSION_LEN],
// From struct bfa_ioc_pci_attr_s
    pub /: *mut *mut u8 chip_rev[BFA_IOC_CHIP_REV_LEN]; / chip revision,
    pub wwn: [wwn_t; BFA_FCS_MAX_LPORTS],
}

// BFAD state machine events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfad_sm_event {
    BFAD_E_CREATE			= 1,
    BFAD_E_KTHREAD_CREATE_FAILED	= 2,
    BFAD_E_INIT			= 3,
    BFAD_E_INIT_SUCCESS		= 4,
    BFAD_E_HAL_INIT_FAILED		= 5,
    BFAD_E_INIT_FAILED		= 6,
    BFAD_E_FCS_EXIT_COMP		= 7,
    BFAD_E_EXIT_COMP		= 8,
    BFAD_E_STOP			= 9
}

extern "C" {
    pub fn void(: *mut *mut bfad_sm_t)(struct bfad_s, bfad_sm_event: enum) -> typedef;
}
//
// BFAD (PCI function) data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_s {
    pub /: *mut *mut bfad_sm_t sm; / state machine,
    pub list_entry: list_head,
    pub bfa: bfa_s,
    pub bfa_fcs: bfa_fcs_s,
    pub pcidev: *mut pci_dev,
    pub pci_name: *const c_char,
    pub hal_pcidev: bfa_pcidev_s,
    pub pci_attr: bfa_ioc_pci_attr_s,
    pub pci_bar0_kva: *mut void __iomem,
    pub pci_bar2_kva: *mut void __iomem,
    pub comp: completion,
    pub suspend: completion,
    pub enable_comp: completion,
    pub disable_comp: completion,
    pub disable_active: bfa_boolean_t,
    pub /: *mut *mut bfad_port_s pport; / physical port of the BFAD,
    pub meminfo: bfa_meminfo_s,
    pub ioc_cfg: bfa_iocfc_cfg_s,
    pub /: *mut *mut u32 inst_no; / BFAD instance number,
    pub bfad_flags: u32,
    pub bfad_lock: spinlock_t,
    pub bfad_tsk: *mut task_struct,
    pub cfg_data: bfad_cfg_param_s,
    pub msix_tab: [bfad_msix_s; MAX_MSIX_ENTRY],
    pub nvec: c_int,
    pub adapter_name: [c_char; BFA_ADAPTER_SYM_NAME_LEN],
    pub port_name: [c_char; BFA_ADAPTER_SYM_NAME_LEN],
    pub hal_tmo: timer_list,
    pub hs_start: c_ulong,
    pub /: *mut *mut *mut bfad_im_s im; / IM specific data,
    pub trcmod: *mut bfa_trc_mod_s,
    pub plog_buf: bfa_plog_s,
    pub ref_count: c_int,
    pub tmp_buf: bfad_tmp_buf,
    pub link_stats: fc_host_statistics,
    pub pbc_vport_list: list_head,
// debugfs specific data
    pub regdata: *mut c_char,
    pub reglen: u32,
    pub bfad_dentry_files: [*mut dentry; 5],
    pub free_aen_q: list_head,
    pub active_aen_q: list_head,
    pub aen_list: [bfa_aen_entry_s; BFA_AEN_MAX_ENTRY],
    pub bfad_aen_spinlock: spinlock_t,
    pub vport_list: list_head,
}

//
// RPORT data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_rport_s {
    pub fcs_rport: bfa_fcs_rport_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_buf_info {
    pub virt: *mut c_void,
    pub phys: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_fcxp {
    pub port: *mut bfad_port_s,
    pub bfa_rport: *mut bfa_rport_s,
    pub req_status: bfa_status_t,
    pub tag: u16,
    pub rsp_len: u16,
    pub rsp_maxlen: u16,
    pub use_ireqbuf: u8,
    pub use_irspbuf: u8,
    pub num_req_sgles: u32,
    pub num_rsp_sgles: u32,
    pub fchs: fchs_s,
    pub reqbuf_info: *mut c_void,
    pub rspbuf_info: *mut c_void,
    pub req_sge: *mut bfa_sge_s,
    pub rsp_sge: *mut bfa_sge_s,
    pub send_cbfn: fcxp_send_cb_t,
    pub send_cbarg: *mut c_void,
    pub bfa_fcxp: *mut c_void,
    pub comp: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfad_hal_comp {
    pub status: bfa_status_t,
    pub comp: completion,
}

extern "C" {
    pub fn bfad_cfg_pport(bfad: *mut bfad_s, role: bfa_lport_role) -> bfa_status_t;
}
extern "C" {
    pub fn bfad_drv_init(bfad: *mut bfad_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfad_start_ops(bfad: *mut bfad_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfad_drv_start(bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_uncfg_pport(bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_stop(bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_fcs_stop(bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_remove_intr(bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_hal_mem_release(bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_hcb_comp(arg: *mut c_void, status: bfa_status_t);
}
extern "C" {
    pub fn bfad_setup_intr(bfad: *mut bfad_s) -> c_int;
}
extern "C" {
    pub fn bfad_remove_intr(bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_update_hal_cfg(bfa_cfg: *mut bfa_iocfc_cfg_s);
}
extern "C" {
    pub fn bfad_hal_mem_alloc(bfad: *mut bfad_s) -> bfa_status_t;
}
extern "C" {
    pub fn bfad_bfa_tmo(t: *mut timer_list);
}
extern "C" {
    pub fn bfad_init_timer(bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_pci_init(pdev: *mut pci_dev, bfad: *mut bfad_s) -> c_int;
}
extern "C" {
    pub fn bfad_pci_uninit(pdev: *mut pci_dev, bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_worker(ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn bfad_debugfs_init(port: *mut bfad_port_s);
}
extern "C" {
    pub fn bfad_debugfs_exit(port: *mut bfad_port_s);
}
extern "C" {
    pub fn bfad_pci_remove(pdev: *mut pci_dev);
}
extern "C" {
    pub fn bfad_pci_probe(pdev: *mut pci_dev, pid: *const pci_device_id) -> c_int;
}
extern "C" {
    pub fn bfad_rport_online_wait(bfad: *mut bfad_s);
}
extern "C" {
    pub fn bfad_get_linkup_delay(bfad: *mut bfad_s) -> c_int;
}
extern "C" {
    pub fn bfad_install_msix_handler(bfad: *mut bfad_s) -> c_int;
}
