//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlegacy/prph.h
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


//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// GPL LICENSE SUMMARY
//
// Copyright(c) 2005 - 2011 Intel Corporation. All rights reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of version 2 of the GNU General Public License as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110,
// USA
//
// The full GNU General Public License is included in this distribution
// in the file called LICENSE.GPL.
//
// Contact Information:
// Intel Linux Wireless <ilw@linux.intel.com>
// Intel Corporation, 5200 N.E. Elam Young Parkway, Hillsboro, OR 97124-6497
//
// BSD LICENSE
//
// Copyright(c) 2005 - 2011 Intel Corporation. All rights reserved.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
// * Neither the name Intel Corporation nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

// Macro flag: #define __il_prph_h__
//
// Registers in this file are internal, not PCI bus memory mapped.
// Driver accesses these via HBUS_TARG_PRPH_* registers.
//

// APMG (power management) constants

//
// BSM (Bootstrap State Machine)
//
// The Bootstrap State Machine (BSM) stores a short bootstrap uCode program
// in special SRAM that does not power down when the embedded control
// processor is sleeping (e.g. for periodic power-saving shutdowns of radio).
//
// When powering back up after sleeps (or during initial uCode load), the BSM
// internally loads the short bootstrap program from the special SRAM into the
// embedded processor's instruction SRAM, and starts the processor so it runs
// the bootstrap program.
//
// This bootstrap program loads (via PCI busmaster DMA) instructions and data
// images for a uCode program from host DRAM locations.  The host driver
// indicates DRAM locations and sizes for instruction and data images via the
// four BSM_DRAM_* registers.  Once the bootstrap program loads the new program,
// the new program starts automatically.
//
// The uCode used for open-source drivers includes two programs:
//
// 1)  Initialization -- performs hardware calibration and sets up some
// internal data, then notifies host via "initialize alive" notification
// (struct il_init_alive_resp) that it has completed all of its work.
// After signal from host, it then loads and starts the runtime program.
// The initialization program must be used when initially setting up the
// NIC after loading the driver.
//
// 2)  Runtime/Protocol -- performs all normal runtime operations.  This
// notifies host via "alive" notification (struct il_alive_resp) that it
// is ready to be used.
//
// When initializing the NIC, the host driver does the following procedure:
//
// 1)  Load bootstrap program (instructions only, no data image for bootstrap)
// into bootstrap memory.  Use dword writes starting at BSM_SRAM_LOWER_BOUND
//
// 2)  Point (via BSM_DRAM_*) to the "initialize" uCode data and instruction
// images in host DRAM.
//
// 3)  Set up BSM to copy from BSM SRAM into uCode instruction SRAM when asked:
// BSM_WR_MEM_SRC_REG = 0
// BSM_WR_MEM_DST_REG = RTC_INST_LOWER_BOUND
// BSM_WR_MEM_DWCOUNT_REG = # dwords in bootstrap instruction image
//
// 4)  Load bootstrap into instruction SRAM:
// BSM_WR_CTRL_REG = BSM_WR_CTRL_REG_BIT_START
//
// 5)  Wait for load completion:
// Poll BSM_WR_CTRL_REG for BSM_WR_CTRL_REG_BIT_START = 0
//
// 6)  Enable future boot loads whenever NIC's power management triggers it:
// BSM_WR_CTRL_REG = BSM_WR_CTRL_REG_BIT_START_EN
//
// 7)  Start the NIC by removing all reset bits:
// CSR_RESET = 0
//
// The bootstrap uCode (already in instruction SRAM) loads initialization
// uCode.  Initialization uCode performs data initialization, sends
// "initialize alive" notification to host, and waits for a signal from
// host to load runtime code.
//
// 4)  Point (via BSM_DRAM_*) to the "runtime" uCode data and instruction
// images in host DRAM.  The last register loaded must be the instruction
// byte count register ("1" in MSbit tells initialization uCode to load
// the runtime uCode):
// BSM_DRAM_INST_BYTECOUNT_REG = byte count | BSM_DRAM_INST_LOAD
//
// 5)  Wait for "alive" notification, then issue normal runtime commands.
//
// Data caching during power-downs:
//
// Just before the embedded controller powers down (e.g for automatic
// power-saving modes, or for RFKILL), uCode stores (via PCI busmaster DMA)
// a current snapshot of the embedded processor's data SRAM into host DRAM.
// This caches the data while the embedded processor's memory is powered down.
// Location and size are controlled by BSM_DRAM_DATA_* registers.
//
// NOTE:  Instruction SRAM does not need to be saved, since that doesn't
// change during operation; the original image (from uCode distribution
// file) can be used for reload.
//
// When powering back up, the BSM loads the bootstrap program.  Bootstrap looks
// at the BSM_DRAM_* registers, which now point to the runtime instruction
// image and the cached (modified) runtime data (*not* the initialization
// uCode).  Bootstrap reloads these runtime images into SRAM, and restarts the
// uCode from where it left off before the power-down.
//
// NOTE:  Initialization uCode does *not* run as part of the save/restore
// procedure.
//
// This save/restore method is mostly for autonomous power management during
// normal operation (result of C_POWER_TBL).  Platform suspend/resume and
// RFKILL should use complete restarts (with total re-initialization) of uCode,
// allowing total shutdown (including BSM memory).
//
// Note that, during normal operation, the host DRAM that held the initial
// startup data for the runtime code is now being used as a backup data cache
// for modified data!  If you need to completely re-initialize the NIC, make
// sure that you use the runtime data image from the uCode distribution file,
// not the modified/saved runtime data.  You may want to store a separate
// "clean" runtime data image in DRAM to avoid disk reads of distribution file.
//
// BSM bit fields

