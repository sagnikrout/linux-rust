//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/null_blk/null_blk.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nullb_cmd {
    pub error: blk_status_t,
    pub fake_timeout: bool,
    pub nq: *mut nullb_queue,
    pub timer: hrtimer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nullb_queue {
    pub dev: *mut nullb_device,
    pub requeue_selection: c_uint,
    pub poll_list: list_head,
    pub poll_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nullb_zone {
//
// Zone lock to prevent concurrent modification of a zone write
// pointer position and condition: with memory backing, a write
// command execution may sleep on memory allocation. For this case,
// use mutex as the zone lock. Otherwise, use the spinlock for
// locking the zone.
//
    pub spinlock: spinlock_t,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nullb_device {
    pub nullb: *mut nullb,
    pub group: config_group,

    pub timeout_config: fault_config,
    pub requeue_config: fault_config,
    pub init_hctx_fault_config: fault_config,

    pub /: *mut *mut radix_tree_root data; / data stored in the disk,
    pub /: *mut *mut radix_tree_root cache; / disk cache data,
    pub /: *mut *mut unsigned long flags; / device flags,
    pub curr_cache: c_uint,
    pub badblocks: badblocks,
    pub badblocks_once: bool,
    pub badblocks_partial_io: bool,
    pub nr_zones: c_uint,
    pub nr_zones_imp_open: c_uint,
    pub nr_zones_exp_open: c_uint,
    pub nr_zones_closed: c_uint,
    pub imp_close_zone_no: c_uint,
    pub zones: *mut nullb_zone,
    pub zone_size_sects: sector_t,
    pub need_zone_res_mgmt: bool,
    pub zone_res_lock: spinlock_t,
    pub /: *mut *mut unsigned long size; / device size in MB,
    pub /: *mut *mut unsigned long completion_nsec; / time in ns to complete a request,
    pub /: *mut *mut unsigned long cache_size; / disk cache size in MB,
    pub /: *mut *mut unsigned long zone_size; / zone size in MB if device is zoned,
    pub /: *mut *mut unsigned long zone_capacity; / zone capacity in MB if device is zoned,
    pub /: *mut *mut unsigned int zone_nr_conv; / number of conventional zones,
    pub /: *mut *mut unsigned int zone_max_open; / max number of open zones,
    pub /: *mut *mut unsigned int zone_max_active; / max number of active zones,
    pub /: *mut *mut unsigned int zone_append_max_sectors; / Max sectors per zone append command,
    pub /: *mut *mut unsigned int submit_queues; / number of submission queues,
    pub /: *mut *mut unsigned int prev_submit_queues; / number of submission queues before change,
    pub /: *mut *mut unsigned int poll_queues; / number of IOPOLL submission queues,
    pub /: *mut *mut unsigned int prev_poll_queues; / number of IOPOLL submission queues before change,
    pub /: *mut *mut unsigned int home_node; / home node for the device,
    pub /: *mut *mut unsigned int queue_mode; / block interface,
    pub /: *mut *mut unsigned int blocksize; / block size,
    pub /: *mut *mut unsigned int max_sectors; / Max sectors per command,
    pub /: *mut *mut unsigned int irqmode; / IRQ completion handler,
    pub /: *mut *mut unsigned int hw_queue_depth; / queue depth,
    pub /: *mut *mut unsigned int index; / index of the disk, only valid with a disk,
    pub /: *mut *mut unsigned int mbps; / Bandwidth throttle cap (in MB/s),
    pub /: *mut *mut bool blocking; / blocking blk-mq device,
    pub /: *mut *mut bool use_per_node_hctx; / use per-node allocation for hardware context,
    pub /: *mut *mut bool power; / power on/off the device,
    pub /: *mut *mut bool memory_backed; / if data is stored in memory,
    pub /: *mut *mut bool discard; / if support discard,
    pub /: *mut *mut bool zoned; / if device is zoned,
    pub /: *mut *mut bool zone_full; / Initialize zones to be full,
    pub /: *mut *mut bool virt_boundary; / virtual boundary on/off for the device,
    pub /: *mut *mut bool no_sched; / no IO scheduler for the device,
    pub /: *mut *mut bool shared_tags; / share tag set between devices for blk-mq,
    pub /: *mut *mut bool shared_tag_bitmap; / use hostwide shared tags,
    pub /: *mut *mut bool fua; / Support FUA,
    pub /: *mut *mut bool rotational; / Fake rotational device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nullb {
    pub dev: *mut nullb_device,
    pub list: list_head,
    pub index: c_uint,
    pub q: *mut request_queue,
    pub disk: *mut gendisk,
    pub tag_set: *mut blk_mq_tag_set,
    pub __tag_set: blk_mq_tag_set,
    pub cur_bytes: atomic_long_t,
    pub bw_timer: hrtimer,
    pub cache_flush_pos: c_ulong,
    pub lock: spinlock_t,
    pub queues: *mut nullb_queue,
    pub disk_name: [c_char; DISK_NAME_LEN],
}

extern "C" {
    pub fn null_init_zoned_dev(dev: *mut nullb_device, lim: *mut queue_limits) -> c_int;
}
extern "C" {
    pub fn null_register_zoned_dev(nullb: *mut nullb) -> c_int;
}
extern "C" {
    pub fn null_free_zoned_dev(dev: *mut nullb_device);
}

