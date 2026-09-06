//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/tw5864/tw5864-reg.h
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
// TW5864 driver - registers description
//
// Copyright (C) 2016 Bluecherry, LLC <maintainers@bluecherrydvr.com>
//
// According to TW5864_datasheet_0.6d.pdf, tw5864b1-ds.pdf
// Register Description - Direct Map Space
// 0x0000 ~ 0x1ffc - H264 Register Map
// [15:0] The Version register for H264 core (Read Only)
pub const TW5864_H264REV: c_uint = 0x0000;
pub const TW5864_EMU: c_uint = 0x0004;
// Define controls in register TW5864_EMU
// DDR controller enabled

// Enable bit for Inter module

// Enable bit for Sensor Interface module

// Enable bit for Host Burst Access

// Enable bit for Loop Filter module

// Enable bit for PLBK module

//
// Video Frame mapping in DDR
// 00 CIF
// 01 D1
// 10 Reserved
// 11 Reserved
//

pub const TW5864_UNDECLARED_H264REV_PART2: c_uint = 0x0008;
pub const TW5864_SLICE: c_uint = 0x000c;
// Define controls in register TW5864_SLICE
// VLC Slice end flag

// Master Slice End Flag

// Host to start a new slice Address

//
// [15:0] Two bit for each channel (channel 0 ~ 7). Each two bits are the buffer
// pointer for the last encoded frame of the corresponding channel.
//
pub const TW5864_ENC_BUF_PTR_REC1: c_uint = 0x0010;
// [5:0] DSP_MB_QP and [15:10] DSP_LPF_OFFSET
pub const TW5864_DSP_QP: c_uint = 0x0018;
// Define controls in register TW5864_DSP_QP
// [5:0] H264 QP Value for codec
pub const TW5864_DSP_MB_QP: c_uint = 0x003f;
//
// [15:10] H264 LPF_OFFSET Address
// (Default 0)
//
pub const TW5864_DSP_LPF_OFFSET: c_uint = 0xfc00;
pub const TW5864_DSP_CODEC: c_uint = 0x001c;
// Define controls in register TW5864_DSP_CODEC
//
// 0: Encode (TW5864 Default)
// 1: Decode
//

//
// 0->3 4 VLC data buffer in DDR (1M each)
// 0->7 8 VLC data buffer in DDR (512k each)
//

//
// 0 4CIF in 1 MB
// 1 1CIF in 1 MB
//

//
// 0 2 falf D1 in 1 MB
// 1 1 half D1 in 1 MB
//

// VLC Stream valid

// MV Vector Valid

// MV Flag Valid

pub const TW5864_DSP_SEN: c_uint = 0x0020;
// Define controls in register TW5864_DSP_SEN
// Org Buffer Base for Luma (default 0)
pub const TW5864_DSP_SEN_PIC_LU: c_uint = 0x000f;
// Org Buffer Base for Chroma (default 4)
pub const TW5864_DSP_SEN_PIC_CHM: c_uint = 0x00f0;
// Maximum Number of Buffers (default 4)
pub const TW5864_DSP_SEN_PIC_MAX: c_uint = 0x0700;
//
// Original Frame D1 or HD1 switch
// (Default 0)
//
pub const TW5864_DSP_SEN_HFULL: c_uint = 0x1000;
pub const TW5864_DSP_REF_PIC: c_uint = 0x0024;
// Define controls in register TW5864_DSP_REF_PIC
// Ref Buffer Base for Luma (default 0)
pub const TW5864_DSP_REF_PIC_LU: c_uint = 0x000f;
// Ref Buffer Base for Chroma (default 4)
pub const TW5864_DSP_REF_PIC_CHM: c_uint = 0x00f0;
// Maximum Number of Buffers (default 4)
pub const TW5864_DSP_REF_PIC_MAX: c_uint = 0x0700;
// [15:0] SEN_EN_CH[n] SENIF original frame capture enable for each channel
pub const TW5864_SEN_EN_CH: c_uint = 0x0028;
pub const TW5864_DSP: c_uint = 0x002c;
// Define controls in register TW5864_DSP
// The ID for channel selected for encoding operation
pub const TW5864_DSP_ENC_CHN: c_uint = 0x000f;
// See DSP_MB_DELAY below
pub const TW5864_DSP_MB_WAIT: c_uint = 0x0010;
//
// DSP Chroma Switch
// 0 DDRB
// 1 DDRA
//
pub const TW5864_DSP_CHROM_SW: c_uint = 0x0020;
// VLC Flow Control: 1 for enable
pub const TW5864_DSP_FLW_CNTL: c_uint = 0x0040;
//
// If DSP_MB_WAIT == 0, MB delay is DSP_MB_DELAY * 16
// If DSP_MB_DELAY == 1, MB delay is DSP_MB_DELAY * 128
//
pub const TW5864_DSP_MB_DELAY: c_uint = 0x0f00;
pub const TW5864_DDR: c_uint = 0x0030;
// Define controls in register TW5864_DDR
// DDR Single Access Page Number
pub const TW5864_DDR_PAGE_CNTL: c_uint = 0x00ff;
// DDR-DPR Burst Read Enable

//
// DDR A/B Select as HOST access
// 0 Select DDRA
// 1 Select DDRB
//

//
// DDR Access Mode Select
// 0 Single R/W Access (Host <-> DDR)
// 1 Burst R/W Access (Host <-> DPR)
//

// The original frame capture pointer. Two bits for each channel
// SENIF_ORG_FRM_PTR [15:0]
pub const TW5864_SENIF_ORG_FRM_PTR1: c_uint = 0x0038;
// SENIF_ORG_FRM_PTR [31:16]
pub const TW5864_SENIF_ORG_FRM_PTR2: c_uint = 0x003c;
pub const TW5864_DSP_SEN_MODE: c_uint = 0x0040;
// Define controls in register TW5864_DSP_SEN_MODE
pub const TW5864_DSP_SEN_MODE_CH0: c_uint = 0x000f;
pub const TW5864_DSP_SEN_MODE_CH1: c_uint = 0x00f0;
//
// [15:0]: ENC_BUF_PTR_REC[31:16] Two bit for each channel (channel 8 ~ 15).
// Each two bits are the buffer pointer for the last encoded frame of a channel
//
pub const TW5864_ENC_BUF_PTR_REC2: c_uint = 0x004c;
// Current MV Flag Status Pointer for Channel n. (Read only)
//
// [1:0] CH0_MV_PTR, ..., [15:14] CH7_MV_PTR
//
pub const TW5864_CH_MV_PTR1: c_uint = 0x0060;
//
// [1:0] CH8_MV_PTR, ..., [15:14] CH15_MV_PTR
//
pub const TW5864_CH_MV_PTR2: c_uint = 0x0064;
//
// [15:0] Reset Current MV Flag Status Pointer for Channel n (one bit each)
//
pub const TW5864_RST_MV_PTR: c_uint = 0x0068;
pub const TW5864_INTERLACING: c_uint = 0x0200;
// Define controls in register TW5864_INTERLACING
//
// Inter_Mode Start. 2-nd bit? A guess. Missing in datasheet. Without this bit
// set, the output video is interlaced (stripy).
//

// Deinterlacer Enable

//
// De-interlacer Mode
// 1 Shuffled frame
// 0 Normal Un-Shuffled Frame
//

//
// Down scale original frame in X direction
// 11: Un-used
// 10: down-sample to 1/4
// 01: down-sample to 1/2
// 00: down-sample disabled
//

//
// Down scale original frame in Y direction
// 11: Un-used
// 10: down-sample to 1/4
// 01: down-sample to 1/2
// 00: down-sample disabled
//

//
// 1 Dual Stream
// 0 Single Stream
//

pub const TW5864_DSP_REF: c_uint = 0x0204;
// Define controls in register TW5864_DSP_REF
// Number of reference frame (Default 1 for TW5864B)
pub const TW5864_DSP_REF_FRM: c_uint = 0x000f;
// Window size
pub const TW5864_DSP_WIN_SIZE: c_uint = 0x02f0;
pub const TW5864_DSP_SKIP: c_uint = 0x0208;
// Define controls in register TW5864_DSP_SKIP
//
// Skip Offset Enable bit
// 0 DSP_SKIP_OFFSET value is not used (default 8)
// 1 DSP_SKIP_OFFSET value is used in HW
//
pub const TW5864_DSP_SKIP_OFEN: c_uint = 0x0080;
// Skip mode cost offset (default 8)
pub const TW5864_DSP_SKIP_OFFSET: c_uint = 0x007f;
pub const TW5864_MOTION_SEARCH_ETC: c_uint = 0x020c;
// Define controls in register TW5864_MOTION_SEARCH_ETC
// Enable quarter pel search mode

// Enable half pel search mode

// Enable motion search mode

// Enable Intra mode

// Enable Skip Mode

// Search Option (Default 2"b01)

