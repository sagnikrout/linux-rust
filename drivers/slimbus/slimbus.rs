//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/slimbus/slimbus.h
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
// Copyright (c) 2011-2017, The Linux Foundation
//

// Standard values per SLIMbus spec needed by controllers and devices
pub const SLIM_CL_PER_SUPERFRAME: c_int = 6144;

// SLIMbus message types. Related to interpretation of message code.
pub const SLIM_MSG_MT_CORE: c_uint = 0x0;
pub const SLIM_MSG_MT_DEST_REFERRED_USER: c_uint = 0x2;
pub const SLIM_MSG_MT_SRC_REFERRED_USER: c_uint = 0x6;
//
// SLIM Broadcast header format
// BYTE 0: MT[7:5] RL[4:0]
// BYTE 1: RSVD[7] MC[6:0]
// BYTE 2: RSVD[7:6] DT[5:4] PI[3:0]
//

pub const SLIM_MSG_MT_SHIFT: c_int = 5;

pub const SLIM_MSG_RL_SHIFT: c_int = 0;

pub const SLIM_MSG_MC_SHIFT: c_int = 0;

pub const SLIM_MSG_DT_SHIFT: c_int = 4;

// Device management messages used by this framework
pub const SLIM_MSG_MC_REPORT_PRESENT: c_uint = 0x1;
pub const SLIM_MSG_MC_ASSIGN_LOGICAL_ADDRESS: c_uint = 0x2;
pub const SLIM_MSG_MC_REPORT_ABSENT: c_uint = 0xF;
// Data channel management messages
pub const SLIM_MSG_MC_CONNECT_SOURCE: c_uint = 0x10;
pub const SLIM_MSG_MC_CONNECT_SINK: c_uint = 0x11;
pub const SLIM_MSG_MC_DISCONNECT_PORT: c_uint = 0x14;
pub const SLIM_MSG_MC_CHANGE_CONTENT: c_uint = 0x18;
// Clock pause Reconfiguration messages
pub const SLIM_MSG_MC_BEGIN_RECONFIGURATION: c_uint = 0x40;
pub const SLIM_MSG_MC_NEXT_PAUSE_CLOCK: c_uint = 0x4A;
pub const SLIM_MSG_MC_NEXT_DEFINE_CHANNEL: c_uint = 0x50;
pub const SLIM_MSG_MC_NEXT_DEFINE_CONTENT: c_uint = 0x51;
pub const SLIM_MSG_MC_NEXT_ACTIVATE_CHANNEL: c_uint = 0x54;
pub const SLIM_MSG_MC_NEXT_DEACTIVATE_CHANNEL: c_uint = 0x55;
pub const SLIM_MSG_MC_NEXT_REMOVE_CHANNEL: c_uint = 0x58;
pub const SLIM_MSG_MC_RECONFIGURE_NOW: c_uint = 0x5F;
// Clock pause values per SLIMbus spec
pub const SLIM_CLK_FAST: c_int = 0;
pub const SLIM_CLK_CONST_PHASE: c_int = 1;
pub const SLIM_CLK_UNSPECIFIED: c_int = 2;
// Destination type Values
pub const SLIM_MSG_DEST_LOGICALADDR: c_int = 0;
pub const SLIM_MSG_DEST_ENUMADDR: c_int = 1;
pub const SLIM_MSG_DEST_BROADCAST: c_int = 3;
// Standard values per SLIMbus spec needed by controllers and devices
pub const SLIM_MAX_CLK_GEAR: c_int = 10;
pub const SLIM_MIN_CLK_GEAR: c_int = 1;
pub const SLIM_SLOT_LEN_BITS: c_int = 4;
// Indicate that the frequency of the flow and the bus frequency are locked

// Standard values per SLIMbus spec needed by controllers and devices
pub const SLIM_CL_PER_SUPERFRAME: c_int = 6144;

// Manager's logical address is set to 0xFF per spec
pub const SLIM_LA_MANAGER: c_uint = 0xFF;
pub const SLIM_MAX_TIDS: c_int = 256;
//
// struct slim_framer - Represents SLIMbus framer.
// Every controller may have multiple framers. There is 1 active framer device
// responsible for clocking the bus.
// Manager is responsible for framer hand-over.
// @dev: Driver model representation of the device.
// @e_addr: Enumeration address of the framer.
// @rootfreq: Root Frequency at which the framer can run. This is maximum
// frequency ('clock gear 10') at which the bus can operate.
// @superfreq: Superframes per root frequency. Every frame is 6144 bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_framer {
    pub dev: device,
    pub e_addr: slim_eaddr,
    pub rootfreq: c_int,
    pub superfreq: c_int,
}

