//! Automatically rewritten from C to Rust
//! Source: sound/core/isadma.c
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
// ISA DMA support functions
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//
// Defining following add some delay. Maybe this helps for some broken
// ISA DMA controllers.
//

//
// snd_dma_program - program an ISA DMA transfer
// @dma: the dma number
// @addr: the physical address of the buffer
// @size: the DMA transfer size
// @mode: the DMA transfer mode, DMA_MODE_XXX
//
// Programs an ISA DMA transfer for the given buffer.
//
    void snd_dma_program(unsigned long dma,
    unsigned long addr, unsigned int size,
    unsigned short mode)
    {
    unsigned long flags;
    flags = claim_dma_lock();
    disable_dma(dma);
    clear_dma_ff(dma);
    set_dma_mode(dma, mode);
    set_dma_addr(dma, addr);
    set_dma_count(dma, size);
    if (!(mode & DMA_MODE_NO_ENABLE))
    enable_dma(dma);
    release_dma_lock(flags);
    }
    EXPORT_SYMBOL(snd_dma_program);
//
// snd_dma_disable - stop the ISA DMA transfer
// @dma: the dma number
//
// Stops the ISA DMA transfer.
//
#[no_mangle]
pub unsafe extern "C" fn snd_dma_disable(dma: c_ulong) {
    void snd_dma_disable(unsigned long dma)
    {
    unsigned long flags;
    flags = claim_dma_lock();
    clear_dma_ff(dma);
    disable_dma(dma);
    release_dma_lock(flags);
    }
    EXPORT_SYMBOL(snd_dma_disable);
//
// snd_dma_pointer - return the current pointer to DMA transfer buffer in bytes
// @dma: the dma number
// @size: the dma transfer size
//
// Return: The current pointer in DMA transfer buffer in bytes.
//
#[no_mangle]
pub unsafe extern "C" fn snd_dma_pointer(dma: c_ulong, size: c_uint) -> c_uint {
    unsigned int snd_dma_pointer(unsigned long dma, unsigned int size)
    {
    unsigned long flags;
    unsigned int result, result1;
    flags = claim_dma_lock();
    clear_dma_ff(dma);
    if (!isa_dma_bridge_buggy)
    disable_dma(dma);
    result = get_dma_residue(dma);
//
// HACK - read the counter again and choose higher value in order to
// avoid reading during counter lower byte roll over if the
// isa_dma_bridge_buggy is set.
//
    result1 = get_dma_residue(dma);
    if (!isa_dma_bridge_buggy)
    enable_dma(dma);
    release_dma_lock(flags);
    if (unlikely(result < result1))
    result = result1;

    if (result > size)
    pr_err("ALSA: pointer (0x%x) for DMA #%ld is greater than transfer size (0x%x)\n", result, dma, size);

    if (result >= size || result == 0)
    return 0;
    else
    return size - result;
    }
    EXPORT_SYMBOL(snd_dma_pointer);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dma_data {
    pub dma: c_int,
}

#[no_mangle]
unsafe extern "C" fn __snd_release_dma(dev: *mut device, data: *mut c_void) {
    static void __snd_release_dma(struct device *dev, void *data)
    {
    struct snd_dma_data *p = data;
    snd_dma_disable(p.dma);
    free_dma(p.dma);
    }
//
// snd_devm_request_dma - the managed version of request_dma()
// @dev: the device pointer
// @dma: the dma number
// @name: the name string of the requester
//
// The requested DMA will be automatically released at unbinding via devres.
//
// Return: zero on success, or a negative error code
//
#[no_mangle]
pub unsafe extern "C" fn snd_devm_request_dma(dev: *mut device, dma: c_int, name: *const c_char) -> c_int {
    int snd_devm_request_dma(struct device *dev, int dma, const char *name)
    {
    struct snd_dma_data *p;
    if (request_dma(dma, name))
    return -EBUSY;
    p = devres_alloc(__snd_release_dma, sizeof(*p), GFP_KERNEL);
    if (!p) {
    free_dma(dma);
    return -ENOMEM;
    }
    p.dma = dma;
    devres_add(dev, p);
    return 0;
    }
    EXPORT_SYMBOL_GPL(snd_devm_request_dma);
