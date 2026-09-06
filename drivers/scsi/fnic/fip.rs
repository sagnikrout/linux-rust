//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/fip.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

// Drop the cast from the standard definition

pub const FCOE_MAX_SIZE: c_uint = 0x082E;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fdls_vlan_state {
    FIP_VLAN_AVAIL,
    FIP_VLAN_SENT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fdls_fip_state {
    FDLS_FIP_INIT,
    FDLS_FIP_VLAN_DISCOVERY_STARTED,
    FDLS_FIP_FCF_DISCOVERY_STARTED,
    FDLS_FIP_FLOGI_STARTED,
    FDLS_FIP_FLOGI_COMPLETE,
}

//
// VLAN entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_vlan {
    pub list: list_head,
    pub /: *mut *mut uint16_t vid; / vlan ID,
    pub /: *mut *mut uint16_t sol_count; / no. of sols sent,
    pub /: *mut *mut uint16_t state; / state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_vlan_req {
    pub eth: ethhdr,
    pub fip: fip_header,
    pub mac_desc: fip_mac_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_vlan_notif {
    pub fip: fip_header,
    pub vlans_desc: [fip_vlan_desc; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_vn_port_ka {
    pub eth: ethhdr,
    pub fip: fip_header,
    pub mac_desc: fip_mac_desc,
    pub vn_port_desc: fip_vn_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_enode_ka {
    pub eth: ethhdr,
    pub fip: fip_header,
    pub mac_desc: fip_mac_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_cvl {
    pub fip: fip_header,
    pub fcf_mac_desc: fip_mac_desc,
    pub name_desc: fip_wwn_desc,
    pub vn_ports_desc: [fip_vn_desc; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_flogi_desc {
    pub fd_desc: fip_desc,
    pub rsvd: u16,
    pub flogi: fc_std_flogi,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_flogi_rsp_desc {
    pub fd_desc: fip_desc,
    pub rsvd: u16,
    pub flogi: fc_std_flogi,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_flogi {
    pub eth: ethhdr,
    pub fip: fip_header,
    pub flogi_desc: fip_flogi_desc,
    pub mac_desc: fip_mac_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_flogi_rsp {
    pub fip: fip_header,
    pub rsp_desc: fip_flogi_rsp_desc,
    pub mac_desc: fip_mac_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_discovery {
    pub eth: ethhdr,
    pub fip: fip_header,
    pub mac_desc: fip_mac_desc,
    pub name_desc: fip_wwn_desc,
    pub fcoe_desc: fip_size_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_disc_adv {
    pub fip: fip_header,
    pub prio_desc: fip_pri_desc,
    pub mac_desc: fip_mac_desc,
    pub name_desc: fip_wwn_desc,
    pub fabric_desc: fip_fab_desc,
    pub fka_adv_desc: fip_fka_desc,
    pub __packed: },
    pub fiph): *mut *mut void fnic_fcoe_process_vlan_resp(struct fnic fnic, struct fip_header,
    pub fiph): *mut *mut void fnic_fcoe_fip_discovery_resp(struct fnic fnic, struct fip_header,
    pub fiph): *mut *mut void fnic_fcoe_process_flogi_resp(struct fnic fnic, struct fip_header,
    pub work): *mut void fnic_work_on_fip_timer(struct work_struct,
    pub work): *mut void fnic_work_on_fcs_ka_timer(struct work_struct,
    pub fnic): *mut void fnic_fcoe_send_vlan_req(struct fnic,
    pub fnic): *mut void fnic_fcoe_start_fcf_discovery(struct fnic,
    pub fnic): *mut void fnic_fcoe_start_flogi(struct fnic,
    pub fiph): *mut *mut void fnic_fcoe_process_cvl(struct fnic fnic, struct fip_header,
    pub fnic): *mut void fnic_vlan_discovery_timeout(struct fnic,
    pub fnic_fip_queue: *mut extern struct workqueue_struct,

    pub 1): *mut *mut *mut fip_header fiph = (fip_header )(eth +,
    pub be16_to_cpu(fiph->fip_op): u16 op =,
    pub fiph->fip_subcode: u8 sub =,
    pub len): pfx, op, sub,,
    pub len): *mut *mut fnic_debug_dump(fnic, (uint8_t )eth,,

