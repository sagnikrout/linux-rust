//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/cs46xx/dsp_spos.h
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
// The driver for the Cirrus Logic's Sound Fusion CS46XX based soundcards
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//
// 2002-07 Benny Sjostrand benny@hostmobility.com
//

pub const DSP_MAX_SYMBOLS: c_int = 1024;
pub const DSP_MAX_MODULES: c_int = 64;
pub const DSP_CODE_BYTE_SIZE: c_uint = 0x00007000UL;
pub const DSP_PARAMETER_BYTE_SIZE: c_uint = 0x00003000UL;
pub const DSP_SAMPLE_BYTE_SIZE: c_uint = 0x00003800UL;
pub const DSP_PARAMETER_BYTE_OFFSET: c_uint = 0x00000000UL;
pub const DSP_SAMPLE_BYTE_OFFSET: c_uint = 0x00010000UL;
pub const DSP_CODE_BYTE_OFFSET: c_uint = 0x00020000UL;
pub const WIDE_INSTR_MASK: c_uint = 0x0040;
pub const WIDE_LADD_INSTR_MASK: c_uint = 0x0380;
// this instruction types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wide_opcode {
    WIDE_FOR_BEGIN_LOOP = 0x20,
    WIDE_FOR_BEGIN_LOOP2,

    WIDE_COND_GOTO_ADDR = 0x30,
    WIDE_COND_GOTO_CALL,

    WIDE_TBEQ_COND_GOTO_ADDR = 0x70,
    WIDE_TBEQ_COND_CALL_ADDR,
    WIDE_TBEQ_NCOND_GOTO_ADDR,
    WIDE_TBEQ_NCOND_CALL_ADDR,
    WIDE_TBEQ_COND_GOTO1_ADDR,
    WIDE_TBEQ_COND_CALL1_ADDR,
    WIDE_TBEQ_NCOND_GOTOI_ADDR,
    WIDE_TBEQ_NCOND_CALL1_ADDR,
}

