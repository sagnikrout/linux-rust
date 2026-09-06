//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/v3d/v3d_sysfs.c
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Igalia S.L.
//

    static ssize_t
    gpu_stats_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct drm_device *drm = dev_get_drvdata(dev);
    struct v3d_dev *v3d = to_v3d_dev(drm);
    enum v3d_queue queue;
    let mut timestamp: u64 = local_clock();
    let mut len: isize = 0;
    len += sysfs_emit(buf, "queue\ttimestamp\tjobs\truntime\n");
    for (queue = 0; queue < V3D_MAX_QUEUES; queue++) {
    struct v3d_stats *stats = v3d.queue[queue].stats;
    u64 active_runtime, jobs_completed;
    v3d_get_stats(stats, timestamp, &active_runtime, &jobs_completed);
// Each line will display the queue name, timestamp, the number
// of jobs sent to that queue and the runtime, as can be seem here:
//
// queue	timestamp	jobs	runtime
// bin		239043069420	22620	17438164056
// render	239043069420	22619	27284814161
// tfu		239043069420	8763	394592566
// csd		239043069420	3168	10787905530
// cache_clean	239043069420	6127	237375940
//
    len += sysfs_emit_at(buf, len, "%s\t%llu\t%llu\t%llu\n",
    v3d_queue_to_string(queue),
    timestamp, jobs_completed, active_runtime);
    }
    return len;
    }
    static DEVICE_ATTR_RO(gpu_stats);
    static struct attribute *v3d_sysfs_entries[] = {
    &dev_attr_gpu_stats.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group v3d_sysfs_attr_group = {
    .attrs = v3d_sysfs_entries,
    };
    int
    v3d_sysfs_init(struct device *dev)
    {
    return sysfs_create_group(&dev.kobj, &v3d_sysfs_attr_group);
    }
    void
    v3d_sysfs_destroy(struct device *dev)
    {
    return sysfs_remove_group(&dev.kobj, &v3d_sysfs_attr_group);
    }
