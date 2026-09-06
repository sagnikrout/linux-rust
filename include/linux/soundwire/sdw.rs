//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soundwire/sdw.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2015-17 Intel Corporation.

// SDW spec defines and enums, as defined by MIPI 1.1. Spec
// SDW Broadcast Device Number
pub const SDW_BROADCAST_DEV_NUM: c_int = 15;
// SDW Enumeration Device Number
pub const SDW_ENUM_DEV_NUM: c_int = 0;
// SDW Group Device Numbers
pub const SDW_GROUP12_DEV_NUM: c_int = 12;
pub const SDW_GROUP13_DEV_NUM: c_int = 13;
// SDW Master Device Number, not supported yet
pub const SDW_MASTER_DEV_NUM: c_int = 14;
pub const SDW_NUM_DEV_ID_REGISTERS: c_int = 6;
// frame shape defines
//
// Note: The maximum row define in SoundWire spec 1.1 is 23. In order to
// fill hole with 0, one more dummy entry is added
//
pub const SDW_FRAME_ROWS: c_int = 24;
pub const SDW_FRAME_COLS: c_int = 8;

pub const SDW_FRAME_CTRL_BITS: c_int = 48;
pub const SDW_MAX_DEVICES: c_int = 11;
pub const SDW_FW_MAX_DEVICES: c_int = 16;
pub const SDW_MAX_PORTS: c_int = 15;

pub const SDW_MAX_LANES: c_int = 8;
//
// constants for flow control, ports and transport
//
// these are bit masks as devices can have multiple capabilities
//
// flow modes for SDW port. These can be isochronous, tx controlled,
// rx controlled or async
//
pub const SDW_PORT_FLOW_MODE_ISOCH: c_int = 0;

// sample packaging for block. It can be per port or per channel

//
// enum sdw_slave_status - Slave status
// @SDW_SLAVE_UNATTACHED: Slave is not attached with the bus.
// @SDW_SLAVE_ATTACHED: Slave is attached with bus.
// @SDW_SLAVE_ALERT: Some alert condition on the Slave
// @SDW_SLAVE_RESERVED: Reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_slave_status {
    SDW_SLAVE_UNATTACHED = 0,
    SDW_SLAVE_ATTACHED = 1,
    SDW_SLAVE_ALERT = 2,
    SDW_SLAVE_RESERVED = 3,
}

//
// enum sdw_clk_stop_type: clock stop operations
//
// @SDW_CLK_PRE_PREPARE: pre clock stop prepare
// @SDW_CLK_POST_PREPARE: post clock stop prepare
// @SDW_CLK_PRE_DEPREPARE: pre clock stop de-prepare
// @SDW_CLK_POST_DEPREPARE: post clock stop de-prepare
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_clk_stop_type {
    SDW_CLK_PRE_PREPARE = 0,
    SDW_CLK_POST_PREPARE,
    SDW_CLK_PRE_DEPREPARE,
    SDW_CLK_POST_DEPREPARE,
}

//
// enum sdw_command_response - Command response as defined by SDW spec
// @SDW_CMD_OK: cmd was successful
// @SDW_CMD_IGNORED: cmd was ignored
// @SDW_CMD_FAIL: cmd was NACKed
// @SDW_CMD_TIMEOUT: cmd timedout
// @SDW_CMD_FAIL_OTHER: cmd failed due to other reason than above
//
// NOTE: The enum is different than actual Spec as response in the Spec is
// combination of ACK/NAK bits
//
// SDW_CMD_TIMEOUT/FAIL_OTHER is defined for SW use, not in spec
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_command_response {
    SDW_CMD_OK = 0,
    SDW_CMD_IGNORED = 1,
    SDW_CMD_FAIL = 2,
    SDW_CMD_TIMEOUT = 3,
    SDW_CMD_FAIL_OTHER = 4,
}

// block group count enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_dpn_grouping {
    SDW_BLK_GRP_CNT_1 = 0,
    SDW_BLK_GRP_CNT_2 = 1,
    SDW_BLK_GRP_CNT_3 = 2,
    SDW_BLK_GRP_CNT_4 = 3,
}

// block packing mode enum
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_dpn_pkg_mode {
    SDW_BLK_PKG_PER_PORT = 0,
    SDW_BLK_PKG_PER_CHANNEL = 1
}

//
// enum sdw_stream_type: data stream type
//
// @SDW_STREAM_PCM: PCM data stream
// @SDW_STREAM_PDM: PDM data stream
// @SDW_STREAM_BPT: BPT data stream
//
// spec doesn't define this, but is used in implementation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_stream_type {
    SDW_STREAM_PCM = 0,
    SDW_STREAM_PDM = 1,
    SDW_STREAM_BPT = 2,
}

//
// enum sdw_data_direction: Data direction
//
// @SDW_DATA_DIR_RX: Data into Port
// @SDW_DATA_DIR_TX: Data out of Port
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_data_direction {
    SDW_DATA_DIR_RX = 0,
    SDW_DATA_DIR_TX = 1,
}

