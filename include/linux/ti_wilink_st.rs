//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ti_wilink_st.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Shared Transport Header file
// To be included by the protocol stack drivers for
// Texas Instruments BT,FM and GPS combo chip drivers
// and also serves the sub-modules of the shared transport driver.
//
// Copyright (C) 2009-2010 Texas Instruments
// Author: Pavan Savoy <pavan_savoy@ti.com>
//

//
// enum proto-type - The protocol on WiLink chips which share a
// common physical interface like UART.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum proto_type {
    ST_BT,
    ST_FM,
    ST_GPS,
    ST_MAX_CHANNELS = 16,
}

//
// struct st_proto_s - Per Protocol structure from BT/FM/GPS to ST
// @type: type of the protocol being registered among the
// available proto_type(BT, FM, GPS the protocol which share TTY).
// @recv: the receiver callback pointing to a function in the
// protocol drivers called by the ST driver upon receiving
// relevant data.
// @match_packet: reserved for future use, to make ST more generic
// @reg_complete_cb: callback handler pointing to a function in protocol
// handler called by ST when the pending registrations are complete.
// The registrations are marked pending, in situations when fw
// download is in progress.
// @write: pointer to function in ST provided to protocol drivers from ST,
// to be made use when protocol drivers have data to send to TTY.
// @priv_data: privdate data holder for the protocol drivers, sent
// from the protocol drivers during registration, and sent back on
// reg_complete_cb and recv.
// @chnl_id: channel id the protocol driver is interested in, the channel
// id is nothing but the 1st byte of the packet in UART frame.
// @max_frame_size: size of the largest frame the protocol can receive.
// @hdr_len: length of the header structure of the protocol.
// @offset_len_in_hdr: this provides the offset of the length field in the
// header structure of the protocol header, to assist ST to know
// how much to receive, if the data is split across UART frames.
// @len_size: whether the length field inside the header is 2 bytes
// or 1 byte.
// @reserve: the number of bytes ST needs to reserve in the skb being
// prepared for the protocol driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_proto_s {
    pub type: proto_type,
    pub ): *mut *mut *mut long (recv) (void , struct sk_buff,
    pub data): *const *const unsigned char (match_packet) (unsigned char,
    pub data): *mut *mut *mut void (reg_complete_cb) (void , int,
    pub skb): *mut *mut long (write) (struct sk_buff,
    pub priv_data: *mut c_void,
    pub chnl_id: c_uchar,
    pub max_frame_size: c_ushort,
    pub hdr_len: c_uchar,
    pub offset_len_in_hdr: c_uchar,
    pub len_size: c_uchar,
    pub reserve: c_uchar,
}

extern "C" {
    pub fn st_register(: *mut st_proto_s) -> c_long;
}
extern "C" {
    pub fn st_unregister(: *mut st_proto_s) -> c_long;
}
//
// header information used by st_core.c
//
// states of protocol list
pub const ST_NOTEMPTY: c_int = 1;
pub const ST_EMPTY: c_int = 0;
//
// possible st_states
//
pub const ST_INITIALIZING: c_int = 1;
pub const ST_REG_IN_PROGRESS: c_int = 2;
pub const ST_REG_PENDING: c_int = 3;
pub const ST_WAITING_FOR_RESP: c_int = 4;
//
// struct st_data_s - ST core internal structure
// @st_state: different states of ST like initializing, registration
// in progress, this is mainly used to return relevant err codes
// when protocol drivers are registering. It is also used to track
// the recv function, as in during fw download only HCI events
// can occur , where as during other times other events CH8, CH9
// can occur.
// @tty: tty provided by the TTY core for line disciplines.
// @tx_skb: If for some reason the tty's write returns lesser bytes written
// then to maintain the rest of data to be written on next instance.
// This needs to be protected, hence the lock inside wakeup func.
// @tx_state: if the data is being written onto the TTY and protocol driver
// wants to send more, queue up data and mark that there is
// more data to send.
// @list: the list of protocols registered, only MAX can exist, one protocol
// can register only once.
// @rx_state: states to be maintained inside st's tty receive
// @rx_count: count to be maintained inside st's tty receieve
// @rx_skb: the skb where all data for a protocol gets accumulated,
// since tty might not call receive when a complete event packet
// is received, the states, count and the skb needs to be maintained.
// @rx_chnl: the channel ID for which the data is getting accumalated for.
// @txq: the list of skbs which needs to be sent onto the TTY.
// @tx_waitq: if the chip is not in AWAKE state, the skbs needs to be queued
// up in here, PM(WAKEUP_IND) data needs to be sent and then the skbs
// from waitq can be moved onto the txq.
// Needs locking too.
// @lock: the lock to protect skbs, queues, and ST states.
// @protos_registered: count of the protocols registered, also when 0 the
// chip enable gpio can be toggled, and when it changes to 1 the fw
// needs to be downloaded to initialize chip side ST.
// @ll_state: the various PM states the chip can be, the states are notified
// to us, when the chip sends relevant PM packets(SLEEP_IND, WAKE_IND).
// @kim_data: reference to the parent encapsulating structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_data_s {
    pub st_state: c_ulong,
    pub tx_skb: *mut sk_buff,
