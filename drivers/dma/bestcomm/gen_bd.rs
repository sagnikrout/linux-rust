//! Automatically rewritten from C to Rust
//! Source: drivers/dma/bestcomm/gen_bd.c
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
// Driver for MPC52xx processor BestComm General Buffer Descriptor
//
// Copyright (C) 2007 Sylvain Munaut <tnt@246tNt.com>
// Copyright (C) 2006 AppSpec Computer Technologies Corp.
// Jeff Gibbons <jeff.gibbons@appspec.com>
//

// ========================================================================
// Task image/var/inc
// ========================================================================
// gen_bd tasks images
    extern u32 bcom_gen_bd_rx_task[];
    extern u32 bcom_gen_bd_tx_task[];
// rx task vars that need to be set before enabling the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_gen_bd_rx_var {
    pub /: *mut *mut *mut u32 enable; / (u16) address of task's control register,
    pub /: *mut *mut *mut u32 fifo; / (u32) address of gen_bd's fifo,
    pub /: *mut *mut *mut u32 bd_base; / (struct bcom_bd) beginning of ring buffer,
    pub /: *mut *mut *mut u32 bd_last; / (struct bcom_bd) end of ring buffer,
    pub /: *mut *mut *mut u32 bd_start; / (struct bcom_bd) current bd,
    pub /: *mut *mut u32 buffer_size; / size of receive buffer,
}

// rx task incs that need to be set before enabling the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_gen_bd_rx_inc {
    pub pad0: u16,
    pub incr_bytes: i16,
    pub pad1: u16,
    pub incr_dst: i16,
}

// tx task vars that need to be set before enabling the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_gen_bd_tx_var {
    pub /: *mut *mut *mut u32 fifo; / (u32) address of gen_bd's fifo,
    pub /: *mut *mut *mut u32 enable; / (u16) address of task's control register,
    pub /: *mut *mut *mut u32 bd_base; / (struct bcom_bd) beginning of ring buffer,
    pub /: *mut *mut *mut u32 bd_last; / (struct bcom_bd) end of ring buffer,
    pub /: *mut *mut *mut u32 bd_start; / (struct bcom_bd) current bd,
    pub /: *mut *mut u32 buffer_size; / set by uCode for each packet,
}

// tx task incs that need to be set before enabling the task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_gen_bd_tx_inc {
    pub pad0: u16,
    pub incr_bytes: i16,
    pub pad1: u16,
    pub incr_src: i16,
    pub pad2: u16,
    pub incr_src_ma: i16,
}

// private structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_gen_bd_priv {
    pub fifo: phys_addr_t,
    pub initiator: c_int,
    pub ipr: c_int,
    pub maxbufsize: c_int,
}

