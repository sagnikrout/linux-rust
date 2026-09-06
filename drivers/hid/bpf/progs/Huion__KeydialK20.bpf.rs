//! Automatically rewritten from C to Rust
//! Source: drivers/hid/bpf/progs/Huion__KeydialK20.bpf.c
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
// Copyright (c) 2024 Red Hat, Inc
//

pub const VID_HUION: c_uint = 0x256C;
pub const PID_KEYDIAL_K20: c_uint = 0x0069;
    HID_BPF_CONFIG(
    HID_DEVICE(BUS_USB, HID_GROUP_GENERIC, VID_HUION, PID_KEYDIAL_K20),
    );
// Filled in by udev-hid-bpf
    char UDEV_PROP_HUION_FIRMWARE_ID[64];
// The prefix of the firmware ID we expect for this device. The full firmware
// string has a date suffix, e.g. HUION_T21h_230511
//
    char EXPECTED_FIRMWARE_ID[] = "HUION_T21h_";
// How this BPF program works: the tablet has two modes, firmware mode and
// tablet mode. In firmware mode (out of the box) the tablet sends button events
// as keyboard shortcuts and the dial as wheel but it's not forwarded by the kernel.
// In tablet mode it uses a vendor specific hid report to report everything instead.
// Depending on the mode some hid reports are never sent and the corresponding
// devices are mute.
//
// To switch the tablet use e.g.  https://github.com/whot/huion-switcher
// or one of the tools from the digimend project
//
// This BPF currently works for both modes only. The huion-switcher tool sets the
// HUION_FIRMWARE_ID udev property - if that is set then we disable the firmware
// pad and pen reports (by making them vendor collections that are ignored).
// If that property is not set we fix all hidraw nodes so the tablet works in
// either mode though the drawback is that the device will show up twice if
// you bind it to all event nodes
//
// Default report descriptor for the first exposed hidraw node:
//
// # HUION Huion Keydial_K20
// # Report descriptor length: 18 bytes
// # 0x06, 0x00, 0xff,              // Usage Page (Vendor Defined Page 0xFF00)   0
// # 0x09, 0x01,                    // Usage (Vendor Usage 0x01)                 3
// # 0xa1, 0x01,                    // Collection (Application)                  5
// # 0x85, 0x08,                    //   Report ID (8)                           7
// # 0x75, 0x58,                    //   Report Size (88)                        9
// # 0x95, 0x01,                    //   Report Count (1)                        11
// # 0x09, 0x01,                    //   Usage (Vendor Usage 0x01)               13
// # 0x81, 0x02,                    //   Input (Data,Var,Abs)                    15
// # 0xc0,                          // End Collection                            17
// R: 18 06 00 ff 09 01 a1 01 85 08 75 58 95 01 09 01 81 02 c0
//
// This report descriptor appears to be identical for all Huion devices.
//
// Second hidraw node is the Pad. This one sends the button events until the tablet is
// switched to raw mode, then it's mute.
//
// # HUION Huion Keydial_K20
// # Report descriptor length: 135 bytes
// # 0x05, 0x01,                    // Usage Page (Generic Desktop)              0
// # 0x09, 0x06,                    // Usage (Keyboard)                          2
// # 0xa1, 0x01,                    // Collection (Application)                  4
// # 0x85, 0x03,                    //   Report ID (3)                           6
// # 0x05, 0x07,                    //   Usage Page (Keyboard/Keypad)            8
// # 0x19, 0xe0,                    //   UsageMinimum (224)                      10
// # 0x29, 0xe7,                    //   UsageMaximum (231)                      12
// # 0x15, 0x00,                    //   Logical Minimum (0)                     14
// # 0x25, 0x01,                    //   Logical Maximum (1)                     16
// # 0x75, 0x01,                    //   Report Size (1)                         18
// # 0x95, 0x08,                    //   Report Count (8)                        20
// # 0x81, 0x02,                    //   Input (Data,Var,Abs)                    22
// # 0x05, 0x07,                    //   Usage Page (Keyboard/Keypad)            24
// # 0x19, 0x00,                    //   UsageMinimum (0)                        26
// # 0x29, 0xff,                    //   UsageMaximum (255)                      28
// # 0x26, 0xff, 0x00,              //   Logical Maximum (255)                   30
// # 0x75, 0x08,                    //   Report Size (8)                         33
// # 0x95, 0x06,                    //   Report Count (6)                        35
// # 0x81, 0x00,                    //   Input (Data,Arr,Abs)                    37
// # 0xc0,                          // End Collection                            39
// # 0x05, 0x0c,                    // Usage Page (Consumer)                     40
// # 0x09, 0x01,                    // Usage (Consumer Control)                  42
// # 0xa1, 0x01,                    // Collection (Application)                  44
// # 0x85, 0x04,                    //   Report ID (4)                           46
// # 0x05, 0x0c,                    //   Usage Page (Consumer)                   48
// # 0x19, 0x00,                    //   UsageMinimum (0)                        50
// # 0x2a, 0x80, 0x03,              //   UsageMaximum (896)                      52
// # 0x15, 0x00,                    //   Logical Minimum (0)                     55
// # 0x26, 0x80, 0x03,              //   Logical Maximum (896)                   57
// # 0x75, 0x10,                    //   Report Size (16)                        60
// # 0x95, 0x01,                    //   Report Count (1)                        62
// # 0x81, 0x00,                    //   Input (Data,Arr,Abs)                    64
// # 0xc0,                          // End Collection                            66
// # 0x05, 0x01,                    // Usage Page (Generic Desktop)              67
// # 0x09, 0x02,                    // Usage (Mouse)                             69
// # 0xa1, 0x01,                    // Collection (Application)                  71
// # 0x09, 0x01,                    //   Usage (Pointer)                         73
// # 0x85, 0x05,                    //   Report ID (5)                           75
// # 0xa1, 0x00,                    //   Collection (Physical)                   77
// # 0x05, 0x09,                    //     Usage Page (Button)                   79
// # 0x19, 0x01,                    //     UsageMinimum (1)                      81
// # 0x29, 0x05,                    //     UsageMaximum (5)                      83
// # 0x15, 0x00,                    //     Logical Minimum (0)                   85
// # 0x25, 0x01,                    //     Logical Maximum (1)                   87
// # 0x95, 0x05,                    //     Report Count (5)                      89
// # 0x75, 0x01,                    //     Report Size (1)                       91
// # 0x81, 0x02,                    //     Input (Data,Var,Abs)                  93
// # 0x95, 0x01,                    //     Report Count (1)                      95
// # 0x75, 0x03,                    //     Report Size (3)                       97
// # 0x81, 0x01,                    //     Input (Cnst,Arr,Abs)                  99
// # 0x05, 0x01,                    //     Usage Page (Generic Desktop)          101
// # 0x09, 0x30,                    //     Usage (X)                             103
// # 0x09, 0x31,                    //     Usage (Y)                             105
// # 0x16, 0x00, 0x80,              //     Logical Minimum (-32768)              107
// # 0x26, 0xff, 0x7f,              //     Logical Maximum (32767)               110
// # 0x75, 0x10,                    //     Report Size (16)                      113
// # 0x95, 0x02,                    //     Report Count (2)                      115
// # 0x81, 0x06,                    //     Input (Data,Var,Rel)                  117
// # 0x95, 0x01,                    //     Report Count (1)                      119
// # 0x75, 0x08,                    //     Report Size (8)                       121
// # 0x05, 0x01,                    //     Usage Page (Generic Desktop)          123
// # 0x09, 0x38,                    //     Usage (Wheel)                         125
// # 0x15, 0x81,                    //     Logical Minimum (-127)                127
// # 0x25, 0x7f,                    //     Logical Maximum (127)                 129
// # 0x81, 0x06,                    //     Input (Data,Var,Rel)                  131
// # 0xc0,                          //   End Collection                          133
// # 0xc0,                          // End Collection                            134
// R: 135 05 01 09 06 a1 01 85 03 05 07 19 e0 29 e7 15 00 25 01 75 01 95 08 81 02 05 07 19 00 29 ff 26 ff 00 75 08 95 06 81 00 c0 05 0c 09 01 a1 01 85 04 05 0c 19 00 2a 80 03 15 00 26 80 03 75 10 95 01 81 00 c0 05 01 09 02 a1 01 09 01 85 05 a1 00 05 09 19 01 29 05 15 00 25 01 95 05 75 01 81 02 95 01 75 03 81 01 05 01 09 30 09 31 16 00 80 26 ff 7f 7510 95 02 81 06 95 01 75 08 05 01 09 38 15 81 25 7f 81 06 c0 c0
//
// Third hidraw node is a multi-axis controller which sends the dial events
// and the button inside the dial. If the tablet is switched to raw mode it is mute.
//
// # HUION Huion Keydial_K20
// # Report descriptor length: 108 bytes
// # 0x05, 0x01,                    // Usage Page (Generic Desktop)              0
// # 0x09, 0x0e,                    // Usage (System Multi-Axis Controller)      2
// # 0xa1, 0x01,                    // Collection (Application)                  4
// # 0x85, 0x11,                    //   Report ID (17)                          6
// # 0x05, 0x0d,                    //   Usage Page (Digitizers)                 8
// # 0x09, 0x21,                    //   Usage (Puck)                            10
// # 0xa1, 0x02,                    //   Collection (Logical)                    12
// # 0x15, 0x00,                    //     Logical Minimum (0)                   14
// # 0x25, 0x01,                    //     Logical Maximum (1)                   16
// # 0x75, 0x01,                    //     Report Size (1)                       18
// # 0x95, 0x01,                    //     Report Count (1)                      20
// # 0xa1, 0x00,                    //     Collection (Physical)                 22
// # 0x05, 0x09,                    //       Usage Page (Button)                 24
// # 0x09, 0x01,                    //       Usage (Button 1)                    26
// # 0x81, 0x02,                    //       Input (Data,Var,Abs)                28
// # 0x05, 0x0d,                    //       Usage Page (Digitizers)             30
// # 0x09, 0x33,                    //       Usage (Touch)                       32
// # 0x81, 0x02,                    //       Input (Data,Var,Abs)                34
// # 0x95, 0x06,                    //       Report Count (6)                    36
// # 0x81, 0x03,                    //       Input (Cnst,Var,Abs)                38
// # 0xa1, 0x02,                    //       Collection (Logical)                40
// # 0x05, 0x01,                    //         Usage Page (Generic Desktop)      42
// # 0x09, 0x37,                    //         Usage (Dial)                      44
// # 0x16, 0x00, 0x80,              //         Logical Minimum (-32768)          46
// # 0x26, 0xff, 0x7f,              //         Logical Maximum (32767)           49
// # 0x75, 0x10,                    //         Report Size (16)                  52
// # 0x95, 0x01,                    //         Report Count (1)                  54
// # 0x81, 0x06,                    //         Input (Data,Var,Rel)              56
// # 0x35, 0x00,                    //         Physical Minimum (0)              58
// # 0x46, 0x10, 0x0e,              //         Physical Maximum (3600)           60
// # 0x15, 0x00,                    //         Logical Minimum (0)               63
// # 0x26, 0x10, 0x0e,              //         Logical Maximum (3600)            65
// # 0x09, 0x48,                    //         Usage (Resolution Multiplier)     68
// # 0xb1, 0x02,                    //         Feature (Data,Var,Abs)            70
// # 0x45, 0x00,                    //         Physical Maximum (0)              72
// # 0xc0,                          //       End Collection                      74
// # 0x75, 0x08,                    //       Report Size (8)                     75
// # 0x95, 0x01,                    //       Report Count (1)                    77
// # 0x81, 0x01,                    //       Input (Cnst,Arr,Abs)                79
// # 0x75, 0x08,                    //       Report Size (8)                     81
// # 0x95, 0x01,                    //       Report Count (1)                    83
// # 0x81, 0x01,                    //       Input (Cnst,Arr,Abs)                85
// # 0x75, 0x08,                    //       Report Size (8)                     87
// # 0x95, 0x01,                    //       Report Count (1)                    89
// # 0x81, 0x01,                    //       Input (Cnst,Arr,Abs)                91
// # 0x75, 0x08,                    //       Report Size (8)                     93
// # 0x95, 0x01,                    //       Report Count (1)                    95
// # 0x81, 0x01,                    //       Input (Cnst,Arr,Abs)                97
// # 0x75, 0x08,                    //       Report Size (8)                     99
// # 0x95, 0x01,                    //       Report Count (1)                    101
// # 0x81, 0x01,                    //       Input (Cnst,Arr,Abs)                103
// # 0xc0,                          //     End Collection                        105
// # 0xc0,                          //   End Collection                          106
// # 0xc0,                          // End Collection                            107
// R: 108 05 01 09 0e a1 01 85 11 05 0d 09 21 a1 02 15 00 25 01 75 01 95 01 a1 00 05 09 09 01 81 02 05 0d 09 33 81 02 95 06 81 03 a1 02 05 01 09 37 16 00 80 26 ff 7f 75 10 95 01 81 06 35 00 46 10 0e 15 00 26 10 0e 09 48 b1 02 45 00 c0 75 08 95 01 81 01 75 08 95 01 81 01 75 08 95 01 81 01 75 08 95 01 81 01 75 08 95 01 81 01 c0 c0 c0
//
pub const PAD_REPORT_DESCRIPTOR_LENGTH: c_int = 135;
pub const PUCK_REPORT_DESCRIPTOR_LENGTH: c_int = 108;
pub const VENDOR_REPORT_DESCRIPTOR_LENGTH: c_int = 18;
pub const PAD_KBD_REPORT_ID: c_int = 3;

