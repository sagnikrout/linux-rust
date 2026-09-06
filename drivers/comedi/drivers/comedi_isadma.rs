//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/comedi_isadma.c
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
// COMEDI ISA DMA support functions
// Copyright (c) 2014 H Hartley Sweeten <hsweeten@visionengravers.com>
//

//
// comedi_isadma_program - program and enable an ISA DMA transfer
// @desc:	the ISA DMA cookie to program and enable
//
#[no_mangle]
pub unsafe extern "C" fn comedi_isadma_program(desc: *mut comedi_isadma_desc) {
    void comedi_isadma_program(struct comedi_isadma_desc *desc)
    {
    unsigned long flags;
    flags = claim_dma_lock();
    clear_dma_ff(desc.chan);
    set_dma_mode(desc.chan, desc.mode);
    set_dma_addr(desc.chan, desc.hw_addr);
    set_dma_count(desc.chan, desc.size);
    enable_dma(desc.chan);
    release_dma_lock(flags);
    }
    EXPORT_SYMBOL_GPL(comedi_isadma_program);
//
// comedi_isadma_disable - disable the ISA DMA channel
// @dma_chan:	the DMA channel to disable
//
// Returns the residue (remaining bytes) left in the DMA transfer.
//
#[no_mangle]
pub unsafe extern "C" fn comedi_isadma_disable(dma_chan: c_uint) -> c_uint {
    unsigned int comedi_isadma_disable(unsigned int dma_chan)
    {
    unsigned long flags;
    unsigned int residue;
    flags = claim_dma_lock();
    disable_dma(dma_chan);
    residue = get_dma_residue(dma_chan);
    release_dma_lock(flags);
    return residue;
    }
    EXPORT_SYMBOL_GPL(comedi_isadma_disable);
//
// comedi_isadma_disable_on_sample - disable the ISA DMA channel
// @dma_chan:	the DMA channel to disable
// @size:	the sample size (in bytes)
//
// Returns the residue (remaining bytes) left in the DMA transfer.
//
    unsigned int comedi_isadma_disable_on_sample(unsigned int dma_chan,
    unsigned int size)
    {
    let mut stalled: c_int = 0;
    unsigned long flags;
    unsigned int residue;
    unsigned int new_residue;
    residue = comedi_isadma_disable(dma_chan);
    while (residue % size) {
// residue is a partial sample, enable DMA to allow more data
    flags = claim_dma_lock();
    enable_dma(dma_chan);
    release_dma_lock(flags);
    udelay(2);
    new_residue = comedi_isadma_disable(dma_chan);
// is DMA stalled?
    if (new_residue == residue) {
    stalled++;
    if (stalled > 10)
    break;
    } else {
    residue = new_residue;
    stalled = 0;
    }
    }
    return residue;
    }
    EXPORT_SYMBOL_GPL(comedi_isadma_disable_on_sample);
//
// comedi_isadma_poll - poll the current DMA transfer
// @dma:	the ISA DMA to poll
//
// Returns the position (in bytes) of the current DMA transfer.
//
#[no_mangle]
pub unsafe extern "C" fn comedi_isadma_poll(dma: *mut comedi_isadma) -> c_uint {
    unsigned int comedi_isadma_poll(struct comedi_isadma *dma)
    {
    struct comedi_isadma_desc *desc = &dma.desc[dma.cur_dma];
    unsigned long flags;
    unsigned int result;
    unsigned int result1;
    flags = claim_dma_lock();
    clear_dma_ff(desc.chan);
    if (!isa_dma_bridge_buggy)
    disable_dma(desc.chan);
    result = get_dma_residue(desc.chan);
//
// Read the counter again and choose higher value in order to
// avoid reading during counter lower byte roll over if the
// isa_dma_bridge_buggy is set.
//
    result1 = get_dma_residue(desc.chan);
    if (!isa_dma_bridge_buggy)
    enable_dma(desc.chan);
    release_dma_lock(flags);
    if (result < result1)
    result = result1;
    if (result >= desc.size || result == 0)
    return 0;
    return desc.size - result;
    }
    EXPORT_SYMBOL_GPL(comedi_isadma_poll);
//
// comedi_isadma_set_mode - set the ISA DMA transfer direction
// @desc:	the ISA DMA cookie to set
// @dma_dir:	the DMA direction
//
#[no_mangle]
pub unsafe extern "C" fn comedi_isadma_set_mode(desc: *mut comedi_isadma_desc, dma_dir: c_char) {
    void comedi_isadma_set_mode(struct comedi_isadma_desc *desc, char dma_dir)
    {
    desc.mode = (dma_dir == COMEDI_ISADMA_READ) ? DMA_MODE_READ
    : DMA_MODE_WRITE;
    }
    EXPORT_SYMBOL_GPL(comedi_isadma_set_mode);
//
// comedi_isadma_alloc - allocate and initialize the ISA DMA
// @dev:	comedi_device struct
// @n_desc:	the number of cookies to allocate
// @dma_chan1:	DMA channel for the first cookie
// @dma_chan2:	DMA channel for the second cookie
// @maxsize:	the size of the buffer to allocate for each cookie
// @dma_dir:	the DMA direction
//
// Returns the allocated and initialized ISA DMA or NULL if anything fails.
//
    struct comedi_isadma *comedi_isadma_alloc(struct comedi_device *dev,
    int n_desc, unsigned int dma_chan1,
    unsigned int dma_chan2,
    unsigned int maxsize, char dma_dir)
    {
    struct comedi_isadma *dma = core::ptr::null_mut();
    struct comedi_isadma_desc *desc;
    unsigned int dma_chans[2];
    int i;
    if (n_desc < 1 || n_desc > 2)
    goto no_dma;
    dma = kzalloc_flex(*dma, desc, n_desc);
    if (!dma)
    goto no_dma;
    dma.n_desc = n_desc;
    if (dev.hw_dev) {
    dma.dev = dev.hw_dev;
    } else {
// Fall back to using the "class" device.
    if (!dev.class_dev)
    goto no_dma;
// Need 24-bit mask for ISA DMA.
    if (dma_coerce_mask_and_coherent(dev.class_dev,
    DMA_BIT_MASK(24))) {
    goto no_dma;
    }
    dma.dev = dev.class_dev;
    }
    dma_chans[0] = dma_chan1;
    if (dma_chan2 == 0 || dma_chan2 == dma_chan1)
    dma_chans[1] = dma_chan1;
    else
    dma_chans[1] = dma_chan2;
    if (request_dma(dma_chans[0], dev.board_name))
    goto no_dma;
    dma.chan = dma_chans[0];
    if (dma_chans[1] != dma_chans[0]) {
    if (request_dma(dma_chans[1], dev.board_name))
    goto no_dma;
    }
    dma.chan2 = dma_chans[1];
    for (i = 0; i < n_desc; i++) {
    desc = &dma.desc[i];
    desc.chan = dma_chans[i];
    desc.maxsize = maxsize;
    desc.virt_addr = dma_alloc_coherent(dma.dev, desc.maxsize,
    &desc.hw_addr,
    GFP_KERNEL);
    if (!desc.virt_addr)
    goto no_dma;
    comedi_isadma_set_mode(desc, dma_dir);
    }
    return dma;
    no_dma:
    comedi_isadma_free(dma);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(comedi_isadma_alloc);
//
// comedi_isadma_free - free the ISA DMA
// @dma:	the ISA DMA to free
//
#[no_mangle]
pub unsafe extern "C" fn comedi_isadma_free(dma: *mut comedi_isadma) {
    void comedi_isadma_free(struct comedi_isadma *dma)
    {
    struct comedi_isadma_desc *desc;
    int i;
    if (!dma)
    return;
    for (i = 0; i < dma.n_desc; i++) {
    desc = &dma.desc[i];
    if (desc.virt_addr)
    dma_free_coherent(dma.dev, desc.maxsize,
    desc.virt_addr,
    desc.hw_addr);
    }
    if (dma.chan2 && dma.chan2 != dma.chan)
    free_dma(dma.chan2);
    if (dma.chan)
    free_dma(dma.chan);
    kfree(dma);
    }
    EXPORT_SYMBOL_GPL(comedi_isadma_free);
    MODULE_AUTHOR("H Hartley Sweeten <hsweeten@visionengravers.com>");
    MODULE_DESCRIPTION("Comedi ISA DMA support");
    MODULE_LICENSE("GPL");
