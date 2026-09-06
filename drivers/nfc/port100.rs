//! Automatically rewritten from C to Rust
//! Source: drivers/nfc/port100.c
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
// Sony NFC Port-100 Series driver
// Copyright (c) 2013, Intel Corporation.
//
// Partly based/Inspired by Stephen Tiedemann's nfcpy
//

pub const SONY_VENDOR_ID: c_uint = 0x054c;
pub const RCS380S_PRODUCT_ID: c_uint = 0x06c1;
pub const RCS380P_PRODUCT_ID: c_uint = 0x06c3;

    NFC_PROTO_MIFARE_MASK   | \
    NFC_PROTO_FELICA_MASK   | \
    NFC_PROTO_NFC_DEP_MASK  | \
    NFC_PROTO_ISO14443_MASK | \
    NFC_PROTO_ISO14443_B_MASK)

    NFC_DIGITAL_DRV_CAPS_TG_CRC)
// Standard port100 frame definitions

    + 2) /* data[0] CC, data[1] SCC */

//
// Max extended frame payload len, excluding CC and SCC
// which are already in PORT100_FRAME_HEADER_LEN.
//
pub const PORT100_FRAME_MAX_PAYLOAD_LEN: c_int = 1001;

    Postamble (1) */
    static u8 ack_frame[PORT100_FRAME_ACK_SIZE] = {
    0x00, 0x00, 0xff, 0x00, 0xff, 0x00
    };

// start of frame
pub const PORT100_FRAME_SOF: c_uint = 0x00FF;
pub const PORT100_FRAME_EXT: c_uint = 0xFFFF;
pub const PORT100_FRAME_ACK: c_uint = 0x00FF;
// Port-100 command: in or out

pub const PORT100_FRAME_DIR_OUT: c_uint = 0xD6;
pub const PORT100_FRAME_DIR_IN: c_uint = 0xD7;
// Port-100 sub-command

pub const PORT100_CMD_GET_FIRMWARE_VERSION: c_uint = 0x20;
pub const PORT100_CMD_GET_COMMAND_TYPE: c_uint = 0x28;
pub const PORT100_CMD_SET_COMMAND_TYPE: c_uint = 0x2A;
pub const PORT100_CMD_IN_SET_RF: c_uint = 0x00;
pub const PORT100_CMD_IN_SET_PROTOCOL: c_uint = 0x02;
pub const PORT100_CMD_IN_COMM_RF: c_uint = 0x04;
pub const PORT100_CMD_TG_SET_RF: c_uint = 0x40;
pub const PORT100_CMD_TG_SET_PROTOCOL: c_uint = 0x42;
pub const PORT100_CMD_TG_SET_RF_OFF: c_uint = 0x46;
pub const PORT100_CMD_TG_COMM_RF: c_uint = 0x48;
pub const PORT100_CMD_SWITCH_RF: c_uint = 0x06;

    ((mask) & (0x01 << (cmd_type)))
pub const PORT100_CMD_TYPE_0: c_int = 0;
pub const PORT100_CMD_TYPE_1: c_int = 1;
pub const PORT100_CMD_STATUS_OK: c_uint = 0x00;
pub const PORT100_CMD_STATUS_TIMEOUT: c_uint = 0x80;
pub const PORT100_MDAA_TGT_HAS_BEEN_ACTIVATED_MASK: c_uint = 0x01;
pub const PORT100_MDAA_TGT_WAS_ACTIVATED_MASK: c_uint = 0x02;
    struct port100;
    typedef void (*port100_send_async_complete_t)(struct port100 *dev, void *arg,
    struct sk_buff *resp);
//
// Setting sets structure for in_set_rf command
//
// @in_*_set_number: Represent the entry indexes in the port-100 RF Base Table.
// This table contains multiple RF setting sets required for RF
// communication.
//
// @in_*_comm_type: Theses fields set the communication type to be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100_in_rf_setting {
    pub in_send_set_number: u8,
    pub in_send_comm_type: u8,
    pub in_recv_set_number: u8,
    pub in_recv_comm_type: u8,
    pub __packed: },
pub const PORT100_COMM_TYPE_IN_212F: c_uint = 0x01;
pub const PORT100_COMM_TYPE_IN_424F: c_uint = 0x02;
pub const PORT100_COMM_TYPE_IN_106A: c_uint = 0x03;
pub const PORT100_COMM_TYPE_IN_106B: c_uint = 0x07;
    static const struct port100_in_rf_setting in_rf_settings[] = {
    [NFC_DIGITAL_RF_TECH_212F] = {
    .in_send_set_number = 1,
    .in_send_comm_type  = PORT100_COMM_TYPE_IN_212F,
    .in_recv_set_number = 15,
    .in_recv_comm_type  = PORT100_COMM_TYPE_IN_212F,
    },
    [NFC_DIGITAL_RF_TECH_424F] = {
    .in_send_set_number = 1,
    .in_send_comm_type  = PORT100_COMM_TYPE_IN_424F,
    .in_recv_set_number = 15,
    .in_recv_comm_type  = PORT100_COMM_TYPE_IN_424F,
    },
    [NFC_DIGITAL_RF_TECH_106A] = {
    .in_send_set_number = 2,
    .in_send_comm_type  = PORT100_COMM_TYPE_IN_106A,
    .in_recv_set_number = 15,
    .in_recv_comm_type  = PORT100_COMM_TYPE_IN_106A,
    },
    [NFC_DIGITAL_RF_TECH_106B] = {
    .in_send_set_number = 3,
    .in_send_comm_type  = PORT100_COMM_TYPE_IN_106B,
    .in_recv_set_number = 15,
    .in_recv_comm_type  = PORT100_COMM_TYPE_IN_106B,
    },
// Ensures the array has NFC_DIGITAL_RF_TECH_LAST elements
    [NFC_DIGITAL_RF_TECH_LAST] = { 0 },
}

//
// struct port100_tg_rf_setting - Setting sets structure for tg_set_rf command
//
// @tg_set_number: Represents the entry index in the port-100 RF Base Table.
// This table contains multiple RF setting sets required for RF
// communication. this field is used for both send and receive
// settings.
//
// @tg_comm_type: Sets the communication type to be used to send and receive
// data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100_tg_rf_setting {
    pub tg_set_number: u8,
    pub tg_comm_type: u8,
    pub __packed: },
pub const PORT100_COMM_TYPE_TG_106A: c_uint = 0x0B;
pub const PORT100_COMM_TYPE_TG_212F: c_uint = 0x0C;
pub const PORT100_COMM_TYPE_TG_424F: c_uint = 0x0D;
    static const struct port100_tg_rf_setting tg_rf_settings[] = {
    [NFC_DIGITAL_RF_TECH_106A] = {
    .tg_set_number = 8,
    .tg_comm_type = PORT100_COMM_TYPE_TG_106A,
    },
    [NFC_DIGITAL_RF_TECH_212F] = {
    .tg_set_number = 8,
    .tg_comm_type = PORT100_COMM_TYPE_TG_212F,
    },
    [NFC_DIGITAL_RF_TECH_424F] = {
    .tg_set_number = 8,
    .tg_comm_type = PORT100_COMM_TYPE_TG_424F,
    },
// Ensures the array has NFC_DIGITAL_RF_TECH_LAST elements
    [NFC_DIGITAL_RF_TECH_LAST] = { 0 },
}