pub const PUCK_REPORT_ID: c_int = 17;
pub const VENDOR_REPORT_ID: c_int = 8;
pub const PAD_KBD_REPORT_LENGTH: c_int = 8;
pub const PAD_CC_REPORT_LENGTH: c_int = 3;
pub const PAD_MOUSE_REPORT_LENGTH: c_int = 7;
pub const PUCK_REPORT_LENGTH: c_int = 9;
pub const VENDOR_REPORT_LENGTH: c_int = 12;
    __u32 last_button_state;
    static const __u8 disabled_rdesc_puck[] = {
    FixedSizeVendorReport(PUCK_REPORT_LENGTH)
    };
    static const __u8 disabled_rdesc_pad[] = {
    FixedSizeVendorReport(PAD_KBD_REPORT_LENGTH)
    FixedSizeVendorReport(PAD_CC_REPORT_LENGTH)
    FixedSizeVendorReport(PAD_MOUSE_REPORT_LENGTH)
    };
    static const __u8 fixed_rdesc_vendor[] = {
    UsagePage_GenericDesktop
    Usage_GD_Keypad
    CollectionApplication(
// Byte 0
// We send our pad events on the vendor report id because why not
    ReportId(VENDOR_REPORT_ID)
    UsagePage_Digitizers
    Usage_Dig_TabletFunctionKeys
    CollectionPhysical(
// Byte 1 is a button so we look like a tablet
    Usage_Dig_BarrelSwitch	 // BTN_STYLUS, needed so we get to be a tablet pad
    ReportCount(1)
    ReportSize(1)
    Input(Var|Abs)
    ReportCount(7) // Padding
    Input(Const)
// Bytes 2/3 - x/y just exist so we get to be a tablet pad
    UsagePage_GenericDesktop
    Usage_GD_X
    Usage_GD_Y
    LogicalMinimum_i8(0x0)
    LogicalMaximum_i8(0x1)
    ReportCount(2)
    ReportSize(8)
    Input(Var|Abs)
// Bytes 4-7 are the button state for 19 buttons + pad out to u32
// We send the first 10 buttons as buttons 1-10 which is BTN_0 -> BTN_9
    UsagePage_Button
    UsageMinimum_i8(1)
    UsageMaximum_i8(10)
    LogicalMinimum_i8(0x0)
    LogicalMaximum_i8(0x1)
    ReportCount(10)
    ReportSize(1)
    Input(Var|Abs)
// We send the other 9 buttons as buttons 0x31 and above -> BTN_A - BTN_TL2
    UsageMinimum_i8(0x31)
    UsageMaximum_i8(0x3a)
    ReportCount(9)
    ReportSize(1)
    Input(Var|Abs)
    ReportCount(13)
    ReportSize(1)
    Input(Const) // padding
// Byte 6 is the wheel
    UsagePage_GenericDesktop
    Usage_GD_Wheel
    LogicalMinimum_i8(-1)
    LogicalMaximum_i8(1)
    ReportCount(1)
    ReportSize(8)
    Input(Var|Rel)
    )
// Make sure we match our original report length
    FixedSizeVendorReport(VENDOR_REPORT_LENGTH)
    )
    };
