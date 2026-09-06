//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ceph/metric.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ceph_metric_type {
    CLIENT_METRIC_TYPE_CAP_INFO,
    CLIENT_METRIC_TYPE_READ_LATENCY,
    CLIENT_METRIC_TYPE_WRITE_LATENCY,
    CLIENT_METRIC_TYPE_METADATA_LATENCY,
    CLIENT_METRIC_TYPE_DENTRY_LEASE,
    CLIENT_METRIC_TYPE_OPENED_FILES,
    CLIENT_METRIC_TYPE_PINNED_ICAPS,
    CLIENT_METRIC_TYPE_OPENED_INODES,
    CLIENT_METRIC_TYPE_READ_IO_SIZES,
    CLIENT_METRIC_TYPE_WRITE_IO_SIZES,
    CLIENT_METRIC_TYPE_AVG_READ_LATENCY,
    CLIENT_METRIC_TYPE_STDEV_READ_LATENCY,
    CLIENT_METRIC_TYPE_AVG_WRITE_LATENCY,
    CLIENT_METRIC_TYPE_STDEV_WRITE_LATENCY,
    CLIENT_METRIC_TYPE_AVG_METADATA_LATENCY,
    CLIENT_METRIC_TYPE_STDEV_METADATA_LATENCY,
    CLIENT_METRIC_TYPE_SUBVOLUME_METRICS,

    CLIENT_METRIC_TYPE_MAX = CLIENT_METRIC_TYPE_SUBVOLUME_METRICS,
}

//
// This will always have the highest metric bit value
// as the last element of the array.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_metric_header {
    pub /: *mut *mut __le32 type; / ceph metric type,
    pub ver: __u8,
    pub compat: __u8,
    pub /: *mut *mut __le32 data_len; / length of sizeof(hit + mis + total),
    pub __packed: },
// metric caps header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_metric_cap {
    pub header: ceph_metric_header,
    pub hit: __le64,
    pub mis: __le64,
    pub total: __le64,
    pub __packed: },
// metric read latency header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_metric_read_latency {
    pub header: ceph_metric_header,
    pub lat: ceph_timespec,
    pub avg: ceph_timespec,
    pub sq_sum: __le64,
    pub count: __le64,
    pub __packed: },
// metric write latency header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_metric_write_latency {
    pub header: ceph_metric_header,
    pub lat: ceph_timespec,
    pub avg: ceph_timespec,
    pub sq_sum: __le64,
    pub count: __le64,
    pub __packed: },
// metric metadata latency header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_metric_metadata_latency {
    pub header: ceph_metric_header,
    pub lat: ceph_timespec,
    pub avg: ceph_timespec,
    pub sq_sum: __le64,
    pub count: __le64,
    pub __packed: },
// metric dentry lease header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_metric_dlease {
    pub header: ceph_metric_header,
    pub hit: __le64,
    pub mis: __le64,
    pub total: __le64,
    pub __packed: },
// metric opened files header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_opened_files {
    pub header: ceph_metric_header,
    pub opened_files: __le64,
    pub total: __le64,
    pub __packed: },
// metric pinned i_caps header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_pinned_icaps {
    pub header: ceph_metric_header,
    pub pinned_icaps: __le64,
    pub total: __le64,
    pub __packed: },
// metric opened inodes header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_opened_inodes {
    pub header: ceph_metric_header,
    pub opened_inodes: __le64,
    pub total: __le64,
    pub __packed: },
// metric read io size header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_read_io_size {
    pub header: ceph_metric_header,
    pub total_ops: __le64,
    pub total_size: __le64,
    pub __packed: },
// metric write io size header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_write_io_size {
    pub header: ceph_metric_header,
    pub total_ops: __le64,
    pub total_size: __le64,
    pub __packed: },
//
// struct ceph_subvolume_metric_entry_wire - On-wire format sent to MDS
// @subvolume_id: Subvolume identifier
// @read_ops: Read operation count (32-bit, clamped from 64-bit internal)
// @write_ops: Write operation count (32-bit, clamped from 64-bit internal)
// @read_bytes: Total bytes read
// @write_bytes: Total bytes written
// @read_latency_us: Cumulative read latency in microseconds
// @write_latency_us: Cumulative write latency in microseconds
// @time_stamp: Collection timestamp (currently unused, set to 0)
//
// Wire format must match C++ AggregatedIOMetrics struct in MDS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_subvolume_metric_entry_wire {
    pub subvolume_id: __le64,
    pub read_ops: __le32,
    pub write_ops: __le32,
    pub read_bytes: __le64,
    pub write_bytes: __le64,
    pub read_latency_us: __le64,
    pub write_latency_us: __le64,
    pub time_stamp: __le64,
    pub __packed: },
// Old struct kept for internal tracking, not used on wire
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_subvolume_metric_entry {
    pub subvolume_id: __le64,
    pub read_ops: __le64,
    pub write_ops: __le64,
    pub read_bytes: __le64,
    pub write_bytes: __le64,
    pub read_latency_us: __le64,
    pub write_latency_us: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_metric_head {
    pub /: *mut *mut __le32 num; / the number of metrics that will be sent,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum metric_type {
    METRIC_READ,
    METRIC_WRITE,
    METRIC_METADATA,
    METRIC_COPYFROM,
    METRIC_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_metric {
    pub lock: spinlock_t,
    pub total: u64,
    pub size_sum: u64,
    pub size_min: u64,
    pub size_max: u64,
    pub latency_sum: ktime_t,
    pub latency_avg: ktime_t,
    pub latency_sq_sum: ktime_t,
    pub latency_min: ktime_t,
    pub latency_max: ktime_t,
}

// This is the global metrics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_client_metric {
    pub total_dentries: core::sync::atomic::AtomicI64,
    pub d_lease_hit: percpu_counter,
    pub d_lease_mis: percpu_counter,
    pub total_caps: core::sync::atomic::AtomicI64,
    pub i_caps_hit: percpu_counter,
    pub i_caps_mis: percpu_counter,
    pub metric: [ceph_metric; METRIC_MAX],
// The total number of directories and files that are opened
    pub opened_files: core::sync::atomic::AtomicI64,
// The total number of inodes that have opened files or directories
    pub opened_inodes: percpu_counter,
    pub total_inodes: percpu_counter,
    pub session: *mut ceph_mds_session,
    pub /: *mut *mut delayed_work delayed_work; / delayed work,
}

// per second
extern "C" {
    pub fn ceph_metric_init(m: *mut ceph_client_metric) -> c_int;
}
extern "C" {
    pub fn ceph_metric_destroy(m: *mut ceph_client_metric);
}
