//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/rdma_vt.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2016 - 2019 Intel Corporation.
//
// Structure that low level drivers will populate in order to register with the
// rdmavt layer.
//

pub const RVT_MAX_PKEY_VALUES: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trap_list {
    pub list_len: u32,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_ibport {
    pub qp: [*mut rvt_qp __rcu; 2],
    pub /: *mut *mut *mut ib_mad_agent send_agent; / agent for SMI (traps),
    pub mcast_tree: rb_root,
    pub /: *mut *mut spinlock_t lock; / protect changes in this struct,
// non-zero when timer is set
    pub mkey_lease_timeout: c_ulong,
    pub trap_timeout: c_ulong,
    pub /: *mut *mut __be64 gid_prefix; / in network order,
    pub mkey: __be64,
    pub tid: u64,
    pub port_cap_flags: u32,
    pub port_cap3_flags: u16,
    pub pma_sample_start: u32,
    pub pma_sample_interval: u32,
    pub pma_counter_select: [__be16; 5],
    pub pma_tag: u16,
    pub mkey_lease_period: u16,
    pub sm_lid: u32,
    pub sm_sl: u8,
    pub mkeyprot: u8,
    pub subnet_timeout: u8,
    pub vl_high_limit: u8,
//
// Driver is expected to keep these up to date. These
// counters are informational only and not required to be
// completely accurate.
//
    pub n_rc_resends: u64,
    pub n_seq_naks: u64,
    pub n_rdma_seq: u64,
    pub n_rnr_naks: u64,
    pub n_other_naks: u64,
    pub n_loop_pkts: u64,
    pub n_pkt_drops: u64,
    pub n_vl15_dropped: u64,
    pub n_rc_timeouts: u64,
    pub n_dmawait: u64,
    pub n_unaligned: u64,
    pub n_rc_dupreq: u64,
    pub n_rc_seqnak: u64,
    pub n_rc_crwaits: u64,
    pub pkey_violations: u16,
    pub qkey_violations: u16,
    pub mkey_violations: u16,
// Hot-path per CPU counters to avoid cacheline trading to update
    pub z_rc_acks: u64,
    pub z_rc_qacks: u64,
    pub z_rc_delayed_comp: u64,
    pub rc_acks: *mut u64 __percpu,
    pub rc_qacks: *mut u64 __percpu,
    pub rc_delayed_comp: *mut u64 __percpu,
    pub /: *mut *mut *mut void priv; / driver private data,
//
// The pkey table is allocated and maintained by the driver. Drivers
// need to have access to this before registering with rdmav. However
// rdmavt will need access to it so drivers need to provide this during
// the attach port API call.
//
    pub pkey_table: *mut u16,
    pub sm_ah: *mut rvt_ah,
//
// Keep a list of traps that have not been repressed.  They will be
// resent based on trap_timer.
//
    pub trap_lists: [trap_list; RVT_MAX_TRAP_LISTS],
    pub trap_timer: timer_list,
}

pub const RVT_SGE_COPY_MEMCPY: c_int = 0;
pub const RVT_SGE_COPY_CACHELESS: c_int = 1;
pub const RVT_SGE_COPY_ADAPTIVE: c_int = 2;
//
// Things that are driver specific, module parameters in hfi1 and qib
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_driver_params {
    pub props: ib_device_attr,
//
// Anything driver specific that is not covered by props
// For instance special module parameters. Goes here.
//
    pub lkey_table_size: c_uint,
    pub qp_table_size: c_uint,
    pub sge_copy_mode: c_uint,
    pub wss_threshold: c_uint,
    pub wss_clean_period: c_uint,
    pub qpn_start: c_int,
    pub qpn_inc: c_int,
    pub qpn_res_start: c_int,
    pub qpn_res_end: c_int,
    pub nports: c_int,
    pub npkeys: c_int,
    pub node: c_int,
    pub psn_mask: c_int,
    pub psn_shift: c_int,
    pub psn_modify_mask: c_int,
    pub core_cap_flags: u32,
    pub max_mad_size: u32,
    pub qos_shift: u8,
    pub max_rdma_atomic: u8,
    pub extra_rdma_atomic: u8,
    pub reserved_operations: u8,
}

// User context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_ucontext {
    pub ibucontext: ib_ucontext,
    pub priv: *mut c_void,
}

// Protection domain
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_pd {
    pub ibpd: ib_pd,
    pub user: bool,
}

// Address handle
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_ah {
    pub ibah: ib_ah,
    pub attr: rdma_ah_attr,
    pub vl: u8,
    pub log_pmtu: u8,
}

//
// This structure is used by rvt_mmap() to validate an offset
// when an mmap() request is made.  The vm_area_struct then uses
// this as its vm_private_data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_mmap_info {
    pub pending_mmaps: list_head,
    pub context: *mut ib_ucontext,
    pub obj: *mut c_void,
    pub offset: __u64,
    pub ref: kref,
    pub size: u32,
}

