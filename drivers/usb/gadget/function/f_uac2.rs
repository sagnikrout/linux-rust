//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/function/f_uac2.c
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
// f_uac2.c -- USB Audio Class 2.0 Function
//
// Copyright (C) 2011
// Yadwinder Singh (yadi.brar01@gmail.com)
// Jaswinder Singh (jaswinder.singh@linaro.org)
//
// Copyright (C) 2020
// Ruslan Bilovol (ruslan.bilovol@gmail.com)
//

// UAC2 spec: 4.1 Audio Channel Cluster Descriptor
pub const UAC2_CHANNEL_MASK: c_uint = 0x07FFFFFF;
//
// The driver implements a simple UAC_2 topology.
// USB-OUT -> IT_1 -> FU -> OT_3 -> ALSA_Capture
// ALSA_Playback -> IT_2 -> FU -> OT_4 -> USB-IN
// Capture and Playback sampling rates are independently
// controlled by two clock sources :
// CLK_5 := c_srate, and CLK_6 := p_srate
//

pub const CONTROL_ABSENT: c_int = 0;
pub const CONTROL_RDONLY: c_int = 1;
pub const CONTROL_RDWR: c_int = 3;
pub const CLK_FREQ_CTRL: c_int = 0;
pub const CLK_VLD_CTRL: c_int = 2;
pub const FU_MUTE_CTRL: c_int = 0;
pub const FU_VOL_CTRL: c_int = 2;
pub const COPY_CTRL: c_int = 0;
pub const CONN_CTRL: c_int = 2;
pub const OVRLD_CTRL: c_int = 4;
pub const CLSTR_CTRL: c_int = 6;
pub const UNFLW_CTRL: c_int = 8;
pub const OVFLW_CTRL: c_int = 10;

    && ((_opts).p_mute_present \
    || (_opts).p_volume_present))

    && ((_opts).c_mute_present \
    || (_opts).c_volume_present))

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_uac2 {
    pub g_audio: g_audio,
    pub as_out_intf: u8 ac_intf, as_in_intf,,
    pub /: *mut *mut u8 ac_alt, as_in_alt, as_out_alt; / needed for get_alt(),
    pub /: *mut *mut usb_ctrlrequest setup_cr; / will be used in data stage,
// Interrupt IN endpoint of AC interface
    pub int_ep: *mut usb_ep,
    pub int_count: core::sync::atomic::AtomicI32,
// transient state, only valid during handling of a single control request
    pub clock_id: c_int,
}

    static inline struct f_uac2 *func_to_uac2(struct usb_function *f)
    {
    return container_of(f, struct f_uac2, g_audio.func);
    }
    static inline
    struct f_uac2_opts *g_audio_to_uac2_opts(struct g_audio *agdev)
    {
    return container_of(agdev.func.fi, struct f_uac2_opts, func_inst);
    }
    static int afunc_notify(struct g_audio *agdev, int unit_id, int cs);
// --------- USB Function Interface -------------
    enum {
    STR_ASSOC,
    STR_IF_CTRL,
    STR_CLKSRC_IN,
    STR_CLKSRC_OUT,
    STR_USB_IT,
    STR_USB_IT_CH,
    STR_IO_IT,
    STR_IO_IT_CH,
    STR_USB_OT,
    STR_IO_OT,
    STR_FU_IN,
    STR_FU_OUT,
    STR_AS_OUT_ALT0,
    STR_AS_OUT_ALT1,
    STR_AS_IN_ALT0,
    STR_AS_IN_ALT1,
    NUM_STR_DESCRIPTORS,
    };
    static struct usb_string strings_fn[NUM_STR_DESCRIPTORS + 1] = {};
    static const char *const speed_names[] = {
    [USB_SPEED_UNKNOWN] = "UNKNOWN",
    [USB_SPEED_LOW] = "LS",
    [USB_SPEED_FULL] = "FS",
    [USB_SPEED_HIGH] = "HS",
    [USB_SPEED_WIRELESS] = "W",
    [USB_SPEED_SUPER] = "SS",
    [USB_SPEED_SUPER_PLUS] = "SS+",
    };
    static struct usb_gadget_strings str_fn = {
    .language = 0x0409,	/* en-us */
    .strings = strings_fn,
    };
    static struct usb_gadget_strings *fn_strings[] = {
    &str_fn,
    core::ptr::null_mut(),
    };
    static struct usb_interface_assoc_descriptor iad_desc = {
    .bLength = sizeof iad_desc,
    .bDescriptorType = USB_DT_INTERFACE_ASSOCIATION,
    .bFirstInterface = 0,
    .bInterfaceCount = 3,
    .bFunctionClass = USB_CLASS_AUDIO,
    .bFunctionSubClass = UAC2_FUNCTION_SUBCLASS_UNDEFINED,
    .bFunctionProtocol = UAC_VERSION_2,
    };
// Audio Control Interface
    static struct usb_interface_descriptor std_ac_if_desc = {
    .bLength = sizeof std_ac_if_desc,
    .bDescriptorType = USB_DT_INTERFACE,
    .bAlternateSetting = 0,
// .bNumEndpoints = DYNAMIC
    .bInterfaceClass = USB_CLASS_AUDIO,
    .bInterfaceSubClass = USB_SUBCLASS_AUDIOCONTROL,
    .bInterfaceProtocol = UAC_VERSION_2,
    };
// Clock source for IN traffic
    static struct uac_clock_source_descriptor in_clk_src_desc = {
    .bLength = sizeof in_clk_src_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC2_CLOCK_SOURCE,
// .bClockID = DYNAMIC
    .bmAttributes = UAC_CLOCK_SOURCE_TYPE_INT_FIXED,
    .bmControls = (CONTROL_RDWR << CLK_FREQ_CTRL),
    .bAssocTerminal = 0,
    };
// Clock source for OUT traffic
    static struct uac_clock_source_descriptor out_clk_src_desc = {
    .bLength = sizeof out_clk_src_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC2_CLOCK_SOURCE,
// .bClockID = DYNAMIC
    .bmAttributes = UAC_CLOCK_SOURCE_TYPE_INT_FIXED,
    .bmControls = (CONTROL_RDWR << CLK_FREQ_CTRL),
    .bAssocTerminal = 0,
    };
// Input Terminal for USB_OUT
    static struct uac2_input_terminal_descriptor usb_out_it_desc = {
    .bLength = sizeof usb_out_it_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC_INPUT_TERMINAL,
// .bTerminalID = DYNAMIC
    .wTerminalType = cpu_to_le16(UAC_TERMINAL_STREAMING),
    .bAssocTerminal = 0,
// .bCSourceID = DYNAMIC
    .iChannelNames = 0,
    .bmControls = cpu_to_le16(CONTROL_RDWR << COPY_CTRL),
    };
// Input Terminal for I/O-In
    static struct uac2_input_terminal_descriptor io_in_it_desc = {
    .bLength = sizeof io_in_it_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC_INPUT_TERMINAL,
// .bTerminalID = DYNAMIC
// .wTerminalType = DYNAMIC
    .bAssocTerminal = 0,
// .bCSourceID = DYNAMIC
    .iChannelNames = 0,
    .bmControls = cpu_to_le16(CONTROL_RDWR << COPY_CTRL),
    };
// Ouput Terminal for USB_IN
    static struct uac2_output_terminal_descriptor usb_in_ot_desc = {
    .bLength = sizeof usb_in_ot_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC_OUTPUT_TERMINAL,
// .bTerminalID = DYNAMIC
    .wTerminalType = cpu_to_le16(UAC_TERMINAL_STREAMING),
    .bAssocTerminal = 0,
// .bSourceID = DYNAMIC
// .bCSourceID = DYNAMIC
    .bmControls = cpu_to_le16(CONTROL_RDWR << COPY_CTRL),
    };
// Ouput Terminal for I/O-Out
    static struct uac2_output_terminal_descriptor io_out_ot_desc = {
    .bLength = sizeof io_out_ot_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC_OUTPUT_TERMINAL,
// .bTerminalID = DYNAMIC
// .wTerminalType = DYNAMIC
    .bAssocTerminal = 0,
// .bSourceID = DYNAMIC
// .bCSourceID = DYNAMIC
    .bmControls = cpu_to_le16(CONTROL_RDWR << COPY_CTRL),
    };
    static struct uac2_feature_unit_descriptor *in_feature_unit_desc;
    static struct uac2_feature_unit_descriptor *out_feature_unit_desc;
    static struct uac2_ac_header_descriptor ac_hdr_desc = {
    .bLength = sizeof ac_hdr_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC_MS_HEADER,
    .bcdADC = cpu_to_le16(0x200),
    .bCategory = UAC2_FUNCTION_IO_BOX,
// .wTotalLength = DYNAMIC
    .bmControls = 0,
    };
// AC IN Interrupt Endpoint
    static struct usb_endpoint_descriptor fs_ep_int_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bEndpointAddress = USB_DIR_IN,
    .bmAttributes = USB_ENDPOINT_XFER_INT,
    .wMaxPacketSize = cpu_to_le16(6),
    .bInterval = 1,
    };
    static struct usb_endpoint_descriptor hs_ep_int_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bmAttributes = USB_ENDPOINT_XFER_INT,
    .wMaxPacketSize = cpu_to_le16(6),
    .bInterval = 4,
    };
    static struct usb_endpoint_descriptor ss_ep_int_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bEndpointAddress = USB_DIR_IN,
    .bmAttributes = USB_ENDPOINT_XFER_INT,
    .wMaxPacketSize = cpu_to_le16(6),
    .bInterval = 4,
    };
    static struct usb_ss_ep_comp_descriptor ss_ep_int_desc_comp = {
    .bLength = sizeof(ss_ep_int_desc_comp),
    .bDescriptorType = USB_DT_SS_ENDPOINT_COMP,
    .wBytesPerInterval = cpu_to_le16(6),
    };
// Audio Streaming OUT Interface - Alt0
    static struct usb_interface_descriptor std_as_out_if0_desc = {
    .bLength = sizeof std_as_out_if0_desc,
    .bDescriptorType = USB_DT_INTERFACE,
    .bAlternateSetting = 0,
    .bNumEndpoints = 0,
    .bInterfaceClass = USB_CLASS_AUDIO,
    .bInterfaceSubClass = USB_SUBCLASS_AUDIOSTREAMING,
    .bInterfaceProtocol = UAC_VERSION_2,
    };
// Audio Streaming OUT Interface - Alt1
    static struct usb_interface_descriptor std_as_out_if1_desc = {
    .bLength = sizeof std_as_out_if1_desc,
    .bDescriptorType = USB_DT_INTERFACE,
    .bAlternateSetting = 1,
    .bNumEndpoints = 1,
    .bInterfaceClass = USB_CLASS_AUDIO,
    .bInterfaceSubClass = USB_SUBCLASS_AUDIOSTREAMING,
    .bInterfaceProtocol = UAC_VERSION_2,
    };
// Audio Stream OUT Intface Desc
    static struct uac2_as_header_descriptor as_out_hdr_desc = {
    .bLength = sizeof as_out_hdr_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC_AS_GENERAL,
// .bTerminalLink = DYNAMIC
    .bmControls = 0,
    .bFormatType = UAC_FORMAT_TYPE_I,
    .bmFormats = cpu_to_le32(UAC_FORMAT_TYPE_I_PCM),
    .iChannelNames = 0,
    };
// Audio USB_OUT Format
    static struct uac2_format_type_i_descriptor as_out_fmt1_desc = {
    .bLength = sizeof as_out_fmt1_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC_FORMAT_TYPE,
    .bFormatType = UAC_FORMAT_TYPE_I,
    };
// STD AS ISO OUT Endpoint
    static struct usb_endpoint_descriptor fs_epout_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bEndpointAddress = USB_DIR_OUT,
// .bmAttributes = DYNAMIC
// .wMaxPacketSize = DYNAMIC
    .bInterval = 1,
    };
    static struct usb_endpoint_descriptor hs_epout_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
