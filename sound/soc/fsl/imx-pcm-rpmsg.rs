//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/imx-pcm-rpmsg.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2017-2021  NXP
//
// Communication stack of audio with rpmsg
//
// Packet structure:
// A SRTM message consists of a 10 bytes header followed by 0~N bytes of data
//
// +---------------+-------------------------------+
// |               |            Content            |
// +---------------+-------------------------------+
// |  Byte Offset  | 7   6   5   4   3   2   1   0 |
// +---------------+---+---+---+---+---+---+---+---+
// |       0       |           Category            |
// +---------------+---+---+---+---+---+---+---+---+
// |     1 ~ 2     |           Version             |
// +---------------+---+---+---+---+---+---+---+---+
// |       3       |             Type              |
// +---------------+---+---+---+---+---+---+---+---+
// |       4       |           Command             |
// +---------------+---+---+---+---+---+---+---+---+
// |       5       |           Reserved0           |
// +---------------+---+---+---+---+---+---+---+---+
// |       6       |           Reserved1           |
// +---------------+---+---+---+---+---+---+---+---+
// |       7       |           Reserved2           |
// +---------------+---+---+---+---+---+---+---+---+
// |       8       |           Reserved3           |
// +---------------+---+---+---+---+---+---+---+---+
// |       9       |           Reserved4           |
// +---------------+---+---+---+---+---+---+---+---+
// |       10      |            DATA 0             |
// +---------------+---+---+---+---+---+---+---+---+
// :   :   :   :   :   :   :   :   :   :   :   :   :
// +---------------+---+---+---+---+---+---+---+---+
// |   N + 10 - 1  |            DATA N-1           |
// +---------------+---+---+---+---+---+---+---+---+
//
// +----------+------------+------------------------------------------------+
// |  Field   |    Byte    |                                                |
// +----------+------------+------------------------------------------------+
// | Category |     0      | The destination category.                      |
// +----------+------------+------------------------------------------------+
// | Version  |   1 ~ 2    | The category version of the sender of the      |
// |          |            | packet.                                        |
// |          |            | The first byte represent the major version of  |
// |          |            | the packet.The second byte represent the minor |
// |          |            | version of the packet.                         |
// +----------+------------+------------------------------------------------+
// |  Type    |     3      | The message type of current message packet.    |
// +----------+------------+------------------------------------------------+
// | Command  |     4      | The command byte sent to remote processor/SoC. |
// +----------+------------+------------------------------------------------+
// | Reserved |   5 ~ 9    | Reserved field for future extension.           |
// +----------+------------+------------------------------------------------+
// | Data     |     N      | The data payload of the message packet.        |
// +----------+------------+------------------------------------------------+
//
// Audio control:
// SRTM Audio Control Category Request Command Table:
// +----------+---------+------+---------+-------------------------------+-----------------------+
// | Category | Version | Type | Command | Data                          | Function              |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x00   | Data[0]: Audio Device Index   | Open a TX Instance.   |
// |          |         |      |         | Data[1]:     format           |                       |
// |          |         |      |         | Data[2]:     channels         |                       |
// |          |         |      |         | Data[3-6]:   samplerate       |                       |
// |          |         |      |         | Data[7-10]:  buffer_addr      |                       |
// |          |         |      |         | Data[11-14]: buffer_size      |                       |
// |          |         |      |         | Data[15-18]: period_size      |                       |
// |          |         |      |         | Data[19-22]: buffer_tail      |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x01   | Data[0]: Audio Device Index   | Start a TX Instance.  |
// |          |         |      |         | Same as above command         |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x02   | Data[0]: Audio Device Index   | Pause a TX Instance.  |
// |          |         |      |         | Same as above command         |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x03   | Data[0]: Audio Device Index   | Resume a TX Instance. |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x04   | Data[0]: Audio Device Index   | Stop a TX Instance.   |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x05   | Data[0]: Audio Device Index   | Close a TX Instance.  |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x06   | Data[0]: Audio Device Index   | Set Parameters for    |
// |          |         |      |         | Data[1]:     format           | a TX Instance.        |
// |          |         |      |         | Data[2]:     channels         |                       |
// |          |         |      |         | Data[3-6]:   samplerate       |                       |
// |          |         |      |         | Data[7-22]:  reserved         |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x07   | Data[0]: Audio Device Index   | Set TX Buffer.        |
// |          |         |      |         | Data[1-6]:   reserved         |                       |
// |          |         |      |         | Data[7-10]:  buffer_addr      |                       |
// |          |         |      |         | Data[11-14]: buffer_size      |                       |
// |          |         |      |         | Data[15-18]: period_size      |                       |
// |          |         |      |         | Data[19-22]: buffer_tail      |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x08   | Data[0]: Audio Device Index   | Suspend a TX Instance |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x09   | Data[0]: Audio Device Index   | Resume a TX Instance. |
// |          |         |      |         | Data[1]:     format           |                       |
// |          |         |      |         | Data[2]:     channels         |                       |
// |          |         |      |         | Data[3-6]:   samplerate       |                       |
// |          |         |      |         | Data[7-10]:  buffer_addr      |                       |
// |          |         |      |         | Data[11-14]: buffer_size      |                       |
// |          |         |      |         | Data[15-18]: period_size      |                       |
// |          |         |      |         | Data[19-22]: buffer_tail      |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x0A   | Data[0]: Audio Device Index   | Open a RX Instance.   |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x0B   | Data[0]: Audio Device Index   | Start a RX Instance.  |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x0C   | Data[0]: Audio Device Index   | Pause a RX Instance.  |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x0D   | Data[0]: Audio Device Index   | Resume a RX Instance. |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x0E   | Data[0]: Audio Device Index   | Stop a RX Instance.   |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x0F   | Data[0]: Audio Device Index   | Close a RX Instance.  |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x10   | Data[0]: Audio Device Index   | Set Parameters for    |
// |          |         |      |         | Data[1]:     format           | a RX Instance.        |
// |          |         |      |         | Data[2]:     channels         |                       |
// |          |         |      |         | Data[3-6]:   samplerate       |                       |
// |          |         |      |         | Data[7-22]:  reserved         |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x11   | Data[0]: Audio Device Index   | Set RX Buffer.        |
// |          |         |      |         | Data[1-6]:   reserved         |                       |
// |          |         |      |         | Data[7-10]:  buffer_addr      |                       |
// |          |         |      |         | Data[11-14]: buffer_size      |                       |
// |          |         |      |         | Data[15-18]: period_size      |                       |
// |          |         |      |         | Data[19-22]: buffer_tail      |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x12   | Data[0]: Audio Device Index   | Suspend a RX Instance.|
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x13   | Data[0]: Audio Device Index   | Resume a RX Instance. |
// |          |         |      |         | Data[1]:     format           |                       |
// |          |         |      |         | Data[2]:     channels         |                       |
// |          |         |      |         | Data[3-6]:   samplerate       |                       |
// |          |         |      |         | Data[7-10]:  buffer_addr      |                       |
// |          |         |      |         | Data[11-14]: buffer_size      |                       |
// |          |         |      |         | Data[15-18]: period_size      |                       |
// |          |         |      |         | Data[19-22]: buffer_tail      |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x14   | Data[0]: Audio Device Index   | Set register value    |
// |          |         |      |         | Data[1-6]:   reserved         | to codec              |
// |          |         |      |         | Data[7-10]:  register         |                       |
// |          |         |      |         | Data[11-14]: value            |                       |
// |          |         |      |         | Data[15-22]: reserved         |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x00 |  0x15   | Data[0]: Audio Device Index   | Get register value    |
// |          |         |      |         | Data[1-6]:   reserved         | from codec            |
// |          |         |      |         | Data[7-10]:  register         |                       |
// |          |         |      |         | Data[11-22]: reserved         |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// Note 1: See <List of Sample Format> for available value of
// Sample Format;
// Note 2: See <List of Audio Channels> for available value of Channels;
// Note 3: Sample Rate of Set Parameters for an Audio TX Instance
// Command and Set Parameters for an Audio RX Instance Command is
// in little-endian format.
//
// SRTM Audio Control Category Response Command Table:
// +----------+---------+------+---------+-------------------------------+-----------------------+
// | Category | Version | Type | Command | Data                          | Function              |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x00   | Data[0]: Audio Device Index   | Reply for Open        |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x01   | Data[0]: Audio Device Index   | Reply for Start       |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x02   | Data[0]: Audio Device Index   | Reply for Pause       |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x03   | Data[0]: Audio Device Index   | Reply for Resume      |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x04   | Data[0]: Audio Device Index   | Reply for Stop        |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x05   | Data[0]: Audio Device Index   | Reply for Close       |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x06   | Data[0]: Audio Device Index   | Reply for Set Param   |
// |          |         |      |         | Data[1]: Return code          | for a TX Instance.    |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x07   | Data[0]: Audio Device Index   | Reply for Set         |
// |          |         |      |         | Data[1]: Return code          | TX Buffer             |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x08   | Data[0]: Audio Device Index   | Reply for Suspend     |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x09   | Data[0]: Audio Device Index   | Reply for Resume      |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x0A   | Data[0]: Audio Device Index   | Reply for Open        |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x0B   | Data[0]: Audio Device Index   | Reply for Start       |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x0C   | Data[0]: Audio Device Index   | Reply for Pause       |
// |          |         |      |         | Data[1]: Return code          | a TX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x0D   | Data[0]: Audio Device Index   | Reply for Resume      |
// |          |         |      |         | Data[1]: Return code          | a RX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x0E   | Data[0]: Audio Device Index   | Reply for Stop        |
// |          |         |      |         | Data[1]: Return code          | a RX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x0F   | Data[0]: Audio Device Index   | Reply for Close       |
// |          |         |      |         | Data[1]: Return code          | a RX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x10   | Data[0]: Audio Device Index   | Reply for Set Param   |
// |          |         |      |         | Data[1]: Return code          | for a RX Instance.    |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x11   | Data[0]: Audio Device Index   | Reply for Set         |
// |          |         |      |         | Data[1]: Return code          | RX Buffer             |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x12   | Data[0]: Audio Device Index   | Reply for Suspend     |
// |          |         |      |         | Data[1]: Return code          | a RX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x13   | Data[0]: Audio Device Index   | Reply for Resume      |
// |          |         |      |         | Data[1]: Return code          | a RX Instance         |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x14   | Data[0]: Audio Device Index   | Reply for Set codec   |
// |          |         |      |         | Data[1]: Return code          | register value        |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x01 |  0x15   | Data[0]: Audio Device Index   | Reply for Get codec   |
// |          |         |      |         | Data[1]: Return code          | register value        |
// |          |         |      |         | Data[2-6]:   reserved         |                       |
// |          |         |      |         | Data[7-10]:  register         |                       |
// |          |         |      |         | Data[11-14]: value            |                       |
// |          |         |      |         | Data[15-22]: reserved         |                       |
// +----------+---------+------+---------+-------------------------------+-----------------------+
//
// SRTM Audio Control Category Notification Command Table:
// +----------+---------+------+---------+-------------------------------+-----------------------+
// | Category | Version | Type | Command | Data                          | Function              |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x02 |  0x00   | Data[0]: Audio Device Index   | Notify one TX period  |
// |          |         |      |         |                               | is finished           |
// +----------+---------+------+---------+-------------------------------+-----------------------+
// |  0x03    | 0x0100  | 0x02 |  0x01   | Data[0]: Audio Device Index   | Notify one RX period  |
// |          |         |      |         |                               | is finished           |
// +----------+---------+------+---------+-------------------------------+-----------------------+
//
// List of Sample Format:
// +------------------+-----------------------+
// | Sample Format    |   Description         |
// +------------------+-----------------------+
// |       0x0        | S16_LE                |
// +------------------+-----------------------+
// |       0x1        | S24_LE                |
// +------------------+-----------------------+
//
// List of Audio Channels
// +------------------+-----------------------+
// |  Audio Channel   |   Description         |
// +------------------+-----------------------+
// |       0x0        | Left Channel          |
// +------------------+-----------------------+
// |       0x1        | Right Channel         |
// +------------------+---------------- ------+
// |       0x2        | Left & Right Channel  |
// +------------------+-----------------------+
//

