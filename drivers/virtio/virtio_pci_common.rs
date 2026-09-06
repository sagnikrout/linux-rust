//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/virtio/virtio_pci_common.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Virtio PCI driver - APIs for common functionality for all device versions
//
// This module allows virtio devices to be used over a virtual PCI device.
// This can be used with QEMU based VMMs like KVM or Xen.
//
// Copyright IBM Corp. 2007
// Copyright Red Hat, Inc. 2014
//
// Authors:
// Anthony Liguori  <aliguori@us.ibm.com>
// Rusty Russell <rusty@rustcorp.com.au>
// Michael S. Tsirkin <mst@redhat.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_vq_info {
// the actual virtqueue
    pub vq: *mut virtqueue,
// the list node for the virtqueues or slow_virtqueues list
    pub node: list_head,
// MSI-X vector (or none)
    pub msix_vector: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_admin_vq {
// Virtqueue info associated with this admin queue.
    pub info: *mut virtio_pci_vq_info,
// Protects virtqueue access.
    pub lock: spinlock_t,
    pub supported_cmds: u64,
    pub supported_caps: u64,
    pub max_dev_parts_objects: u8,
    pub dev_parts_ida: ida,
// Name of the admin queue: avq.$vq_index.
    pub name: [c_char; 10],
    pub vq_index: u16,
}

// Our device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_device {
    pub vdev: virtio_device,
    pub pci_dev: *mut pci_dev,
    pub ldev: virtio_pci_legacy_device,
    pub mdev: virtio_pci_modern_device,
}

// Where to read and clear interrupt
// Lists of queues and potentially slow path queues
// so we can dispatch IRQs.
//
// Array of all virtqueues reported in the
// PCI common config num_queues field
//
// MSI-X support
// Name strings for interrupts. This size should be enough,
// and I'm too lazy to allocate each name separately.
// Number of available vectors
// Vectors allocated, excluding per-vq vectors if any
// Whether we have vector per vq
// Constants for MSI-X
// Use first vector for configuration changes, second and the rest for
// virtqueues Thus, we need at least 2 vectors for MSI.
// Convert a generic virtio device to our structure
extern "C" {
    pub fn container_of(_arg: vdev, virtio_pci_device: struct, _arg: vdev) -> return;
}
// wait for pending irq handlers
extern "C" {
    pub fn vp_synchronize_vectors(vdev: *mut virtio_device);
}
// the notify function used when creating a virt queue
extern "C" {
    pub fn vp_notify(vq: *mut virtqueue) -> bool;
}
// the config->del_vqs() implementation
extern "C" {
    pub fn vp_del_vqs(vdev: *mut virtio_device);
}
// the config->find_vqs() implementation
// Setup the affinity for a virtqueue:
// - force the affinity for per vq vector
// - OR over all affinities for shared MSI
// - ignore the affinity request if we're using INTX
//
extern "C" {
    pub fn vp_set_vq_affinity(vq: *mut virtqueue, cpu_mask: *const cpumask) -> c_int;
}

extern "C" {
    pub fn virtio_pci_legacy_probe(: *mut virtio_pci_device) -> c_int;
}
extern "C" {
    pub fn virtio_pci_legacy_remove(: *mut virtio_pci_device);
}

extern "C" {
    pub fn virtio_pci_modern_probe(: *mut virtio_pci_device) -> c_int;
}
extern "C" {
    pub fn virtio_pci_modern_remove(: *mut virtio_pci_device);
}

// Unlike modern drivers which support hardware virtio devices, legacy drivers
// assume software-based devices: e.g. they don't use proper memory barriers
// on ARM, use big endian on PPC, etc. X86 drivers are mostly ok though, more
// or less by chance. For now, only support legacy IO on X86.
//

extern "C" {
    pub fn vp_is_avq(vdev: *mut virtio_device, index: c_uint) -> bool;
}
extern "C" {
    pub fn vp_modern_avq_done(vq: *mut virtqueue);
}