pub const TW5864_DSP_ENC_REC: c_uint = 0x0210;
// Define controls in register TW5864_DSP_ENC_REC
// Reference Buffer Pointer for encoding
pub const TW5864_DSP_ENC_REF_PTR: c_uint = 0x0007;
// Reconstruct Buffer pointer
pub const TW5864_DSP_REC_BUF_PTR: c_uint = 0x7000;
// [15:0] Lambda Value for H264
pub const TW5864_DSP_REF_MVP_LAMBDA: c_uint = 0x0214;
pub const TW5864_DSP_PIC_MAX_MB: c_uint = 0x0218;
// Define controls in register TW5864_DSP_PIC_MAX_MB
// The MB number in Y direction for a frame
pub const TW5864_DSP_PIC_MAX_MB_Y: c_uint = 0x007f;
// The MB number in X direction for a frame
pub const TW5864_DSP_PIC_MAX_MB_X: c_uint = 0x7f00;
// The original frame pointer for encoding
pub const TW5864_DSP_ENC_ORG_PTR_REG: c_uint = 0x021c;
// Mask to use with TW5864_DSP_ENC_ORG_PTR
pub const TW5864_DSP_ENC_ORG_PTR_MASK: c_uint = 0x7000;
// Number of bits to shift with TW5864_DSP_ENC_ORG_PTR
pub const TW5864_DSP_ENC_ORG_PTR_SHIFT: c_int = 12;
// DDR base address of OSD rectangle attribute data
pub const TW5864_DSP_OSD_ATTRI_BASE: c_uint = 0x0220;
// OSD enable bit for each channel
pub const TW5864_DSP_OSD_ENABLE: c_uint = 0x0228;
// 0x0280 ~ 0x029c - Motion Vector for 1st 4x4 Block, e.g., 80 (X), 84 (Y)
pub const TW5864_ME_MV_VEC1: c_uint = 0x0280;
// 0x02a0 ~ 0x02bc - Motion Vector for 2nd 4x4 Block, e.g., A0 (X), A4 (Y)
pub const TW5864_ME_MV_VEC2: c_uint = 0x02a0;
// 0x02c0 ~ 0x02dc - Motion Vector for 3rd 4x4 Block, e.g., C0 (X), C4 (Y)
pub const TW5864_ME_MV_VEC3: c_uint = 0x02c0;
// 0x02e0 ~ 0x02fc - Motion Vector for 4th 4x4 Block, e.g., E0 (X), E4 (Y)
pub const TW5864_ME_MV_VEC4: c_uint = 0x02e0;
//
// [5:0]
// if (intra16x16_cost < (intra4x4_cost+dsp_i4x4_offset))
// Intra_mode = intra16x16_mode
// Else
// Intra_mode = intra4x4_mode
//
pub const TW5864_DSP_I4x4_OFFSET: c_uint = 0x040c;
//
// [6:4]
// 0x5 Only 4x4
// 0x6 Only 16x16
// 0x7 16x16 & 4x4
//
pub const TW5864_DSP_INTRA_MODE: c_uint = 0x0410;
pub const TW5864_DSP_INTRA_MODE_SHIFT: c_int = 4;

pub const TW5864_DSP_INTRA_MODE_4x4: c_uint = 0x5;
pub const TW5864_DSP_INTRA_MODE_16x16: c_uint = 0x6;
pub const TW5864_DSP_INTRA_MODE_4x4_AND_16x16: c_uint = 0x7;
//
// [5:0] WEIGHT Factor for I4x4 cost calculation (QP dependent)
//
pub const TW5864_DSP_I4x4_WEIGHT: c_uint = 0x0414;
//
// [7:0] Offset used to affect Intra/ME model decision
// If (me_cost < intra_cost + dsp_resid_mode_offset)
// Pred_Mode = me_mode
// Else
// Pred_mode = intra_mode
//
pub const TW5864_DSP_RESID_MODE_OFFSET: c_uint = 0x0604;
// 0x0800 ~ 0x09ff - Quantization TABLE Values
pub const TW5864_QUAN_TAB: c_uint = 0x0800;
// Valid channel value [0; f], frame value [0; 3]

pub const TW5864_FRAME_BUS1: c_uint = 0x0d00;
//
// 1 Progressive in part A in bus n
// 0 Interlaced in part A in bus n
//

//
// 1 Progressive in part B in bus n
// 0 Interlaced in part B in bus n
//

//
// 1 Frame Mode in bus n
// 0 Field Mode in bus n
//

//
// 0 4CIF in bus n
// 1 1D1 + 4 CIF in bus n
// 2 2D1 in bus n
//

// Bus 1 goes in TW5864_FRAME_BUS1 in [4:0]
// Bus 2 goes in TW5864_FRAME_BUS1 in [12:8]
pub const TW5864_FRAME_BUS2: c_uint = 0x0d04;
// Bus 3 goes in TW5864_FRAME_BUS2 in [4:0]
// Bus 4 goes in TW5864_FRAME_BUS2 in [12:8]
// [15:0] Horizontal Mirror for channel n
pub const TW5864_SENIF_HOR_MIR: c_uint = 0x0d08;
// [15:0] Vertical Mirror for channel n
pub const TW5864_SENIF_VER_MIR: c_uint = 0x0d0c;
//
// FRAME_WIDTH_BUSn_A
// 0x15f: 4 CIF
// 0x2cf: 1 D1 + 3 CIF
// 0x2cf: 2 D1
// FRAME_WIDTH_BUSn_B
// 0x15f: 4 CIF
// 0x2cf: 1 D1 + 3 CIF
// 0x2cf: 2 D1
// FRAME_HEIGHT_BUSn_A
// 0x11f: 4CIF (PAL)
// 0x23f: 1D1 + 3CIF (PAL)
// 0x23f: 2 D1 (PAL)
// 0x0ef: 4CIF (NTSC)
// 0x1df: 1D1 + 3CIF (NTSC)
// 0x1df: 2 D1 (NTSC)
// FRAME_HEIGHT_BUSn_B
// 0x11f: 4CIF (PAL)
// 0x23f: 1D1 + 3CIF (PAL)
// 0x23f: 2 D1 (PAL)
// 0x0ef: 4CIF (NTSC)
// 0x1df: 1D1 + 3CIF (NTSC)
// 0x1df: 2 D1 (NTSC)
//

//
// 1: the bus mapped Channel n Full D1
// 0: the bus mapped Channel n Half D1
//
pub const TW5864_FULL_HALF_FLAG: c_uint = 0x0d50;
//
// 0 The bus mapped Channel select partA Mode
// 1 The bus mapped Channel select partB Mode
//
pub const TW5864_FULL_HALF_MODE_SEL: c_uint = 0x0d54;
pub const TW5864_VLC: c_uint = 0x1000;
// Define controls in register TW5864_VLC
// QP Value used by H264 CAVLC
pub const TW5864_VLC_SLICE_QP: c_uint = 0x003f;
//
// Swap byte order of VLC stream in d-word.
// 1 Normal (VLC output= [31:0])
// 0 Swap (VLC output={[23:16],[31:24],[7:0], [15:8]})
//

// Enable Adding 03 circuit for VLC stream

// Number of bit for VLC bit Align
pub const TW5864_VLC_BIT_ALIGN_SHIFT: c_int = 8;

//
// Synchronous Interface select for VLC Stream
// 1 CDC_VLCS_MAS read VLC stream
// 0 CPU read VLC stream
//

// Enable VLC overflow control

//
// 1 PCI Master Mode
// 0 Non PCI Master Mode
//

//
// 0 Enable Adding 03 to VLC header and stream
// 1 Disable Adding 03 to VLC header of "00000001"
//

//
// Status of VLC stream in DDR (one bit for each buffer)
// 1 VLC is ready in buffer n (HW set)
// 0 VLC is not ready in buffer n (SW clear)
//
pub const TW5864_VLC_BUF_RDY_SHIFT: c_int = 24;

// Total number of bit in the slice
pub const TW5864_SLICE_TOTAL_BIT: c_uint = 0x1004;
// Total number of bit in the residue
pub const TW5864_RES_TOTAL_BIT: c_uint = 0x1008;
pub const TW5864_VLC_BUF: c_uint = 0x100c;
// Define controls in register TW5864_VLC_BUF
// VLC BK0 full status, write '1' to clear

// VLC BK1 full status, write '1' to clear

// VLC end slice status, write '1' to clear

// VLC Buffer overflow status, write '1' to clear

// VLC string length in either buffer 0 or 1 at end of frame
pub const TW5864_VLC_STREAM_LEN_SHIFT: c_int = 4;

// [15:0] Total coefficient number in a frame
pub const TW5864_TOTAL_COEF_NO: c_uint = 0x1010;
// [0] VLC Encoder Interrupt. Write '1' to clear
pub const TW5864_VLC_DSP_INTR: c_uint = 0x1014;
// [31:0] VLC stream CRC checksum
pub const TW5864_VLC_STREAM_CRC: c_uint = 0x1018;
pub const TW5864_VLC_RD: c_uint = 0x101c;
// Define controls in register TW5864_VLC_RD
//
// 1 Read VLC lookup Memory
// 0 Read VLC Stream Memory
//

//
// 1 Read VLC Stream Memory in burst mode
// 0 Read VLC Stream Memory in single mode
//

// 0x2000 ~ 0x2ffc - H264 Stream Memory Map
//
// A word is 4 bytes. I.e.,
// VLC_STREAM_MEM[0] address: 0x2000
// VLC_STREAM_MEM[1] address: 0x2004
// ...
// VLC_STREAM_MEM[3FF] address: 0x2ffc
//
pub const TW5864_VLC_STREAM_MEM_START: c_uint = 0x2000;
pub const TW5864_VLC_STREAM_MEM_MAX_OFFSET: c_uint = 0x3ff;

// 0x4000 ~ 0x4ffc - Audio Register Map
// [31:0] config 1ms cnt = Realtime clk/1000
pub const TW5864_CFG_1MS_CNT: c_uint = 0x4000;
pub const TW5864_ADPCM: c_uint = 0x4004;
// Define controls in register TW5864_ADPCM
// ADPCM decoder enable