pub const PORT100_IN_PROT_INITIAL_GUARD_TIME: c_uint = 0x00;
pub const PORT100_IN_PROT_ADD_CRC: c_uint = 0x01;
pub const PORT100_IN_PROT_CHECK_CRC: c_uint = 0x02;
pub const PORT100_IN_PROT_MULTI_CARD: c_uint = 0x03;
pub const PORT100_IN_PROT_ADD_PARITY: c_uint = 0x04;
pub const PORT100_IN_PROT_CHECK_PARITY: c_uint = 0x05;
pub const PORT100_IN_PROT_BITWISE_AC_RECV_MODE: c_uint = 0x06;
pub const PORT100_IN_PROT_VALID_BIT_NUMBER: c_uint = 0x07;
pub const PORT100_IN_PROT_CRYPTO1: c_uint = 0x08;
pub const PORT100_IN_PROT_ADD_SOF: c_uint = 0x09;
pub const PORT100_IN_PROT_CHECK_SOF: c_uint = 0x0A;
pub const PORT100_IN_PROT_ADD_EOF: c_uint = 0x0B;
pub const PORT100_IN_PROT_CHECK_EOF: c_uint = 0x0C;
pub const PORT100_IN_PROT_DEAF_TIME: c_uint = 0x0E;
pub const PORT100_IN_PROT_CRM: c_uint = 0x0F;
pub const PORT100_IN_PROT_CRM_MIN_LEN: c_uint = 0x10;
pub const PORT100_IN_PROT_T1_TAG_FRAME: c_uint = 0x11;
pub const PORT100_IN_PROT_RFCA: c_uint = 0x12;
pub const PORT100_IN_PROT_GUARD_TIME_AT_INITIATOR: c_uint = 0x13;
pub const PORT100_IN_PROT_END: c_uint = 0x14;
pub const PORT100_IN_MAX_NUM_PROTOCOLS: c_int = 19;
pub const PORT100_TG_PROT_TU: c_uint = 0x00;
pub const PORT100_TG_PROT_RF_OFF: c_uint = 0x01;
pub const PORT100_TG_PROT_CRM: c_uint = 0x02;
pub const PORT100_TG_PROT_END: c_uint = 0x03;
pub const PORT100_TG_MAX_NUM_PROTOCOLS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100_protocol {
    pub number: u8,
    pub value: u8,
    pub __packed: },
    static const struct port100_protocol
    in_protocols[][PORT100_IN_MAX_NUM_PROTOCOLS + 1] = {
    [NFC_DIGITAL_FRAMING_NFCA_SHORT] = {
    { PORT100_IN_PROT_INITIAL_GUARD_TIME,      6 },
    { PORT100_IN_PROT_ADD_CRC,                 0 },
    { PORT100_IN_PROT_CHECK_CRC,               0 },
    { PORT100_IN_PROT_MULTI_CARD,              0 },
    { PORT100_IN_PROT_ADD_PARITY,              0 },
    { PORT100_IN_PROT_CHECK_PARITY,            1 },
    { PORT100_IN_PROT_BITWISE_AC_RECV_MODE,    0 },
    { PORT100_IN_PROT_VALID_BIT_NUMBER,        7 },
    { PORT100_IN_PROT_CRYPTO1,                 0 },
    { PORT100_IN_PROT_ADD_SOF,                 0 },
    { PORT100_IN_PROT_CHECK_SOF,               0 },
    { PORT100_IN_PROT_ADD_EOF,                 0 },
    { PORT100_IN_PROT_CHECK_EOF,               0 },
    { PORT100_IN_PROT_DEAF_TIME,               4 },
    { PORT100_IN_PROT_CRM,                     0 },
    { PORT100_IN_PROT_CRM_MIN_LEN,             0 },
    { PORT100_IN_PROT_T1_TAG_FRAME,            0 },
    { PORT100_IN_PROT_RFCA,                    0 },
    { PORT100_IN_PROT_GUARD_TIME_AT_INITIATOR, 6 },
    { PORT100_IN_PROT_END,                     0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_STANDARD] = {
    { PORT100_IN_PROT_INITIAL_GUARD_TIME,      6 },
    { PORT100_IN_PROT_ADD_CRC,                 0 },
    { PORT100_IN_PROT_CHECK_CRC,               0 },
    { PORT100_IN_PROT_MULTI_CARD,              0 },
    { PORT100_IN_PROT_ADD_PARITY,              1 },
    { PORT100_IN_PROT_CHECK_PARITY,            1 },
    { PORT100_IN_PROT_BITWISE_AC_RECV_MODE,    0 },
    { PORT100_IN_PROT_VALID_BIT_NUMBER,        8 },
    { PORT100_IN_PROT_CRYPTO1,                 0 },
    { PORT100_IN_PROT_ADD_SOF,                 0 },
    { PORT100_IN_PROT_CHECK_SOF,               0 },
    { PORT100_IN_PROT_ADD_EOF,                 0 },
    { PORT100_IN_PROT_CHECK_EOF,               0 },
    { PORT100_IN_PROT_DEAF_TIME,               4 },
    { PORT100_IN_PROT_CRM,                     0 },
    { PORT100_IN_PROT_CRM_MIN_LEN,             0 },
    { PORT100_IN_PROT_T1_TAG_FRAME,            0 },
    { PORT100_IN_PROT_RFCA,                    0 },
    { PORT100_IN_PROT_GUARD_TIME_AT_INITIATOR, 6 },
    { PORT100_IN_PROT_END,                     0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_STANDARD_WITH_CRC_A] = {
    { PORT100_IN_PROT_INITIAL_GUARD_TIME,      6 },
    { PORT100_IN_PROT_ADD_CRC,                 1 },
    { PORT100_IN_PROT_CHECK_CRC,               1 },
    { PORT100_IN_PROT_MULTI_CARD,              0 },
    { PORT100_IN_PROT_ADD_PARITY,              1 },
    { PORT100_IN_PROT_CHECK_PARITY,            1 },
    { PORT100_IN_PROT_BITWISE_AC_RECV_MODE,    0 },
    { PORT100_IN_PROT_VALID_BIT_NUMBER,        8 },
    { PORT100_IN_PROT_CRYPTO1,                 0 },
    { PORT100_IN_PROT_ADD_SOF,                 0 },
    { PORT100_IN_PROT_CHECK_SOF,               0 },
    { PORT100_IN_PROT_ADD_EOF,                 0 },
    { PORT100_IN_PROT_CHECK_EOF,               0 },
    { PORT100_IN_PROT_DEAF_TIME,               4 },
    { PORT100_IN_PROT_CRM,                     0 },
    { PORT100_IN_PROT_CRM_MIN_LEN,             0 },
    { PORT100_IN_PROT_T1_TAG_FRAME,            0 },
    { PORT100_IN_PROT_RFCA,                    0 },
    { PORT100_IN_PROT_GUARD_TIME_AT_INITIATOR, 6 },
    { PORT100_IN_PROT_END,                     0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_T1T] = {
// nfc_digital_framing_nfca_short
    { PORT100_IN_PROT_ADD_CRC,          2 },
    { PORT100_IN_PROT_CHECK_CRC,        2 },
    { PORT100_IN_PROT_VALID_BIT_NUMBER, 8 },
    { PORT100_IN_PROT_T1_TAG_FRAME,     2 },
    { PORT100_IN_PROT_END,              0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_T2T] = {
// nfc_digital_framing_nfca_standard
    { PORT100_IN_PROT_ADD_CRC,   1 },
    { PORT100_IN_PROT_CHECK_CRC, 0 },
    { PORT100_IN_PROT_END,       0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_T4T] = {
// nfc_digital_framing_nfca_standard_with_crc_a
    { PORT100_IN_PROT_END,       0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_NFC_DEP] = {
// nfc_digital_framing_nfca_standard
    { PORT100_IN_PROT_END, 0 },
    },
    [NFC_DIGITAL_FRAMING_NFCF] = {
    { PORT100_IN_PROT_INITIAL_GUARD_TIME,     18 },
    { PORT100_IN_PROT_ADD_CRC,                 1 },
    { PORT100_IN_PROT_CHECK_CRC,               1 },
    { PORT100_IN_PROT_MULTI_CARD,              0 },
    { PORT100_IN_PROT_ADD_PARITY,              0 },
    { PORT100_IN_PROT_CHECK_PARITY,            0 },
    { PORT100_IN_PROT_BITWISE_AC_RECV_MODE,    0 },
    { PORT100_IN_PROT_VALID_BIT_NUMBER,        8 },
    { PORT100_IN_PROT_CRYPTO1,                 0 },
    { PORT100_IN_PROT_ADD_SOF,                 0 },
    { PORT100_IN_PROT_CHECK_SOF,               0 },
    { PORT100_IN_PROT_ADD_EOF,                 0 },
    { PORT100_IN_PROT_CHECK_EOF,               0 },
    { PORT100_IN_PROT_DEAF_TIME,               4 },
    { PORT100_IN_PROT_CRM,                     0 },
    { PORT100_IN_PROT_CRM_MIN_LEN,             0 },
    { PORT100_IN_PROT_T1_TAG_FRAME,            0 },
    { PORT100_IN_PROT_RFCA,                    0 },
    { PORT100_IN_PROT_GUARD_TIME_AT_INITIATOR, 6 },
    { PORT100_IN_PROT_END,                     0 },
    },
    [NFC_DIGITAL_FRAMING_NFCF_T3T] = {
// nfc_digital_framing_nfcf
    { PORT100_IN_PROT_END, 0 },
    },
    [NFC_DIGITAL_FRAMING_NFCF_NFC_DEP] = {
// nfc_digital_framing_nfcf
    { PORT100_IN_PROT_INITIAL_GUARD_TIME,     18 },
    { PORT100_IN_PROT_ADD_CRC,                 1 },
    { PORT100_IN_PROT_CHECK_CRC,               1 },
    { PORT100_IN_PROT_MULTI_CARD,              0 },
    { PORT100_IN_PROT_ADD_PARITY,              0 },
    { PORT100_IN_PROT_CHECK_PARITY,            0 },
    { PORT100_IN_PROT_BITWISE_AC_RECV_MODE,    0 },
    { PORT100_IN_PROT_VALID_BIT_NUMBER,        8 },
    { PORT100_IN_PROT_CRYPTO1,                 0 },
    { PORT100_IN_PROT_ADD_SOF,                 0 },
    { PORT100_IN_PROT_CHECK_SOF,               0 },
    { PORT100_IN_PROT_ADD_EOF,                 0 },
    { PORT100_IN_PROT_CHECK_EOF,               0 },
    { PORT100_IN_PROT_DEAF_TIME,               4 },
    { PORT100_IN_PROT_CRM,                     0 },
    { PORT100_IN_PROT_CRM_MIN_LEN,             0 },
    { PORT100_IN_PROT_T1_TAG_FRAME,            0 },
    { PORT100_IN_PROT_RFCA,                    0 },
    { PORT100_IN_PROT_GUARD_TIME_AT_INITIATOR, 6 },
    { PORT100_IN_PROT_END,                     0 },
    },
    [NFC_DIGITAL_FRAMING_NFC_DEP_ACTIVATED] = {
    { PORT100_IN_PROT_END, 0 },
    },
    [NFC_DIGITAL_FRAMING_NFCB] = {
    { PORT100_IN_PROT_INITIAL_GUARD_TIME,     20 },
    { PORT100_IN_PROT_ADD_CRC,                 1 },
    { PORT100_IN_PROT_CHECK_CRC,               1 },
    { PORT100_IN_PROT_MULTI_CARD,              0 },
    { PORT100_IN_PROT_ADD_PARITY,              0 },
    { PORT100_IN_PROT_CHECK_PARITY,            0 },
    { PORT100_IN_PROT_BITWISE_AC_RECV_MODE,    0 },
    { PORT100_IN_PROT_VALID_BIT_NUMBER,        8 },
    { PORT100_IN_PROT_CRYPTO1,                 0 },
    { PORT100_IN_PROT_ADD_SOF,                 1 },
    { PORT100_IN_PROT_CHECK_SOF,               1 },
    { PORT100_IN_PROT_ADD_EOF,                 1 },
    { PORT100_IN_PROT_CHECK_EOF,               1 },
    { PORT100_IN_PROT_DEAF_TIME,               4 },
    { PORT100_IN_PROT_CRM,                     0 },
    { PORT100_IN_PROT_CRM_MIN_LEN,             0 },
    { PORT100_IN_PROT_T1_TAG_FRAME,            0 },
    { PORT100_IN_PROT_RFCA,                    0 },
    { PORT100_IN_PROT_GUARD_TIME_AT_INITIATOR, 6 },
    { PORT100_IN_PROT_END,                     0 },
    },
    [NFC_DIGITAL_FRAMING_NFCB_T4T] = {
// nfc_digital_framing_nfcb
    { PORT100_IN_PROT_END,                     0 },
    },
// Ensures the array has NFC_DIGITAL_FRAMING_LAST elements
    [NFC_DIGITAL_FRAMING_LAST] = {
    { PORT100_IN_PROT_END, 0 },
    },
}

    static const struct port100_protocol
    tg_protocols[][PORT100_TG_MAX_NUM_PROTOCOLS + 1] = {
    [NFC_DIGITAL_FRAMING_NFCA_SHORT] = {
    { PORT100_TG_PROT_END, 0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_STANDARD] = {
    { PORT100_TG_PROT_END, 0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_STANDARD_WITH_CRC_A] = {
    { PORT100_TG_PROT_END, 0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_T1T] = {
    { PORT100_TG_PROT_END, 0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_T2T] = {
    { PORT100_TG_PROT_END, 0 },
    },
    [NFC_DIGITAL_FRAMING_NFCA_NFC_DEP] = {
    { PORT100_TG_PROT_TU,     1 },
    { PORT100_TG_PROT_RF_OFF, 0 },
    { PORT100_TG_PROT_CRM,    7 },
    { PORT100_TG_PROT_END,    0 },
    },
    [NFC_DIGITAL_FRAMING_NFCF] = {
    { PORT100_TG_PROT_END, 0 },
    },
    [NFC_DIGITAL_FRAMING_NFCF_T3T] = {
    { PORT100_TG_PROT_END, 0 },
    },
    [NFC_DIGITAL_FRAMING_NFCF_NFC_DEP] = {
    { PORT100_TG_PROT_TU,     1 },
    { PORT100_TG_PROT_RF_OFF, 0 },
    { PORT100_TG_PROT_CRM,    7 },
    { PORT100_TG_PROT_END,    0 },
    },
    [NFC_DIGITAL_FRAMING_NFC_DEP_ACTIVATED] = {
    { PORT100_TG_PROT_RF_OFF, 1 },
    { PORT100_TG_PROT_END,    0 },
    },
// Ensures the array has NFC_DIGITAL_FRAMING_LAST elements
    [NFC_DIGITAL_FRAMING_LAST] = {
    { PORT100_TG_PROT_END,    0 },
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100 {
    pub nfc_digital_dev: *mut nfc_digital_dev,
    pub skb_headroom: c_int,
    pub skb_tailroom: c_int,
    pub udev: *mut usb_device,
    pub interface: *mut usb_interface,
    pub out_urb: *mut urb,
    pub in_urb: *mut urb,
// This mutex protects the out_urb and avoids to submit a new command
// through port100_send_frame_async() while the previous one is being
// canceled through port100_abort_cmd().
//
    pub out_urb_lock: mutex,
    pub cmd_complete_work: work_struct,
    pub cmd_type: u8,
// The digital stack serializes commands to be sent. There is no need
// for any queuing/locking mechanism at driver level.
//
    pub cmd: *mut port100_cmd,
    pub cmd_cancel: bool,
    pub cmd_cancel_done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100_cmd {
    pub code: u8,
    pub status: c_int,
    pub req: *mut sk_buff,
    pub resp: *mut sk_buff,
    pub resp_len: c_int,
    pub complete_cb: port100_send_async_complete_t,
    pub complete_cb_context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100_frame {
    pub preamble: u8,
    pub start_frame: __be16,
    pub extended_frame: __be16,
    pub datalen: __le16,
    pub datalen_checksum: u8,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100_ack_frame {
    pub preamble: u8,
    pub start_frame: __be16,
    pub ack_frame: __be16,
    pub postambule: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100_cb_arg {
    pub complete_cb: nfc_digital_cmd_complete_t,
    pub complete_arg: *mut c_void,
    pub mdaa: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100_tg_comm_rf_cmd {
    pub guard_time: __le16,
    pub send_timeout: __le16,
    pub mdaa: u8,
    pub nfca_param: [u8; 6],
    pub nfcf_param: [u8; 18],
    pub mf_halted: u8,
    pub arae_flag: u8,
    pub recv_timeout: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100_tg_comm_rf_res {
    pub comm_type: u8,
    pub ar_status: u8,
    pub target_activated: u8,
    pub status: __le32,
    pub data: [u8; ],
    pub __packed: },
// The rule: value + checksum = 0
#[no_mangle]
pub unsafe extern "C" fn port100_checksum(value: u16) -> u8 {
    static inline u8 port100_checksum(u16 value)
    {
    pub 1: *mut *mut *mut return ~(((u8 )&value)[0] + ((u8 )&value)[1]) +,
    }
// The rule: sum(data elements) + checksum = 0
#[no_mangle]
unsafe extern "C" fn port100_data_checksum(data: *const u8, datalen: c_int) -> u8 {
    static u8 port100_data_checksum(const u8 *data, int datalen)
    {
    pub 0: u8 sum =,
    pub i: c_int,
    pub i++): for (i = 0; i < datalen;,
    pub data: [sum +=; i],
    pub port100_checksum(sum): return,
    }
#[no_mangle]
unsafe extern "C" fn port100_tx_frame_init(_frame: *mut c_void, cmd_code: u8) {
    static void port100_tx_frame_init(void *_frame, u8 cmd_code)
    {
    pub _frame: *mut *mut port100_frame frame =,
    pub 0: frame->preamble =,
    pub cpu_to_be16(PORT100_FRAME_SOF): frame->start_frame =,
    pub cpu_to_be16(PORT100_FRAME_EXT): frame->extended_frame =,
    pub PORT100_FRAME_DIR_OUT: PORT100_FRAME_DIRECTION(frame) =,
    pub cmd_code: PORT100_FRAME_CMD(frame) =,
    pub cpu_to_le16(2): frame->datalen =,
    }
#[no_mangle]
unsafe extern "C" fn port100_tx_frame_finish(_frame: *mut c_void) {
    static void port100_tx_frame_finish(void *_frame)
    {
    pub _frame: *mut *mut port100_frame frame =,
    pub port100_checksum(le16_to_cpu(frame->datalen)): frame->datalen_checksum =,
    PORT100_FRAME_CHECKSUM(frame) =
    pub le16_to_cpu(frame->datalen)): port100_data_checksum(frame->data,,
    pub 0: PORT100_FRAME_POSTAMBLE(frame) =,
    }
#[no_mangle]
unsafe extern "C" fn port100_tx_update_payload_len(_frame: *mut c_void, len: c_int) {
    static void port100_tx_update_payload_len(void *_frame, int len)
    {
    pub _frame: *mut *mut port100_frame frame =,
    pub len): le16_add_cpu(&frame->datalen,,
    }
#[no_mangle]
unsafe extern "C" fn port100_rx_frame_is_valid(_frame: *const c_void) -> bool {
    static bool port100_rx_frame_is_valid(const void *_frame)
    {
    pub checksum: u8,
    pub _frame: *const *const port100_frame frame =,
    if (frame.start_frame != cpu_to_be16(PORT100_FRAME_SOF) ||
    frame.extended_frame != cpu_to_be16(PORT100_FRAME_EXT))
    pub false: return,
    pub port100_checksum(le16_to_cpu(frame->datalen)): checksum =,
    if (checksum != frame.datalen_checksum)
    pub false: return,
    checksum = port100_data_checksum(frame.data,
    if (checksum != PORT100_FRAME_CHECKSUM(frame))
    pub false: return,
    pub true: return,
    }
#[no_mangle]
unsafe extern "C" fn port100_rx_frame_is_ack(frame: *const port100_ack_frame) -> bool {
    static bool port100_rx_frame_is_ack(const struct port100_ack_frame *frame)
    {
    return (frame.start_frame == cpu_to_be16(PORT100_FRAME_SOF) &&
    pub cpu_to_be16(PORT100_FRAME_ACK)): frame->ack_frame ==,
    }
#[no_mangle]
pub unsafe extern "C" fn port100_rx_frame_size(frame: *const c_void) -> c_int {
    static inline int port100_rx_frame_size(const void *frame)
    {
    pub frame: *const *const port100_frame f =,
    return sizeof(struct port100_frame) + le16_to_cpu(f.datalen) +
    }
    static bool port100_rx_frame_is_cmd_response(const struct port100 *dev,
    const void *frame)
    {
    pub frame: *const *const port100_frame f =,
    pub PORT100_CMD_RESPONSE(dev->cmd->code)): return (PORT100_FRAME_CMD(f) ==,
    }
#[no_mangle]
unsafe extern "C" fn port100_recv_response(urb: *mut urb) {
    static void port100_recv_response(struct urb *urb)
    {
    pub urb->context: *mut *mut port100 dev =,
    pub dev->cmd: *mut *mut port100_cmd cmd =,
    pub in_frame: *mut u8,
    pub urb->status: cmd->status =,
    switch (urb.status) {
    case 0:
    pub /: *mut *mut break; / success,
    case -ECONNRESET:
    case -ENOENT:
    nfc_dbg(&dev.interface.dev,
    pub urb->status): "The urb has been canceled (status %d)\n",,
    pub sched_wq: goto,
    case -ESHUTDOWN:
    default:
    nfc_err(&dev.interface.dev, "Urb failure (status %d)\n",
    pub sched_wq: goto,
    }
    pub dev->in_urb->transfer_buffer: in_frame =,
    if (!port100_rx_frame_is_valid(in_frame)) {
    pub frame\n"): nfc_err(&dev->interface->dev, "Received an invalid,
    pub -EIO: cmd->status =,
    pub sched_wq: goto,
    }
    print_hex_dump_debug("PORT100 RX: ", DUMP_PREFIX_NONE, 16, 1, in_frame,
    pub false): port100_rx_frame_size(in_frame),,
    if (!port100_rx_frame_is_cmd_response(dev, in_frame)) {
    nfc_err(&dev.interface.dev,
    pub command\n"): "It's not the response to the last,
    pub -EIO: cmd->status =,
    pub sched_wq: goto,
    }
    sched_wq:
    }
    static int port100_submit_urb_for_response(const struct port100 *dev,
    gfp_t flags)
    {
    pub port100_recv_response: dev->in_urb->complete =,
    pub flags): return usb_submit_urb(dev->in_urb,,
    }
#[no_mangle]
unsafe extern "C" fn port100_recv_ack(urb: *mut urb) {
    static void port100_recv_ack(struct urb *urb)
    {
    pub urb->context: *mut *mut port100 dev =,
    pub dev->cmd: *mut *mut port100_cmd cmd =,
    pub in_frame: *const port100_ack_frame,
    pub rc: c_int,
    pub urb->status: cmd->status =,
    switch (urb.status) {
    case 0:
    pub /: *mut *mut break; / success,
    case -ECONNRESET:
    case -ENOENT:
    nfc_dbg(&dev.interface.dev,
    pub urb->status): "The urb has been stopped (status %d)\n",,
    pub sched_wq: goto,
    case -ESHUTDOWN:
    default:
    nfc_err(&dev.interface.dev, "Urb failure (status %d)\n",
    pub sched_wq: goto,
    }
    pub dev->in_urb->transfer_buffer: in_frame =,
    if (!port100_rx_frame_is_ack(in_frame)) {
    pub ack\n"): nfc_err(&dev->interface->dev, "Received an invalid,
    pub -EIO: cmd->status =,
    pub sched_wq: goto,
    }
    pub GFP_ATOMIC): rc = port100_submit_urb_for_response(dev,,
    if (rc) {
    nfc_err(&dev.interface.dev,
    pub rc): "usb_submit_urb failed with result %d\n",,
    pub rc: cmd->status =,
    pub sched_wq: goto,
    }
    sched_wq:
    }
#[no_mangle]
unsafe extern "C" fn port100_submit_urb_for_ack(dev: *const port100, flags: gfp_t) -> c_int {
    static int port100_submit_urb_for_ack(const struct port100 *dev, gfp_t flags)
    {
    pub port100_recv_ack: dev->in_urb->complete =,
    pub flags): return usb_submit_urb(dev->in_urb,,
    }
#[no_mangle]
unsafe extern "C" fn port100_send_ack(dev: *mut port100) -> c_int {
    static int port100_send_ack(struct port100 *dev)
    {
    pub 0: int rc =,
//
// If prior cancel is in-flight (dev->cmd_cancel == true), we
// can skip to send cancel. Then this will wait the prior
// cancel, or merged into the next cancel rarely if next
// cancel was started before waiting done. In any case, this
// will be waked up soon or later.
//
    if (!dev.cmd_cancel) {
    pub ack_frame: dev->out_urb->transfer_buffer =,
    pub sizeof(ack_frame): dev->out_urb->transfer_buffer_length =,
    pub GFP_KERNEL): rc = usb_submit_urb(dev->out_urb,,
//
// Set the cmd_cancel flag only if the URB has been
// successfully submitted. It will be reset by the out
// URB completion callback port100_send_complete().
//
    pub !rc: dev->cmd_cancel =,
    }
    if (!rc)
    pub rc: return,
    }
    static int port100_send_frame_async(struct port100 *dev,
    const struct sk_buff *out,
    const struct sk_buff *in, int in_len)
    {
    pub rc: c_int,
// A command cancel frame as been sent through dev->out_urb. Don't try
// to submit a new one.
//
    if (dev.cmd_cancel) {
    pub -EAGAIN: rc =,
    pub exit: goto,
    }
    pub out->data: dev->out_urb->transfer_buffer =,
    pub out->len: dev->out_urb->transfer_buffer_length =,
    pub in->data: dev->in_urb->transfer_buffer =,
    pub in_len: dev->in_urb->transfer_buffer_length =,
    print_hex_dump_debug("PORT100 TX: ", DUMP_PREFIX_NONE, 16, 1,
    pub false): out->data, out->len,,
    pub GFP_KERNEL): rc = usb_submit_urb(dev->out_urb,,
    if (rc)
    pub exit: goto,
    pub GFP_KERNEL): rc = port100_submit_urb_for_ack(dev,,
    if (rc)
    exit:
    pub rc: return,
    }
    static void port100_build_cmd_frame(struct port100 *dev, u8 cmd_code,
    struct sk_buff *skb)
    {
// payload is already there, just update datalen
    pub skb->len: int payload_len =,
    pub PORT100_FRAME_HEADER_LEN): skb_push(skb,,
    pub PORT100_FRAME_TAIL_LEN): skb_put(skb,,
    pub cmd_code): port100_tx_frame_init(skb->data,,
    pub payload_len): port100_tx_update_payload_len(skb->data,,
    }
#[no_mangle]
unsafe extern "C" fn port100_send_async_complete(dev: *mut port100) {
    static void port100_send_async_complete(struct port100 *dev)
    {
    pub dev->cmd: *mut *mut port100_cmd cmd =,
    pub cmd->status: int status =,
    pub cmd->req: *mut *mut sk_buff req =,
    pub cmd->resp: *mut *mut sk_buff resp =,
    pub NULL: dev->cmd =,
    if (status < 0) {
    cmd.complete_cb(dev, cmd.complete_cb_context,
    pub done: goto,
    }
    pub port100_rx_frame_size(resp->data)): skb_put(resp,,
    pub PORT100_FRAME_HEADER_LEN): skb_pull(resp,,
    pub PORT100_FRAME_TAIL_LEN): skb_trim(resp, resp->len -,
    pub resp): cmd->complete_cb(dev, cmd->complete_cb_context,,
    done:
    }
    static int port100_send_cmd_async(struct port100 *dev, u8 cmd_code,
    struct sk_buff *req,
    port100_send_async_complete_t complete_cb,
    void *complete_cb_context)
    {
    pub cmd: *mut port100_cmd,
    pub resp: *mut sk_buff,
    pub rc: c_int,
    int  resp_len = PORT100_FRAME_HEADER_LEN +
    PORT100_FRAME_MAX_PAYLOAD_LEN +
    if (dev.cmd) {
    nfc_err(&dev.interface.dev,
    pub process\n"): "A command is still in,
    pub -EBUSY: return,
    }
    pub GFP_KERNEL): resp = alloc_skb(resp_len,,
    if (!resp)
    pub -ENOMEM: return,
    pub kzalloc_obj(*cmd): *mut cmd =,
    if (!cmd) {
    pub -ENOMEM: return,
    }
    pub cmd_code: cmd->code =,
    pub req: cmd->req =,
    pub resp: cmd->resp =,
    pub resp_len: cmd->resp_len =,
    pub complete_cb: cmd->complete_cb =,
    pub complete_cb_context: cmd->complete_cb_context =,
    pub req): port100_build_cmd_frame(dev, cmd_code,,
    pub cmd: dev->cmd =,
    pub resp_len): rc = port100_send_frame_async(dev, req, resp,,
    if (rc) {
    pub NULL: dev->cmd =,
    }
    pub rc: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct port100_sync_cmd_response {
    pub resp: *mut sk_buff,
    pub done: completion,
}

#[no_mangle]
unsafe extern "C" fn port100_wq_cmd_complete(work: *mut work_struct) {
    static void port100_wq_cmd_complete(struct work_struct *work)
    {
    struct port100 *dev = container_of(work, struct port100,
    cmd_complete_work);
    port100_send_async_complete(dev);
    }
    static void port100_send_sync_complete(struct port100 *dev, void *_arg,
    struct sk_buff *resp)
    {
    struct port100_sync_cmd_response *arg = _arg;
    arg.resp = resp;
    complete(&arg.done);
    }
    static struct sk_buff *port100_send_cmd_sync(struct port100 *dev, u8 cmd_code,
    struct sk_buff *req)
    {
    int rc;
    struct port100_sync_cmd_response arg;
    init_completion(&arg.done);
    rc = port100_send_cmd_async(dev, cmd_code, req,
    port100_send_sync_complete, &arg);
    if (rc) {
    dev_kfree_skb(req);
    return ERR_PTR(rc);
    }
    wait_for_completion(&arg.done);
    return arg.resp;
    }
#[no_mangle]
unsafe extern "C" fn port100_send_complete(urb: *mut urb) {
    static void port100_send_complete(struct urb *urb)
    {
    struct port100 *dev = urb.context;
    if (dev.cmd_cancel) {
    complete_all(&dev.cmd_cancel_done);
    dev.cmd_cancel = false;
    }
    switch (urb.status) {
    case 0:
    break; /* success */
    case -ECONNRESET:
    case -ENOENT:
    nfc_dbg(&dev.interface.dev,
    "The urb has been stopped (status %d)\n", urb.status);
    break;
    case -ESHUTDOWN:
    default:
    nfc_err(&dev.interface.dev, "Urb failure (status %d)\n",
    urb.status);
    }
    }
#[no_mangle]
unsafe extern "C" fn port100_abort_cmd(ddev: *mut nfc_digital_dev) {
    static void port100_abort_cmd(struct nfc_digital_dev *ddev)
    {
    struct port100 *dev = nfc_digital_get_drvdata(ddev);
// An ack will cancel the last issued command
    port100_send_ack(dev);
// cancel the urb request
    usb_kill_urb(dev.in_urb);
    }
    static struct sk_buff *port100_alloc_skb(const struct port100 *dev, unsigned int size)
    {
    struct sk_buff *skb;
    skb = alloc_skb(dev.skb_headroom + dev.skb_tailroom + size,
    GFP_KERNEL);
    if (skb)
    skb_reserve(skb, dev.skb_headroom);
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn port100_set_command_type(dev: *mut port100, command_type: u8) -> c_int {
    static int port100_set_command_type(struct port100 *dev, u8 command_type)
    {
    struct sk_buff *skb;
    struct sk_buff *resp;
    int rc;
    skb = port100_alloc_skb(dev, 1);
    if (!skb)
    return -ENOMEM;
    skb_put_u8(skb, command_type);
    resp = port100_send_cmd_sync(dev, PORT100_CMD_SET_COMMAND_TYPE, skb);
    if (IS_ERR(resp))
    return PTR_ERR(resp);
    rc = resp.data[0];
    dev_kfree_skb(resp);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn port100_get_command_type_mask(dev: *mut port100) -> u64 {
    static u64 port100_get_command_type_mask(struct port100 *dev)
    {
    struct sk_buff *skb;
    struct sk_buff *resp;
    u64 mask;
    skb = port100_alloc_skb(dev, 0);
    if (!skb)
    return 0;
    resp = port100_send_cmd_sync(dev, PORT100_CMD_GET_COMMAND_TYPE, skb);
    if (IS_ERR(resp))
    return 0;
    if (resp.len < 8)
    mask = 0;
    else
    mask = be64_to_cpu(*(__be64 *)resp.data);
    dev_kfree_skb(resp);
    return mask;
    }
#[no_mangle]
unsafe extern "C" fn port100_get_firmware_version(dev: *mut port100) -> u16 {
    static u16 port100_get_firmware_version(struct port100 *dev)
    {
    struct sk_buff *skb;
    struct sk_buff *resp;
    u16 fw_ver;
    skb = port100_alloc_skb(dev, 0);
    if (!skb)
    return 0;
    resp = port100_send_cmd_sync(dev, PORT100_CMD_GET_FIRMWARE_VERSION,
    skb);
    if (IS_ERR(resp))
    return 0;
    fw_ver = le16_to_cpu(*(__le16 *)resp.data);
    dev_kfree_skb(resp);
    return fw_ver;
    }
#[no_mangle]
unsafe extern "C" fn port100_switch_rf(ddev: *mut nfc_digital_dev, on: bool) -> c_int {
    static int port100_switch_rf(struct nfc_digital_dev *ddev, bool on)
    {
    struct port100 *dev = nfc_digital_get_drvdata(ddev);
    struct sk_buff *skb, *resp;
    skb = port100_alloc_skb(dev, 1);
    if (!skb)
    return -ENOMEM;
    skb_put_u8(skb, on ? 1 : 0);
// Cancel the last command if the device is being switched off
    if (!on)
    port100_abort_cmd(ddev);
    resp = port100_send_cmd_sync(dev, PORT100_CMD_SWITCH_RF, skb);
    if (IS_ERR(resp))
    return PTR_ERR(resp);
    dev_kfree_skb(resp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn port100_in_set_rf(ddev: *mut nfc_digital_dev, rf: u8) -> c_int {
    static int port100_in_set_rf(struct nfc_digital_dev *ddev, u8 rf)
    {
    struct port100 *dev = nfc_digital_get_drvdata(ddev);
    struct sk_buff *skb;
    struct sk_buff *resp;
    int rc;
    if (rf >= NFC_DIGITAL_RF_TECH_LAST)
    return -EINVAL;
    skb = port100_alloc_skb(dev, sizeof(struct port100_in_rf_setting));
    if (!skb)
    return -ENOMEM;
    skb_put_data(skb, &in_rf_settings[rf],
    sizeof(struct port100_in_rf_setting));
    resp = port100_send_cmd_sync(dev, PORT100_CMD_IN_SET_RF, skb);
    if (IS_ERR(resp))
    return PTR_ERR(resp);
    rc = resp.data[0];
    dev_kfree_skb(resp);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn port100_in_set_framing(ddev: *mut nfc_digital_dev, param: c_int) -> c_int {
    static int port100_in_set_framing(struct nfc_digital_dev *ddev, int param)
    {
    struct port100 *dev = nfc_digital_get_drvdata(ddev);
    const struct port100_protocol *protocols;
    struct sk_buff *skb;
    struct sk_buff *resp;
    int num_protocols;
    size_t size;
    int rc;
    if (param >= NFC_DIGITAL_FRAMING_LAST)
    return -EINVAL;
    protocols = in_protocols[param];
    num_protocols = 0;
    while (protocols[num_protocols].number != PORT100_IN_PROT_END)
    num_protocols++;
    if (!num_protocols)
    return 0;
    size = sizeof(struct port100_protocol) * num_protocols;
    skb = port100_alloc_skb(dev, size);
    if (!skb)
    return -ENOMEM;
    skb_put_data(skb, protocols, size);
    resp = port100_send_cmd_sync(dev, PORT100_CMD_IN_SET_PROTOCOL, skb);
    if (IS_ERR(resp))
    return PTR_ERR(resp);
    rc = resp.data[0];
    dev_kfree_skb(resp);
    return rc;
    }
    static int port100_in_configure_hw(struct nfc_digital_dev *ddev, int type,
    int param)
    {
    if (type == NFC_DIGITAL_CONFIG_RF_TECH)
    return port100_in_set_rf(ddev, param);
    if (type == NFC_DIGITAL_CONFIG_FRAMING)
    return port100_in_set_framing(ddev, param);
    return -EINVAL;
    }
    static void port100_in_comm_rf_complete(struct port100 *dev, void *arg,
    struct sk_buff *resp)
    {
    const struct port100_cb_arg *cb_arg = arg;
    let mut cb: nfc_digital_cmd_complete_t = cb_arg.complete_cb;
    u32 status;
    int rc;
    if (IS_ERR(resp)) {
    rc =  PTR_ERR(resp);
    goto exit;
    }
    if (resp.len < 4) {
    nfc_err(&dev.interface.dev,
    "Invalid packet length received\n");
    rc = -EIO;
    goto error;
    }
    status = le32_to_cpu(*(__le32 *)resp.data);
    skb_pull(resp, sizeof(u32));
    if (status == PORT100_CMD_STATUS_TIMEOUT) {
    rc = -ETIMEDOUT;
    goto error;
    }
    if (status != PORT100_CMD_STATUS_OK) {
    nfc_err(&dev.interface.dev,
    "in_comm_rf failed with status 0x%08x\n", status);
    rc = -EIO;
    goto error;
    }
// Remove collision bits byte
    skb_pull(resp, 1);
    goto exit;
    error:
    kfree_skb(resp);
    resp = ERR_PTR(rc);
    exit:
    cb(dev.nfc_digital_dev, cb_arg.complete_arg, resp);
    kfree(cb_arg);
    }
    static int port100_in_send_cmd(struct nfc_digital_dev *ddev,
    struct sk_buff *skb, u16 _timeout,
    nfc_digital_cmd_complete_t cb, void *arg)
    {
    struct port100 *dev = nfc_digital_get_drvdata(ddev);
    struct port100_cb_arg *cb_arg;
    __le16 timeout;
    cb_arg = kzalloc_obj(struct port100_cb_arg);
    if (!cb_arg)
    return -ENOMEM;
    cb_arg.complete_cb = cb;
    cb_arg.complete_arg = arg;
    timeout = cpu_to_le16(_timeout * 10);
    memcpy(skb_push(skb, sizeof(__le16)), &timeout, sizeof(__le16));
    return port100_send_cmd_async(dev, PORT100_CMD_IN_COMM_RF, skb,
    port100_in_comm_rf_complete, cb_arg);
    }
#[no_mangle]
unsafe extern "C" fn port100_tg_set_rf(ddev: *mut nfc_digital_dev, rf: u8) -> c_int {
    static int port100_tg_set_rf(struct nfc_digital_dev *ddev, u8 rf)
    {
    struct port100 *dev = nfc_digital_get_drvdata(ddev);
    struct sk_buff *skb;
    struct sk_buff *resp;
    int rc;
    if (rf >= NFC_DIGITAL_RF_TECH_LAST)
    return -EINVAL;
    skb = port100_alloc_skb(dev, sizeof(struct port100_tg_rf_setting));
    if (!skb)
    return -ENOMEM;
    skb_put_data(skb, &tg_rf_settings[rf],
    sizeof(struct port100_tg_rf_setting));
    resp = port100_send_cmd_sync(dev, PORT100_CMD_TG_SET_RF, skb);
    if (IS_ERR(resp))
    return PTR_ERR(resp);
    rc = resp.data[0];
    dev_kfree_skb(resp);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn port100_tg_set_framing(ddev: *mut nfc_digital_dev, param: c_int) -> c_int {
    static int port100_tg_set_framing(struct nfc_digital_dev *ddev, int param)
    {
    struct port100 *dev = nfc_digital_get_drvdata(ddev);
    const struct port100_protocol *protocols;
    struct sk_buff *skb;
    struct sk_buff *resp;
    int rc;
    int num_protocols;
    size_t size;
    if (param >= NFC_DIGITAL_FRAMING_LAST)
    return -EINVAL;
    protocols = tg_protocols[param];
    num_protocols = 0;
    while (protocols[num_protocols].number != PORT100_TG_PROT_END)
    num_protocols++;
    if (!num_protocols)
    return 0;
    size = sizeof(struct port100_protocol) * num_protocols;
    skb = port100_alloc_skb(dev, size);
    if (!skb)
    return -ENOMEM;
    skb_put_data(skb, protocols, size);
    resp = port100_send_cmd_sync(dev, PORT100_CMD_TG_SET_PROTOCOL, skb);
    if (IS_ERR(resp))
    return PTR_ERR(resp);
    rc = resp.data[0];
    dev_kfree_skb(resp);
    return rc;
    }
    static int port100_tg_configure_hw(struct nfc_digital_dev *ddev, int type,
    int param)
    {
    if (type == NFC_DIGITAL_CONFIG_RF_TECH)
    return port100_tg_set_rf(ddev, param);
    if (type == NFC_DIGITAL_CONFIG_FRAMING)
    return port100_tg_set_framing(ddev, param);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn port100_tg_target_activated(dev: *mut port100, tgt_activated: u8) -> bool {
    static bool port100_tg_target_activated(struct port100 *dev, u8 tgt_activated)
    {
    u8 mask;
    switch (dev.cmd_type) {
    case PORT100_CMD_TYPE_0:
    mask = PORT100_MDAA_TGT_HAS_BEEN_ACTIVATED_MASK;
    break;
    case PORT100_CMD_TYPE_1:
    mask = PORT100_MDAA_TGT_HAS_BEEN_ACTIVATED_MASK |
    PORT100_MDAA_TGT_WAS_ACTIVATED_MASK;
    break;
    default:
    nfc_err(&dev.interface.dev, "Unknown command type\n");
    return false;
    }
    return ((tgt_activated & mask) == mask);
    }
    static void port100_tg_comm_rf_complete(struct port100 *dev, void *arg,
    struct sk_buff *resp)
    {
    u32 status;
    const struct port100_cb_arg *cb_arg = arg;
    let mut cb: nfc_digital_cmd_complete_t = cb_arg.complete_cb;
    struct port100_tg_comm_rf_res *hdr;
    if (IS_ERR(resp))
    goto exit;
    hdr = (struct port100_tg_comm_rf_res *)resp.data;
    status = le32_to_cpu(hdr.status);
    if (cb_arg.mdaa &&
    !port100_tg_target_activated(dev, hdr.target_activated)) {
    kfree_skb(resp);
    resp = ERR_PTR(-ETIMEDOUT);
    goto exit;
    }
    skb_pull(resp, sizeof(struct port100_tg_comm_rf_res));
    if (status != PORT100_CMD_STATUS_OK) {
    kfree_skb(resp);
    if (status == PORT100_CMD_STATUS_TIMEOUT)
    resp = ERR_PTR(-ETIMEDOUT);
    else
    resp = ERR_PTR(-EIO);
    }
    exit:
    cb(dev.nfc_digital_dev, cb_arg.complete_arg, resp);
    kfree(cb_arg);
    }
    static int port100_tg_send_cmd(struct nfc_digital_dev *ddev,
    struct sk_buff *skb, u16 timeout,
    nfc_digital_cmd_complete_t cb, void *arg)
    {
    struct port100 *dev = nfc_digital_get_drvdata(ddev);
    struct port100_tg_comm_rf_cmd *hdr;
    struct port100_cb_arg *cb_arg;
    cb_arg = kzalloc_obj(struct port100_cb_arg);
    if (!cb_arg)
    return -ENOMEM;
    cb_arg.complete_cb = cb;
    cb_arg.complete_arg = arg;
    skb_push(skb, sizeof(struct port100_tg_comm_rf_cmd));
    hdr = (struct port100_tg_comm_rf_cmd *)skb.data;
    memset(hdr, 0, sizeof(struct port100_tg_comm_rf_cmd));
    hdr.guard_time = cpu_to_le16(500);
    hdr.send_timeout = cpu_to_le16(0xFFFF);
    hdr.recv_timeout = cpu_to_le16(timeout);
    return port100_send_cmd_async(dev, PORT100_CMD_TG_COMM_RF, skb,
    port100_tg_comm_rf_complete, cb_arg);
    }
    static int port100_listen_mdaa(struct nfc_digital_dev *ddev,
    struct digital_tg_mdaa_params *params,
    u16 timeout,
    nfc_digital_cmd_complete_t cb, void *arg)
    {
    struct port100 *dev = nfc_digital_get_drvdata(ddev);
    struct port100_tg_comm_rf_cmd *hdr;
    struct port100_cb_arg *cb_arg;
    struct sk_buff *skb;
    int rc;
    rc = port100_tg_configure_hw(ddev, NFC_DIGITAL_CONFIG_RF_TECH,
    NFC_DIGITAL_RF_TECH_106A);
    if (rc)
    return rc;
    rc = port100_tg_configure_hw(ddev, NFC_DIGITAL_CONFIG_FRAMING,
    NFC_DIGITAL_FRAMING_NFCA_NFC_DEP);
    if (rc)
    return rc;
    cb_arg = kzalloc_obj(struct port100_cb_arg);
    if (!cb_arg)
    return -ENOMEM;
    cb_arg.complete_cb = cb;
    cb_arg.complete_arg = arg;
    cb_arg.mdaa = 1;
    skb = port100_alloc_skb(dev, 0);
    if (!skb) {
    kfree(cb_arg);
    return -ENOMEM;
    }
    skb_push(skb, sizeof(struct port100_tg_comm_rf_cmd));
    hdr = (struct port100_tg_comm_rf_cmd *)skb.data;
    memset(hdr, 0, sizeof(struct port100_tg_comm_rf_cmd));
    hdr.guard_time = 0;
    hdr.send_timeout = cpu_to_le16(0xFFFF);
    hdr.mdaa = 1;
    hdr.nfca_param[0] = (params.sens_res >> 8) & 0xFF;
    hdr.nfca_param[1] = params.sens_res & 0xFF;
    memcpy(hdr.nfca_param + 2, params.nfcid1, 3);
    hdr.nfca_param[5] = params.sel_res;
    memcpy(hdr.nfcf_param, params.nfcid2, 8);
    hdr.nfcf_param[16] = (params.sc >> 8) & 0xFF;
    hdr.nfcf_param[17] = params.sc & 0xFF;
    hdr.recv_timeout = cpu_to_le16(timeout);
    return port100_send_cmd_async(dev, PORT100_CMD_TG_COMM_RF, skb,
    port100_tg_comm_rf_complete, cb_arg);
    }
    static int port100_listen(struct nfc_digital_dev *ddev, u16 timeout,
    nfc_digital_cmd_complete_t cb, void *arg)
    {
    const struct port100 *dev = nfc_digital_get_drvdata(ddev);
    struct sk_buff *skb;
    skb = port100_alloc_skb(dev, 0);
    if (!skb)
    return -ENOMEM;
    return port100_tg_send_cmd(ddev, skb, timeout, cb, arg);
    }
    static const struct nfc_digital_ops port100_digital_ops = {
    .in_configure_hw = port100_in_configure_hw,
    .in_send_cmd = port100_in_send_cmd,
    .tg_listen_mdaa = port100_listen_mdaa,
    .tg_listen = port100_listen,
    .tg_configure_hw = port100_tg_configure_hw,
    .tg_send_cmd = port100_tg_send_cmd,
    .switch_rf = port100_switch_rf,
    .abort_cmd = port100_abort_cmd,
    };
    static const struct usb_device_id port100_table[] = {
    { USB_DEVICE(SONY_VENDOR_ID, RCS380S_PRODUCT_ID) },
    { USB_DEVICE(SONY_VENDOR_ID, RCS380P_PRODUCT_ID) },
    { }
    };
    MODULE_DEVICE_TABLE(usb, port100_table);
    static int port100_probe(struct usb_interface *interface,
    const struct usb_device_id *id)
    {
    struct usb_endpoint_descriptor *ep_in, *ep_out;
    struct port100 *dev;
    int rc;
    u16 fw_version;
    u64 cmd_type_mask;
    dev = devm_kzalloc(&interface.dev, sizeof(struct port100), GFP_KERNEL);
    if (!dev)
    return -ENOMEM;
    mutex_init(&dev.out_urb_lock);
    dev.udev = interface_to_usbdev(interface);
    dev.interface = interface;
    usb_set_intfdata(interface, dev);
    rc = usb_find_common_endpoints(interface.cur_altsetting, &ep_in,
    &ep_out, core::ptr::null_mut(), core::ptr::null_mut());
    if (rc) {
    nfc_err(&interface.dev,
    "Could not find bulk-in or bulk-out endpoint\n");
    goto error;
    }
    dev.in_urb = usb_alloc_urb(0, GFP_KERNEL);
    dev.out_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!dev.in_urb || !dev.out_urb) {
    nfc_err(&interface.dev, "Could not allocate USB URBs\n");
    rc = -ENOMEM;
    goto error;
    }
    usb_fill_bulk_urb(dev.in_urb, dev.udev,
    usb_rcvbulkpipe(dev.udev, usb_endpoint_num(ep_in)),
    core::ptr::null_mut(), 0, core::ptr::null_mut(), dev);
    usb_fill_bulk_urb(dev.out_urb, dev.udev,
    usb_sndbulkpipe(dev.udev, usb_endpoint_num(ep_out)),
    core::ptr::null_mut(), 0, port100_send_complete, dev);
    dev.out_urb.transfer_flags = URB_ZERO_PACKET;
    dev.skb_headroom = PORT100_FRAME_HEADER_LEN +
    PORT100_COMM_RF_HEAD_MAX_LEN;
    dev.skb_tailroom = PORT100_FRAME_TAIL_LEN;
    init_completion(&dev.cmd_cancel_done);
    INIT_WORK(&dev.cmd_complete_work, port100_wq_cmd_complete);
// The first thing to do with the Port-100 is to set the command type
// to be used. If supported we use command type 1. 0 otherwise.
//
    cmd_type_mask = port100_get_command_type_mask(dev);
    if (!cmd_type_mask) {
    nfc_err(&interface.dev,
    "Could not get supported command types\n");
    rc = -ENODEV;
    goto error;
    }
    if (PORT100_CMD_TYPE_IS_SUPPORTED(cmd_type_mask, PORT100_CMD_TYPE_1))
    dev.cmd_type = PORT100_CMD_TYPE_1;
    else
    dev.cmd_type = PORT100_CMD_TYPE_0;
    rc = port100_set_command_type(dev, dev.cmd_type);
    if (rc) {
    nfc_err(&interface.dev,
    "The device does not support command type %u\n",
    dev.cmd_type);
    goto error;
    }
    fw_version = port100_get_firmware_version(dev);
    if (!fw_version)
    nfc_err(&interface.dev,
    "Could not get device firmware version\n");
    nfc_info(&interface.dev,
    "Sony NFC Port-100 Series attached (firmware v%x.%02x)\n",
    (fw_version & 0xFF00) >> 8, fw_version & 0xFF);
    dev.nfc_digital_dev = nfc_digital_allocate_device(&port100_digital_ops,
    PORT100_PROTOCOLS,
    PORT100_CAPABILITIES,
    dev.skb_headroom,
    dev.skb_tailroom);
    if (!dev.nfc_digital_dev) {
    nfc_err(&interface.dev,
    "Could not allocate nfc_digital_dev\n");
    rc = -ENOMEM;
    goto error;
    }
    nfc_digital_set_parent_dev(dev.nfc_digital_dev, &interface.dev);
    nfc_digital_set_drvdata(dev.nfc_digital_dev, dev);
    rc = nfc_digital_register_device(dev.nfc_digital_dev);
    if (rc) {
    nfc_err(&interface.dev,
    "Could not register digital device\n");
    goto free_nfc_dev;
    }
    return 0;
    free_nfc_dev:
    nfc_digital_free_device(dev.nfc_digital_dev);
    error:
    usb_kill_urb(dev.in_urb);
    usb_free_urb(dev.in_urb);
    usb_kill_urb(dev.out_urb);
    usb_free_urb(dev.out_urb);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn port100_disconnect(interface: *mut usb_interface) {
    static void port100_disconnect(struct usb_interface *interface)
    {
    struct port100 *dev;
    dev = usb_get_intfdata(interface);
    usb_set_intfdata(interface, core::ptr::null_mut());
    nfc_digital_unregister_device(dev.nfc_digital_dev);
    nfc_digital_free_device(dev.nfc_digital_dev);
    usb_kill_urb(dev.in_urb);
    usb_kill_urb(dev.out_urb);
    usb_free_urb(dev.in_urb);
    usb_free_urb(dev.out_urb);
    kfree(dev.cmd);
    nfc_info(&interface.dev, "Sony Port-100 NFC device disconnected\n");
    }
    static struct usb_driver port100_driver = {
    .name =		"port100",
    .probe =	port100_probe,
    .disconnect =	port100_disconnect,
    .id_table =	port100_table,
    };
    module_usb_driver(port100_driver);
    MODULE_DESCRIPTION("NFC Port-100 series usb driver ver " VERSION);
    MODULE_VERSION(VERSION);
    MODULE_LICENSE("GPL");
