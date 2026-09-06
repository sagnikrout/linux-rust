//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/cell/spufs/spu_restore.c
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
// spu_restore.c
//
// (C) Copyright IBM Corp. 2005
//
// SPU-side context restore sequence outlined in
// Synergistic Processor Element Book IV
//
// Author: Mark Nutter <mnutter@us.ibm.com>
//

pub const LS_SIZE: c_uint = 0x40000	/* 256K (in bytes) */;

    typedef unsigned int u32;
    typedef unsigned long long u64;

pub const BR_INSTR: c_uint = 0x327fff80	/* br -4         */;
pub const NOP_INSTR: c_uint = 0x40200000	/* nop           */;
pub const HEQ_INSTR: c_uint = 0x7b000000	/* heq $0, $0    */;
pub const STOP_INSTR: c_uint = 0x00000000	/* stop 0x0      */;
pub const ILLEGAL_INSTR: c_uint = 0x00800000	/* illegal instr */;
pub const RESTORE_COMPLETE: c_uint = 0x00003ffc	/* stop 0x3ffc   */;
#[no_mangle]
pub unsafe extern "C" fn fetch_regs_from_mem(lscsa_ea: addr64) {
    static inline void fetch_regs_from_mem(addr64 lscsa_ea)
    {
    let mut ls: c_uint = (unsigned int)&regs_spill[0];
    let mut size: c_uint = sizeof(regs_spill);
    let mut tag_id: c_uint = 0;
    unsigned int cmd = 0x40;	/* GET */
    spu_writech(MFC_LSA, ls);
    spu_writech(MFC_EAH, lscsa_ea.ui[0]);
    spu_writech(MFC_EAL, lscsa_ea.ui[1]);
    spu_writech(MFC_Size, size);
    spu_writech(MFC_TagID, tag_id);
    spu_writech(MFC_Cmd, cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn restore_upper_240kb(lscsa_ea: addr64) {
    static inline void restore_upper_240kb(addr64 lscsa_ea)
    {
    let mut ls: c_uint = 16384;
    let mut list: c_uint = (unsigned int)&dma_list[0];
    let mut size: c_uint = sizeof(dma_list);
    let mut tag_id: c_uint = 0;
    unsigned int cmd = 0x44;	/* GETL */
// Restore, Step 4:
// Enqueue the GETL command (tag 0) to the MFC SPU command
// queue to transfer the upper 240 kb of LS from CSA.
//
    spu_writech(MFC_LSA, ls);
    spu_writech(MFC_EAH, lscsa_ea.ui[0]);
    spu_writech(MFC_EAL, list);
    spu_writech(MFC_Size, size);
    spu_writech(MFC_TagID, tag_id);
    spu_writech(MFC_Cmd, cmd);
    }
#[no_mangle]
pub unsafe extern "C" fn restore_decr() {
    static inline void restore_decr(void)
    {
    unsigned int offset;
    unsigned int decr_running;
    unsigned int decr;
// Restore, Step 6(moved):
// If the LSCSA "decrementer running" flag is set
// then write the SPU_WrDec channel with the
// decrementer value from LSCSA.
//
    offset = LSCSA_QW_OFFSET(decr_status);
    decr_running = regs_spill[offset].slot[0] & SPU_DECR_STATUS_RUNNING;
    if (decr_running) {
    offset = LSCSA_QW_OFFSET(decr);
    decr = regs_spill[offset].slot[0];
    spu_writech(SPU_WrDec, decr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn write_ppu_mb() {
    static inline void write_ppu_mb(void)
    {
    unsigned int offset;
    unsigned int data;
// Restore, Step 11:
// Write the MFC_WrOut_MB channel with the PPU_MB
// data from LSCSA.
//
    offset = LSCSA_QW_OFFSET(ppu_mb);
    data = regs_spill[offset].slot[0];
    spu_writech(SPU_WrOutMbox, data);
    }
#[no_mangle]
pub unsafe extern "C" fn write_ppuint_mb() {
    static inline void write_ppuint_mb(void)
    {
    unsigned int offset;
    unsigned int data;
// Restore, Step 12:
// Write the MFC_WrInt_MB channel with the PPUINT_MB
// data from LSCSA.
//
    offset = LSCSA_QW_OFFSET(ppuint_mb);
    data = regs_spill[offset].slot[0];
    spu_writech(SPU_WrOutIntrMbox, data);
    }
#[no_mangle]
pub unsafe extern "C" fn restore_fpcr() {
    static inline void restore_fpcr(void)
    {
    unsigned int offset;
    vector unsigned int fpcr;
// Restore, Step 13:
// Restore the floating-point status and control
// register from the LSCSA.
//
    offset = LSCSA_QW_OFFSET(fpcr);
    fpcr = regs_spill[offset].v;
    spu_mtfpscr(fpcr);
    }
#[no_mangle]
pub unsafe extern "C" fn restore_srr0() {
    static inline void restore_srr0(void)
    {
    unsigned int offset;
    unsigned int srr0;
// Restore, Step 14:
// Restore the SPU SRR0 data from the LSCSA.
//
    offset = LSCSA_QW_OFFSET(srr0);
    srr0 = regs_spill[offset].slot[0];
    spu_writech(SPU_WrSRR0, srr0);
    }
#[no_mangle]
pub unsafe extern "C" fn restore_event_mask() {
    static inline void restore_event_mask(void)
    {
    unsigned int offset;
    unsigned int event_mask;
// Restore, Step 15:
// Restore the SPU_RdEventMsk data from the LSCSA.
//
    offset = LSCSA_QW_OFFSET(event_mask);
    event_mask = regs_spill[offset].slot[0];
    spu_writech(SPU_WrEventMask, event_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn restore_tag_mask() {
    static inline void restore_tag_mask(void)
    {
    unsigned int offset;
    unsigned int tag_mask;
// Restore, Step 16:
// Restore the SPU_RdTagMsk data from the LSCSA.
//
    offset = LSCSA_QW_OFFSET(tag_mask);
    tag_mask = regs_spill[offset].slot[0];
    spu_writech(MFC_WrTagMask, tag_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn restore_complete() {
    static inline void restore_complete(void)
    {
    extern void exit_fini(void);
    unsigned int *exit_instrs = (unsigned int *)exit_fini;
    unsigned int offset;
    unsigned int stopped_status;
    unsigned int stopped_code;
// Restore, Step 18:
// Issue a stop-and-signal instruction with
// "good context restore" signal value.
//
// Restore, Step 19:
// There may be additional instructions placed
// here by the PPE Sequence for SPU Context
// Restore in order to restore the correct
// "stopped state".
//
// This step is handled here by analyzing the
// LSCSA.stopped_status and then modifying the
// exit() function to behave appropriately.
//
    offset = LSCSA_QW_OFFSET(stopped_status);
    stopped_status = regs_spill[offset].slot[0];
    stopped_code = regs_spill[offset].slot[1];
    switch (stopped_status) {
    case SPU_STOPPED_STATUS_P_I:
// SPU_Status[P,I]=1.  Add illegal instruction
// followed by stop-and-signal instruction after
// end of restore code.
//
    exit_instrs[0] = RESTORE_COMPLETE;
    exit_instrs[1] = ILLEGAL_INSTR;
    exit_instrs[2] = STOP_INSTR | stopped_code;
    break;
    case SPU_STOPPED_STATUS_P_H:
// SPU_Status[P,H]=1.  Add 'heq $0, $0' followed
// by stop-and-signal instruction after end of
// restore code.
//
    exit_instrs[0] = RESTORE_COMPLETE;
    exit_instrs[1] = HEQ_INSTR;
    exit_instrs[2] = STOP_INSTR | stopped_code;
    break;
    case SPU_STOPPED_STATUS_S_P:
// SPU_Status[S,P]=1.  Add nop instruction
// followed by 'br -4' after end of restore
// code.
//
    exit_instrs[0] = RESTORE_COMPLETE;
    exit_instrs[1] = STOP_INSTR | stopped_code;
    exit_instrs[2] = NOP_INSTR;
    exit_instrs[3] = BR_INSTR;
    break;
    case SPU_STOPPED_STATUS_S_I:
// SPU_Status[S,I]=1.  Add  illegal instruction
// followed by 'br -4' after end of restore code.
//
    exit_instrs[0] = RESTORE_COMPLETE;
    exit_instrs[1] = ILLEGAL_INSTR;
    exit_instrs[2] = NOP_INSTR;
    exit_instrs[3] = BR_INSTR;
    break;
    case SPU_STOPPED_STATUS_I:
// SPU_Status[I]=1. Add illegal instruction followed
// by infinite loop after end of restore sequence.
//
    exit_instrs[0] = RESTORE_COMPLETE;
    exit_instrs[1] = ILLEGAL_INSTR;
    exit_instrs[2] = NOP_INSTR;
    exit_instrs[3] = BR_INSTR;
    break;
    case SPU_STOPPED_STATUS_S:
// SPU_Status[S]=1. Add two 'nop' instructions.
    exit_instrs[0] = RESTORE_COMPLETE;
    exit_instrs[1] = NOP_INSTR;
    exit_instrs[2] = NOP_INSTR;
    exit_instrs[3] = BR_INSTR;
    break;
    case SPU_STOPPED_STATUS_H:
// SPU_Status[H]=1. Add 'heq $0, $0' instruction
// after end of restore code.
//
    exit_instrs[0] = RESTORE_COMPLETE;
    exit_instrs[1] = HEQ_INSTR;
    exit_instrs[2] = NOP_INSTR;
    exit_instrs[3] = BR_INSTR;
    break;
    case SPU_STOPPED_STATUS_P:
// SPU_Status[P]=1. Add stop-and-signal instruction
// after end of restore code.
//
    exit_instrs[0] = RESTORE_COMPLETE;
    exit_instrs[1] = STOP_INSTR | stopped_code;
    break;
    case SPU_STOPPED_STATUS_R:
// SPU_Status[I,S,H,P,R]=0. Add infinite loop.
    exit_instrs[0] = RESTORE_COMPLETE;
    exit_instrs[1] = NOP_INSTR;
    exit_instrs[2] = NOP_INSTR;
    exit_instrs[3] = BR_INSTR;
    break;
    default:
// SPU_Status[R]=1. No additional instructions.
    break;
    }
    spu_sync();
    }
//
// main - entry point for SPU-side context restore.
//
// This code deviates from the documented sequence in the
// following aspects:
//
// 1. The EA for LSCSA is passed from PPE in the
// signal notification channels.
// 2. The register spill area is pulled by SPU
// into LS, rather than pushed by PPE.
// 3. All 128 registers are restored by exit().
// 4. The exit() function is modified at run
// time in order to properly restore the
// SPU_Status register.
//
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {
    addr64 lscsa_ea;
    lscsa_ea.ui[0] = spu_readch(SPU_RdSigNotify1);
    lscsa_ea.ui[1] = spu_readch(SPU_RdSigNotify2);
    fetch_regs_from_mem(lscsa_ea);
    set_event_mask();		/* Step 1.  */
    set_tag_mask();			/* Step 2.  */
    build_dma_list(lscsa_ea);	/* Step 3.  */
    restore_upper_240kb(lscsa_ea);	/* Step 4.  */
// Step 5: done by 'exit'.
    enqueue_putllc(lscsa_ea);	/* Step 7. */
    set_tag_update();		/* Step 8. */
    read_tag_status();		/* Step 9. */
    restore_decr();			/* moved Step 6. */
    read_llar_status();		/* Step 10. */
    write_ppu_mb();			/* Step 11. */
    write_ppuint_mb();		/* Step 12. */
    restore_fpcr();			/* Step 13. */
    restore_srr0();			/* Step 14. */
    restore_event_mask();		/* Step 15. */
    restore_tag_mask();		/* Step 16. */
// Step 17. done by 'exit'.
    restore_complete();		/* Step 18. */
    return 0;
    }