pub const ST_TX_SENDING: c_int = 1;
pub const ST_TX_WAKEUP: c_int = 2;
    pub tx_state: c_ulong,
    pub list: [*mut st_proto_s; ST_MAX_CHANNELS],
    pub is_registered: [bool; ST_MAX_CHANNELS],
    pub rx_state: c_ulong,
    pub rx_count: c_ulong,
    pub rx_skb: *mut sk_buff,
    pub rx_chnl: c_uchar,
    pub tx_waitq: sk_buff_head txq,,
    pub lock: spinlock_t,
    pub protos_registered: c_uchar,
    pub ll_state: c_ulong,
    pub kim_data: *mut c_void,
    pub tty: *mut tty_struct,
    pub work_write_wakeup: work_struct,
}

//
// wrapper around tty->ops->write_room to check
// availability during firmware download
//
extern "C" {
    pub fn st_get_uart_wr_room(st_gdata: *mut st_data_s) -> c_int;
}
//
// st_int_write -
// point this to tty->driver->write or tty->ops->write
// depending upon the kernel version
//
extern "C" {
    pub fn st_int_write(st_data_s*: *mut struct, char*: *const unsigned, _arg: c_int) -> c_int;
}
//
// st_write -
// internal write function, passed onto protocol drivers
// via the write function ptr of protocol struct
//
extern "C" {
    pub fn st_write(: *mut sk_buff) -> c_long;
}
// function to be called from ST-LL
extern "C" {
    pub fn st_ll_send_frame(proto_type: enum, : *mut sk_buff);
}
// internal wake up function
extern "C" {
    pub fn st_tx_wakeup(st_data: *mut st_data_s);
}
// init, exit entry funcs called from KIM
extern "C" {
    pub fn st_core_init(: *mut st_data_s) -> c_int;
}
extern "C" {
    pub fn st_core_exit(: *mut st_data_s);
}
// ask for reference from KIM
extern "C" {
    pub fn st_kim_ref(: *mut st_data_s, _arg: c_int);
}
// Macro flag: #define GPS_STUB_TEST

extern "C" {
    pub fn gps_chrdrv_stub_write(char*: *const unsigned, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn gps_chrdrv_stub_init();
}

//
// header information used by st_kim.c
//
// time in msec to wait for
// line discipline to be installed
//
pub const LDISC_TIME: c_int = 1000;
pub const CMD_RESP_TIME: c_int = 800;
pub const CMD_WR_TIME: c_int = 5000;

pub const GPIO_HIGH: c_int = 1;
pub const GPIO_LOW: c_int = 0;
// the Power-On-Reset logic, requires to attempt
// to download firmware onto chip more than once
// since the self-test for chip takes a while
//
pub const POR_RETRY_COUNT: c_int = 5;
//
// struct chip_version - save the chip version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chip_version {
    pub full: c_ushort,
    pub chip: c_ushort,
    pub min_ver: c_ushort,
    pub maj_ver: c_ushort,
}

