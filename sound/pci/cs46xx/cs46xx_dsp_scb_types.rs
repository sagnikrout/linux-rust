//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/cs46xx/cs46xx_dsp_scb_types.h
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
// NOTE: comments are copy/paste from cwcemb80.lst
// provided by Tom Woller at Cirrus (my only
// documentation about the SP OS running inside
// the DSP)
//

// This structs are used internally by the SP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_basic_dma_req {
// DMA Requestor Word 0 (DCW)  fields:
//
    pub /: *mut *mut u32 dcw; / DMA Control Word,
    pub /: *mut *mut u32 dmw; / DMA Mode Word,
    pub /: *mut *mut u32 saw; / Source Address Word,
    pub /: *mut *mut u32 daw; / Destination Address Word,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_scatter_gather_ext {
    pub /: *mut *mut u32 npaw; / Next-Page Address Word,
// DMA Requestor Word 5 (NPCW)  fields:
//
    pub /: *mut *mut u32 npcw; / Next-Page Control Word,
    pub /: *mut *mut u32 lbaw; / Loop-Begin Address Word,
    pub /: *mut *mut u32 nplbaw; / Next-Page after Loop-Begin Address Word,
    pub /: *mut *mut u32 sgaw; / Scatter/Gather Address Word,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_volume_control {
}

// Generic stream control block (SCB) structure definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_generic_scb {
// For streaming I/O, the DSP should never alter any words in the DMA
//
// Initialized by the host, only modified by DMA
    pub /: *mut *mut dsp_basic_dma_req basic_req; / Optional,
// Scatter/gather DMA requestor extension   (5 ints)
//
    pub /: *mut *mut dsp_scatter_gather_ext sg_ext; / Optional,
// Sublist pointer & next stream control block (SCB) link.
//
// Pointer to this tasks parameter block & stream function pointer
// rsConfig register for stream buffer (rsDMA reg.
//
    pub /: *mut *mut u32 strm_rs_config; / REQUIRED,
//
// On mixer input streams: indicates mixer input stream configuration
//
    pub /: *mut *mut u32 strm_buf_ptr; / REQUIRED,
// On mixer input streams: points to next mixer input and is updated by the
//
    pub strmPhiIncr: u32,
// Standard stereo volume control
//
    pub /: *mut *mut dsp_volume_control vol_ctrl_t; / Optional,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_spos_control_block {
// WARNING: Certain items in this structure are modified by the host
//
// First element on the Hyper forground task tree
// First 3 dwords are written by the host and read-only on the DSP
// Point to this data structure to enable easy access
// Currently Unused
// Enable extension of SPOS data structure
// Modified by the DSP
// Set by DSP upon encountering a trap (breakpoint) or a spurious
// pointer to forground task tree header for use in next task search
// Data structure for controlling synchronous link update
// Place holder for holding sleep timing
    pub unused5: u32,
// State flags, used to assist control of execution of Hyper Forground
// Space for saving enough context so that we can set up enough
//
    pub rFE_save_for_invalid_IP: u32,
    pub r32_save_for_spurious_int: u32,
    pub r32_save_for_trap: u32,
    pub r32_save_for_HFG: u32,
}

// SPB for MIX_TO_OSTREAM algorithm family
// 16b.16b integer.frac approximation to the
//
// 16b.16b integer.frac accumulated number of
//
// SCB for Timing master algorithm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_timing_master_scb {
// First 12 dwords from generic_scb_t
    pub /: *mut *mut dsp_basic_dma_req basic_req; / Optional,
    pub /: *mut *mut dsp_scatter_gather_ext sg_ext; / Optional,
// Initial values are 0000:xxxx
// Initial values are xxxx:0000
//
// Init. 8000:0005 for 44.1k
//
// Init. 0001:0005 for 44.1k
//
// Init. 44.1k*65536/8k = 0x00058333 for 44.1k
    pub nsamp_per_frm_q15: u32,
}

