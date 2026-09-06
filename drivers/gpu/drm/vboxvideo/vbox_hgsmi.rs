//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vboxvideo/vbox_hgsmi.c
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


// SPDX-License-Identifier: MIT
//
// Copyright (C) 2017 Oracle Corporation
// Authors: Hans de Goede <hdegoede@redhat.com>
//

// One-at-a-Time Hash from https://www.burtleburtle.net/bob/hash/doobs.html
#[no_mangle]
unsafe extern "C" fn hgsmi_hash_process(hash: u32, data: *const u8, size: c_int) -> u32 {
    static u32 hgsmi_hash_process(u32 hash, const u8 *data, int size)
    {
    while (size--) {
    hash += *data++;
    hash += (hash << 10);
    hash ^= (hash >> 6);
    }
    return hash;
    }
#[no_mangle]
unsafe extern "C" fn hgsmi_hash_end(hash: u32) -> u32 {
    static u32 hgsmi_hash_end(u32 hash)
    {
    hash += (hash << 3);
    hash ^= (hash >> 11);
    hash += (hash << 15);
    return hash;
    }
// Not really a checksum but that is the naming used in all vbox code
    static u32 hgsmi_checksum(u32 offset,
    const struct hgsmi_buffer_header *header,
    const struct hgsmi_buffer_tail *tail)
    {
    u32 checksum;
    checksum = hgsmi_hash_process(0, (u8 *)&offset, sizeof(offset));
    checksum = hgsmi_hash_process(checksum, (u8 *)header, sizeof(*header));
// 4 -> Do not checksum the checksum itself
    checksum = hgsmi_hash_process(checksum, (u8 *)tail, 4);
    return hgsmi_hash_end(checksum);
    }
    void *hgsmi_buffer_alloc(struct gen_pool *guest_pool, size_t size,
    u8 channel, u16 channel_info)
    {
    struct hgsmi_buffer_header *h;
    struct hgsmi_buffer_tail *t;
    size_t total_size;
    dma_addr_t offset;
    total_size = size + sizeof(*h) + sizeof(*t);
    h = gen_pool_dma_alloc(guest_pool, total_size, &offset);
    if (!h)
    return core::ptr::null_mut();
    t = (struct hgsmi_buffer_tail *)((u8 *)h + sizeof(*h) + size);
    h.flags = HGSMI_BUFFER_HEADER_F_SEQ_SINGLE;
    h.data_size = size;
    h.channel = channel;
    h.channel_info = channel_info;
    memset(&h.u.header_data, 0, sizeof(h.u.header_data));
    t.reserved = 0;
    t.checksum = hgsmi_checksum(offset, h, t);
    return (u8 *)h + sizeof(*h);
    }
#[no_mangle]
pub unsafe extern "C" fn hgsmi_buffer_free(guest_pool: *mut gen_pool, buf: *mut c_void) {
    void hgsmi_buffer_free(struct gen_pool *guest_pool, void *buf)
    {
    struct hgsmi_buffer_header *h =
    (struct hgsmi_buffer_header *)((u8 *)buf - sizeof(*h));
    size_t total_size = h.data_size + sizeof(*h) +
    sizeof(struct hgsmi_buffer_tail);
    gen_pool_free(guest_pool, (unsigned long)h, total_size);
    }
#[no_mangle]
pub unsafe extern "C" fn hgsmi_buffer_submit(guest_pool: *mut gen_pool, buf: *mut c_void) -> c_int {
    int hgsmi_buffer_submit(struct gen_pool *guest_pool, void *buf)
    {
    phys_addr_t offset;
    offset = gen_pool_virt_to_phys(guest_pool, (unsigned long)buf -
    sizeof(struct hgsmi_buffer_header));
    outl(offset, VGA_PORT_HGSMI_GUEST);
// Make the compiler aware that the host has changed memory.
    mb();
    return 0;
    }