//
// enum sdw_port_data_mode: Data Port mode
//
// @SDW_PORT_DATA_MODE_NORMAL: Normal data mode where audio data is received
// and transmitted.
// @SDW_PORT_DATA_MODE_PRBS: Test mode which uses a PRBS generator to produce
// a pseudo random data pattern that is transferred
// @SDW_PORT_DATA_MODE_STATIC_0: Simple test mode which uses static value of
// logic 0. The encoding will result in no signal transitions
// @SDW_PORT_DATA_MODE_STATIC_1: Simple test mode which uses static value of
// logic 1. The encoding will result in signal transitions at every bitslot
// owned by this Port
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_port_data_mode {
    SDW_PORT_DATA_MODE_NORMAL = 0,
    SDW_PORT_DATA_MODE_PRBS = 1,
    SDW_PORT_DATA_MODE_STATIC_0 = 2,
    SDW_PORT_DATA_MODE_STATIC_1 = 3,
}

//
// SDW properties, defined in MIPI DisCo spec v1.0
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_clk_stop_reset_behave {
    SDW_CLK_STOP_KEEP_STATUS = 1,
}

//
// enum sdw_p15_behave - Slave Port 15 behaviour when the Master attempts a
// read
// @SDW_P15_READ_IGNORED: Read is ignored
// @SDW_P15_CMD_OK: Command is ok
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_p15_behave {
    SDW_P15_READ_IGNORED = 0,
    SDW_P15_CMD_OK = 1,
}

//
// enum sdw_dpn_type - Data port types
// @SDW_DPN_FULL: Full Data Port is supported
// @SDW_DPN_SIMPLE: Simplified Data Port as defined in spec.
// DPN_SampleCtrl2, DPN_OffsetCtrl2, DPN_HCtrl and DPN_BlockCtrl3
// are not implemented.
// @SDW_DPN_REDUCED: Reduced Data Port as defined in spec.
// DPN_SampleCtrl2, DPN_HCtrl are not implemented.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_dpn_type {
    SDW_DPN_FULL = 0,
    SDW_DPN_SIMPLE = 1,
    SDW_DPN_REDUCED = 2,
}

//
// enum sdw_clk_stop_mode - Clock Stop modes
// @SDW_CLK_STOP_MODE0: Slave can continue operation seamlessly on clock
// restart
// @SDW_CLK_STOP_MODE1: Slave may have entered a deeper power-saving mode,
// not capable of continuing operation seamlessly when the clock restarts
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_clk_stop_mode {
    SDW_CLK_STOP_MODE0 = 0,
    SDW_CLK_STOP_MODE1 = 1,
}

//
// struct sdw_dp0_prop - DP0 properties
// @words: wordlengths supported
// @max_word: Maximum number of bits in a Payload Channel Sample, 1 to 64
// (inclusive)
// @min_word: Minimum number of bits in a Payload Channel Sample, 1 to 64
// (inclusive)
// @num_words: number of wordlengths supported
// @ch_prep_timeout: Port-specific timeout value, in milliseconds
// @BRA_flow_controlled: Slave implementation results in an OK_NotReady
// response
// @simple_ch_prep_sm: If channel prepare sequence is required
// @imp_def_interrupts: If set, each bit corresponds to support for
// implementation-defined interrupts
// @num_lanes: array size of @lane_list
// @lane_list: indicates which Lanes can be used by DP0
//
// The wordlengths are specified by Spec as max, min AND number of
// discrete values, implementation can define based on the wordlengths they
// support
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_dp0_prop {
    pub words: *mut u32,
    pub max_word: u32,
    pub min_word: u32,
    pub num_words: u32,
    pub ch_prep_timeout: u32,
    pub BRA_flow_controlled: bool,
    pub simple_ch_prep_sm: bool,
    pub imp_def_interrupts: bool,
    pub num_lanes: c_int,
    pub lane_list: *mut u32,
}

//
// struct sdw_dpn_prop - Data Port DPn properties
// @num: port number
// @max_word: Maximum number of bits in a Payload Channel Sample, 1 to 64
// (inclusive)
// @min_word: Minimum number of bits in a Payload Channel Sample, 1 to 64
// (inclusive)
// @num_words: Number of discrete supported wordlengths
// @words: Discrete supported wordlength
// @type: Data port type. Full, Simplified or Reduced
// @max_grouping: Maximum number of samples that can be grouped together for
// a full data port
// @ch_prep_timeout: Port-specific timeout value, in milliseconds
// @imp_def_interrupts: If set, each bit corresponds to support for
// implementation-defined interrupts
// @max_ch: Maximum channels supported
// @min_ch: Minimum channels supported
// @num_channels: Number of discrete channels supported
// @num_ch_combinations: Number of channel combinations supported
// @channels: Discrete channels supported
// @ch_combinations: Channel combinations supported
// @lane_list: indicates which Lanes can be used by DPn
// @num_lanes: array size of @lane_list
// @modes: SDW mode supported
// @max_async_buffer: Number of samples that this port can buffer in
// asynchronous modes
// @port_encoding: Payload Channel Sample encoding schemes supported
// @block_pack_mode: Type of block port mode supported
// @read_only_wordlength: Read Only wordlength field in DPN_BlockCtrl1 register
// @simple_ch_prep_sm: If the port supports simplified channel prepare state
// machine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_dpn_prop {
    pub num: u32,
    pub max_word: u32,
    pub min_word: u32,
    pub num_words: u32,
    pub words: *mut u32,
    pub type: sdw_dpn_type,
    pub max_grouping: u32,
    pub ch_prep_timeout: u32,
    pub imp_def_interrupts: u32,
    pub max_ch: u32,
    pub min_ch: u32,
    pub num_channels: u32,
    pub num_ch_combinations: u32,
    pub channels: *mut u32,
    pub ch_combinations: *mut u32,
    pub lane_list: *mut u32,
    pub num_lanes: c_int,
    pub modes: u32,
    pub max_async_buffer: u32,
    pub port_encoding: u32,
    pub block_pack_mode: bool,
    pub read_only_wordlength: bool,
    pub simple_ch_prep_sm: bool,
}

