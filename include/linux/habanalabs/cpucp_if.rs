//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/habanalabs/cpucp_if.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2020-2023 HabanaLabs, Ltd.
// All Rights Reserved.
//

pub const NUM_HBM_PSEUDO_CH: c_int = 2;
pub const NUM_HBM_CH_PER_DEV: c_int = 8;
pub const CPUCP_PKT_HBM_ECC_INFO_WR_PAR_SHIFT: c_int = 0;
pub const CPUCP_PKT_HBM_ECC_INFO_WR_PAR_MASK: c_uint = 0x00000001;
pub const CPUCP_PKT_HBM_ECC_INFO_RD_PAR_SHIFT: c_int = 1;
pub const CPUCP_PKT_HBM_ECC_INFO_RD_PAR_MASK: c_uint = 0x00000002;
pub const CPUCP_PKT_HBM_ECC_INFO_CA_PAR_SHIFT: c_int = 2;
pub const CPUCP_PKT_HBM_ECC_INFO_CA_PAR_MASK: c_uint = 0x00000004;
pub const CPUCP_PKT_HBM_ECC_INFO_DERR_SHIFT: c_int = 3;
pub const CPUCP_PKT_HBM_ECC_INFO_DERR_MASK: c_uint = 0x00000008;
pub const CPUCP_PKT_HBM_ECC_INFO_SERR_SHIFT: c_int = 4;
pub const CPUCP_PKT_HBM_ECC_INFO_SERR_MASK: c_uint = 0x00000010;
pub const CPUCP_PKT_HBM_ECC_INFO_TYPE_SHIFT: c_int = 5;
pub const CPUCP_PKT_HBM_ECC_INFO_TYPE_MASK: c_uint = 0x00000020;
pub const CPUCP_PKT_HBM_ECC_INFO_HBM_CH_SHIFT: c_int = 6;
pub const CPUCP_PKT_HBM_ECC_INFO_HBM_CH_MASK: c_uint = 0x000007C0;
pub const PLL_MAP_MAX_BITS: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eq_event_id {
    EQ_EVENT_NIC_STS_REQUEST = 0,
    EQ_EVENT_PWR_MODE_0,
    EQ_EVENT_PWR_MODE_1,
    EQ_EVENT_PWR_MODE_2,
    EQ_EVENT_PWR_MODE_3,
    EQ_EVENT_PWR_BRK_ENTRY,
    EQ_EVENT_PWR_BRK_EXIT,
    EQ_EVENT_HEARTBEAT,
    EQ_EVENT_CPLD_RESET_REASON,
    EQ_EVENT_CPLD_SHUTDOWN,
    EQ_EVENT_POWER_EVT_START,
    EQ_EVENT_POWER_EVT_END,
    EQ_EVENT_THERMAL_EVT_START,
    EQ_EVENT_THERMAL_EVT_END,
}

