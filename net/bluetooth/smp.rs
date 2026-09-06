//! Automatically rewritten from C Header to Rust Module
//! Source: net/bluetooth/smp.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_command_hdr {
    pub code: __u8,
    pub __packed: },
pub const SMP_CMD_PAIRING_REQ: c_uint = 0x01;
pub const SMP_CMD_PAIRING_RSP: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_pairing {
    pub io_capability: __u8,
    pub oob_flag: __u8,
    pub auth_req: __u8,
    pub max_key_size: __u8,
    pub init_key_dist: __u8,
    pub resp_key_dist: __u8,
    pub __packed: },
pub const SMP_IO_DISPLAY_ONLY: c_uint = 0x00;
pub const SMP_IO_DISPLAY_YESNO: c_uint = 0x01;
pub const SMP_IO_KEYBOARD_ONLY: c_uint = 0x02;
pub const SMP_IO_NO_INPUT_OUTPUT: c_uint = 0x03;
pub const SMP_IO_KEYBOARD_DISPLAY: c_uint = 0x04;
pub const SMP_OOB_NOT_PRESENT: c_uint = 0x00;
pub const SMP_OOB_PRESENT: c_uint = 0x01;
pub const SMP_DIST_ENC_KEY: c_uint = 0x01;
pub const SMP_DIST_ID_KEY: c_uint = 0x02;
pub const SMP_DIST_SIGN: c_uint = 0x04;
pub const SMP_DIST_LINK_KEY: c_uint = 0x08;
pub const SMP_AUTH_NONE: c_uint = 0x00;
pub const SMP_AUTH_BONDING: c_uint = 0x01;
pub const SMP_AUTH_MITM: c_uint = 0x04;
pub const SMP_AUTH_SC: c_uint = 0x08;
pub const SMP_AUTH_KEYPRESS: c_uint = 0x10;
pub const SMP_AUTH_CT2: c_uint = 0x20;
pub const SMP_CMD_PAIRING_CONFIRM: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_pairing_confirm {
    pub confirm_val: [__u8; 16],
    pub __packed: },
pub const SMP_CMD_PAIRING_RANDOM: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_pairing_random {
    pub rand_val: [__u8; 16],
    pub __packed: },
pub const SMP_CMD_PAIRING_FAIL: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_pairing_fail {
    pub reason: __u8,
    pub __packed: },
pub const SMP_CMD_ENCRYPT_INFO: c_uint = 0x06;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_encrypt_info {
    pub ltk: [__u8; 16],
    pub __packed: },
pub const SMP_CMD_INITIATOR_IDENT: c_uint = 0x07;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_initiator_ident {
    pub ediv: __le16,
    pub rand: __le64,
    pub __packed: },
pub const SMP_CMD_IDENT_INFO: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_ident_info {
    pub irk: [__u8; 16],
    pub __packed: },
pub const SMP_CMD_IDENT_ADDR_INFO: c_uint = 0x09;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_ident_addr_info {
    pub addr_type: __u8,
    pub bdaddr: bdaddr_t,
    pub __packed: },
pub const SMP_CMD_SIGN_INFO: c_uint = 0x0a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_sign_info {
    pub csrk: [__u8; 16],
    pub __packed: },
pub const SMP_CMD_SECURITY_REQ: c_uint = 0x0b;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_security_req {
    pub auth_req: __u8,
    pub __packed: },
pub const SMP_CMD_PUBLIC_KEY: c_uint = 0x0c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_public_key {
    pub x: [__u8; 32],
    pub y: [__u8; 32],
    pub __packed: },
pub const SMP_CMD_DHKEY_CHECK: c_uint = 0x0d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_dhkey_check {
    pub e: [__u8; 16],
    pub __packed: },
pub const SMP_CMD_KEYPRESS_NOTIFY: c_uint = 0x0e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_cmd_keypress_notify {
    pub value: __u8,
    pub __packed: },
pub const SMP_CMD_MAX: c_uint = 0x0e;
pub const SMP_PASSKEY_ENTRY_FAILED: c_uint = 0x01;
pub const SMP_OOB_NOT_AVAIL: c_uint = 0x02;
pub const SMP_AUTH_REQUIREMENTS: c_uint = 0x03;
pub const SMP_CONFIRM_FAILED: c_uint = 0x04;
pub const SMP_PAIRING_NOTSUPP: c_uint = 0x05;
pub const SMP_ENC_KEY_SIZE: c_uint = 0x06;
pub const SMP_CMD_NOTSUPP: c_uint = 0x07;
pub const SMP_UNSPECIFIED: c_uint = 0x08;
pub const SMP_REPEATED_ATTEMPTS: c_uint = 0x09;
pub const SMP_INVALID_PARAMS: c_uint = 0x0a;
pub const SMP_DHKEY_CHECK_FAILED: c_uint = 0x0b;
pub const SMP_NUMERIC_COMP_FAILED: c_uint = 0x0c;
pub const SMP_BREDR_PAIRING_IN_PROGRESS: c_uint = 0x0d;
pub const SMP_CROSS_TRANSP_NOT_ALLOWED: c_uint = 0x0e;
pub const SMP_KEY_REJECTED: c_uint = 0x0f;
pub const SMP_MIN_ENC_KEY_SIZE: c_int = 7;
pub const SMP_MAX_ENC_KEY_SIZE: c_int = 16;
// LTK types used in internal storage (struct smp_ltk)
}

// Key preferences for smp_sufficient security
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smp_key_pref {
    SMP_ALLOW_STK,
    SMP_USE_LTK,
}

// SMP Commands
extern "C" {
    pub fn smp_conn_security(hcon: *mut hci_conn, sec_level: __u8) -> c_int;
}
extern "C" {
    pub fn smp_generate_rpa(hdev: *mut hci_dev, irk[16]: u8, rpa: *mut bdaddr_t) -> c_int;
}
extern "C" {
    pub fn smp_generate_oob(hdev: *mut hci_dev, hash[16]: u8, rand[16]: u8) -> c_int;
}
extern "C" {
    pub fn smp_force_bredr(hdev: *mut hci_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn smp_register(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn smp_unregister(hdev: *mut hci_dev);
}

extern "C" {
    pub fn bt_selftest_smp() -> c_int;
}