//
// struct slim_msg_txn - Message to be sent by the controller.
// This structure has packet header,
// payload and buffer to be filled (if any)
// @rl: Header field. remaining length.
// @mt: Header field. Message type.
// @mc: Header field. LSB is message code for type mt.
// @dt: Header field. Destination type.
// @ec: Element code. Used for elemental access APIs.
// @tid: Transaction ID. Used for messages expecting response.
// (relevant for message-codes involving read operation)
// @la: Logical address of the device this message is going to.
// (Not used when destination type is broadcast.)
// @msg: Elemental access message to be read/written
// @comp: completion if read/write is synchronous, used internally
// for tid based transactions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_msg_txn {
    pub rl: u8,
    pub mt: u8,
    pub mc: u8,
    pub dt: u8,
    pub ec: u16,
    pub tid: u8,
    pub la: u8,
    pub msg: *mut slim_val_inf,
    pub comp: *mut completion,
}

// Frequently used message transaction structures

//
// enum slim_clk_state: SLIMbus controller's clock state used internally for
// maintaining current clock state.
// @SLIM_CLK_ACTIVE: SLIMbus clock is active
// @SLIM_CLK_ENTERING_PAUSE: SLIMbus clock pause sequence is being sent on the
// bus. If this succeeds, state changes to SLIM_CLK_PAUSED. If the
// transition fails, state changes back to SLIM_CLK_ACTIVE
// @SLIM_CLK_PAUSED: SLIMbus controller clock has paused.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slim_clk_state {
    SLIM_CLK_ACTIVE,
    SLIM_CLK_ENTERING_PAUSE,
    SLIM_CLK_PAUSED,
}

//
// struct slim_sched: Framework uses this structure internally for scheduling.
// @clk_state: Controller's clock state from enum slim_clk_state
// @pause_comp: Signals completion of clock pause sequence. This is useful when
// client tries to call SLIMbus transaction when controller is entering
// clock pause.
// @m_reconf: This mutex is held until current reconfiguration (data channel
// scheduling, message bandwidth reservation) is done. Message APIs can
// use the bus concurrently when this mutex is held since elemental access
// messages can be sent on the bus when reconfiguration is in progress.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_sched {
    pub clk_state: slim_clk_state,
    pub pause_comp: completion,
    pub m_reconf: mutex,
}

//
// enum slim_port_direction: SLIMbus port direction
//
// @SLIM_PORT_SINK: SLIMbus port is a sink
// @SLIM_PORT_SOURCE: SLIMbus port is a source
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slim_port_direction {
    SLIM_PORT_SINK = 0,
    SLIM_PORT_SOURCE,
}

//
// enum slim_port_state: SLIMbus Port/Endpoint state machine
// according to SLIMbus Spec 2.0
// @SLIM_PORT_DISCONNECTED: SLIMbus port is disconnected
// entered from Unconfigure/configured state after
// DISCONNECT_PORT or REMOVE_CHANNEL core command
// @SLIM_PORT_UNCONFIGURED: SLIMbus port is in unconfigured state.
// entered from disconnect state after CONNECT_SOURCE/SINK core command
// @SLIM_PORT_CONFIGURED: SLIMbus port is in configured state.
// entered from unconfigured state after DEFINE_CHANNEL, DEFINE_CONTENT
// and ACTIVATE_CHANNEL core commands. Ready for data transmission.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slim_port_state {
    SLIM_PORT_DISCONNECTED = 0,
    SLIM_PORT_UNCONFIGURED,
    SLIM_PORT_CONFIGURED,
}

//
// enum slim_channel_state: SLIMbus channel state machine used by core.
// @SLIM_CH_STATE_DISCONNECTED: SLIMbus channel is disconnected
// @SLIM_CH_STATE_ALLOCATED: SLIMbus channel is allocated
// @SLIM_CH_STATE_ASSOCIATED: SLIMbus channel is associated with port
// @SLIM_CH_STATE_DEFINED: SLIMbus channel parameters are defined
// @SLIM_CH_STATE_CONTENT_DEFINED: SLIMbus channel content is defined
// @SLIM_CH_STATE_ACTIVE: SLIMbus channel is active and ready for data
// @SLIM_CH_STATE_REMOVED: SLIMbus channel is inactive and removed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slim_channel_state {
    SLIM_CH_STATE_DISCONNECTED = 0,
    SLIM_CH_STATE_ALLOCATED,
    SLIM_CH_STATE_ASSOCIATED,
    SLIM_CH_STATE_DEFINED,
    SLIM_CH_STATE_CONTENT_DEFINED,
    SLIM_CH_STATE_ACTIVE,
    SLIM_CH_STATE_REMOVED,
}