// ADPCM input data enable

// ADPCM encoder enable

pub const TW5864_AUD: c_uint = 0x4008;
// Define controls in register TW5864_AUD
// Record path PCM Audio enable bit for each channel
pub const TW5864_AUD_ORG_CH_EN: c_uint = 0x00ff;
// Speaker path PCM Audio Enable

//
// 0 16bit
// 1 8bit
//

pub const TW5864_AUD_TYPE_SHIFT: c_int = 18;
//
// 0 PCM
// 3 ADPCM
//

pub const TW5864_AUD_SAMPLE_RATE_SHIFT: c_int = 22;
//
// 0 8K
// 1 16K
//

// Channel ID used to select audio channel (0 to 16) for loopback
pub const TW5864_TESTLOOP_CHID_SHIFT: c_int = 24;

// Enable AD Loopback Test

//
// 0 Asynchronous Mode or PCI target mode
// 1 PCI Initiator Mode
//

pub const TW5864_AUD_ADPCM: c_uint = 0x400c;
// Define controls in register TW5864_AUD_ADPCM
// Record path ADPCM audio channel enable, one bit for each
pub const TW5864_AUD_ADPCM_CH_EN: c_uint = 0x00ff;
// Speaker path ADPCM audio channel enable

pub const TW5864_PC_BLOCK_ADPCM_RD_NO: c_uint = 0x4018;
pub const TW5864_PC_BLOCK_ADPCM_RD_NO_MASK: c_uint = 0x1f;
//
// For ADPCM_ENC_WR_PTR, ADPCM_ENC_RD_PTR (see below):
// Bit[2:0] ch0
// Bit[5:3] ch1
// Bit[8:6] ch2
// Bit[11:9] ch3
// Bit[14:12] ch4
// Bit[17:15] ch5
// Bit[20:18] ch6
// Bit[23:21] ch7
// Bit[26:24] ch8
// Bit[29:27] ch9
// Bit[32:30] ch10
// Bit[35:33] ch11
// Bit[38:36] ch12
// Bit[41:39] ch13
// Bit[44:42] ch14
// Bit[47:45] ch15
// Bit[50:48] ch16
//
pub const TW5864_ADPCM_ENC_XX_MASK: c_uint = 0x3fff;
pub const TW5864_ADPCM_ENC_XX_PTR2_SHIFT: c_int = 30;
// ADPCM_ENC_WR_PTR[29:0]
pub const TW5864_ADPCM_ENC_WR_PTR1: c_uint = 0x401c;
// ADPCM_ENC_WR_PTR[50:30]
pub const TW5864_ADPCM_ENC_WR_PTR2: c_uint = 0x4020;
// ADPCM_ENC_RD_PTR[29:0]
pub const TW5864_ADPCM_ENC_RD_PTR1: c_uint = 0x4024;
// ADPCM_ENC_RD_PTR[50:30]
pub const TW5864_ADPCM_ENC_RD_PTR2: c_uint = 0x4028;
// [3:0] rd ch0, [7:4] rd ch1, [11:8] wr ch0, [15:12] wr ch1
pub const TW5864_ADPCM_DEC_RD_WR_PTR: c_uint = 0x402c;
//
// For TW5864_AD_ORIG_WR_PTR, TW5864_AD_ORIG_RD_PTR:
// Bit[3:0] ch0
// Bit[7:4] ch1
// Bit[11:8] ch2
// Bit[15:12] ch3
// Bit[19:16] ch4
// Bit[23:20] ch5
// Bit[27:24] ch6
// Bit[31:28] ch7
// Bit[35:32] ch8
// Bit[39:36] ch9
// Bit[43:40] ch10
// Bit[47:44] ch11
// Bit[51:48] ch12
// Bit[55:52] ch13
// Bit[59:56] ch14
// Bit[63:60] ch15
// Bit[67:64] ch16
//
// AD_ORIG_WR_PTR[31:0]
pub const TW5864_AD_ORIG_WR_PTR1: c_uint = 0x4030;
// AD_ORIG_WR_PTR[63:32]
pub const TW5864_AD_ORIG_WR_PTR2: c_uint = 0x4034;
// AD_ORIG_WR_PTR[67:64]
pub const TW5864_AD_ORIG_WR_PTR3: c_uint = 0x4038;
// AD_ORIG_RD_PTR[31:0]
pub const TW5864_AD_ORIG_RD_PTR1: c_uint = 0x403c;
// AD_ORIG_RD_PTR[63:32]
pub const TW5864_AD_ORIG_RD_PTR2: c_uint = 0x4040;
// AD_ORIG_RD_PTR[67:64]
pub const TW5864_AD_ORIG_RD_PTR3: c_uint = 0x4044;
pub const TW5864_PC_BLOCK_ORIG_RD_NO: c_uint = 0x4048;
pub const TW5864_PC_BLOCK_ORIG_RD_NO_MASK: c_uint = 0x1f;
pub const TW5864_PCI_AUD: c_uint = 0x404c;
// Define controls in register TW5864_PCI_AUD
//
// The register is applicable to PCI initiator mode only. Used to select PCM(0)
// or ADPCM(1) audio data sent to PC. One bit for each channel
//
pub const TW5864_PCI_DATA_SEL: c_uint = 0xffff;
//
// Audio flow control mode selection bit.
// 0 Flow control disabled. TW5864 continuously sends audio frame to PC
// (initiator mode)
// 1 Flow control enabled
//

//
// When PCI_FLOW_EN is set, PCI need to toggle this bit to send an audio frame
// to PC. One toggle to send one frame.
//

// [1:0] CS valid to data valid CLK cycles when writing operation
pub const TW5864_CS2DAT_CNT: c_uint = 0x8000;
// [2:0] Data valid signal width by system clock cycles
pub const TW5864_DATA_VLD_WIDTH: c_uint = 0x8004;
pub const TW5864_SYNC: c_uint = 0x8008;
// Define controls in register TW5864_SYNC
//
// 0 vlc stream to synchronous port
// 1 vlc stream to ddr buffers
//

//
// 0 SYNC Address sampled on Rising edge
// 1 SYNC Address sampled on Falling edge
//

pub const TW5864_VLC_STR_DELAY_SHIFT: c_int = 1;
//
// 0 No system delay
// 1 One system clock delay
// 2 Two system clock delay
// 3 Three system clock delay
//

//
// 0 Rising edge output
// 1 Falling edge output
//

//
// [1:0]
// 2'b00 phase set to 180 degree
// 2'b01 phase set to 270 degree
// 2'b10 phase set to 0 degree
// 2'b11 phase set to 90 degree
//
pub const TW5864_I2C_PHASE_CFG: c_uint = 0x800c;
//
// The system / DDR clock (166 MHz) is generated with an on-chip system clock
// PLL (SYSPLL) using input crystal clock of 27 MHz. The system clock PLL
// frequency is controlled with the following equation.
// CLK_OUT = CLK_IN * (M+1) / ((N+1) * P)
// SYSPLL_M M parameter
// SYSPLL_N N parameter
// SYSPLL_P P parameter
//
// SYSPLL_M[7:0]
pub const TW5864_SYSPLL1: c_uint = 0x8018;
// Define controls in register TW5864_SYSPLL1
pub const TW5864_SYSPLL_M_LOW: c_uint = 0x00ff;
// [2:0]: SYSPLL_M[10:8], [7:3]: SYSPLL_N[4:0]
pub const TW5864_SYSPLL2: c_uint = 0x8019;
// Define controls in register TW5864_SYSPLL2
pub const TW5864_SYSPLL_M_HI: c_uint = 0x07;
pub const TW5864_SYSPLL_N_LOW_SHIFT: c_int = 3;

//
// [1:0]: SYSPLL_N[6:5], [3:2]: SYSPLL_P, [4]: SYSPLL_IREF, [7:5]: SYSPLL_CP_SEL
//
pub const TW5864_SYSPLL3: c_uint = 0x8020;
// Define controls in register TW5864_SYSPLL3
pub const TW5864_SYSPLL_N_HI: c_uint = 0x03;
pub const TW5864_SYSPLL_P_SHIFT: c_int = 2;

//
// SYSPLL bias current control
// 0 Lower current (default)
// 1 30% higher current
//

//
// SYSPLL charge pump current selection
// 0 1,5 uA
// 1 4 uA
// 2 9 uA
// 3 19 uA
// 4 39 uA
// 5 79 uA
// 6 159 uA
// 7 319 uA
//
pub const TW5864_SYSPLL_CP_SEL_SHIFT: c_int = 5;

//
// [1:0]: SYSPLL_VCO, [3:2]: SYSPLL_LP_X8, [5:4]: SYSPLL_ICP_SEL,
// [6]: SYSPLL_LPF_5PF, [7]: SYSPLL_ED_SEL
//
pub const TW5864_SYSPLL4: c_uint = 0x8021;
// Define controls in register TW5864_SYSPLL4
//
// SYSPLL_VCO VCO Range selection
// 00 5 ~ 75 MHz
// 01 50 ~ 140 MHz
// 10 110 ~ 320 MHz
// 11 270 ~ 700 MHz
//
pub const TW5864_SYSPLL_VCO: c_uint = 0x03;
pub const TW5864_SYSPLL_LP_X8_SHIFT: c_int = 2;
//
// Loop resister
// 0 38.5K ohms
// 1 6.6K ohms (default)
// 2 2.2K ohms
// 3 1.1K ohms
//