// memory working set size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_wss {
    pub entries: *mut c_ulong,
    pub total_count: core::sync::atomic::AtomicI32,
    pub clean_counter: core::sync::atomic::AtomicI32,
    pub clean_entry: core::sync::atomic::AtomicI32,
    pub threshold: c_int,
    pub num_entries: c_int,
    pub pages_mask: c_long,
    pub clean_period: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_driver_provided {
//
// Which functions are required depends on which verbs rdmavt is
// providing and which verbs the driver is overriding. See
// check_support() for details.
//
// hot path calldowns in a single cacheline
//
// Give the driver a notice that there is send work to do. It is up to
// the driver to generally push the packets out, this just queues the
// work with the driver. There are two variants here. The no_lock
// version requires the s_lock not to be held. The other assumes the
// s_lock is held.
//
    pub qp): *mut *mut bool (schedule_send)(struct rvt_qp,
    pub qp): *mut *mut bool (schedule_send_no_lock)(struct rvt_qp,
//
// Driver specific work request setup and checking.
// This function is allowed to perform any setup, checks, or
// adjustments required to the SWQE in order to be usable by
// underlying protocols. This includes private data structure
// allocations.
//
    pub call_send): *mut bool,
//
// Sometimes rdmavt needs to kick the driver's send progress. That is
// done by this call back.
//
    pub qp): *mut *mut void (do_send)(struct rvt_qp,
//
// Returns a pointer to the underlying hardware's PCI device. This is
// used to display information as to what hardware is being referenced
// in an output message
//
    pub rdi): *mut *mut *mut pci_dev  (get_pci_dev)(rvt_dev_info,
//
// Allocate a private queue pair data structure for driver specific
// information which is opaque to rdmavt.  Errors are returned via
// ERR_PTR(err).  The driver is free to return NULL or a valid
// pointer.
//
    pub qp): *mut *mut *mut *mut void  (qp_priv_alloc)(struct rvt_dev_info rdi, struct rvt_qp,
//
// Init a structure allocated with qp_priv_alloc(). This should be
// called after all qp fields have been initialized in rdmavt.
//
    pub init_attr): *mut ib_qp_init_attr,
//
// Free the driver's private qp structure.
//
    pub qp): *mut *mut *mut void (qp_priv_free)(struct rvt_dev_info rdi, struct rvt_qp,
//
// Inform the driver the particular qp in question has been reset so
// that it can clean up anything it needs to.
//
    pub qp): *mut *mut void (notify_qp_reset)(struct rvt_qp,
//
// Get a path mtu from the driver based on qp attributes.
//
    pub attr): *mut ib_qp_attr,
//
// Notify driver that it needs to flush any outstanding IO requests that
// are waiting on a qp.
//
    pub qp): *mut *mut void (flush_qp_waiters)(struct rvt_qp,
//
// Notify driver to stop its queue of sending packets. Nothing else
// should be posted to the queue pair after this has been called.
//
    pub qp): *mut *mut void (stop_send_queue)(struct rvt_qp,
//
// Have the driver drain any in progress operations
//
    pub qp): *mut *mut void (quiesce_qp)(struct rvt_qp,
//
// Inform the driver a qp has went to error state.
//
    pub qp): *mut *mut void (notify_error_qp)(struct rvt_qp,
//
// Get an MTU for a qp.
//
    pub pmtu): u32,
//
// Convert an mtu to a path mtu
//
    pub mtu): *mut *mut int (mtu_to_path_mtu)(u32,
//
// Get the guid of a port in big endian byte order
//
    pub guid): *mut int guid_index, __be64,
//
// Query driver for the state of the port.
//
    pub props): *mut ib_port_attr,
//
// Tell driver to shutdown a port
//
    pub port_num): *mut *mut *mut int (shut_down_port)(struct rvt_dev_info rdi, u32,
// Tell driver to send a trap for changed  port capabilities
    pub port_num): *mut *mut *mut void (cap_mask_chg)(struct rvt_dev_info rdi, u32,
//
// The following functions can be safely ignored completely. Any use of
// these is checked for NULL before blindly calling. Rdmavt should also
// be functional if drivers omit these.
//
// Called to inform the driver that all qps should now be freed.
    pub rdi): *mut *mut unsigned (free_all_qps)(struct rvt_dev_info,
// Driver specific AH validation
    pub ): *mut *mut *mut int (check_ah)(struct ib_device , struct rdma_ah_attr,
// Inform the driver a new AH has been created
    pub ): *mut rvt_ah,
// Let the driver pick the next queue pair number
    pub port_num): ib_qp_type type, u32,
// Determine if its safe or allowed to modify the qp
    pub udata): *mut int attr_mask, struct ib_udata,
// Driver specific QP modification/notification-of
    pub udata): *mut int attr_mask, struct ib_udata,