//
// struct sdw_slave_prop - SoundWire Slave properties
// @dp0_prop: Data Port 0 properties
// @src_dpn_prop: Source Data Port N properties
// @sink_dpn_prop: Sink Data Port N properties
// @mipi_revision: Spec version of the implementation
// @wake_capable: Wake-up events are supported
// @test_mode_capable: If test mode is supported
// @clk_stop_mode1: Clock-Stop Mode 1 is supported
// @simple_clk_stop_capable: Simple clock mode is supported
// @clk_stop_timeout: Worst-case latency of the Clock Stop Prepare State
// Machine transitions, in milliseconds
// @ch_prep_timeout: Worst-case latency of the Channel Prepare State Machine
// transitions, in milliseconds
// @reset_behave: Slave keeps the status of the SlaveStopClockPrepare
// state machine (P=1 SCSP_SM) after exit from clock-stop mode1
// @high_PHY_capable: Slave is HighPHY capable
// @paging_support: Slave implements paging registers SCP_AddrPage1 and
// SCP_AddrPage2
// @bank_delay_support: Slave implements bank delay/bridge support registers
// SCP_BankDelay and SCP_NextFrame
// @lane_control_support: Slave supports lane control
// @p15_behave: Slave behavior when the Master attempts a read to the Port15
// alias
// @master_count: Number of Masters present on this Slave
// @source_ports: Bitmap identifying source ports
// @sink_ports: Bitmap identifying sink ports
// @quirks: bitmask identifying deltas from the MIPI specification
// @sdca_interrupt_register_list: indicates which sets of SDCA interrupt status
// and masks are supported
// @commit_register_supported: is PCP_Commit register supported
// @scp_int1_mask: SCP_INT1_MASK desired settings
// @lane_maps: Lane mapping for the slave, only valid if lane_control_support is set
// @bra_block_alignment: If non-zero the length of data in a BRA frame must be
// a multiple of this number of bytes.
// @bra_max_data_per_frame: If non-zero the maximum data payload size (in bytes per
// frame excluding header, CRC, and footer) for this BRA Mode
// @clock_reg_supported: the Peripheral implements the clock base and scale
// registers introduced with the SoundWire 1.2 specification. SDCA devices
// do not need to set this boolean property as the registers are required.
// @use_domain_irq: call actual IRQ handler on slave, as well as callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_slave_prop {
    pub dp0_prop: *mut sdw_dp0_prop,
    pub src_dpn_prop: *mut sdw_dpn_prop,
    pub sink_dpn_prop: *mut sdw_dpn_prop,
    pub mipi_revision: u32,
    pub wake_capable: bool,
    pub test_mode_capable: bool,
    pub clk_stop_mode1: bool,
    pub simple_clk_stop_capable: bool,
    pub clk_stop_timeout: u32,
    pub ch_prep_timeout: u32,
    pub reset_behave: sdw_clk_stop_reset_behave,
    pub high_PHY_capable: bool,
    pub paging_support: bool,
    pub bank_delay_support: bool,
    pub lane_control_support: bool,
    pub p15_behave: sdw_p15_behave,
    pub master_count: u32,
    pub source_ports: u32,
    pub sink_ports: u32,
    pub quirks: u32,
    pub sdca_interrupt_register_list: u32,
    pub commit_register_supported: u8,
    pub scp_int1_mask: u8,
    pub lane_maps: [u8; SDW_MAX_LANES],
    pub bra_block_alignment: u32,
    pub bra_max_data_per_frame: u32,
    pub clock_reg_supported: bool,
    pub use_domain_irq: bool,
}