pub const TW5864_SYSPLL_ICP_SEL_SHIFT: c_int = 4;
//
// PLL charge pump fine tune
// 00 x1 (default)
// 01 x1/2
// 10 x1/7
// 11 x1/8
//

//
// PLL low pass filter phase margin adjustment
// 0 no 5pF (default)
// 1 5pF added
//

//
// PFD select edge for detection
// 0 Falling edge (default)
// 1 Rising edge
//

// [0]: SYSPLL_RST, [4]: SYSPLL_PD
pub const TW5864_SYSPLL5: c_uint = 0x8024;
// Define controls in register TW5864_SYSPLL5
// Reset SYSPLL

// Power down SYSPLL

pub const TW5864_PLL_CFG: c_uint = 0x801c;
// Define controls in register TW5864_PLL_CFG
//
// Issue Soft Reset from Async Host Interface / PCI Interface clock domain.
// Become valid after sync to the xtal clock domain. This bit is set only if
// LOAD register bit is also set to 1.
//

//
// Issue SYSPLL (166 MHz) configuration latch from Async host interface / PCI
// Interface clock domain. The configuration setting becomes effective only if
// LOAD register bit is also set to 1.
//

//
// Issue SPLL (108 MHz) configuration load from Async host interface / PCI
// Interface clock domain. The configuration setting becomes effective only if
// the LOAD register bit is also set to 1.
//

//
// Set this bit to latch the SRST, SYSPLL_CFG, SPLL_CFG setting into the xtal
// clock domain to restart the PLL. This bit is self cleared.
//

// SPLL_IREF, SPLL_LPX4, SPLL_CPX4, SPLL_PD, SPLL_DBG
pub const TW5864_SPLL: c_uint = 0x8028;
// 0x8800 ~ 0x88fc - Interrupt Register Map
//
// Trigger mode of interrupt source 0 ~ 15
// 1 Edge trigger mode
// 0 Level trigger mode
//
pub const TW5864_TRIGGER_MODE_L: c_uint = 0x8800;
// Trigger mode of interrupt source 16 ~ 31
pub const TW5864_TRIGGER_MODE_H: c_uint = 0x8804;
// Enable of interrupt source 0 ~ 15
pub const TW5864_INTR_ENABLE_L: c_uint = 0x8808;
// Enable of interrupt source 16 ~ 31
pub const TW5864_INTR_ENABLE_H: c_uint = 0x880c;
// Clear interrupt command of interrupt source 0 ~ 15
pub const TW5864_INTR_CLR_L: c_uint = 0x8810;
// Clear interrupt command of interrupt source 16 ~ 31
pub const TW5864_INTR_CLR_H: c_uint = 0x8814;
//
// Assertion of interrupt source 0 ~ 15
// 1 High level or pos-edge is assertion
// 0 Low level or neg-edge is assertion
//
pub const TW5864_INTR_ASSERT_L: c_uint = 0x8818;
// Assertion of interrupt source 16 ~ 31
pub const TW5864_INTR_ASSERT_H: c_uint = 0x881c;
//
// Output level of interrupt
// 1 Interrupt output is high assertion
// 0 Interrupt output is low assertion
//
pub const TW5864_INTR_OUT_LEVEL: c_uint = 0x8820;
//
// Status of interrupt source 0 ~ 15
// Bit[0]: VLC 4k RAM interrupt
// Bit[1]: BURST DDR RAM interrupt
// Bit[2]: MV DSP interrupt
// Bit[3]: video lost interrupt
// Bit[4]: gpio 0 interrupt
// Bit[5]: gpio 1 interrupt
// Bit[6]: gpio 2 interrupt
// Bit[7]: gpio 3 interrupt
// Bit[8]: gpio 4 interrupt
// Bit[9]: gpio 5 interrupt
// Bit[10]: gpio 6 interrupt
// Bit[11]: gpio 7 interrupt
// Bit[12]: JPEG interrupt
// Bit[13:15]: Reserved
//
pub const TW5864_INTR_STATUS_L: c_uint = 0x8838;
//
// Status of interrupt source 16 ~ 31
// Bit[0]: Reserved
// Bit[1]: VLC done interrupt
// Bit[2]: Reserved
// Bit[3]: AD Vsync interrupt
// Bit[4]: Preview eof interrupt
// Bit[5]: Preview overflow interrupt
// Bit[6]: Timer interrupt
// Bit[7]: Reserved
// Bit[8]: Audio eof interrupt
// Bit[9]: I2C done interrupt
// Bit[10]: AD interrupt
// Bit[11:15]: Reserved
//
pub const TW5864_INTR_STATUS_H: c_uint = 0x883c;
// Defines of interrupt bits, united for both low and high word registers

// n belongs to [0; 7]

// 0x9000 ~ 0x920c - Video Capture (VIF) Register Map
//
// H264EN_CH_STATUS[n] Status of Vsync synchronized H264EN_CH_EN (Read Only)
// 1 Channel Enabled
// 0 Channel Disabled
//
pub const TW5864_H264EN_CH_STATUS: c_uint = 0x9000;
//
// [15:0] H264EN_CH_EN[n] H264 Encoding Path Enable for channel
// 1 Channel Enabled
// 0 Channel Disabled
//
pub const TW5864_H264EN_CH_EN: c_uint = 0x9004;
//
// H264EN_CH_DNS[n] H264 Encoding Path Downscale Video Decoder Input for
// channel n
// 1 Downscale Y to 1/2
// 0 Does not downscale
//
pub const TW5864_H264EN_CH_DNS: c_uint = 0x9008;
//
// H264EN_CH_PROG[n] H264 Encoding Path channel n is progressive
// 1 Progressive (Not valid for TW5864)
// 0 Interlaced (TW5864 default)
//
pub const TW5864_H264EN_CH_PROG: c_uint = 0x900c;
//
// [3:0] H264EN_BUS_MAX_CH[n]
// H264 Encoding Path maximum number of channel on BUS n
// 0 Max 4 channels
// 1 Max 2 channels
//
pub const TW5864_H264EN_BUS_MAX_CH: c_uint = 0x9010;
//
// H264EN_RATE_MAX_LINE_n H264 Encoding path Rate Mapping Maximum Line Number
// on Bus n
//
pub const TW5864_H264EN_RATE_MAX_LINE_EVEN: c_uint = 0x1f;
pub const TW5864_H264EN_RATE_MAX_LINE_ODD_SHIFT: c_int = 5;

//
// [4:0] H264EN_RATE_MAX_LINE_0
// [9:5] H264EN_RATE_MAX_LINE_1
//
pub const TW5864_H264EN_RATE_MAX_LINE_REG1: c_uint = 0x9014;
//
// [4:0] H264EN_RATE_MAX_LINE_2
// [9:5] H264EN_RATE_MAX_LINE_3
//
pub const TW5864_H264EN_RATE_MAX_LINE_REG2: c_uint = 0x9018;
//
// H264EN_CHn_FMT H264 Encoding Path Format configuration of Channel n
// 00 D1 (For D1 and hD1 frame)
// 01 (Reserved)
// 10 (Reserved)
// 11 D1 with 1/2 size in X (for CIF frame)
// Note: To be used with 0x9008 register to configure the frame size
//
// [1:0]: H264EN_CH0_FMT,
// ..., [15:14]: H264EN_CH7_FMT
//
pub const TW5864_H264EN_CH_FMT_REG1: c_uint = 0x9020;
//
// [1:0]: H264EN_CH8_FMT (?),
// ..., [15:14]: H264EN_CH15_FMT (?)
//
pub const TW5864_H264EN_CH_FMT_REG2: c_uint = 0x9024;
//
// H264EN_RATE_CNTL_BUSm_CHn H264 Encoding Path BUS m Rate Control for Channel n
//

//
// H264EN_BUSm_MAP_CHn The 16-to-1 MUX configuration register for each encoding
// channel (total of 16 channels). Four bits for each channel.
//
pub const TW5864_H264EN_BUS0_MAP: c_uint = 0x9200;
pub const TW5864_H264EN_BUS1_MAP: c_uint = 0x9204;
pub const TW5864_H264EN_BUS2_MAP: c_uint = 0x9208;
pub const TW5864_H264EN_BUS3_MAP: c_uint = 0x920c;
// This register is not defined in datasheet, but used in reference driver
pub const TW5864_UNDECLARED_ERROR_FLAGS_0x9218: c_uint = 0x9218;
pub const TW5864_GPIO1: c_uint = 0x9800;
pub const TW5864_GPIO2: c_uint = 0x9804;
// Define controls in registers TW5864_GPIO1, TW5864_GPIO2
// GPIO DATA of Group n
pub const TW5864_GPIO_DATA: c_uint = 0x00ff;
pub const TW5864_GPIO_OEN_SHIFT: c_int = 8;
// GPIO Output Enable of Group n

// 0xa000 ~ 0xa8ff - DDR Controller Register Map
// DDR Controller A
//
// [2:0] Data valid counter after read command to DDR. This is the delay value
// to show how many cycles the data will be back from DDR after we issue a read
// command.
//
pub const TW5864_RD_ACK_VLD_MUX: c_uint = 0xa000;
pub const TW5864_DDR_PERIODS: c_uint = 0xa004;
// Define controls in register TW5864_DDR_PERIODS
//
// Tras value, the minimum cycle of active to precharge command period,
// default is 7
//
pub const TW5864_TRAS_CNT_MAX: c_uint = 0x000f;
//
// Trfc value, the minimum cycle of refresh to active or refresh command period,
// default is 4"hf
//
pub const TW5864_RFC_CNT_MAX_SHIFT: c_int = 8;