//
// enum slim_ch_data_fmt: SLIMbus channel data Type identifiers according to
// Table 60 of SLIMbus Spec 1.01.01
// @SLIM_CH_DATA_FMT_NOT_DEFINED: Undefined
// @SLIM_CH_DATA_FMT_LPCM_AUDIO: LPCM audio
// @SLIM_CH_DATA_FMT_IEC61937_COMP_AUDIO: IEC61937 Compressed audio
// @SLIM_CH_DATA_FMT_PACKED_PDM_AUDIO: Packed PDM audio
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slim_ch_data_fmt {
    SLIM_CH_DATA_FMT_NOT_DEFINED = 0,
    SLIM_CH_DATA_FMT_LPCM_AUDIO = 1,
    SLIM_CH_DATA_FMT_IEC61937_COMP_AUDIO = 2,
    SLIM_CH_DATA_FMT_PACKED_PDM_AUDIO = 3,
}

//
// enum slim_ch_aux_bit_fmt: SLIMbus channel Aux Field format IDs according to
// Table 63 of SLIMbus Spec 2.0
// @SLIM_CH_AUX_FMT_NOT_APPLICABLE: Undefined
// @SLIM_CH_AUX_FMT_ZCUV_TUNNEL_IEC60958: ZCUV for tunneling IEC60958
// @SLIM_CH_AUX_FMT_USER_DEFINED: User defined
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slim_ch_aux_bit_fmt {
    SLIM_CH_AUX_FMT_NOT_APPLICABLE = 0,
    SLIM_CH_AUX_FMT_ZCUV_TUNNEL_IEC60958 = 1,
    SLIM_CH_AUX_FMT_USER_DEFINED = 0xF,
}

//
// struct slim_channel  - SLIMbus channel, used for state machine
//
// @id: ID of channel
// @prrate: Presense rate of channel from Table 66 of SLIMbus 2.0 Specs
// @seg_dist: segment distribution code from Table 20 of SLIMbus 2.0 Specs
// @data_fmt: Data format of channel.
// @aux_fmt: Aux format for this channel.
// @state: channel state machine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_channel {
    pub id: c_int,
    pub prrate: c_int,
    pub seg_dist: c_int,
    pub data_fmt: slim_ch_data_fmt,
    pub aux_fmt: slim_ch_aux_bit_fmt,
    pub state: slim_channel_state,
}

//
// struct slim_port  - SLIMbus port
//
// @id: Port id
// @direction: Port direction, Source or Sink.
// @state: state machine of port.
// @ch: channel associated with this port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_port {
    pub id: c_int,
    pub direction: slim_port_direction,
    pub state: slim_port_state,
    pub ch: slim_channel,
}

//
// enum slim_transport_protocol: SLIMbus Transport protocol list from
// Table 47 of SLIMbus 2.0 specs.
// @SLIM_PROTO_ISO: Isochronous Protocol, no flow control as data rate match
// channel rate flow control embedded in the data.
// @SLIM_PROTO_PUSH: Pushed Protocol, includes flow control, Used to carry
// data whose rate	is equal to, or lower than the channel rate.
// @SLIM_PROTO_PULL: Pulled Protocol, similar usage as pushed protocol
// but pull is a unicast.
// @SLIM_PROTO_LOCKED: Locked Protocol
// @SLIM_PROTO_ASYNC_SMPLX: Asynchronous Protocol-Simplex
// @SLIM_PROTO_ASYNC_HALF_DUP: Asynchronous Protocol-Half-duplex
// @SLIM_PROTO_EXT_SMPLX: Extended Asynchronous Protocol-Simplex
// @SLIM_PROTO_EXT_HALF_DUP: Extended Asynchronous Protocol-Half-duplex
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slim_transport_protocol {
    SLIM_PROTO_ISO = 0,
    SLIM_PROTO_PUSH,
    SLIM_PROTO_PULL,
    SLIM_PROTO_LOCKED,
    SLIM_PROTO_ASYNC_SMPLX,
    SLIM_PROTO_ASYNC_HALF_DUP,
    SLIM_PROTO_EXT_SMPLX,
    SLIM_PROTO_EXT_HALF_DUP,
}

//
// struct slim_stream_runtime  - SLIMbus stream runtime instance
//
// @name: Name of the stream
// @dev: SLIM Device instance associated with this stream
// @direction: direction of stream
// @prot: Transport protocol used in this stream
// @rate: Data rate of samples
// @bps: bits per sample
// @ratem: rate multipler which is super frame rate/data rate
// @num_ports: number of ports
// @ports: pointer to instance of ports
// @node: list head for stream associated with slim device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_stream_runtime {
    pub name: *const c_char,
    pub dev: *mut slim_device,
    pub direction: c_int,
    pub prot: slim_transport_protocol,
    pub rate: c_uint,
    pub bps: c_uint,
    pub ratem: c_uint,
    pub num_ports: c_int,
    pub ports: *mut slim_port,
    pub node: list_head,
}

