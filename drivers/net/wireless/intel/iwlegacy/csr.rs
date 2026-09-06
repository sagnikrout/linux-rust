//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlegacy/csr.h
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

// Macro flag: #define __il_csr_h__
//
// CSR (control and status registers)
//
// CSR registers are mapped directly into PCI bus space, and are accessible
// whenever platform supplies power to device, even when device is in
// low power states due to driver-invoked device resets
// (e.g. CSR_RESET_REG_FLAG_SW_RESET) or uCode-driven power-saving modes.
//
// Use _il_wr() and _il_rd() family to access these registers;
// these provide simple PCI bus access, without waking up the MAC.
// Do not use il_wr() family for these registers;
// no need to "grab nic access" via CSR_GP_CNTRL_REG_FLAG_MAC_ACCESS_REQ.
// The MAC (uCode processor, etc.) does not need to be powered up for accessing
// the CSR registers.
//
// NOTE:  Device does need to be awake in order to read this memory
// via CSR_EEPROM register
//

// 2nd byte of CSR_INT_COALESCING, not accessible via _il_wr()!

//
// Hardware revision info
// Bit fields:
// 31-8:  Reserved
// 7-4:  Type of device:  see CSR_HW_REV_TYPE_xxx definitions
// 3-2:  Revision step:  0 = A, 1 = B, 2 = C, 3 = D
// 1-0:  "Dash" (-) value, as in A-1, etc.
//
// NOTE:  Revision step affects calculation of CCK txpower for 4965.
// NOTE:  See also CSR_HW_REV_WA_REG (work-around for bug in 4965).
//

//
// EEPROM memory reads
//
// NOTE:  Device must be awake, initialized via apm_ops.init(),
// in order to read.
//

//
// UCODE-DRIVER GP (general purpose) mailbox registers.
// SET/CLR registers set/clear bit(s) if "1" is written.
//

// GIO Chicken Bits (PCI Express bus link power management)

// Analog phase-lock-loop configuration

//
// CSR Hardware Revision Workaround Register.  Indicates hardware rev;
// "step" determines CCK backoff for txpower calculation.  Used for 4965 only.
// See also CSR_HW_REV register.
// Bit fields:
// 3-2:  0 = A, 1 = B, 2 = C, 3 = D step
// 1-0:  "Dash" (-) value, as in C-1, etc.
//

// Bits for CSR_HW_IF_CONFIG_REG

// interrupt flags in INTA, set by uCode or hardware (e.g. dma),
// acknowledged (reset) by host writing "1" to flagged bits.

// interrupt flags in FH (flow handler) (PCI busmaster DMA)

// GPIO

// RESET

//
// GP (general purpose) CONTROL REGISTER
// Bit fields:
// 27:  HW_RF_KILL_SW
// Indicates state of (platform's) hardware RF-Kill switch
// 26-24:  POWER_SAVE_TYPE
// Indicates current power-saving mode:
// 000 -- No power saving
// 001 -- MAC power-down
// 010 -- PHY (radio) power-down
// 011 -- Error
// 9-6:  SYS_CONFIG
// Indicates current system configuration, reflecting pins on chip
// as forced high/low by device circuit board.
// 4:  GOING_TO_SLEEP
// Indicates MAC is entering a power-saving sleep power-down.
// Not a good time to access device-internal resources.
// 3:  MAC_ACCESS_REQ
// Host sets this to request and maintain MAC wakeup, to allow host
// access to device-internal resources.  Host must wait for
// MAC_CLOCK_READY (and !GOING_TO_SLEEP) before accessing non-CSR
// device registers.
// 2:  INIT_DONE
// Host sets this to put device into fully operational D0 power mode.
// Host resets this after SW_RESET to put device into low power mode.
// 0:  MAC_CLOCK_READY
// Indicates MAC (ucode processor, etc.) is powered up and can run.
// Internal resources are accessible.
// NOTE:  This does not indicate that the processor is actually running.
// NOTE:  This does not indicate that 4965 or 3945 has completed
// init or post-power-down restore of internal SRAM memory.
// Use CSR_UCODE_DRV_GP1_BIT_MAC_SLEEP as indication that
// SRAM is restored and uCode is in normal operation mode.
// Later devices (5xxx/6xxx/1xxx) use non-volatile SRAM, and
// do not need to save/restore it.
// NOTE:  After device reset, this bit remains "0" until host sets
// INIT_DONE
//

// EEPROM REG

// EEPROM GP

// GP REG

// CSR GIO

//
// UCODE-DRIVER GP (general purpose) mailbox register 1
// Host driver and uCode write and/or read this register to communicate with
// each other.
// Bit fields:
// 4:  UCODE_DISABLE
// Host sets this to request permanent halt of uCode, same as
// sending CARD_STATE command with "halt" bit set.
// 3:  CT_KILL_EXIT
// Host sets this to request exit from CT_KILL state, i.e. host thinks
// device temperature is low enough to continue normal operation.
// 2:  CMD_BLOCKED
// Host sets this during RF KILL power-down sequence (HW, SW, CT KILL)
// to release uCode to clear all Tx and command queues, enter
// unassociated mode, and power down.
// NOTE:  Some devices also use HBUS_TARG_MBX_C register for this bit.
// 1:  SW_BIT_RFKILL
// Host sets this when issuing CARD_STATE command to request
// device sleep.
// 0:  MAC_SLEEP
// uCode sets this when preparing a power-saving power-down.
// uCode resets this when power-up is complete and SRAM is sane.
// NOTE:  3945/4965 saves internal SRAM data to host when powering down,
// and must restore this data after powering back up.
// MAC_SLEEP is the best indication that restore is complete.
// Later devices (5xxx/6xxx/1xxx) use non-volatile SRAM, and
// do not need to save/restore it.
//

// GIO Chicken Bits (PCI Express bus link power management)

// LED

// ANA_PLL

// HPET MEM debug

// DRAM INT TBL

//
// HBUS (Host-side Bus)
//
// HBUS registers are mapped directly into PCI bus space, but are used
// to indirectly access device's internal memory or registers that
// may be powered-down.
//
// Use il_wr()/il_rd() family
// for these registers;
// host must "grab nic access" via CSR_GP_CNTRL_REG_FLAG_MAC_ACCESS_REQ
// to make sure the MAC (uCode processor, etc.) is powered up for accessing
// internal resources.
//
// Do not use _il_wr()/_il_rd() family to access these registers;
// these provide only simple PCI bus access, without waking up the MAC.
//

//
// Registers for accessing device's internal SRAM memory (e.g. SCD SRAM
// structures, error log, event log, verifying uCode load).
// First write to address register, then read from or write to data register
// to complete the job.  Once the address register is set up, accesses to
// data registers auto-increment the address by one dword.
// Bit usage for address registers (read or write):
// 0-31:  memory address within device
//

// Mailbox C, used as workaround alternative to CSR_UCODE_DRV_GP1 mailbox

//
// Registers for accessing device's internal peripheral registers
// (e.g. SCD, BSM, etc.).  First write to address register,
// then read from or write to data register to complete the job.
// Bit usage for address registers (read or write):
// 0-15:  register address (offset) within device
// 24-25:  (# bytes - 1) to read or write (e.g. 3 for dword)
//

//
// Per-Tx-queue write pointer (idx, really!)
// Indicates idx to next TFD that driver will fill (1 past latest filled).
// Bit usage:
// 0-7:  queue write idx
// 11-8:  queue selector
//