// .bmAttributes = DYNAMIC
// .wMaxPacketSize = DYNAMIC
// .bInterval = DYNAMIC
    };
    static struct usb_endpoint_descriptor ss_epout_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bEndpointAddress = USB_DIR_OUT,
// .bmAttributes = DYNAMIC
// .wMaxPacketSize = DYNAMIC
// .bInterval = DYNAMIC
    };
    static struct usb_ss_ep_comp_descriptor ss_epout_desc_comp = {
    .bLength		= sizeof(ss_epout_desc_comp),
    .bDescriptorType	= USB_DT_SS_ENDPOINT_COMP,
    .bMaxBurst		= 0,
    .bmAttributes		= 0,
// wBytesPerInterval = DYNAMIC
    };
// CS AS ISO OUT Endpoint
    static struct uac2_iso_endpoint_descriptor as_iso_out_desc = {
    .bLength = sizeof as_iso_out_desc,
    .bDescriptorType = USB_DT_CS_ENDPOINT,
    .bDescriptorSubtype = UAC_EP_GENERAL,
    .bmAttributes = 0,
    .bmControls = 0,
    .bLockDelayUnits = 0,
    .wLockDelay = 0,
    };
// STD AS ISO IN Feedback Endpoint
    static struct usb_endpoint_descriptor fs_epin_fback_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bEndpointAddress = USB_DIR_IN,
    .bmAttributes = USB_ENDPOINT_XFER_ISOC | USB_ENDPOINT_USAGE_FEEDBACK,
    .wMaxPacketSize = cpu_to_le16(3),
    .bInterval = 1,
    };
    static struct usb_endpoint_descriptor hs_epin_fback_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bmAttributes = USB_ENDPOINT_XFER_ISOC | USB_ENDPOINT_USAGE_FEEDBACK,
    .wMaxPacketSize = cpu_to_le16(4),
    .bInterval = 4,
    };
    static struct usb_endpoint_descriptor ss_epin_fback_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bEndpointAddress = USB_DIR_IN,
    .bmAttributes = USB_ENDPOINT_XFER_ISOC | USB_ENDPOINT_USAGE_FEEDBACK,
    .wMaxPacketSize = cpu_to_le16(4),
    .bInterval = 4,
    };
    static struct usb_ss_ep_comp_descriptor ss_epin_fback_desc_comp = {
    .bLength		= sizeof(ss_epin_fback_desc_comp),
    .bDescriptorType	= USB_DT_SS_ENDPOINT_COMP,
    .bMaxBurst		= 0,
    .bmAttributes		= 0,
    .wBytesPerInterval	= cpu_to_le16(4),
    };
// Audio Streaming IN Interface - Alt0
    static struct usb_interface_descriptor std_as_in_if0_desc = {
    .bLength = sizeof std_as_in_if0_desc,
    .bDescriptorType = USB_DT_INTERFACE,
    .bAlternateSetting = 0,
    .bNumEndpoints = 0,
    .bInterfaceClass = USB_CLASS_AUDIO,
    .bInterfaceSubClass = USB_SUBCLASS_AUDIOSTREAMING,
    .bInterfaceProtocol = UAC_VERSION_2,
    };
// Audio Streaming IN Interface - Alt1
    static struct usb_interface_descriptor std_as_in_if1_desc = {
    .bLength = sizeof std_as_in_if1_desc,
    .bDescriptorType = USB_DT_INTERFACE,
    .bAlternateSetting = 1,
    .bNumEndpoints = 1,
    .bInterfaceClass = USB_CLASS_AUDIO,
    .bInterfaceSubClass = USB_SUBCLASS_AUDIOSTREAMING,
    .bInterfaceProtocol = UAC_VERSION_2,
    };
// Audio Stream IN Intface Desc
    static struct uac2_as_header_descriptor as_in_hdr_desc = {
    .bLength = sizeof as_in_hdr_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC_AS_GENERAL,
// .bTerminalLink = DYNAMIC
    .bmControls = 0,
    .bFormatType = UAC_FORMAT_TYPE_I,
    .bmFormats = cpu_to_le32(UAC_FORMAT_TYPE_I_PCM),
    .iChannelNames = 0,
    };
// Audio USB_IN Format
    static struct uac2_format_type_i_descriptor as_in_fmt1_desc = {
    .bLength = sizeof as_in_fmt1_desc,
    .bDescriptorType = USB_DT_CS_INTERFACE,
    .bDescriptorSubtype = UAC_FORMAT_TYPE,
    .bFormatType = UAC_FORMAT_TYPE_I,
    };
// STD AS ISO IN Endpoint
    static struct usb_endpoint_descriptor fs_epin_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bEndpointAddress = USB_DIR_IN,
    .bmAttributes = USB_ENDPOINT_XFER_ISOC | USB_ENDPOINT_SYNC_ASYNC,
// .wMaxPacketSize = DYNAMIC
    .bInterval = 1,
    };
    static struct usb_endpoint_descriptor hs_epin_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bmAttributes = USB_ENDPOINT_XFER_ISOC | USB_ENDPOINT_SYNC_ASYNC,
// .wMaxPacketSize = DYNAMIC
// .bInterval = DYNAMIC
    };
    static struct usb_endpoint_descriptor ss_epin_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bEndpointAddress = USB_DIR_IN,
    .bmAttributes = USB_ENDPOINT_XFER_ISOC | USB_ENDPOINT_SYNC_ASYNC,
// .wMaxPacketSize = DYNAMIC
// .bInterval = DYNAMIC
    };
    static struct usb_ss_ep_comp_descriptor ss_epin_desc_comp = {
    .bLength		= sizeof(ss_epin_desc_comp),
    .bDescriptorType	= USB_DT_SS_ENDPOINT_COMP,
    .bMaxBurst		= 0,
    .bmAttributes		= 0,
// wBytesPerInterval = DYNAMIC
    };
