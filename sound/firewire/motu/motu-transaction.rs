//! Automatically rewritten from C to Rust
//! Source: sound/firewire/motu/motu-transaction.c
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
// motu-transaction.c - a part of driver for MOTU FireWire series
//
// Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
//

pub const SND_MOTU_ADDR_BASE: c_uint = 0xfffff0000000ULL;
pub const ASYNC_ADDR_HI: c_uint = 0x0b04;
pub const ASYNC_ADDR_LO: c_uint = 0x0b08;
    int snd_motu_transaction_read(struct snd_motu *motu, u32 offset, __be32 *reg,
    size_t size)
    {
    int tcode;
    if (size % sizeof(__be32) > 0 || size <= 0)
    return -EINVAL;
    if (size == sizeof(__be32))
    tcode = TCODE_READ_QUADLET_REQUEST;
    else
    tcode = TCODE_READ_BLOCK_REQUEST;
    return snd_fw_transaction(motu.unit, tcode,
    SND_MOTU_ADDR_BASE + offset, reg, size, 0);
    }
    int snd_motu_transaction_write(struct snd_motu *motu, u32 offset, __be32 *reg,
    size_t size)
    {
    int tcode;
    if (size % sizeof(__be32) > 0 || size <= 0)
    return -EINVAL;
    if (size == sizeof(__be32))
    tcode = TCODE_WRITE_QUADLET_REQUEST;
    else
    tcode = TCODE_WRITE_BLOCK_REQUEST;
    return snd_fw_transaction(motu.unit, tcode,
    SND_MOTU_ADDR_BASE + offset, reg, size, 0);
    }
    static void handle_message(struct fw_card *card, struct fw_request *request,
    int tcode, int destination, int source,
    int generation, unsigned long long offset,
    void *data, size_t length, void *callback_data)
    {
    struct snd_motu *motu = callback_data;
    __be32 *buf = (__be32 *)data;
    if (tcode != TCODE_WRITE_QUADLET_REQUEST) {
    fw_send_response(card, request, RCODE_COMPLETE);
    return;
    }
    if (offset != motu.async_handler.offset || length != 4) {
    fw_send_response(card, request, RCODE_ADDRESS_ERROR);
    return;
    }
    scoped_guard(spinlock_irqsave, &motu.lock) {
    motu.msg = be32_to_cpu(*buf);
    }
    fw_send_response(card, request, RCODE_COMPLETE);
    wake_up(&motu.hwdep_wait);
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_transaction_reregister(motu: *mut snd_motu) -> c_int {
    int snd_motu_transaction_reregister(struct snd_motu *motu)
    {
    struct fw_device *device = fw_parent_device(motu.unit);
    __be32 data;
    int err;
    if (motu.async_handler.callback_data == core::ptr::null_mut())
    return -EINVAL;
// Register messaging address. Block transaction is not allowed.
    data = cpu_to_be32((device.card.node_id << 16) |
    (motu.async_handler.offset >> 32));
    err = snd_motu_transaction_write(motu, ASYNC_ADDR_HI, &data,
    sizeof(data));
    if (err < 0)
    return err;
    data = cpu_to_be32(motu.async_handler.offset);
    return snd_motu_transaction_write(motu, ASYNC_ADDR_LO, &data,
    sizeof(data));
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_transaction_register(motu: *mut snd_motu) -> c_int {
    int snd_motu_transaction_register(struct snd_motu *motu)
    {
    static const struct fw_address_region resp_register_region = {
    .start	= 0xffffe0000000ull,
    .end	= 0xffffe000ffffull,
    };
    int err;
// Perhaps, 4 byte messages are transferred.
    motu.async_handler.length = 4;
    motu.async_handler.address_callback = handle_message;
    motu.async_handler.callback_data = motu;
    err = fw_core_add_address_handler(&motu.async_handler,
    &resp_register_region);
    if (err < 0)
    return err;
    err = snd_motu_transaction_reregister(motu);
    if (err < 0) {
    fw_core_remove_address_handler(&motu.async_handler);
    motu.async_handler.address_callback = core::ptr::null_mut();
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_motu_transaction_unregister(motu: *mut snd_motu) {
    void snd_motu_transaction_unregister(struct snd_motu *motu)
    {
    __be32 data;
    if (motu.async_handler.address_callback != core::ptr::null_mut())
    fw_core_remove_address_handler(&motu.async_handler);
    motu.async_handler.address_callback = core::ptr::null_mut();
// Unregister the address.
    data = cpu_to_be32(0x00000000);
    snd_motu_transaction_write(motu, ASYNC_ADDR_HI, &data, sizeof(data));
    snd_motu_transaction_write(motu, ASYNC_ADDR_LO, &data, sizeof(data));
    }