//
// Trcd value, the minimum cycle of active to internal read/write command
// period, default is 4"h2
//
pub const TW5864_TCD_CNT_MAX_SHIFT: c_int = 4;

// Twr value, write recovery time, default is 4"h3
pub const TW5864_TWR_CNT_MAX_SHIFT: c_int = 12;

//
// [2:0] CAS latency, the delay cycle between internal read command and the
// availability of the first bit of output data, default is 3
//
pub const TW5864_CAS_LATENCY: c_uint = 0xa008;
//
// [15:0] Maximum average periodic refresh, the value is based on the current
// frequency to match 7.8mcs
//
pub const TW5864_DDR_REF_CNTR_MAX: c_uint = 0xa00c;
//
// DDR_ON_CHIP_MAP [1:0]
// 0 256M DDR on board
// 1 512M DDR on board
// 2 1G DDR on board
// DDR_ON_CHIP_MAP [2]
// 0 Only one DDR chip
// 1 Two DDR chips
//
pub const TW5864_DDR_ON_CHIP_MAP: c_uint = 0xa01c;
pub const TW5864_DDR_SELFTEST_MODE: c_uint = 0xa020;
// Define controls in register TW5864_DDR_SELFTEST_MODE
//
// 0 Common read/write mode
// 1 DDR self-test mode
//

//
// 0 DDR self-test single read/write
// 1 DDR self-test burst read/write
//

//
// 0 DDR self-test write command
// 1 DDR self-test read command
//

pub const TW5864_DATA_MODE_SHIFT: c_int = 4;
//
// 0 write 32'haaaa5555 to DDR
// 1 write 32'hffffffff to DDR
// 2 write 32'hha5a55a5a to DDR
// 3 write increasing data to DDR
//

// [7:0] The maximum data of one burst in DDR self-test mode
pub const TW5864_BURST_CNTR_MAX: c_uint = 0xa024;
// [15:0] The maximum burst counter (bit 15~0) in DDR self-test mode
pub const TW5864_DDR_PROC_CNTR_MAX_L: c_uint = 0xa028;
// The maximum burst counter (bit 31~16) in DDR self-test mode
pub const TW5864_DDR_PROC_CNTR_MAX_H: c_uint = 0xa02c;
// [0]: Start one DDR self-test
pub const TW5864_DDR_SELF_TEST_CMD: c_uint = 0xa030;
// The maximum error counter (bit 15 ~ 0) in DDR self-test
pub const TW5864_ERR_CNTR_L: c_uint = 0xa034;
pub const TW5864_ERR_CNTR_H_AND_FLAG: c_uint = 0xa038;
// Define controls in register TW5864_ERR_CNTR_H_AND_FLAG
// The maximum error counter (bit 30 ~ 16) in DDR self-test
pub const TW5864_ERR_CNTR_H_MASK: c_uint = 0x3fff;
// DDR self-test end flag
pub const TW5864_END_FLAG: c_uint = 0x8000;
//
// DDR Controller B: same as 0xa000 ~ 0xa038, but add TW5864_DDR_B_OFFSET to all
// addresses
//
pub const TW5864_DDR_B_OFFSET: c_uint = 0x0800;
// 0xb004 ~ 0xb018 - HW version/ARB12 Register Map
// [15:0] Default is C013
pub const TW5864_HW_VERSION: c_uint = 0xb004;
pub const TW5864_REQS_ENABLE: c_uint = 0xb010;
// Define controls in register TW5864_REQS_ENABLE
// Audio data in to DDR enable (default 1)

// Audio encode request to DDR enable (default 1)

// Audio decode request0 to DDR enable (default 1)

// Audio decode request1 to DDR enable (default 1)

// VLC stream request to DDR enable (default 1)

// H264 MV request to DDR enable (default 1)

// mux_core MVD request to DDR enable (default 1)

// mux_core MVD temp data request to DDR enable (default 1)

// JPEG request to DDR enable (default 1)

// mv_flag request to DDR enable (default 1)

pub const TW5864_ARB12: c_uint = 0xb018;
// Define controls in register TW5864_ARB12
// ARB12 Enable (default 1)

// ARB12 maximum value of time out counter (default 15"h1FF)
pub const TW5864_ARB12_TIME_OUT_CNT: c_uint = 0x7fff;
// 0xb800 ~ 0xb80c - Indirect Access Register Map
//
// Spec says:
// In order to access the indirect register space, the following procedure is
// followed.
// But reference driver implementation, and current driver, too, does it
// differently.
//
// Write Registers:
// (1) Write IND_DATA at 0xb804 ~ 0xb807
// (2) Read BUSY flag from 0xb803. Wait until BUSY signal is 0.
// (3) Write IND_ADDR at 0xb800 ~ 0xb801. Set R/W to "1", ENABLE to "1"
// Read Registers:
// (1) Read BUSY flag from 0xb803. Wait until BUSY signal is 0.
// (2) Write IND_ADDR at 0xb800 ~ 0xb801. Set R/W to "0", ENABLE to "1"
// (3) Read BUSY flag from 0xb803. Wait until BUSY signal is 0.
// (4) Read IND_DATA from 0xb804 ~ 0xb807
//
pub const TW5864_IND_CTL: c_uint = 0xb800;
// Define controls in register TW5864_IND_CTL
// Address used to access indirect register space
pub const TW5864_IND_ADDR: c_uint = 0x0000ffff;
// Wait until this bit is "0" before using indirect access

// Activate the indirect access. This bit is self cleared

// Read/Write command

// [31:0] Data used to read/write indirect register space
pub const TW5864_IND_DATA: c_uint = 0xb804;
// 0xc000 ~ 0xc7fc - Preview Register Map
// Mostly skipped this section.
//
// [15:0] Status of Vsync Synchronized PCI_PV_CH_EN (Read Only)
// 1 Channel Enabled
// 0 Channel Disabled
//
pub const TW5864_PCI_PV_CH_STATUS: c_uint = 0xc000;
//
// [15:0] PCI Preview Path Enable for channel n
// 1 Channel Enable
// 0 Channel Disable
//
pub const TW5864_PCI_PV_CH_EN: c_uint = 0xc004;
// 0xc800 ~ 0xc804 - JPEG Capture Register Map
// Skipped.
// 0xd000 ~ 0xd0fc - JPEG Control Register Map
// Skipped.
// 0xe000 ~ 0xfc04 - Motion Vector Register Map
// ME Motion Vector data (Four Byte Each) 0xe000 ~ 0xe7fc
pub const TW5864_ME_MV_VEC_START: c_uint = 0xe000;
pub const TW5864_ME_MV_VEC_MAX_OFFSET: c_uint = 0x1ff;

pub const TW5864_MV: c_uint = 0xfc00;
// Define controls in register TW5864_MV
// mv bank0 full status , write "1" to clear

// mv bank1 full status , write "1" to clear

// slice end status; write "1" to clear

// mv encode interrupt status; write "1" to clear

// mv write memory overflow, write "1" to clear

pub const TW5864_MV_LEN_SHIFT: c_int = 5;
// mv stream length

// The configured status bit written into bit 15 of 0xfc04

pub const TW5864_MPI_DDR_SEL_REG: c_uint = 0xfc04;
// Define controls in register TW5864_MPI_DDR_SEL_REG
//
// SW configure register
// 0 MV is saved in internal DPR
// 1 MV is saved in DDR
//

// 0x18000 ~ 0x181fc - PCI Master/Slave Control Map
pub const TW5864_PCI_INTR_STATUS: c_uint = 0x18000;
// Define controls in register TW5864_PCI_INTR_STATUS
// vlc done

// ad vsync

// preview eof

// preview overflow interrupt

// timer interrupt

// audio eof

// IIC done

// ad interrupt (e.g.: video lost, video format changed)

pub const TW5864_PCI_INTR_CTL: c_uint = 0x18004;
// Define controls in register TW5864_PCI_INTR_CTL
// master enable

// mvd&vlc master enable
pub const TW5864_MVD_VLC_MAST_ENB: c_uint = 0x06;
// (Need to set 0 in TW5864A)

// preview master enable

// preview overflow enable

// timer interrupt enable

// JPEG master (push mode) enable

pub const TW5864_AU_MAST_ENB_CHN_SHIFT: c_int = 8;
// audio master channel enable

// IIC interrupt enable

// ad interrupt enable

// target burst enable

// vlc stream burst enable

// ddr burst enable (1 enable, and must set DDR_BRST_EN)

//
// Because preview and audio have 16 channels separately, so using this
// registers to indicate interrupt status for every channels. This is secondary
// interrupt status register. OR operating of the PREV_INTR_REG is
// PREV_EOF_INTR, OR operating of the AU_INTR_REG bits is AUDIO_EOF_INTR
//
pub const TW5864_PREV_AND_AU_INTR: c_uint = 0x18008;
// Define controls in register TW5864_PREV_AND_AU_INTR
// preview eof interrupt flag
pub const TW5864_PREV_INTR_REG: c_uint = 0x0000ffff;
pub const TW5864_AU_INTR_REG_SHIFT: c_int = 16;
// audio eof interrupt flag

