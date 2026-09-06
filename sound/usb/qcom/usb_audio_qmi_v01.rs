//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/qcom/usb_audio_qmi_v01.h
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
// Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
//
pub const UAUDIO_STREAM_SERVICE_ID_V01: c_uint = 0x41D;
pub const UAUDIO_STREAM_SERVICE_VERS_V01: c_uint = 0x01;
pub const QMI_UAUDIO_STREAM_RESP_V01: c_uint = 0x0001;
pub const QMI_UAUDIO_STREAM_REQ_V01: c_uint = 0x0001;
pub const QMI_UAUDIO_STREAM_IND_V01: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_info_v01 {
    pub /: *mut *mut u64 iova; / mapped into sysdev,
    pub /: *mut *mut u64 dma; / mapped into usb host,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apps_mem_info_v01 {
    pub evt_ring: mem_info_v01,
    pub tr_data: mem_info_v01,
    pub tr_sync: mem_info_v01,
    pub xfer_buff: mem_info_v01,
    pub dcba: mem_info_v01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_endpoint_descriptor_v01 {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
    pub bRefresh: u8,
    pub bSynchAddress: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_interface_descriptor_v01 {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_qmi_audio_stream_status_enum_v01 {
    USB_QMI_STREAM_STATUS_ENUM_MIN_VAL_V01 = INT_MIN,
    USB_QMI_STREAM_REQ_SUCCESS_V01 = 0,
    USB_QMI_STREAM_REQ_FAILURE_V01 = 1,
    USB_QMI_STREAM_REQ_FAILURE_NOT_FOUND_V01 = 2,
    USB_QMI_STREAM_REQ_FAILURE_INVALID_PARAM_V01 = 3,
    USB_QMI_STREAM_REQ_FAILURE_MEMALLOC_V01 = 4,
    USB_QMI_STREAM_STATUS_ENUM_MAX_VAL_V01 = INT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_qmi_audio_device_indication_enum_v01 {
    USB_QMI_DEVICE_INDICATION_ENUM_MIN_VAL_V01 = INT_MIN,
    USB_QMI_DEV_CONNECT_V01 = 0,
    USB_QMI_DEV_DISCONNECT_V01 = 1,
    USB_QMI_DEV_SUSPEND_V01 = 2,
    USB_QMI_DEV_RESUME_V01 = 3,
    USB_QMI_DEVICE_INDICATION_ENUM_MAX_VAL_V01 = INT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_qmi_audio_device_speed_enum_v01 {
    USB_QMI_DEVICE_SPEED_ENUM_MIN_VAL_V01 = INT_MIN,
    USB_QMI_DEVICE_SPEED_INVALID_V01 = 0,
    USB_QMI_DEVICE_SPEED_LOW_V01 = 1,
    USB_QMI_DEVICE_SPEED_FULL_V01 = 2,
    USB_QMI_DEVICE_SPEED_HIGH_V01 = 3,
    USB_QMI_DEVICE_SPEED_SUPER_V01 = 4,
    USB_QMI_DEVICE_SPEED_SUPER_PLUS_V01 = 5,
    USB_QMI_DEVICE_SPEED_ENUM_MAX_VAL_V01 = INT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_uaudio_stream_req_msg_v01 {
    pub enable: u8,
    pub usb_token: u32,
    pub audio_format_valid: u8,
    pub audio_format: u32,
    pub number_of_ch_valid: u8,
    pub number_of_ch: u32,
    pub bit_rate_valid: u8,
    pub bit_rate: u32,
    pub xfer_buff_size_valid: u8,
    pub xfer_buff_size: u32,
    pub service_interval_valid: u8,
    pub service_interval: u32,
}

pub const QMI_UAUDIO_STREAM_REQ_MSG_V01_MAX_MSG_LEN: c_int = 46;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_uaudio_stream_resp_msg_v01 {
    pub resp: qmi_response_type_v01,
    pub status_valid: u8,
    pub status: usb_qmi_audio_stream_status_enum_v01,
    pub internal_status_valid: u8,
    pub internal_status: u32,
    pub slot_id_valid: u8,
    pub slot_id: u32,
    pub usb_token_valid: u8,
    pub usb_token: u32,
    pub std_as_opr_intf_desc_valid: u8,
    pub std_as_opr_intf_desc: usb_interface_descriptor_v01,
    pub std_as_data_ep_desc_valid: u8,
    pub std_as_data_ep_desc: usb_endpoint_descriptor_v01,
    pub std_as_sync_ep_desc_valid: u8,
    pub std_as_sync_ep_desc: usb_endpoint_descriptor_v01,
    pub usb_audio_spec_revision_valid: u8,
    pub usb_audio_spec_revision: u16,
    pub data_path_delay_valid: u8,
    pub data_path_delay: u8,
    pub usb_audio_subslot_size_valid: u8,
    pub usb_audio_subslot_size: u8,
    pub xhci_mem_info_valid: u8,
    pub xhci_mem_info: apps_mem_info_v01,
    pub interrupter_num_valid: u8,
    pub interrupter_num: u8,
    pub speed_info_valid: u8,
    pub speed_info: usb_qmi_audio_device_speed_enum_v01,
    pub controller_num_valid: u8,
    pub controller_num: u8,
}

pub const QMI_UAUDIO_STREAM_RESP_MSG_V01_MAX_MSG_LEN: c_int = 202;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmi_uaudio_stream_ind_msg_v01 {
    pub dev_event: usb_qmi_audio_device_indication_enum_v01,
    pub slot_id: u32,
    pub usb_token_valid: u8,
    pub usb_token: u32,
    pub std_as_opr_intf_desc_valid: u8,
    pub std_as_opr_intf_desc: usb_interface_descriptor_v01,
    pub std_as_data_ep_desc_valid: u8,
    pub std_as_data_ep_desc: usb_endpoint_descriptor_v01,
    pub std_as_sync_ep_desc_valid: u8,
    pub std_as_sync_ep_desc: usb_endpoint_descriptor_v01,
    pub usb_audio_spec_revision_valid: u8,
    pub usb_audio_spec_revision: u16,
    pub data_path_delay_valid: u8,
    pub data_path_delay: u8,
    pub usb_audio_subslot_size_valid: u8,
    pub usb_audio_subslot_size: u8,
    pub xhci_mem_info_valid: u8,
    pub xhci_mem_info: apps_mem_info_v01,
    pub interrupter_num_valid: u8,
    pub interrupter_num: u8,
    pub controller_num_valid: u8,
    pub controller_num: u8,
}

pub const QMI_UAUDIO_STREAM_IND_MSG_V01_MAX_MSG_LEN: c_int = 181;
