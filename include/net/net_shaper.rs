//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/net_shaper.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum net_shaper_binding_type {
    NET_SHAPER_BINDING_TYPE_NETDEV,
// NET_SHAPER_BINDING_TYPE_DEVLINK_PORT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_shaper_binding {
    pub type: net_shaper_binding_type,
    pub netdev: *mut net_device,
    pub devlink: *mut devlink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_shaper_handle {
    pub scope: net_shaper_scope,
    pub id: u32,
}

//
// struct net_shaper - represents a shaping node on the NIC H/W
// zeroed field are considered not set.
// @parent: Unique identifier for the shaper parent, usually implied
// @handle: Unique identifier for this shaper
// @metric: Specify if the rate limits refers to PPS or BPS
// @bw_min: Minimum guaranteed rate for this shaper
// @bw_max: Maximum peak rate allowed for this shaper
// @burst: Maximum burst for the peek rate of this shaper
// @priority: Scheduling priority for this shaper
// @weight: Scheduling weight for this shaper
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_shaper {
    pub parent: net_shaper_handle,
    pub handle: net_shaper_handle,
    pub metric: net_shaper_metric,
    pub bw_min: u64,
    pub bw_max: u64,
    pub burst: u64,
    pub priority: u32,
    pub weight: u32,
// private:
    pub /: *mut *mut u32 leaves; / accounted only for NODE scope,
    pub valid: bool,
    pub rcu: rcu_head,
}

//
// struct net_shaper_ops - Operations on device H/W shapers
//
// The operations applies to either net_device and devlink objects.
// The initial shaping configuration at device initialization is empty:
// does not constraint the rate in any way.
// The network core keeps track of the applied user-configuration in
// the net_device or devlink structure.
// The operations are serialized via a per device lock.
//
// Device not supporting any kind of nesting should not provide the
// @group operation.
//
// Each shaper is uniquely identified within the device with a 'handle'
// comprising the shaper scope and a scope-specific id.
//
// Driver ops vs uAPI
// ------------------
// Members of the driver ops mirror the Netlink uAPI but driver calls do not
// map 1:1 to user calls. Drivers need to be careful when assuming that calls
// disallowed at the uAPI level will never be made at the driver level.
// The shaper core performs automatic reparenting and cleanup, generating
// additional calls. Notably:
//
// - @group calls in the driver facing API may have nodes as leaves (user is
// only allowed to construct groups with queues as leaves)
// - @group calls may update leaf's parent if the parent is about
// to be removed (re-parenting nodes explicitly is not supported in the uAPI)
//
// Implicit creation
// -----------------
// Shapers are created implicitly, meaning that @set and @group operations
// are called both for existing and new shapers. The driver has to infer
// whether the operation is an update or a creation by tracking the handles.
// Removal of shapers is explicit and done with a @delete call.
//
// The @set operation implicitly creates NET_SHAPER_SCOPE_NETDEV and
// NET_SHAPER_SCOPE_QUEUE shapers.
// The @group operation implicitly creates NET_SHAPER_SCOPE_NETDEV and
// NET_SHAPER_SCOPE_NODE shapers (the group shaper itself), as well as
// NET_SHAPER_SCOPE_QUEUE shapers (leaves).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_shaper_ops {
//
// @group: create a scheduling group or add leaves
//
// Nest the @leaves shapers identified under the @node shaper.
// All the shapers belong to the device specified by @binding.
// The @leaves array's size is specified by @leaves_count.
//
// @node and @leaves may or may not already exist
// (see the "Implicit creation" note). If @node already exists,
// the @leaves should be *added* to its children. In this case,
// the @leaves array only holds new/modified leaves, not the full list.
//
// Re-parenting @leaves is implemented by a @group call on a new parent.
// There's no explicit call to remove the children from the old parent.
//
    pub extack): *mut netlink_ext_ack,
//
// @set: Updates the specified shaper
//
// Updates or creates the @shaper on the device specified by @binding.
//
    pub extack): *mut netlink_ext_ack,
//
// @delete: Removes the specified shaper
//
// Removes the shaper configuration as identified by the given @handle
// on the device specified by @binding, restoring the default behavior.
//
// Note that a @delete call on a NET_SHAPER_SCOPE_QUEUE shaper also
// implicitly removes the associated queue from the scheduling
// hierarchy. The driver must take care of that step.
// @delete calls on NET_SHAPER_SCOPE_NODE should not require any
// implicit re-parenting in the driver as core will re-parent the leaves
// first, before deleting the SCOPE_NODE shaper.
//
    pub extack): *mut netlink_ext_ack,
//
// @capabilities: get the shaper features supported by the device
//
// Fills the bitmask @cap with the supported capabilities for the
// specified @scope and device specified by @binding.
//
    pub cap): *mut net_shaper_scope scope, unsigned long,
}
