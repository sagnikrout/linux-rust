//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/kvm_page_track.h
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
// The notifier represented by @kvm_page_track_notifier_node is linked into
// the head which will be notified when guest is triggering the track event.
//
// Write access on the head is protected by kvm->mmu_lock, read access
// is protected by track_srcu.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_page_track_notifier_head {
    pub track_srcu: srcu_struct,
    pub track_notifier_list: hlist_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_page_track_notifier_node {
    pub node: hlist_node,
//
// It is called when guest is writing the write-tracked page
// and write emulation is finished at that time.
//
// @gpa: the physical address written by guest.
// @new: the data was written to the address.
// @bytes: the written length.
// @node: this node
//
    pub node): *mut kvm_page_track_notifier_node,
//
// Invoked when a memory region is removed from the guest.  Or in KVM
// terms, when a memslot is deleted.
//
// @gfn:       base gfn of the region being removed
// @nr_pages:  number of pages in the to-be-removed region
// @node:      this node
//
    pub node): *mut kvm_page_track_notifier_node,
}

extern "C" {
    pub fn kvm_write_track_add_gfn(kvm: *mut kvm, gfn: gfn_t) -> c_int;
}
extern "C" {
    pub fn kvm_write_track_remove_gfn(kvm: *mut kvm, gfn: gfn_t) -> c_int;
}

//
// Allow defining a node in a structure even if page tracking is disabled, e.g.
// to play nice with testing headers via direct inclusion from the command line.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_page_track_notifier_node {