// BSM addresses

//
// Pointers and size regs for bootstrap load and data SRAM save/restore.
// NOTE:  3945 pointers use bits 31:0 of DRAM address.
// 4965 pointers use bits 35:4 of DRAM address.
//

//
// BSM special memory, stays powered on during power-save sleeps.
// Read/write, address range from LOWER_BOUND to (LOWER_BOUND + SIZE -1)
//

// 3945 Tx scheduler registers

//
// Tx Scheduler
//
// The Tx Scheduler selects the next frame to be transmitted, choosing TFDs
// (Transmit Frame Descriptors) from up to 16 circular Tx queues resident in
// host DRAM.  It steers each frame's Tx command (which contains the frame
// data) into one of up to 7 prioritized Tx DMA FIFO channels within the
// device.  A queue maps to only one (selectable by driver) Tx DMA channel,
// but one DMA channel may take input from several queues.
//
// Tx DMA FIFOs have dedicated purposes.  For 4965, they are used as follows
// (cf. default_queue_to_tx_fifo in 4965.c):
//
// 0 -- EDCA BK (background) frames, lowest priority
// 1 -- EDCA BE (best effort) frames, normal priority
// 2 -- EDCA VI (video) frames, higher priority
// 3 -- EDCA VO (voice) and management frames, highest priority
// 4 -- Commands (e.g. RXON, etc.)
// 5 -- unused (HCCA)
// 6 -- unused (HCCA)
// 7 -- not used by driver (device-internal only)
//
// Driver should normally map queues 0-6 to Tx DMA/FIFO channels 0-6.
// In addition, driver can map the remaining queues to Tx DMA/FIFO
// channels 0-3 to support 11n aggregation via EDCA DMA channels.
//
// The driver sets up each queue to work in one of two modes:
//
// 1)  Scheduler-Ack, in which the scheduler automatically supports a
// block-ack (BA) win of up to 64 TFDs.  In this mode, each queue
// contains TFDs for a unique combination of Recipient Address (RA)
// and Traffic Identifier (TID), that is, traffic of a given
// Quality-Of-Service (QOS) priority, destined for a single station.
//
// In scheduler-ack mode, the scheduler keeps track of the Tx status of
// each frame within the BA win, including whether it's been transmitted,
// and whether it's been acknowledged by the receiving station.  The device
// automatically processes block-acks received from the receiving STA,
// and reschedules un-acked frames to be retransmitted (successful
// Tx completion may end up being out-of-order).
//
// The driver must maintain the queue's Byte Count table in host DRAM
// (struct il4965_sched_queue_byte_cnt_tbl) for this mode.
// This mode does not support fragmentation.
//
// 2)  FIFO (a.k.a. non-Scheduler-ACK), in which each TFD is processed in order.
// The device may automatically retry Tx, but will retry only one frame
// at a time, until receiving ACK from receiving station, or reaching
// retry limit and giving up.
//
// The command queue (#4/#9) must use this mode!
// This mode does not require use of the Byte Count table in host DRAM.
//
// Driver controls scheduler operation via 3 means:
// 1)  Scheduler registers
// 2)  Shared scheduler data base in internal 4956 SRAM
// 3)  Shared data in host DRAM
//
// Initialization:
//
// When loading, driver should allocate memory for:
// 1)  16 TFD circular buffers, each with space for (typically) 256 TFDs.
// 2)  16 Byte Count circular buffers in 16 KBytes contiguous memory
// (1024 bytes for each queue).
//
// After receiving "Alive" response from uCode, driver must initialize
// the scheduler (especially for queue #4/#9, the command queue, otherwise
// the driver can't issue commands!):
//
// Max Tx win size is the max number of contiguous TFDs that the scheduler
// can keep track of at one time when creating block-ack chains of frames.
// Note that "64" matches the number of ack bits in a block-ack packet.
// Driver should use SCD_WIN_SIZE and SCD_FRAME_LIMIT values to initialize
// IL49_SCD_CONTEXT_QUEUE_OFFSET(x) values.
//
pub const SCD_WIN_SIZE: c_int = 64;
pub const SCD_FRAME_LIMIT: c_int = 64;
// SCD registers are internal, must be accessed via HBUS_TARG_PRPH regs
pub const IL49_SCD_START_OFFSET: c_uint = 0xa02c00;
//
// 4965 tells driver SRAM address for internal scheduler structs via this reg.
// Value is valid only after "Alive" response from uCode.
//

