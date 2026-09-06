//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn60/dalsmc.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.
//
// @file dalsmc.h
//
// @brief VBIOS and DAL to PMFW Interface
//
// Clients:  VBIOS and DAL
// Protocols: dalsmc
//
// @date 2016 - 2026
//
// @mainpage PMFW-DAL Message Interface
//
// The protocol uses six registers:
//
// - MSG_REG   — write the message ID (DALSMC_MSG_*) to trigger the transaction
// - ARG_REG_0 — input argument Reg0; also carries response data on completion
// - ARG_REG_1 — input argument Reg1
// - ARG_REG_2 — input argument Reg2
// - ARG_REG_3 — input argument Reg3
// - RESP_REG  — poll until non-zero; value is a DALSMC_Result_* response code
//
// Programming sequence:
// 1. Clear RESP_REG to 0
// 2. Write input arguments to ARG_REG_0..3
// 3. Write message ID to MSG_REG (triggers PMFW interrupt)
// 4. Poll RESP_REG until non-zero — value is the result code
// 5. Read response data from ARG_REG_0 (message-specific)
//
// For payloads too large for the four argument registers, the protocol supports
// DRAM table transfers where DAL allocates a DRAM buffer and exchanges bulk data
// with PMFW through system memory.
//
// This documentation contains the subsections:\n\n
// @ref ResponseCodes\n
// @ref Messages\n
// @ref DramTables\n
//
pub const DALSMC_VERSION: c_uint = 0x1;
// @defgroup ResponseCodes PMFW Response Codes
// @{
//
// SMU Response Codes:
pub const DALSMC_Result_OK: c_uint = 0x01;
pub const DALSMC_Result_Failed: c_uint = 0xFF;
pub const DALSMC_Result_UnknownCmd: c_uint = 0xFE;
pub const DALSMC_Result_CmdRejectedPrereq: c_uint = 0xFD;
pub const DALSMC_Result_CmdRejectedBusy: c_uint = 0xFC;
// @}
// @defgroup Messages Message definitions
// @{
//
// Generic register overlay — four 32-bit C2PMSG argument registers.
//
// DALSMC_MSG_TestMessage - Test interface connectivity.
//
// Echos back the argument value incremented by 1. Use to verify the mailbox
// is functional before sending real messages.
//
// Request:  TestValue — arbitrary test integer
// Response: Reg0      — TestValue + 1
//
pub const DALSMC_MSG_TestMessage: c_uint = 0x01;
//
// DALSMC_MSG_GetMsgHeaderVersion - Query the DALSMC header version running on PMFW.
//
// DAL uses the returned version to determine which messages are supported in
// environments that require backwards compatibility.
//
// Request:  (none)
// Response: Reg0 — DALSMC_VERSION value compiled into PMFW
//
pub const DALSMC_MSG_GetMsgHeaderVersion: c_uint = 0x02;
//
// DALSMC_MSG_TransferTableSmu2Dram - Transfer a PMFW table into DRAM.
// DALSMC_MSG_TransferTableDram2Smu - Transfer a DRAM buffer into PMFW.
//
// Both directions use the same argument layout. The DRAM address must be set
// beforehand (AddrLow / AddrHigh are the GPU MC address bits [31:0] / [63:32]).
//
// Smu2Dram supported tables: TABLE_DAL_INIT (DPM clocks + UTM QoS + memory config)
// Dram2Smu supported tables: TABLE_SOC_UTM  (debug override of UTM QoS parameters)
//
// Request:  TableId  — table identifier (TABLE_* defines below)
// AddrLow  — GPU MC address bits [31:0]  of destination/source buffer
// AddrHigh — GPU MC address bits [63:32] of destination/source buffer
// Response: (none beyond result code)
//
pub const DALSMC_MSG_TransferTableSmu2Dram: c_uint = 0x03;
pub const DALSMC_MSG_TransferTableDram2Smu: c_uint = 0x04;
//
// DALSMC_MSG_SetHardMinByFreq - Set a lower bound frequency constraint on a PPCLK.
//
// Response does not indicate that the effective clock has already been raised to
// meet the minimum requirement; poll DALSMC_MSG_ReturnHardMinStatus for the status
// of the request.
//
// Supported clocks: SOCCLK, DISPCLK, DPPCLK, DCFCLK, DTBCLK.
//
// Request:  FreqKhz[23:0] — target minimum frequency in kHz (0 to ~16.7 GHz)
// Ppclk[31:24]  — PPCLK_e clock identifier
// Response: (none beyond result code)
//
pub const DALSMC_MSG_SetHardMinByFreq: c_uint = 0x05;
//
// DALSMC_MSG_SetMinDeepSleepDcfclk - Set the minimum DCFCLK frequency in deep sleep.
//
// Request:  MinDcfclkMhz — minimum DCFCLK frequency in MHz during deep sleep
// Response: (none beyond result code)
//
pub const DALSMC_MSG_SetMinDeepSleepDcfclk: c_uint = 0x06;
//
// DALSMC_MSG_BacoAudioD3PME - Wake the audio block from D3/BACO.
//
// Triggers PMFW to bring the AZ (audio) block out of its D3 power state.
// No arguments or response data; result code indicates success.
//
pub const DALSMC_MSG_BacoAudioD3PME: c_uint = 0x07;
//
// DALSMC_MSG_ReturnHardMinStatus - Query outstanding hard-min request status.
//
// Returns a bitmask reporting which PPCLK hard-min requests have been satisfied
// by the arbiter. Each bit position corresponds to the matching PPCLK_e value.
// A set bit means the arbiter has reached or exceeded the requested minimum.
//
// Request:  (none)
// Response: Reg0 — bitmask of satisfied PPCLKs (bit N set ↔ PPCLK_e N is satisfied)
//
pub const DALSMC_MSG_ReturnHardMinStatus: c_uint = 0x08;
//
// DALSMC_MSG_IndicatePstateStatus - Indicate to PMFW various DMU behaviors required
// to support UCLK P-state, for example whether or not DMU needs to modulate refresh
// rate to perform UCLK switches.
//
// Request:  WaitResp[0]   — DAL requires a synchronous response before proceeding
// DrrEnable[1]  — DRR (dynamic refresh rate modulation) is active
// AltCh[2]      — alternate-channel mode is active
// AllowUclk[16] — DCN can tolerate UCLK P-state switches
// AllowFclk[17] — DCN can tolerate FCLK P-state switches
// Response: (none beyond result code)
//
pub const DALSMC_MSG_IndicatePstateStatus: c_uint = 0x09;
//
// DALSMC_MSG_UpdateUTMQoSRequest - Update the active UTM QoS bandwidth/latency request.
//
// Passes the current display bandwidth and latency requirements to PMFW so it
// can select the appropriate SoC operating point (UCLK/FCLK level) from the
// UTM table. Called whenever the display configuration changes.
//
// The QoS requirement must take effect before PMFW sends its response.
//
// Request:  LatencySopIndex      — index into the UTM SOP table that satisfies latency
// NominalBandwidthKBps — required nominal (average) bandwidth in KB/s
// UrgentBandwidthKBps  — required urgent bandwidth in KB/s
// LsdmaBandwidthKBps   — required LSDMA bandwidth in KB/s
// Response: (none beyond result code)
//
pub const DALSMC_MSG_UpdateUTMQoSRequest: c_uint = 0x0A;
//
// DALSMC_MSG_SetDisplayIdleOptimizations - Notify PMFW of DCN idle-state conditions.
//
// Indicates which display-side power optimizations are currently safe to apply.
// PMFW uses these flags to gate deeper SoC power states such as S0i2.
//
// Request:  DfRequestDisabled[0] — DF (data fabric) requests from DCN are disabled
// PhyRefClkOff[1]      — PHY reference clock has been gated off
// S0i2Rdy[2]           — DCN is ready for the system to enter S0i2
// Response: (none beyond result code)
//
pub const DALSMC_MSG_SetDisplayIdleOptimizations: c_uint = 0x0B;
//
// DALSMC_MSG_SetStutterEfficiency - Report DCN stutter efficiency to PMFW.
//
// Informs PMFW of the current stutter utilisation for base and low-power stutter
// modes so PMFW can adjust memory power policy accordingly.
//
// Base mode    — lower enter+exit latency (PHY LP1, no UCIE LP).
// Low-power mode — higher enter+exit latency (PHY LP2, UCIE LP1).
//
// Request:  BaseEfficiencyPct[7:0]     — stutter efficiency % in base mode
// LowPowerEfficiencyPct[15:8] — stutter efficiency % in low-power mode
// Response: (none beyond result code)
//
pub const DALSMC_MSG_SetStutterEfficiency: c_uint = 0x0C;
pub const DALSMC_Message_Count: c_uint = 0x0D ///< Total number of messages;
// @}
// @defgroup DramTables DRAM Tables
// @brief Bulk data structures exchanged between DAL and PMFW via system DRAM.
//
// Used when the payload exceeds the four 32-bit C2PMSG argument registers
// (DALSMC_args_t). DAL allocates a DRAM buffer, passes its address through
// DALSMC_TransferTable_arg_t, and issues either a
// DALSMC_MSG_TransferTableSmu2Dram or DALSMC_MSG_TransferTableDram2Smu message.
// @{
//
pub const NUM_CLOCK_LEVELS: c_int = 8;
//
// TABLE_SOC_UTM - SoC UTM QoS table.
//
// Provides per-load-level bandwidth and latency bounds used by the display
// engine to meet memory access requirements.
//
// Normal path: embedded in DalInitTable_t, fetched once via
// DALSMC_MSG_TransferTableSmu2Dram(TABLE_DAL_INIT).
//
// Override path (debug only): TABLE_SOC_UTM via
// DALSMC_MSG_TransferTableDram2Smu to override custom QoS parameters into PMFW.
//
pub const TABLE_SOC_UTM: c_uint = 0xC;
// TODO: rename back to MAX_UTM_SOP_COUNT once utm_qos_model_types.h conflict is resolved
pub const DALSMC_MAX_UTM_SOP_COUNT: c_int = 16;
pub const MAX_UTM_LOAD_LEVEL_COUNT: c_int = 16;
pub const UTM_LOAD_LEVEL_INDEX_IDLE: c_int = 0;
pub const UTM_LOAD_LEVEL_INDEX_ACTIVE_ALTERNATE_PSTATE: c_int = 1;
pub const UTM_LOAD_LEVEL_INDEX_ACTIVE: c_int = 2;

//
// TABLE_DAL_INIT - Full TABLE_DAL_INIT payload transferred from SMU to DRAM.
//
pub const TABLE_DAL_INIT: c_uint = 0xD;
pub const MAX_PPCLK_COUNT: c_int = 12;

// @}
