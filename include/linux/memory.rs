//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/memory.h
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
// include/linux/memory.h - generic memory definition
//
// This is mainly for topological representation. We define the
// basic "struct memory_block" here, which can be embedded in per-arch
// definitions or NUMA information.
//
// Basic handling of the devices is done in drivers/base/memory.c
// and system devices are handled in drivers/base/sys.c.
//
// Memory block are exported via sysfs in the class/memory/devices
// directory.
//

//
// struct memory_group - a logical group of memory blocks
// @nid: The node id for all memory blocks inside the memory group.
// @memory_blocks: List of all memory blocks belonging to this memory group.
// @present_kernel_pages: Present (online) memory outside ZONE_MOVABLE of this
// memory group.
// @present_movable_pages: Present (online) memory in ZONE_MOVABLE of this
// memory group.
// @is_dynamic: The memory group type: static vs. dynamic
// @s.max_pages: Valid with &memory_group.is_dynamic == false. The maximum
// number of pages we'll have in this static memory group.
// @d.unit_pages: Valid with &memory_group.is_dynamic == true. Unit in pages
// in which memory is added/removed in this dynamic memory group.
// This granularity defines the alignment of a unit in physical
// address space; it has to be at least as big as a single
// memory block.
//
// A memory group logically groups memory blocks; each memory block
// belongs to at most one memory group. A memory group corresponds to
// a memory device, such as a DIMM or a NUMA node, which spans multiple
// memory blocks and might even span multiple non-contiguous physical memory
// ranges.
//
// Modification of members after registration is serialized by memory
// hot(un)plug code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_group {
    pub nid: c_int,
    pub memory_blocks: list_head,
    pub present_kernel_pages: c_ulong,
    pub present_movable_pages: c_ulong,
    pub is_dynamic: bool,
    pub max_pages: c_ulong,
    pub s: },
    pub unit_pages: c_ulong,
    pub d: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum memory_block_state {
// These states are exposed to userspace as text strings in sysfs
    MEM_ONLINE,		/* exposed to userspace */
    MEM_GOING_OFFLINE,	/* exposed to userspace */
    MEM_OFFLINE,		/* exposed to userspace */
    MEM_GOING_ONLINE,
    MEM_CANCEL_ONLINE,
    MEM_CANCEL_OFFLINE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_block {
    pub start_section_nr: c_ulong,
    pub /: *mut *mut memory_block_state state; / serialized by the dev->lock,
    pub /: *mut *mut mmop online_type; / for passing data to online routine,
    pub /: *mut *mut int nid; / NID for this memory block,
//
// The single zone of this memory block if all PFNs of this memory block
// that are System RAM (not a memory hole, not ZONE_DEVICE ranges) are
// managed by a single zone. NULL if multiple zones (including nodes)
// apply.
//
    pub zone: *mut zone,
    pub dev: device,
    pub altmap: *mut vmem_altmap,
    pub /: *mut *mut *mut memory_group group; / group (if any) for this block,
    pub /: *mut *mut list_head group_next; / next block inside memory group,

    pub nr_hwpoison: atomic_long_t,

}

extern "C" {
    pub fn arch_get_memory_phys_device(start_pfn: c_ulong) -> c_int;
}
extern "C" {
    pub fn memory_block_size_bytes() -> c_ulong;
}
extern "C" {
    pub fn set_memory_block_size_order(order: c_uint) -> c_int;
}
//
// memory_block_aligned_range - align a physical address range to memory blocks
// @range: the input range to align
//
// Aligns the start address up and the end address down to memory block
// boundaries. This is required for memory hotplug operations which must
// operate on memory-block aligned ranges.
//
// Returns the aligned range. Callers should check that the returned
// range is valid (aligned.start < aligned.end) before using it.
//
// No whole block fits (e.g. range below the first boundary): empty.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_notify {
    pub start_pfn: c_ulong,
    pub nr_pages: c_ulong,
}

//
// Priorities for the hotplug memory callback routines. Invoked from
// high to low. Higher priorities correspond to higher numbers.
//
pub const DEFAULT_CALLBACK_PRI: c_int = 0;
pub const SLAB_CALLBACK_PRI: c_int = 1;
pub const CXL_CALLBACK_PRI: c_int = 5;
pub const HMAT_CALLBACK_PRI: c_int = 6;
pub const MM_COMPUTE_BATCH_PRI: c_int = 10;
pub const CPUSET_CALLBACK_PRI: c_int = 10;
pub const MEMTIER_HOTPLUG_PRI: c_int = 100;
pub const KSM_CALLBACK_PRI: c_int = 100;

extern "C" {
    pub fn register_memory_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_memory_notifier(nb: *mut notifier_block);
}
extern "C" {
    pub fn remove_memory_block_devices(start: c_ulong, size: c_ulong);
}
extern "C" {
    pub fn memory_dev_init();
}
extern "C" {
    pub fn memory_notify(state: memory_block_state, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn int(: *mut *mut walk_memory_blocks_func_t)(struct memory_block, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn for_each_memory_block(arg: *mut c_void, func: walk_memory_blocks_func_t) -> c_int;
}
extern "C" {
    pub fn memory_group_register_static(nid: c_int, max_pages: c_ulong) -> c_int;
}
extern "C" {
    pub fn memory_group_register_dynamic(nid: c_int, unit_pages: c_ulong) -> c_int;
}
extern "C" {
    pub fn memory_group_unregister(mgid: c_int) -> c_int;
}
extern "C" {
    pub fn int(: *mut *mut walk_memory_groups_func_t)(struct memory_group, : *mut c_void) -> typedef;
}

extern "C" {
    pub fn memory_block_id(_arg: pfn_to_section_nr(pfn)) -> return;
}
extern "C" {
    pub fn pfn_to_block_id(_arg: PFN_DOWN(phys)) -> return;
}

extern "C" {
    pub fn memory_block_add_nid_early(mem: *mut memory_block, nid: c_int);
}

extern "C" {
    pub fn memory_block_advise_max_size(size: c_ulong) -> c_int;
}
extern "C" {
    pub fn memory_block_advised_max_size() -> c_ulong;
}

//
// Kernel text modification mutex, used for code patching. Users of this lock
// can sleep.
//
