//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kobject.h
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
// kobject.h - generic kernel object infrastructure.
//
// Copyright (c) 2002-2003 Patrick Mochel
// Copyright (c) 2002-2003 Open Source Development Labs
// Copyright (c) 2006-2008 Greg Kroah-Hartman <greg@kroah.com>
// Copyright (c) 2006-2008 Novell Inc.
//
// Please read Documentation/core-api/kobject.rst before using the kobject
// interface, ESPECIALLY the parts about reference counts and object
// destructors.
//

pub const UEVENT_HELPER_PATH_LEN: c_int = 256;

// path to the userspace helper executed on an event

// counter to tag the uevent, read only except for the kobject core
//
// The actions here must match the index to the string array
// in lib/kobject_uevent.c
//
// Do not add new actions here without checking with the driver-core
// maintainers. Action strings are not meant to express subsystem
// or device specific properties. In most cases you want to send a
// kobject_uevent_env(kobj, KOBJ_CHANGE, env) with additional event
// specific variables added to the event environment.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kobject_action {
    KOBJ_ADD,
    KOBJ_REMOVE,
    KOBJ_CHANGE,
    KOBJ_MOVE,
    KOBJ_ONLINE,
    KOBJ_OFFLINE,
    KOBJ_BIND,
    KOBJ_UNBIND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kobject {
    pub name: *const c_char,
    pub entry: list_head,
    pub parent: *mut kobject,
    pub kset: *mut kset,
    pub ktype: *const kobj_type,
    pub /: *mut *mut *mut kernfs_node sd; / sysfs directory entry,
    pub kref: kref,
    pub state_initialized:1: c_uint,
    pub state_in_sysfs:1: c_uint,
    pub state_add_uevent_sent:1: c_uint,
    pub state_remove_uevent_sent:1: c_uint,
    pub uevent_suppress:1: c_uint,

    pub release: delayed_work,

}

extern "C" {
    pub fn kobject_init(kobj: *mut kobject, ktype: *const kobj_type);
}
extern "C" {
    pub fn kobject_del(kobj: *mut kobject);
}
extern "C" {
    pub fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject  __must_check;
}
extern "C" {
    pub fn kobject_rename(: *mut kobject, new_name: *const c_char) -> int __must_check;
}
extern "C" {
    pub fn kobject_move(: *mut kobject, : *mut kobject) -> int __must_check;
}
extern "C" {
    pub fn kobject_get_unless_zero(kobj: *mut kobject) -> *mut kobject  __must_check;
}
extern "C" {
    pub fn kobject_put(kobj: *mut kobject);
}
extern "C" {
    pub fn kobject_get_ownership(kobj: *const kobject, uid: *mut kuid_t, gid: *mut kgid_t);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kobj_type {
    pub kobj): *mut *mut void (release)(struct kobject,
    pub sysfs_ops: *const sysfs_ops,
    pub default_groups: *const attribute_group,
    pub kobj): *const *const *const kobj_ns_type_operations (child_ns_type)(kobject,
    pub kobj): *const *const *const ns_common (namespace)(kobject,
    pub gid): *const *const *const *const void (get_ownership)(struct kobject kobj, kuid_t uid, kgid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kobj_uevent_env {
    pub argv: [*mut c_char; 3],
    pub envp: [*mut c_char; UEVENT_NUM_ENVP],
    pub envp_idx: c_int,
    pub buf: [c_char; UEVENT_BUFFER_SIZE],
    pub buflen: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kset_uevent_ops {
    pub kobj): *const *const int ( filter)(struct kobject,
    pub kobj): *const *const *const char ( name)(struct kobject,
    pub env): *const *const *const int ( uevent)(struct kobject kobj, struct kobj_uevent_env,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kobj_attribute {
    pub attr: attribute,
    pub buf): *mut *mut *mut *mut ssize_t (show)(struct kobject kobj, struct kobj_attribute attr, char,
    pub buf): *mut c_char,
    pub count): *const *const char buf, size_t,
    pub count): *const *const char buf, size_t,
}

//
// struct kset - a set of kobjects of a specific type, belonging to a specific subsystem.
//
// A kset defines a group of kobjects.  They can be individually
// different "types" but overall these kobjects all want to be grouped
// together and operated on in the same manner.  ksets are used to
// define the attribute callbacks and other common events that happen to
// a kobject.
//
// @list: the list of all kobjects for this kset
// @list_lock: a lock for iterating over the kobjects
// @kobj: the embedded kobject for this kset (recursion, isn't it fun...)
// @uevent_ops: the set of uevent operations for this kset.  These are
// called whenever a kobject has something happen to it so that the kset
// can add new environment variables, or filter out the uevents if so
// desired.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kset {
    pub list: list_head,
    pub list_lock: spinlock_t,
    pub kobj: kobject,
    pub uevent_ops: *const kset_uevent_ops,
    pub __randomize_layout: },
    pub kset): *mut void kset_init(struct kset,
    pub kset): *mut int __must_check kset_register(struct kset,
    pub kset): *mut void kset_unregister(struct kset,
    pub parent_kobj): *mut kobject,
    pub NULL: return kobj ? container_of(kobj, struct kset, kobj) :,
    pub NULL: return k ? to_kset(kobject_get(&k->kobj)) :,
    pub kobj->ktype: return,
    pub ): *const *const *const kobject kset_find_obj(kset , char,
// The global /sys/kernel/ kobject for people to chain off of
    pub kernel_kobj: *mut extern struct kobject,
// The global /sys/kernel/mm/ kobject for people to chain off of
    pub mm_kobj: *mut extern struct kobject,
// The global /sys/hypervisor/ kobject for people to chain off of
    pub hypervisor_kobj: *mut extern struct kobject,
// The global /sys/power/ kobject for people to chain off of
    pub power_kobj: *mut extern struct kobject,
// The global /sys/firmware/ kobject for people to chain off of
    pub firmware_kobj: *mut extern struct kobject,
    pub action): *mut *mut int kobject_uevent(struct kobject kobj, enum kobject_action,
    pub envp[]): *mut c_char,
    pub count): *const *const *const int kobject_synth_uevent(struct kobject kobj, char buf, size_t,
    pub ...): *const *const *const int add_uevent_var(struct kobj_uevent_env env, char format,,
