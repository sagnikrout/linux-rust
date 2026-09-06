//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-fh.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014, 2018-2021, 2023-2025 Intel Corporation
// Copyright (C) 2015-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fh_h__

//
// Flow Handler Definitions
//
// This I/O area is directly read/writable by driver (e.g. Linux uses writel())
// Addresses are offsets from device's PCI hardware base address.
//

//
// Keep-Warm (KW) buffer base address.
//
// Driver must allocate a 4KByte buffer that is for keeping the
// host DRAM powered on (via dummy accesses to DRAM) to maintain low-latency
// DRAM access when doing Txing or Rxing.  The dummy accesses prevent host
// from going into a power-savings mode that would cause higher DRAM latency,
// and possible data over/under-runs, before all Tx/Rx is complete.
//
// Driver loads FH_KW_MEM_ADDR_REG with the physical address (bits 35:4)
// of the buffer, which must be 4K aligned.  Once this is set up, the device
// automatically invokes keep-warm accesses when normal accesses might not
// be sufficient to maintain fast DRAM response.
//
// Bit fields:
// 31-0:  Keep-warm buffer physical base address [35:4], must be 4K aligned
//

//
// TFD Circular Buffers Base (CBBC) addresses
//
// Device has 16 base pointer registers, one for each of 16 host-DRAM-resident
// circular buffers (CBs/queues) containing Transmit Frame Descriptors (TFDs)
// (see struct iwl_tfd_frame).  These 16 pointer registers are offset by 0x04
// bytes from one another.  Each TFD circular buffer in DRAM must be 256-byte
// aligned (address bits 0-7 must be 0).
// Later devices have 20 (5000 series) or 30 (higher) queues, but the registers
// for them are in different places.
//
// Bit fields in each pointer register:
// 27-0: TFD CB physical base address [35:8], must be 256-byte aligned
//

// 22000 TFD table address, 64 bit

// Find TFD CB base pointer for given queue
// 22000 configuration registers
//
// TFH Configuration register.
//
// BIT fields:
//
// Bits 3:0:
// Define the maximum number of pending read requests.
// Maximum configuration value allowed is 0xC
// Bits 9:8:
// Define the maximum transfer size. (64 / 128 / 256)
// Bit 10:
// When bit is set and transfer size is set to 128B, the TFH will enable
// reading chunks of more than 64B only if the read address is aligned to 128B.
// In case of DRAM read address which is not aligned to 128B, the TFH will
// enable transfer size which doesn't cross 64B DRAM address boundary.
//

pub const TFH_TRANSFER_MAX_PENDING_REQ: c_uint = 0xc;

//
// Defines the offset address in dwords referring from the beginning of the
// Tx CMD which will be updated in DRAM.
// Note that the TFH offset address for Tx CMD update is always referring to
// the start of the TFD first TB.
// In case of a DRAM Tx CMD update the TFH will update PN and Key ID
//

//
// Controls TX DMA operation
//
// BIT fields:
//
// Bits 31:30: Enable the SRAM DMA channel.
// Turning on bit 31 will kick the SRAM2DRAM DMA.
// Note that the sram2dram may be enabled only after configuring the DRAM and
// SRAM addresses registers and the byte count register.
// Bits 25:24: Defines the interrupt target upon dram2sram transfer done. When
// set to 1 - interrupt is sent to the driver
// Bit 0: Indicates the snoop configuration
//

// Defines the DMA SRAM write start address to transfer a data block

// Defines the 64bits DRAM start address to read the DMA data block from

//
// Defines the number of bytes to transfer from DRAM to SRAM.
// Note that this register may be configured with non-dword aligned size.
//