//
// Driver may need to update queue-empty bits after changing queue's
// write and read pointers (idxes) during (re-)initialization (i.e. when
// scheduler is not tracking what's happening).
// Bit fields:
// 31-16:  Write mask -- 1: update empty bit, 0: don't change empty bit
// 15-00:  Empty state, one for each queue -- 1: empty, 0: non-empty
// NOTE:  This register is not used by Linux driver.
//

//
// Physical base address of array of byte count (BC) circular buffers (CBs).
// Each Tx queue has a BC CB in host DRAM to support Scheduler-ACK mode.
// This register points to BC CB for queue 0, must be on 1024-byte boundary.
// Others are spaced by 1024 bytes.
// Each BC CB is 2 bytes * (256 + 64) = 740 bytes, followed by 384 bytes pad.
// (Index into a queue's BC CB) = (idx into queue's TFD CB) = (SSN & 0xff).
// Bit fields:
// 25-00:  Byte Count CB physical address [35:10], must be 1024-byte aligned.
//

//
// Enables any/all Tx DMA/FIFO channels.
// Scheduler generates requests for only the active channels.
// Set this to 0xff to enable all 8 channels (normal usage).
// Bit fields:
// 7- 0:  Enable (1), disable (0), one bit for each channel 0-7
//

//
// Queue (x) Write Pointers (idxes, really!), one for each Tx queue.
// Initialized and updated by driver as new TFDs are added to queue.
// NOTE:  If using Block Ack, idx must correspond to frame's
// Start Sequence Number; idx = (SSN & 0xff)
// NOTE:  Alternative to HBUS_TARG_WRPTR, which is what Linux driver uses?
//

//
// Queue (x) Read Pointers (idxes, really!), one for each Tx queue.
// For FIFO mode, idx indicates next frame to transmit.
// For Scheduler-ACK mode, idx indicates first frame in Tx win.
// Initialized by driver, updated by scheduler.
//

//
// Select which queues work in chain mode (1) vs. not (0).
// Use chain mode to build chains of aggregated frames.
// Bit fields:
// 31-16:  Reserved
// 15-00:  Mode, one bit for each queue -- 1: Chain mode, 0: one-at-a-time
// NOTE:  If driver sets up queue for chain mode, it should be also set up
// Scheduler-ACK mode as well, via SCD_QUEUE_STATUS_BITS(x).
//