//
// struct sdw_master_prop - Master properties
// @clk_gears: Clock gears supported
// @clk_freq: Clock frequencies supported, in Hz
// @quirks: bitmask identifying optional behavior beyond the scope of the MIPI specification
// @revision: MIPI spec version of the implementation
// @clk_stop_modes: Bitmap, bit N set when clock-stop-modeN supported
// @max_clk_freq: Maximum Bus clock frequency, in Hz
// @num_clk_gears: Number of clock gears supported
// @num_clk_freq: Number of clock frequencies supported, in Hz
// @default_frame_rate: Controller default Frame rate, in Hz
// @default_row: Number of rows
// @default_col: Number of columns
// @dynamic_frame: Dynamic frame shape supported
// @err_threshold: Number of times that software may retry sending a single
// command
// @mclk_freq: clock reference passed to SoundWire Master, in Hz.
// @hw_disabled: if true, the Master is not functional, typically due to pin-mux
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_master_prop {
    pub clk_gears: *mut u32,
    pub clk_freq: *mut u32,
    pub quirks: u64,
    pub revision: u32,
    pub clk_stop_modes: u32,
    pub max_clk_freq: u32,
    pub num_clk_gears: u32,
    pub num_clk_freq: u32,
    pub default_frame_rate: u32,
    pub default_row: u32,
    pub default_col: u32,
    pub err_threshold: u32,
    pub mclk_freq: u32,
    pub dynamic_frame: bool,
    pub hw_disabled: bool,
}

// Definitions for Master quirks
//
// In a number of platforms bus clashes are reported after a hardware
// reset but without any explanations or evidence of a real problem.
// The following quirk will discard all initial bus clash interrupts
// but will leave the detection on should real bus clashes happen
//

//
// Some Slave devices have known issues with incorrect parity errors
// reported after a hardware reset. However during integration unexplained
// parity errors can be reported by Slave devices, possibly due to electrical
// issues at the Master level.
// The following quirk will discard all initial parity errors but will leave
// the detection on should real parity errors happen.
//

extern "C" {
    pub fn sdw_master_read_prop(bus: *mut sdw_bus) -> c_int;
}
extern "C" {
    pub fn sdw_slave_read_prop(slave: *mut sdw_slave) -> c_int;
}
extern "C" {
    pub fn sdw_slave_read_lane_mapping(slave: *mut sdw_slave) -> c_int;
}
//
// SDW Slave Structures and APIs
//
pub const SDW_IGNORED_UNIQUE_ID: c_uint = 0xFF;
//
// struct sdw_slave_id - Slave ID
// @mfg_id: MIPI Manufacturer ID
// @part_id: Device Part ID
// @class_id: MIPI Class ID (defined starting with SoundWire 1.2 spec)
// @unique_id: Device unique ID
// @sdw_version: SDW version implemented
//
// The order of the IDs here does not follow the DisCo spec definitions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_slave_id {
    pub mfg_id: __u16,
    pub part_id: __u16,
    pub class_id: __u8,
    pub unique_id: __u8,
    pub sdw_version:4: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_peripherals {
    pub num_peripherals: c_int,
    pub array: [*mut sdw_slave; ],
}

//
// Helper macros to extract the MIPI-defined IDs
//
// Spec definition
// Register		Bit	Contents
// DevId_0 [7:4]	47:44	sdw_version
// DevId_0 [3:0]	43:40	unique_id
// DevId_1		39:32	mfg_id [15:8]
// DevId_2		31:24	mfg_id [7:0]
// DevId_3		23:16	part_id [15:8]
// DevId_4		15:08	part_id [7:0]
// DevId_5		07:00	class_id
//
// The MIPI DisCo for SoundWire defines in addition the link_id as bits 51:48
//

//
// struct sdw_slave_intr_status - Slave interrupt status
// @sdca_cascade: set if the Slave device reports an SDCA interrupt
// @control_port: control port status
// @port: data port status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_slave_intr_status {
    pub sdca_cascade: bool,
    pub control_port: u8,
    pub port: [u8; 15],
}

//
// enum sdw_reg_bank - SoundWire register banks
// @SDW_BANK0: Soundwire register bank 0
// @SDW_BANK1: Soundwire register bank 1
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_reg_bank {
    SDW_BANK0,
    SDW_BANK1,
}

//
// struct sdw_prepare_ch: Prepare/De-prepare Data Port channel
//
// @num: Port number
// @ch_mask: Active channel mask
// @prepare: Prepare (true) /de-prepare (false) channel
// @bank: Register bank, which bank Slave/Master driver should program for
// implementation defined registers. This is always updated to next_bank
// value read from bus params.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_prepare_ch {
    pub num: c_uint,
    pub ch_mask: c_uint,
    pub prepare: bool,
    pub bank: c_uint,
}

//
// enum sdw_port_prep_ops: Prepare operations for Data Port
//
// @SDW_OPS_PORT_PRE_PREP: Pre prepare operation for the Port
// @SDW_OPS_PORT_PRE_DEPREP: Pre deprepare operation for the Port
// @SDW_OPS_PORT_POST_PREP: Post prepare operation for the Port
// @SDW_OPS_PORT_POST_DEPREP: Post deprepare operation for the Port
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_port_prep_ops {
    SDW_OPS_PORT_PRE_PREP = 0,
    SDW_OPS_PORT_PRE_DEPREP,
    SDW_OPS_PORT_POST_PREP,
    SDW_OPS_PORT_POST_DEPREP,
}