//
// info of the pkt queue pointers in the first async occurrence
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_pkt_sync_err {
    pub pi: __le32,
    pub ci: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_hbm_ecc_data {
// SERR counter
    pub sec_cnt: __le32,
// DERR counter
    pub dec_cnt: __le32,
// Supplemental Information according to the mask bits
    pub hbm_ecc_info: __le32,
// Address in hbm where the ecc happened
    pub first_addr: __le32,
// SERR continuous address counter
    pub sec_cont_cnt: __le32,
    pub pad: __le32,
}

//
// EVENT QUEUE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_header {
    pub reserved: __le32,
    pub ctl: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_ecc_data {
    pub ecc_address: __le64,
    pub ecc_syndrom: __le64,
    pub memory_wrapper_idx: __u8,
    pub is_critical: __u8,
    pub block_id: __le16,
    pub pad: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_sm_sei_cause {
    SM_SEI_SO_OVERFLOW,
    SM_SEI_LBW_4B_UNALIGNED,
    SM_SEI_AXI_RESPONSE_ERR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_sm_sei_data {
    pub sei_log: __le32,
// enum hl_sm_sei_cause
    pub sei_cause: __u8,
    pub pad: [__u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_fw_alive_severity {
    FW_ALIVE_SEVERITY_MINOR,
    FW_ALIVE_SEVERITY_CRITICAL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_fw_alive {
    pub uptime_seconds: __le64,
    pub process_id: __le32,
    pub thread_id: __le32,
// enum hl_fw_alive_severity
    pub severity: __u8,
    pub pad: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_intr_cause {
    pub intr_cause_data: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_pcie_drain_ind_data {
    pub intr_cause: hl_eq_intr_cause,
    pub drain_wr_addr_lbw: __le64,
    pub drain_rd_addr_lbw: __le64,
    pub drain_wr_addr_hbw: __le64,
    pub drain_rd_addr_hbw: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_razwi_lbw_info_regs {
    pub rr_aw_razwi_reg: __le32,
    pub rr_aw_razwi_id_reg: __le32,
    pub rr_ar_razwi_reg: __le32,
    pub rr_ar_razwi_id_reg: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_razwi_hbw_info_regs {
    pub rr_aw_razwi_hi_reg: __le32,
    pub rr_aw_razwi_lo_reg: __le32,
    pub rr_aw_razwi_id_reg: __le32,
    pub rr_ar_razwi_hi_reg: __le32,
    pub rr_ar_razwi_lo_reg: __le32,
    pub rr_ar_razwi_id_reg: __le32,
}

// razwi_happened masks
pub const RAZWI_HAPPENED_HBW: c_uint = 0x1;
pub const RAZWI_HAPPENED_LBW: c_uint = 0x2;
pub const RAZWI_HAPPENED_AW: c_uint = 0x4;
pub const RAZWI_HAPPENED_AR: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_razwi_info {
    pub razwi_happened_mask: __le32,
    pub lbw: hl_eq_razwi_lbw_info_regs,
    pub hbw: hl_eq_razwi_hbw_info_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_razwi_with_intr_cause {
    pub razwi_info: hl_eq_razwi_info,
    pub intr_cause: hl_eq_intr_cause,
}

pub const HBM_CA_ERR_CMD_LIFO_LEN: c_int = 8;
pub const HBM_RD_ERR_DATA_LIFO_LEN: c_int = 8;
pub const HBM_WR_PAR_CMD_LIFO_LEN: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_hbm_sei_cause {
// Command/address parity error event is split into 2 events due to
// size limitation: ODD suffix for odd HBM CK_t cycles and EVEN  suffix
// for even HBM CK_t cycles
//
    HBM_SEI_CMD_PARITY_EVEN,
    HBM_SEI_CMD_PARITY_ODD,
// Read errors can be reflected as a combination of SERR/DERR/parity
// errors. Therefore, we define one event for all read error types.
// LKD will perform further proccessing.
//
    HBM_SEI_READ_ERR,
    HBM_SEI_WRITE_DATA_PARITY_ERR,
    HBM_SEI_CATTRIP,
    HBM_SEI_MEM_BIST_FAIL,
    HBM_SEI_DFI,
    HBM_SEI_INV_TEMP_READ_OUT,
    HBM_SEI_BIST_FAIL,
}

// Masks for parsing hl_hbm_sei_headr fields
pub const HBM_ECC_SERR_CNTR_MASK: c_uint = 0xFF;
pub const HBM_ECC_DERR_CNTR_MASK: c_uint = 0xFF00;
pub const HBM_RD_PARITY_CNTR_MASK: c_uint = 0xFF0000;
// HBM index and MC index are known by the event_id
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_hbm_sei_header {
// relevant only in case of HBM read error
    pub ecc_serr_cnt: __u8,
    pub ecc_derr_cnt: __u8,
    pub read_par_cnt: __u8,
    pub reserved: __u8,
}

// All other cases
pub const HBM_RD_ADDR_SID_SHIFT: c_int = 0;
pub const HBM_RD_ADDR_SID_MASK: c_uint = 0x1;
pub const HBM_RD_ADDR_BG_SHIFT: c_int = 1;
pub const HBM_RD_ADDR_BG_MASK: c_uint = 0x6;
pub const HBM_RD_ADDR_BA_SHIFT: c_int = 3;
pub const HBM_RD_ADDR_BA_MASK: c_uint = 0x18;
pub const HBM_RD_ADDR_COL_SHIFT: c_int = 5;
pub const HBM_RD_ADDR_COL_MASK: c_uint = 0x7E0;
pub const HBM_RD_ADDR_ROW_SHIFT: c_int = 11;
pub const HBM_RD_ADDR_ROW_MASK: c_uint = 0x3FFF800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_rd_addr {
// bit fields are only for FW use
    pub dbg_rd_err_addr_sid:1: u32,
    pub dbg_rd_err_addr_bg:2: u32,
    pub dbg_rd_err_addr_ba:2: u32,
    pub dbg_rd_err_addr_col:6: u32,
    pub dbg_rd_err_addr_row:15: u32,
    pub reserved:6: u32,
}

pub const HBM_RD_ERR_BEAT_SHIFT: c_int = 2;
// dbg_rd_err_misc fields:
// Read parity is calculated per DW on every beat
pub const HBM_RD_ERR_PAR_ERR_BEAT0_SHIFT: c_int = 0;
pub const HBM_RD_ERR_PAR_ERR_BEAT0_MASK: c_uint = 0x3;
pub const HBM_RD_ERR_PAR_DATA_BEAT0_SHIFT: c_int = 8;
pub const HBM_RD_ERR_PAR_DATA_BEAT0_MASK: c_uint = 0x300;
// ECC is calculated per PC on every beat
pub const HBM_RD_ERR_SERR_BEAT0_SHIFT: c_int = 16;
pub const HBM_RD_ERR_SERR_BEAT0_MASK: c_uint = 0x10000;
pub const HBM_RD_ERR_DERR_BEAT0_SHIFT: c_int = 24;
pub const HBM_RD_ERR_DERR_BEAT0_MASK: c_uint = 0x100000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_hbm_sei_read_err_intr_info {
// DFI_RD_ERR_REP_ADDR
    pub dbg_rd_err_addr: hbm_rd_addr,
// DFI_RD_ERR_REP_ERR
// bit fields are only for FW use
    pub dbg_rd_err_par:8: u32,
    pub dbg_rd_err_par_data:8: u32,
    pub dbg_rd_err_serr:4: u32,
    pub dbg_rd_err_derr:4: u32,
    pub reserved:8: u32,
}

// DFI_RD_ERR_REP_DM
// DFI_RD_ERR_REP_SYNDROME
// DFI_RD_ERR_REP_DATA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_hbm_sei_ca_par_intr_info {
// 14 LSBs
    pub dbg_row: [__le16; HBM_CA_ERR_CMD_LIFO_LEN],
// 18 LSBs
    pub dbg_col: [__le32; HBM_CA_ERR_CMD_LIFO_LEN],
}

pub const WR_PAR_LAST_CMD_COL_SHIFT: c_int = 0;
pub const WR_PAR_LAST_CMD_COL_MASK: c_uint = 0x3F;
pub const WR_PAR_LAST_CMD_BG_SHIFT: c_int = 6;
pub const WR_PAR_LAST_CMD_BG_MASK: c_uint = 0xC0;
pub const WR_PAR_LAST_CMD_BA_SHIFT: c_int = 8;
pub const WR_PAR_LAST_CMD_BA_MASK: c_uint = 0x300;
pub const WR_PAR_LAST_CMD_SID_SHIFT: c_int = 10;
pub const WR_PAR_LAST_CMD_SID_MASK: c_uint = 0x400;
// Row address isn't latched
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbm_sei_wr_cmd_address {
// DFI_DERR_LAST_CMD
// bit fields are only for FW use
    pub col:6: u32,
    pub bg:2: u32,
    pub ba:2: u32,
    pub sid:1: u32,
    pub reserved:21: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_hbm_sei_wr_par_intr_info {
// entry 0: WR command address from the 1st cycle prior to the error
// entry 1: WR command address from the 2nd cycle prior to the error
// and so on...
//
    pub dbg_last_wr_cmds: [hbm_sei_wr_cmd_address; HBM_WR_PAR_CMD_LIFO_LEN],
// derr[0:1] - 1st HBM cycle DERR output
// derr[2:3] - 2nd HBM cycle DERR output
//
    pub dbg_derr: __u8,
// extend to reach 8B
    pub pad: [__u8; 3],
}

//
// this struct represents the following sei causes:
// command parity, ECC double error, ECC single error, dfi error, cattrip,
// temperature read-out, read parity error and write parity error.
// some only use the header while some have extra data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_hbm_sei_data {
    pub hdr: hl_hbm_sei_header,
    pub ca_parity_even_info: hl_eq_hbm_sei_ca_par_intr_info,
    pub ca_parity_odd_info: hl_eq_hbm_sei_ca_par_intr_info,
    pub read_err_info: hl_eq_hbm_sei_read_err_intr_info,
    pub wr_parity_info: hl_eq_hbm_sei_wr_par_intr_info,
}

// Engine/farm arc interrupt type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_engine_arc_interrupt_type {
// Qman/farm ARC DCCM QUEUE FULL interrupt type
    ENGINE_ARC_DCCM_QUEUE_FULL_IRQ = 1
}

// Data structure specifies details of payload of DCCM QUEUE FULL interrupt
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_engine_arc_dccm_queue_full_irq {
// Queue index value which caused DCCM QUEUE FULL
    pub queue_index: __le32,
    pub pad: __le32,
}

// Data structure specifies details of QM/FARM ARC interrupt
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_engine_arc_intr_data {
// ARC engine id e.g.  DCORE0_TPC0_QM_ARC, DCORE0_TCP1_QM_ARC
    pub engine_id: __le32,
    pub /: *mut *mut __le32 intr_type; / enum hl_engine_arc_interrupt_type,
// More info related to the interrupt e.g. queue index
// incase of DCCM_QUEUE_FULL interrupt.
//
    pub payload: __le64,
    pub pad: [__le64; 5],
}

pub const ADDR_DEC_ADDRESS_COUNT_MAX: c_int = 4;
// Data structure specifies details of ADDR_DEC interrupt
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_addr_dec_intr_data {
    pub intr_cause: hl_eq_intr_cause,
    pub addr: [__le64; ADDR_DEC_ADDRESS_COUNT_MAX],
    pub addr_cnt: __u8,
    pub pad: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq_entry {
    pub hdr: hl_eq_header,
    pub data_placeholder: __le64,
    pub ecc_data: hl_eq_ecc_data,
    pub /: *mut *mut hl_eq_hbm_ecc_data hbm_ecc_data; / Obsolete,
    pub sm_sei_data: hl_eq_sm_sei_data,
    pub pkt_sync_err: cpucp_pkt_sync_err,
    pub fw_alive: hl_eq_fw_alive,
    pub intr_cause: hl_eq_intr_cause,
    pub pcie_drain_ind_data: hl_eq_pcie_drain_ind_data,
    pub razwi_info: hl_eq_razwi_info,
    pub razwi_with_intr_cause: hl_eq_razwi_with_intr_cause,
    pub /: *mut *mut hl_eq_hbm_sei_data sei_data; / Gaudi2 HBM,
    pub arc_data: hl_eq_engine_arc_intr_data,
    pub addr_dec: hl_eq_addr_dec_intr_data,
    pub data: [__le64; 7],
}

pub const EQ_CTL_READY_SHIFT: c_int = 31;
pub const EQ_CTL_READY_MASK: c_uint = 0x80000000;
pub const EQ_CTL_EVENT_MODE_SHIFT: c_int = 28;
pub const EQ_CTL_EVENT_MODE_MASK: c_uint = 0x70000000;
pub const EQ_CTL_EVENT_TYPE_SHIFT: c_int = 16;
pub const EQ_CTL_EVENT_TYPE_MASK: c_uint = 0x0FFF0000;
pub const EQ_CTL_INDEX_SHIFT: c_int = 0;
pub const EQ_CTL_INDEX_MASK: c_uint = 0x0000FFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pq_init_status {
    PQ_INIT_STATUS_NA = 0,
    PQ_INIT_STATUS_READY_FOR_CP,
    PQ_INIT_STATUS_READY_FOR_HOST,
    PQ_INIT_STATUS_READY_FOR_CP_SINGLE_MSI,
    PQ_INIT_STATUS_LEN_NOT_POWER_OF_TWO_ERR,
    PQ_INIT_STATUS_ILLEGAL_Q_ADDR_ERR
}

//
// CpuCP Primary Queue Packets
//
// During normal operation, the host's kernel driver needs to send various
// messages to CpuCP, usually either to SET some value into a H/W periphery or
// to GET the current value of some H/W periphery. For example, SET the
// frequency of MME/TPC and GET the value of the thermal sensor.
//
// These messages can be initiated either by the User application or by the
// host's driver itself, e.g. power management code. In either case, the
// communication from the host's driver to CpuCP will *always* be in
// synchronous mode, meaning that the host will send a single message and poll
// until the message was acknowledged and the results are ready (if results are
// needed).
//
// This means that only a single message can be sent at a time and the host's
// driver must wait for its result before sending the next message. Having said
// that, because these are control messages which are sent in a relatively low
// frequency, this limitation seems acceptable. It's important to note that
// in case of multiple devices, messages to different devices *can* be sent
// at the same time.
//
// The message, inputs/outputs (if relevant) and fence object will be located
// on the device DDR at an address that will be determined by the host's driver.
// During device initialization phase, the host will pass to CpuCP that address.
// Most of the message types will contain inputs/outputs inside the message
// itself. The common part of each message will contain the opcode of the
// message (its type) and a field representing a fence object.
//
// When the host's driver wishes to send a message to CPU CP, it will write the
// message contents to the device DDR, clear the fence object and then write to
// the PSOC_ARC1_AUX_SW_INTR, to issue interrupt 121 to ARC Management CPU.
//
// Upon receiving the interrupt (#121), CpuCP will read the message from the
// DDR. In case the message is a SET operation, CpuCP will first perform the
// operation and then write to the fence object on the device DDR. In case the
// message is a GET operation, CpuCP will first fill the results section on the
// device DDR and then write to the fence object. If an error occurred, CpuCP
// will fill the rc field with the right error code.
//
// In the meantime, the host's driver will poll on the fence object. Once the
// host sees that the fence object is signaled, it will read the results from
// the device DDR (if relevant) and resume the code execution in the host's
// driver.
//
// To use QMAN packets, the opcode must be the QMAN opcode, shifted by 8
// so the value being put by the host's driver matches the value read by CpuCP
//
// Non-QMAN packets should be limited to values 1 through (2^8 - 1)
//
// Detailed description:
//
// CPUCP_PACKET_DISABLE_PCI_ACCESS -
// After receiving this packet the embedded CPU must NOT issue PCI
// transactions (read/write) towards the Host CPU. This also include
// sending MSI-X interrupts.
// This packet is usually sent before the device is moved to D3Hot state.
//
// CPUCP_PACKET_ENABLE_PCI_ACCESS -
// After receiving this packet the embedded CPU is allowed to issue PCI
// transactions towards the Host CPU, including sending MSI-X interrupts.
// This packet is usually send after the device is moved to D0 state.
//
// CPUCP_PACKET_TEMPERATURE_GET -
// Fetch the current temperature / Max / Max Hyst / Critical
// Critical Hyst of a specified thermal sensor. The packet's
// arguments specify the desired sensor and the field to get.
//
// CPUCP_PACKET_VOLTAGE_GET -
// Fetch the voltage / Max / Min of a specified sensor. The packet's
// arguments specify the sensor and type.
//
// CPUCP_PACKET_CURRENT_GET -
// Fetch the current / Max / Min of a specified sensor. The packet's
// arguments specify the sensor and type.
//
// CPUCP_PACKET_FAN_SPEED_GET -
// Fetch the speed / Max / Min of a specified fan. The packet's
// arguments specify the sensor and type.
//
// CPUCP_PACKET_PWM_GET -
// Fetch the pwm value / mode of a specified pwm. The packet's
// arguments specify the sensor and type.
//
// CPUCP_PACKET_PWM_SET -
// Set the pwm value / mode of a specified pwm. The packet's
// arguments specify the sensor, type and value.
//
// CPUCP_PACKET_FREQUENCY_SET -
// Set the frequency of a specified PLL. The packet's arguments specify
// the PLL and the desired frequency. The actual frequency in the device
// might differ from the requested frequency.
//
// CPUCP_PACKET_FREQUENCY_GET -
// Fetch the frequency of a specified PLL. The packet's arguments specify
// the PLL.
//
// CPUCP_PACKET_LED_SET -
// Set the state of a specified led. The packet's arguments
// specify the led and the desired state.
//
// CPUCP_PACKET_I2C_WR -
// Write 32-bit value to I2C device. The packet's arguments specify the
// I2C bus, address and value.
//
// CPUCP_PACKET_I2C_RD -
// Read 32-bit value from I2C device. The packet's arguments specify the
// I2C bus and address.
//
// CPUCP_PACKET_INFO_GET -
// Fetch information from the device as specified in the packet's
// structure. The host's driver passes the max size it allows the CpuCP to
// write to the structure, to prevent data corruption in case of
// mismatched driver/FW versions.
//
// CPUCP_PACKET_FLASH_PROGRAM_REMOVED - this packet was removed
//
// CPUCP_PACKET_UNMASK_RAZWI_IRQ -
// Unmask the given IRQ. The IRQ number is specified in the value field.
// The packet is sent after receiving an interrupt and printing its
// relevant information.
//
// CPUCP_PACKET_UNMASK_RAZWI_IRQ_ARRAY -
// Unmask the given IRQs. The IRQs numbers are specified in an array right
// after the cpucp_packet structure, where its first element is the array
// length. The packet is sent after a soft reset was done in order to
// handle any interrupts that were sent during the reset process.
//
// CPUCP_PACKET_TEST -
// Test packet for CpuCP connectivity. The CPU will put the fence value
// in the result field.
//
// CPUCP_PACKET_FREQUENCY_CURR_GET -
// Fetch the current frequency of a specified PLL. The packet's arguments
// specify the PLL.
//
// CPUCP_PACKET_MAX_POWER_GET -
// Fetch the maximal power of the device.
//
// CPUCP_PACKET_MAX_POWER_SET -
// Set the maximal power of the device. The packet's arguments specify
// the power.
//
// CPUCP_PACKET_EEPROM_DATA_GET -
// Get EEPROM data from the CpuCP kernel. The buffer is specified in the
// addr field. The CPU will put the returned data size in the result
// field. In addition, the host's driver passes the max size it allows the
// CpuCP to write to the structure, to prevent data corruption in case of
// mismatched driver/FW versions.
//
// CPUCP_PACKET_NIC_INFO_GET -
// Fetch information from the device regarding the NIC. the host's driver
// passes the max size it allows the CpuCP to write to the structure, to
// prevent data corruption in case of mismatched driver/FW versions.
//
// CPUCP_PACKET_TEMPERATURE_SET -
// Set the value of the offset property of a specified thermal sensor.
// The packet's arguments specify the desired sensor and the field to
// set.
//
// CPUCP_PACKET_VOLTAGE_SET -
// Trigger the reset_history property of a specified voltage sensor.
// The packet's arguments specify the desired sensor and the field to
// set.
//
// CPUCP_PACKET_CURRENT_SET -
// Trigger the reset_history property of a specified current sensor.
// The packet's arguments specify the desired sensor and the field to
// set.
//
// CPUCP_PACKET_PCIE_THROUGHPUT_GET -
// Get throughput of PCIe.
// The packet's arguments specify the transaction direction (TX/RX).
// The window measurement is 10[msec], and the return value is in KB/sec.
//
// CPUCP_PACKET_PCIE_REPLAY_CNT_GET
// Replay count measures number of "replay" events, which is basicly
// number of retries done by PCIe.
//
// CPUCP_PACKET_TOTAL_ENERGY_GET -
// Total Energy is measurement of energy from the time FW Linux
// is loaded. It is calculated by multiplying the average power
// by time (passed from armcp start). The units are in MilliJouls.
//
// CPUCP_PACKET_PLL_INFO_GET -
// Fetch frequencies of PLL from the required PLL IP.
// The packet's arguments specify the device PLL type
// Pll type is the PLL from device pll_index enum.
// The result is composed of 4 outputs, each is 16-bit
// frequency in MHz.
//
// CPUCP_PACKET_POWER_GET -
// Fetch the present power consumption of the device (Current * Voltage).
//
// CPUCP_PACKET_NIC_PFC_SET -
// Enable/Disable the NIC PFC feature. The packet's arguments specify the
// NIC port, relevant lanes to configure and one bit indication for
// enable/disable.
//
// CPUCP_PACKET_NIC_FAULT_GET -
// Fetch the current indication for local/remote faults from the NIC MAC.
// The result is 32-bit value of the relevant register.
//
// CPUCP_PACKET_NIC_LPBK_SET -
// Enable/Disable the MAC loopback feature. The packet's arguments specify
// the NIC port, relevant lanes to configure and one bit indication for
// enable/disable.
//
// CPUCP_PACKET_NIC_MAC_INIT -
// Configure the NIC MAC channels. The packet's arguments specify the
// NIC port and the speed.
//
// CPUCP_PACKET_MSI_INFO_SET -
// set the index number for each supported msi type going from
// host to device
//
// CPUCP_PACKET_NIC_XPCS91_REGS_GET -
// Fetch the un/correctable counters values from the NIC MAC.
//
// CPUCP_PACKET_NIC_STAT_REGS_GET -
// Fetch various NIC MAC counters from the NIC STAT.
//
// CPUCP_PACKET_NIC_STAT_REGS_CLR -
// Clear the various NIC MAC counters in the NIC STAT.
//
// CPUCP_PACKET_NIC_STAT_REGS_ALL_GET -
// Fetch all NIC MAC counters from the NIC STAT.
//
// CPUCP_PACKET_IS_IDLE_CHECK -
// Check if the device is IDLE in regard to the DMA/compute engines
// and QMANs. The f/w will return a bitmask where each bit represents
// a different engine or QMAN according to enum cpucp_idle_mask.
// The bit will be 1 if the engine is NOT idle.
//
// CPUCP_PACKET_HBM_REPLACED_ROWS_INFO_GET -
// Fetch all HBM replaced-rows and prending to be replaced rows data.
//
// CPUCP_PACKET_HBM_PENDING_ROWS_STATUS -
// Fetch status of HBM rows pending replacement and need a reboot to
// be replaced.
//
// CPUCP_PACKET_POWER_SET -
// Resets power history of device to 0
//
// CPUCP_PACKET_ENGINE_CORE_ASID_SET -
// Packet to perform engine core ASID configuration
//
// CPUCP_PACKET_SEC_ATTEST_GET -
// Get the attestaion data that is collected during various stages of the
// boot sequence. the attestation data is also hashed with some unique
// number (nonce) provided by the host to prevent replay attacks.
// public key and certificate also provided as part of the FW response.
//
// CPUCP_PACKET_INFO_SIGNED_GET -
// Get the device information signed by the Trusted Platform device.
// device info data is also hashed with some unique number (nonce) provided
// by the host to prevent replay attacks. public key and certificate also
// provided as part of the FW response.
//
// CPUCP_PACKET_MONITOR_DUMP_GET -
// Get monitors registers dump from the CpuCP kernel.
// The CPU will put the registers dump in the a buffer allocated by the driver
// which address is passed via the CpuCp packet. In addition, the host's driver
// passes the max size it allows the CpuCP to write to the structure, to prevent
// data corruption in case of mismatched driver/FW versions.
// Obsolete.
//
// CPUCP_PACKET_GENERIC_PASSTHROUGH -
// Generic opcode for all firmware info that is only passed to host
// through the LKD, without getting parsed there.
//
// CPUCP_PACKET_ACTIVE_STATUS_SET -
// LKD sends FW indication whether device is free or in use, this indication is reported
// also to the BMC.
//
// CPUCP_PACKET_SOFT_RESET -
// Packet to perform soft-reset.
//
// CPUCP_PACKET_INTS_REGISTER -
// Packet to inform FW that queues have been established and LKD is ready to receive
// EQ events.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_packet_id {
    CPUCP_PACKET_DISABLE_PCI_ACCESS = 1,	/* internal */
    CPUCP_PACKET_ENABLE_PCI_ACCESS,		/* internal */
    CPUCP_PACKET_TEMPERATURE_GET,		/* sysfs */
    CPUCP_PACKET_VOLTAGE_GET,		/* sysfs */
    CPUCP_PACKET_CURRENT_GET,		/* sysfs */
    CPUCP_PACKET_FAN_SPEED_GET,		/* sysfs */
    CPUCP_PACKET_PWM_GET,			/* sysfs */
    CPUCP_PACKET_PWM_SET,			/* sysfs */
    CPUCP_PACKET_FREQUENCY_SET,		/* sysfs */
    CPUCP_PACKET_FREQUENCY_GET,		/* sysfs */
    CPUCP_PACKET_LED_SET,			/* debugfs */
    CPUCP_PACKET_I2C_WR,			/* debugfs */
    CPUCP_PACKET_I2C_RD,			/* debugfs */
    CPUCP_PACKET_INFO_GET,			/* IOCTL */
    CPUCP_PACKET_FLASH_PROGRAM_REMOVED,
    CPUCP_PACKET_UNMASK_RAZWI_IRQ,		/* internal */
    CPUCP_PACKET_UNMASK_RAZWI_IRQ_ARRAY,	/* internal */
    CPUCP_PACKET_TEST,			/* internal */
    CPUCP_PACKET_FREQUENCY_CURR_GET,	/* sysfs */
    CPUCP_PACKET_MAX_POWER_GET,		/* sysfs */
    CPUCP_PACKET_MAX_POWER_SET,		/* sysfs */
    CPUCP_PACKET_EEPROM_DATA_GET,		/* sysfs */
    CPUCP_PACKET_NIC_INFO_GET,		/* internal */
    CPUCP_PACKET_TEMPERATURE_SET,		/* sysfs */
    CPUCP_PACKET_VOLTAGE_SET,		/* sysfs */
    CPUCP_PACKET_CURRENT_SET,		/* sysfs */
    CPUCP_PACKET_PCIE_THROUGHPUT_GET,	/* internal */
    CPUCP_PACKET_PCIE_REPLAY_CNT_GET,	/* internal */
    CPUCP_PACKET_TOTAL_ENERGY_GET,		/* internal */
    CPUCP_PACKET_PLL_INFO_GET,		/* internal */
    CPUCP_PACKET_NIC_STATUS,		/* internal */
    CPUCP_PACKET_POWER_GET,			/* internal */
    CPUCP_PACKET_NIC_PFC_SET,		/* internal */
    CPUCP_PACKET_NIC_FAULT_GET,		/* internal */
    CPUCP_PACKET_NIC_LPBK_SET,		/* internal */
    CPUCP_PACKET_NIC_MAC_CFG,		/* internal */
    CPUCP_PACKET_MSI_INFO_SET,		/* internal */
    CPUCP_PACKET_NIC_XPCS91_REGS_GET,	/* internal */
    CPUCP_PACKET_NIC_STAT_REGS_GET,		/* internal */
    CPUCP_PACKET_NIC_STAT_REGS_CLR,		/* internal */
    CPUCP_PACKET_NIC_STAT_REGS_ALL_GET,	/* internal */
    CPUCP_PACKET_IS_IDLE_CHECK,		/* internal */
    CPUCP_PACKET_HBM_REPLACED_ROWS_INFO_GET,/* internal */
    CPUCP_PACKET_HBM_PENDING_ROWS_STATUS,	/* internal */
    CPUCP_PACKET_POWER_SET,			/* internal */
    CPUCP_PACKET_RESERVED,			/* not used */
    CPUCP_PACKET_ENGINE_CORE_ASID_SET,	/* internal */
    CPUCP_PACKET_RESERVED2,			/* not used */
    CPUCP_PACKET_SEC_ATTEST_GET,		/* internal */
    CPUCP_PACKET_INFO_SIGNED_GET,		/* internal */
    CPUCP_PACKET_RESERVED4,			/* not used */
    CPUCP_PACKET_MONITOR_DUMP_GET,		/* debugfs */
    CPUCP_PACKET_RESERVED5,			/* not used */
    CPUCP_PACKET_RESERVED6,			/* not used */
    CPUCP_PACKET_RESERVED7,			/* not used */
    CPUCP_PACKET_GENERIC_PASSTHROUGH,	/* IOCTL */
    CPUCP_PACKET_RESERVED8,			/* not used */
    CPUCP_PACKET_ACTIVE_STATUS_SET,		/* internal */
    CPUCP_PACKET_RESERVED9,			/* not used */
    CPUCP_PACKET_RESERVED10,		/* not used */
    CPUCP_PACKET_RESERVED11,		/* not used */
    CPUCP_PACKET_RESERVED12,		/* internal */
    CPUCP_PACKET_RESERVED13,                /* internal */
    CPUCP_PACKET_SOFT_RESET,		/* internal */
    CPUCP_PACKET_INTS_REGISTER,		/* internal */
    CPUCP_PACKET_ID_MAX			/* must be last */
}

pub const CPUCP_PACKET_FENCE_VAL: c_uint = 0xFE8CE7A5;
pub const CPUCP_PKT_CTL_RC_SHIFT: c_int = 12;
pub const CPUCP_PKT_CTL_RC_MASK: c_uint = 0x0000F000;
pub const CPUCP_PKT_CTL_OPCODE_SHIFT: c_int = 16;
pub const CPUCP_PKT_CTL_OPCODE_MASK: c_uint = 0x1FFF0000;
pub const CPUCP_PKT_RES_PLL_OUT0_SHIFT: c_int = 0;
pub const CPUCP_PKT_RES_PLL_OUT0_MASK: c_uint = 0x000000000000FFFFull;
pub const CPUCP_PKT_RES_PLL_OUT1_SHIFT: c_int = 16;
pub const CPUCP_PKT_RES_PLL_OUT1_MASK: c_uint = 0x00000000FFFF0000ull;
pub const CPUCP_PKT_RES_PLL_OUT2_SHIFT: c_int = 32;
pub const CPUCP_PKT_RES_PLL_OUT2_MASK: c_uint = 0x0000FFFF00000000ull;
pub const CPUCP_PKT_RES_PLL_OUT3_SHIFT: c_int = 48;
pub const CPUCP_PKT_RES_PLL_OUT3_MASK: c_uint = 0xFFFF000000000000ull;
pub const CPUCP_PKT_RES_EEPROM_OUT0_SHIFT: c_int = 0;
pub const CPUCP_PKT_RES_EEPROM_OUT0_MASK: c_uint = 0x000000000000FFFFull;
pub const CPUCP_PKT_RES_EEPROM_OUT1_SHIFT: c_int = 16;
pub const CPUCP_PKT_RES_EEPROM_OUT1_MASK: c_uint = 0x0000000000FF0000ull;
pub const CPUCP_PKT_VAL_PFC_IN1_SHIFT: c_int = 0;
pub const CPUCP_PKT_VAL_PFC_IN1_MASK: c_uint = 0x0000000000000001ull;
pub const CPUCP_PKT_VAL_PFC_IN2_SHIFT: c_int = 1;
pub const CPUCP_PKT_VAL_PFC_IN2_MASK: c_uint = 0x000000000000001Eull;
pub const CPUCP_PKT_VAL_LPBK_IN1_SHIFT: c_int = 0;
pub const CPUCP_PKT_VAL_LPBK_IN1_MASK: c_uint = 0x0000000000000001ull;
pub const CPUCP_PKT_VAL_LPBK_IN2_SHIFT: c_int = 1;
pub const CPUCP_PKT_VAL_LPBK_IN2_MASK: c_uint = 0x000000000000001Eull;
pub const CPUCP_PKT_VAL_MAC_CNT_IN1_SHIFT: c_int = 0;
pub const CPUCP_PKT_VAL_MAC_CNT_IN1_MASK: c_uint = 0x0000000000000001ull;
pub const CPUCP_PKT_VAL_MAC_CNT_IN2_SHIFT: c_int = 1;
pub const CPUCP_PKT_VAL_MAC_CNT_IN2_MASK: c_uint = 0x00000000FFFFFFFEull;
// heartbeat status bits
pub const CPUCP_PKT_HB_STATUS_EQ_FAULT_SHIFT: c_int = 0;
pub const CPUCP_PKT_HB_STATUS_EQ_FAULT_MASK: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_packet {
    pub /: *mut *mut __le64 value; / For SET packets,
    pub /: *mut *mut __le64 result; / For GET packets,
    pub /: *mut *mut __le64 addr; / For PQ,
}

//
// In legacy implemetations, i2c_len was not present,
// was unused and just added as pad.
// So if i2c_len is 0, it is treated as legacy
// and r/w 1 Byte, else if i2c_len is specified,
// its treated as new multibyte r/w support.
//
// TODO pll_reg is kept temporary before removal
// For any general request
// For frequency get/set
// For led set
// For get CpuCP info/EEPROM data/NIC info
//
// For any general status bitmask. Shall be used whenever the
// result cannot be used to hold general purpose data.
//
// For NIC requests
// For Generic packet sub index
// random, used once number, for security packets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_unmask_irq_arr_packet {
    pub cpucp_pkt: cpucp_packet,
    pub length: __le32,
    pub irqs: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_nic_status_packet {
    pub cpucp_pkt: cpucp_packet,
    pub length: __le32,
    pub data: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_array_data_packet {
    pub cpucp_pkt: cpucp_packet,
    pub length: __le32,
    pub data: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_led_index {
    CPUCP_LED0_INDEX = 0,
    CPUCP_LED1_INDEX,
    CPUCP_LED2_INDEX,
    CPUCP_LED_MAX_INDEX = CPUCP_LED2_INDEX
}

//
// enum cpucp_packet_rc - Error return code
// @cpucp_packet_success	-> in case of success.
// @cpucp_packet_invalid	-> this is to support first generation platforms.
// @cpucp_packet_fault		-> in case of processing error like failing to
// get device binding or semaphore etc.
// @cpucp_packet_invalid_pkt	-> when cpucp packet is un-supported.
// @cpucp_packet_invalid_params	-> when checking parameter like length of buffer
// or attribute value etc.
// @cpucp_packet_rc_max		-> It indicates size of enum so should be at last.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_packet_rc {
    cpucp_packet_success,
    cpucp_packet_invalid,
    cpucp_packet_fault,
    cpucp_packet_invalid_pkt,
    cpucp_packet_invalid_params,
    cpucp_packet_rc_max
}

//
// cpucp_temp_type should adhere to hwmon_temp_attributes
// defined in Linux kernel hwmon.h file
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_temp_type {
    cpucp_temp_input,
    cpucp_temp_min = 4,
    cpucp_temp_min_hyst,
    cpucp_temp_max = 6,
    cpucp_temp_max_hyst,
    cpucp_temp_crit,
    cpucp_temp_crit_hyst,
    cpucp_temp_offset = 19,
    cpucp_temp_lowest = 21,
    cpucp_temp_highest = 22,
    cpucp_temp_reset_history = 23,
    cpucp_temp_warn = 24,
    cpucp_temp_max_crit = 25,
    cpucp_temp_max_warn = 26,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_in_attributes {
    cpucp_in_input,
    cpucp_in_min,
    cpucp_in_max,
    cpucp_in_lowest = 6,
    cpucp_in_highest = 7,
    cpucp_in_reset_history,
    cpucp_in_intr_alarm_a,
    cpucp_in_intr_alarm_b,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_curr_attributes {
    cpucp_curr_input,
    cpucp_curr_min,
    cpucp_curr_max,
    cpucp_curr_lowest = 6,
    cpucp_curr_highest = 7,
    cpucp_curr_reset_history
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_fan_attributes {
    cpucp_fan_input,
    cpucp_fan_min = 2,
    cpucp_fan_max
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_pwm_attributes {
    cpucp_pwm_input,
    cpucp_pwm_enable
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_pcie_throughput_attributes {
    cpucp_pcie_throughput_tx,
    cpucp_pcie_throughput_rx
}

// TODO temporary kept before removal
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_pll_reg_attributes {
    cpucp_pll_nr_reg,
    cpucp_pll_nf_reg,
    cpucp_pll_od_reg,
    cpucp_pll_div_factor_reg,
    cpucp_pll_div_sel_reg
}

// TODO temporary kept before removal
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_pll_type_attributes {
    cpucp_pll_cpu,
    cpucp_pll_pci,
}

//
// cpucp_power_type aligns with hwmon_power_attributes
// defined in Linux kernel hwmon.h file
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_power_type {
    CPUCP_POWER_INPUT = 8,
    CPUCP_POWER_INPUT_HIGHEST = 9,
    CPUCP_POWER_RESET_INPUT_HISTORY = 11
}

//
// MSI type enumeration table for all ASICs and future SW versions.
// For future ASIC-LKD compatibility, we can only add new enumerations.
// at the end of the table (before CPUCP_NUM_OF_MSI_TYPES).
// Changing the order of entries or removing entries is not allowed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_msi_type {
    CPUCP_EVENT_QUEUE_MSI_TYPE,
    CPUCP_NIC_PORT1_MSI_TYPE,
    CPUCP_NIC_PORT3_MSI_TYPE,
    CPUCP_NIC_PORT5_MSI_TYPE,
    CPUCP_NIC_PORT7_MSI_TYPE,
    CPUCP_NIC_PORT9_MSI_TYPE,
    CPUCP_EVENT_QUEUE_ERR_MSI_TYPE,
    CPUCP_NUM_OF_MSI_TYPES
}

//
// PLL enumeration table used for all ASICs and future SW versions.
// For future ASIC-LKD compatibility, we can only add new enumerations.
// at the end of the table.
// Changing the order of entries or removing entries is not allowed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pll_index {
    CPU_PLL = 0,
    PCI_PLL = 1,
    NIC_PLL = 2,
    DMA_PLL = 3,
    MESH_PLL = 4,
    MME_PLL = 5,
    TPC_PLL = 6,
    IF_PLL = 7,
    SRAM_PLL = 8,
    NS_PLL = 9,
    HBM_PLL = 10,
    MSS_PLL = 11,
    DDR_PLL = 12,
    VID_PLL = 13,
    BANK_PLL = 14,
    MMU_PLL = 15,
    IC_PLL = 16,
    MC_PLL = 17,
    EMMC_PLL = 18,
    D2D_PLL = 19,
    CS_PLL = 20,
    C2C_PLL = 21,
    NCH_PLL = 22,
    C2M_PLL = 23,
    PLL_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rl_index {
    TPC_RL = 0,
    MME_RL,
    EDMA_RL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvt_index {
    PVT_SW,
    PVT_SE,
    PVT_NW,
    PVT_NE
}

// Event Queue Packets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eq_generic_event {
    pub data: [__le64; 7],
}

//
// CpuCP info
//
pub const CARD_NAME_MAX_LEN: c_int = 16;
pub const CPUCP_MAX_SENSORS: c_int = 128;
pub const CPUCP_MAX_NICS: c_int = 128;
pub const CPUCP_LANES_PER_NIC: c_int = 4;
pub const CPUCP_NIC_QSFP_EEPROM_MAX_LEN: c_int = 1024;

pub const CPUCP_HBM_ROW_REPLACE_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_sensor {
    pub type: __le32,
    pub flags: __le32,
}

//
// struct cpucp_card_types - ASIC card type.
// @cpucp_card_type_pci: PCI card.
// @cpucp_card_type_pmc: PCI Mezzanine Card.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_card_types {
    cpucp_card_type_pci,
    cpucp_card_type_pmc
}

pub const CPUCP_SEC_CONF_ENABLED_SHIFT: c_int = 0;
pub const CPUCP_SEC_CONF_ENABLED_MASK: c_uint = 0x00000001;
pub const CPUCP_SEC_CONF_FLASH_WP_SHIFT: c_int = 1;
pub const CPUCP_SEC_CONF_FLASH_WP_MASK: c_uint = 0x00000002;
pub const CPUCP_SEC_CONF_EEPROM_WP_SHIFT: c_int = 2;
pub const CPUCP_SEC_CONF_EEPROM_WP_MASK: c_uint = 0x00000004;
//
// struct cpucp_security_info - Security information.
// @config: configuration bit field
// @keys_num: number of stored keys
// @revoked_keys: revoked keys bit field
// @min_svn: minimal security version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_security_info {
    pub config: __u8,
    pub keys_num: __u8,
    pub revoked_keys: __u8,
    pub min_svn: __u8,
}

//
// struct cpucp_info - Info from CpuCP that is necessary to the host's driver
// @sensors: available sensors description.
// @kernel_version: CpuCP linux kernel version.
// @reserved: reserved field.
// @card_type: card configuration type.
// @card_location: in a server, each card has different connections topology
// depending on its location (relevant for PMC card type)
// @cpld_version: CPLD programmed F/W version.
// @infineon_version: Infineon main DC-DC version.
// @fuse_version: silicon production FUSE information.
// @thermal_version: thermald S/W version.
// @cpucp_version: CpuCP S/W version.
// @infineon_second_stage_version: Infineon 2nd stage DC-DC version.
// @dram_size: available DRAM size.
// @card_name: card name that will be displayed in HWMON subsystem on the host
// @tpc_binning_mask: TPC binning mask, 1 bit per TPC instance
// (0 = functional, 1 = binned)
// @decoder_binning_mask: Decoder binning mask, 1 bit per decoder instance
// (0 = functional, 1 = binned), maximum 1 per dcore
// @sram_binning: Categorize SRAM functionality
// (0 = fully functional, 1 = lower-half is not functional,
// 2 = upper-half is not functional)
// @sec_info: security information
// @cpld_timestamp: CPLD programmed F/W timestamp.
// @pll_map: Bit map of supported PLLs for current ASIC version.
// @mme_binning_mask: MME binning mask,
// bits [0:6]   <==> dcore0 mme fma
// bits [7:13]  <==> dcore1 mme fma
// bits [14:20] <==> dcore0 mme ima
// bits [21:27] <==> dcore1 mme ima
// For each group, if the 6th bit is set then first 5 bits
// represent the col's idx [0-31], otherwise these bits are
// ignored, and col idx 32 is binned. 7th bit is don't care.
// @dram_binning_mask: DRAM binning mask, 1 bit per dram instance
// (0 = functional 1 = binned)
// @memory_repair_flag: eFuse flag indicating memory repair
// @edma_binning_mask: EDMA binning mask, 1 bit per EDMA instance
// (0 = functional 1 = binned)
// @xbar_binning_mask: Xbar binning mask, 1 bit per Xbar instance
// (0 = functional 1 = binned)
// @interposer_version: Interposer version programmed in eFuse
// @substrate_version: Substrate version programmed in eFuse
// @eq_health_check_supported: eq health check feature supported in FW.
// @fw_hbm_region_size: Size in bytes of FW reserved region in HBM.
// @fw_os_version: Firmware OS Version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_info {
    pub sensors: [cpucp_sensor; CPUCP_MAX_SENSORS],
    pub kernel_version: [__u8; VERSION_MAX_LEN],
    pub reserved1: __le32,
    pub card_type: __le32,
    pub card_location: __le32,
    pub cpld_version: __le32,
    pub infineon_version: __le32,
    pub fuse_version: [__u8; VERSION_MAX_LEN],
    pub thermal_version: [__u8; VERSION_MAX_LEN],
    pub cpucp_version: [__u8; VERSION_MAX_LEN],
    pub infineon_second_stage_version: __le32,
    pub dram_size: __le64,
    pub card_name: [c_char; CARD_NAME_MAX_LEN],
    pub tpc_binning_mask: __le64,
    pub decoder_binning_mask: __le64,
    pub sram_binning: __u8,
    pub dram_binning_mask: __u8,
    pub memory_repair_flag: __u8,
    pub edma_binning_mask: __u8,
    pub xbar_binning_mask: __u8,
    pub interposer_version: __u8,
    pub substrate_version: __u8,
    pub eq_health_check_supported: __u8,
    pub sec_info: cpucp_security_info,
    pub cpld_timestamp: __le32,
    pub pll_map: [__u8; PLL_MAP_LEN],
    pub mme_binning_mask: __le64,
    pub fw_os_version: [__u8; VERSION_MAX_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_mac_addr {
    pub mac_addr: [__u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_serdes_type {
    TYPE_1_SERDES_TYPE,
    TYPE_2_SERDES_TYPE,
    HLS1_SERDES_TYPE,
    HLS1H_SERDES_TYPE,
    HLS2_SERDES_TYPE,
    HLS2_TYPE_1_SERDES_TYPE,
    MAX_NUM_SERDES_TYPE,		/* number of types */
    UNKNOWN_SERDES_TYPE = 0xFFFF	/* serdes_type is u16 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_nic_info {
    pub mac_addrs: [cpucp_mac_addr; CPUCP_MAX_NICS],
    pub link_mask: [__le64; CPUCP_NIC_MASK_ARR_LEN],
    pub pol_tx_mask: [__le64; CPUCP_NIC_POLARITY_ARR_LEN],
    pub pol_rx_mask: [__le64; CPUCP_NIC_POLARITY_ARR_LEN],
    pub link_ext_mask: [__le64; CPUCP_NIC_MASK_ARR_LEN],
    pub qsfp_eeprom: [__u8; CPUCP_NIC_QSFP_EEPROM_MAX_LEN],
    pub auto_neg_mask: [__le64; CPUCP_NIC_MASK_ARR_LEN],
    pub /: *mut *mut __le16 serdes_type; / enum cpucp_serdes_type,
    pub tx_swap_map: [__le16; CPUCP_MAX_NICS],
    pub reserved: [__u8; 6],
}

pub const PAGE_DISCARD_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_discard_info {
    pub num_entries: __u8,
    pub reserved: [__u8; 7],
    pub mmu_page_idx: [__le32; PAGE_DISCARD_MAX],
}

//
// struct frac_val - fracture value represented by "integer.frac".
// @integer: the integer part of the fracture value;
// @frac: the fracture part of the fracture value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frac_val {
    pub integer: __le16,
    pub frac: __le16,
}

//
// struct ser_val - the SER (symbol error rate) value is represented by "integer * 10 ^ -exp".
// @integer: the integer part of the SER value;
// @exp: the exponent part of the SER value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ser_val {
    pub integer: __le16,
    pub exp: __le16,
}

//
// struct cpucp_nic_status - describes the status of a NIC port.
// @port: NIC port index.
// @bad_format_cnt: e.g. CRC.
// @responder_out_of_sequence_psn_cnt: e.g NAK.
// @high_ber_reinit_cnt: link reinit due to high BER.
// @correctable_err_cnt: e.g. bit-flip.
// @uncorrectable_err_cnt: e.g. MAC errors.
// @retraining_cnt: re-training counter.
// @up: is port up.
// @pcs_link: has PCS link.
// @phy_ready: is PHY ready.
// @auto_neg: is Autoneg enabled.
// @timeout_retransmission_cnt: timeout retransmission events.
// @high_ber_cnt: high ber events.
// @pre_fec_ser: pre FEC SER value.
// @post_fec_ser: post FEC SER value.
// @throughput: measured throughput.
// @latency: measured latency.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_nic_status {
    pub port: __le32,
    pub bad_format_cnt: __le32,
    pub responder_out_of_sequence_psn_cnt: __le32,
    pub high_ber_reinit: __le32,
    pub correctable_err_cnt: __le32,
    pub uncorrectable_err_cnt: __le32,
    pub retraining_cnt: __le32,
    pub up: __u8,
    pub pcs_link: __u8,
    pub phy_ready: __u8,
    pub auto_neg: __u8,
    pub timeout_retransmission_cnt: __le32,
    pub high_ber_cnt: __le32,
    pub pre_fec_ser: ser_val,
    pub post_fec_ser: ser_val,
    pub bandwidth: frac_val,
    pub lat: frac_val,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpucp_hbm_row_replace_cause {
    REPLACE_CAUSE_DOUBLE_ECC_ERR,
    REPLACE_CAUSE_MULTI_SINGLE_ECC_ERR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_hbm_row_info {
    pub hbm_idx: __u8,
    pub pc: __u8,
    pub sid: __u8,
    pub bank_idx: __u8,
    pub row_addr: __le16,
    pub /: *mut *mut __u8 replaced_row_cause; / enum cpucp_hbm_row_replace_cause,
    pub pad: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_hbm_row_replaced_rows_info {
    pub num_replaced_rows: __le16,
    pub pad: [__u8; 6],
    pub replaced_rows: [cpucp_hbm_row_info; CPUCP_HBM_ROW_REPLACE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpu_reset_status {
    CPU_RST_STATUS_NA = 0,
    CPU_RST_STATUS_SOFT_RST_DONE = 1,
}

pub const SEC_PCR_DATA_BUF_SZ: c_int = 256;

//
// struct cpucp_sec_attest_info - attestation report of the boot
// @pcr_data: raw values of the PCR registers
// @pcr_num_reg: number of PCR registers in the pcr_data array
// @pcr_reg_len: length of each PCR register in the pcr_data array (bytes)
// @nonce: number only used once. random number provided by host. this also
// passed to the quote command as a qualifying data.
// @pcr_quote_len: length of the attestation quote data (bytes)
// @pcr_quote: attestation report data structure
// @quote_sig_len: length of the attestation report signature (bytes)
// @quote_sig: signature structure of the attestation report
// @pub_data_len: length of the public data (bytes)
// @public_data: public key for the signed attestation
// (outPublic + name + qualifiedName)
// @certificate_len: length of the certificate (bytes)
// @certificate: certificate for the attestation signing key
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_sec_attest_info {
    pub pcr_data: [__u8; SEC_PCR_DATA_BUF_SZ],
    pub pcr_num_reg: __u8,
    pub pcr_reg_len: __u8,
    pub pad0: __le16,
    pub nonce: __le32,
    pub pcr_quote_len: __le16,
    pub pcr_quote: [__u8; SEC_PCR_QUOTE_BUF_SZ],
    pub quote_sig_len: __u8,
    pub quote_sig: [__u8; SEC_SIGNATURE_BUF_SZ],
    pub pub_data_len: __le16,
    pub public_data: [__u8; SEC_PUB_DATA_BUF_SZ],
    pub certificate_len: __le16,
    pub certificate: [__u8; SEC_CERTIFICATE_BUF_SZ],
}

//
// struct cpucp_dev_info_signed - device information signed by a secured device
// @info: device information structure as defined above
// @nonce: number only used once. random number provided by host. this number is
// hashed and signed along with the device information.
// @info_sig_len: length of the attestation signature (bytes)
// @info_sig: signature of the info + nonce data.
// @pub_data_len: length of the public data (bytes)
// @public_data: public key info signed info data
// (outPublic + name + qualifiedName)
// @certificate_len: length of the certificate (bytes)
// @certificate: certificate for the signing key
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_dev_info_signed {
    pub /: *mut *mut cpucp_info info; / assumed to be 64bit aligned,
    pub nonce: __le32,
    pub pad0: __le32,
    pub info_sig_len: __u8,
    pub info_sig: [__u8; SEC_SIGNATURE_BUF_SZ],
    pub pub_data_len: __le16,
    pub public_data: [__u8; SEC_PUB_DATA_BUF_SZ],
    pub certificate_len: __le16,
    pub certificate: [__u8; SEC_CERTIFICATE_BUF_SZ],
}

pub const DCORE_MON_REGS_SZ: c_int = 512;
//
// struct dcore_monitor_regs_data - DCORE monitor regs data.
// the structure follows sync manager block layout. Obsolete.
// @mon_pay_addrl: array of payload address low bits.
// @mon_pay_addrh: array of payload address high bits.
// @mon_pay_data: array of payload data.
// @mon_arm: array of monitor arm.
// @mon_status: array of monitor status.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcore_monitor_regs_data {
    pub mon_pay_addrl: [__le32; DCORE_MON_REGS_SZ],
    pub mon_pay_addrh: [__le32; DCORE_MON_REGS_SZ],
    pub mon_pay_data: [__le32; DCORE_MON_REGS_SZ],
    pub mon_arm: [__le32; DCORE_MON_REGS_SZ],
    pub mon_status: [__le32; DCORE_MON_REGS_SZ],
}

// contains SM data for each SYNC_MNGR (Obsolete)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucp_monitor_dump {
    pub sync_mngr_w_s: dcore_monitor_regs_data,
    pub sync_mngr_e_s: dcore_monitor_regs_data,
    pub sync_mngr_w_n: dcore_monitor_regs_data,
    pub sync_mngr_e_n: dcore_monitor_regs_data,
}

//
// The Type of the generic request (and other input arguments) will be fetched from user by reading
// from "pkt_subidx" field in struct cpucp_packet.
//
// HL_PASSTHROUGHT_VERSIONS	- Fetch all firmware versions.
// HL_GET_ERR_COUNTERS_CMD	- Command to get error counters
// HL_GET_P_STATE		- get performance state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_passthrough_type {
    HL_PASSTHROUGH_VERSIONS,
    HL_GET_ERR_COUNTERS_CMD,
    HL_GET_P_STATE,
}
