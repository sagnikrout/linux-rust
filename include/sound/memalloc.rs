//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/memalloc.h
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
//
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Takashi Iwai <tiwai@suse.de>
//
// Generic memory allocators
//

//
// buffer device info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dma_device {
    pub /: *mut *mut int type; / SNDRV_DMA_TYPE_XXX,
    pub /: *mut *mut dma_data_direction dir; / DMA direction,
    pub /: *mut *mut bool need_sync; / explicit sync needed?,
    pub /: *mut *mut *mut device dev; / generic device,
}

//
// buffer types
//

//
// info for buffer allocation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dma_buffer {
    pub /: *mut *mut snd_dma_device dev; / device type,
    pub /: *mut *mut *mut unsigned char area; / virtual pointer,
    pub /: *mut *mut dma_addr_t addr; / physical address,
    pub /: *mut *mut size_t bytes; / buffer size in bytes,
    pub /: *mut *mut *mut void private_data; / private for allocator; don't touch,
}

//
// return the pages matching with the given byte size
//
// allocate/release a buffer
extern "C" {
    pub fn snd_dma_alloc_dir_pages(_arg: type, _arg: dev, _arg: DMA_BIDIRECTIONAL, _arg: size, _arg: dmab) -> return;
}
extern "C" {
    pub fn snd_dma_free_pages(dmab: *mut snd_dma_buffer);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_dma_sync_mode {

    void snd_dma_buffer_sync(struct snd_dma_buffer *dmab,
    enum snd_dma_sync_mode mode);

    static inline void snd_dma_buffer_sync(struct snd_dma_buffer *dmab,
    enum snd_dma_sync_mode mode) {}

    dma_addr_t snd_sgbuf_get_addr(struct snd_dma_buffer *dmab, size_t offset);
    struct page *snd_sgbuf_get_page(struct snd_dma_buffer *dmab, size_t offset);
    unsigned int snd_sgbuf_get_chunk_size(struct snd_dma_buffer *dmab,
    unsigned int ofs, unsigned int size);

// device-managed memory allocator
    struct snd_dma_buffer *snd_devm_alloc_dir_pages(struct device *dev, int type,
    enum dma_data_direction dir,
    size_t size);

    static inline struct snd_dma_buffer *
    snd_devm_alloc_pages(struct device *dev, int type, size_t size)
    {
    return snd_devm_alloc_dir_pages(dev, type, DMA_BIDIRECTIONAL, size);
    }

    static inline struct sg_table *
    snd_dma_noncontig_sg_table(struct snd_dma_buffer *dmab)
    {
    return dmab->private_data;
    }