pub const RPMSG_TIMEOUT: c_int = 1000;
// RPMSG Command (TYPE A)
pub const TX_OPEN: c_uint = 0x0;
pub const TX_START: c_uint = 0x1;
pub const TX_PAUSE: c_uint = 0x2;
pub const TX_RESTART: c_uint = 0x3;
pub const TX_TERMINATE: c_uint = 0x4;
pub const TX_CLOSE: c_uint = 0x5;
pub const TX_HW_PARAM: c_uint = 0x6;
pub const TX_BUFFER: c_uint = 0x7;
pub const TX_SUSPEND: c_uint = 0x8;
pub const TX_RESUME: c_uint = 0x9;
pub const RX_OPEN: c_uint = 0xA;
pub const RX_START: c_uint = 0xB;
pub const RX_PAUSE: c_uint = 0xC;
pub const RX_RESTART: c_uint = 0xD;
pub const RX_TERMINATE: c_uint = 0xE;
pub const RX_CLOSE: c_uint = 0xF;
pub const RX_HW_PARAM: c_uint = 0x10;
pub const RX_BUFFER: c_uint = 0x11;
pub const RX_SUSPEND: c_uint = 0x12;
pub const RX_RESUME: c_uint = 0x13;
pub const SET_CODEC_VALUE: c_uint = 0x14;
pub const GET_CODEC_VALUE: c_uint = 0x15;
pub const TX_POINTER: c_uint = 0x16;
pub const RX_POINTER: c_uint = 0x17;
// Total msg numver for type A
pub const MSG_TYPE_A_NUM: c_uint = 0x18;
// RPMSG Command (TYPE C)
pub const TX_PERIOD_DONE: c_uint = 0x0;
pub const RX_PERIOD_DONE: c_uint = 0x1;
// Total msg numver for type C
pub const MSG_TYPE_C_NUM: c_uint = 0x2;

