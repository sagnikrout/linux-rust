//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/types.h
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

pub type s128 = __s128;
pub type u128 = __u128;

pub type __kernel_dev_t = u32;
pub type fd_set = __kernel_fd_set;
pub type dev_t = __kernel_dev_t;
pub type ino_t = __kernel_ulong_t;
pub type mode_t = __kernel_mode_t;
pub type umode_t = c_ushort;
pub type nlink_t = u32;
pub type off_t = __kernel_off_t;
pub type pid_t = __kernel_pid_t;
pub type daddr_t = __kernel_daddr_t;
pub type key_t = __kernel_key_t;
pub type suseconds_t = __kernel_suseconds_t;
pub type timer_t = __kernel_timer_t;
pub type clockid_t = __kernel_clockid_t;
pub type mqd_t = __kernel_mqd_t;
pub type bool = _Bool;
pub type uid_t = __kernel_uid32_t;
pub type gid_t = __kernel_gid32_t;
pub type uid16_t = __kernel_uid16_t;
pub type gid16_t = __kernel_gid16_t;
pub type uintptr_t = c_ulong;
pub type intptr_t = c_long;

// This is defined by arch/{arch}/include/asm/posix_types.h
pub type old_uid_t = __kernel_old_uid_t;
pub type old_gid_t = __kernel_old_gid_t;

pub type loff_t = __kernel_loff_t;
pub type uoff_t = __kernel_uoff_t;

//
// The following typedefs are also protected by individual ifdefs for
// historical reasons:
//
pub type size_t = __kernel_size_t;

pub type ssize_t = __kernel_ssize_t;

pub type ptrdiff_t = __kernel_ptrdiff_t;

pub type clock_t = __kernel_clock_t;

pub type caddr_t = __kernel_caddr_t;

// bsd
pub type u_char = c_uchar;
pub type u_short = c_ushort;
pub type u_int = c_uint;
pub type u_long = c_ulong;
// sysv
pub type unchar = c_uchar;
pub type ushort = c_ushort;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type ullong = c_ulonglong;
pub type u_int8_t = u8;
pub type int8_t = i8;
pub type u_int16_t = u16;
pub type int16_t = i16;
pub type u_int32_t = u32;
pub type int32_t = i32;

pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;

pub type uint64_t = u64;
pub type u_int64_t = u64;
pub type int64_t = i64;

// These are the special 64-bit data types that are 8-byte aligned

// Nanosecond scalar representation for kernel time values
pub type ktime_t = i64;
//
// The type used for indexing onto a disc or disc partition.
//
// Linux always considers sectors to be 512 bytes long independently
// of the devices real block size.
//
// blkcnt_t is the type of the inode's block count.
//
pub type sector_t = u64;
pub type blkcnt_t = u64;
// generic data direction definitions
pub const READ: c_int = 0;
pub const WRITE: c_int = 1;
//
// The type of an index into the pagecache.
//

//
// A dma_addr_t can hold any valid DMA address, i.e., any address returned
// by the DMA API.
//
// If the DMA API only uses 32-bit addresses, dma_addr_t need only be 32
// bits wide.  Bus addresses, e.g., PCI BARs, may be wider than 32 bits,
// but drivers do memory-mapped I/O to ioremapped kernel virtual addresses,
// so they don't care about the size of the actual bus addresses.
//

pub type dma_addr_t = u64;

pub type dma_addr_t = u32;

pub type gfp_t = u32;
pub type slab_flags_t = u32;
pub type fmode_t = u32;
pub type blk_mode_t = u32;
pub type fop_flags_t = u32;

pub type phys_addr_t = u64;

pub type phys_addr_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct phys_vec {
    pub paddr: phys_addr_t,
    pub len: usize,
}

pub type resource_size_t = phys_addr_t;
//
// This type is the placeholder for a hardware interrupt number. It has to be
// big enough to enclose whatever representation is used by a given platform.
//
pub type irq_hw_number_t = c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub prev: *mut *mut list_head next,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_node {
    pub pprev: *mut *mut hlist_node next,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustat {
    pub f_tfree: __kernel_daddr_t,

    pub f_tinode: c_uint,

    pub f_tinode: c_ulong,
    pub f_fname: [c_char; 6],
    pub f_fpack: [c_char; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcov_common_handle_id {

    pub val: u64,

}

//
// struct callback_head - callback structure for use with RCU and task_work
// @next: next update requests in a list
// @func: actual update function to call after the grace period.
//
// The struct is aligned to size of pointer. On most architectures it happens
// naturally due ABI requirements, but some architectures (like CRIS) have
// weird ABI and we need to ask it explicitly.
//
// The alignment is required to guarantee that bit 0 of @next will be
// clear under normal conditions -- as long as we use call_rcu() or
// call_srcu() to queue the callback.
//
// This guarantee is important for few reasons:
// - future call_rcu_lazy() will make use of lower bits in the pointer;
// - the structure shares storage space in struct page with @compound_info,
// which encode PageTail() in bit 0. The guarantee is needed to avoid
// false-positive PageTail().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_head {
    pub next: *mut callback_head,
    pub func: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvfree_rcu_head {
    pub next: *mut kvfree_rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvfree_rcu_head {
    pub head: rcu_head,
}

extern "C" {
    pub fn void(head: *mut *mut rcu_callback_t)(struct rcu_head) -> typedef;
}
extern "C" {
    pub fn void(head: *mut *mut call_rcu_func_t)(struct rcu_head, func: rcu_callback_t) -> typedef;
}
extern "C" {
    pub fn void(a: *mut *mut swap_r_func_t)(void, b: *mut c_void, size: c_int, priv: *const c_void) -> typedef;
}
extern "C" {
    pub fn void(a: *mut *mut swap_func_t)(void, b: *mut c_void, size: c_int) -> typedef;
}
extern "C" {
    pub fn int(a: *const *const cmp_r_func_t)(void, b: *const c_void, priv: *const c_void) -> typedef;
}
extern "C" {
    pub fn int(a: *const *const cmp_func_t)(void, b: *const c_void) -> typedef;
}
//
// rcuwait provides a way of blocking and waking up a single
// task in an rcu-safe manner.
//
// The only time @task is non-nil is when a user is blocked (or
// checking if it needs to) on a condition, and reset as soon as we
// know that the condition has succeeded and are awoken.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcuwait {
    pub task: *mut task___rcu,
}

