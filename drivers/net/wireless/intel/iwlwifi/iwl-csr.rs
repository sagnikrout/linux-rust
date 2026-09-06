//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-csr.h
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
// Copyright (C) 2005-2014, 2018-2026 Intel Corporation
// Copyright (C) 2013-2014 Intel Mobile Communications GmbH
// Copyright (C) 2016 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_csr_h__
//
// CSR (control and status registers)
//
// CSR registers are mapped directly into PCI bus space, and are accessible
// whenever platform supplies power to device, even when device is in
// low power states due to driver-invoked device resets
// (e.g. CSR_RESET_REG_FLAG_SW_RESET) or uCode-driven power-saving modes.
//
// Use iwl_write32() and iwl_read32() family to access these registers;
// these provide simple PCI bus access, without waking up the MAC.
// Do not use iwl_write_direct32() family for these registers;
// no need to "grab nic access" via CSR_GP_CNTRL_REG_FLAG_MAC_ACCESS_REQ.
// The MAC (uCode processor, etc.) does not need to be powered up for accessing
// the CSR registers.
//
// NOTE:  Device does need to be awake in order to read this memory
// via CSR_EEPROM and CSR_OTP registers
//

// 2nd byte of CSR_INT_COALESCING, not accessible via iwl_write32()!

//
// Hardware revision info
// Bit fields:
// 31-16:  Reserved
// 15-4:  Type of device:  see CSR_HW_REV_TYPE_xxx definitions
// 3-2:  Revision step:  0 = A, 1 = B, 2 = C, 3 = D
// 1-0:  "Dash" (-) value, as in A-1, etc.
//

//
// RF ID revision info
// Bit fields:
// 31:24: Reserved (set to 0x0)
// 23:12: Type
// 11:8:  Step (A - 0x0, B - 0x1, etc)
// 7:4:   Dash
// 3:0:   Flavor
//

//
// EEPROM and OTP (one-time-programmable) memory reads
//
// NOTE:  Device must be awake, initialized via apm_ops.init(),
// in order to read.
//

//
// UCODE-DRIVER GP (general purpose) mailbox registers.
// SET/CLR registers set/clear bit(s) if "1" is written.
//

pub const CSR_MAC_SHADOW_REG_CTL2_RX_WAKE: c_uint = 0xFFFF;
// LTR control (since IWL_DEVICE_FAMILY_22000)

pub const CSR_LTR_LONG_VAL_AD_NO_SNOOP_REQ: c_uint = 0x80000000;
pub const CSR_LTR_LONG_VAL_AD_NO_SNOOP_SCALE: c_uint = 0x1c000000;
pub const CSR_LTR_LONG_VAL_AD_NO_SNOOP_VAL: c_uint = 0x03ff0000;
pub const CSR_LTR_LONG_VAL_AD_SNOOP_REQ: c_uint = 0x00008000;
pub const CSR_LTR_LONG_VAL_AD_SNOOP_SCALE: c_uint = 0x00001c00;
pub const CSR_LTR_LONG_VAL_AD_SNOOP_VAL: c_uint = 0x000003ff;
pub const CSR_LTR_LONG_VAL_AD_SCALE_USEC: c_int = 2;

// GIO Chicken Bits (PCI Express bus link power management)

pub const CSR_IPC_STATE_RESET: c_uint = 0x00000030;
pub const CSR_IPC_STATE_RESET_NONE: c_int = 0;
pub const CSR_IPC_STATE_RESET_SW_READY: c_int = 1;
pub const CSR_IPC_STATE_RESET_TOP_READY: c_int = 2;
pub const CSR_IPC_STATE_RESET_TOP_FOLLOWER: c_int = 3;

pub const CSR_IPC_SLEEP_CONTROL_SUSPEND: c_uint = 0x3;
pub const CSR_IPC_SLEEP_CONTROL_RESUME: c_int = 0;
// Doorbell - since Bz
// connected to UREG_DOORBELL_TO_ISR6 (lower 16 bits only)
//

// host chicken bits

// Analog phase-lock-loop configuration

//
// CSR HW resources monitor registers
//

//
// CSR Hardware Revision Workaround Register.  Indicates hardware rev;
// "step" determines CCK backoff for txpower calculation.
// See also CSR_HW_REV register.
// Bit fields:
// 3-2:  0 = A, 1 = B, 2 = C, 3 = D step
// 1-0:  "Dash" (-) value, as in C-1, etc.
//