//
// Rx SRAM Control and Status Registers (RSCSR)
//
// These registers provide handshake between driver and device for the Rx queue
// (this queue handles *all* command responses, notifications, Rx data, etc.
// sent from uCode to host driver).  Unlike Tx, there is only one Rx
// queue, and only one Rx DMA/FIFO channel.  Also unlike Tx, which can
// concatenate up to 20 DRAM buffers to form a Tx frame, each Receive Buffer
// Descriptor (RBD) points to only one Rx Buffer (RB); there is a 1:1
// mapping between RBDs and RBs.
//
// Driver must allocate host DRAM memory for the following, and set the
// physical address of each into device registers:
//
// 1)  Receive Buffer Descriptor (RBD) circular buffer (CB), typically with 256
// entries (although any power of 2, up to 4096, is selectable by driver).
// Each entry (1 dword) points to a receive buffer (RB) of consistent size
// (typically 4K, although 8K or 16K are also selectable by driver).
// Driver sets up RB size and number of RBDs in the CB via Rx config
// register FH_MEM_RCSR_CHNL0_CONFIG_REG.
//
// Bit fields within one RBD:
// 27-0:  Receive Buffer physical address bits [35:8], 256-byte aligned
//
// Driver sets physical address [35:8] of base of RBD circular buffer
// into FH_RSCSR_CHNL0_RBDCB_BASE_REG [27:0].
//
// 2)  Rx status buffer, 8 bytes, in which uCode indicates which Rx Buffers
// (RBs) have been filled, via a "write pointer", actually the index of
// the RB's corresponding RBD within the circular buffer.  Driver sets
// physical address [35:4] into FH_RSCSR_CHNL0_STTS_WPTR_REG [31:0].
//
// Bit fields in lower dword of Rx status buffer (upper dword not used
// by driver:
// 31-12:  Not used by driver
// 11- 0:  Index of last filled Rx buffer descriptor
// (device writes, driver reads this value)
//
// As the driver prepares Receive Buffers (RBs) for device to fill, driver must
// enter pointers to these RBs into contiguous RBD circular buffer entries,
// and update the device's "write" index register,
// FH_RSCSR_CHNL0_RBDCB_WPTR_REG.
//
// This "write" index corresponds to the *next* RBD that the driver will make
// available, i.e. one RBD past the tail of the ready-to-fill RBDs within
// the circular buffer.  This value should initially be 0 (before preparing any
// RBs), should be 8 after preparing the first 8 RBs (for example), and must
// wrap back to 0 at the end of the circular buffer (but don't wrap before
// "read" index has advanced past 1!  See below).
// NOTE:  DEVICE EXPECTS THE WRITE INDEX TO BE INCREMENTED IN MULTIPLES OF 8.
//
// As the device fills RBs (referenced from contiguous RBDs within the circular
// buffer), it updates the Rx status buffer in host DRAM, 2) described above,
// to tell the driver the index of the latest filled RBD.  The driver must
// read this "read" index from DRAM after receiving an Rx interrupt from device
//
// The driver must also internally keep track of a third index, which is the
// next RBD to process.  When receiving an Rx interrupt, driver should process
// all filled but unprocessed RBs up to, but not including, the RB
// corresponding to the "read" index.  For example, if "read" index becomes "1",
// driver may process the RB pointed to by RBD 0.  Depending on volume of
// traffic, there may be many RBs to process.
//
// If read index == write index, device thinks there is no room to put new data.
// Due to this, the maximum number of filled RBs is 255, instead of 256.  To
// be safe, make sure that there is a gap of at least 2 RBDs between "write"
// and "read" indexes; that is, make sure that there are no more than 254
// buffers waiting to be filled.
//

//
// Physical base address of 8-byte Rx Status buffer.
// Bit fields:
// 31-0: Rx status buffer physical base address [35:4], must 16-byte aligned.
//

//
// Physical base address of Rx Buffer Descriptor Circular Buffer.
// Bit fields:
// 27-0:  RBD CD physical base address [35:8], must be 256-byte aligned.
//

//
// Rx write pointer (index, really!).
// Bit fields:
// 11-0:  Index of driver's most recent prepared-to-be-filled RBD, + 1.
// NOTE:  For 256-entry circular buffer, use only bits [7:0].
//

//
// Rx Config/Status Registers (RCSR)
// Rx Config Reg for channel 0 (only channel used)
//
// Driver must initialize FH_MEM_RCSR_CHNL0_CONFIG_REG as follows for
// normal operation (see bit fields).
//
// Clearing FH_MEM_RCSR_CHNL0_CONFIG_REG to 0 turns off Rx DMA.
// Driver should poll FH_MEM_RSSR_RX_STATUS_REG	for
// FH_RSSR_CHNL0_RX_STATUS_CHNL_IDLE (bit 24) before continuing.
//
// Bit fields:
// 31-30: Rx DMA channel enable: '00' off/pause, '01' pause at end of frame,
// '10' operate normally
// 29-24: reserved
// 23-20: # RBDs in circular buffer = 2^value; use "8" for 256 RBDs (normal),
// min "5" for 32 RBDs, max "12" for 4096 RBDs.
// 19-18: reserved
// 17-16: size of each receive buffer; '00' 4K (normal), '01' 8K,
// '10' 12K, '11' 16K.
// 15-14: reserved
// 13-12: IRQ destination; '00' none, '01' host driver (normal operation)
// 11- 4: timeout for closing Rx buffer and interrupting host (units 32 usec)
// typical value 0x10 (about 1/2 msec)
// 3- 0: reserved
//