// CS AS ISO IN Endpoint
    static struct uac2_iso_endpoint_descriptor as_iso_in_desc = {
    .bLength = sizeof as_iso_in_desc,
    .bDescriptorType = USB_DT_CS_ENDPOINT,
    .bDescriptorSubtype = UAC_EP_GENERAL,
    .bmAttributes = 0,
    .bmControls = 0,
    .bLockDelayUnits = 0,
    .wLockDelay = 0,
    };
    static struct usb_descriptor_header *fs_audio_desc[] = {
    (struct usb_descriptor_header *)&iad_desc,
    (struct usb_descriptor_header *)&std_ac_if_desc,
    (struct usb_descriptor_header *)&ac_hdr_desc,
    (struct usb_descriptor_header *)&in_clk_src_desc,
    (struct usb_descriptor_header *)&out_clk_src_desc,
    (struct usb_descriptor_header *)&usb_out_it_desc,
    (struct usb_descriptor_header *)&out_feature_unit_desc,
    (struct usb_descriptor_header *)&io_in_it_desc,
    (struct usb_descriptor_header *)&usb_in_ot_desc,
    (struct usb_descriptor_header *)&in_feature_unit_desc,
    (struct usb_descriptor_header *)&io_out_ot_desc,
    (struct usb_descriptor_header *)&fs_ep_int_desc,
    (struct usb_descriptor_header *)&std_as_out_if0_desc,
    (struct usb_descriptor_header *)&std_as_out_if1_desc,
    (struct usb_descriptor_header *)&as_out_hdr_desc,
    (struct usb_descriptor_header *)&as_out_fmt1_desc,
    (struct usb_descriptor_header *)&fs_epout_desc,
    (struct usb_descriptor_header *)&as_iso_out_desc,
    (struct usb_descriptor_header *)&fs_epin_fback_desc,
    (struct usb_descriptor_header *)&std_as_in_if0_desc,
    (struct usb_descriptor_header *)&std_as_in_if1_desc,
    (struct usb_descriptor_header *)&as_in_hdr_desc,
    (struct usb_descriptor_header *)&as_in_fmt1_desc,
    (struct usb_descriptor_header *)&fs_epin_desc,
    (struct usb_descriptor_header *)&as_iso_in_desc,
    core::ptr::null_mut(),
    };
    static struct usb_descriptor_header *hs_audio_desc[] = {
    (struct usb_descriptor_header *)&iad_desc,
    (struct usb_descriptor_header *)&std_ac_if_desc,
    (struct usb_descriptor_header *)&ac_hdr_desc,
    (struct usb_descriptor_header *)&in_clk_src_desc,
    (struct usb_descriptor_header *)&out_clk_src_desc,
    (struct usb_descriptor_header *)&usb_out_it_desc,
    (struct usb_descriptor_header *)&out_feature_unit_desc,
    (struct usb_descriptor_header *)&io_in_it_desc,
    (struct usb_descriptor_header *)&usb_in_ot_desc,
    (struct usb_descriptor_header *)&in_feature_unit_desc,
    (struct usb_descriptor_header *)&io_out_ot_desc,
    (struct usb_descriptor_header *)&hs_ep_int_desc,
    (struct usb_descriptor_header *)&std_as_out_if0_desc,
    (struct usb_descriptor_header *)&std_as_out_if1_desc,
    (struct usb_descriptor_header *)&as_out_hdr_desc,
    (struct usb_descriptor_header *)&as_out_fmt1_desc,
    (struct usb_descriptor_header *)&hs_epout_desc,
    (struct usb_descriptor_header *)&as_iso_out_desc,
    (struct usb_descriptor_header *)&hs_epin_fback_desc,
    (struct usb_descriptor_header *)&std_as_in_if0_desc,
    (struct usb_descriptor_header *)&std_as_in_if1_desc,
    (struct usb_descriptor_header *)&as_in_hdr_desc,
    (struct usb_descriptor_header *)&as_in_fmt1_desc,
    (struct usb_descriptor_header *)&hs_epin_desc,
    (struct usb_descriptor_header *)&as_iso_in_desc,
    core::ptr::null_mut(),
    };
    static struct usb_descriptor_header *ss_audio_desc[] = {
    (struct usb_descriptor_header *)&iad_desc,
    (struct usb_descriptor_header *)&std_ac_if_desc,
    (struct usb_descriptor_header *)&ac_hdr_desc,
    (struct usb_descriptor_header *)&in_clk_src_desc,
    (struct usb_descriptor_header *)&out_clk_src_desc,
    (struct usb_descriptor_header *)&usb_out_it_desc,
    (struct usb_descriptor_header *)&out_feature_unit_desc,
    (struct usb_descriptor_header *)&io_in_it_desc,
    (struct usb_descriptor_header *)&usb_in_ot_desc,
    (struct usb_descriptor_header *)&in_feature_unit_desc,
    (struct usb_descriptor_header *)&io_out_ot_desc,
    (struct usb_descriptor_header *)&ss_ep_int_desc,
    (struct usb_descriptor_header *)&ss_ep_int_desc_comp,
    (struct usb_descriptor_header *)&std_as_out_if0_desc,
    (struct usb_descriptor_header *)&std_as_out_if1_desc,
    (struct usb_descriptor_header *)&as_out_hdr_desc,
    (struct usb_descriptor_header *)&as_out_fmt1_desc,
    (struct usb_descriptor_header *)&ss_epout_desc,
    (struct usb_descriptor_header *)&ss_epout_desc_comp,
    (struct usb_descriptor_header *)&as_iso_out_desc,
    (struct usb_descriptor_header *)&ss_epin_fback_desc,
    (struct usb_descriptor_header *)&ss_epin_fback_desc_comp,
    (struct usb_descriptor_header *)&std_as_in_if0_desc,
    (struct usb_descriptor_header *)&std_as_in_if1_desc,
    (struct usb_descriptor_header *)&as_in_hdr_desc,
    (struct usb_descriptor_header *)&as_in_fmt1_desc,
    (struct usb_descriptor_header *)&ss_epin_desc,
    (struct usb_descriptor_header *)&ss_epin_desc_comp,
    (struct usb_descriptor_header *)&as_iso_in_desc,
    core::ptr::null_mut(),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cntrl_cur_lay2 {
    pub wCUR: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cntrl_range_lay2 {
    pub wNumSubRanges: __le16,
    pub wMIN: __le16,
    pub wMAX: __le16,
    pub wRES: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cntrl_cur_lay3 {
    pub dCUR: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cntrl_subrange_lay3 {
    pub dMIN: __le32,
    pub dMAX: __le32,
    pub dRES: __le32,
    pub __packed: },

    + le16_to_cpu(c.wNumSubRanges)		\
// sizeof(struct cntrl_subrange_lay3))

    struct cntrl_ranges_lay3_##k {			\
    pub \: __le16 wNumSubRanges;,
    pub \: cntrl_subrange_lay3 r[n];,
    } __packed
    pub UAC_MAX_RATES): DECLARE_UAC2_CNTRL_RANGES_LAY3(srates,,
#[no_mangle]
unsafe extern "C" fn get_max_srate(srates: *const c_int) -> c_int {
    static int get_max_srate(const int *srates)
    {
    pub 0: int i, max_srate =,
    pub {: for (i = 0; i < UAC_MAX_RATES; i++),
    if (srates[i] == 0)
    if (srates[i] > max_srate)
    pub srates: [max_srate =; i],
    }
    pub max_srate: return,
    }
    static int get_max_bw_for_bint(const struct f_uac2_opts *uac2_opts,
    u8 bint, unsigned int factor, bool is_playback)
    {
    pub ssize: int chmask, srate,,
    pub max_size_bw: u16,
    if (is_playback) {
    pub uac2_opts->p_chmask: chmask =,
    pub get_max_srate(uac2_opts->p_srates): srate =,
    pub uac2_opts->p_ssize: ssize =,
    } else {
    pub uac2_opts->c_chmask: chmask =,
    pub get_max_srate(uac2_opts->c_srates): srate =,
    pub uac2_opts->c_ssize: ssize =,
    }
    if (is_playback || (uac2_opts.c_sync == USB_ENDPOINT_SYNC_ASYNC)) {
// playback is always async, capture only when configured
// Win10 requires max packet size + 1 frame
    pub 1000: *mut *mut srate = srate  (1000 + uac2_opts->fb_max) /,
// updated srate is always bigger, therefore DIV_ROUND_UP always yields +1
    max_size_bw = num_channels(chmask) * ssize *
    pub 1)))): (DIV_ROUND_UP(srate, factor / (1 << (bint -,
    } else {
// adding 1 frame provision for Win10
    max_size_bw = num_channels(chmask) * ssize *
    pub 1): (DIV_ROUND_UP(srate, factor / (1 << (bint - 1))) +,
    }
    pub max_size_bw: return,
    }
    static int set_ep_max_packet_size_bint(struct device *dev, const struct f_uac2_opts *uac2_opts,
    struct usb_endpoint_descriptor *ep_desc,
    enum usb_device_speed speed, bool is_playback)
    {
    pub max_size_ep: u16 max_size_bw,,
    pub opts_bint: u8 bint,,
    pub dir: *mut c_char,
    switch (speed) {
    case USB_SPEED_FULL:
    pub 1023: max_size_ep =,
// fixed
    pub ep_desc->bInterval: bint =,
    pub is_playback): max_size_bw = get_max_bw_for_bint(uac2_opts, bint, 1000,,
    case USB_SPEED_HIGH:
    case USB_SPEED_SUPER:
    pub 1024: max_size_ep =,
    if (is_playback)
    pub uac2_opts->p_hs_bint: opts_bint =,
    else
    pub uac2_opts->c_hs_bint: opts_bint =,
    if (opts_bint > 0) {
// fixed bint
    pub opts_bint: bint =,
    pub is_playback): max_size_bw = get_max_bw_for_bint(uac2_opts, bint, 8000,,
    } else {
// checking bInterval from 4 to 1 whether the required bandwidth fits
    pub {: for (bint = 4; bint > 0; --bint),
    max_size_bw = get_max_bw_for_bint(
    pub is_playback): uac2_opts, bint, 8000,,
    if (max_size_bw <= max_size_ep)
    }
    }
    default:
    pub -EINVAL: return,
    }
    if (is_playback)
    pub "Playback": dir =,
    else
    pub "Capture": dir =,
    if (max_size_bw <= max_size_ep)
    dev_dbg(dev,
    "%s %s: Would use wMaxPacketSize %d and bInterval %d\n",
    pub bint): speed_names[speed], dir, max_size_bw,,
    else {
    dev_warn(dev,
    "%s %s: Req. wMaxPacketSize %d at bInterval %d > max ISOC %d, may drop data!\n",
    pub max_size_ep): speed_names[speed], dir, max_size_bw, bint,,
    pub max_size_ep: max_size_bw =,
    }
    pub cpu_to_le16(max_size_bw): ep_desc->wMaxPacketSize =,
    pub bint: ep_desc->bInterval =,
    pub 0: return,
    }
    static struct uac2_feature_unit_descriptor *build_fu_desc(int chmask)
    {
    pub fu_desc: *mut uac2_feature_unit_descriptor,
    pub num_channels(chmask): int channels =,
    pub UAC2_DT_FEATURE_UNIT_SIZE(channels): int fu_desc_size =,
    pub GFP_KERNEL): fu_desc = kzalloc(fu_desc_size,,
    if (!fu_desc)
    pub NULL: return,
    pub fu_desc_size: fu_desc->bLength =,
    pub USB_DT_CS_INTERFACE: fu_desc->bDescriptorType =,
    pub UAC_FEATURE_UNIT: fu_desc->bDescriptorSubtype =,
// bUnitID, bSourceID and bmaControls will be defined later
    pub fu_desc: return,
    }
// Use macro to overcome line length limitation

    static void setup_headers(struct f_uac2_opts *opts,
    struct usb_descriptor_header **headers,
    enum usb_device_speed speed)
    {
    pub NULL: *mut *mut usb_ss_ep_comp_descriptor epout_desc_comp =,
    pub NULL: *mut *mut usb_ss_ep_comp_descriptor epin_desc_comp =,
    pub NULL: *mut *mut usb_ss_ep_comp_descriptor epin_fback_desc_comp =,
    pub NULL: *mut *mut usb_ss_ep_comp_descriptor ep_int_desc_comp =,
    pub epout_desc: *mut usb_endpoint_descriptor,
    pub epin_desc: *mut usb_endpoint_descriptor,
    pub epin_fback_desc: *mut usb_endpoint_descriptor,
    pub ep_int_desc: *mut usb_endpoint_descriptor,
    pub i: c_int,
    switch (speed) {
    case USB_SPEED_FULL:
    pub &fs_epout_desc: epout_desc =,
    pub &fs_epin_desc: epin_desc =,
    pub &fs_epin_fback_desc: epin_fback_desc =,
    pub &fs_ep_int_desc: ep_int_desc =,
    case USB_SPEED_HIGH:
    pub &hs_epout_desc: epout_desc =,
    pub &hs_epin_desc: epin_desc =,
    pub &hs_epin_fback_desc: epin_fback_desc =,
    pub &hs_ep_int_desc: ep_int_desc =,
    default:
    pub &ss_epout_desc: epout_desc =,
    pub &ss_epin_desc: epin_desc =,
    pub &ss_epout_desc_comp: epout_desc_comp =,
    pub &ss_epin_desc_comp: epin_desc_comp =,
    pub &ss_epin_fback_desc: epin_fback_desc =,
    pub &ss_epin_fback_desc_comp: epin_fback_desc_comp =,
    pub &ss_ep_int_desc: ep_int_desc =,
    pub &ss_ep_int_desc_comp: ep_int_desc_comp =,
    }
    pub 0: i =,
    pub USBDHDR(&iad_desc): headers[i++] =,
    pub USBDHDR(&std_ac_if_desc): headers[i++] =,
    pub USBDHDR(&ac_hdr_desc): headers[i++] =,
    if (EPIN_EN(opts))
    pub USBDHDR(&in_clk_src_desc): headers[i++] =,
    if (EPOUT_EN(opts)) {
    pub USBDHDR(&out_clk_src_desc): headers[i++] =,
    pub USBDHDR(&usb_out_it_desc): headers[i++] =,
    if (FUOUT_EN(opts))
    pub USBDHDR(out_feature_unit_desc): headers[i++] =,
    }
    if (EPIN_EN(opts)) {
    pub USBDHDR(&io_in_it_desc): headers[i++] =,
    if (FUIN_EN(opts))
    pub USBDHDR(in_feature_unit_desc): headers[i++] =,
    pub USBDHDR(&usb_in_ot_desc): headers[i++] =,
    }
    if (EPOUT_EN(opts))
    pub USBDHDR(&io_out_ot_desc): headers[i++] =,
    if (FUOUT_EN(opts) || FUIN_EN(opts)) {
    pub USBDHDR(ep_int_desc): headers[i++] =,
    if (ep_int_desc_comp)
    pub USBDHDR(ep_int_desc_comp): headers[i++] =,
    }
    if (EPOUT_EN(opts)) {
    pub USBDHDR(&std_as_out_if0_desc): headers[i++] =,
    pub USBDHDR(&std_as_out_if1_desc): headers[i++] =,
    pub USBDHDR(&as_out_hdr_desc): headers[i++] =,
    pub USBDHDR(&as_out_fmt1_desc): headers[i++] =,
    pub USBDHDR(epout_desc): headers[i++] =,
    if (epout_desc_comp)
    pub USBDHDR(epout_desc_comp): headers[i++] =,
    pub USBDHDR(&as_iso_out_desc): headers[i++] =,
    if (EPOUT_FBACK_IN_EN(opts)) {
    pub USBDHDR(epin_fback_desc): headers[i++] =,
    if (epin_fback_desc_comp)
    pub USBDHDR(epin_fback_desc_comp): headers[i++] =,
    }
    }
    if (EPIN_EN(opts)) {
    pub USBDHDR(&std_as_in_if0_desc): headers[i++] =,
    pub USBDHDR(&std_as_in_if1_desc): headers[i++] =,
    pub USBDHDR(&as_in_hdr_desc): headers[i++] =,
    pub USBDHDR(&as_in_fmt1_desc): headers[i++] =,
    pub USBDHDR(epin_desc): headers[i++] =,
    if (epin_desc_comp)
    pub USBDHDR(epin_desc_comp): headers[i++] =,
    pub USBDHDR(&as_iso_in_desc): headers[i++] =,
    }
    pub NULL: headers[i] =,
    }
#[no_mangle]
unsafe extern "C" fn setup_descriptor(opts: *mut f_uac2_opts) {
    static void setup_descriptor(struct f_uac2_opts *opts)
    {
// patch descriptors
    pub /: *mut *mut int i = 1; / ID's start with 1,
    if (EPOUT_EN(opts))
    pub i++: usb_out_it_desc.bTerminalID =,
    if (EPIN_EN(opts))
    pub i++: io_in_it_desc.bTerminalID =,
    if (EPOUT_EN(opts))
    pub i++: io_out_ot_desc.bTerminalID =,
    if (EPIN_EN(opts))
    pub i++: usb_in_ot_desc.bTerminalID =,
    if (FUOUT_EN(opts))
    pub i++: out_feature_unit_desc->bUnitID =,
    if (FUIN_EN(opts))
    pub i++: in_feature_unit_desc->bUnitID =,
    if (EPOUT_EN(opts))
    pub i++: out_clk_src_desc.bClockID =,
    if (EPIN_EN(opts))
    pub i++: in_clk_src_desc.bClockID =,
    pub out_clk_src_desc.bClockID: usb_out_it_desc.bCSourceID =,
    if (FUIN_EN(opts)) {
    pub in_feature_unit_desc->bUnitID: usb_in_ot_desc.bSourceID =,
    pub io_in_it_desc.bTerminalID: in_feature_unit_desc->bSourceID =,
    } else {
    pub io_in_it_desc.bTerminalID: usb_in_ot_desc.bSourceID =,
    }
    pub in_clk_src_desc.bClockID: usb_in_ot_desc.bCSourceID =,
    pub in_clk_src_desc.bClockID: io_in_it_desc.bCSourceID =,
    pub out_clk_src_desc.bClockID: io_out_ot_desc.bCSourceID =,
    if (FUOUT_EN(opts)) {
    pub out_feature_unit_desc->bUnitID: io_out_ot_desc.bSourceID =,
    pub usb_out_it_desc.bTerminalID: out_feature_unit_desc->bSourceID =,
    } else {
    pub usb_out_it_desc.bTerminalID: io_out_ot_desc.bSourceID =,
    }
    pub usb_out_it_desc.bTerminalID: as_out_hdr_desc.bTerminalLink =,
    pub usb_in_ot_desc.bTerminalID: as_in_hdr_desc.bTerminalLink =,
    pub 1: iad_desc.bInterfaceCount =,
    pub cpu_to_le16(sizeof(ac_hdr_desc)): ac_hdr_desc.wTotalLength =,
    if (EPIN_EN(opts)) {
    pub le16_to_cpu(ac_hdr_desc.wTotalLength): u16 len =,
    pub sizeof(in_clk_src_desc): len +=,
    pub sizeof(usb_in_ot_desc): len +=,
    if (FUIN_EN(opts))
    pub in_feature_unit_desc->bLength: len +=,
    pub sizeof(io_in_it_desc): len +=,
    pub cpu_to_le16(len): ac_hdr_desc.wTotalLength =,
    }
    if (EPOUT_EN(opts)) {
    pub le16_to_cpu(ac_hdr_desc.wTotalLength): u16 len =,
    pub sizeof(out_clk_src_desc): len +=,
    pub sizeof(usb_out_it_desc): len +=,
    if (FUOUT_EN(opts))
    pub out_feature_unit_desc->bLength: len +=,
    pub sizeof(io_out_ot_desc): len +=,
    pub cpu_to_le16(len): ac_hdr_desc.wTotalLength =,
    }
    pub cpu_to_le16(opts->c_terminal_type): io_in_it_desc.wTerminalType =,
    pub cpu_to_le16(opts->p_terminal_type): io_out_ot_desc.wTerminalType =,
    pub USB_SPEED_FULL): setup_headers(opts, fs_audio_desc,,
    pub USB_SPEED_HIGH): setup_headers(opts, hs_audio_desc,,
    pub USB_SPEED_SUPER): setup_headers(opts, ss_audio_desc,,
    }
#[no_mangle]
unsafe extern "C" fn afunc_validate_opts(agdev: *mut g_audio, dev: *mut device) -> c_int {
    static int afunc_validate_opts(struct g_audio *agdev, struct device *dev)
    {
    pub g_audio_to_uac2_opts(agdev): *mut *mut f_uac2_opts opts =,
    pub NULL: *const *const char msg =,
    if (!opts.p_chmask && !opts.c_chmask)
    pub channels": msg = "no playback and capture,
#[no_mangle]
pub unsafe extern "C" fn if(~UAC2_CHANNEL_MASK: opts->p_chmask &) -> else {
    else if (opts.p_chmask & ~UAC2_CHANNEL_MASK)
    pub mask": msg = "unsupported playback channels,
#[no_mangle]
pub unsafe extern "C" fn if(~UAC2_CHANNEL_MASK: opts->c_chmask &) -> else {
    else if (opts.c_chmask & ~UAC2_CHANNEL_MASK)
    pub mask": msg = "unsupported capture channels,
#[no_mangle]
pub unsafe extern "C" fn if(4): (opts->p_ssize < 1) || (opts->p_ssize >) -> else {
    else if ((opts.p_ssize < 1) || (opts.p_ssize > 4))
    pub size": msg = "incorrect playback sample,
#[no_mangle]
pub unsafe extern "C" fn if(4): (opts->c_ssize < 1) || (opts->c_ssize >) -> else {
    else if ((opts.c_ssize < 1) || (opts.c_ssize > 4))
    pub size": msg = "incorrect capture sample,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !opts->p_srates[0]) -> else {
    else if (!opts.p_srates[0])
    pub rate": msg = "incorrect playback sampling,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !opts->c_srates[0]) -> else {
    else if (!opts.c_srates[0])
    pub rate": msg = "incorrect capture sampling,
#[no_mangle]
pub unsafe extern "C" fn if(opts->p_volume_min: opts->p_volume_max <=) -> else {
    else if (opts.p_volume_max <= opts.p_volume_min)
    pub max/min": msg = "incorrect playback volume,
#[no_mangle]
pub unsafe extern "C" fn if(opts->c_volume_min: opts->c_volume_max <=) -> else {
    else if (opts.c_volume_max <= opts.c_volume_min)
    pub max/min": msg = "incorrect capture volume,
#[no_mangle]
pub unsafe extern "C" fn if(0: opts->p_volume_res <=) -> else {
    else if (opts.p_volume_res <= 0)
    pub resolution": msg = "negative/zero playback volume,
#[no_mangle]
pub unsafe extern "C" fn if(0: opts->c_volume_res <=) -> else {
    else if (opts.c_volume_res <= 0)
    pub resolution": msg = "negative/zero capture volume,
#[no_mangle]
pub unsafe extern "C" fn if(opts->p_volume_res: (opts->p_volume_max - opts->p_volume_min) %) -> else {
    else if ((opts.p_volume_max - opts.p_volume_min) % opts.p_volume_res)
    pub resolution": msg = "incorrect playback volume,
#[no_mangle]
pub unsafe extern "C" fn if(opts->c_volume_res: (opts->c_volume_max - opts->c_volume_min) %) -> else {
    else if ((opts.c_volume_max - opts.c_volume_min) % opts.c_volume_res)
    pub resolution": msg = "incorrect capture volume,
#[no_mangle]
pub unsafe extern "C" fn if(4): (opts->p_hs_bint < 0) || (opts->p_hs_bint >) -> else {
    else if ((opts.p_hs_bint < 0) || (opts.p_hs_bint > 4))
    pub auto)": msg = "incorrect playback HS/SS bInterval (1-4: fixed, 0:,
#[no_mangle]
pub unsafe extern "C" fn if(4): (opts->c_hs_bint < 0) || (opts->c_hs_bint >) -> else {
    else if ((opts.c_hs_bint < 0) || (opts.c_hs_bint > 4))
    pub auto)": msg = "incorrect capture HS/SS bInterval (1-4: fixed, 0:,
    if (msg) {
    pub msg): dev_err(dev, "Error: %s\n",,
    pub -EINVAL: return,
    }
    pub 0: return,
    }
    static int
    afunc_bind(struct usb_configuration *cfg, struct usb_function *fn)
    {
    pub func_to_uac2(fn): *mut *mut f_uac2 uac2 =,
    pub func_to_g_audio(fn): *mut *mut g_audio agdev =,
    pub cfg->cdev: *mut *mut usb_composite_dev cdev =,
    pub cdev->gadget: *mut *mut usb_gadget gadget =,
    pub &gadget->dev: *mut *mut device dev =,
    pub g_audio_to_uac2_opts(agdev): *mut *mut f_uac2_opts uac2_opts =,
    pub us: *mut usb_string,
    pub ret: c_int,
    pub dev): ret = afunc_validate_opts(agdev,,
    if (ret)
    pub ret: return,
    pub uac2_opts->function_name: strings_fn[STR_ASSOC].s =,
    pub uac2_opts->if_ctrl_name: strings_fn[STR_IF_CTRL].s =,
    pub uac2_opts->clksrc_in_name: strings_fn[STR_CLKSRC_IN].s =,
    pub uac2_opts->clksrc_out_name: strings_fn[STR_CLKSRC_OUT].s =,
    pub uac2_opts->c_it_name: strings_fn[STR_USB_IT].s =,
    pub uac2_opts->c_it_ch_name: strings_fn[STR_USB_IT_CH].s =,
    pub uac2_opts->c_ot_name: strings_fn[STR_IO_OT].s =,
    pub uac2_opts->c_fu_vol_name: strings_fn[STR_FU_OUT].s =,
    pub Inactive": strings_fn[STR_AS_OUT_ALT0].s = "Playback,
    pub Active": strings_fn[STR_AS_OUT_ALT1].s = "Playback,
    pub uac2_opts->p_it_name: strings_fn[STR_IO_IT].s =,
    pub uac2_opts->p_it_ch_name: strings_fn[STR_IO_IT_CH].s =,
    pub uac2_opts->p_ot_name: strings_fn[STR_USB_OT].s =,
    pub uac2_opts->p_fu_vol_name: strings_fn[STR_FU_IN].s =,
    pub Inactive": strings_fn[STR_AS_IN_ALT0].s = "Capture,
    pub Active": strings_fn[STR_AS_IN_ALT1].s = "Capture,
    pub ARRAY_SIZE(strings_fn)): us = usb_gstrings_attach(cdev, fn_strings,,
    if (IS_ERR(us))
    pub PTR_ERR(us): return,
    if (FUOUT_EN(uac2_opts)) {
    pub build_fu_desc(uac2_opts->c_chmask): out_feature_unit_desc =,
    if (!out_feature_unit_desc)
    pub -ENOMEM: return,
    }
    if (FUIN_EN(uac2_opts)) {
    pub build_fu_desc(uac2_opts->p_chmask): in_feature_unit_desc =,
    if (!in_feature_unit_desc) {
    pub -ENOMEM: ret =,
    pub err_free_fu: goto,
    }
    }
    pub us[STR_ASSOC].id: iad_desc.iFunction =,
    pub us[STR_IF_CTRL].id: std_ac_if_desc.iInterface =,
    pub us[STR_CLKSRC_IN].id: in_clk_src_desc.iClockSource =,
    pub us[STR_CLKSRC_OUT].id: out_clk_src_desc.iClockSource =,
    pub us[STR_USB_IT].id: usb_out_it_desc.iTerminal =,
    pub us[STR_USB_IT_CH].id: usb_out_it_desc.iChannelNames =,
    pub us[STR_IO_IT].id: io_in_it_desc.iTerminal =,
    pub us[STR_IO_IT_CH].id: io_in_it_desc.iChannelNames =,
    pub us[STR_USB_OT].id: usb_in_ot_desc.iTerminal =,
    pub us[STR_IO_OT].id: io_out_ot_desc.iTerminal =,
    pub us[STR_AS_OUT_ALT0].id: std_as_out_if0_desc.iInterface =,
    pub us[STR_AS_OUT_ALT1].id: std_as_out_if1_desc.iInterface =,
    pub us[STR_AS_IN_ALT0].id: std_as_in_if0_desc.iInterface =,
    pub us[STR_AS_IN_ALT1].id: std_as_in_if1_desc.iInterface =,
    if (FUOUT_EN(uac2_opts)) {
    u8 *i_feature = (u8 *)out_feature_unit_desc +
    pub 1: out_feature_unit_desc->bLength -,
// i_feature = us[STR_FU_OUT].id;
    }
    if (FUIN_EN(uac2_opts)) {
    u8 *i_feature = (u8 *)in_feature_unit_desc +
    pub 1: in_feature_unit_desc->bLength -,
// i_feature = us[STR_FU_IN].id;
    }
// Initialize the configurable parameters
    pub num_channels(uac2_opts->c_chmask): usb_out_it_desc.bNrChannels =,
    pub cpu_to_le32(uac2_opts->c_chmask): usb_out_it_desc.bmChannelConfig =,
    pub num_channels(uac2_opts->p_chmask): io_in_it_desc.bNrChannels =,
    pub cpu_to_le32(uac2_opts->p_chmask): io_in_it_desc.bmChannelConfig =,
    pub num_channels(uac2_opts->c_chmask): as_out_hdr_desc.bNrChannels =,
    pub cpu_to_le32(uac2_opts->c_chmask): as_out_hdr_desc.bmChannelConfig =,
    pub num_channels(uac2_opts->p_chmask): as_in_hdr_desc.bNrChannels =,
    pub cpu_to_le32(uac2_opts->p_chmask): as_in_hdr_desc.bmChannelConfig =,
    pub uac2_opts->c_ssize: as_out_fmt1_desc.bSubslotSize =,
    pub 8: *mut *mut as_out_fmt1_desc.bBitResolution = uac2_opts->c_ssize,
    pub uac2_opts->p_ssize: as_in_fmt1_desc.bSubslotSize =,
    pub 8: *mut *mut as_in_fmt1_desc.bBitResolution = uac2_opts->p_ssize,
    if (FUOUT_EN(uac2_opts)) {
    pub )&out_feature_unit_desc->bmaControls[0]: *mut *mut __le32 bma = (__le32,
    pub 0: u32 control =,
    if (uac2_opts.c_mute_present)
    pub FU_MUTE_CTRL: control |= CONTROL_RDWR <<,
    if (uac2_opts.c_volume_present)
    pub FU_VOL_CTRL: control |= CONTROL_RDWR <<,
// bma = cpu_to_le32(control);
    }
    if (FUIN_EN(uac2_opts)) {
    pub )&in_feature_unit_desc->bmaControls[0]: *mut *mut __le32 bma = (__le32,
    pub 0: u32 control =,
    if (uac2_opts.p_mute_present)
    pub FU_MUTE_CTRL: control |= CONTROL_RDWR <<,
    if (uac2_opts.p_volume_present)
    pub FU_VOL_CTRL: control |= CONTROL_RDWR <<,
// bma = cpu_to_le32(control);
    }
    pub fn): ret = usb_interface_id(cfg,,
    if (ret < 0) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub err_free_fu: goto,
    }
    pub ret: iad_desc.bFirstInterface =,
    pub ret: std_ac_if_desc.bInterfaceNumber =,
    pub ret: uac2->ac_intf =,
    pub 0: uac2->ac_alt =,
    if (EPOUT_EN(uac2_opts)) {
    pub fn): ret = usb_interface_id(cfg,,
    if (ret < 0) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub err_free_fu: goto,
    }
    pub ret: std_as_out_if0_desc.bInterfaceNumber =,
    pub ret: std_as_out_if1_desc.bInterfaceNumber =,
    pub 1: std_as_out_if1_desc.bNumEndpoints =,
    pub ret: uac2->as_out_intf =,
    pub 0: uac2->as_out_alt =,
    if (EPOUT_FBACK_IN_EN(uac2_opts)) {
    fs_epout_desc.bmAttributes =
    pub USB_ENDPOINT_SYNC_ASYNC: USB_ENDPOINT_XFER_ISOC |,
    hs_epout_desc.bmAttributes =
    pub USB_ENDPOINT_SYNC_ASYNC: USB_ENDPOINT_XFER_ISOC |,
    ss_epout_desc.bmAttributes =
    pub USB_ENDPOINT_SYNC_ASYNC: USB_ENDPOINT_XFER_ISOC |,
    } else {
    fs_epout_desc.bmAttributes =
    pub USB_ENDPOINT_SYNC_ADAPTIVE: USB_ENDPOINT_XFER_ISOC |,
    hs_epout_desc.bmAttributes =
    pub USB_ENDPOINT_SYNC_ADAPTIVE: USB_ENDPOINT_XFER_ISOC |,
    ss_epout_desc.bmAttributes =
    pub USB_ENDPOINT_SYNC_ADAPTIVE: USB_ENDPOINT_XFER_ISOC |,
    }
    }
    if (EPIN_EN(uac2_opts)) {
    pub fn): ret = usb_interface_id(cfg,,
    if (ret < 0) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub err_free_fu: goto,
    }
    pub ret: std_as_in_if0_desc.bInterfaceNumber =,
    pub ret: std_as_in_if1_desc.bInterfaceNumber =,
    pub ret: uac2->as_in_intf =,
    pub 0: uac2->as_in_alt =,
    }
    pub 0: std_ac_if_desc.bNumEndpoints =,
    if (FUOUT_EN(uac2_opts) || FUIN_EN(uac2_opts)) {
    pub &fs_ep_int_desc): uac2->int_ep = usb_ep_autoconfig(gadget,,
    if (!uac2.int_ep) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub -ENODEV: ret =,
    pub err_free_fu: goto,
    }
    pub 1: std_ac_if_desc.bNumEndpoints =,
    }
    pub uac2_opts->p_hs_bint: hs_epin_desc.bInterval =,
    pub uac2_opts->p_hs_bint: ss_epin_desc.bInterval =,
    pub uac2_opts->c_hs_bint: hs_epout_desc.bInterval =,
    pub uac2_opts->c_hs_bint: ss_epout_desc.bInterval =,
// Calculate wMaxPacketSize according to audio bandwidth
    ret = set_ep_max_packet_size_bint(dev, uac2_opts, &fs_epin_desc,
    pub true): USB_SPEED_FULL,,
    if (ret < 0) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub ret: return,
    }
    ret = set_ep_max_packet_size_bint(dev, uac2_opts, &fs_epout_desc,
    pub false): USB_SPEED_FULL,,
    if (ret < 0) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub ret: return,
    }
    ret = set_ep_max_packet_size_bint(dev, uac2_opts, &hs_epin_desc,
    pub true): USB_SPEED_HIGH,,
    if (ret < 0) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub ret: return,
    }
    ret = set_ep_max_packet_size_bint(dev, uac2_opts, &hs_epout_desc,
    pub false): USB_SPEED_HIGH,,
    if (ret < 0) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub ret: return,
    }
    ret = set_ep_max_packet_size_bint(dev, uac2_opts, &ss_epin_desc,
    pub true): USB_SPEED_SUPER,,
    if (ret < 0) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub ret: return,
    }
    ret = set_ep_max_packet_size_bint(dev, uac2_opts, &ss_epout_desc,
    pub false): USB_SPEED_SUPER,,
    if (ret < 0) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub ret: return,
    }
    if (EPOUT_EN(uac2_opts)) {
    pub &fs_epout_desc): agdev->out_ep = usb_ep_autoconfig(gadget,,
    if (!agdev.out_ep) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub -ENODEV: ret =,
    pub err_free_fu: goto,
    }
    if (EPOUT_FBACK_IN_EN(uac2_opts)) {
    agdev.in_ep_fback = usb_ep_autoconfig(gadget,
    if (!agdev.in_ep_fback) {
    dev_err(dev, "%s:%d Error!\n",
    pub __LINE__): __func__,,
    pub -ENODEV: ret =,
    pub err_free_fu: goto,
    }
    }
    }
    if (EPIN_EN(uac2_opts)) {
    pub &fs_epin_desc): agdev->in_ep = usb_ep_autoconfig(gadget,,
    if (!agdev.in_ep) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub -ENODEV: ret =,
    pub err_free_fu: goto,
    }
    }
    agdev.in_ep_maxpsize = max_t(u16,
    le16_to_cpu(fs_epin_desc.wMaxPacketSize),
    agdev.out_ep_maxpsize = max_t(u16,
    le16_to_cpu(fs_epout_desc.wMaxPacketSize),
    agdev.in_ep_maxpsize = max_t(u16, agdev.in_ep_maxpsize,
    agdev.out_ep_maxpsize = max_t(u16, agdev.out_ep_maxpsize,
    pub ss_epin_desc.wMaxPacketSize: ss_epin_desc_comp.wBytesPerInterval =,
    pub ss_epout_desc.wMaxPacketSize: ss_epout_desc_comp.wBytesPerInterval =,
// HS and SS endpoint addresses are copied from autoconfigured FS descriptors
    pub fs_ep_int_desc.bEndpointAddress: hs_ep_int_desc.bEndpointAddress =,
    pub fs_epout_desc.bEndpointAddress: hs_epout_desc.bEndpointAddress =,
    pub fs_epin_fback_desc.bEndpointAddress: hs_epin_fback_desc.bEndpointAddress =,
    pub fs_epin_desc.bEndpointAddress: hs_epin_desc.bEndpointAddress =,
    pub fs_epout_desc.bEndpointAddress: ss_epout_desc.bEndpointAddress =,
    pub fs_epin_fback_desc.bEndpointAddress: ss_epin_fback_desc.bEndpointAddress =,
    pub fs_epin_desc.bEndpointAddress: ss_epin_desc.bEndpointAddress =,
    pub fs_ep_int_desc.bEndpointAddress: ss_ep_int_desc.bEndpointAddress =,
    ret = usb_assign_descriptors(fn, fs_audio_desc, hs_audio_desc, ss_audio_desc,
    if (ret)
    pub err_free_fu: goto,
    pub gadget: agdev->gadget =,
    pub uac2_opts->p_chmask: agdev->params.p_chmask =,
    memcpy(agdev.params.p_srates, uac2_opts.p_srates,
    pub uac2_opts->p_ssize: agdev->params.p_ssize =,
    if (FUIN_EN(uac2_opts)) {
    pub USB_IN_FU_ID: agdev->params.p_fu.id =,
    pub uac2_opts->p_mute_present: agdev->params.p_fu.mute_present =,
    pub uac2_opts->p_volume_present: agdev->params.p_fu.volume_present =,
    pub uac2_opts->p_volume_min: agdev->params.p_fu.volume_min =,
    pub uac2_opts->p_volume_max: agdev->params.p_fu.volume_max =,
    pub uac2_opts->p_volume_res: agdev->params.p_fu.volume_res =,
    }
    pub uac2_opts->c_chmask: agdev->params.c_chmask =,
    memcpy(agdev.params.c_srates, uac2_opts.c_srates,
    pub uac2_opts->c_ssize: agdev->params.c_ssize =,
    if (FUOUT_EN(uac2_opts)) {
    pub USB_OUT_FU_ID: agdev->params.c_fu.id =,
    pub uac2_opts->c_mute_present: agdev->params.c_fu.mute_present =,
    pub uac2_opts->c_volume_present: agdev->params.c_fu.volume_present =,
    pub uac2_opts->c_volume_min: agdev->params.c_fu.volume_min =,
    pub uac2_opts->c_volume_max: agdev->params.c_fu.volume_max =,
    pub uac2_opts->c_volume_res: agdev->params.c_fu.volume_res =,
    }
    pub uac2_opts->req_number: agdev->params.req_number =,
    pub uac2_opts->fb_max: agdev->params.fb_max =,
    if (FUOUT_EN(uac2_opts) || FUIN_EN(uac2_opts))
    pub afunc_notify: agdev->notify =,
    pub "UAC2_Gadget"): ret = g_audio_setup(agdev, "UAC2 PCM",,
    if (ret)
    pub err_free_descs: goto,
    pub 0: return,
    err_free_descs:
    pub NULL: agdev->gadget =,
    err_free_fu:
    pub NULL: out_feature_unit_desc =,
    pub NULL: in_feature_unit_desc =,
    pub ret: return,
    }
    static void
    afunc_notify_complete(struct usb_ep *_ep, struct usb_request *req)
    {
    pub req->context: *mut *mut g_audio agdev =,
    pub func_to_uac2(&agdev->func): *mut *mut f_uac2 uac2 =,
    pub req): usb_ep_free_request(_ep,,
    }
    static int
    afunc_notify(struct g_audio *agdev, int unit_id, int cs)
    {
    pub func_to_uac2(&agdev->func): *mut *mut f_uac2 uac2 =,
    pub req: *mut usb_request,
    pub msg: *mut uac2_interrupt_data_msg,
    pub w_value: u16 w_index,,
    pub ret: c_int,
    if (!uac2.int_ep.enabled)
    pub 0: return,
    if (atomic_inc_return(&uac2.int_count) > UAC2_DEF_INT_REQ_NUM) {
    pub 0: return,
    }
    pub GFP_ATOMIC): req = usb_ep_alloc_request(uac2->int_ep,,
    if (req == core::ptr::null_mut()) {
    pub -ENOMEM: ret =,
    pub err_dec_int_count: goto,
    }
    pub GFP_ATOMIC): *mut *mut msg = kzalloc_obj(msg,,
    if (msg == core::ptr::null_mut()) {
    pub -ENOMEM: ret =,
    pub err_free_request: goto,
    }
    pub uac2->ac_intf: w_index = unit_id << 8 |,
    pub 8: w_value = cs <<,
    pub /: *mut *mut msg->bInfo = 0; / Non-vendor, interface interrupt,
    pub UAC2_CS_CUR: msg->bAttribute =,
    pub cpu_to_le16(w_index): msg->wIndex =,
    pub cpu_to_le16(w_value): msg->wValue =,
    pub sizeof(*msg): *mut req->length =,
    pub msg: req->buf =,
    pub agdev: req->context =,
    pub afunc_notify_complete: req->complete =,
    pub GFP_ATOMIC): ret = usb_ep_queue(uac2->int_ep, req,,
    if (ret)
    pub err_free_msg: goto,
    pub 0: return,
    err_free_msg:
    err_free_request:
    pub req): usb_ep_free_request(uac2->int_ep,,
    err_dec_int_count:
    pub ret: return,
    }
    static int
    afunc_set_alt(struct usb_function *fn, unsigned intf, unsigned alt)
    {
    pub fn->config->cdev: *mut *mut usb_composite_dev cdev =,
    pub func_to_uac2(fn): *mut *mut f_uac2 uac2 =,
    pub func_to_g_audio(fn): *mut *mut g_audio agdev =,
    pub cdev->gadget: *mut *mut usb_gadget gadget =,
    pub &gadget->dev: *mut *mut device dev =,
    pub 0: int ret =,
// No i/f has more than 2 alt settings
    if (alt > 1) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub -EINVAL: return,
    }
    if (intf == uac2.ac_intf) {
// Control I/f has only 1 AltSetting - 0
    if (alt) {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub -EINVAL: return,
    }
// restart interrupt endpoint
    if (uac2.int_ep) {
    pub uac2->int_ep): config_ep_by_speed(gadget, &agdev->func,,
    }
    pub 0: return,
    }
    if (intf == uac2.as_out_intf) {
    pub alt: uac2->as_out_alt =,
    if (alt)
    pub u_audio_start_capture(&uac2->g_audio): ret =,
    else
    } else if (intf == uac2.as_in_intf) {
    pub alt: uac2->as_in_alt =,
    if (alt)
    pub u_audio_start_playback(&uac2->g_audio): ret =,
    else
    } else {
    pub __LINE__): dev_err(dev, "%s:%d Error!\n", __func__,,
    pub -EINVAL: return,
    }
    pub ret: return,
    }
    static int
    afunc_get_alt(struct usb_function *fn, unsigned intf)
    {
    pub func_to_uac2(fn): *mut *mut f_uac2 uac2 =,
    pub func_to_g_audio(fn): *mut *mut g_audio agdev =,
    if (intf == uac2.ac_intf)
    pub uac2->ac_alt: return,
#[no_mangle]
pub unsafe extern "C" fn if(uac2->as_out_intf: intf ==) -> else {
    else if (intf == uac2.as_out_intf)
    pub uac2->as_out_alt: return,
#[no_mangle]
pub unsafe extern "C" fn if(uac2->as_in_intf: intf ==) -> else {
    else if (intf == uac2.as_in_intf)
    pub uac2->as_in_alt: return,
    else
    dev_err(&agdev.gadget.dev,
    "%s:%d Invalid Interface %d!\n",
    pub intf): __func__, __LINE__,,
    pub -EINVAL: return,
    }
    static void
    afunc_disable(struct usb_function *fn)
    {
    pub func_to_uac2(fn): *mut *mut f_uac2 uac2 =,
    pub 0: uac2->as_in_alt =,
    pub 0: uac2->as_out_alt =,
    if (uac2.int_ep)
    }
    static void
    afunc_suspend(struct usb_function *fn)
    {
    pub func_to_uac2(fn): *mut *mut f_uac2 uac2 =,
    }
    static int
    in_rq_cur(struct usb_function *fn, const struct usb_ctrlrequest *cr)
    {
    pub fn->config->cdev->req: *mut *mut usb_request req =,
    pub func_to_g_audio(fn): *mut *mut g_audio agdev =,
    pub g_audio_to_uac2_opts(agdev): *mut *mut f_uac2_opts opts =,
    pub le16_to_cpu(cr->wLength): u16 w_length =,
    pub le16_to_cpu(cr->wIndex): u16 w_index =,
    pub le16_to_cpu(cr->wValue): u16 w_value =,
    pub 0xff: u8 entity_id = (w_index >> 8) &,
    pub 8: u8 control_selector = w_value >>,
    pub -EOPNOTSUPP: int value =,
    pub c_srate: u32 p_srate,,
    pub &p_srate): u_audio_get_playback_srate(agdev,,
    pub &c_srate): u_audio_get_capture_srate(agdev,,
    if ((entity_id == USB_IN_CLK_ID) || (entity_id == USB_OUT_CLK_ID)) {
    if (control_selector == UAC2_CS_CONTROL_SAM_FREQ) {
    pub c: cntrl_cur_lay3,
    pub cntrl_cur_lay3)): memset(&c, 0, sizeof(struct,
    if (entity_id == USB_IN_CLK_ID)
    pub cpu_to_le32(p_srate): c.dCUR =,
#[no_mangle]
pub unsafe extern "C" fn if(USB_OUT_CLK_ID: entity_id ==) -> else {
    else if (entity_id == USB_OUT_CLK_ID)
    pub cpu_to_le32(c_srate): c.dCUR =,
    pub sizeof(c)): value = min_t(unsigned int, w_length,,
    pub value): memcpy(req->buf, &c,,
    } else if (control_selector == UAC2_CS_CONTROL_CLOCK_VALID) {
// (u8 *)req->buf = 1;
    pub 1): value = min_t(unsigned int, w_length,,
    } else {
    dev_err(&agdev.gadget.dev,
    "%s:%d control_selector=%d TODO!\n",
    pub control_selector): __func__, __LINE__,,
    }
    } else if ((FUIN_EN(opts) && (entity_id == USB_IN_FU_ID)) ||
    (FUOUT_EN(opts) && (entity_id == USB_OUT_FU_ID))) {
    pub 0: unsigned int is_playback =,
    if (FUIN_EN(opts) && (entity_id == USB_IN_FU_ID))
    pub 1: is_playback =,
    if (control_selector == UAC_FU_MUTE) {
    pub mute: c_uint,
    pub &mute): u_audio_get_mute(agdev, is_playback,,
// (u8 *)req->buf = mute;
    pub 1): value = min_t(unsigned int, w_length,,
    } else if (control_selector == UAC_FU_VOLUME) {
    pub c: cntrl_cur_lay2,
    pub volume: i16,
    pub cntrl_cur_lay2)): memset(&c, 0, sizeof(struct,
    pub &volume): u_audio_get_volume(agdev, is_playback,,
    pub cpu_to_le16(volume): c.wCUR =,
    pub sizeof(c)): value = min_t(unsigned int, w_length,,
    pub value): memcpy(req->buf, &c,,
    } else {
    dev_err(&agdev.gadget.dev,
    "%s:%d control_selector=%d TODO!\n",
    pub control_selector): __func__, __LINE__,,
    }
    } else {
    dev_err(&agdev.gadget.dev,
    "%s:%d entity_id=%d control_selector=%d TODO!\n",
    pub control_selector): __func__, __LINE__, entity_id,,
    }
    pub value: return,
    }
    static int
    in_rq_range(struct usb_function *fn, const struct usb_ctrlrequest *cr)
    {
    pub fn->config->cdev->req: *mut *mut usb_request req =,
    pub func_to_g_audio(fn): *mut *mut g_audio agdev =,
    pub g_audio_to_uac2_opts(agdev): *mut *mut f_uac2_opts opts =,
    pub le16_to_cpu(cr->wLength): u16 w_length =,
    pub le16_to_cpu(cr->wIndex): u16 w_index =,
    pub le16_to_cpu(cr->wValue): u16 w_value =,
    pub 0xff: u8 entity_id = (w_index >> 8) &,
    pub 8: u8 control_selector = w_value >>,
    pub -EOPNOTSUPP: int value =,
    if ((entity_id == USB_IN_CLK_ID) || (entity_id == USB_OUT_CLK_ID)) {
    if (control_selector == UAC2_CS_CONTROL_SAM_FREQ) {
    pub rs: cntrl_ranges_lay3_srates,
    pub i: c_int,
    pub 0: int wNumSubRanges =,
    pub srate: c_int,
    pub srates: *mut c_int,
    if (entity_id == USB_IN_CLK_ID)
    pub opts->p_srates: srates =,
#[no_mangle]
pub unsafe extern "C" fn if(USB_OUT_CLK_ID: entity_id ==) -> else {
    else if (entity_id == USB_OUT_CLK_ID)
    pub opts->c_srates: srates =,
    else
    pub -EOPNOTSUPP: return,
    pub {: for (i = 0; i < UAC_MAX_RATES; i++),
    pub srates: [srate =; i],
    if (srate == 0)
    pub cpu_to_le32(srate): rs.r[wNumSubRanges].dMIN =,
    pub cpu_to_le32(srate): rs.r[wNumSubRanges].dMAX =,
    pub 0: rs.r[wNumSubRanges].dRES =,
    dev_dbg(&agdev.gadget.dev,
    "%s(): clk %d: rate ID %d: %d\n",
    pub srate): __func__, entity_id, wNumSubRanges,,
    }
    pub cpu_to_le16(wNumSubRanges): rs.wNumSubRanges =,
    pub ranges_lay3_size(rs)): value = min_t(unsigned int, w_length,,
    dev_dbg(&agdev.gadget.dev, "%s(): sending %d rates, size %d\n",
    pub value): __func__, rs.wNumSubRanges,,
    pub value): memcpy(req->buf, &rs,,
    } else {
    dev_err(&agdev.gadget.dev,
    "%s:%d control_selector=%d TODO!\n",
    pub control_selector): __func__, __LINE__,,
    }
    } else if ((FUIN_EN(opts) && (entity_id == USB_IN_FU_ID)) ||
    (FUOUT_EN(opts) && (entity_id == USB_OUT_FU_ID))) {
    pub 0: unsigned int is_playback =,
    if (FUIN_EN(opts) && (entity_id == USB_IN_FU_ID))
    pub 1: is_playback =,
    if (control_selector == UAC_FU_VOLUME) {
    pub r: cntrl_range_lay2,
    pub res_db: s16 max_db, min_db,,
    if (is_playback) {
    pub opts->p_volume_max: max_db =,
    pub opts->p_volume_min: min_db =,
    pub opts->p_volume_res: res_db =,
    } else {
    pub opts->c_volume_max: max_db =,
    pub opts->c_volume_min: min_db =,
    pub opts->c_volume_res: res_db =,
    }
    pub cpu_to_le16(max_db): r.wMAX =,
    pub cpu_to_le16(min_db): r.wMIN =,
    pub cpu_to_le16(res_db): r.wRES =,
    pub cpu_to_le16(1): r.wNumSubRanges =,
    pub sizeof(r)): value = min_t(unsigned int, w_length,,
    pub value): memcpy(req->buf, &r,,
    } else {
    dev_err(&agdev.gadget.dev,
    "%s:%d control_selector=%d TODO!\n",
    pub control_selector): __func__, __LINE__,,
    }
    } else {
    dev_err(&agdev.gadget.dev,
    "%s:%d entity_id=%d control_selector=%d TODO!\n",
    pub control_selector): __func__, __LINE__, entity_id,,
    }
    pub value: return,
    }
    static int
    ac_rq_in(struct usb_function *fn, const struct usb_ctrlrequest *cr)
    {
    if (cr.bRequest == UAC2_CS_CUR)
    pub cr): return in_rq_cur(fn,,
#[no_mangle]
pub unsafe extern "C" fn if(UAC2_CS_RANGE: cr->bRequest ==) -> else {
    else if (cr.bRequest == UAC2_CS_RANGE)
    pub cr): return in_rq_range(fn,,
    else
    pub -EOPNOTSUPP: return,
    }
#[no_mangle]
unsafe extern "C" fn uac2_cs_control_sam_freq(ep: *mut usb_ep, req: *mut usb_request) {
    static void uac2_cs_control_sam_freq(struct usb_ep *ep, struct usb_request *req)
    {
    pub ep->driver_data: *mut *mut usb_function fn =,
    pub func_to_g_audio(fn): *mut *mut g_audio agdev =,
    pub func_to_uac2(fn): *mut *mut f_uac2 uac2 =,
    pub val: u32,
    if (req.actual != 4)
    pub )req->buf)): *mut *mut val = le32_to_cpu(((__le32,
    pub val): dev_dbg(&agdev->gadget->dev, "%s val: %d.\n", __func__,,
    if (uac2.clock_id == USB_IN_CLK_ID) {
    pub val): u_audio_set_playback_srate(agdev,,
    } else if (uac2.clock_id == USB_OUT_CLK_ID) {
    pub val): u_audio_set_capture_srate(agdev,,
    }
    }
    static void
    out_rq_cur_complete(struct usb_ep *ep, struct usb_request *req)
    {
    pub req->context: *mut *mut g_audio agdev =,
    pub agdev->func.config->cdev: *mut *mut usb_composite_dev cdev =,
    pub g_audio_to_uac2_opts(agdev): *mut *mut f_uac2_opts opts =,
    pub func_to_uac2(&agdev->func): *mut *mut f_uac2 uac2 =,
    pub &uac2->setup_cr: *mut *mut usb_ctrlrequest cr =,
    pub le16_to_cpu(cr->wIndex): u16 w_index =,
    pub le16_to_cpu(cr->wValue): u16 w_value =,
    pub 0xff: u8 entity_id = (w_index >> 8) &,
    pub 8: u8 control_selector = w_value >>,
    if (req.status != 0) {
    pub req->status): dev_dbg(&cdev->gadget->dev, "completion err %d\n",,
    }
    if ((FUIN_EN(opts) && (entity_id == USB_IN_FU_ID)) ||
    (FUOUT_EN(opts) && (entity_id == USB_OUT_FU_ID))) {
    pub 0: unsigned int is_playback =,
    if (FUIN_EN(opts) && (entity_id == USB_IN_FU_ID))
    pub 1: is_playback =,
    if (control_selector == UAC_FU_MUTE) {
    pub )req->buf: *mut *mut u8 mute = (u8,
    pub mute): u_audio_set_mute(agdev, is_playback,,
    } else if (control_selector == UAC_FU_VOLUME) {
    pub req->buf: *mut *mut cntrl_cur_lay2 c =,
    pub volume: i16,
    pub le16_to_cpu(c->wCUR): volume =,
    pub volume): u_audio_set_volume(agdev, is_playback,,
    } else {
    dev_err(&agdev.gadget.dev,
    "%s:%d control_selector=%d TODO!\n",
    pub control_selector): __func__, __LINE__,,
    }
    }
    }
    static int
    out_rq_cur(struct usb_function *fn, const struct usb_ctrlrequest *cr)
    {
    pub fn->config->cdev: *mut *mut usb_composite_dev cdev =,
    pub fn->config->cdev->req: *mut *mut usb_request req =,
    pub func_to_g_audio(fn): *mut *mut g_audio agdev =,
    pub g_audio_to_uac2_opts(agdev): *mut *mut f_uac2_opts opts =,
    pub func_to_uac2(fn): *mut *mut f_uac2 uac2 =,
    pub le16_to_cpu(cr->wLength): u16 w_length =,
    pub le16_to_cpu(cr->wIndex): u16 w_index =,
    pub le16_to_cpu(cr->wValue): u16 w_value =,
    pub 0xff: u8 entity_id = (w_index >> 8) &,
    pub 8: u8 control_selector = w_value >>,
    pub 8: u8 clock_id = w_index >>,
    if ((entity_id == USB_IN_CLK_ID) || (entity_id == USB_OUT_CLK_ID)) {
    if (control_selector == UAC2_CS_CONTROL_SAM_FREQ) {
    dev_dbg(&agdev.gadget.dev,
    pub clock_id): "control_selector UAC2_CS_CONTROL_SAM_FREQ, clock: %d\n",,
    pub fn: cdev->gadget->ep0->driver_data =,
    pub clock_id: uac2->clock_id =,
    pub uac2_cs_control_sam_freq: req->complete =,
    pub w_length: return,
    }
    } else if ((FUIN_EN(opts) && (entity_id == USB_IN_FU_ID)) ||
    (FUOUT_EN(opts) && (entity_id == USB_OUT_FU_ID))) {
    pub sizeof(*cr)): *mut memcpy(&uac2->setup_cr, cr,,
    pub agdev: req->context =,
    pub out_rq_cur_complete: req->complete =,
    pub w_length: return,
    } else {
    dev_err(&agdev.gadget.dev,
    "%s:%d entity_id=%d control_selector=%d TODO!\n",
    pub control_selector): __func__, __LINE__, entity_id,,
    }
    pub -EOPNOTSUPP: return,
    }
    static int
    setup_rq_inf(struct usb_function *fn, const struct usb_ctrlrequest *cr)
    {
    pub func_to_uac2(fn): *mut *mut f_uac2 uac2 =,
    pub func_to_g_audio(fn): *mut *mut g_audio agdev =,
    pub le16_to_cpu(cr->wIndex): u16 w_index =,
    pub 0xff: u8 intf = w_index &,
    if (intf != uac2.ac_intf) {
    dev_err(&agdev.gadget.dev,
    pub __LINE__): "%s:%d Error!\n", __func__,,
    pub -EOPNOTSUPP: return,
    }
    if (cr.bRequestType & USB_DIR_IN)
    pub cr): return ac_rq_in(fn,,
#[no_mangle]
pub unsafe extern "C" fn if(UAC2_CS_CUR: cr->bRequest ==) -> else {
    else if (cr.bRequest == UAC2_CS_CUR)
    pub cr): return out_rq_cur(fn,,
    pub -EOPNOTSUPP: return,
    }
    static int
    afunc_setup(struct usb_function *fn, const struct usb_ctrlrequest *cr)
    {
    pub fn->config->cdev: *mut *mut usb_composite_dev cdev =,
    pub func_to_g_audio(fn): *mut *mut g_audio agdev =,
    pub cdev->req: *mut *mut usb_request req =,
    pub le16_to_cpu(cr->wLength): u16 w_length =,
    pub -EOPNOTSUPP: int value =,
// Only Class specific requests are supposed to reach here
    if ((cr.bRequestType & USB_TYPE_MASK) != USB_TYPE_CLASS)
    pub -EOPNOTSUPP: return,
    if ((cr.bRequestType & USB_RECIP_MASK) == USB_RECIP_INTERFACE)
    pub cr): value = setup_rq_inf(fn,,
    else
    dev_err(&agdev.gadget.dev, "%s:%d Error!\n",
    pub __LINE__): __func__,,
    if (value >= 0) {
    pub value: req->length =,
    pub w_length: req->zero = value <,
    pub GFP_ATOMIC): value = usb_ep_queue(cdev->gadget->ep0, req,,
    if (value < 0) {
    dev_err(&agdev.gadget.dev,
    pub __LINE__): "%s:%d Error!\n", __func__,,
    pub 0: req->status =,
    }
    }
    pub value: return,
    }
    static inline struct f_uac2_opts *to_f_uac2_opts(struct config_item *item)
    {
    return container_of(to_config_group(item), struct f_uac2_opts,
    }
#[no_mangle]
unsafe extern "C" fn f_uac2_attr_release(item: *mut config_item) {
    static void f_uac2_attr_release(struct config_item *item)
    {
    pub to_f_uac2_opts(item): *mut *mut f_uac2_opts opts =,
    }
    static const struct configfs_item_operations f_uac2_item_ops = {
    .release	= f_uac2_attr_release,
}

    static const char *u8_fmt = "%u\n";
    static const char *u32_fmt = "%u\n";
    static const char *s16_fmt = "%hd\n";
    static const char *bool_fmt = "%u\n";

    static ssize_t f_uac2_opts_##name##_show(struct config_item *item,	\
    char *page)			\
    {									\
    struct f_uac2_opts *opts = to_f_uac2_opts(item);		\
    int result;							\
    \
    mutex_lock(&opts.lock);					\
    result = sprintf(page, type##_fmt, opts.name);			\
    mutex_unlock(&opts.lock);					\
    \
    return result;							\
    }									\
    \
    static ssize_t f_uac2_opts_##name##_store(struct config_item *item,	\
    const char *page, size_t len)	\
    {									\
    struct f_uac2_opts *opts = to_f_uac2_opts(item);		\
    int ret;							\
    type num;							\
    \
    mutex_lock(&opts.lock);					\
    if (opts.refcnt) {						\
    ret = -EBUSY;						\
    goto end;						\
    }								\
    \
    ret = uac2_kstrto##type(page, 0, &num);				\
    if (ret)							\
    goto end;						\
    \
    opts.name = num;						\
    ret = len;							\
    \
    end:									\
    mutex_unlock(&opts.lock);					\
    return ret;							\
    }									\
    \
    CONFIGFS_ATTR(f_uac2_opts_, name)

    static ssize_t f_uac2_opts_##name##_show(struct config_item *item,	\
    char *page)			\
    {									\
    struct f_uac2_opts *opts = to_f_uac2_opts(item);		\
    int result;							\
    char *str;							\
    \
    mutex_lock(&opts.lock);					\
    switch (opts.name) {						\
    case USB_ENDPOINT_SYNC_ASYNC:					\
    str = "async";						\
    break;							\
    case USB_ENDPOINT_SYNC_ADAPTIVE:				\
    str = "adaptive";					\
    break;							\
    default:							\
    str = "unknown";					\
    break;							\
    }								\
    result = sprintf(page, "%s\n", str);				\
    mutex_unlock(&opts.lock);					\
    \
    return result;							\
    }									\
    \
    static ssize_t f_uac2_opts_##name##_store(struct config_item *item,	\
    const char *page, size_t len)	\
    {									\
    struct f_uac2_opts *opts = to_f_uac2_opts(item);		\
    int ret = 0;							\
    \
    mutex_lock(&opts.lock);					\
    if (opts.refcnt) {						\
    ret = -EBUSY;						\
    goto end;						\
    }								\
    \
    if (!strncmp(page, "async", 5))					\
    opts.name = USB_ENDPOINT_SYNC_ASYNC;			\
    else if (!strncmp(page, "adaptive", 8))				\
    opts.name = USB_ENDPOINT_SYNC_ADAPTIVE;		\
    else {								\
    ret = -EINVAL;						\
    goto end;						\
    }								\
    \
    ret = len;							\
    \
    end:									\
    mutex_unlock(&opts.lock);					\
    return ret;							\
    }									\
    \
    CONFIGFS_ATTR(f_uac2_opts_, name)

    static ssize_t f_uac2_opts_##name##_show(struct config_item *item,	\
    char *page)			\
    {									\
    struct f_uac2_opts *opts = to_f_uac2_opts(item);		\
    int result = 0;							\
    int i;								\
    \
    mutex_lock(&opts.lock);					\
    page[0] = '\0';							\
    for (i = 0; i < UAC_MAX_RATES; i++) {				\
    if (opts.name##s[i] == 0)				\
    break;						\
    result += sprintf(page + strlen(page), "%u,",		\
    opts.name##s[i]);			\
    }								\
    if (strlen(page) > 0)						\
    page[strlen(page) - 1] = '\n';				\
    mutex_unlock(&opts.lock);					\
    \
    return result;							\
    }									\
    \
    static ssize_t f_uac2_opts_##name##_store(struct config_item *item,	\
    const char *page, size_t len)	\
    {									\
    struct f_uac2_opts *opts = to_f_uac2_opts(item);		\
    char *buf = core::ptr::null_mut();						\
    char *split_page;						\
    int ret = -EINVAL;						\
    char *token;							\
    u32 num;							\
    int i;								\
    \
    mutex_lock(&opts.lock);					\
    if (opts.refcnt) {						\
    ret = -EBUSY;						\
    goto end;						\
    }								\
    \
    i = 0;								\
    memset(opts.name##s, 0x00, sizeof(opts.name##s));		\
    buf = kstrdup(page, GFP_KERNEL);				\
    split_page = buf;						\
    while ((token = strsep(&split_page, ",")) != core::ptr::null_mut()) {		\
    ret = kstrtou32(token, 0, &num);			\
    if (ret)						\
    goto end;					\
    if (i >= UAC_MAX_RATES) {				\
    ret = -EINVAL;					\
    goto end;					\
    }							\
    opts.name##s[i++] = num;				\
    ret = len;						\
    };								\
    \
    end:									\
    kfree(buf);							\
    mutex_unlock(&opts.lock);					\
    return ret;							\
    }									\
    \
    CONFIGFS_ATTR(f_uac2_opts_, name)

    static ssize_t f_uac2_opts_##name##_show(struct config_item *item,	\
    char *page)			\
    {									\
    struct f_uac2_opts *opts = to_f_uac2_opts(item);		\
    int result;							\
    \
    mutex_lock(&opts.lock);					\
    result = sysfs_emit(page, "%s", opts.name);	                \
    mutex_unlock(&opts.lock);					\
    \
    return result;							\
    }									\
    \
    static ssize_t f_uac2_opts_##name##_store(struct config_item *item,	\
    const char *page, size_t len)	\
    {									\
    struct f_uac2_opts *opts = to_f_uac2_opts(item);		\
    int ret = len;							\
    \
    mutex_lock(&opts.lock);					\
    if (opts.refcnt) {						\
    ret = -EBUSY;						\
    goto end;						\
    }								\
    \
    if (len && page[len - 1] == '\n')				\
    len--;							\
    \
    scnprintf(opts.name, min(sizeof(opts.name), len + 1),		\
    "%s", page);						\
    \
    end:									\
    mutex_unlock(&opts.lock);					\
    return ret;							\
    }									\
    \
    CONFIGFS_ATTR(f_uac2_opts_, name)
    UAC2_ATTRIBUTE(u32, p_chmask);
    UAC2_RATE_ATTRIBUTE(p_srate);
    UAC2_ATTRIBUTE(u32, p_ssize);
    UAC2_ATTRIBUTE(u8, p_hs_bint);
    UAC2_ATTRIBUTE(u32, c_chmask);
    UAC2_RATE_ATTRIBUTE(c_srate);
    UAC2_ATTRIBUTE_SYNC(c_sync);
    UAC2_ATTRIBUTE(u32, c_ssize);
    UAC2_ATTRIBUTE(u8, c_hs_bint);
    UAC2_ATTRIBUTE(u32, req_number);
    UAC2_ATTRIBUTE(bool, p_mute_present);
    UAC2_ATTRIBUTE(bool, p_volume_present);
    UAC2_ATTRIBUTE(s16, p_volume_min);
    UAC2_ATTRIBUTE(s16, p_volume_max);
    UAC2_ATTRIBUTE(s16, p_volume_res);
    UAC2_ATTRIBUTE(bool, c_mute_present);
    UAC2_ATTRIBUTE(bool, c_volume_present);
    UAC2_ATTRIBUTE(s16, c_volume_min);
    UAC2_ATTRIBUTE(s16, c_volume_max);
    UAC2_ATTRIBUTE(s16, c_volume_res);
    UAC2_ATTRIBUTE(u32, fb_max);
    UAC2_ATTRIBUTE_STRING(function_name);
    UAC2_ATTRIBUTE_STRING(if_ctrl_name);
    UAC2_ATTRIBUTE_STRING(clksrc_in_name);
    UAC2_ATTRIBUTE_STRING(clksrc_out_name);
    UAC2_ATTRIBUTE_STRING(p_it_name);
    UAC2_ATTRIBUTE_STRING(p_it_ch_name);
    UAC2_ATTRIBUTE_STRING(p_ot_name);
    UAC2_ATTRIBUTE_STRING(p_fu_vol_name);
    UAC2_ATTRIBUTE_STRING(c_it_name);
    UAC2_ATTRIBUTE_STRING(c_it_ch_name);
    UAC2_ATTRIBUTE_STRING(c_ot_name);
    UAC2_ATTRIBUTE_STRING(c_fu_vol_name);
    UAC2_ATTRIBUTE(s16, p_terminal_type);
    UAC2_ATTRIBUTE(s16, c_terminal_type);
    static struct configfs_attribute *f_uac2_attrs[] = {
    &f_uac2_opts_attr_p_chmask,
    &f_uac2_opts_attr_p_srate,
    &f_uac2_opts_attr_p_ssize,
    &f_uac2_opts_attr_p_hs_bint,
    &f_uac2_opts_attr_c_chmask,
    &f_uac2_opts_attr_c_srate,
    &f_uac2_opts_attr_c_ssize,
    &f_uac2_opts_attr_c_hs_bint,
    &f_uac2_opts_attr_c_sync,
    &f_uac2_opts_attr_req_number,
    &f_uac2_opts_attr_fb_max,
    &f_uac2_opts_attr_p_mute_present,
    &f_uac2_opts_attr_p_volume_present,
    &f_uac2_opts_attr_p_volume_min,
    &f_uac2_opts_attr_p_volume_max,
    &f_uac2_opts_attr_p_volume_res,
    &f_uac2_opts_attr_c_mute_present,
    &f_uac2_opts_attr_c_volume_present,
    &f_uac2_opts_attr_c_volume_min,
    &f_uac2_opts_attr_c_volume_max,
    &f_uac2_opts_attr_c_volume_res,
    &f_uac2_opts_attr_function_name,
    &f_uac2_opts_attr_if_ctrl_name,
    &f_uac2_opts_attr_clksrc_in_name,
    &f_uac2_opts_attr_clksrc_out_name,
    &f_uac2_opts_attr_p_it_name,
    &f_uac2_opts_attr_p_it_ch_name,
    &f_uac2_opts_attr_p_ot_name,
    &f_uac2_opts_attr_p_fu_vol_name,
    &f_uac2_opts_attr_c_it_name,
    &f_uac2_opts_attr_c_it_ch_name,
    &f_uac2_opts_attr_c_ot_name,
    &f_uac2_opts_attr_c_fu_vol_name,
    &f_uac2_opts_attr_p_terminal_type,
    &f_uac2_opts_attr_c_terminal_type,
    core::ptr::null_mut(),
    };
    static const struct config_item_type f_uac2_func_type = {
    .ct_item_ops	= &f_uac2_item_ops,
    .ct_attrs	= f_uac2_attrs,
    .ct_owner	= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn afunc_free_inst(f: *mut usb_function_instance) {
    static void afunc_free_inst(struct usb_function_instance *f)
    {
    struct f_uac2_opts *opts;
    opts = container_of(f, struct f_uac2_opts, func_inst);
    kfree(opts);
    }
    static struct usb_function_instance *afunc_alloc_inst(void)
    {
    struct f_uac2_opts *opts;
    opts = kzalloc_obj(*opts);
    if (!opts)
    return ERR_PTR(-ENOMEM);
    mutex_init(&opts.lock);
    opts.func_inst.free_func_inst = afunc_free_inst;
    config_group_init_type_name(&opts.func_inst.group, "",
    &f_uac2_func_type);
    opts.p_chmask = UAC2_DEF_PCHMASK;
    opts.p_srates[0] = UAC2_DEF_PSRATE;
    opts.p_ssize = UAC2_DEF_PSSIZE;
    opts.p_hs_bint = UAC2_DEF_PHSBINT;
    opts.c_chmask = UAC2_DEF_CCHMASK;
    opts.c_srates[0] = UAC2_DEF_CSRATE;
    opts.c_ssize = UAC2_DEF_CSSIZE;
    opts.c_hs_bint = UAC2_DEF_CHSBINT;
    opts.c_sync = UAC2_DEF_CSYNC;
    opts.p_mute_present = UAC2_DEF_MUTE_PRESENT;
    opts.p_volume_present = UAC2_DEF_VOLUME_PRESENT;
    opts.p_volume_min = UAC2_DEF_MIN_DB;
    opts.p_volume_max = UAC2_DEF_MAX_DB;
    opts.p_volume_res = UAC2_DEF_RES_DB;
    opts.c_mute_present = UAC2_DEF_MUTE_PRESENT;
    opts.c_volume_present = UAC2_DEF_VOLUME_PRESENT;
    opts.c_volume_min = UAC2_DEF_MIN_DB;
    opts.c_volume_max = UAC2_DEF_MAX_DB;
    opts.c_volume_res = UAC2_DEF_RES_DB;
    opts.req_number = UAC2_DEF_REQ_NUM;
    opts.fb_max = FBACK_FAST_MAX;
    scnprintf(opts.function_name, sizeof(opts.function_name), "Source/Sink");
    scnprintf(opts.if_ctrl_name, sizeof(opts.if_ctrl_name), "Topology Control");
    scnprintf(opts.clksrc_in_name, sizeof(opts.clksrc_in_name), "Input Clock");
    scnprintf(opts.clksrc_out_name, sizeof(opts.clksrc_out_name), "Output Clock");
    scnprintf(opts.p_it_name, sizeof(opts.p_it_name), "USBD Out");
    scnprintf(opts.p_it_ch_name, sizeof(opts.p_it_ch_name), "Capture Channels");
    scnprintf(opts.p_ot_name, sizeof(opts.p_ot_name), "USBH In");
    scnprintf(opts.p_fu_vol_name, sizeof(opts.p_fu_vol_name), "Capture Volume");
    scnprintf(opts.c_it_name, sizeof(opts.c_it_name), "USBH Out");
    scnprintf(opts.c_it_ch_name, sizeof(opts.c_it_ch_name), "Playback Channels");
    scnprintf(opts.c_ot_name, sizeof(opts.c_ot_name), "USBD In");
    scnprintf(opts.c_fu_vol_name, sizeof(opts.c_fu_vol_name), "Playback Volume");
    opts.p_terminal_type = UAC2_DEF_P_TERM_TYPE;
    opts.c_terminal_type = UAC2_DEF_C_TERM_TYPE;
    return &opts.func_inst;
    }
#[no_mangle]
unsafe extern "C" fn afunc_free(f: *mut usb_function) {
    static void afunc_free(struct usb_function *f)
    {
    struct g_audio *agdev;
    struct f_uac2_opts *opts;
    agdev = func_to_g_audio(f);
    opts = container_of(f.fi, struct f_uac2_opts, func_inst);
    kfree(agdev);
    mutex_lock(&opts.lock);
    --opts.refcnt;
    mutex_unlock(&opts.lock);
    }
#[no_mangle]
unsafe extern "C" fn afunc_unbind(c: *mut usb_configuration, f: *mut usb_function) {
    static void afunc_unbind(struct usb_configuration *c, struct usb_function *f)
    {
    struct g_audio *agdev = func_to_g_audio(f);
    g_audio_cleanup(agdev);
    usb_free_all_descriptors(f);
    agdev.gadget = core::ptr::null_mut();
    kfree(out_feature_unit_desc);
    out_feature_unit_desc = core::ptr::null_mut();
    kfree(in_feature_unit_desc);
    in_feature_unit_desc = core::ptr::null_mut();
    }
    static struct usb_function *afunc_alloc(struct usb_function_instance *fi)
    {
    struct f_uac2	*uac2;
    struct f_uac2_opts *opts;
    uac2 = kzalloc_obj(*uac2);
    if (uac2 == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    opts = container_of(fi, struct f_uac2_opts, func_inst);
    mutex_lock(&opts.lock);
    ++opts.refcnt;
    mutex_unlock(&opts.lock);
    uac2.g_audio.func.name = "uac2_func";
    uac2.g_audio.func.bind = afunc_bind;
    uac2.g_audio.func.unbind = afunc_unbind;
    uac2.g_audio.func.set_alt = afunc_set_alt;
    uac2.g_audio.func.get_alt = afunc_get_alt;
    uac2.g_audio.func.disable = afunc_disable;
    uac2.g_audio.func.suspend = afunc_suspend;
    uac2.g_audio.func.setup = afunc_setup;
    uac2.g_audio.func.free_func = afunc_free;
    return &uac2.g_audio.func;
    }
    DECLARE_USB_FUNCTION_INIT(uac2, afunc_alloc_inst, afunc_alloc);
    MODULE_DESCRIPTION("USB Audio Class 2.0 Function");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Yadwinder Singh");
    MODULE_AUTHOR("Jaswinder Singh");
    MODULE_AUTHOR("Ruslan Bilovol");