pub const MSG_TYPE_A: c_uint = 0x0;
pub const MSG_TYPE_B: c_uint = 0x1;
pub const MSG_TYPE_C: c_uint = 0x2;
pub const RESP_NONE: c_uint = 0x0;
pub const RESP_NOT_ALLOWED: c_uint = 0x1;
pub const RESP_SUCCESS: c_uint = 0x2;
pub const RESP_FAILED: c_uint = 0x3;
pub const RPMSG_S16_LE: c_uint = 0x0;
pub const RPMSG_S24_LE: c_uint = 0x1;
pub const RPMSG_S32_LE: c_uint = 0x2;

pub const RPMSG_DSD_U24_LE: c_uint = 0x4;

pub const RPMSG_CH_LEFT: c_uint = 0x0;
pub const RPMSG_CH_RIGHT: c_uint = 0x1;
pub const RPMSG_CH_STEREO: c_uint = 0x2;
pub const WORK_MAX_NUM: c_uint = 0x30;
// Category define
pub const IMX_RMPSG_LIFECYCLE: c_int = 1;
pub const IMX_RPMSG_PMIC: c_int = 2;
pub const IMX_RPMSG_AUDIO: c_int = 3;
pub const IMX_RPMSG_KEY: c_int = 4;
pub const IMX_RPMSG_GPIO: c_int = 5;
pub const IMX_RPMSG_RTC: c_int = 6;
pub const IMX_RPMSG_SENSOR: c_int = 7;
// rpmsg version
pub const IMX_RMPSG_MAJOR: c_int = 1;
pub const IMX_RMPSG_MINOR: c_int = 0;

