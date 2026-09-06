//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/action-manager.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2023 Red Hat
//

//
// An action_manager provides a generic mechanism for applying actions to multi-zone entities (such
// as the block map or slab depot). Each action manager is tied to a specific context for which it
// manages actions. The manager ensures that only one action is active on that context at a time,
// and supports at most one pending action. Calls to schedule an action when there is already a
// pending action will result in VDO_COMPONENT_BUSY errors. Actions may only be submitted to the
// action manager from a single thread (which thread is determined when the action manager is
// constructed).
//
// A scheduled action consists of four components:
//
// preamble
// an optional method to be run on the initiator thread before applying the action to all zones
// zone_action
// an optional method to be applied to each of the zones
// conclusion
// an optional method to be run on the initiator thread once the per-zone method has been
// applied to all zones
// parent
// an optional completion to be finished once the conclusion is done
//
// At least one of the three methods must be provided.
//
// A function which is to be applied asynchronously to a set of zones.
// @context: The object which holds the per-zone context for the action
// @zone_number: The number of zone to which the action is being applied
// @parent: The object to notify when the action is complete
//
// A function which is to be applied asynchronously on an action manager's initiator thread as the
// preamble of an action.
// @context: The object which holds the per-zone context for the action
// @parent: The object to notify when the action is complete
//
extern "C" {
    pub fn void(context: *mut *mut vdo_action_preamble_fn)(void, parent: *mut vdo_completion) -> typedef;
}
//
// A function which will run on the action manager's initiator thread as the conclusion of an
// action.
// @context: The object which holds the per-zone context for the action
//
// Return: VDO_SUCCESS or an error
//
extern "C" {
    pub fn int(context: *mut *mut vdo_action_conclusion_fn)(void) -> typedef;
}
//
// A function to schedule an action.
// @context: The object which holds the per-zone context for the action
//
// Return: true if an action was scheduled
//
extern "C" {
    pub fn bool(context: *mut *mut vdo_action_scheduler_fn)(void) -> typedef;
}
//
// A function to get the id of the thread associated with a given zone.
// @context: The action context
// @zone_number: The number of the zone for which the thread ID is desired
//
extern "C" {
    pub fn thread_id_t(context: *mut *mut vdo_zone_thread_getter_fn)(void, zone_number: zone_count_t) -> typedef;
}
extern "C" {
    pub fn vdo_get_current_action_context(manager: *mut action_manager) -> *mut void  __must_check;
}
extern "C" {
    pub fn vdo_schedule_default_action(manager: *mut action_manager) -> bool;
}