//
// Scratch register initial configuration - this is set on init, and read
// during a error FW error.
//

pub const CSR_FUNC_SCRATCH_POWER_OFF_MASK: c_uint = 0xFFFF;
// Bits for CSR_HW_IF_CONFIG_REG

pub const CSR_HW_IF_CONFIG_REG_HAP_WAKE: c_uint = 0x00080000;
// NOTE: EEPROM_OWN_SEM is no longer defined for new HW
pub const CSR_HW_IF_CONFIG_REG_EEPROM_OWN_SEM: c_uint = 0x00200000;
pub const CSR_HW_IF_CONFIG_REG_PCI_OWN_SET: c_uint = 0x00400000;
pub const CSR_HW_IF_CONFIG_REG_IAMT_UP: c_uint = 0x01000000;
pub const CSR_HW_IF_CONFIG_REG_ME_OWN: c_uint = 0x02000000;
pub const CSR_HW_IF_CONFIG_REG_WAKE_ME: c_uint = 0x08000000;
pub const CSR_HW_IF_CONFIG_REG_WAKE_ME_PCIE_OWNER_EN: c_uint = 0x10000000;
pub const CSR_HW_IF_CONFIG_REG_PERSISTENCE: c_uint = 0x40000000;

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
// 10:  XTAL ON request
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
// NOTE:  This does not indicate that device has completed
// init or post-power-down restore of internal SRAM memory.
// Use CSR_UCODE_DRV_GP1_BIT_MAC_SLEEP as indication that
// SRAM is restored and uCode is in normal operation mode.
// Later devices (5xxx/6xxx/1xxx) use non-volatile SRAM, and
// do not need to save/restore it.
// NOTE:  After device reset, this bit remains "0" until host sets
// INIT_DONE
//

// From Bz we use these instead during init/reset flow

// HW REV

// HW RFID

// hw_rev values

// RF_ID value

// HW_RF CHIP STEP

// EEPROM REG

// EEPROM GP

// One-time-programmable memory general purpose reg

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
// NOTE:  device saves internal SRAM data to host when powering down,
// and must restore this data after powering back up.
// MAC_SLEEP is the best indication that restore is complete.
// Later devices (5xxx/6xxx/1xxx) use non-volatile SRAM, and
// do not need to save/restore it.
//

// GP Driver

// GIO Chicken Bits (PCI Express bus link power management)

// LED

// ANA_PLL

// HPET MEM debug

// DRAM INT TABLE

//
// SHR target access (Shared block memory space)
//
// Shared internal registers can be accessed directly from PCI bus through SHR
// arbiter without need for the MAC HW to be powered up. This is possible due to
// indirect read/write via HEEP_CTRL_WRD_PCIEX_CTRL (0xEC) and
// HEEP_CTRL_WRD_PCIEX_DATA (0xF4) registers.
//
// Use iwl_write32()/iwl_read32() family to access these registers. The MAC HW
// need not be powered up so no "grab inc access" is required.
//
// Registers for accessing shared registers (e.g. SHR_APMG_GP1,
// SHR_APMG_XTAL_CFG). For example, to read from SHR_APMG_GP1 register (0x1DC),
// first, write to the control register:
// HEEP_CTRL_WRD_PCIEX_CTRL[15:0] = 0x1DC (offset of the SHR_APMG_GP1 register)
// HEEP_CTRL_WRD_PCIEX_CTRL[29:28] = 2 (read access)
// second, read from the data register HEEP_CTRL_WRD_PCIEX_DATA[31:0].
//
// To write the register, first, write to the data register
// HEEP_CTRL_WRD_PCIEX_DATA[31:0] and then:
// HEEP_CTRL_WRD_PCIEX_CTRL[15:0] = 0x1DC (offset of the SHR_APMG_GP1 register)
// HEEP_CTRL_WRD_PCIEX_CTRL[29:28] = 3 (write access)
//

//
// HBUS (Host-side Bus)
//
// HBUS registers are mapped directly into PCI bus space, but are used
// to indirectly access device's internal memory or registers that
// may be powered-down.
//
// Use iwl_write_direct32()/iwl_read_direct32() family for these registers;
// host must "grab nic access" via CSR_GP_CNTRL_REG_FLAG_MAC_ACCESS_REQ
// to make sure the MAC (uCode processor, etc.) is powered up for accessing
// internal resources.
//
// Do not use iwl_write32()/iwl_read32() family to access these registers;
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