//
// Rx Shared Status Registers (RSSR)
//
// After stopping Rx DMA channel (writing 0 to
// FH_MEM_RCSR_CHNL0_CONFIG_REG), driver must poll
// FH_MEM_RSSR_RX_STATUS_REG until Rx channel is idle.
//
// Bit fields:
// 24:  1 = Channel 0 is idle
//
// FH_MEM_RSSR_SHARED_CTRL_REG and FH_MEM_RSSR_RX_ENABLE_ERR_IRQ2DRV
// contain default values that should not be altered by the driver.
//

// Macro flag: #define FH_MEM_RSSR_RX_ENABLE_ERR_IRQ2DRV\

pub const FH_MEM_TFDIB_REG1_ADDR_BITSHIFT: c_int = 28;

// 9000 rx series registers
pub const RFH_Q0_FRBDCB_BA_LSB: c_uint = 0xA08000 /* 64 bit address */;

// Write index table
pub const RFH_Q0_FRBDCB_WIDX: c_uint = 0xA08080;

// Write index table - shadow registers
pub const RFH_Q0_FRBDCB_WIDX_TRG: c_uint = 0x1C80;

// Read index table
pub const RFH_Q0_FRBDCB_RIDX: c_uint = 0xA080C0;

// Used list table
pub const RFH_Q0_URBDCB_BA_LSB: c_uint = 0xA08100 /* 64 bit address */;

// Write index table
pub const RFH_Q0_URBDCB_WIDX: c_uint = 0xA08180;

pub const RFH_Q0_URBDCB_VAID: c_uint = 0xA081C0;

// stts
pub const RFH_Q0_URBD_STTS_WPTR_LSB: c_uint = 0xA08200 /*64 bits address */;

pub const RFH_Q0_ORB_WPTR_LSB: c_uint = 0xA08280;

pub const RFH_RBDBUF_RBD0_LSB: c_uint = 0xA08300;

//
// RFH Status Register
//
// Bit fields:
//
// Bit 29: RBD_FETCH_IDLE
// This status flag is set by the RFH when there is no active RBD fetch from
// DRAM.
// Once the RFH RBD controller starts fetching (or when there is a pending
// RBD read response from DRAM), this flag is immediately turned off.
//
// Bit 30: SRAM_DMA_IDLE
// This status flag is set by the RFH when there is no active transaction from
// SRAM to DRAM.
// Once the SRAM to DRAM DMA is active, this flag is immediately turned off.
//
// Bit 31: RXF_DMA_IDLE
// This status flag is set by the RFH when there is no active transaction from
// RXF to DRAM.
// Once the RXF-to-DRAM DMA is active, this flag is immediately turned off.
//
pub const RFH_GEN_STATUS: c_uint = 0xA09808;
pub const RFH_GEN_STATUS_AX210: c_uint = 0xA07824;

// DMA configuration
pub const RFH_RXF_DMA_CFG: c_uint = 0xA09820;
pub const RFH_RXF_DMA_CFG_AX210: c_uint = 0xA07880;
// RB size

pub const RFH_RXF_DMA_RB_SIZE_POS: c_int = 16;

// RB Circular Buffer size:defines the table sizes in RBD units

pub const RFH_RXF_DMA_RBDCB_SIZE_POS: c_int = 20;

pub const RFH_RXF_DMA_MIN_RB_SIZE_POS: c_int = 24;

pub const RFH_RXF_RXQ_ACTIVE: c_uint = 0xA0980C;
pub const RFH_GEN_CFG: c_uint = 0xA09800;

pub const RFH_GEN_CFG_RB_CHUNK_SIZE_128: c_int = 1;
pub const RFH_GEN_CFG_RB_CHUNK_SIZE_64: c_int = 0;
// the driver assumes everywhere that the default RXQ is 0
pub const RFH_GEN_CFG_DEFAULT_RXQ_NUM: c_uint = 0xF00;

// end of 9000 rx series registers
// TFDB  Area - TFDs buffer table