pub const UART_DEV_NAME_LEN: c_int = 32;
//
// struct kim_data_s - the KIM internal data, embedded as the
// platform's drv data. One for each ST device in the system.
// @uim_pid: KIM needs to communicate with UIM to request to install
// the ldisc by opening UART when protocol drivers register.
// @kim_pdev: the platform device added in one of the board-XX.c file
// in arch/XX/ directory, 1 for each ST device.
// @kim_rcvd: completion handler to notify when data was received,
// mainly used during fw download, which involves multiple send/wait
// for each of the HCI-VS commands.
// @ldisc_installed: completion handler to notify that the UIM accepted
// the request to install ldisc, notify from tty_open which suggests
// the ldisc was properly installed.
// @resp_buffer: data buffer for the .bts fw file name.
// @fw_entry: firmware class struct to request/release the fw.
// @rx_state: the rx state for kim's receive func during fw download.
// @rx_count: the rx count for the kim's receive func during fw download.
// @rx_skb: all of fw data might not come at once, and hence data storage for
// whole of the fw response, only HCI_EVENTs and hence diff from ST's
// response.
// @core_data: ST core's data, which mainly is the tty's disc_data
// @version: chip version available via a sysfs entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kim_data_s {
    pub uim_pid: c_long,
    pub kim_pdev: *mut platform_device,
    pub ldisc_installed: completion kim_rcvd,,
    pub resp_buffer: [c_char; 30],
    pub fw_entry: *const firmware,
    pub nshutdown: unsigned,
    pub rx_state: c_ulong,
    pub rx_count: c_ulong,
    pub rx_skb: *mut sk_buff,
    pub core_data: *mut st_data_s,
    pub version: chip_version,
    pub ldisc_install: c_uchar,
    pub 1]: unsigned char dev_name[UART_DEV_NAME_LEN +,
    pub flow_cntrl: unsigned,
    pub baud_rate: unsigned,
}

//
// functions called when 1 of the protocol drivers gets
// registered, these need to communicate with UIM to request
// ldisc installed, read chip_version, download relevant fw
//
extern "C" {
    pub fn st_kim_start(: *mut c_void) -> c_long;
}
extern "C" {
    pub fn st_kim_stop(: *mut c_void) -> c_long;
}
extern "C" {
    pub fn st_kim_complete(: *mut c_void);
}
extern "C" {
    pub fn kim_st_list_protocols(: *mut st_data_s, : *mut c_void);
}
extern "C" {
    pub fn st_kim_recv(disc_data: *mut c_void, data: *const u8, count: usize);
}
//
// BTS headers
//
pub const ACTION_SEND_COMMAND: c_int = 1;
pub const ACTION_WAIT_EVENT: c_int = 2;
pub const ACTION_SERIAL: c_int = 3;
pub const ACTION_DELAY: c_int = 4;
pub const ACTION_RUN_SCRIPT: c_int = 5;
pub const ACTION_REMARKS: c_int = 6;
//
// struct bts_header - the fw file is NOT binary which can
// be sent onto TTY as is. The .bts is more a script
// file which has different types of actions.
// Each such action needs to be parsed by the KIM and
// relevant procedure to be called.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bts_header {
    pub magic: u32,
    pub version: u32,
    pub future: [u8; 24],
    pub actions: [u8; ],