//
// struct sdw_bus_params: Structure holding bus configuration
//
// @curr_bank: Current bank in use (BANK0/BANK1)
// @next_bank: Next bank to use (BANK0/BANK1). next_bank will always be
// set to !curr_bank
// @max_dr_freq: Maximum double rate clock frequency supported, in Hz
// @curr_dr_freq: Current double rate clock frequency, in Hz
// @bandwidth: Current bandwidth
// @col: Active columns
// @row: Active rows
// @s_data_mode: NORMAL, STATIC or PRBS mode for all Slave ports
// @m_data_mode: NORMAL, STATIC or PRBS mode for all Master ports. The value
// should be the same to detect transmission issues, but can be different to
// test the interrupt reports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_bus_params {
    pub curr_bank: sdw_reg_bank,
    pub next_bank: sdw_reg_bank,
    pub max_dr_freq: c_uint,
    pub curr_dr_freq: c_uint,
    pub bandwidth: c_uint,
    pub col: c_uint,
    pub row: c_uint,
    pub s_data_mode: c_int,
    pub m_data_mode: c_int,
}

//
// struct sdw_slave_ops: Slave driver callback ops
//
// @read_prop: Read Slave properties
// @interrupt_callback: Device interrupt notification (invoked in thread
// context)
// @update_status: Update Slave status
// @bus_config: Update the bus config for Slave
// @port_prep: Prepare the port with parameters
// @clk_stop: handle imp-def sequences before and after prepare and de-prepare
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_slave_ops {
    pub sdw): *mut *mut int (read_prop)(struct sdw_slave,
    pub status): *mut sdw_slave_intr_status,
    pub status): sdw_slave_status,
    pub params): *mut sdw_bus_params,
    pub pre_ops): sdw_port_prep_ops,
    pub type): sdw_clk_stop_type,
}

//
// struct sdw_slave - SoundWire Slave
// @id: MIPI device ID
// @dev: Linux device
// @index: internal ID for this slave
// @irq: IRQ number
// @status: Status reported by the Slave
// @bus: Bus handle
// @prop: Slave properties
// @debugfs: Slave debugfs
// @node: node for bus list
// @port_ready: Port ready completion flag for each Slave port
// @m_port_map: static Master port map for each Slave port
// @dev_num: Current Device Number, values can be 0 or dev_num_sticky
// @dev_num_sticky: one-time static Device Number assigned by Bus
// @probed: boolean tracking driver state
// @enumeration_complete: completion utility to control potential races
// on startup between device enumeration and read/write access to the
// Slave device
// @initialization_complete: completion utility to control potential races
// on startup between device enumeration and settings being restored
// @unattach_request: mask field to keep track why the Slave re-attached and
// was re-initialized. This is useful to deal with potential race conditions
// between the Master suspending and the codec resuming, and make sure that
// when the Master triggered a reset the Slave is properly enumerated and
// initialized
// @first_interrupt_done: status flag tracking if the interrupt handling
// for a Slave happens for the first time after enumeration
// @is_mockup_device: status flag used to squelch errors in the command/control
// protocol for SoundWire mockup devices
// @sdw_dev_lock: mutex used to protect callbacks/remove races
// @sdca_data: structure containing all device data for SDCA helpers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_slave {
    pub id: sdw_slave_id,
    pub dev: device,
    pub index: c_int,
    pub irq: c_int,
    pub status: sdw_slave_status,
    pub bus: *mut sdw_bus,
    pub prop: sdw_slave_prop,

    pub debugfs: *mut dentry,

    pub node: list_head,
    pub port_ready: [completion; SDW_MAX_PORTS],
    pub m_port_map: [c_uint; SDW_MAX_PORTS],
    pub dev_num: u16,
    pub dev_num_sticky: u16,
    pub probed: bool,
    pub enumeration_complete: completion,
    pub initialization_complete: completion,
    pub unattach_request: u32,
    pub first_interrupt_done: bool,
    pub is_mockup_device: bool,
    pub /: *mut *mut mutex sdw_dev_lock; / protect callbacks/remove races,
    pub sdca_data: sdca_device_data,
}

//
// struct sdw_master_device - SoundWire 'Master Device' representation
// @dev: Linux device for this Master
// @bus: Bus handle shortcut
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_master_device {
    pub dev: device,
    pub bus: *mut sdw_bus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_driver {
    pub id): *const *const *const int (probe)(struct sdw_slave sdw, struct sdw_device_id,
    pub sdw): *mut *mut void (remove)(struct sdw_slave,
    pub sdw): *mut *mut void (shutdown)(struct sdw_slave,
    pub id_table: *const sdw_device_id,
    pub ops: *const sdw_slave_ops,
    pub driver: device_driver,
}

//
// SDW master structures and APIs
//
// struct sdw_port_params: Data Port parameters
//
// @num: Port number
// @bps: Word length of the Port
// @flow_mode: Port Data flow mode
// @data_mode: Test modes or normal mode
//
// This is used to program the Data Port based on Data Port stream
// parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_port_params {
    pub num: c_uint,
    pub bps: c_uint,
    pub flow_mode: c_uint,
    pub data_mode: c_uint,
}

