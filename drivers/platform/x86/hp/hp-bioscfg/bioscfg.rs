//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/hp/hp-bioscfg/bioscfg.h
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
// Definitions for kernel modules using hp_bioscfg driver
//
// Copyright (c) 2022 HP Development Company, L.P.
//

pub const MAX_BUFF_SIZE: c_int = 512;
pub const MAX_KEY_MOD_SIZE: c_int = 256;
pub const MAX_PASSWD_SIZE: c_int = 64;
pub const MAX_PREREQUISITES_SIZE: c_int = 20;
pub const MAX_REQ_ELEM_SIZE: c_int = 128;
pub const MAX_VALUES_SIZE: c_int = 16;
pub const MAX_ENCODINGS_SIZE: c_int = 16;
pub const MAX_ELEMENTS_SIZE: c_int = 16;

// Sure Admin Functions

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mechanism_values {
    PASSWORD		= 0x00,
    SIGNING_KEY		= 0x01,
    ENDORSEMENT_KEY		= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_wmi_spm_commandtype {
    HPWMI_SECUREPLATFORM_GET_STATE  = 0x10,
    HPWMI_SECUREPLATFORM_SET_KEK	= 0x11,
    HPWMI_SECUREPLATFORM_SET_SK	= 0x12,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_wmi_surestart_commandtype {
    HPWMI_SURESTART_GET_LOG_COUNT	= 0x01,
    HPWMI_SURESTART_GET_LOG		= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_wmi_command {
    HPWMI_READ		= 0x01,
    HPWMI_WRITE		= 0x02,
    HPWMI_ODM		= 0x03,
    HPWMI_SURESTART		= 0x20006,
    HPWMI_GM		= 0x20008,
    HPWMI_SECUREPLATFORM	= 0x20010,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bios_return {
    pub sigpass: u32,
    pub return_code: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_error_values {
    SUCCESS				= 0x00,
    CMD_FAILED			= 0x01,
    INVALID_SIGN			= 0x02,
    INVALID_CMD_VALUE		= 0x03,
    INVALID_CMD_TYPE		= 0x04,
    INVALID_DATA_SIZE		= 0x05,
    INVALID_CMD_PARAM		= 0x06,
    ENCRYP_CMD_REQUIRED		= 0x07,
    NO_SECURE_SESSION		= 0x08,
    SECURE_SESSION_FOUND		= 0x09,
    SECURE_SESSION_FAILED		= 0x0A,
    AUTH_FAILED			= 0x0B,
    INVALID_BIOS_AUTH		= 0x0E,
    NONCE_DID_NOT_MATCH		= 0x18,
    GENERIC_ERROR			= 0x1C,
    BIOS_ADMIN_POLICY_NOT_MET	= 0x28,
    BIOS_ADMIN_NOT_SET		= 0x38,
    P21_NO_PROVISIONED		= 0x1000,
    P21_PROVISION_IN_PROGRESS	= 0x1001,
    P21_IN_USE			= 0x1002,
    HEP_NOT_ACTIVE			= 0x1004,
    HEP_ALREADY_SET			= 0x1006,
    HEP_CHECK_STATE			= 0x1007,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_data {
    pub display_name: [u8; MAX_BUFF_SIZE],
    pub path: [u8; MAX_BUFF_SIZE],
    pub is_readonly: u32,
    pub display_in_ui: u32,
    pub requires_physical_presence: u32,
    pub sequence: u32,
    pub prerequisites_size: u32,
    pub prerequisites: [u8; MAX_PREREQUISITES_SIZE][MAX_BUFF_SIZE],
    pub security_level: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct string_data {
    pub common: common_data,
    pub attr_name_kobj: *mut kobject,
    pub current_value: [u8; MAX_BUFF_SIZE],
    pub new_value: [u8; MAX_BUFF_SIZE],
    pub min_length: u32,
    pub max_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct integer_data {
    pub common: common_data,
    pub attr_name_kobj: *mut kobject,
    pub current_value: u32,
    pub new_value: u32,
    pub lower_bound: u32,
    pub upper_bound: u32,
    pub scalar_increment: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enumeration_data {
    pub common: common_data,
    pub attr_name_kobj: *mut kobject,
    pub current_value: [u8; MAX_BUFF_SIZE],
    pub new_value: [u8; MAX_BUFF_SIZE],
    pub possible_values_size: u32,
    pub possible_values: [u8; MAX_VALUES_SIZE][MAX_BUFF_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ordered_list_data {
    pub common: common_data,
    pub attr_name_kobj: *mut kobject,
    pub current_value: [u8; MAX_BUFF_SIZE],
    pub new_value: [u8; MAX_BUFF_SIZE],
    pub elements_size: u32,
    pub elements: [u8; MAX_ELEMENTS_SIZE][MAX_BUFF_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct password_data {
    pub common: common_data,
    pub attr_name_kobj: *mut kobject,
    pub current_password: [u8; MAX_PASSWD_SIZE],
    pub new_password: [u8; MAX_PASSWD_SIZE],
    pub min_password_length: u32,
    pub max_password_length: u32,
    pub encodings_size: u32,
    pub encodings: [u8; MAX_ENCODINGS_SIZE][MAX_BUFF_SIZE],
    pub is_enabled: bool,
//
// 'role' identifies the type of authentication.
// Two known types are bios-admin and power-on.
// 'bios-admin' represents BIOS administrator password
// 'power-on' represents a password required to use the system
//
    pub role: u32,
//
// 'mechanism' represents the means of authentication.
// Only supported type currently is "password"
//
    pub mechanism: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct secure_platform_data {
    pub attr_name_kobj: *mut kobject,
    pub attribute_name: [u8; MAX_BUFF_SIZE],
    pub endorsement_key: *mut u8,
    pub signing_key: *mut u8,
    pub auth_token: *mut u8,
    pub is_enabled: bool,
    pub mechanism: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bioscfg_priv {
    pub authentication_dir_kset: *mut kset,
    pub main_dir_kset: *mut kset,
    pub class_dev: *mut device,
    pub string_data: *mut string_data,
    pub string_instances_count: u32,
    pub integer_data: *mut integer_data,
    pub integer_instances_count: u32,
    pub enumeration_data: *mut enumeration_data,
    pub enumeration_instances_count: u32,
    pub ordered_list_data: *mut ordered_list_data,
    pub ordered_list_instances_count: u32,
    pub password_data: *mut password_data,
    pub password_instances_count: u32,
    pub sure_start_attr_kobj: *mut kobject,
    pub spm_data: secure_platform_data,
    pub display_name_language_code: [u8; MAX_BUFF_SIZE],
    pub pending_reboot: bool,
    pub mutex: mutex,
}

// global structure used by multiple WMI interfaces
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_wmi_data_type {
    HPWMI_STRING_TYPE,
    HPWMI_INTEGER_TYPE,
    HPWMI_ENUMERATION_TYPE,
    HPWMI_ORDERED_LIST_TYPE,
    HPWMI_PASSWORD_TYPE,
    HPWMI_SECURE_PLATFORM_TYPE,
    HPWMI_SURE_START_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_wmi_data_elements {
// Common elements
    NAME = 0,
    VALUE = 1,
    PATH = 2,
    IS_READONLY = 3,
    DISPLAY_IN_UI = 4,
    REQUIRES_PHYSICAL_PRESENCE = 5,
    SEQUENCE = 6,
    PREREQUISITES_SIZE = 7,
    PREREQUISITES = 8,
    SECURITY_LEVEL = 9,

// String elements
    STR_MIN_LENGTH = 10,
    STR_MAX_LENGTH = 11,
    STR_ELEM_CNT = 12,

// Integer elements
    INT_LOWER_BOUND = 10,
    INT_UPPER_BOUND = 11,
    INT_SCALAR_INCREMENT = 12,
    INT_ELEM_CNT = 13,

// Enumeration elements
    ENUM_CURRENT_VALUE = 10,
    ENUM_SIZE = 11,
    ENUM_POSSIBLE_VALUES = 12,
    ENUM_ELEM_CNT = 13,

// Ordered list elements
    ORD_LIST_SIZE = 10,
    ORD_LIST_ELEMENTS = 11,
    ORD_ELEM_CNT = 12,

// Password elements
    PSWD_MIN_LENGTH = 10,
    PSWD_MAX_LENGTH = 11,
    PSWD_SIZE = 12,
    PSWD_ENCODINGS = 13,
    PSWD_IS_SET = 14,
    PSWD_ELEM_CNT = 15,

// Minimum elements shared by all attribute types (NAME..SECURITY_LEVEL)
    COMMON_ELEM_CNT = SECURITY_LEVEL + 1,
}

// Prototypes
// String attributes
extern "C" {
    pub fn hp_alloc_string_data() -> c_int;
}
extern "C" {
    pub fn hp_exit_string_attributes();
}
// Integer attributes
extern "C" {
    pub fn hp_alloc_integer_data() -> c_int;
}
extern "C" {
    pub fn hp_exit_integer_attributes();
}
// Enumeration attributes
extern "C" {
    pub fn hp_alloc_enumeration_data() -> c_int;
}
extern "C" {
    pub fn hp_exit_enumeration_attributes();
}
// Ordered list
extern "C" {
    pub fn hp_alloc_ordered_list_data() -> c_int;
}
extern "C" {
    pub fn hp_exit_ordered_list_attributes();
}
// Password authentication attributes
extern "C" {
    pub fn hp_alloc_password_data() -> c_int;
}
extern "C" {
    pub fn hp_get_password_instance_for_type(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn hp_clear_all_credentials() -> c_int;
}
extern "C" {
    pub fn hp_set_attribute(a_name: *const c_char, a_value: *const c_char) -> c_int;
}
// SPM attributes
extern "C" {
    pub fn hp_exit_password_attributes();
}
extern "C" {
    pub fn hp_exit_secure_platform_attributes();
}
extern "C" {
    pub fn hp_populate_secure_platform_data(attr_name_kobj: *mut kobject) -> c_int;
}
extern "C" {
    pub fn hp_populate_security_buffer(buffer: *mut u16, authentication: *const c_char) -> c_int;
}
// Bios Attributes interface
extern "C" {
    pub fn hp_wmi_set_bios_setting(input_buffer: *mut u16, input_size: u32) -> c_int;
}
// Sure Start attributes
extern "C" {
    pub fn hp_exit_sure_start_attributes();
}
extern "C" {
    pub fn hp_populate_sure_start_data(attr_name_kobj: *mut kobject) -> c_int;
}
// Bioscfg
extern "C" {
    pub fn hp_exit_attr_set_interface();
}
extern "C" {
    pub fn hp_init_attr_set_interface() -> c_int;
}
extern "C" {
    pub fn hp_calculate_string_buffer(str: *const c_char) -> usize;
}
extern "C" {
    pub fn hp_calculate_security_buffer(authentication: *const c_char) -> usize;
}
extern "C" {
    pub fn hp_get_integer_from_buffer(buffer: *mut u8, buffer_size: *mut u32, integer: *mut u32) -> c_int;
}
extern "C" {
    pub fn hp_get_string_from_buffer(buffer: *mut u8, buffer_size: *mut u32, dst: *mut c_char, dst_size: u32) -> c_int;
}
extern "C" {
    pub fn hp_convert_hexstr_to_str(input: *const c_char, input_len: u32, str: *mut c_char, len: *mut c_int) -> c_int;
}
extern "C" {
    pub fn hp_encode_outsize_for_pvsz(outsize: c_int) -> c_int;
}
extern "C" {
    pub fn hp_enforce_single_line_input(buf: *mut c_char, count: usize) -> c_int;
}
extern "C" {
    pub fn hp_set_reboot_and_signal_event();
}
extern "C" {
    pub fn hp_get_instance_count(guid_string: *const c_char) -> c_int;
}
extern "C" {
    pub fn hp_update_attribute_permissions(isreadonly: bool, current_val: *mut kobj_attribute);
}
extern "C" {
    pub fn hp_wmi_error_and_message(error_code: c_int) -> c_int;
}
extern "C" {
    pub fn hp_get_common_data_from_buffer(buffer_ptr: *mut u8, buffer_size: *mut u32, common: *mut common_data) -> c_int;
}