// Notify driver a mad agent has been created
    pub port_idx): *mut *mut *mut void (notify_create_mad_agent)(struct rvt_dev_info rdi, int,
// Notify driver a mad agent has been removed
    pub port_idx): *mut *mut *mut void (notify_free_mad_agent)(struct rvt_dev_info rdi, int,
// Notify driver to restart rc
    pub wait): *mut *mut *mut void (notify_restart_rc)(struct rvt_qp qp, u32 psn, int,
// Get and return CPU to pin CQ processing thread
    pub comp_vect): *mut *mut *mut int (comp_vect_cpu_lookup)(struct rvt_dev_info rdi, int,
// allocate a ucontext
    pub udata): *mut *mut *mut int (alloc_ucontext)(struct ib_ucontext uctx, struct ib_udata,
// deallocate a ucontext
    pub context): *mut *mut void (dealloc_ucontext)(struct ib_ucontext,
// driver mmap
    pub vma): *mut *mut *mut int (mmap)(struct ib_ucontext context, struct vm_area_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvt_dev_info {
    pub /: *mut *mut ib_device ibdev; / Keep this first. Nothing above here,
//
// Prior to calling for registration the driver will be responsible for
// allocating space for this structure.
//
// The driver will also be responsible for filling in certain members of
// dparms.props. The driver needs to fill in dparms exactly as it would
// want values reported to a ULP. This will be returned to the caller
// in rdmavt's device. The driver should also therefore refrain from
// modifying this directly after registration with rdmavt.
//
// Driver specific properties
    pub dparms: rvt_driver_params,
// post send table
    pub post_parms: *const rvt_operation_params,
// opcode translation table
    pub wc_opcode: *const ib_wc_opcode,
// Driver specific helper functions
    pub driver_f: rvt_driver_provided,
    pub dma_mr: *mut rvt_mregion __rcu,
    pub lkey_table: rvt_lkey_table,
// Internal use
    pub n_pds_allocated: c_int,
    pub /: *mut *mut spinlock_t n_pds_lock; / Protect pd allocated count,
    pub n_ahs_allocated: c_int,
    pub /: *mut *mut spinlock_t n_ahs_lock; / Protect ah allocated count,
    pub n_srqs_allocated: u32,
    pub /: *mut *mut spinlock_t n_srqs_lock; / Protect srqs allocated count,
    pub flags: c_int,
    pub ports: *mut rvt_ibport,
// QP
    pub qp_dev: *mut rvt_qp_ibdev,
    pub /: *mut *mut u32 n_qps_allocated; / number of QPs allocated for device,
    pub /: *mut *mut u32 n_rc_qps; / number of RC QPs allocated for device,
    pub /: *mut *mut u32 busy_jiffies; / timeout scaling based on RC QP count,
    pub /: *mut *mut spinlock_t n_qps_lock; / protect qps, rc qps and busy jiffy counts,
// memory maps
    pub pending_mmaps: list_head,
    pub /: *mut *mut spinlock_t mmap_offset_lock; / protect mmap_offset,
    pub mmap_offset: u32,
    pub /: *mut *mut spinlock_t pending_lock; / protect pending mmap list,
// CQ
    pub /: *mut *mut u32 n_cqs_allocated; / number of CQs allocated for device,
    pub /: *mut *mut spinlock_t n_cqs_lock; / protect count of in use cqs,
// Multicast
    pub /: *mut *mut u32 n_mcast_grps_allocated; / number of mcast groups allocated,
    pub n_mcast_grps_lock: spinlock_t,
// Memory Working Set Size
    pub wss: *mut rvt_wss,
}

//
// rvt_get_ibdev_name - return the IB name
// @rdi: rdmavt device
//
// Return the registered name of the device.
//
extern "C" {
    pub fn dev_name(_arg: &rdi->ibdev.dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibpd, rvt_pd: struct, _arg: ibpd) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibah, rvt_ah: struct, _arg: ibah) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibdev, rvt_dev_info: struct, _arg: ibdev) -> return;
}
//
// All ports have same number of pkeys.
//
// Return the max atomic suitable for determining
// the size of the ack ring buffer in a QP.
//
// Return the indexed PKEY from the port PKEY table.
//
extern "C" {
    pub fn rvt_dealloc_device(rdi: *mut rvt_dev_info);
}
extern "C" {
    pub fn rvt_register_device(rvd: *mut rvt_dev_info) -> c_int;
}
extern "C" {
    pub fn rvt_unregister_device(rvd: *mut rvt_dev_info);
}
extern "C" {
    pub fn rvt_check_ah(ibdev: *mut ib_device, ah_attr: *mut rdma_ah_attr) -> c_int;
}
extern "C" {
    pub fn rvt_invalidate_rkey(qp: *mut rvt_qp, rkey: u32) -> c_int;
}