//
// Transmit DMA Channel Control/Status Registers (TCSR)
//
// Device has one configuration register for each of 8 Tx DMA/FIFO channels
// supported in hardware (don't confuse these with the 16 Tx queues in DRAM,
// which feed the DMA/FIFO channels); config regs are separated by 0x20 bytes.
//
// To use a Tx DMA channel, driver must initialize its
// FH_TCSR_CHNL_TX_CONFIG_REG(chnl) with:
//
// FH_TCSR_TX_CONFIG_REG_VAL_DMA_CHNL_ENABLE |
// FH_TCSR_TX_CONFIG_REG_VAL_DMA_CREDIT_ENABLE_VAL
//
// All other bits should be 0.
//
// Bit fields:
// 31-30: Tx DMA channel enable: '00' off/pause, '01' pause at end of frame,
// '10' operate normally
// 29- 4: Reserved, set to "0"
// 3: Enable internal DMA requests (1, normal operation), disable (0)
// 2- 0: Reserved, set to "0"
//

// Find Control/Status reg for given Tx DMA/FIFO channel

// TCSR: tx_config register values

//
// Tx Shared Status Registers (TSSR)
//
// After stopping Tx DMA channel (writing 0 to
// FH_TCSR_CHNL_TX_CONFIG_REG(chnl)), driver must poll
// FH_TSSR_TX_STATUS_REG until selected Tx channel is idle
// (channel's buffers empty | no pending requests).
//
// Bit fields:
// 31-24:  1 = Channel buffers empty (channel 7:0)
// 23-16:  1 = No pending requests (channel 7:0)
//

//
// Bit fields for TSSR(Tx Shared Status & Control) error status register:
// 31:  Indicates an address error when accessed to internal memory
// uCode/driver must write "1" in order to clear this flag
// 30:  Indicates that Host did not send the expected number of dwords to FH
// uCode/driver must write "1" in order to clear this flag
// 16-9:Each status bit is for one channel. Indicates that an (Error) ActDMA
// command was received from the scheduler while the TRB was already full
// with previous command
// uCode/driver must write "1" in order to clear this flag
// 7-0: Each status bit indicates a channel's TxCredit error. When an error
// bit is set, it indicates that the FH has received a full indication
// from the RTC TxFIFO and the current value of the TxCredit counter was
// not equal to zero. This mean that the credit mechanism was not
// synchronized to the TxFIFO status
// uCode/driver must write "1" in order to clear this flag
//

// Tx service channels

// Instruct FH to increment the retry count of a packet when
// it is brought from the memory to TX-FIFO
//

// cb size is the exponent

pub const RX_QUEUE_SIZE: c_int = 256;
pub const RX_QUEUE_MASK: c_int = 255;
pub const RX_QUEUE_SIZE_LOG: c_int = 8;
pub const IWL_DEFAULT_RX_QUEUE: c_int = 0;
//
// struct iwl_rb_status - reserve buffer status
// host memory mapped FH registers
// @closed_rb_num: [0:11] Indicates the index of the RB which was closed
// @closed_fr_num: [0:11] Indicates the index of the RX Frame which was closed
// @finished_rb_num: [0:11] Indicates the index of the current RB
// in which the last frame was written to
// @finished_fr_num: [0:11] Indicates the index of the RX Frame
// which was transferred
// @__spare: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rb_status {
    pub closed_rb_num: __le16,
    pub closed_fr_num: __le16,
    pub finished_rb_num: __le16,
    pub finished_fr_num: __le16,
    pub __spare: __le32,
    pub __packed: },

// cb size is the exponent - 3

pub const TFD_QUEUE_BC_SIZE_AX210: c_int = 1024;

pub const IWL_NUM_OF_TBS: c_int = 20;
pub const IWL_TFH_NUM_TBS: c_int = 25;
// IMR DMA registers
pub const IMR_TFH_SRV_DMA_CHNL0_CTRL: c_uint = 0x00a0a51c;
pub const IMR_TFH_SRV_DMA_CHNL0_SRAM_ADDR: c_uint = 0x00a0a520;
pub const IMR_TFH_SRV_DMA_CHNL0_DRAM_ADDR_LSB: c_uint = 0x00a0a524;
pub const IMR_TFH_SRV_DMA_CHNL0_DRAM_ADDR_MSB: c_uint = 0x00a0a528;
pub const IMR_TFH_SRV_DMA_CHNL0_BC: c_uint = 0x00a0a52c;
pub const TFH_SRV_DMA_CHNL0_LEFT_BC: c_uint = 0x00a0a530;
// RFH S2D DMA registers
pub const IMR_RFH_GEN_CFG_SERVICE_DMA_RS_MSK: c_uint = 0x0000000c;
pub const IMR_RFH_GEN_CFG_SERVICE_DMA_SNOOP_MSK: c_uint = 0x00000002;
// TFH D2S DMA registers
pub const IMR_UREG_CHICK_HALT_UMAC_PERMANENTLY_MSK: c_uint = 0x80000000;
pub const IMR_UREG_CHICK: c_uint = 0x00d05c00;
pub const IMR_TFH_SRV_DMA_CHNL0_CTRL_D2S_IRQ_TARGET_POS: c_uint = 0x00800000;
pub const IMR_TFH_SRV_DMA_CHNL0_CTRL_D2S_RS_MSK: c_uint = 0x00000030;
pub const IMR_TFH_SRV_DMA_CHNL0_CTRL_D2S_DMA_EN_POS: c_uint = 0x80000000;
    pub 0xF: return (sizeof(addr) > sizeof(u32) ? upper_32_bits(addr) : 0) &,