pub const TW5864_MASTER_ENB_REG: c_uint = 0x1800c;
// Define controls in register TW5864_MASTER_ENB_REG
// master enable

// mvd and vlc master enable

// ad vsync master enable

// jpeg master enable

// preview master enable

//
// Every channel of preview and audio have ping-pong buffers in system memory,
// this register is the buffer flag to notify software which buffer is been
// operated.
//
pub const TW5864_PREV_AND_AU_BUF_FLAG: c_uint = 0x18010;
// Define controls in register TW5864_PREV_AND_AU_BUF_FLAG
// preview buffer A/B flag
pub const TW5864_PREV_BUF_FLAG: c_uint = 0xffff;
pub const TW5864_AUDIO_BUF_FLAG_SHIFT: c_int = 16;
// audio buffer A/B flag

pub const TW5864_IIC: c_uint = 0x18014;
// Define controls in register TW5864_IIC
// register data
pub const TW5864_IIC_DATA: c_uint = 0x00ff;
pub const TW5864_IIC_REG_ADDR_SHIFT: c_int = 8;
// register addr

// rd/wr flag rd=1,wr=0

pub const TW5864_IIC_DEV_ADDR_SHIFT: c_int = 17;
// device addr

//
// iic done, software kick off one time iic transaction through setting this
// bit to 1. Then poll this bit, value 1 indicate iic transaction have
// completed, if read, valid data have been stored in iic_data
//

pub const TW5864_RST_AND_IF_INFO: c_uint = 0x18018;
// Define controls in register TW5864_RST_AND_IF_INFO
// application software soft reset

pub const TW5864_PCI_INF_VERSION_SHIFT: c_int = 16;
// PCI interface version, read only

// vlc stream crc value, it is calculated in pci module
pub const TW5864_VLC_CRC_REG: c_uint = 0x1801c;
//
// vlc max length, it is defined by software based on software assign memory
// space for vlc
//
pub const TW5864_VLC_MAX_LENGTH: c_uint = 0x18020;
// vlc length of one frame
pub const TW5864_VLC_LENGTH: c_uint = 0x18024;
// vlc original crc value
pub const TW5864_VLC_INTRA_CRC_I_REG: c_uint = 0x18028;
// vlc original crc value
pub const TW5864_VLC_INTRA_CRC_O_REG: c_uint = 0x1802c;
// mv stream crc value, it is calculated in pci module
pub const TW5864_VLC_PAR_CRC_REG: c_uint = 0x18030;
// mv length
pub const TW5864_VLC_PAR_LENGTH_REG: c_uint = 0x18034;
// mv original crc value
pub const TW5864_VLC_PAR_I_REG: c_uint = 0x18038;
// mv original crc value
pub const TW5864_VLC_PAR_O_REG: c_uint = 0x1803c;
//
// Configuration register for 9[or 10] CIFs or 1D1+15QCIF Preview mode.
// PREV_PCI_ENB_CHN[0] Enable 9th preview channel (9CIF prev) or 1D1 channel in
// (1D1+15QCIF prev)
// PREV_PCI_ENB_CHN[1] Enable 10th preview channel
//
pub const TW5864_PREV_PCI_ENB_CHN: c_uint = 0x18040;
// Description skipped.
pub const TW5864_PREV_FRAME_FORMAT_IN: c_uint = 0x18044;
// IIC enable
pub const TW5864_IIC_ENB: c_uint = 0x18048;
//
// Timer interrupt interval
// 0 1ms
// 1 2ms
// 2 4ms
// 3 8ms
//
pub const TW5864_PCI_INTTM_SCALE: c_uint = 0x1804c;
//
// The above register is pci base address registers. Application software will
// initialize them to tell chip where the corresponding stream will be dumped
// to. Application software will select appropriate base address interval based
// on the stream length.
//
// VLC stream base address
pub const TW5864_VLC_STREAM_BASE_ADDR: c_uint = 0x18080;
// MV stream base address
pub const TW5864_MV_STREAM_BASE_ADDR: c_uint = 0x18084;
// 0x180a0 ~ 0x180bc: audio burst base address. Skipped.
// 0x180c0 ~ 0x180dc: JPEG Push Mode Buffer Base Address. Skipped.
// 0x18100 ~ 0x1817c: preview burst base address. Skipped.
// 0x80000 ~ 0x87fff - DDR Burst RW Register Map
pub const TW5864_DDR_CTL: c_uint = 0x80000;
// Define controls in register TW5864_DDR_CTL
pub const TW5864_BRST_LENGTH_SHIFT: c_int = 2;
// Length of 32-bit data burst

//
// Burst Read/Write
// 0 Read Burst from DDR
// 1 Write Burst to DDR
//

// Begin a new DDR Burst. This bit is self cleared

// DDR Burst End Flag

// Enable Error Interrupt for Single DDR Access

// Enable Error Interrupt for Burst DDR Access

// Enable Interrupt for End of DDR Burst Access

// DDR Single Access Error Flag

// DDR Single Access Busy Flag

// DDR Burst Access Error Flag

// DDR Burst Access Busy Flag

// [27:0] DDR Access Address. Bit [1:0] has to be 0
pub const TW5864_DDR_ADDR: c_uint = 0x80004;
// DDR Access Internal Buffer Address. Bit [1:0] has to be 0
pub const TW5864_DPR_BUF_ADDR: c_uint = 0x80008;
// SRAM Buffer MPI Access Space. Totally 16 KB
pub const TW5864_DPR_BUF_START: c_uint = 0x84000;
// 0x84000 - 0x87ffc
pub const TW5864_DPR_BUF_SIZE: c_uint = 0x4000;
// Indirect Map Space
//
// The indirect space is accessed through 0xb800 ~ 0xb807 registers in direct
// access space
//
// Analog Video / Audio Decoder / Encoder
// Allowed channel values: [0; 3]
// Read-only register

// Define controls in register TW5864_INDIR_VIN_0
//
// 1 Video not present. (sync is not detected in number of consecutive line
// periods specified by MISSCNT register)
// 0 Video detected.
//

//
// 1 Horizontal sync PLL is locked to the incoming video source.
// 0 Horizontal sync PLL is not locked.
//

//
// 1 Sub-carrier PLL is locked to the incoming video source.
// 0 Sub-carrier PLL is not locked.
//

//
// 1 Even field is being decoded.
// 0 Odd field is being decoded.
//

//
// 1 Vertical logic is locked to the incoming video source.
// 0 Vertical logic is not locked.
//

//
// 1 No color burst signal detected.
// 0 Color burst signal detected.
//

//
// 0 60Hz source detected
// 1 50Hz source detected
// The actual vertical scanning frequency depends on the current standard
// invoked.
//

// VCR signal indicator. Read-only.

// Weak signal indicator 2. Read-only.

// Weak signal indicator controlled by WKTH. Read-only.

//
// 1 = Standard signal
// 0 = Non-standard signal
// Read-only
//

//
// 1 = Non-interlaced signal
// 0 = interlaced signal
// Read-only
//

//
// Vertical Sharpness Control. Writable.
// 0 = None (default)
// 7 = Highest
// **Note: VSHP must be set to '0' if COMB = 0
//
pub const TW5864_INDIR_VIN_1_VSHP: c_uint = 0x07;
// HDELAY_XY[7:0]

// HACTIVE_XY[7:0]

// VDELAY_XY[7:0]

// VACTIVE_XY[7:0]

// Define controls in register TW5864_INDIR_VIN_6
pub const TW5864_INDIR_VIN_6_HDELAY_XY_HI: c_uint = 0x03;
pub const TW5864_INDIR_VIN_6_HACTIVE_XY_HI_SHIFT: c_int = 2;

//
// HDELAY_XY This 10bit register defines the starting location of horizontal
// active pixel for display / record path. A unit is 1 pixel. The default value
// is 0x00f for NTSC and 0x00a for PAL.
//
// HACTIVE_XY This 10bit register defines the number of horizontal active pixel
// for display / record path. A unit is 1 pixel. The default value is decimal
// 720.
//
// VDELAY_XY This 9bit register defines the starting location of vertical
// active for display / record path. A unit is 1 line. The default value is
// decimal 6.
//
// VACTIVE_XY This 9bit register defines the number of vertical active lines
// for display / record path. A unit is 1 line. The default value is decimal
// 240.
//
// HUE These bits control the color hue as 2's complement number. They have
// value from +36o (7Fh) to -36o (80h) with an increment of 2.8o. The 2 LSB has
// no effect. The positive value gives greenish tone and negative value gives
// purplish tone. The default value is 0o (00h). This is effective only on NTSC
// system. The default is 00h.
//

// Define controls in register TW5864_INDIR_VIN_8
//
// This bit controls the center frequency of the peaking filter.
// The corresponding gain adjustment is HFLT.
// 0 Low
// 1 center
//

// CTI level selection. The default is 1.
// 0 None
// 3 Highest
//
pub const TW5864_INDIR_VIN_8_CTI_SHIFT: c_int = 4;

//
// These bits control the amount of sharpness enhancement on the luminance
// signals. There are 16 levels of control with "0" having no effect on the
// output image. 1 through 15 provides sharpness enhancement with "F" being the
// strongest. The default is 1.
//
pub const TW5864_INDIR_VIN_8_SHARPNESS: c_uint = 0x0f;
//
// These bits control the luminance contrast gain. A value of 100 (64h) has a
// gain of 1. The range adjustment is from 0% to 255% at 1% per step. The
// default is 64h.
//

