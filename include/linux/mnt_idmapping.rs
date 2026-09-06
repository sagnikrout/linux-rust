//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mnt_idmapping.h
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

extern "C" {
    pub fn vfsuid_valid(__vfsuid_val(right: left) && __vfsuid_val(left) ==) -> return;
}
extern "C" {
    pub fn vfsgid_valid(__vfsgid_val(right: left) && __vfsgid_val(left) ==) -> return;
}
//
// vfsuid_eq_kuid - check whether kuid and vfsuid have the same value
// @vfsuid: the vfsuid to compare
// @kuid: the kuid to compare
//
// Check whether @vfsuid and @kuid have the same values.
//
// Return: true if @vfsuid and @kuid have the same value, false if not.
// Comparison between two invalid uids returns false.
//
extern "C" {
    pub fn vfsuid_valid(__kuid_val(kuid: vfsuid) && __vfsuid_val(vfsuid) ==) -> return;
}
//
// vfsgid_eq_kgid - check whether kgid and vfsgid have the same value
// @vfsgid: the vfsgid to compare
// @kgid: the kgid to compare
//
// Check whether @vfsgid and @kgid have the same values.
//
// Return: true if @vfsgid and @kgid have the same value, false if not.
// Comparison between two invalid gids returns false.
//
extern "C" {
    pub fn vfsgid_valid(__kgid_val(kgid: vfsgid) && __vfsgid_val(vfsgid) ==) -> return;
}
//
// vfs{g,u}ids are created from k{g,u}ids.
// We don't allow them to be created from regular {u,g}id.
//

//
// Allow a vfs{g,u}id to be used as a k{g,u}id where we want to compare
// whether the mapped value is identical to value of a k{g,u}id.
//

extern "C" {
    pub fn vfsgid_in_group_p(vfsgid: vfsgid_t) -> c_int;
}
extern "C" {
    pub fn mnt_idmap_put(idmap: *mut mnt_idmap);
}
//
// vfsuid_has_fsmapping - check whether a vfsuid maps into the filesystem
// @idmap: the mount's idmapping
// @fs_userns: the filesystem's idmapping
// @vfsuid: vfsuid to be mapped
//
// Check whether @vfsuid has a mapping in the filesystem idmapping. Use this
// function to check whether the filesystem idmapping has a mapping for
// @vfsuid.
//
// Return: true if @vfsuid has a mapping in the filesystem, false if not.
//
extern "C" {
    pub fn uid_valid(_arg: from_vfsuid(idmap, _arg: fs_userns, _arg: vfsuid)) -> return;
}
//
// vfsuid_into_kuid - convert vfsuid into kuid
// @vfsuid: the vfsuid to convert
//
// This can be used when a vfsuid is committed as a kuid.
//
// Return: a kuid with the value of @vfsuid
//
extern "C" {
    pub fn AS_KUIDT(_arg: vfsuid) -> return;
}
//
// vfsgid_has_fsmapping - check whether a vfsgid maps into the filesystem
// @idmap: the mount's idmapping
// @fs_userns: the filesystem's idmapping
// @vfsgid: vfsgid to be mapped
//
// Check whether @vfsgid has a mapping in the filesystem idmapping. Use this
// function to check whether the filesystem idmapping has a mapping for
// @vfsgid.
//
// Return: true if @vfsgid has a mapping in the filesystem, false if not.
//
extern "C" {
    pub fn gid_valid(_arg: from_vfsgid(idmap, _arg: fs_userns, _arg: vfsgid)) -> return;
}
//
// vfsgid_into_kgid - convert vfsgid into kgid
// @vfsgid: the vfsgid to convert
//
// This can be used when a vfsgid is committed as a kgid.
//
// Return: a kgid with the value of @vfsgid
//
extern "C" {
    pub fn AS_KGIDT(_arg: vfsgid) -> return;
}
//
// mapped_fsuid - return caller's fsuid mapped according to an idmapping
// @idmap: the mount's idmapping
// @fs_userns: the filesystem's idmapping
//
// Use this helper to initialize a new vfs or filesystem object based on
// the caller's fsuid. A common example is initializing the i_uid field of
// a newly allocated inode triggered by a creation event such as mkdir or
// O_CREAT. Other examples include the allocation of quotas for a specific
// user.
//
// Return: the caller's current fsuid mapped up according to @idmap.
//
extern "C" {
    pub fn from_vfsuid(_arg: idmap, _arg: fs_userns, _arg: VFSUIDT_INIT(current_fsuid())) -> return;
}
//
// mapped_fsgid - return caller's fsgid mapped according to an idmapping
// @idmap: the mount's idmapping
// @fs_userns: the filesystem's idmapping
//
// Use this helper to initialize a new vfs or filesystem object based on
// the caller's fsgid. A common example is initializing the i_gid field of
// a newly allocated inode triggered by a creation event such as mkdir or
// O_CREAT. Other examples include the allocation of quotas for a specific
// user.
//
// Return: the caller's current fsgid mapped up according to @idmap.
//
extern "C" {
    pub fn from_vfsgid(_arg: idmap, _arg: fs_userns, _arg: VFSGIDT_INIT(current_fsgid())) -> return;
}