//
// struct sdw_transport_params: Data Port Transport Parameters
//
// @blk_grp_ctrl_valid: Port implements block group control
// @port_num: Port number
// @blk_grp_ctrl: Block group control value
// @sample_interval: Sample interval
// @offset1: Blockoffset of the payload data
// @offset2: Blockoffset of the payload data
// @hstart: Horizontal start of the payload data
// @hstop: Horizontal stop of the payload data
// @blk_pkg_mode: Block per channel or block per port
// @lane_ctrl: Data lane Port uses for Data transfer. Currently only single
// data lane is supported in bus
//
// This is used to program the Data Port based on Data Port transport
// parameters. All these parameters are banked and can be modified
// during a bank switch without any artifacts in audio stream.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_transport_params {
    pub blk_grp_ctrl_valid: bool,
    pub port_num: c_uint,
    pub blk_grp_ctrl: c_uint,
    pub sample_interval: c_uint,
    pub offset1: c_uint,
    pub offset2: c_uint,
    pub hstart: c_uint,
    pub hstop: c_uint,
    pub blk_pkg_mode: c_uint,
    pub lane_ctrl: c_uint,
}

//
// struct sdw_enable_ch: Enable/disable Data Port channel
//
// @port_num: Port number
// @ch_mask: Active channel mask
// @enable: Enable (true) /disable (false) channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_enable_ch {
    pub port_num: c_uint,
    pub ch_mask: c_uint,
    pub enable: bool,
}

//
// struct sdw_master_port_ops: Callback functions from bus to Master
// driver to set Master Data ports.
//
// @dpn_set_port_params: Set the Port parameters for the Master Port.
// Mandatory callback
// @dpn_set_port_transport_params: Set transport parameters for the Master
// Port. Mandatory callback
// @dpn_port_prep: Port prepare operations for the Master Data Port.
// @dpn_port_enable_ch: Enable the channels of Master Port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_master_port_ops {
    pub bank): c_uint,
    pub bank): sdw_reg_bank,
    pub prepare_ch): *mut *mut *mut int (dpn_port_prep)(struct sdw_bus bus, struct sdw_prepare_ch,
    pub bank): *mut *mut sdw_enable_ch enable_ch, unsigned int,
}

//
// struct sdw_defer - SDW deferred message
// @complete: message completion
// @msg: SDW message
// @length: message length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_defer {
    pub msg: *mut sdw_msg,
    pub length: c_int,
    pub complete: completion,
}

//
// Add a practical limit to BPT transfer sizes. BPT is typically used
// to transfer firmware, and larger firmware transfers will increase
// the cold latency beyond typical OS or user requirements.
//

//
// According to mipi SoundWire DisCo Specification_v2-1,
// this maximum value shall not exceed 470.
// Note that the largest number of bytes accessible by a single BRA operation is limited to 470
// bytes when using lane 0, but goes up to 502 bytes when using one of the optional extra lanes.
//
pub const SDW_BRA_MAX_BYTES_PER_FRAME: c_int = 470;
//
// struct sdw_master_ops - Master driver ops
// @read_prop: Read Master properties
// @override_adr: Override value read from firmware (quirk for buggy firmware)
// @xfer_msg: Transfer message callback
// @xfer_msg_defer: Defer version of transfer message callback. The message is handled with the
// bus struct @sdw_defer
// @set_bus_conf: Set the bus configuration
// @pre_bank_switch: Callback for pre bank switch
// @post_bank_switch: Callback for post bank switch
// @read_ping_status: Read status from PING frames, reported with two bits per Device.
// Bits 31:24 are reserved.
// @get_device_num: Callback for vendor-specific device_number allocation
// @put_device_num: Callback for vendor-specific device_number release
// @new_peripheral_assigned: Callback to handle enumeration of new peripheral.
// @bpt_send_async: reserve resources for BPT stream and send message
// using BTP protocol
// @bpt_wait: wait for message completion using BTP protocol
// and release resources
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_master_ops {
    pub bus): *mut *mut int (read_prop)(struct sdw_bus,
    pub addr): *mut *mut *mut u64 (override_adr)(struct sdw_bus bus, u64,
    pub msg): *mut *mut *mut sdw_command_response (xfer_msg)(struct sdw_bus bus, struct sdw_msg,
    pub bus): *mut *mut sdw_command_response (xfer_msg_defer)(struct sdw_bus,
    pub params): *mut sdw_bus_params,
    pub bus): *mut *mut int (pre_bank_switch)(struct sdw_bus,
    pub bus): *mut *mut int (post_bank_switch)(struct sdw_bus,
    pub bus): *mut *mut u32 (read_ping_status)(struct sdw_bus,
    pub slave): *mut *mut *mut int (get_device_num)(struct sdw_bus bus, struct sdw_slave,
    pub slave): *mut *mut *mut void (put_device_num)(struct sdw_bus bus, struct sdw_slave,
    pub dev_num): c_int,
    pub msg): *mut sdw_bpt_msg,
    pub msg): *mut *mut *mut *mut int (bpt_wait)(struct sdw_bus bus, struct sdw_slave slave, struct sdw_bpt_msg,
}