//
// These bits control the brightness. They have value of -128 to 127 in 2's
// complement form. Positive value increases brightness. A value 0 has no
// effect on the data. The default is 00h.
//

//
// These bits control the digital gain adjustment to the U (or Cb) component of
// the digital video signal. The color saturation can be adjusted by adjusting
// the U and V color gain components by the same amount in the normal
// situation. The U and V can also be adjusted independently to provide greater
// flexibility. The range of adjustment is 0 to 200%. A value of 128 (80h) has
// gain of 100%. The default is 80h.
//

//
// These bits control the digital gain adjustment to the V (or Cr) component of
// the digital video signal. The color saturation can be adjusted by adjusting
// the U and V color gain components by the same amount in the normal
// situation. The U and V can also be adjusted independently to provide greater
// flexibility. The range of adjustment is 0 to 200%. A value of 128 (80h) has
// gain of 100%. The default is 80h.
//

// Read-only

// Define controls in register TW5864_INDIR_VIN_D
// Macrovision color stripe detection may be un-reliable

// Macrovision AGC pulse detected

// Macrovision color stripe protection burst detected

//
// This bit is valid only when color stripe protection is detected, i.e. if
// CSTRIPE=1,
// 1 Type 2 color stripe protection
// 0 Type 3 color stripe protection
//

// Read-only

// Define controls in register TW5864_INDIR_VIN_E
//
// Read-only.
// 0 Idle
// 1 Detection in progress
//

//
// STDNOW Current standard invoked
// 0 NTSC (M)
// 1 PAL (B, D, G, H, I)
// 2 SECAM
// 3 NTSC4.43
// 4 PAL (M)
// 5 PAL (CN)
// 6 PAL 60
// 7 Not valid
//
pub const TW5864_INDIR_VIN_E_STDNOW_SHIFT: c_int = 4;

//
// 1 Disable the shadow registers
// 0 Enable VACTIVE and HDELAY shadow registers value depending on STANDARD.
// (Default)
//

//
// STANDARD Standard selection
// 0 NTSC (M)
// 1 PAL (B, D, G, H, I)
// 2 SECAM
// 3 NTSC4.43
// 4 PAL (M)
// 5 PAL (CN)
// 6 PAL 60
// 7 Auto detection (Default)
//
pub const TW5864_INDIR_VIN_E_STANDARD: c_uint = 0x07;

// Define controls in register TW5864_INDIR_VIN_F
//
// 1 Writing 1 to this bit will manually initiate the auto format detection
// process. This bit is a self-clearing bit
// 0 Manual initiation of auto format detection is done. (Default)
//

// Enable recognition of PAL60 (Default)

// Enable recognition of PAL (CN). (Default)

// Enable recognition of PAL (M). (Default)

// Enable recognition of NTSC 4.43. (Default)

// Enable recognition of SECAM. (Default)

// Enable recognition of PAL (B, D, G, H, I). (Default)

// Enable recognition of NTSC (M). (Default)

// Some registers skipped.
// Use falling edge to sample VD1-VD4 from 54 MHz to 108 MHz
pub const TW5864_INDIR_VD_108_POL: c_uint = 0x041;

// Some registers skipped.
//
// Audio Input ADC gain control
// 0 0.25
// 1 0.31
// 2 0.38
// 3 0.44
// 4 0.50
// 5 0.63
// 6 0.75
// 7 0.88
// 8 1.00 (default)
// 9 1.25
// 10 1.50
// 11 1.75
// 12 2.00
// 13 2.25
// 14 2.50
// 15 2.75
//
// [3:0] channel 0, [7:4] channel 1
pub const TW5864_INDIR_AIGAIN1: c_uint = 0x060;
// [3:0] channel 2, [7:4] channel 3
pub const TW5864_INDIR_AIGAIN2: c_uint = 0x061;
// Some registers skipped
pub const TW5864_INDIR_AIN_0x06D: c_uint = 0x06d;
// Define controls in register TW5864_INDIR_AIN_0x06D
//
// LAWMD Select u-Law/A-Law/PCM/SB data output format on ADATR and ADATM pin.
// 0 PCM output (default)
// 1 SB (Signed MSB bit in PCM data is inverted) output
// 2 u-Law output
// 3 A-Law output
//
pub const TW5864_INDIR_AIN_LAWMD_SHIFT: c_int = 6;

//
// Disable the mixing ratio value for all audio.
// 0 Apply individual mixing ratio value for each audio (default)
// 1 Apply nominal value for all audio commonly
//

//
// Enable the mute function for audio channel AINn when n is 0 to 3. It effects
// only for mixing. When n = 4, it enable the mute function of the playback
// audio input. It effects only for single chip or the last stage chip
// 0 Normal
// 1 Muted (default)
//
pub const TW5864_INDIR_AIN_MIX_MUTE: c_uint = 0x1f;
// Some registers skipped
pub const TW5864_INDIR_AIN_0x0E3: c_uint = 0x0e3;
// Define controls in register TW5864_INDIR_AIN_0x0E3
//
// ADATP signal is coming from external ADPCM decoder, instead of on-chip ADPCM
// decoder
//

// ACLKP output signal polarity inverse

//
// ACLKR input signal polarity inverse.
// 0 Not inversed (Default)
// 1 Inversed
//

//
// ACLKP input signal polarity inverse.
// 0 Not inversed (Default)
// 1 Inversed
//

//
// ACKI [21:0] control automatic set up with AFMD registers
// This mode is only effective when ACLKRMASTER=1
// 0 ACKI [21:0] registers set up ACKI control
// 1 ACKI control is automatically set up by AFMD register values
//

//
// AFAUTO control mode
// 0 8kHz setting (Default)
// 1 16kHz setting
// 2 32kHz setting
// 3 44.1kHz setting
// 4 48kHz setting
//
pub const TW5864_INDIR_AIN_0x0E3_AFMD: c_uint = 0x07;
pub const TW5864_INDIR_AIN_0x0E4: c_uint = 0x0e4;
// Define controls in register TW5864_INDIR_AIN_0x0ED
//
// 8bit I2S Record output mode.
// 0 L/R half length separated output (Default).
// 1 One continuous packed output equal to DSP output format.
//

//
// Audio Clock Master ACLKR output wave format.
// 0 High periods is one 27MHz clock period (default).
// 1 Almost duty 50-50% clock output on ACLKR pin. If this mode is selected, two
// times bigger number value need to be set up on the ACKI register. If
// AFAUTO=1, ACKI control is automatically set up even if MASCKMD=1.
//

// Playback ACLKP/ASYNP/ADATP input data MSB-LSB swapping

//
// ASYNR input signal delay.
// 0 No delay
// 1 Add one 27MHz period delay in ASYNR signal input
//

//
// ASYNP input signal delay.
// 0 no delay
// 1 add one 27MHz period delay in ASYNP signal input
//

//
// ADATP input data delay by one ACLKP clock.
// 0 No delay (Default). This is for I2S type 1T delay input interface.
// 1 Add 1 ACLKP clock delay in ADATP input data. This is for left-justified
// type 0T delay input interface.
//

//
// Select u-Law/A-Law/PCM/SB data input format on ADATP pin.
// 0 PCM input (Default)
// 1 SB (Signed MSB bit in PCM data is inverted) input
// 2 u-Law input
// 3 A-Law input
//
pub const TW5864_INDIR_AIN_0x0E4_INLAWMD: c_uint = 0x03;
//
// Enable state register updating and interrupt request of audio AIN5 detection
// for each input
//
pub const TW5864_INDIR_AIN_A5DETENA: c_uint = 0x0e5;
// Some registers skipped
//
// [7:3]: DEV_ID The TW5864 product ID code is 01000
// [2:0]: REV_ID The revision number is 0h
//
pub const TW5864_INDIR_ID: c_uint = 0x0fe;

// Some registers skipped
pub const TW5864_INDIR_CROP_ETC: c_uint = 0x260;
// Define controls in register TW5864_INDIR_CROP_ETC
// Enable cropping from 720 to 704
pub const TW5864_INDIR_CROP_ETC_CROP_EN: c_uint = 0x4;
//
// Interrupt status register from the front-end. Write "1" to each bit to clear
// the interrupt
// 15:0 Motion detection interrupt for channel 0 ~ 15
// 31:16 Night detection interrupt for channel 0 ~ 15
// 47:32 Blind detection interrupt for channel 0 ~ 15
// 63:48 No video interrupt for channel 0 ~ 15
// 79:64 Line mode underflow interrupt for channel 0 ~ 15
// 95:80 Line mode overflow interrupt for channel 0 ~ 15
//
// 0x2d0~0x2d7: [63:0] bits
pub const TW5864_INDIR_INTERRUPT1: c_uint = 0x2d0;
// 0x2e0~0x2e3: [95:64] bits
pub const TW5864_INDIR_INTERRUPT2: c_uint = 0x2e0;
//
// Interrupt mask register for interrupts in 0x2d0 ~ 0x2d7
// 15:0 Motion detection interrupt for channel 0 ~ 15
// 31:16 Night detection interrupt for channel 0 ~ 15
// 47:32 Blind detection interrupt for channel 0 ~ 15
// 63:48 No video interrupt for channel 0 ~ 15
// 79:64 Line mode underflow interrupt for channel 0 ~ 15
// 95:80 Line mode overflow interrupt for channel 0 ~ 15
//
// 0x2d8~0x2df: [63:0] bits
pub const TW5864_INDIR_INTERRUPT_MASK1: c_uint = 0x2d8;
// 0x2e8~0x2eb: [95:64] bits
pub const TW5864_INDIR_INTERRUPT_MASK2: c_uint = 0x2e8;
// [11:0]: Interrupt summary register for interrupts & interrupt mask from in
// 0x2d0 ~ 0x2d7 and 0x2d8 ~ 0x2df
// bit 0: interrupt occurs in 0x2d0 & 0x2d8
// bit 1: interrupt occurs in 0x2d1 & 0x2d9
// bit 2: interrupt occurs in 0x2d2 & 0x2da
// bit 3: interrupt occurs in 0x2d3 & 0x2db
// bit 4: interrupt occurs in 0x2d4 & 0x2dc
// bit 5: interrupt occurs in 0x2d5 & 0x2dd
// bit 6: interrupt occurs in 0x2d6 & 0x2de
// bit 7: interrupt occurs in 0x2d7 & 0x2df
// bit 8: interrupt occurs in 0x2e0 & 0x2e8
// bit 9: interrupt occurs in 0x2e1 & 0x2e9
// bit 10: interrupt occurs in 0x2e2 & 0x2ea
// bit 11: interrupt occurs in 0x2e3 & 0x2eb
//
pub const TW5864_INDIR_INTERRUPT_SUMMARY: c_uint = 0x2f0;
// Motion / Blind / Night Detection
// valid value for channel is [0:15]