// SAMPLE segment
pub const VARI_DECIMATE_BUF1: c_uint = 0x0000;
pub const WRITE_BACK_BUF1: c_uint = 0x0400;
pub const CODEC_INPUT_BUF1: c_uint = 0x0500;
pub const PCM_READER_BUF1: c_uint = 0x0600;
pub const SRC_DELAY_BUF1: c_uint = 0x0680;
pub const VARI_DECIMATE_BUF0: c_uint = 0x0780;
pub const SRC_OUTPUT_BUF1: c_uint = 0x07A0;
pub const ASYNC_IP_OUTPUT_BUFFER1: c_uint = 0x0A00;
pub const OUTPUT_SNOOP_BUFFER: c_uint = 0x0B00;
pub const SPDIFI_IP_OUTPUT_BUFFER1: c_uint = 0x0E00;
pub const SPDIFO_IP_OUTPUT_BUFFER1: c_uint = 0x1000;
pub const MIX_SAMPLE_BUF1: c_uint = 0x1400;
pub const MIX_SAMPLE_BUF2: c_uint = 0x2E80;
pub const MIX_SAMPLE_BUF3: c_uint = 0x2F00;
pub const MIX_SAMPLE_BUF4: c_uint = 0x2F80;
pub const MIX_SAMPLE_BUF5: c_uint = 0x3000;
// Task stack address
pub const HFG_STACK: c_uint = 0x066A;
pub const FG_STACK: c_uint = 0x066E;
pub const BG_STACK: c_uint = 0x068E;
// SCB's addresses
pub const SPOSCB_ADDR: c_uint = 0x070;
pub const BG_TREE_SCB_ADDR: c_uint = 0x635;
pub const NULL_SCB_ADDR: c_uint = 0x000;
pub const TIMINGMASTER_SCB_ADDR: c_uint = 0x010;
pub const CODECOUT_SCB_ADDR: c_uint = 0x020;
pub const PCMREADER_SCB_ADDR: c_uint = 0x030;
pub const WRITEBACK_SCB_ADDR: c_uint = 0x040;
pub const CODECIN_SCB_ADDR: c_uint = 0x080;
pub const MASTERMIX_SCB_ADDR: c_uint = 0x090;
pub const SRCTASK_SCB_ADDR: c_uint = 0x0A0;
pub const VARIDECIMATE_SCB_ADDR: c_uint = 0x0B0;
pub const PCMSERIALIN_SCB_ADDR: c_uint = 0x0C0;
pub const FG_TASK_HEADER_ADDR: c_uint = 0x600;
pub const ASYNCTX_SCB_ADDR: c_uint = 0x0E0;
pub const ASYNCRX_SCB_ADDR: c_uint = 0x0F0;
pub const SRCTASKII_SCB_ADDR: c_uint = 0x100;
pub const OUTPUTSNOOP_SCB_ADDR: c_uint = 0x110;
pub const PCMSERIALINII_SCB_ADDR: c_uint = 0x120;
pub const SPIOWRITE_SCB_ADDR: c_uint = 0x130;
pub const REAR_CODECOUT_SCB_ADDR: c_uint = 0x140;
pub const OUTPUTSNOOPII_SCB_ADDR: c_uint = 0x150;
pub const PCMSERIALIN_PCM_SCB_ADDR: c_uint = 0x160;
pub const RECORD_MIXER_SCB_ADDR: c_uint = 0x170;
pub const REAR_MIXER_SCB_ADDR: c_uint = 0x180;
pub const CLFE_MIXER_SCB_ADDR: c_uint = 0x190;
pub const CLFE_CODEC_SCB_ADDR: c_uint = 0x1A0;
// hyperforground SCB's
pub const HFG_TREE_SCB: c_uint = 0xBA0;
pub const SPDIFI_SCB_INST: c_uint = 0xBB0;
pub const SPDIFO_SCB_INST: c_uint = 0xBC0;
pub const WRITE_BACK_SPB: c_uint = 0x0D0;
// offsets
pub const AsyncCIOFIFOPointer: c_uint = 0xd;
pub const SPDIFOFIFOPointer: c_uint = 0xd;
pub const SPDIFIFIFOPointer: c_uint = 0xd;
pub const TCBData: c_uint = 0xb;
pub const HFGFlags: c_uint = 0xa;
pub const TCBContextBlk: c_uint = 0x10;
pub const AFGTxAccumPhi: c_uint = 0x4;
pub const SCBsubListPtr: c_uint = 0x9;
pub const SCBfuncEntryPtr: c_uint = 0xA;
pub const SRCCorPerGof: c_uint = 0x2;
pub const SRCPhiIncr6Int26Frac: c_uint = 0xd;
pub const SCBVolumeCtrl: c_uint = 0xe;
// conf
pub const UseASER1Input: c_int = 1;
//
// The following defines are for the flags in the rsConfig01/23 registers of
// the SP.
//
pub const RSCONFIG_MODULO_SIZE_MASK: c_uint = 0x0000000FL;
pub const RSCONFIG_MODULO_16: c_uint = 0x00000001L;
pub const RSCONFIG_MODULO_32: c_uint = 0x00000002L;
pub const RSCONFIG_MODULO_64: c_uint = 0x00000003L;
pub const RSCONFIG_MODULO_128: c_uint = 0x00000004L;
pub const RSCONFIG_MODULO_256: c_uint = 0x00000005L;
pub const RSCONFIG_MODULO_512: c_uint = 0x00000006L;
pub const RSCONFIG_MODULO_1024: c_uint = 0x00000007L;
pub const RSCONFIG_MODULO_4: c_uint = 0x00000008L;
pub const RSCONFIG_MODULO_8: c_uint = 0x00000009L;
pub const RSCONFIG_SAMPLE_SIZE_MASK: c_uint = 0x000000C0L;
pub const RSCONFIG_SAMPLE_8MONO: c_uint = 0x00000000L;
pub const RSCONFIG_SAMPLE_8STEREO: c_uint = 0x00000040L;
pub const RSCONFIG_SAMPLE_16MONO: c_uint = 0x00000080L;
pub const RSCONFIG_SAMPLE_16STEREO: c_uint = 0x000000C0L;
pub const RSCONFIG_UNDERRUN_ZERO: c_uint = 0x00004000L;
pub const RSCONFIG_DMA_TO_HOST: c_uint = 0x00008000L;
pub const RSCONFIG_STREAM_NUM_MASK: c_uint = 0x00FF0000L;
pub const RSCONFIG_MAX_DMA_SIZE_MASK: c_uint = 0x1F000000L;
pub const RSCONFIG_DMA_ENABLE: c_uint = 0x20000000L;
pub const RSCONFIG_PRIORITY_MASK: c_uint = 0xC0000000L;
pub const RSCONFIG_PRIORITY_HIGH: c_uint = 0x00000000L;
pub const RSCONFIG_PRIORITY_MEDIUM_HIGH: c_uint = 0x40000000L;
pub const RSCONFIG_PRIORITY_MEDIUM_LOW: c_uint = 0x80000000L;
pub const RSCONFIG_PRIORITY_LOW: c_uint = 0xC0000000L;

// SP constants
pub const FG_INTERVAL_TIMER_PERIOD: c_uint = 0x0051;
pub const BG_INTERVAL_TIMER_PERIOD: c_uint = 0x0100;
// Only SP accessible registers
pub const SP_ASER_COUNTDOWN: c_uint = 0x8040;
pub const SP_SPDOUT_FIFO: c_uint = 0x0108;
pub const SP_SPDIN_MI_FIFO: c_uint = 0x01E0;
pub const SP_SPDIN_D_FIFO: c_uint = 0x01F0;
pub const SP_SPDIN_STATUS: c_uint = 0x8048;
pub const SP_SPDIN_CONTROL: c_uint = 0x8049;
pub const SP_SPDIN_FIFOPTR: c_uint = 0x804A;
pub const SP_SPDOUT_STATUS: c_uint = 0x804C;
pub const SP_SPDOUT_CONTROL: c_uint = 0x804D;
pub const SP_SPDOUT_CSUV: c_uint = 0x808E;
// wrap all 8 bits
// update nextSCB and subListPtr in SCB