// SCB for CODEC output algorithm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_codec_output_scb {
// First 13 dwords from generic_scb_t
    pub /: *mut *mut dsp_basic_dma_req basic_req; / Optional,
    pub /: *mut *mut dsp_scatter_gather_ext sg_ext; / Optional,
    pub /: *mut *mut u32 strm_rs_config; / REQUIRED,
    pub /: *mut *mut u32 strm_buf_ptr; / REQUIRED,
// NOTE: The CODEC output task reads samples from the first task on its
//
// Init. 0000:0010 for SDout
//
// Init: 0x0080:0004 for non-AC-97
//
// Pointer to SCB at end of input chain
}

// SCB for CODEC input algorithm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_codec_input_scb {
// First 13 dwords from generic_scb_t
    pub /: *mut *mut dsp_basic_dma_req basic_req; / Optional,
    pub /: *mut *mut dsp_scatter_gather_ext sg_ext; / Optional,
    pub /: *mut *mut u32 strm_rs_config; / REQUIRED,
    pub /: *mut *mut u32 strm_buf_ptr; / REQUIRED,
// NOTE: The CODEC input task reads samples from the hardware FIFO
//
// Init. 0000:0010 for SDout
//
// Init. ?:fffc
//
    pub reserved2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_pcm_serial_input_scb {
// First 13 dwords from generic_scb_t
    pub /: *mut *mut dsp_basic_dma_req basic_req; / Optional,
    pub /: *mut *mut dsp_scatter_gather_ext sg_ext; / Optional,
    pub /: *mut *mut u32 strm_buf_ptr; / REQUIRED,
    pub /: *mut *mut u32 strm_rs_config; / REQUIRED,
// Init. Ptr to CODEC input SCB
//
// Initialized by the host (host updates target volumes)
    pub psi_vol_ctrl: dsp_volume_control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_src_task_scb {
    pub input_buf_strm_config: u32,
    pub accum_phi: u32,
    pub src_strm_rs_config: u32,
    pub src_strm_buf_ptr: u32,
    pub phiIncr6int_26frac: u32,
    pub src_vol_ctrl: dsp_volume_control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_decimate_by_pow2_scb {
// decimationFactor = 2, 4, or 8 (larger factors waste too much memory
//
// coefIncrement = 128 / decimationFactor (for our ROM filter)
//
// extraInSamples: # of accumulated, unused input samples (init. to 0)
//
// halfNumTapsM5: (1/2 number of taps in decimation filter) minus 5
//
    pub dec2_reserved2: u32,
    pub dec2_input_nuf_strm_config: u32,
// inputBufStrmConfig: rsConfig for the input buffer to the decimator
//
// inputBufConsumerPtr: Input buffer read pointer (into SRC filter)
//
    pub dec2_reserved3: u32,
// inputBufProducerPtr: Input buffer write pointer
//
    pub dec2_strm_rs_config: u32,
    pub dec2_strm_buf_ptr: u32,
    pub dec2_reserved4: u32,
    pub /: *mut *mut dsp_volume_control dec2_vol_ctrl; / Not used!,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_vari_decimate_scb {
// extraInSamples: # of accumulated, unused input samples (init. to 0)
// inputBufConsumerPtr: Input buffer read pointer (into SRC filter)
    pub vdec_input_buf_strm_config: u32,
// inputBufStrmConfig: rsConfig for the input buffer to the decimator
    pub vdec_coef_increment: u32,
// coefIncrement = - 128.0 / decimationFactor (as a 32Q15 number)
    pub vdec_accumphi: u32,
// accumPhi: accumulated fractional phase increment (6.26)
// inputBufProducerPtr: Input buffer write pointer
    pub vdec_strm_rs_config: u32,
    pub vdec_strm_buf_ptr: u32,
    pub vdec_phi_incr_6int_26frac: u32,
    pub vdec_vol_ctrl: dsp_volume_control,
}

// SCB for MIX_TO_OSTREAM algorithm family
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_mix2_ostream_scb {
// First 13 dwords from generic_scb_t
    pub /: *mut *mut dsp_basic_dma_req basic_req; / Optional,
    pub /: *mut *mut dsp_scatter_gather_ext sg_ext; / Optional,
    pub /: *mut *mut u32 strm_rs_config; / REQUIRED,
    pub /: *mut *mut u32 strm_buf_ptr; / REQUIRED,
// hi: Number of mixed-down input triplets
//
// hi: Exponential volume change rate
//
}

// SCB for S16_MIX algorithm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_mix_only_scb {
// First 13 dwords from generic_scb_t
    pub /: *mut *mut dsp_basic_dma_req basic_req; / Optional,
    pub /: *mut *mut dsp_scatter_gather_ext sg_ext; / Optional,
    pub /: *mut *mut u32 strm_rs_config; / REQUIRED,
    pub /: *mut *mut u32 strm_buf_ptr; / REQUIRED,
    pub reserved: u32,
    pub vol_ctrl: dsp_volume_control,
}

// SCB for the async. CODEC input algorithm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_async_codec_input_scb {
    pub io_free2: u32,
    pub io_current_total: u32,
    pub io_previous_total: u32,
    pub io_count: u16,
    pub io_count_limit: u16,
    pub o_fifo_base_addr: u16,
    pub ost_mo_format: u16,
// 1 = stereo; 0 = mono
    pub /: *mut xxx for ASER 1 (not allowed); 118 for ASER2,
    pub ostrm_rs_config: u32,
    pub ostrm_buf_ptr: u32,
    pub io_free4: u32,
    pub istrm_rs_config: u32,
    pub istrm_buf_ptr: u32,
// Init. 0000:8042: for ASER1
// Init 1 stero:100 ASER1
//
    pub i_free: u32,
}

// SCB for the SP/DIF CODEC input and output
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_spdifiscb {
    pub current_total: u32,
    pub previous_total: u32,
    pub status_data: u32,
    pub free3: u32,
    pub temp_status: u32,
    pub strm_rs_config: u32,
    pub strm_buf_ptr: u32,
    pub free1: u32,
}

// SCB for the SP/DIF CODEC input and output
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_spdifoscb {
    pub free2: u32,
    pub free3: [u32; 4],
// Need to be here for compatibility with AsynchFGTxCode
    pub strm_rs_config: u32,
    pub strm_buf_ptr: u32,
    pub free4: u32,
    pub free6: [u32; 2],
    pub free1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_asynch_fg_rx_scb {
    pub unused2: [u32; 5],
    pub strm_rs_config: u32,
    pub strm_buf_ptr: u32,
    pub unused_phi_incr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_asynch_fg_tx_scb {
    pub accum_phi: u32,
    pub unused3: [u32; 3],
    pub strm_rs_config: u32,
    pub strm_buf_ptr: u32,
    pub phi_incr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_output_snoop_scb {
// First 13 dwords from generic_scb_t
    pub /: *mut *mut dsp_basic_dma_req basic_req; / Optional,
    pub /: *mut *mut dsp_scatter_gather_ext sg_ext; / Optional,
    pub /: *mut *mut u32 strm_rs_config; / REQUIRED,
    pub /: *mut *mut u32 strm_buf_ptr; / REQUIRED,
    pub snoop_input_buf_ptr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_spio_write_scb {
    pub data1: u32,
    pub data2: u32,
    pub data3: u32,
    pub data4: u32,
    pub unused2: [u32; 2],
    pub unused3: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_magic_snoop_task {
    pub i0: u32,
    pub i1: u32,
    pub strm_buf_ptr1: u32,
    pub i2: u16,
    pub snoop_scb: u16,
    pub i3: u32,
    pub i4: u32,
    pub i5: u32,
    pub i6: u32,
    pub i7: u32,
    pub strm_buf_config: u32,
    pub strm_buf_ptr2: u32,
    pub i8: u32,
    pub vdec_vol_ctrl: dsp_volume_control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_filter_scb {
    pub /: *mut *mut u32 prev_sample_output1; / 0x05,
    pub /: *mut *mut u32 prev_sample_output2; / 0x06,
    pub /: *mut *mut u32 prev_sample_input1; / 0x07,
    pub /: *mut *mut u32 prev_sample_input2; / 0x08,
    pub /: *mut *mut u32 strm_rs_config; / 0x0B,
    pub /: *mut *mut u32 strm_buf_ptr; / 0x0C,
}