// Define controls in register TW5864_INDIR_DETECTION_CTL0
//
// Disable the motion and blind detection.
// 0 Enable motion and blind detection (default)
// 1 Disable motion and blind detection
//

//
// Request to start motion detection on manual trigger mode
// 0 None Operation (default)
// 1 Request to start motion detection
//

//
// Select the trigger mode of motion detection
// 0 Automatic trigger mode of motion detection (default)
// 1 Manual trigger mode for motion detection
//

//
// Define the threshold of cell for blind detection.
// 0 Low threshold (More sensitive) (default)
// : :
// 3 High threshold (Less sensitive)
//
pub const TW5864_INDIR_DETECTION_CTL0_BD_CELSENS: c_uint = 0x03;

// Define controls in register TW5864_INDIR_DETECTION_CTL1
//
// Control the temporal sensitivity of motion detector.
// 0 More Sensitive (default)
// : :
// 15 Less Sensitive
//
pub const TW5864_INDIR_DETECTION_CTL1_MD_TMPSENS_SHIFT: c_int = 4;

//
// Adjust the horizontal starting position for motion detection
// 0 0 pixel (default)
// : :
// 15 15 pixels
//
pub const TW5864_INDIR_DETECTION_CTL1_MD_PIXEL_OS: c_uint = 0x0f;

// Define controls in register TW5864_INDIR_DETECTION_CTL2
//
// Control the updating time of reference field for motion detection.
// 0 Update reference field every field (default)
// 1 Update reference field according to MD_SPEED
//

//
// Select the field for motion detection.
// 0 Detecting motion for only odd field (default)
// 1 Detecting motion for only even field
// 2 Detecting motion for any field
// 3 Detecting motion for both odd and even field
//
pub const TW5864_INDIR_DETECTION_CTL2_MD_FIELD_SHIFT: c_int = 5;

//
// Control the level sensitivity of motion detector.
// 0 More sensitive (default)
// : :
// 15 Less sensitive
//
pub const TW5864_INDIR_DETECTION_CTL2_MD_LVSENS: c_uint = 0x1f;

// Define controls in register TW5864_INDIR_DETECTION_CTL3
//
// Define the threshold of sub-cell number for motion detection.
// 0 Motion is detected if 1 sub-cell has motion (More sensitive) (default)
// 1 Motion is detected if 2 sub-cells have motion
// 2 Motion is detected if 3 sub-cells have motion
// 3 Motion is detected if 4 sub-cells have motion (Less sensitive)
//
pub const TW5864_INDIR_DETECTION_CTL3_MD_CELSENS_SHIFT: c_int = 6;

//
// Control the velocity of motion detector.
// Large value is suitable for slow motion detection.
// In MD_DUAL_EN = 1, MD_SPEED should be limited to 0 ~ 31.
// 0 1 field intervals (default)
// 1 2 field intervals
// : :
// 61 62 field intervals
// 62 63 field intervals
// 63 Not supported
//
pub const TW5864_INDIR_DETECTION_CTL3_MD_SPEED: c_uint = 0x3f;

// Define controls in register TW5864_INDIR_DETECTION_CTL4
//
// Control the spatial sensitivity of motion detector.
// 0 More Sensitive (default)
// : :
// 15 Less Sensitive
//
pub const TW5864_INDIR_DETECTION_CTL4_MD_SPSENS_SHIFT: c_int = 4;

//
// Define the threshold of level for blind detection.
// 0 Low threshold (More sensitive) (default)
// : :
// 15 High threshold (Less sensitive)
//
pub const TW5864_INDIR_DETECTION_CTL4_BD_LVSENS: c_uint = 0x0f;

//
// Define the threshold of temporal sensitivity for night detection.
// 0 Low threshold (More sensitive) (default)
// : :
// 15 High threshold (Less sensitive)
//
pub const TW5864_INDIR_DETECTION_CTL5_ND_TMPSENS_SHIFT: c_int = 4;

//
// Define the threshold of level for night detection.
// 0 Low threshold (More sensitive) (default)
// : :
// 3 High threshold (Less sensitive)
//
pub const TW5864_INDIR_DETECTION_CTL5_ND_LVSENS: c_uint = 0x0f;
//
// [11:0] The base address of the motion detection buffer. This address is in
// unit of 64K bytes. The generated DDR address will be {MD_BASE_ADDR,
// 16"h0000}. The default value should be 12"h000
//
pub const TW5864_INDIR_MD_BASE_ADDR: c_uint = 0x380;
//
// This controls the channel of the motion detection result shown in register
// 0x3a0 ~ 0x3b7. Before reading back motion result, always set this first.
//
pub const TW5864_INDIR_RGR_MOTION_SEL: c_uint = 0x382;
// [15:0] MD strobe has been performed at channel n (read only)
pub const TW5864_INDIR_MD_STRB: c_uint = 0x386;
// NO_VIDEO Detected from channel n (read only)
pub const TW5864_INDIR_NOVID_DET: c_uint = 0x388;
// Motion Detected from channel n (read only)
pub const TW5864_INDIR_MD_DET: c_uint = 0x38a;
// Blind Detected from channel n (read only)
pub const TW5864_INDIR_BD_DET: c_uint = 0x38c;
// Night Detected from channel n (read only)
pub const TW5864_INDIR_ND_DET: c_uint = 0x38e;
// 192 bit motion flag of the channel specified by RGR_MOTION_SEL in 0x382
pub const TW5864_INDIR_MOTION_FLAG: c_uint = 0x3a0;
pub const TW5864_INDIR_MOTION_FLAG_BYTE_COUNT: c_int = 24;
//
// [9:0] The motion cell count of a specific channel selected by 0x382. This is
// for DI purpose
//
pub const TW5864_INDIR_MD_DI_CNT: c_uint = 0x3b8;
// The motion detection cell sensitivity for DI purpose
pub const TW5864_INDIR_MD_DI_CELLSENS: c_uint = 0x3ba;
// The motion detection threshold level for DI purpose
pub const TW5864_INDIR_MD_DI_LVSENS: c_uint = 0x3bb;
// 192 bit motion mask of the channel specified by MASK_CH_SEL in 0x3fe
pub const TW5864_INDIR_MOTION_MASK: c_uint = 0x3e0;
pub const TW5864_INDIR_MOTION_MASK_BYTE_COUNT: c_int = 24;
// [4:0] The channel selection to access masks in 0x3e0 ~ 0x3f7
pub const TW5864_INDIR_MASK_CH_SEL: c_uint = 0x3fe;
// Clock PLL / Analog IP Control
// Some registers skipped
pub const TW5864_INDIR_DDRA_DLL_DQS_SEL0: c_uint = 0xee6;
pub const TW5864_INDIR_DDRA_DLL_DQS_SEL1: c_uint = 0xee7;
pub const TW5864_INDIR_DDRA_DLL_CLK90_SEL: c_uint = 0xee8;
pub const TW5864_INDIR_DDRA_DLL_TEST_SEL_AND_TAP_S: c_uint = 0xee9;
pub const TW5864_INDIR_DDRB_DLL_DQS_SEL0: c_uint = 0xeeb;
pub const TW5864_INDIR_DDRB_DLL_DQS_SEL1: c_uint = 0xeec;
pub const TW5864_INDIR_DDRB_DLL_CLK90_SEL: c_uint = 0xeed;
pub const TW5864_INDIR_DDRB_DLL_TEST_SEL_AND_TAP_S: c_uint = 0xeee;
pub const TW5864_INDIR_RESET: c_uint = 0xef0;

pub const TW5864_INDIR_PV_VD_CK_POL: c_uint = 0xefd;

pub const TW5864_INDIR_CLK0_SEL: c_uint = 0xefe;
pub const TW5864_INDIR_CLK0_SEL_VD_SHIFT: c_int = 0;
pub const TW5864_INDIR_CLK0_SEL_VD_MASK: c_uint = 0x3;
pub const TW5864_INDIR_CLK0_SEL_PV_SHIFT: c_int = 2;

pub const TW5864_INDIR_CLK0_SEL_PV2_SHIFT: c_int = 4;
