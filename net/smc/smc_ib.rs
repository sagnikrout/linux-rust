//! Automatically rewritten from C Header to Rust Module
//! Source: net/smc/smc_ib.h
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
//
// Shared Memory Communications over RDMA (SMC-R) and RoCE
//
// Definitions for IB environment
//
// Copyright IBM Corp. 2016
//
// Author(s):  Ursula Braun <Ursula Braun@linux.vnet.ibm.com>
//

pub const SMC_IB_MAX_SEND_SGE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_ib_devices {
    pub list: list_head,
    pub /: *mut *mut mutex mutex; / protects list of smc ib devices,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_ib_device {
    pub list: list_head,
    pub ibdev: *mut ib_device,
    pub /: *mut *mut ib_port_attr pattr[SMC_MAX_PORTS]; / ib dev. port attrs,
    pub /: *mut *mut ib_event_handler event_handler; / global ib_event handler,
    pub /: *mut *mut *mut ib_cq roce_cq_send; / send completion queue,
    pub /: *mut *mut *mut ib_cq roce_cq_recv; / recv completion queue,
    pub /: *mut *mut tasklet_send_tasklet; / called by send cq handler,
    pub /: *mut *mut tasklet_recv_tasklet; / called by recv cq handler,
    pub mac: [c_char; SMC_MAX_PORTS][ETH_ALEN],
// mac address per port
    pub pnetid: [u8; SMC_MAX_PORTS][SMC_MAX_PNETID_LEN],
// pnetid per port
    pub pnetid_by_user: [bool; SMC_MAX_PORTS],
// pnetid defined by user?
    pub /: *mut *mut u8 initialized : 1; / ib dev CQ, evthdl done,
    pub port_event_work: work_struct,
    pub port_event_mask: c_ulong,
    pub SMC_MAX_PORTS): DECLARE_BITMAP(ports_going_away,,
    pub /: *mut *mut atomic_t lnk_cnt; / number of links on ibdev,
    pub links*/: *mut *mut wait_queue_head_t lnks_deleted; / wait 4 removal of all,
    pub /: *mut *mut mutex mutex; / protect dev setup+cleanup,
    pub lnk_cnt_by_port: [core::sync::atomic::AtomicI32; SMC_MAX_PORTS],
// number of links per port
    pub /: *mut *mut int ndev_ifidx[SMC_MAX_PORTS]; / ndev if indexes,
}

extern "C" {
    pub fn cpu_to_be32(_arg: INADDR_NONE) -> return;
}
extern "C" {
    pub fn read_pnet(_arg: &smcibdev->ibdev->coredev.rdma_net) -> return;
}
extern "C" {
    pub fn smc_ib_ndev_change(ndev: *mut net_device, event: c_ulong);
}
extern "C" {
    pub fn smc_ib_unregister_client();
}
extern "C" {
    pub fn smc_ib_port_active(smcibdev: *mut smc_ib_device, ibport: u8) -> bool;
}
extern "C" {
    pub fn smc_ib_dealloc_protection_domain(lnk: *mut smc_link);
}
extern "C" {
    pub fn smc_ib_create_protection_domain(lnk: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smc_ib_destroy_queue_pair(lnk: *mut smc_link);
}
extern "C" {
    pub fn smc_ib_create_queue_pair(lnk: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smc_ib_ready_link(lnk: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smc_ib_modify_qp_rts(lnk: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smc_ib_modify_qp_error(lnk: *mut smc_link) -> c_int;
}
extern "C" {
    pub fn smc_ib_setup_per_ibdev(smcibdev: *mut smc_ib_device) -> c_long;
}
extern "C" {
    pub fn smc_ib_put_memory_region(mr: *mut ib_mr);
}
extern "C" {
    pub fn smc_ib_is_valid_local_systemid() -> bool;
}
extern "C" {
    pub fn smcr_nl_get_device(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
