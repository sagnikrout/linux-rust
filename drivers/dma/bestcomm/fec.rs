//! Automatically rewritten from C to Rust
//! Source: drivers/dma/bestcomm/fec.c
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
// Bestcomm FEC tasks driver
//
// Copyright (C) 2006-2007 Sylvain Munaut <tnt@246tNt.com>
// Copyright (C) 2003-2004 MontaVista, Software, Inc.
// ( by Dale Farnsworth <dfarnsworth@mvista.com> )
//

// ========================================================================
// Task image/var/inc
// ========================================================================
// fec tasks images
    extern u32 bcom_fec_rx_task[];
    extern u32 bcom_fec_tx_task[];
// rx task vars that need to be set before enabling the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_fec_rx_var {
    pub /: *mut *mut *mut u32 enable; / (u16) address of task's control register,
    pub /: *mut *mut *mut u32 fifo; / (u32) address of fec's fifo,
    pub /: *mut *mut *mut u32 bd_base; / (struct bcom_bd) beginning of ring buffer,
    pub /: *mut *mut *mut u32 bd_last; / (struct bcom_bd) end of ring buffer,
    pub /: *mut *mut *mut u32 bd_start; / (struct bcom_bd) current bd,
    pub /: *mut *mut u32 buffer_size; / size of receive buffer,
}

// rx task incs that need to be set before enabling the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_fec_rx_inc {
    pub pad0: u16,
    pub incr_bytes: i16,
    pub pad1: u16,
    pub incr_dst: i16,
    pub pad2: u16,
    pub incr_dst_ma: i16,
}

// tx task vars that need to be set before enabling the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_fec_tx_var {
    pub /: *mut *mut *mut u32 DRD; / (u32) address of self-modified DRD,
    pub /: *mut *mut *mut u32 fifo; / (u32) address of fec's fifo,
    pub /: *mut *mut *mut u32 enable; / (u16) address of task's control register,
    pub /: *mut *mut *mut u32 bd_base; / (struct bcom_bd) beginning of ring buffer,
    pub /: *mut *mut *mut u32 bd_last; / (struct bcom_bd) end of ring buffer,
    pub /: *mut *mut *mut u32 bd_start; / (struct bcom_bd) current bd,
    pub /: *mut *mut u32 buffer_size; / set by uCode for each packet,
}

// tx task incs that need to be set before enabling the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_fec_tx_inc {
    pub pad0: u16,
    pub incr_bytes: i16,
    pub pad1: u16,
    pub incr_src: i16,
    pub pad2: u16,
    pub incr_src_ma: i16,
}

// private structure in the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_fec_priv {
    pub fifo: phys_addr_t,
    pub maxbufsize: c_int,
}

// ========================================================================
// Task support code
// ========================================================================
    struct bcom_task *
    bcom_fec_rx_init(int queue_len, phys_addr_t fifo, int maxbufsize)
    {
    struct bcom_task *tsk;
    struct bcom_fec_priv *priv;
    tsk = bcom_task_alloc(queue_len, sizeof(struct bcom_fec_bd),
    sizeof(struct bcom_fec_priv));
    if (!tsk)
    return core::ptr::null_mut();
    tsk.flags = BCOM_FLAGS_NONE;
    priv = tsk.priv;
    priv.fifo = fifo;
    priv.maxbufsize = maxbufsize;
    if (bcom_fec_rx_reset(tsk)) {
    bcom_task_free(tsk);
    return core::ptr::null_mut();
    }
    return tsk;
    }
    EXPORT_SYMBOL_GPL(bcom_fec_rx_init);
    int
    bcom_fec_rx_reset(struct bcom_task *tsk)
    {
    struct bcom_fec_priv *priv = tsk.priv;
    struct bcom_fec_rx_var *var;
    struct bcom_fec_rx_inc *inc;
// Shutdown the task
    bcom_disable_task(tsk.tasknum);
// Reset the microcode
    var = (struct bcom_fec_rx_var *) bcom_task_var(tsk.tasknum);
    inc = (struct bcom_fec_rx_inc *) bcom_task_inc(tsk.tasknum);
    if (bcom_load_image(tsk.tasknum, bcom_fec_rx_task))
    return -1;
    var.enable	= bcom_eng.regs_base +
    offsetof(struct mpc52xx_sdma, tcr[tsk.tasknum]);
    var.fifo	= (u32) priv.fifo;
    var.bd_base	= tsk.bd_pa;
    var.bd_last	= tsk.bd_pa + ((tsk.num_bd-1) * tsk.bd_size);
    var.bd_start	= tsk.bd_pa;
    var.buffer_size = priv.maxbufsize;
    inc.incr_bytes	= -(s16)sizeof(u32);	/* These should be in the   */
    inc.incr_dst	= sizeof(u32);		/* task image, but we stick */
    inc.incr_dst_ma= sizeof(u8);		/* to the official ones     */
// Reset the BDs
    tsk.index = 0;
    tsk.outdex = 0;
    memset_io(tsk.bd, 0x00, tsk.num_bd * tsk.bd_size);
// Configure some stuff
    bcom_set_task_pragma(tsk.tasknum, BCOM_FEC_RX_BD_PRAGMA);
    bcom_set_task_auto_start(tsk.tasknum, tsk.tasknum);
    out_8(&bcom_eng.regs.ipr[BCOM_INITIATOR_FEC_RX], BCOM_IPR_FEC_RX);
    out_be32(&bcom_eng.regs.IntPend, 1<<tsk.tasknum);	/* Clear ints */
    return 0;
    }
    EXPORT_SYMBOL_GPL(bcom_fec_rx_reset);
    void
    bcom_fec_rx_release(struct bcom_task *tsk)
    {
// Nothing special for the FEC tasks
    bcom_task_free(tsk);
    }
    EXPORT_SYMBOL_GPL(bcom_fec_rx_release);
