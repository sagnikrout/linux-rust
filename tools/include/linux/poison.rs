//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/poison.h
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
// include/linux/list.h
//
// Architectures might want to move the poison pointer offset
// into some well-recognized area such as 0xdead000000000000,
// that is also not mappable by user-space exploits:
//

//
// These are non-NULL pointers that will result in page faults
// under normal circumstances, used to verify that nobody uses
// non-initialized list entries.
//

// include/linux/timer.h
//
// Magic number "tsta" to indicate a static timer initializer
// for the object debugging code.
//

// mm/page_poison.c
pub const PAGE_POISON: c_uint = 0xaa;
// mm/page_alloc.c

// mm/slab.c
//
// Magic nums for obj red zoning.
// Placed in the first word before and the first word after an obj.
//
pub const SLUB_RED_INACTIVE: c_uint = 0xbb	/* when obj is inactive */;
pub const SLUB_RED_ACTIVE: c_uint = 0xcc	/* when obj is active */;
// ...and for poisoning
pub const POISON_INUSE: c_uint = 0x5a	/* for use-uninitialised poisoning */;
pub const POISON_FREE: c_uint = 0x6b	/* for use-after-free poisoning */;
pub const POISON_END: c_uint = 0xa5	/* end-byte of poisoning */;
// arch/$ARCH/mm/init.c
pub const POISON_FREE_INITMEM: c_uint = 0xcc;
// arch/ia64/hp/common/sba_iommu.c
//
// arch/ia64/hp/common/sba_iommu.c uses a 16-byte poison string with a
// value of "SBAIOMMU POISON\0" for spill-over poisoning.
//
// fs/jbd/journal.c
pub const JBD_POISON_FREE: c_uint = 0x5b;
pub const JBD2_POISON_FREE: c_uint = 0x5c;
// drivers/base/dmapool.c
pub const POOL_POISON_FREED: c_uint = 0xa7	/* !inuse */;
pub const POOL_POISON_ALLOCATED: c_uint = 0xa9	/* !initted */;
// drivers/atm/
pub const ATM_POISON_FREE: c_uint = 0x12;
pub const ATM_POISON: c_uint = 0xdeadbeef;
// kernel/mutexes
pub const MUTEX_DEBUG_INIT: c_uint = 0x11;
pub const MUTEX_DEBUG_FREE: c_uint = 0x22;
// security/
pub const KEY_DESTROY: c_uint = 0xbd;
