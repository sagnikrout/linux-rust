//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hv/mshv_root.h
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
// Copyright (c) 2023, Microsoft Corporation.
//

//
// Hypervisor must be between these version numbers (inclusive)
// to guarantee compatibility
//

pub const MSHV_MAX_VPS: c_int = 256;
pub const MSHV_PARTITIONS_HASH_BITS: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_vp {
    pub vp_index: u32,
    pub vp_partition: *mut mshv_partition,
    pub vp_mutex: mutex,
    pub vp_register_page: *mut hv_vp_register_page,
    pub vp_intercept_msg_page: *mut hv_message,
    pub vp_ghcb_page: *mut c_void,
    pub vp_stats_pages: [*mut hv_stats_page; 2],
    pub vp_signaled_count: core::sync::atomic::AtomicI64,
    pub 1: u64 intercept_suspend:,
    pub /: *mut *mut u64 root_sched_blocked: 1; / root scheduler only,
    pub /: *mut *mut u64 root_sched_dispatched: 1; / root scheduler only,
    pub 61: u64 reserved:,
    pub flags: },
    pub kicked_by_hv: c_uint,
    pub vp_suspend_queue: wait_queue_head_t,
    pub run: },

    pub vp_stats_dentry: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mshv_region_type {
    MSHV_REGION_TYPE_MEM_PINNED,
    MSHV_REGION_TYPE_MEM_MOVABLE,
    MSHV_REGION_TYPE_MMIO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_mem_region {
    pub hnode: hlist_node,
    pub mreg_refcount: kref,
    pub nr_pages: u64,
    pub start_gfn: u64,
    pub start_uaddr: u64,
    pub hv_map_flags: u32,
    pub partition: *mut mshv_partition,
    pub mreg_type: mshv_region_type,
    pub mreg_mni: mmu_interval_notifier,
    pub /: *mut *mut mutex mreg_mutex; / protects region pages remapping,
    pub mreg_pages: [*mut page; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_irq_ack_notifier {
    pub link: hlist_node,
    pub irq_ack_gsi: c_uint,
    pub mian): *mut *mut void (irq_acked)(struct mshv_irq_ack_notifier,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_partition {
    pub pt_module_dev: *mut device,
    pub pt_hnode: hlist_node,
    pub pt_id: u64,
    pub pt_ref_count: refcount_t,
    pub pt_mutex: mutex,
    pub pt_mem_regions_lock: spinlock_t,
    pub ordered: hlist_head pt_mem_regions; // not,
    pub pt_vp_count: u32,
    pub pt_vp_array: [*mut mshv_vp; MSHV_MAX_VPS],
    pub pt_irq_lock: mutex,
    pub pt_irq_srcu: srcu_struct,
    pub irq_ack_notifier_list: hlist_head,
    pub pt_devices: hlist_head,
//
// MSHV does not support more than one async hypercall in flight
// for a single partition. Thus, it is okay to define per partition
// async hypercall status.
//
    pub async_hypercall: completion,
    pub async_hypercall_status: u64,
    pub pt_irqfds_lock: spinlock_t,
    pub pt_irqfds_list: hlist_head,
    pub irqfds_resampler_lock: mutex,
    pub irqfds_resampler_list: hlist_head,
    pub ioeventfds_list: hlist_head,
    pub pt_girq_tbl: *mut mshv_girq_routing_table __rcu,
    pub isolation_type: u64,
    pub import_completed: bool,
    pub pt_initialized: bool,

    pub pt_stats_dentry: *mut dentry,
    pub pt_vp_dentry: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_lapic_irq {
    pub lapic_vector: u32,
    pub lapic_apic_id: u64,
    pub lapic_control: hv_interrupt_control,
}

pub const MSHV_MAX_GUEST_IRQS: c_int = 4096;
// representation of one guest irq entry, either msi or legacy
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_guest_irq_ent {
    pub /: *mut *mut u32 girq_entry_valid; / vfio looks at this,
    pub /: *mut *mut u32 guest_irq_num; / a unique number for each irq,
    pub /: *mut *mut u32 girq_addr_lo; / guest irq msi address info,
    pub girq_addr_hi: u32,
    pub /: *mut *mut u32 girq_irq_data; / idt vector in some cases,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_girq_routing_table {
    pub num_rt_entries: u32,
    pub mshv_girq_info_tbl: [mshv_guest_irq_ent; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_synic_pages {
    pub hyp_synic_message_page: *mut hv_message_page,
    pub synic_event_flags_page: *mut hv_synic_event_flags_page,
    pub synic_event_ring_page: *mut hv_synic_event_ring_page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_root {
    pub pt_ht_lock: spinlock_t,
    pub MSHV_PARTITIONS_HASH_BITS): DECLARE_HASHTABLE(pt_htable,,
    pub vmm_caps: hv_partition_property_vmm_capabilities,
}

//
// Callback for doorbell events.
// NOTE: This is called in interrupt context. Callback
// should defer slow and sleeping logic to later.
//
extern "C" {
    pub fn void(doorbell_id: *mut *mut doorbell_cb_t) (int, : *mut c_void) -> typedef;
}
//
// port table information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_table_info {
    pub portbl_rcu: rcu_head,
    pub hv_port_type: hv_port_type,
    pub reserved: [u64; 2],
    pub hv_port_message: },
    pub reserved: [u64; 2],
    pub hv_port_event: },
    pub reserved: [u64; 2],
    pub hv_port_monitor: },
    pub doorbell_cb: doorbell_cb_t,
    pub data: *mut c_void,
    pub hv_port_doorbell: },
}

extern "C" {
    pub fn mshv_free_routing_table(partition: *mut mshv_partition);
}
extern "C" {
    pub fn mshv_irqfd_routing_update(partition: *mut mshv_partition);
}
extern "C" {
    pub fn mshv_port_table_fini();
}
extern "C" {
    pub fn mshv_portid_alloc(info: *mut port_table_info) -> c_int;
}
extern "C" {
    pub fn mshv_portid_lookup(port_id: c_int, info: *mut port_table_info) -> c_int;
}
extern "C" {
    pub fn mshv_portid_free(port_id: c_int);
}
extern "C" {
    pub fn mshv_unregister_doorbell(partition_id: u64, doorbell_portid: c_int);
}
extern "C" {
    pub fn mshv_isr();
}
extern "C" {
    pub fn mshv_synic_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn mshv_synic_exit();
}
extern "C" {
    pub fn mshv_partition_put(partition: *mut mshv_partition);
}
extern "C" {
    pub fn hv_l1vh_partition(HV_PARTITION_ID_SELF: ) && (partition_id ==) -> return;
}
// hypercalls
extern "C" {
    pub fn hv_call_withdraw_memory(count: u64, node: c_int, partition_id: u64) -> c_int;
}
extern "C" {
    pub fn hv_call_initialize_partition(partition_id: u64) -> c_int;
}
extern "C" {
    pub fn hv_call_finalize_partition(partition_id: u64) -> c_int;
}
extern "C" {
    pub fn hv_call_delete_partition(partition_id: u64) -> c_int;
}
extern "C" {
    pub fn hv_call_map_mmio_pages(partition_id: u64, gfn: u64, mmio_spa: u64, numpgs: u64) -> c_int;
}
extern "C" {
    pub fn hv_call_delete_vp(partition_id: u64, vp_index: u32) -> c_int;
}
extern "C" {
    pub fn hv_call_clear_virtual_interrupt(partition_id: u64) -> c_int;
}
// Choose between pages and ret_output
// Choose between pages and bytes
extern "C" {
    pub fn hv_call_delete_port(port_partition_id: u64, port_id: hv_port_id) -> c_int;
}
extern "C" {
    pub fn hv_call_notify_port_ring_empty(sint_index: u32) -> c_int;
}

extern "C" {
    pub fn mshv_debugfs_init() -> int __init;
}
extern "C" {
    pub fn mshv_debugfs_exit();
}
extern "C" {
    pub fn mshv_debugfs_partition_create(partition: *mut mshv_partition) -> c_int;
}
extern "C" {
    pub fn mshv_debugfs_partition_remove(partition: *mut mshv_partition);
}
extern "C" {
    pub fn mshv_debugfs_vp_create(vp: *mut mshv_vp) -> c_int;
}
extern "C" {
    pub fn mshv_debugfs_vp_remove(vp: *mut mshv_vp);
}

extern "C" {
    pub fn mshv_region_share(region: *mut mshv_mem_region) -> c_int;
}
extern "C" {
    pub fn mshv_region_unshare(region: *mut mshv_mem_region) -> c_int;
}
extern "C" {
    pub fn mshv_region_map(region: *mut mshv_mem_region) -> c_int;
}
extern "C" {
    pub fn mshv_region_invalidate(region: *mut mshv_mem_region);
}
extern "C" {
    pub fn mshv_region_pin(region: *mut mshv_mem_region) -> c_int;
}
extern "C" {
    pub fn mshv_region_put(region: *mut mshv_mem_region);
}
extern "C" {
    pub fn mshv_region_get(region: *mut mshv_mem_region) -> c_int;
}
extern "C" {
    pub fn mshv_region_handle_gfn_fault(region: *mut mshv_mem_region, gfn: u64) -> bool;
}
extern "C" {
    pub fn mshv_region_movable_fini(region: *mut mshv_mem_region);
}
extern "C" {
    pub fn mshv_region_movable_init(region: *mut mshv_mem_region) -> bool;
}