//
// Select which queues interrupt driver when scheduler increments
// a queue's read pointer (idx).
// Bit fields:
// 31-16:  Reserved
// 15-00:  Interrupt enable, one bit for each queue -- 1: enabled, 0: disabled
// NOTE:  This functionality is apparently a no-op; driver relies on interrupts
// from Rx queue to read Tx command responses and update Tx queues.
//

//
// Queue search status registers.  One for each queue.
// Sets up queue mode and assigns queue to Tx DMA channel.
// Bit fields:
// 19-10: Write mask/enable bits for bits 0-9
// 9: Driver should init to "0"
// 8: Scheduler-ACK mode (1), non-Scheduler-ACK (i.e. FIFO) mode (0).
// Driver should init to "1" for aggregation mode, or "0" otherwise.
// 7-6: Driver should init to "0"
// 5: Window Size Left; indicates whether scheduler can request
// another TFD, based on win size, etc.  Driver should init
// this bit to "1" for aggregation mode, or "0" for non-agg.
// 4-1: Tx FIFO to use (range 0-7).
// 0: Queue is active (1), not active (0).
// Other bits should be written as "0"
//
// NOTE:  If enabling Scheduler-ACK mode, chain mode should also be enabled
// via SCD_QUEUECHAIN_SEL.
//
// Macro flag: #define IL49_SCD_QUEUE_STATUS_BITS(x)\
// Bit field positions

// Write masks

//
// 4965 internal SRAM structures for scheduler, shared with driver ...
//
// Driver should clear and initialize the following areas after receiving
// "Alive" response from 4965 uCode, i.e. after initial
// uCode load, or after a uCode load done for error recovery:
//
// SCD_CONTEXT_DATA_OFFSET (size 128 bytes)
// SCD_TX_STTS_BITMAP_OFFSET (size 256 bytes)
// SCD_TRANSLATE_TBL_OFFSET (size 32 bytes)
//
// Driver accesses SRAM via HBUS_TARG_MEM_* registers.
// Driver reads base address of this scheduler area from SCD_SRAM_BASE_ADDR.
// All OFFSET values must be added to this base address.
//
// Queue context.  One 8-byte entry for each of 16 queues.
//
// Driver should clear this entire area (size 0x80) to 0 after receiving
// "Alive" notification from uCode.  Additionally, driver should init
// each queue's entry as follows:
//
// LS Dword bit fields:
// 0-06:  Max Tx win size for Scheduler-ACK.  Driver should init to 64.
//
// MS Dword bit fields:
// 16-22:  Frame limit.  Driver should init to 10 (0xa).
//
// Driver should init all other bits to 0.
//
// Init must be done after driver receives "Alive" response from 4965 uCode,
// and when setting up queue for aggregation.
//
pub const IL49_SCD_CONTEXT_DATA_OFFSET: c_uint = 0x380;

//
// Tx Status Bitmap
//
// Driver should clear this entire area (size 0x100) to 0 after receiving
// "Alive" notification from uCode.  Area is used only by device itself;
// no other support (besides clearing) is required from driver.
//
pub const IL49_SCD_TX_STTS_BITMAP_OFFSET: c_uint = 0x400;
//
// RAxTID to queue translation mapping.
//
// When queue is in Scheduler-ACK mode, frames placed in a that queue must be
// for only one combination of receiver address (RA) and traffic ID (TID), i.e.
// one QOS priority level destined for one station (for this wireless link,
// not final destination).  The SCD_TRANSLATE_TBL area provides 16 16-bit
// mappings, one for each of the 16 queues.  If queue is not in Scheduler-ACK
// mode, the device ignores the mapping value.
//
// Bit fields, for each 16-bit map:
// 15-9:  Reserved, set to 0
// 8-4:  Index into device's station table for recipient station
// 3-0:  Traffic ID (tid), range 0-15
//
// Driver should clear this entire area (size 32 bytes) to 0 after receiving
// "Alive" notification from uCode.  To update a 16-bit map value, driver
// must read a dword-aligned value from device SRAM, replace the 16-bit map
// value of interest, and write the dword value back into device SRAM.
//
pub const IL49_SCD_TRANSLATE_TBL_OFFSET: c_uint = 0x500;
// Find translation table dword to read/write for given queue

// END TX SCHEDULER
