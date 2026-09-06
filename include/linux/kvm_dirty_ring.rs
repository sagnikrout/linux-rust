//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kvm_dirty_ring.h
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


//
// kvm_dirty_ring: KVM internal dirty ring structure
//
// @dirty_index: free running counter that points to the next slot in
// dirty_ring->dirty_gfns, where a new dirty page should go
// @reset_index: free running counter that points to the next dirty page
// in dirty_ring->dirty_gfns for which dirty trap needs to
// be reenabled
// @size:        size of the compact list, dirty_ring->dirty_gfns
// @soft_limit:  when the number of dirty pages in the list reaches this
// limit, vcpu that owns this ring should exit to userspace
// to allow userspace to harvest all the dirty pages
// @dirty_gfns:  the array to keep the dirty gfns
// @index:       index of this dirty ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_dirty_ring {
    pub dirty_index: u32,
    pub reset_index: u32,
    pub size: u32,
    pub soft_limit: u32,
    pub dirty_gfns: *mut kvm_dirty_gfn,
    pub index: c_int,
}

//
// If CONFIG_HAVE_HVM_DIRTY_RING not defined, kvm_dirty_ring.o should
// not be included as well, so define these nop functions for the arch.
//

extern "C" {
    pub fn kvm_cpu_dirty_log_size(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_use_dirty_bitmap(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn kvm_arch_allow_write_without_running_vcpu(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn kvm_dirty_ring_get_rsvd_entries(kvm: *mut kvm) -> u32;
}
extern "C" {
    pub fn kvm_dirty_ring_push(vcpu: *mut kvm_vcpu, slot: u32, offset: u64);
}
extern "C" {
    pub fn kvm_dirty_ring_check_request(vcpu: *mut kvm_vcpu) -> bool;
}
// for use in vm_operations_struct
extern "C" {
    pub fn kvm_dirty_ring_free(ring: *mut kvm_dirty_ring);
}