extern "C" {
    pub fn sdw_bus_master_delete(bus: *mut sdw_bus);
}
extern "C" {
    pub fn sdw_show_ping_status(bus: *mut sdw_bus, sync_delay: bool);
}
//
// struct sdw_port_config: Master or Slave Port configuration
//
// @num: Port number
// @ch_mask: channels mask for port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_port_config {
    pub num: c_uint,
    pub ch_mask: c_uint,
}

//
// struct sdw_stream_config: Master or Slave stream configuration
//
// @frame_rate: Audio frame rate of the stream, in Hz
// @ch_count: Channel count of the stream
// @bps: Number of bits per audio sample
// @direction: Data direction
// @type: Stream type PCM, PDM or BPT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_stream_config {
    pub frame_rate: c_uint,
    pub ch_count: c_uint,
    pub bps: c_uint,
    pub direction: sdw_data_direction,
    pub type: sdw_stream_type,
}

//
// enum sdw_stream_state: Stream states
//
// @SDW_STREAM_ALLOCATED: New stream allocated.
// @SDW_STREAM_CONFIGURED: Stream configured
// @SDW_STREAM_PREPARED: Stream prepared
// @SDW_STREAM_ENABLED: Stream enabled
// @SDW_STREAM_DISABLED: Stream disabled
// @SDW_STREAM_DEPREPARED: Stream de-prepared
// @SDW_STREAM_RELEASED: Stream released
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdw_stream_state {
    SDW_STREAM_ALLOCATED = 0,
    SDW_STREAM_CONFIGURED = 1,
    SDW_STREAM_PREPARED = 2,
    SDW_STREAM_ENABLED = 3,
    SDW_STREAM_DISABLED = 4,
    SDW_STREAM_DEPREPARED = 5,
    SDW_STREAM_RELEASED = 6,
}

//
// struct sdw_stream_params: Stream parameters
//
// @rate: Sampling frequency, in Hz
// @ch_count: Number of channels
// @bps: bits per channel sample
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_stream_params {
    pub rate: c_uint,
    pub ch_count: c_uint,
    pub bps: c_uint,
}

//
// struct sdw_stream_runtime: Runtime stream parameters
//
// @name: SoundWire stream name
// @params: Stream parameters
// @state: Current state of the stream
// @type: Stream type PCM, PDM or BPT
// @m_rt_count: Count of Master runtime(s) in this stream
// @master_list: List of Master runtime(s) in this stream.
// master_list can contain only one m_rt per Master instance
// for a stream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_stream_runtime {
    pub name: *const c_char,
    pub params: sdw_stream_params,
    pub state: sdw_stream_state,
    pub type: sdw_stream_type,
    pub m_rt_count: c_int,
    pub master_list: list_head,
}

//
// struct sdw_bus - SoundWire bus
// @dev: Shortcut to &bus->md->dev to avoid changing the entire code.
// @md: Master device
// @bus_lock_key: bus lock key associated to @bus_lock
// @bus_lock: bus lock
// @slave_ida: IDA for allocating internal slave IDs
// @slaves: list of Slaves on this bus
// @msg_lock_key: message lock key associated to @msg_lock
// @msg_lock: message lock
// @m_rt_list: List of Master instance of all stream(s) running on Bus. This
// is used to compute and program bus bandwidth, clock, frame shape,
// transport and port parameters
// @defer_msg: Defer message
// @params: Current bus parameters
// @stream_refcount: number of streams currently using this bus
// @bpt_stream_refcount: number of BTP streams currently using this bus (should
// be zero or one, multiple streams per link is not supported).
// @bpt_stream: pointer stored to handle BTP streams.
// @ops: Master callback ops
// @port_ops: Master port callback ops
// @prop: Master properties
// @vendor_specific_prop: pointer to non-standard properties
// @hw_sync_min_links: Number of links used by a stream above which
// hardware-based synchronization is required. This value is only
// meaningful if multi_link is set. If set to 1, hardware-based
// synchronization will be used even if a stream only uses a single
// SoundWire segment.
// @controller_id: system-unique controller ID. If set to -1, the bus @id will be used.
// @link_id: Link id number, can be 0 to N, unique for each Controller
// @id: bus system-wide unique id
// @compute_params: points to Bus resource management implementation
// @assigned: Bitmap for Slave device numbers.
// Bit set implies used number, bit clear implies unused number.
// @clk_stop_timeout: Clock stop timeout computed
// @bank_switch_timeout: Bank switch timeout computed
// @domain: IRQ domain
// @irq_chip: IRQ chip
// @debugfs: Bus debugfs (optional)
// @multi_link: Store bus property that indicates if multi links
// are supported. This flag is populated by drivers after reading
// appropriate firmware (ACPI/DT).
// @lane_used_bandwidth: how much bandwidth in bits per second is used by each lane
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_bus {
    pub dev: *mut device,
    pub md: *mut sdw_master_device,
    pub bus_lock_key: lock_class_key,
    pub bus_lock: mutex,
    pub slave_ida: ida,
    pub slaves: list_head,
    pub msg_lock_key: lock_class_key,
    pub msg_lock: mutex,
    pub m_rt_list: list_head,
    pub defer_msg: sdw_defer,
    pub params: sdw_bus_params,
    pub stream_refcount: c_int,
    pub bpt_stream_refcount: c_int,
    pub bpt_stream: *mut sdw_stream_runtime,
    pub ops: *const sdw_master_ops,
    pub port_ops: *const sdw_master_port_ops,
    pub prop: sdw_master_prop,
    pub vendor_specific_prop: *mut c_void,
    pub hw_sync_min_links: c_int,
    pub controller_id: c_int,
    pub link_id: c_uint,
    pub id: c_int,
    pub stream): *mut *mut *mut int (compute_params)(struct sdw_bus bus, struct sdw_stream_runtime,
    pub SDW_MAX_DEVICES): DECLARE_BITMAP(assigned,,
    pub clk_stop_timeout: c_uint,
    pub bank_switch_timeout: u32,
    pub irq_chip: irq_chip,
    pub domain: *mut irq_domain,

    pub debugfs: *mut dentry,

    pub multi_link: bool,
    pub lane_used_bandwidth: [c_uint; SDW_MAX_LANES],
}