// Identical to fixed_rdesc_pad but with different FixedSizeVendorReport
    static const __u8 fixed_rdesc_pad[] = {
    UsagePage_GenericDesktop
    Usage_GD_Keypad
    CollectionApplication(
// Byte 0
// We send our pad events on the vendor report id because why not
    ReportId(VENDOR_REPORT_ID)
    UsagePage_Digitizers
    Usage_Dig_TabletFunctionKeys
    CollectionPhysical(
// Byte 1 is a button so we look like a tablet
    Usage_Dig_BarrelSwitch	 // BTN_STYLUS, needed so we get to be a tablet pad
    ReportCount(1)
    ReportSize(1)
    Input(Var|Abs)
    ReportCount(7) // Padding
    Input(Const)
// Bytes 2/3 - x/y just exist so we get to be a tablet pad
    UsagePage_GenericDesktop
    Usage_GD_X
    Usage_GD_Y
    LogicalMinimum_i8(0x0)
    LogicalMaximum_i8(0x1)
    ReportCount(2)
    ReportSize(8)
    Input(Var|Abs)
// Bytes 4-7 are the button state for 19 buttons + pad out to u32
// We send the first 10 buttons as buttons 1-10 which is BTN_0 -> BTN_9
    UsagePage_Button
    UsageMinimum_i8(1)
    UsageMaximum_i8(10)
    LogicalMinimum_i8(0x0)
    LogicalMaximum_i8(0x1)
    ReportCount(10)
    ReportSize(1)
    Input(Var|Abs)
// We send the other 9 buttons as buttons 0x31 and above -> BTN_A - BTN_TL2
    UsageMinimum_i8(0x31)
    UsageMaximum_i8(0x3a)
    ReportCount(9)
    ReportSize(1)
    Input(Var|Abs)
    ReportCount(13)
    ReportSize(1)
    Input(Const) // padding
// Byte 6 is the wheel
    UsagePage_GenericDesktop
    Usage_GD_Wheel
    LogicalMinimum_i8(-1)
    LogicalMaximum_i8(1)
    ReportCount(1)
    ReportSize(8)
    Input(Var|Rel)
    )
// Make sure we match our original report lengths
    FixedSizeVendorReport(PAD_KBD_REPORT_LENGTH)
    FixedSizeVendorReport(PAD_CC_REPORT_LENGTH)
    FixedSizeVendorReport(PAD_MOUSE_REPORT_LENGTH)
    )
    };
    SEC(HID_BPF_RDESC_FIXUP)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: k20_fix_rdesc, hctx: *mut hid_bpf_ctx) -> c_int {
    int BPF_PROG(k20_fix_rdesc, struct hid_bpf_ctx *hctx)
    {
    __u8 *data = hid_bpf_get_data(hctx, 0 /* offset */, HID_MAX_DESCRIPTOR_SIZE /* size */);
    let mut rdesc_size: __s32 = hctx.size;
    __u8 have_fw_id;
    if (!data)
    return 0; /* EPERM check */
// If we have a firmware ID and it matches our expected prefix, we
// disable the default pad/puck nodes. They won't send events
// but cause duplicate devices.
//
    have_fw_id = __builtin_memcmp(UDEV_PROP_HUION_FIRMWARE_ID,
    EXPECTED_FIRMWARE_ID,
    sizeof(EXPECTED_FIRMWARE_ID) - 1) == 0;
    if (rdesc_size == PAD_REPORT_DESCRIPTOR_LENGTH) {
    if (have_fw_id) {
    __builtin_memcpy(data, disabled_rdesc_pad, sizeof(disabled_rdesc_pad));
    return sizeof(disabled_rdesc_pad);
    } else {
    __builtin_memcpy(data, fixed_rdesc_pad, sizeof(fixed_rdesc_pad));
    return sizeof(fixed_rdesc_pad);
    }
    }
    if (rdesc_size == PUCK_REPORT_DESCRIPTOR_LENGTH) {
    if (have_fw_id) {
    __builtin_memcpy(data, disabled_rdesc_puck, sizeof(disabled_rdesc_puck));
    return sizeof(disabled_rdesc_puck);
    }
    }
// Always fix the vendor mode so the tablet will work even if nothing sets
// the udev property (e.g. huion-switcher run manually)
//
    if (rdesc_size == VENDOR_REPORT_DESCRIPTOR_LENGTH) {
    __builtin_memcpy(data, fixed_rdesc_vendor, sizeof(fixed_rdesc_vendor));
    return sizeof(fixed_rdesc_vendor);
    }
    return 0;
    }
    SEC(HID_BPF_DEVICE_EVENT)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: k20_fix_events, hctx: *mut hid_bpf_ctx) -> c_int {
    int BPF_PROG(k20_fix_events, struct hid_bpf_ctx *hctx)
    {
    __u8 *data = hid_bpf_get_data(hctx, 0 /* offset */, 10 /* size */);
    if (!data)
    return 0; /* EPERM check */
// Only sent if tablet is in raw mode
    if (data[0] == VENDOR_REPORT_ID) {
// See fixed_rdesc_pad
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pad_report {
    pub report_id: __u8,
    pub btn_stylus:1: __u8,
    pub pad:7: __u8,
    pub x: __u8,
    pub y: __u8,
    pub buttons: __u32,
    pub wheel: __u8,
// C attribute field omitted
    pub 0: __u8 wheel =,
// Wheel report
    if (data[1] == 0xf1) {
    if (data[5] == 2)
    pub 0xff: wheel =,
    else
    pub data: [wheel =; 5],
    } else {
// data[4..6] are the buttons, mapped correctly
    pub 16): last_button_state = data[4] | (data[5] << 8) | (data[6] <<,
    pub wheel: wheel = 0; //,
    }
    pub )data: *mut pad_report = (struct pad_report,
    pub VENDOR_REPORT_ID: pad_report->report_id =,
    pub 0: pad_report->btn_stylus =,
    pub 0: pad_report->x =,
    pub 0: pad_report->y =,
    pub last_button_state: pad_report->buttons =,
    pub wheel: pad_report->wheel =,
    pub pad_report): return sizeof(struct,
    }
    if (data[0] == PAD_KBD_REPORT_ID) {
    const __u8 button_mapping[] = {
    0x0e, /* Button 1:  K */
    0x0a, /* Button 2:  G */
    0x0f, /* Button 3:  L */
    0x4c, /* Button 4:  Delete */
    0x0c, /* Button 5:  I */
    0x07, /* Button 6:  D */
    0x05, /* Button 7:  B */
    0x08, /* Button 8:  E */
    0x16, /* Button 9:  S */
    0x1d, /* Button 10: Z */
    0x06, /* Button 11: C */
    0x19, /* Button 12: V */
    0xff, /* Button 13: LeftControl */
    0xff, /* Button 14: LeftAlt */
    0xff, /* Button 15: LeftShift */
    0x28, /* Button 16: Return Enter */
    0x2c, /* Button 17: Spacebar */
    0x11, /* Button 18: N */
}

// See fixed_rdesc_pad
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pad_report {
    pub report_id: __u8,
    pub btn_stylus:1: __u8,
    pub pad:7: __u8,
    pub x: __u8,
    pub y: __u8,
    pub buttons: __u32,
    pub wheel: __u8,
// C attribute field omitted
    pub i: c_int,
    pub b: usize,
    pub data: [__u8 modifiers =; 1],
    pub 0: __u32 buttons =,
    if (modifiers & 0x01) { /* Control */
    pub BIT(12): buttons |=,
    }
    if (modifiers & 0x02) { /* Shift */
    pub BIT(14): buttons |=,
    }
    if (modifiers & 0x04) { /* Alt */
    pub BIT(13): buttons |=,
    }
    pub {: for (i = 2; i < PAD_KBD_REPORT_LENGTH; i++),
    if (!data[i])
    pub {: for (b = 0; b < ARRAY_SIZE(button_mapping); b++),
    if (data[i] == button_mapping[b]) {
    pub BIT(b): buttons |=,
    }
    }
    pub 0: data[i] =,
    }
    pub )data: *mut pad_report = (struct pad_report,
    pub VENDOR_REPORT_ID: pad_report->report_id =,
    pub 0: pad_report->btn_stylus =,
    pub 0: pad_report->x =,
    pub 0: pad_report->y =,
    pub buttons: pad_report->buttons =,
// The wheel happens on a different hidraw node but its
// values are unreliable (as is the button inside the wheel).
// So the wheel is simply always zero, if you want the wheel
// to work reliably, use the tablet mode.
    pub 0: pad_report->wheel =,
    pub pad_report): return sizeof(struct,
    }
    pub 0: return,
    }
    HID_BPF_OPS(keydial_k20) = {
    .hid_device_event = (void *)k20_fix_events,
    .hid_rdesc_fixup = (void *)k20_fix_rdesc,
}

    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut hid_bpf_probe_args) -> c_int {
    int probe(struct hid_bpf_probe_args *ctx)
    {
    switch (ctx.rdesc_size) {
    case PAD_REPORT_DESCRIPTOR_LENGTH:
    case PUCK_REPORT_DESCRIPTOR_LENGTH:
    case VENDOR_REPORT_DESCRIPTOR_LENGTH:
    ctx.retval = 0;
    break;
    default:
    ctx.retval = -EINVAL;
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