// C attribute field omitted
//
// struct bts_action - Each .bts action has its own type of
// data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bts_action {
    pub type: u16,
    pub size: u16,
    pub data: [u8; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bts_action_send {
    pub data: [u8; 0],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bts_action_wait {
    pub msec: u32,
    pub size: u32,
    pub data: [u8; ],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bts_action_delay {
    pub msec: u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bts_action_serial {
    pub baud: u32,
    pub flow_control: u32,
// C attribute field omitted
//
// struct hci_command - the HCI-VS for intrepreting
// the change baud rate of host-side UART, which
// needs to be ignored, since UIM would do that
// when it receives request from KIM for ldisc installation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_command {
    pub prefix: u8,
    pub opcode: u16,
    pub plen: u8,
    pub speed: u32,
// C attribute field omitted
//
// header information used by st_ll.c
//
// ST LL receiver states
pub const ST_W4_PACKET_TYPE: c_int = 0;
pub const ST_W4_HEADER: c_int = 1;
pub const ST_W4_DATA: c_int = 2;
// ST LL state machines
pub const ST_LL_ASLEEP: c_int = 0;
pub const ST_LL_ASLEEP_TO_AWAKE: c_int = 1;
pub const ST_LL_AWAKE: c_int = 2;
pub const ST_LL_AWAKE_TO_ASLEEP: c_int = 3;
pub const ST_LL_INVALID: c_int = 4;
// different PM notifications coming from chip
pub const LL_SLEEP_IND: c_uint = 0x30;
pub const LL_SLEEP_ACK: c_uint = 0x31;
pub const LL_WAKE_UP_IND: c_uint = 0x32;
pub const LL_WAKE_UP_ACK: c_uint = 0x33;
// initialize and de-init ST LL
    pub ): *mut long st_ll_init(struct st_data_s,
    pub ): *mut long st_ll_deinit(struct st_data_s,
//
// enable/disable ST LL along with KIM start/stop
// called by ST Core
//
    pub ): *mut void st_ll_enable(struct st_data_s,
    pub ): *mut void st_ll_disable(struct st_data_s,
//
// various funcs used by ST core to set/get the various PM states
// of the chip.
//
    pub ): *mut unsigned long st_ll_getstate(struct st_data_s,
    pub char): *mut *mut unsigned long st_ll_sleep_state(struct st_data_s , unsigned,
    pub ): *mut void st_ll_wakeup(struct st_data_s,
//
// header information used by st_core.c for FM and GPS
// packet parsing, the bluetooth headers are already available
// at net/bluetooth
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fm_event_hdr {
    pub plen: u8,
// C attribute field omitted
pub const FM_MAX_FRAME_SIZE: c_uint = 0xFF	/* TODO: */;

pub const ST_FM_CH8_PKT: c_uint = 0x8;
// gps stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gps_event_hdr {
    pub opcode: u8,
    pub plen: u16,
// C attribute field omitted
//
// struct ti_st_plat_data - platform data shared between ST driver and
// platform specific board file which adds the ST device.
// @nshutdown_gpio: Host's GPIO line to which chip's BT_EN is connected.
// @dev_name: The UART/TTY name to which chip is interfaced. (eg: /dev/ttyS1)
// @flow_cntrl: Should always be 1, since UART's CTS/RTS is used for PM
// purposes.
// @baud_rate: The baud rate supported by the Host UART controller, this will
// be shared across with the chip via a HCI VS command from User-Space Init
// Mgr application.
// @suspend:
// @resume: legacy PM routines hooked to platform specific board file, so as
// to take chip-host interface specific action.
// @chip_enable:
// @chip_disable: Platform/Interface specific mux mode setting, GPIO
// configuring, Host side PM disabling etc.. can be done here.
// @chip_asleep:
// @chip_awake: Chip specific deep sleep states is communicated to Host
// specific board-xx.c to take actions such as cut UART clocks when chip
// asleep or run host faster when chip awake etc..
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_st_plat_data {
    pub nshutdown_gpio: u32,
    pub /: *mut *mut unsigned char dev_name[UART_DEV_NAME_LEN]; / uart name,
    pub /: *mut *mut u32 flow_cntrl; / flow control flag,
    pub baud_rate: u32,
    pub pm_message_t): *mut *mut *mut int (suspend)(struct platform_device ,,
    pub ): *mut *mut int (resume)(struct platform_device,
    pub ): *mut *mut int (chip_enable) (struct kim_data_s,
    pub ): *mut *mut int (chip_disable) (struct kim_data_s,
    pub ): *mut *mut int (chip_asleep) (struct kim_data_s,
    pub ): *mut *mut int (chip_awake) (struct kim_data_s,
}
