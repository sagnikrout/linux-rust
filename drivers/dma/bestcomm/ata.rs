//! Automatically rewritten from C to Rust
//! Source: drivers/dma/bestcomm/ata.c
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
// Bestcomm ATA task driver
//
// Patterned after bestcomm/fec.c by Dale Farnsworth <dfarnsworth@mvista.com>
// 2003-2004 (c) MontaVista, Software, Inc.
//
// Copyright (C) 2006-2007 Sylvain Munaut <tnt@246tNt.com>
// Copyright (C) 2006      Freescale - John Rigby
//

// ========================================================================
// Task image/var/inc
// ========================================================================
// ata task image
    extern u32 bcom_ata_task[];
// ata task vars that need to be set before enabling the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_ata_var {
    pub /: *mut *mut *mut u32 enable; / (u16) address of task's control register,
    pub /: *mut *mut *mut u32 bd_base; / (struct bcom_bd) beginning of ring buffer,
    pub /: *mut *mut *mut u32 bd_last; / (struct bcom_bd) end of ring buffer,
    pub /: *mut *mut *mut u32 bd_start; / (struct bcom_bd) current bd,
    pub /: *mut *mut u32 buffer_size; / size of receive buffer,
}

// ata task incs that need to be set before enabling the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_ata_inc {
    pub pad0: u16,
    pub incr_bytes: i16,
    pub pad1: u16,
    pub incr_dst: i16,
    pub pad2: u16,
    pub incr_src: i16,
}

// ========================================================================
// Task support code
// ========================================================================
    struct bcom_task *
    bcom_ata_init(int queue_len, int maxbufsize)
    {
    struct bcom_task *tsk;
    struct bcom_ata_var *var;
    struct bcom_ata_inc *inc;
// Prefetch breaks ATA DMA.  Turn it off for ATA DMA
    bcom_disable_prefetch();
    tsk = bcom_task_alloc(queue_len, sizeof(struct bcom_ata_bd), 0);
    if (!tsk)
    return core::ptr::null_mut();
    tsk.flags = BCOM_FLAGS_NONE;
    bcom_ata_reset_bd(tsk);
    var = (struct bcom_ata_var *) bcom_task_var(tsk.tasknum);
    inc = (struct bcom_ata_inc *) bcom_task_inc(tsk.tasknum);
    if (bcom_load_image(tsk.tasknum, bcom_ata_task)) {
    bcom_task_free(tsk);
    return core::ptr::null_mut();
    }
    var.enable	= bcom_eng.regs_base +
    offsetof(struct mpc52xx_sdma, tcr[tsk.tasknum]);
    var.bd_base	= tsk.bd_pa;
    var.bd_last	= tsk.bd_pa + ((tsk.num_bd-1) * tsk.bd_size);
    var.bd_start	= tsk.bd_pa;
    var.buffer_size = maxbufsize;
// Configure some stuff
    bcom_set_task_pragma(tsk.tasknum, BCOM_ATA_PRAGMA);
    bcom_set_task_auto_start(tsk.tasknum, tsk.tasknum);
    out_8(&bcom_eng.regs.ipr[BCOM_INITIATOR_ATA_RX], BCOM_IPR_ATA_RX);
    out_8(&bcom_eng.regs.ipr[BCOM_INITIATOR_ATA_TX], BCOM_IPR_ATA_TX);
    out_be32(&bcom_eng.regs.IntPend, 1<<tsk.tasknum); /* Clear ints */
    return tsk;
    }
    EXPORT_SYMBOL_GPL(bcom_ata_init);
#[no_mangle]
pub unsafe extern "C" fn bcom_ata_rx_prepare(tsk: *mut bcom_task) {
    void bcom_ata_rx_prepare(struct bcom_task *tsk)
    {
    struct bcom_ata_inc *inc;
    inc = (struct bcom_ata_inc *) bcom_task_inc(tsk.tasknum);
    inc.incr_bytes	= -(s16)sizeof(u32);
    inc.incr_src	= 0;
    inc.incr_dst	= sizeof(u32);
    bcom_set_initiator(tsk.tasknum, BCOM_INITIATOR_ATA_RX);
    }
    EXPORT_SYMBOL_GPL(bcom_ata_rx_prepare);
#[no_mangle]
pub unsafe extern "C" fn bcom_ata_tx_prepare(tsk: *mut bcom_task) {
    void bcom_ata_tx_prepare(struct bcom_task *tsk)
    {
    struct bcom_ata_inc *inc;
    inc = (struct bcom_ata_inc *) bcom_task_inc(tsk.tasknum);
    inc.incr_bytes	= -(s16)sizeof(u32);
    inc.incr_src	= sizeof(u32);
    inc.incr_dst	= 0;
    bcom_set_initiator(tsk.tasknum, BCOM_INITIATOR_ATA_TX);
    }
    EXPORT_SYMBOL_GPL(bcom_ata_tx_prepare);
#[no_mangle]
pub unsafe extern "C" fn bcom_ata_reset_bd(tsk: *mut bcom_task) {
    void bcom_ata_reset_bd(struct bcom_task *tsk)
    {
    struct bcom_ata_var *var;
// Reset all BD
    memset_io(tsk.bd, 0x00, tsk.num_bd * tsk.bd_size);
    tsk.index = 0;
    tsk.outdex = 0;
    var = (struct bcom_ata_var *) bcom_task_var(tsk.tasknum);
    var.bd_start = var.bd_base;
    }
    EXPORT_SYMBOL_GPL(bcom_ata_reset_bd);
#[no_mangle]
pub unsafe extern "C" fn bcom_ata_release(tsk: *mut bcom_task) {
    void bcom_ata_release(struct bcom_task *tsk)
    {
// Nothing special for the ATA tasks
    bcom_task_free(tsk);
    }
    EXPORT_SYMBOL_GPL(bcom_ata_release);
    MODULE_DESCRIPTION("BestComm ATA task driver");
    MODULE_AUTHOR("John Rigby");
    MODULE_LICENSE("GPL v2");