// ========================================================================
// Task support code
// ========================================================================
    struct bcom_task *
    bcom_gen_bd_rx_init(int queue_len, phys_addr_t fifo,
    int initiator, int ipr, int maxbufsize)
    {
    struct bcom_task *tsk;
    struct bcom_gen_bd_priv *priv;
    tsk = bcom_task_alloc(queue_len, sizeof(struct bcom_gen_bd),
    sizeof(struct bcom_gen_bd_priv));
    if (!tsk)
    return core::ptr::null_mut();
    tsk.flags = BCOM_FLAGS_NONE;
    priv = tsk.priv;
    priv.fifo	= fifo;
    priv.initiator	= initiator;
    priv.ipr	= ipr;
    priv.maxbufsize = maxbufsize;
    if (bcom_gen_bd_rx_reset(tsk)) {
    bcom_task_free(tsk);
    return core::ptr::null_mut();
    }
    return tsk;
    }
    EXPORT_SYMBOL_GPL(bcom_gen_bd_rx_init);
    int
    bcom_gen_bd_rx_reset(struct bcom_task *tsk)
    {
    struct bcom_gen_bd_priv *priv = tsk.priv;
    struct bcom_gen_bd_rx_var *var;
    struct bcom_gen_bd_rx_inc *inc;
// Shutdown the task
    bcom_disable_task(tsk.tasknum);
// Reset the microcode
    var = (struct bcom_gen_bd_rx_var *) bcom_task_var(tsk.tasknum);
    inc = (struct bcom_gen_bd_rx_inc *) bcom_task_inc(tsk.tasknum);
    if (bcom_load_image(tsk.tasknum, bcom_gen_bd_rx_task))
    return -1;
    var.enable	= bcom_eng.regs_base +
    offsetof(struct mpc52xx_sdma, tcr[tsk.tasknum]);
    var.fifo	= (u32) priv.fifo;
    var.bd_base	= tsk.bd_pa;
    var.bd_last	= tsk.bd_pa + ((tsk.num_bd-1) * tsk.bd_size);
    var.bd_start	= tsk.bd_pa;
    var.buffer_size = priv.maxbufsize;
    inc.incr_bytes	= -(s16)sizeof(u32);
    inc.incr_dst	= sizeof(u32);
// Reset the BDs
    tsk.index = 0;
    tsk.outdex = 0;
    memset_io(tsk.bd, 0x00, tsk.num_bd * tsk.bd_size);
// Configure some stuff
    bcom_set_task_pragma(tsk.tasknum, BCOM_GEN_RX_BD_PRAGMA);
    bcom_set_task_auto_start(tsk.tasknum, tsk.tasknum);
    out_8(&bcom_eng.regs.ipr[priv.initiator], priv.ipr);
    bcom_set_initiator(tsk.tasknum, priv.initiator);
    out_be32(&bcom_eng.regs.IntPend, 1<<tsk.tasknum);	/* Clear ints */
    return 0;
    }
    EXPORT_SYMBOL_GPL(bcom_gen_bd_rx_reset);
    void
    bcom_gen_bd_rx_release(struct bcom_task *tsk)
    {
// Nothing special for the GenBD tasks
    bcom_task_free(tsk);
    }
    EXPORT_SYMBOL_GPL(bcom_gen_bd_rx_release);
    extern struct bcom_task *
    bcom_gen_bd_tx_init(int queue_len, phys_addr_t fifo,
    int initiator, int ipr)
    {
    struct bcom_task *tsk;
    struct bcom_gen_bd_priv *priv;
    tsk = bcom_task_alloc(queue_len, sizeof(struct bcom_gen_bd),
    sizeof(struct bcom_gen_bd_priv));
    if (!tsk)
    return core::ptr::null_mut();
    tsk.flags = BCOM_FLAGS_NONE;
    priv = tsk.priv;
    priv.fifo	= fifo;
    priv.initiator	= initiator;
    priv.ipr	= ipr;
    if (bcom_gen_bd_tx_reset(tsk)) {
    bcom_task_free(tsk);
    return core::ptr::null_mut();
    }
    return tsk;
    }
    EXPORT_SYMBOL_GPL(bcom_gen_bd_tx_init);
    int
    bcom_gen_bd_tx_reset(struct bcom_task *tsk)
    {
    struct bcom_gen_bd_priv *priv = tsk.priv;
    struct bcom_gen_bd_tx_var *var;
    struct bcom_gen_bd_tx_inc *inc;
// Shutdown the task
    bcom_disable_task(tsk.tasknum);
// Reset the microcode
    var = (struct bcom_gen_bd_tx_var *) bcom_task_var(tsk.tasknum);
    inc = (struct bcom_gen_bd_tx_inc *) bcom_task_inc(tsk.tasknum);
    if (bcom_load_image(tsk.tasknum, bcom_gen_bd_tx_task))
    return -1;
    var.enable	= bcom_eng.regs_base +
    offsetof(struct mpc52xx_sdma, tcr[tsk.tasknum]);
    var.fifo	= (u32) priv.fifo;
    var.bd_base	= tsk.bd_pa;
    var.bd_last	= tsk.bd_pa + ((tsk.num_bd-1) * tsk.bd_size);
    var.bd_start	= tsk.bd_pa;
    inc.incr_bytes	= -(s16)sizeof(u32);
    inc.incr_src	= sizeof(u32);
    inc.incr_src_ma = sizeof(u8);
// Reset the BDs
    tsk.index = 0;
    tsk.outdex = 0;
    memset_io(tsk.bd, 0x00, tsk.num_bd * tsk.bd_size);
// Configure some stuff
    bcom_set_task_pragma(tsk.tasknum, BCOM_GEN_TX_BD_PRAGMA);
    bcom_set_task_auto_start(tsk.tasknum, tsk.tasknum);
    out_8(&bcom_eng.regs.ipr[priv.initiator], priv.ipr);
    bcom_set_initiator(tsk.tasknum, priv.initiator);
    out_be32(&bcom_eng.regs.IntPend, 1<<tsk.tasknum);	/* Clear ints */
    return 0;
    }
    EXPORT_SYMBOL_GPL(bcom_gen_bd_tx_reset);
    void
    bcom_gen_bd_tx_release(struct bcom_task *tsk)
    {
// Nothing special for the GenBD tasks
    bcom_task_free(tsk);
    }
    EXPORT_SYMBOL_GPL(bcom_gen_bd_tx_release);