//
// struct slim_controller  - Controls every instance of SLIMbus
// (similar to 'master' on SPI)
// @dev: Device interface to this driver
// @id: Board-specific number identifier for this controller/bus
// @name: Name for this controller
// @min_cg: Minimum clock gear supported by this controller (default value: 1)
// @max_cg: Maximum clock gear supported by this controller (default value: 10)
// @clkgear: Current clock gear in which this bus is running
// @laddr_ida: logical address id allocator
// @a_framer: Active framer which is clocking the bus managed by this controller
// @lock: Mutex protecting controller data structures
// @devices: Slim device list
// @tid_idr: tid id allocator
// @txn_lock: Lock to protect table of transactions
// @sched: scheduler structure used by the controller
// @xfer_msg: Transfer a message on this controller (this can be a broadcast
// control/status message like data channel setup, or a unicast message
// like value element read/write.
// @set_laddr: Setup logical address at laddr for the slave with elemental
// address e_addr. Drivers implementing controller will be expected to
// send unicast message to this device with its logical address.
// @get_laddr: It is possible that controller needs to set fixed logical
// address table and get_laddr can be used in that case so that controller
// can do this assignment. Use case is when the master is on the remote
// processor side, who is resposible for allocating laddr.
// @wakeup: This function pointer implements controller-specific procedure
// to wake it up from clock-pause. Framework will call this to bring
// the controller out of clock pause.
// @enable_stream: This function pointer implements controller-specific procedure
// to enable a stream.
// @disable_stream: This function pointer implements controller-specific procedure
// to disable stream.
//
// 'Manager device' is responsible for  device management, bandwidth
// allocation, channel setup, and port associations per channel.
// Device management means Logical address assignment/removal based on
// enumeration (report-present, report-absent) of a device.
// Bandwidth allocation is done dynamically by the manager based on active
// channels on the bus, message-bandwidth requests made by SLIMbus devices.
// Based on current bandwidth usage, manager chooses a frequency to run
// the bus at (in steps of 'clock-gear', 1 through 10, each clock gear
// representing twice the frequency than the previous gear).
// Manager is also responsible for entering (and exiting) low-power-mode
// (known as 'clock pause').
// Manager can do handover of framer if there are multiple framers on the
// bus and a certain usecase warrants using certain framer to avoid keeping
// previous framer being powered-on.
//
// Controller here performs duties of the manager device, and 'interface
// device'. Interface device is responsible for monitoring the bus and
// reporting information such as loss-of-synchronization, data
// slot-collision.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slim_controller {
    pub dev: *mut device,
    pub id: c_uint,
    pub name: [c_char; SLIMBUS_NAME_SIZE],
    pub min_cg: c_int,
    pub max_cg: c_int,
    pub clkgear: c_int,
    pub laddr_ida: ida,
    pub a_framer: *mut slim_framer,
    pub lock: mutex,
    pub devices: list_head,
    pub tid_idr: idr,
    pub txn_lock: spinlock_t,
    pub sched: slim_sched,
    pub tx): *mut slim_msg_txn,
    pub laddr): *mut *mut slim_eaddr ea, u8,
    pub laddr): *mut *mut slim_eaddr ea, u8,
    pub rt): *mut *mut int (enable_stream)(struct slim_stream_runtime,
    pub rt): *mut *mut int (disable_stream)(struct slim_stream_runtime,
    pub ctrl): *mut *mut int (wakeup)(struct slim_controller,
}

extern "C" {
    pub fn slim_report_absent(sbdev: *mut slim_device);
}
extern "C" {
    pub fn slim_register_controller(ctrl: *mut slim_controller) -> c_int;
}
extern "C" {
    pub fn slim_unregister_controller(ctrl: *mut slim_controller) -> c_int;
}
extern "C" {
    pub fn slim_msg_response(ctrl: *mut slim_controller, reply: *mut u8, tid: u8, l: u8);
}
extern "C" {
    pub fn slim_do_transfer(ctrl: *mut slim_controller, txn: *mut slim_msg_txn) -> c_int;
}
extern "C" {
    pub fn slim_ctrl_clk_pause(ctrl: *mut slim_controller, wakeup: bool, restart: u8) -> c_int;
}
extern "C" {
    pub fn slim_alloc_txn_tid(ctrl: *mut slim_controller, txn: *mut slim_msg_txn) -> c_int;
}
extern "C" {
    pub fn slim_free_txn_tid(ctrl: *mut slim_controller, txn: *mut slim_msg_txn);
}