//
// struct rpmsg_head: rpmsg header structure
//
// @cate: category
// @major: major version
// @minor: minor version
// @type: message type (A/B/C)
// @cmd: message command
// @reserved: reserved space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_head {
    pub cate: u8,
    pub major: u8,
    pub minor: u8,
    pub type: u8,
    pub cmd: u8,
    pub reserved: [u8; 5],
    pub __packed: },
//
// struct param_s: sent rpmsg parameter
//
// @audioindex: audio instance index
// @format: audio format
// @channels: audio channel number
// @rate: sample rate
// @buffer_addr: dma buffer physical address or register for SET_CODEC_VALUE
// @buffer_size: dma buffer size or register value for SET_CODEC_VALUE
// @period_size: period size
// @buffer_tail: current period index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_s {
    pub audioindex: c_uchar,
    pub format: c_uchar,
    pub channels: c_uchar,
    pub rate: c_uint,
    pub buffer_addr: c_uint,
    pub buffer_size: c_uint,
    pub period_size: c_uint,
    pub buffer_tail: c_uint,
    pub __packed: },
//
// struct param_s: send rpmsg parameter
//
// @audioindex: audio instance index
// @resp: response value
// @reserved1: reserved space
// @buffer_offset: the consumed offset of buffer
// @reg_addr: register addr of codec
// @reg_data: register value of codec
// @reserved2: reserved space
// @buffer_tail: current period index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_r {
    pub audioindex: c_uchar,
    pub resp: c_uchar,
    pub reserved1: [c_uchar; 1],
    pub buffer_offset: c_uint,
    pub reg_addr: c_uint,
    pub reg_data: c_uint,
    pub reserved2: [c_uchar; 4],
    pub buffer_tail: c_uint,
    pub __packed: },
