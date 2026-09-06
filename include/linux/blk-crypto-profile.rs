//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/blk-crypto-profile.h
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
// Copyright 2019 Google LLC
//

//
// struct blk_crypto_ll_ops - functions to control inline encryption hardware
//
// Low-level operations for controlling inline encryption hardware.  This
// interface must be implemented by storage drivers that support inline
// encryption.  All functions may sleep, are serialized by profile->lock, and
// are never called while profile->dev (if set) is runtime-suspended.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_ll_ops {
//
// @keyslot_program: Program a key into the inline encryption hardware.
//
// Program @key into the specified @slot in the inline encryption
// hardware, overwriting any key that the keyslot may already contain.
// The keyslot is guaranteed to not be in-use by any I/O.
//
// This is required if the device has keyslots.  Otherwise (i.e. if the
// device is a layered device, or if the device is real hardware that
// simply doesn't have the concept of keyslots) it is never called.
//
// Must return 0 on success, or -errno on failure.
//
    pub slot): c_uint,
//
// @keyslot_evict: Evict a key from the inline encryption hardware.
//
// If the device has keyslots, this function must evict the key from the
// specified @slot.  The slot will contain @key, but there should be no
// need for the @key argument to be used as @slot should be sufficient.
// The keyslot is guaranteed to not be in-use by any I/O.
//
// If the device doesn't have keyslots itself, this function must evict
// @key from any underlying devices.  @slot won't be valid in this case.
//
// If there are no keyslots and no underlying devices, this function
// isn't required.
//
// Must return 0 on success, or -errno on failure.
//
    pub slot): c_uint,
//
// @derive_sw_secret: Derive the software secret from a hardware-wrapped
// key in ephemerally-wrapped form.
//
// This only needs to be implemented if BLK_CRYPTO_KEY_TYPE_HW_WRAPPED
// is supported.
//
// Must return 0 on success, -EBADMSG if the key is invalid, or another
// -errno code on other errors.
//
    pub sw_secret[BLK_CRYPTO_SW_SECRET_SIZE]): u8,
//
// @import_key: Create a hardware-wrapped key by importing a raw key.
//
// This only needs to be implemented if BLK_CRYPTO_KEY_TYPE_HW_WRAPPED
// is supported.
//
// On success, must write the new key in long-term wrapped form to
// @lt_key and return its size in bytes.  On failure, must return a
// -errno value.
//
    pub lt_key[BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE]): u8,
//
// @generate_key: Generate a hardware-wrapped key.
//
// This only needs to be implemented if BLK_CRYPTO_KEY_TYPE_HW_WRAPPED
// is supported.
//
// On success, must write the new key in long-term wrapped form to
// @lt_key and return its size in bytes.  On failure, must return a
// -errno value.
//
    pub lt_key[BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE]): u8,
//
// @prepare_key: Prepare a hardware-wrapped key to be used.
//
// Prepare a hardware-wrapped key to be used by converting it from
// long-term wrapped form to ephemerally-wrapped form.  This only needs
// to be implemented if BLK_CRYPTO_KEY_TYPE_HW_WRAPPED is supported.
//
// On success, must write the key in ephemerally-wrapped form to
// @eph_key and return its size in bytes.  On failure, must return
// -EBADMSG if the key is invalid, or another -errno on other error.
//
    pub eph_key[BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE]): u8,
}

//
// struct blk_crypto_profile - inline encryption profile for a device
//
// This struct contains a storage device's inline encryption capabilities (e.g.
// the supported crypto algorithms), driver-provided functions to control the
// inline encryption hardware (e.g. programming and evicting keys), and optional
// device-independent keyslot management data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_crypto_profile {
// public: Drivers must initialize the following fields.
//
// @ll_ops: Driver-provided functions to control the inline encryption
// hardware, e.g. program and evict keys.
//
    pub ll_ops: blk_crypto_ll_ops,
//
// @max_dun_bytes_supported: The maximum number of bytes supported for
// specifying the data unit number (DUN).  Specifically, the range of
// supported DUNs is 0 through (1 << (8 * max_dun_bytes_supported)) - 1.
//
    pub max_dun_bytes_supported: c_uint,
//
// @key_types_supported: A bitmask of the supported key types:
// BLK_CRYPTO_KEY_TYPE_RAW and/or BLK_CRYPTO_KEY_TYPE_HW_WRAPPED.
//
    pub key_types_supported: c_uint,
//
// @modes_supported: Array of bitmasks that specifies whether each
// combination of crypto mode and data unit size is supported.
// Specifically, the i'th bit of modes_supported[crypto_mode] is set if
// crypto_mode can be used with a data unit size of (1 << i).  Note that
// only data unit sizes that are powers of 2 can be supported.
//
    pub modes_supported: [c_uint; BLK_ENCRYPTION_MODE_MAX],
//
// @dev: An optional device for runtime power management.  If the driver
// provides this device, it will be runtime-resumed before any function
// in @ll_ops is called and will remain resumed during the call.
//
    pub dev: *mut device,
// private: The following fields shouldn't be accessed by drivers.
// Number of keyslots, or 0 if not applicable
    pub num_slots: c_uint,
//
// Serializes all calls to functions in @ll_ops as well as all changes
// to @slot_hashtable.  This can also be taken in read mode to look up
// keyslots while ensuring that they can't be changed concurrently.
//
    pub lock: rw_semaphore,
    pub lockdep_key: lock_class_key,
// List of idle slots, with least recently used slot at front
    pub idle_slots_wait_queue: wait_queue_head_t,
    pub idle_slots: list_head,
    pub idle_slots_lock: spinlock_t,
//
// Hash table which maps struct *blk_crypto_key to keyslots, so that we
// can find a key's keyslot in O(1) time rather than O(num_slots).
// Protected by 'lock'.
//
    pub slot_hashtable: *mut hlist_head,
    pub log_slot_ht_size: c_uint,
// Per-keyslot data
    pub slots: *mut blk_crypto_keyslot,
}

extern "C" {
    pub fn blk_crypto_keyslot_index(slot: *mut blk_crypto_keyslot) -> c_uint;
}
extern "C" {
    pub fn blk_crypto_reprogram_all_keys(profile: *mut blk_crypto_profile);
}
extern "C" {
    pub fn blk_crypto_profile_destroy(profile: *mut blk_crypto_profile);
}
