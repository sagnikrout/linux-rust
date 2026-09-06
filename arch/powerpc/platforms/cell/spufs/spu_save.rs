//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/cell/spufs/spu_save.c
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
// spu_save.c
//
// (C) Copyright IBM Corp. 2005
//
// SPU-side context save sequence outlined in
// Synergistic Processor Element Book IV
//
// Author: Mark Nutter <mnutter@us.ibm.com>
//

pub const LS_SIZE: c_uint = 0x40000	/* 256K (in bytes) */;

    typedef unsigned int u32;
    typedef unsigned long long u64;

#[no_mangle]
pub unsafe extern "C" fn save_event_mask() {
    static inline void save_event_mask(void)
    {
    unsigned int offset;
// Save, Step 2:
// Read the SPU_RdEventMsk channel and save to the LSCSA.
//
    offset = LSCSA_QW_OFFSET(event_mask);
    regs_spill[offset].slot[0] = spu_readch(SPU_RdEventMask);
    }
#[no_mangle]
pub unsafe extern "C" fn save_tag_mask() {
    static inline void save_tag_mask(void)
    {
    unsigned int offset;
// Save, Step 3:
// Read the SPU_RdTagMsk channel and save to the LSCSA.
//
    offset = LSCSA_QW_OFFSET(tag_mask);
    regs_spill[offset].slot[0] = spu_readch(MFC_RdTagMask);
    }
#[no_mangle]
pub unsafe extern "C" fn save_upper_240kb(lscsa_ea: addr64) {
    static inline void save_upper_240kb(addr64 lscsa_ea)
    {
    let mut ls: c_uint = 16384;
    let mut list: c_uint = (unsigned int)&dma_list[0];
    let mut size: c_uint = sizeof(dma_list);
    let mut tag_id: c_uint = 0;
    unsigned int cmd = 0x24;	/* PUTL */
// Save, Step 7:
// Enqueue the PUTL command (tag 0) to the MFC SPU command
// queue to transfer the remaining 240 kb of LS to CSA.
//
    spu_writech(MFC_LSA, ls);
    spu_writech(MFC_EAH, lscsa_ea.ui[0]);
    spu_writech(MFC_EAL, list);
    spu_writech(MFC_Size, size);
    spu_writech(MFC_TagID, tag_id);
    spu_writech(MFC_Cmd, cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn save_fpcr() {
    static inline void save_fpcr(void)
    {
// vector unsigned int fpcr;
    unsigned int offset;
// Save, Step 9:
// Issue the floating-point status and control register
// read instruction, and save to the LSCSA.
//
    offset = LSCSA_QW_OFFSET(fpcr);
    regs_spill[offset].v = spu_mffpscr();
    }
#[no_mangle]
pub unsafe extern "C" fn save_decr() {
    static inline void save_decr(void)
    {
    unsigned int offset;
// Save, Step 10:
// Read and save the SPU_RdDec channel data to
// the LSCSA.
//
    offset = LSCSA_QW_OFFSET(decr);
    regs_spill[offset].slot[0] = spu_readch(SPU_RdDec);
    }
#[no_mangle]
pub unsafe extern "C" fn save_srr0() {
    static inline void save_srr0(void)
    {
    unsigned int offset;
// Save, Step 11:
// Read and save the SPU_WSRR0 channel data to
// the LSCSA.
//
    offset = LSCSA_QW_OFFSET(srr0);
    regs_spill[offset].slot[0] = spu_readch(SPU_RdSRR0);
    }
#[no_mangle]
pub unsafe extern "C" fn spill_regs_to_mem(lscsa_ea: addr64) {
    static inline void spill_regs_to_mem(addr64 lscsa_ea)
    {
    let mut ls: c_uint = (unsigned int)&regs_spill[0];
    let mut size: c_uint = sizeof(regs_spill);
    let mut tag_id: c_uint = 0;
    unsigned int cmd = 0x20;	/* PUT */
// Save, Step 13:
// Enqueue a PUT command (tag 0) to send the LSCSA
// to the CSA.
//
    spu_writech(MFC_LSA, ls);
    spu_writech(MFC_EAH, lscsa_ea.ui[0]);
    spu_writech(MFC_EAL, lscsa_ea.ui[1]);
    spu_writech(MFC_Size, size);
    spu_writech(MFC_TagID, tag_id);
    spu_writech(MFC_Cmd, cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn enqueue_sync(lscsa_ea: addr64) {
    static inline void enqueue_sync(addr64 lscsa_ea)
    {
    let mut tag_id: c_uint = 0;
    let mut cmd: c_uint = 0xCC;
// Save, Step 14:
// Enqueue an MFC_SYNC command (tag 0).
//
    spu_writech(MFC_TagID, tag_id);
    spu_writech(MFC_Cmd, cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn save_complete() {
    static inline void save_complete(void)
    {
// Save, Step 18:
// Issue a stop-and-signal instruction indicating
// "save complete".  Note: This function will not
// return!!
//
    spu_stop(SPU_SAVE_COMPLETE);
    }
//
// main - entry point for SPU-side context save.
//
// This code deviates from the documented sequence as follows:
//
// 1. The EA for LSCSA is passed from PPE in the
// signal notification channels.
// 2. All 128 registers are saved by crt0.o.
//
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {
    addr64 lscsa_ea;
    lscsa_ea.ui[0] = spu_readch(SPU_RdSigNotify1);
    lscsa_ea.ui[1] = spu_readch(SPU_RdSigNotify2);
// Step 1: done by exit().
    save_event_mask();	/* Step 2.  */
    save_tag_mask();	/* Step 3.  */
    set_event_mask();	/* Step 4.  */
    set_tag_mask();		/* Step 5.  */
    build_dma_list(lscsa_ea);	/* Step 6.  */
    save_upper_240kb(lscsa_ea);	/* Step 7.  */
// Step 8: done by exit().
    save_fpcr();		/* Step 9.  */
    save_decr();		/* Step 10. */
    save_srr0();		/* Step 11. */
    enqueue_putllc(lscsa_ea);	/* Step 12. */
    spill_regs_to_mem(lscsa_ea);	/* Step 13. */
    enqueue_sync(lscsa_ea);	/* Step 14. */
    set_tag_update();	/* Step 15. */
    read_tag_status();	/* Step 16. */
    read_llar_status();	/* Step 17. */
    save_complete();	/* Step 18. */
    return 0;
    }