// Used to enable DBGM

//
// Per-Tx-queue write pointer (index, really!)
// Indicates index to next TFD that driver will fill (1 past latest filled).
// Bit usage:
// 0-7:  queue write index
// 11-8:  queue selector
//

// This register is common for Tx and Rx, Rx queues start from 512

//
// CSR values
//
// host interrupt timeout value
// used with setting interrupt coalescing timer
// the CSR_INT_COALESCING is an 8 bit register in 32-usec unit
//
// default interrupt coalescing timer is 64 x 32 = 2048 usecs
//

//
// 7000/3000 series SHR DTS addresses
//
// Diode Results Register Structure:
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dtd_diode_reg {
    DTS_DIODE_REG_DIG_VAL			= 0x000000FF, /* bits [7:0] */
    DTS_DIODE_REG_VREF_LOW			= 0x0000FF00, /* bits [15:8] */
    DTS_DIODE_REG_VREF_HIGH			= 0x00FF0000, /* bits [23:16] */
    DTS_DIODE_REG_VREF_ID			= 0x03000000, /* bits [25:24] */
    DTS_DIODE_REG_PASS_ONCE			= 0x80000000, /* bits [31:31] */
    DTS_DIODE_REG_FLAGS_MSK			= 0xFF000000, /* bits [31:24] */
// Those are the masks INSIDE the flags bit-field:
    DTS_DIODE_REG_FLAGS_VREFS_ID_POS	= 0,
    DTS_DIODE_REG_FLAGS_VREFS_ID		= 0x00000003, /* bits [1:0] */
    DTS_DIODE_REG_FLAGS_PASS_ONCE_POS	= 7,
    DTS_DIODE_REG_FLAGS_PASS_ONCE		= 0x00000080, /* bits [7:7] */
}

//
// MSIX related registers
//

//
// Causes for the FH register interrupts
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msix_fh_int_causes {
    MSIX_FH_INT_CAUSES_Q0			= BIT(0),
    MSIX_FH_INT_CAUSES_Q1			= BIT(1),
    MSIX_FH_INT_CAUSES_D2S_CH0_NUM		= BIT(16),
    MSIX_FH_INT_CAUSES_D2S_CH1_NUM		= BIT(17),
    MSIX_FH_INT_CAUSES_S2D			= BIT(19),
    MSIX_FH_INT_CAUSES_FH_ERR		= BIT(21),
}

// The low 16 bits are for rx data queue indication
pub const MSIX_FH_INT_CAUSES_DATA_QUEUE: c_uint = 0xffff;
//
// Causes for the HW register interrupts
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msix_hw_int_causes {
    MSIX_HW_INT_CAUSES_REG_ALIVE		= BIT(0),
    MSIX_HW_INT_CAUSES_REG_WAKEUP		= BIT(1),
    MSIX_HW_INT_CAUSES_REG_IML              = BIT(1),
    MSIX_HW_INT_CAUSES_REG_RESET_DONE	= BIT(2),
    MSIX_HW_INT_CAUSES_REG_TOP_FATAL_ERR	= BIT(3),
    MSIX_HW_INT_CAUSES_REG_SW_ERR_BZ	= BIT(5),
    MSIX_HW_INT_CAUSES_REG_CT_KILL		= BIT(6),
    MSIX_HW_INT_CAUSES_REG_RF_KILL		= BIT(7),
    MSIX_HW_INT_CAUSES_REG_PERIODIC		= BIT(8),
    MSIX_HW_INT_CAUSES_REG_SW_ERR		= BIT(25),
    MSIX_HW_INT_CAUSES_REG_SCD		= BIT(26),
    MSIX_HW_INT_CAUSES_REG_FH_TX		= BIT(27),
    MSIX_HW_INT_CAUSES_REG_HW_ERR		= BIT(29),
    MSIX_HW_INT_CAUSES_REG_HAP		= BIT(30),
}

pub const MSIX_MIN_INTERRUPT_VECTORS: c_int = 2;
pub const MSIX_AUTO_CLEAR_CAUSE: c_int = 0;

//
// HW address related registers
//

