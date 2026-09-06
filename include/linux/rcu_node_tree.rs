//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rcu_node_tree.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// RCU node combining tree definitions.  These are used to compute
// global attributes while avoiding common-case global contention.  A key
// property that these computations rely on is a tournament-style approach
// where only one of the tasks contending a lower level in the tree need
// advance to the next higher level.  If properly configured, this allows
// unlimited scalability while maintaining a constant level of contention
// on the root node.
//
// This seemingly RCU-private file must be available to SRCU users
// because the size of the TREE SRCU srcu_struct structure depends
// on these definitions.
//
// Copyright IBM Corporation, 2017
//
// Author: Paul E. McKenney <paulmck@linux.ibm.com>
//

//
// Define shape of hierarchy based on NR_CPUS, CONFIG_RCU_FANOUT, and
// CONFIG_RCU_FANOUT_LEAF.
// In theory, it should be possible to add more levels straightforwardly.
// In practice, this did work well going from three levels to four.
// Of course, your mileage may vary.
//

pub const RCU_FANOUT_LEAF: c_int = 16;