// Struct of sent message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_s_msg {
    pub header: rpmsg_head,
    pub param: param_s,
}

// Struct of received message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_r_msg {
    pub header: rpmsg_head,
    pub param: param_r,
}

// Struct of rpmsg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_msg {
    pub s_msg: rpmsg_s_msg,
    pub r_msg: rpmsg_r_msg,
}

// Struct of rpmsg for workqueue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_of_rpmsg {
    pub info: *mut rpmsg_info,
// Sent msg for each work
    pub msg: rpmsg_msg,
    pub work: work_struct,
}

// Struct of timer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream_timer {
    pub timer: timer_list,
    pub info: *mut rpmsg_info,
    pub substream: *mut snd_pcm_substream,
}

extern "C" {
    pub fn void(arg: *mut *mut dma_callback)(void) -> typedef;
}
//
// struct rpmsg_info: rpmsg audio information
//
// @rpdev: pointer of rpmsg_device
// @dev: pointer for imx_pcm_rpmsg device
// @cmd_complete: command is finished
// @pm_qos_req: request of pm qos
// @r_msg: received rpmsg
// @msg: array of rpmsg
// @notify: notification msg (type C) for TX & RX
// @notify_updated: notification flag for TX & RX
// @rpmsg_wq: rpmsg workqueue
// @work_list: array of work list for workqueue
// @work_write_index: write index of work list
// @work_read_index: read index of work list
// @msg_drop_count: counter of dropped msg for TX & RX
// @num_period: period number for TX & RX
// @callback_param: parameter for period elapse callback for TX & RX
// @callback: period elapse callback for TX & RX
// @send_message: function pointer for send message
// @lock: spin lock for TX & RX
// @wq_lock: lock for work queue
// @msg_lock: lock for send message
// @stream_timer: timer for tigger workqueue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_info {
    pub rpdev: *mut rpmsg_device,
    pub dev: *mut device,
    pub cmd_complete: completion,
    pub pm_qos_req: pm_qos_request,
// Received msg (global)
    pub r_msg: rpmsg_r_msg,
    pub msg: [rpmsg_msg; MSG_MAX_NUM],
// period done
    pub notify: [rpmsg_msg; 2],
    pub notify_updated: [bool; 2],
    pub rpmsg_wq: *mut workqueue_struct,
    pub work_list: [work_of_rpmsg; WORK_MAX_NUM],
    pub work_write_index: c_int,
    pub work_read_index: c_int,
    pub msg_drop_count: [c_int; 2],
    pub num_period: [c_int; 2],
    pub callback_param: [*mut c_void; 2],
    pub callback: [dma_callback; 2],
    pub info): *mut *mut *mut int (send_message)(struct rpmsg_msg msg, struct rpmsg_info,
    pub /: *mut *mut spinlock_t lock[2]; / spin lock for resource protection,
    pub /: *mut *mut spinlock_t wq_lock; / spin lock for resource protection,
    pub /: *mut *mut mutex msg_lock; / mutex for resource protection,
    pub stream_timer: [stream_timer; 2],
}