extern "C" {
    pub fn sdw_release_stream(stream: *mut sdw_stream_runtime);
}
extern "C" {
    pub fn sdw_compute_params(bus: *mut sdw_bus, stream: *mut sdw_stream_runtime) -> c_int;
}
extern "C" {
    pub fn sdw_startup_stream(sdw_substream: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sdw_prepare_stream(stream: *mut sdw_stream_runtime) -> c_int;
}
extern "C" {
    pub fn sdw_enable_stream(stream: *mut sdw_stream_runtime) -> c_int;
}
extern "C" {
    pub fn sdw_disable_stream(stream: *mut sdw_stream_runtime) -> c_int;
}
extern "C" {
    pub fn sdw_deprepare_stream(stream: *mut sdw_stream_runtime) -> c_int;
}
extern "C" {
    pub fn sdw_shutdown_stream(sdw_substream: *mut c_void);
}
extern "C" {
    pub fn sdw_bus_prep_clk_stop(bus: *mut sdw_bus) -> c_int;
}
extern "C" {
    pub fn sdw_bus_clk_stop(bus: *mut sdw_bus) -> c_int;
}
extern "C" {
    pub fn sdw_bus_exit_clk_stop(bus: *mut sdw_bus) -> c_int;
}
extern "C" {
    pub fn sdw_compare_devid(slave: *mut sdw_slave, id: sdw_slave_id) -> c_int;
}
extern "C" {
    pub fn sdw_extract_slave_id(bus: *mut sdw_bus, addr: u64, id: *mut sdw_slave_id);
}
extern "C" {
    pub fn is_clock_scaling_supported_by_slave(slave: *mut sdw_slave) -> bool;
}
extern "C" {
    pub fn sdw_bpt_send_async(bus: *mut sdw_bus, slave: *mut sdw_slave, msg: *mut sdw_bpt_msg) -> c_int;
}
extern "C" {
    pub fn sdw_bpt_wait(bus: *mut sdw_bus, slave: *mut sdw_slave, msg: *mut sdw_bpt_msg) -> c_int;
}
extern "C" {
    pub fn sdw_bpt_send_sync(bus: *mut sdw_bus, slave: *mut sdw_slave, msg: *mut sdw_bpt_msg) -> c_int;
}

extern "C" {
    pub fn sdw_slave_get_current_bank(sdev: *mut sdw_slave) -> c_int;
}
extern "C" {
    pub fn sdw_slave_get_scale_index(slave: *mut sdw_slave, base: *mut u8) -> c_int;
}
// messaging and data APIs
extern "C" {
    pub fn sdw_read(slave: *mut sdw_slave, addr: u32) -> c_int;
}
extern "C" {
    pub fn sdw_write(slave: *mut sdw_slave, addr: u32, value: u8) -> c_int;
}
extern "C" {
    pub fn sdw_write_no_pm(slave: *mut sdw_slave, addr: u32, value: u8) -> c_int;
}
extern "C" {
    pub fn sdw_read_no_pm(slave: *mut sdw_slave, addr: u32) -> c_int;
}
extern "C" {
    pub fn sdw_nread(slave: *mut sdw_slave, addr: u32, count: usize, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn sdw_nread_no_pm(slave: *mut sdw_slave, addr: u32, count: usize, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn sdw_nwrite(slave: *mut sdw_slave, addr: u32, count: usize, val: *const u8) -> c_int;
}
extern "C" {
    pub fn sdw_nwrite_no_pm(slave: *mut sdw_slave, addr: u32, count: usize, val: *const u8) -> c_int;
}
extern "C" {
    pub fn sdw_update(slave: *mut sdw_slave, addr: u32, mask: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn sdw_update_no_pm(slave: *mut sdw_slave, addr: u32, mask: u8, val: u8) -> c_int;
}

// messaging and data APIs

//
// sdw_slave_wait_for_init - Wait for device initialisation
// @slave: Pointer to the SoundWire peripheral.
// @timeout_ms: Timeout in milliseconds.
//
// Wait for a peripheral device to enumerate and be initialised by the
// SoundWire core.
//
// Return: Zero on success, and a negative error code on failure.
//
