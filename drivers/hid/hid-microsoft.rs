//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-microsoft.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// HID driver for some microsoft "special" devices
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2006-2007 Jiri Kosina
// Copyright (c) 2008 Jiri Slaby
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_data {
    pub quirks: c_ulong,
    pub hdev: *mut hid_device,
    pub ff_worker: work_struct,
    pub strong: __u8,
    pub weak: __u8,
    pub output_report_dmabuf: *mut c_void,
}

pub const XB1S_FF_REPORT: c_int = 3;

    enum {
    MAGNITUDE_STRONG = 2,
    MAGNITUDE_WEAK,
    MAGNITUDE_NUM
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xb1s_ff_report {
    pub report_id: __u8,
    pub enable: __u8,
    pub magnitude: [__u8; MAGNITUDE_NUM],
    pub duration_10ms: __u8,
    pub start_delay_10ms: __u8,
    pub loop_count: __u8,
    pub __packed: },
    static const __u8 *ms_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    pub hid_get_drvdata(hdev): *mut *mut ms_data ms =,
    pub ms->quirks: unsigned long quirks =,
//
// Microsoft Wireless Desktop Receiver (Model 1028) has
// 'Usage Min/Max' where it ought to have 'Physical Min/Max'
//
    if ((quirks & MS_RDESC) && *rsize == 571 && rdesc[557] == 0x19 &&
    rdesc[559] == 0x29) {
    pub descriptor\n"): hid_info(hdev, "fixing up Microsoft Wireless Receiver Model 1028 report,
    pub 0x35: rdesc[557] =,
    pub 0x45: rdesc[559] =,
    }
    pub rdesc: return,
    }

    EV_KEY, (c))
    static int ms_ergonomy_kb_quirk(struct hid_input *hi, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    pub hi->input: *mut *mut input_dev input =,
    if ((usage.hid & HID_USAGE_PAGE) == HID_UP_CONSUMER) {
    switch (usage.hid & HID_USAGE) {
//
// Microsoft uses these 2 reserved usage ids for 2 keys on
// the MS office kb labelled "Office Home" and "Task Pane".
//
    case 0x29d:
    pub 1: return,
    case 0x29e:
    pub 1: return,
    }
    pub 0: return,
    }
    if ((usage.hid & HID_USAGE_PAGE) != HID_UP_MSVENDOR)
    pub 0: return,
    switch (usage.hid & HID_USAGE) {
    pub break: case 0xfd06: ms_map_key_clear(KEY_CHAT);,
    pub break: case 0xfd07: ms_map_key_clear(KEY_PHONE);,
    case 0xff00:
// Special keypad keys
    pub input->keybit): set_bit(KEY_KPLEFTPAREN,,
    pub input->keybit): set_bit(KEY_KPRIGHTPAREN,,
    case 0xff01:
// Scroll wheel
    pub REL_WHEEL): hid_map_usage_clear(hi, usage, bit, max, EV_REL,,
    case 0xff02:
//
// This byte contains a copy of the modifier keys byte of a
// standard hid keyboard report, as send by interface 0
// (this usage is found on interface 1).
//
// This byte only gets send when another key in the same report
// changes state, and as such is useless, ignore it.
//
    pub -1: return,
    case 0xff05:
    pub input->evbit): set_bit(EV_REP,,
    pub input->keybit): set_bit(KEY_F14,,
    pub input->keybit): set_bit(KEY_F15,,
    pub input->keybit): set_bit(KEY_F16,,
    pub input->keybit): set_bit(KEY_F17,,
    pub input->keybit): set_bit(KEY_F18,,
    default:
    pub 0: return,
    }
    pub 1: return,
    }
    static int ms_presenter_8k_quirk(struct hid_input *hi, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    if ((usage.hid & HID_USAGE_PAGE) != HID_UP_MSVENDOR)
    pub 0: return,
    pub hi->input->evbit): set_bit(EV_REP,,
    switch (usage.hid & HID_USAGE) {
    pub break: case 0xfd08: ms_map_key_clear(KEY_FORWARD);,
    pub break: case 0xfd09: ms_map_key_clear(KEY_BACK);,
    pub break: case 0xfd0b: ms_map_key_clear(KEY_PLAYPAUSE);,
    pub break: case 0xfd0e: ms_map_key_clear(KEY_CLOSE);,
    pub break: case 0xfd0f: ms_map_key_clear(KEY_PLAY);,
    default:
    pub 0: return,
    }
    pub 1: return,
    }
    static int ms_surface_dial_quirk(struct hid_input *hi, struct hid_field *field,
    struct hid_usage *usage, unsigned long **bit, int *max)
    {
    switch (usage.hid & HID_USAGE_PAGE) {
    case 0xff070000:
    case HID_UP_DIGITIZER:
// ignore those axis
    pub -1: return,
    case HID_UP_GENDESK:
    switch (usage.hid) {
    case HID_GD_X:
    case HID_GD_Y:
    case HID_GD_RFKILL_BTN:
// ignore those axis
    pub -1: return,
    }
    }
    pub 0: return,
    }
    static int ms_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    pub hid_get_drvdata(hdev): *mut *mut ms_data ms =,
    pub ms->quirks: unsigned long quirks =,
    if (quirks & MS_ERGONOMY) {
    pub max): int ret = ms_ergonomy_kb_quirk(hi, usage, bit,,
    if (ret)
    pub ret: return,
    }
    if ((quirks & MS_PRESENTER) &&
    ms_presenter_8k_quirk(hi, usage, bit, max))
    pub 1: return,
    if (quirks & MS_SURFACE_DIAL) {
    pub max): int ret = ms_surface_dial_quirk(hi, field, usage, bit,,
    if (ret)
    pub ret: return,
    }
    pub 0: return,
    }
    static int ms_input_mapped(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    pub hid_get_drvdata(hdev): *mut *mut ms_data ms =,
    pub ms->quirks: unsigned long quirks =,
    if (quirks & MS_DUPLICATE_USAGES)
    pub bit): *mut clear_bit(usage->code,,
    pub 0: return,
    }
    static int ms_event(struct hid_device *hdev, struct hid_field *field,
    struct hid_usage *usage, __s32 value)
    {
    pub hid_get_drvdata(hdev): *mut *mut ms_data ms =,
    pub ms->quirks: unsigned long quirks =,
    pub input: *mut input_dev,
    if (!(hdev.claimed & HID_CLAIMED_INPUT) || !field.hidinput ||
    !usage.type)
    pub 0: return,
    pub field->hidinput->input: input =,
// Handling MS keyboards special buttons
    if (quirks & MS_ERGONOMY && usage.hid == (HID_UP_MSVENDOR | 0xff00)) {
// Special keypad keys
    pub 0x01): input_report_key(input, KEY_KPEQUAL, value &,
    pub 0x02): input_report_key(input, KEY_KPLEFTPAREN, value &,
    pub 0x04): input_report_key(input, KEY_KPRIGHTPAREN, value &,
    pub 1: return,
    }
    if (quirks & MS_ERGONOMY && usage.hid == (HID_UP_MSVENDOR | 0xff01)) {
// Scroll wheel
    pub 1: int step = ((value & 0x60) >> 5) +,
    switch (value & 0x1f) {
    case 0x01:
    pub step): input_report_rel(input, REL_WHEEL,,
    case 0x1f:
    pub -step): input_report_rel(input, REL_WHEEL,,
    }
    pub 1: return,
    }
    if (quirks & MS_ERGONOMY && usage.hid == (HID_UP_MSVENDOR | 0xff05)) {
    pub 0: static unsigned int last_key =,
    pub 0: unsigned int key =,
    switch (value) {
    pub break: case 0x01: key = KEY_F14;,
    pub break: case 0x02: key = KEY_F15;,
    pub break: case 0x04: key = KEY_F16;,
    pub break: case 0x08: key = KEY_F17;,
    pub break: case 0x10: key = KEY_F18;,
    }
    if (key) {
    pub 1): input_event(input, usage->type, key,,
    pub key: last_key =,
    } else
    pub 0): input_event(input, usage->type, last_key,,
    pub 1: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ms_ff_worker(work: *mut work_struct) {
    static void ms_ff_worker(struct work_struct *work)
    {
    pub ff_worker): *mut *mut ms_data ms = container_of(work, ms_data,,
    pub ms->hdev: *mut *mut hid_device hdev =,
    pub ms->output_report_dmabuf: *mut *mut xb1s_ff_report r =,
    pub ret: c_int,
    pub sizeof(*r)): *mut memset(r, 0,,
    pub XB1S_FF_REPORT: r->report_id =,
    pub ENABLE_STRONG: r->enable = ENABLE_WEAK |,
//
// Specifying maximum duration and maximum loop count should
// cover maximum duration of a single effect, which is 65536
// ms
//
    pub U8_MAX: r->duration_10ms =,
    pub U8_MAX: r->loop_count =,
    pub /: *mut *mut r->magnitude[MAGNITUDE_STRONG] = ms->strong; / left actuator,
    pub /: *mut *mut r->magnitude[MAGNITUDE_WEAK] = ms->weak; / right actuator,
    pub sizeof(*r)): *mut *mut ret = hid_hw_output_report(hdev, (__u8 )r,,
    if (ret < 0)
    pub report\n"): hid_warn(hdev, "failed to send FF,
    }
    static int ms_play_effect(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    pub input_get_drvdata(dev): *mut *mut hid_device hid =,
    pub hid_get_drvdata(hid): *mut *mut ms_data ms =,
    if (effect.type != FF_RUMBLE)
    pub 0: return,
//
// Magnitude is 0..100 so scale the 16-bit input here
//
    pub U16_MAX: *mut *mut ms->strong = ((u32) effect->u.rumble.strong_magnitude  100) /,
    pub U16_MAX: *mut *mut ms->weak = ((u32) effect->u.rumble.weak_magnitude  100) /,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ms_input_configured(hdev: *mut hid_device, hidinput: *mut hid_input) -> c_int {
    static int ms_input_configured(struct hid_device *hdev, struct hid_input *hidinput)
    {
    pub hid_get_drvdata(hdev): *mut *mut ms_data ms =,
    pub hidinput->input: *mut *mut input_dev input_dev =,
    if (!(ms.quirks & MS_QUIRK_FF))
    pub 0: return,
    if (!list_is_first(&hidinput.list, &hdev.inputs))
    pub 0: return,
    pub hdev: ms->hdev =,
    pub ms_ff_worker): INIT_WORK(&ms->ff_worker,,
    ms.output_report_dmabuf = devm_kzalloc(&hdev.dev,
    sizeof(struct xb1s_ff_report),
    if (ms.output_report_dmabuf == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub FF_RUMBLE): input_set_capability(input_dev, EV_FF,,
    pub ms_play_effect): return input_ff_create_memless(input_dev, NULL,,
    }
#[no_mangle]
unsafe extern "C" fn ms_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int ms_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    pub id->driver_data: unsigned long quirks =,
    pub ms: *mut ms_data,
    pub ret: c_int,
    pub GFP_KERNEL): *mut *mut ms = devm_kzalloc(&hdev->dev, sizeof(ms),,
    if (ms == core::ptr::null_mut())
    pub -ENOMEM: return,
    pub quirks: ms->quirks =,
    pub ms): hid_set_drvdata(hdev,,
    if (quirks & MS_NOGET)
    pub HID_QUIRK_NOGET: hdev->quirks |=,
    if (quirks & MS_SURFACE_DIAL)
    pub HID_QUIRK_INPUT_PER_APP: hdev->quirks |=,
    pub hid_parse(hdev): ret =,
    if (ret) {
    pub failed\n"): hid_err(hdev, "parse,
    pub ret: return,
    }
    ret = hid_hw_start(hdev, HID_CONNECT_DEFAULT | ((quirks & MS_HIDINPUT) ?
    pub 0)): HID_CONNECT_HIDINPUT_FORCE :,
    if (ret) {
    pub failed\n"): hid_err(hdev, "hw start,
    pub ret: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ms_remove(hdev: *mut hid_device) {
    static void ms_remove(struct hid_device *hdev)
    {
    }
    static const struct hid_device_id ms_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_SIDEWINDER_GV),
    .driver_data = MS_HIDINPUT },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_OFFICE_KB),
    .driver_data = MS_ERGONOMY },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_NE4K),
    .driver_data = MS_ERGONOMY },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_NE4K_JP),
    .driver_data = MS_ERGONOMY },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_NE7K),
    .driver_data = MS_ERGONOMY },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_LK6K),
    .driver_data = MS_ERGONOMY | MS_RDESC },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_PRESENTER_8K_USB),
    .driver_data = MS_PRESENTER },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_DIGITAL_MEDIA_3K),
    .driver_data = MS_ERGONOMY },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_DIGITAL_MEDIA_7K),
    .driver_data = MS_ERGONOMY },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_DIGITAL_MEDIA_600),
    .driver_data = MS_ERGONOMY },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_DIGITAL_MEDIA_3KV1),
    .driver_data = MS_ERGONOMY },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_WIRELESS_OPTICAL_DESKTOP_3_0),
    .driver_data = MS_NOGET },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_COMFORT_MOUSE_4500),
    .driver_data = MS_DUPLICATE_USAGES },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_POWER_COVER),
    .driver_data = MS_HIDINPUT },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_COMFORT_KEYBOARD),
    .driver_data = MS_ERGONOMY},
    { HID_BLUETOOTH_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_PRESENTER_8K_BT),
    .driver_data = MS_PRESENTER },
    { HID_BLUETOOTH_DEVICE(USB_VENDOR_ID_MICROSOFT, 0x091B),
    .driver_data = MS_SURFACE_DIAL },
    { HID_BLUETOOTH_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_XBOX_CONTROLLER_MODEL_1708),
    .driver_data = MS_QUIRK_FF },
    { HID_BLUETOOTH_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_XBOX_CONTROLLER_MODEL_1708_BLE),
    .driver_data = MS_QUIRK_FF },
    { HID_BLUETOOTH_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_XBOX_CONTROLLER_MODEL_1914),
    .driver_data = MS_QUIRK_FF },
    { HID_BLUETOOTH_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_XBOX_CONTROLLER_MODEL_1797),
    .driver_data = MS_QUIRK_FF },
    { HID_BLUETOOTH_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_MS_XBOX_CONTROLLER_MODEL_1797_BLE),
    .driver_data = MS_QUIRK_FF },
    { HID_BLUETOOTH_DEVICE(USB_VENDOR_ID_MICROSOFT, USB_DEVICE_ID_8BITDO_SN30_PRO_PLUS),
    .driver_data = MS_QUIRK_FF },
    { }
}

    MODULE_DEVICE_TABLE(hid, ms_devices);
    static struct hid_driver ms_driver = {
    .name = "microsoft",
    .id_table = ms_devices,
    .report_fixup = ms_report_fixup,
    .input_mapping = ms_input_mapping,
    .input_mapped = ms_input_mapped,
    .input_configured = ms_input_configured,
    .event = ms_event,
    .probe = ms_probe,
    .remove = ms_remove,
    };
    module_hid_driver(ms_driver);
    MODULE_DESCRIPTION("HID driver for some microsoft \"special\" devices");
    MODULE_LICENSE("GPL");