//
// enum iwl_tfd_tb_hi_n_len - TB hi_n_len bits
// @TB_HI_N_LEN_ADDR_HI_MSK: high 4 bits (to make it 36) of DMA address
// @TB_HI_N_LEN_LEN_MSK: length of the TB
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tfd_tb_hi_n_len {
    TB_HI_N_LEN_ADDR_HI_MSK	= 0xf,
    TB_HI_N_LEN_LEN_MSK	= 0xfff0,
}

//
// struct iwl_tfd_tb - transmit buffer descriptor within transmit frame descriptor
//
// This structure contains dma address and length of transmission address
//
// @lo: low [31:0] portion of the dma address of TX buffer
// every even is unaligned on 16 bit boundary
// @hi_n_len: &enum iwl_tfd_tb_hi_n_len
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tfd_tb {
    pub lo: __le32,
    pub hi_n_len: __le16,
    pub __packed: },
//
// struct iwl_tfh_tb - transmit buffer descriptor within transmit frame descriptor
//
// This structure contains dma address and length of transmission address
//
// @tb_len: length of the tx buffer
// @addr: 64 bits dma address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tfh_tb {
    pub tb_len: __le16,
    pub addr: __le64,
    pub __packed: },
//
// Each Tx queue uses a circular buffer of 256 TFDs stored in host DRAM.
// Both driver and device share these circular buffers, each of which must be
// contiguous 256 TFDs.
// For pre 22000 HW it is 256 x 128 bytes-per-TFD = 32 KBytes
// For 22000 HW and on it is 256 x 256 bytes-per-TFD = 65 KBytes
//
// Driver must indicate the physical address of the base of each
// circular buffer via the FH_MEM_CBBC_QUEUE registers.
//
// Each TFD contains pointer/size information for up to 20 / 25 data buffers
// in host DRAM.  These buffers collectively contain the (one) frame described
// by the TFD.  Each buffer must be a single contiguous block of memory within
// itself, but buffers may be scattered in host DRAM.  Each buffer has max size
// of (4K - 4).  The concatenates all of a TFD's buffers into a single
// Tx frame, up to 8 KBytes in size.
//
// A maximum of 255 (not 256!) TFDs may be on a queue waiting for Tx.
//
// struct iwl_tfd - Transmit Frame Descriptor (TFD)
// @__reserved1: reserved
// @num_tbs:
// 0-4 number of active tbs
// 5   reserved
// 6-7 padding (not used)
// @tbs: transmit frame buffer descriptors
// @__pad: padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tfd {
    pub __reserved1: [u8; 3],
    pub num_tbs: u8,
    pub tbs: [iwl_tfd_tb; IWL_NUM_OF_TBS],
    pub __pad: __le32,
    pub __packed: },
//
// struct iwl_tfh_tfd - Transmit Frame Descriptor (TFD)
// @num_tbs:
// 0-4 number of active tbs
// 5-15   reserved
// @tbs:	transmit frame buffer descriptors
// @__pad:	padding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tfh_tfd {
    pub num_tbs: __le16,
    pub tbs: [iwl_tfh_tb; IWL_TFH_NUM_TBS],
    pub __pad: __le32,
    pub __packed: },
// Keep Warm Size
pub const IWL_KW_SIZE: c_uint = 0x1000	/* 4k */;
// Fixed (non-configurable) rx data from phy
//
// struct iwl_bc_tbl_entry - scheduler byte count table entry
// base physical address provided by SCD_DRAM_BASE_ADDR
// For devices up to 22000:
// @tfd_offset:
// For devices up to 22000:
// 0-12 - tx command byte count
// 12-16 - station index
// For 22000 and on:
// 0-12 - tx command byte count
// 12-13 - number of 64 byte chunks
// 14-16 - reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bc_tbl_entry {
    pub tfd_offset: __le16,
    pub __packed: },