// Return 2nd to last DRD
// This is an ugly hack, but at least it's only done
    once at initialization */
    static u32 *self_modified_drd(int tasknum)
    {
    u32 *desc;
    int num_descs;
    int drd_count;
    int i;
    num_descs = bcom_task_num_descs(tasknum);
    desc = bcom_task_desc(tasknum) + num_descs - 1;
    drd_count = 0;
    for (i=0; i<num_descs; i++, desc--)
    if (bcom_desc_is_drd(*desc) && ++drd_count == 3)
    break;
    return desc;
    }
    struct bcom_task *
    bcom_fec_tx_init(int queue_len, phys_addr_t fifo)
    {
    struct bcom_task *tsk;
    struct bcom_fec_priv *priv;
    tsk = bcom_task_alloc(queue_len, sizeof(struct bcom_fec_bd),
    sizeof(struct bcom_fec_priv));
    if (!tsk)
    return core::ptr::null_mut();
    tsk.flags = BCOM_FLAGS_ENABLE_TASK;
    priv = tsk.priv;
    priv.fifo = fifo;
    if (bcom_fec_tx_reset(tsk)) {
    bcom_task_free(tsk);
    return core::ptr::null_mut();
    }
    return tsk;
    }
    EXPORT_SYMBOL_GPL(bcom_fec_tx_init);
    int
    bcom_fec_tx_reset(struct bcom_task *tsk)
    {
    struct bcom_fec_priv *priv = tsk.priv;
    struct bcom_fec_tx_var *var;
    struct bcom_fec_tx_inc *inc;
// Shutdown the task
    bcom_disable_task(tsk.tasknum);
// Reset the microcode
    var = (struct bcom_fec_tx_var *) bcom_task_var(tsk.tasknum);
    inc = (struct bcom_fec_tx_inc *) bcom_task_inc(tsk.tasknum);
    if (bcom_load_image(tsk.tasknum, bcom_fec_tx_task))
    return -1;
    var.enable	= bcom_eng.regs_base +
    offsetof(struct mpc52xx_sdma, tcr[tsk.tasknum]);
    var.fifo	= (u32) priv.fifo;
    var.DRD	= bcom_sram_va2pa(self_modified_drd(tsk.tasknum));
    var.bd_base	= tsk.bd_pa;
    var.bd_last	= tsk.bd_pa + ((tsk.num_bd-1) * tsk.bd_size);
    var.bd_start	= tsk.bd_pa;
    inc.incr_bytes	= -(s16)sizeof(u32);	/* These should be in the   */
    inc.incr_src	= sizeof(u32);		/* task image, but we stick */
    inc.incr_src_ma= sizeof(u8);		/* to the official ones     */
// Reset the BDs
    tsk.index = 0;
    tsk.outdex = 0;
    memset_io(tsk.bd, 0x00, tsk.num_bd * tsk.bd_size);
// Configure some stuff
    bcom_set_task_pragma(tsk.tasknum, BCOM_FEC_TX_BD_PRAGMA);
    bcom_set_task_auto_start(tsk.tasknum, tsk.tasknum);
    out_8(&bcom_eng.regs.ipr[BCOM_INITIATOR_FEC_TX], BCOM_IPR_FEC_TX);
    out_be32(&bcom_eng.regs.IntPend, 1<<tsk.tasknum);	/* Clear ints */
    return 0;
    }
    EXPORT_SYMBOL_GPL(bcom_fec_tx_reset);
    void
    bcom_fec_tx_release(struct bcom_task *tsk)
    {
// Nothing special for the FEC tasks
    bcom_task_free(tsk);
    }
    EXPORT_SYMBOL_GPL(bcom_fec_tx_release);
    MODULE_DESCRIPTION("BestComm FEC tasks driver");
    MODULE_AUTHOR("Dale Farnsworth <dfarnsworth@mvista.com>");
    MODULE_LICENSE("GPL v2");