// ---------------------------------------------------------------------
// PSC support code
//
// bcom_psc_parameters - Bestcomm initialization value table for PSC devices
//
// This structure is only used internally.  It is a lookup table for PSC
// specific parameters to bestcomm tasks.
//
    static struct bcom_psc_params {
    int rx_initiator;
    int rx_ipr;
    int tx_initiator;
    int tx_ipr;
    } bcom_psc_params[] = {
    [0] = {
    .rx_initiator = BCOM_INITIATOR_PSC1_RX,
    .rx_ipr = BCOM_IPR_PSC1_RX,
    .tx_initiator = BCOM_INITIATOR_PSC1_TX,
    .tx_ipr = BCOM_IPR_PSC1_TX,
    },
    [1] = {
    .rx_initiator = BCOM_INITIATOR_PSC2_RX,
    .rx_ipr = BCOM_IPR_PSC2_RX,
    .tx_initiator = BCOM_INITIATOR_PSC2_TX,
    .tx_ipr = BCOM_IPR_PSC2_TX,
    },
    [2] = {
    .rx_initiator = BCOM_INITIATOR_PSC3_RX,
    .rx_ipr = BCOM_IPR_PSC3_RX,
    .tx_initiator = BCOM_INITIATOR_PSC3_TX,
    .tx_ipr = BCOM_IPR_PSC3_TX,
    },
    [3] = {
    .rx_initiator = BCOM_INITIATOR_PSC4_RX,
    .rx_ipr = BCOM_IPR_PSC4_RX,
    .tx_initiator = BCOM_INITIATOR_PSC4_TX,
    .tx_ipr = BCOM_IPR_PSC4_TX,
    },
    [4] = {
    .rx_initiator = BCOM_INITIATOR_PSC5_RX,
    .rx_ipr = BCOM_IPR_PSC5_RX,
    .tx_initiator = BCOM_INITIATOR_PSC5_TX,
    .tx_ipr = BCOM_IPR_PSC5_TX,
    },
    [5] = {
    .rx_initiator = BCOM_INITIATOR_PSC6_RX,
    .rx_ipr = BCOM_IPR_PSC6_RX,
    .tx_initiator = BCOM_INITIATOR_PSC6_TX,
    .tx_ipr = BCOM_IPR_PSC6_TX,
    },
    };
//
// bcom_psc_gen_bd_rx_init - Allocate a receive bcom_task for a PSC port
// @psc_num:	Number of the PSC to allocate a task for
// @queue_len:	number of buffer descriptors to allocate for the task
// @fifo:	physical address of FIFO register
// @maxbufsize:	Maximum receive data size in bytes.
//
// Allocate a bestcomm task structure for receiving data from a PSC.
//
    struct bcom_task * bcom_psc_gen_bd_rx_init(unsigned psc_num, int queue_len,
    phys_addr_t fifo, int maxbufsize)
    {
    if (psc_num >= MPC52xx_PSC_MAXNUM)
    return core::ptr::null_mut();
    return bcom_gen_bd_rx_init(queue_len, fifo,
    bcom_psc_params[psc_num].rx_initiator,
    bcom_psc_params[psc_num].rx_ipr,
    maxbufsize);
    }
    EXPORT_SYMBOL_GPL(bcom_psc_gen_bd_rx_init);
//
// bcom_psc_gen_bd_tx_init - Allocate a transmit bcom_task for a PSC port
// @psc_num:	Number of the PSC to allocate a task for
// @queue_len:	number of buffer descriptors to allocate for the task
// @fifo:	physical address of FIFO register
//
// Allocate a bestcomm task structure for transmitting data to a PSC.
//
    struct bcom_task *
    bcom_psc_gen_bd_tx_init(unsigned psc_num, int queue_len, phys_addr_t fifo)
    {
    struct psc;
    return bcom_gen_bd_tx_init(queue_len, fifo,
    bcom_psc_params[psc_num].tx_initiator,
    bcom_psc_params[psc_num].tx_ipr);
    }
    EXPORT_SYMBOL_GPL(bcom_psc_gen_bd_tx_init);
    MODULE_DESCRIPTION("BestComm General Buffer Descriptor tasks driver");
    MODULE_AUTHOR("Jeff Gibbons <jeff.gibbons@appspec.com>");
    MODULE_LICENSE("GPL v2");
