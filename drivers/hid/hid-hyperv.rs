//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-hyperv.c
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
// Copyright (c) 2009, Citrix Systems, Inc.
// Copyright (c) 2010, Microsoft Corporation.
// Copyright (c) 2011, Novell Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_dev_info {
    pub size: c_uint,
    pub vendor: c_ushort,
    pub product: c_ushort,
    pub version: c_ushort,
    pub reserved: [c_ushort; 11],
}

//
// Current version
//
// History:
// Beta, RC < 2008/1/22        1,0
// RC > 2008/1/22              2,0
//
pub const SYNTHHID_INPUT_VERSION_MAJOR: c_int = 2;
pub const SYNTHHID_INPUT_VERSION_MINOR: c_int = 0;

    (SYNTHHID_INPUT_VERSION_MAJOR << 16))

//
// Message types in the synthetic input protocol
//
    enum synthhid_msg_type {
    SYNTH_HID_PROTOCOL_REQUEST,
    SYNTH_HID_PROTOCOL_RESPONSE,
    SYNTH_HID_INITIAL_DEVICE_INFO,
    SYNTH_HID_INITIAL_DEVICE_INFO_ACK,
    SYNTH_HID_INPUT_REPORT,
    SYNTH_HID_MAX
    };
//
// Basic message structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthhid_msg_hdr {
    pub type: enum synthhid_msg_type,
    pub size: u32,
}

    union synthhid_version {
    struct {
    u16 minor_version;
    u16 major_version;
    };
    u32 version;
    };
//
// Protocol messages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthhid_protocol_request {
    pub header: synthhid_msg_hdr,
    pub version_requested: union synthhid_version,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthhid_protocol_response {
    pub header: synthhid_msg_hdr,
    pub version_requested: union synthhid_version,
    pub approved: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthhid_device_info {
    pub header: synthhid_msg_hdr,
    pub hid_dev_info: hv_input_dev_info,
    pub hid_descriptor: hid_descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthhid_device_info_ack {
    pub header: synthhid_msg_hdr,
    pub reserved: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct synthhid_input_report {
    pub header: synthhid_msg_hdr,
    pub buffer: [c_char; ],
}

    enum pipe_prot_msg_type {
    PIPE_MESSAGE_INVALID,
    PIPE_MESSAGE_DATA,
    PIPE_MESSAGE_MAXIMUM
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_prt_msg {
    pub type: enum pipe_prot_msg_type,
    pub size: u32,
    pub data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mousevsc_prt_msg {
    pub type: enum pipe_prot_msg_type,
    pub size: u32,
    union {
    pub request: synthhid_protocol_request,
    pub response: synthhid_protocol_response,
    pub ack: synthhid_device_info_ack,
}

    };
//
// Represents an mousevsc device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mousevsc_dev {
    pub device: *mut hv_device,
    pub init_complete: bool,
    pub connected: bool,
    pub protocol_req: mousevsc_prt_msg,
    pub protocol_resp: mousevsc_prt_msg,
// Synchronize the request/response if needed
    pub wait_event: completion,
    pub dev_info_status: c_int,
    pub hid_desc: *mut hid_descriptor,
    pub report_desc: *mut c_uchar,
    pub report_desc_size: u32,
    pub hid_dev_info: hv_input_dev_info,
    pub hid_device: *mut hid_device,
    pub input_buf: [u8; HID_MAX_BUFFER_SIZE],
}

    static struct mousevsc_dev *mousevsc_alloc_device(struct hv_device *device)
    {
    struct mousevsc_dev *input_dev;
    input_dev = kzalloc_obj(struct mousevsc_dev);
    if (!input_dev)
    return core::ptr::null_mut();
    input_dev.device = device;
    hv_set_drvdata(device, input_dev);
    init_completion(&input_dev.wait_event);
    input_dev.init_complete = false;
    return input_dev;
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_free_device(device: *mut mousevsc_dev) {
    static void mousevsc_free_device(struct mousevsc_dev *device)
    {
    kfree(device.hid_desc);
    kfree(device.report_desc);
    hv_set_drvdata(device.device, core::ptr::null_mut());
    kfree(device);
    }
    static void mousevsc_on_receive_device_info(struct mousevsc_dev *input_device,
    struct synthhid_device_info *device_info,
    u32 device_info_size)
    {
    let mut ret: c_int = 0;
    struct hid_descriptor *desc;
    struct mousevsc_prt_msg ack;
    size_t desc_offset;
    size_t desc_size;
    input_device.dev_info_status = -ENOMEM;
    if (device_info_size < sizeof(*device_info)) {
    input_device.dev_info_status = -EINVAL;
    goto cleanup;
    }
    input_device.hid_dev_info = device_info.hid_dev_info;
    desc = &device_info.hid_descriptor;
    desc_offset = offsetof(struct synthhid_device_info, hid_descriptor);
    desc_size = device_info_size - desc_offset;
    if (desc.bLength == 0)
    goto cleanup;
    if (desc.bLength < sizeof(*desc) || desc.bLength > desc_size) {
    input_device.dev_info_status = -EINVAL;
    goto cleanup;
    }
// The pointer is not NULL when we resume from hibernation
    kfree(input_device.hid_desc);
    input_device.hid_desc = kmemdup(desc, desc.bLength, GFP_ATOMIC);
    if (!input_device.hid_desc)
    goto cleanup;
    input_device.report_desc_size = le16_to_cpu(
    desc.rpt_desc.wDescriptorLength);
    if (input_device.report_desc_size == 0) {
    input_device.dev_info_status = -EINVAL;
    goto cleanup;
    }
    if (input_device.report_desc_size > desc_size - desc.bLength) {
    input_device.dev_info_status = -EINVAL;
    goto cleanup;
    }
// The pointer is not NULL when we resume from hibernation
    kfree(input_device.report_desc);
    input_device.report_desc = kzalloc(input_device.report_desc_size,
    GFP_ATOMIC);
    if (!input_device.report_desc) {
    input_device.dev_info_status = -ENOMEM;
    goto cleanup;
    }
    memcpy(input_device.report_desc,
    ((unsigned char *)desc) + desc.bLength,
    le16_to_cpu(desc.rpt_desc.wDescriptorLength));
// Send the ack
    memset(&ack, 0, sizeof(struct mousevsc_prt_msg));
    ack.type = PIPE_MESSAGE_DATA;
    ack.size = sizeof(struct synthhid_device_info_ack);
    ack.ack.header.type = SYNTH_HID_INITIAL_DEVICE_INFO_ACK;
    ack.ack.header.size = 1;
    ack.ack.reserved = 0;
    if (IS_ENABLED(CONFIG_HID_HYPERV_MOUSE_KUNIT_TEST) &&
    !input_device.device) {
    ret = 0;
    } else {
    ret = vmbus_sendpacket(input_device.device.channel,
    &ack,
    sizeof(struct pipe_prt_msg) +
    sizeof(struct synthhid_device_info_ack),
    (unsigned long)&ack,
    VM_PKT_DATA_INBAND,
    VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED);
    }
    if (!ret)
    input_device.dev_info_status = 0;
    cleanup:
    complete(&input_device.wait_event);
    return;
    }
    static void mousevsc_on_receive(struct hv_device *device,
    struct vmpacket_descriptor *packet)
    {
    struct pipe_prt_msg *pipe_msg;
    struct synthhid_msg_hdr *hid_msg_hdr;
    struct mousevsc_dev *input_dev = hv_get_drvdata(device);
    struct synthhid_input_report *input_report;
    size_t len;
    pipe_msg = (struct pipe_prt_msg *)((unsigned long)packet +
    (packet.offset8 << 3));
    if (pipe_msg.type != PIPE_MESSAGE_DATA)
    return;
    hid_msg_hdr = (struct synthhid_msg_hdr *)pipe_msg.data;
    switch (hid_msg_hdr.type) {
    case SYNTH_HID_PROTOCOL_RESPONSE:
    len = struct_size(pipe_msg, data, pipe_msg.size);
//
// While it will be impossible for us to protect against
// malicious/buggy hypervisor/host, add a check here to
// ensure we don't corrupt memory.
//
    if (WARN_ON(len > sizeof(struct mousevsc_prt_msg)))
    break;
    memcpy(&input_dev.protocol_resp, pipe_msg, len);
    complete(&input_dev.wait_event);
    break;
    case SYNTH_HID_INITIAL_DEVICE_INFO:
    if (WARN_ON_ONCE(pipe_msg.size <
    sizeof(struct synthhid_device_info)))
    break;
//
// Parse out the device info into device attr,
// hid desc and report desc
//
    mousevsc_on_receive_device_info(input_dev,
    (struct synthhid_device_info *)pipe_msg.data,
    pipe_msg.size);
    break;
    case SYNTH_HID_INPUT_REPORT:
    input_report =
    (struct synthhid_input_report *)pipe_msg.data;
    if (!input_dev.init_complete)
    break;
    len = min(input_report.header.size,
    (u32)sizeof(input_dev.input_buf));
    memcpy(input_dev.input_buf, input_report.buffer, len);
    hid_input_report(input_dev.hid_device, HID_INPUT_REPORT,
    input_dev.input_buf, len, 1);
    pm_wakeup_hard_event(&input_dev.device.device);
    break;
    default:
    pr_err("unsupported hid msg type - type %d len %d\n",
    hid_msg_hdr.type, hid_msg_hdr.size);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_on_channel_callback(context: *mut c_void) {
    static void mousevsc_on_channel_callback(void *context)
    {
    struct hv_device *device = context;
    struct vmpacket_descriptor *desc;
    foreach_vmbus_pkt(desc, device.channel) {
    switch (desc.type) {
    case VM_PKT_COMP:
    break;
    case VM_PKT_DATA_INBAND:
    mousevsc_on_receive(device, desc);
    break;
    default:
    pr_err("Unhandled packet type %d, tid %llx len %d\n",
    desc.type, desc.trans_id, desc.len8 * 8);
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_connect_to_vsp(device: *mut hv_device) -> c_int {
    static int mousevsc_connect_to_vsp(struct hv_device *device)
    {
    let mut ret: c_int = 0;
    unsigned long t;
    struct mousevsc_dev *input_dev = hv_get_drvdata(device);
    struct mousevsc_prt_msg *request;
    struct mousevsc_prt_msg *response;
    reinit_completion(&input_dev.wait_event);
    request = &input_dev.protocol_req;
    memset(request, 0, sizeof(struct mousevsc_prt_msg));
    request.type = PIPE_MESSAGE_DATA;
    request.size = sizeof(struct synthhid_protocol_request);
    request.request.header.type = SYNTH_HID_PROTOCOL_REQUEST;
    request.request.header.size = sizeof(unsigned int);
    request.request.version_requested.version = SYNTHHID_INPUT_VERSION;
    ret = vmbus_sendpacket(device.channel, request,
    sizeof(struct pipe_prt_msg) +
    sizeof(struct synthhid_protocol_request),
    (unsigned long)request,
    VM_PKT_DATA_INBAND,
    VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED);
    if (ret)
    goto cleanup;
    t = wait_for_completion_timeout(&input_dev.wait_event, 5*HZ);
    if (!t) {
    ret = -ETIMEDOUT;
    goto cleanup;
    }
    response = &input_dev.protocol_resp;
    if (!response.response.approved) {
    pr_err("synthhid protocol request failed (version %d)\n",
    SYNTHHID_INPUT_VERSION);
    ret = -ENODEV;
    goto cleanup;
    }
    t = wait_for_completion_timeout(&input_dev.wait_event, 5*HZ);
    if (!t) {
    ret = -ETIMEDOUT;
    goto cleanup;
    }
//
// We should have gotten the device attr, hid desc and report
// desc at this point
//
    ret = input_dev.dev_info_status;
    cleanup:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_hid_parse(hid: *mut hid_device) -> c_int {
    static int mousevsc_hid_parse(struct hid_device *hid)
    {
    struct hv_device *dev = hid_get_drvdata(hid);
    struct mousevsc_dev *input_dev = hv_get_drvdata(dev);
    return hid_parse_report(hid, input_dev.report_desc,
    input_dev.report_desc_size);
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_hid_open(hid: *mut hid_device) -> c_int {
    static int mousevsc_hid_open(struct hid_device *hid)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_hid_start(hid: *mut hid_device) -> c_int {
    static int mousevsc_hid_start(struct hid_device *hid)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_hid_close(hid: *mut hid_device) {
    static void mousevsc_hid_close(struct hid_device *hid)
    {
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_hid_stop(hid: *mut hid_device) {
    static void mousevsc_hid_stop(struct hid_device *hid)
    {
    }
    static int mousevsc_hid_raw_request(struct hid_device *hid,
    unsigned char report_num,
    __u8 *buf, size_t len,
    unsigned char rtype,
    int reqtype)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_hid_probe(hid_dev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int mousevsc_hid_probe(struct hid_device *hid_dev, const struct hid_device_id *id)
    {
    int ret;
    ret = hid_parse(hid_dev);
    if (ret) {
    hid_err(hid_dev, "parse failed\n");
    return ret;
    }
    ret = hid_hw_start(hid_dev, HID_CONNECT_HIDINPUT | HID_CONNECT_HIDDEV);
    if (ret) {
    hid_err(hid_dev, "hw start failed\n");
    return ret;
    }
    return 0;
    }
    static const struct hid_ll_driver mousevsc_ll_driver = {
    .parse = mousevsc_hid_parse,
    .open = mousevsc_hid_open,
    .close = mousevsc_hid_close,
    .start = mousevsc_hid_start,
    .stop = mousevsc_hid_stop,
    .raw_request = mousevsc_hid_raw_request,
    };
    static const struct hid_device_id mousevsc_devices[] = {
    { HID_DEVICE(BUS_VIRTUAL, HID_GROUP_ANY, 0x045E, 0x0621) },
    { }
    };
    static struct hid_driver mousevsc_hid_driver = {
    .name = "hid-hyperv",
    .id_table = mousevsc_devices,
    .probe = mousevsc_hid_probe,
    };
    static int mousevsc_probe(struct hv_device *device,
    const struct hv_vmbus_device_id *dev_id)
    {
    int ret;
    struct mousevsc_dev *input_dev;
    struct hid_device *hid_dev;
    input_dev = mousevsc_alloc_device(device);
    if (!input_dev)
    return -ENOMEM;
    ret = vmbus_open(device.channel,
    INPUTVSC_SEND_RING_BUFFER_SIZE,
    INPUTVSC_RECV_RING_BUFFER_SIZE,
    core::ptr::null_mut(),
    0,
    mousevsc_on_channel_callback,
    device
    );
    if (ret)
    goto probe_err0;
    ret = mousevsc_connect_to_vsp(device);
    if (ret)
    goto probe_err1;
// workaround SA-167
    if (input_dev.report_desc[14] == 0x25)
    input_dev.report_desc[14] = 0x29;
    hid_dev = hid_allocate_device();
    if (IS_ERR(hid_dev)) {
    ret = PTR_ERR(hid_dev);
    goto probe_err1;
    }
    hid_dev.ll_driver = &mousevsc_ll_driver;
    hid_dev.bus = BUS_VIRTUAL;
    hid_dev.vendor = input_dev.hid_dev_info.vendor;
    hid_dev.product = input_dev.hid_dev_info.product;
    hid_dev.version = input_dev.hid_dev_info.version;
    input_dev.hid_device = hid_dev;
    sprintf(hid_dev.name, "%s", "Microsoft Vmbus HID-compliant Mouse");
    hid_set_drvdata(hid_dev, device);
    ret = hid_add_device(hid_dev);
    if (ret)
    goto probe_err2;
    device_init_wakeup(&device.device, true);
    input_dev.connected = true;
    input_dev.init_complete = true;
    return ret;
    probe_err2:
    hid_destroy_device(hid_dev);
    probe_err1:
    vmbus_close(device.channel);
    probe_err0:
    mousevsc_free_device(input_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_remove(dev: *mut hv_device) {
    static void mousevsc_remove(struct hv_device *dev)
    {
    struct mousevsc_dev *input_dev = hv_get_drvdata(dev);
    device_init_wakeup(&dev.device, false);
    vmbus_close(dev.channel);
    hid_hw_stop(input_dev.hid_device);
    hid_destroy_device(input_dev.hid_device);
    mousevsc_free_device(input_dev);
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_suspend(dev: *mut hv_device) -> c_int {
    static int mousevsc_suspend(struct hv_device *dev)
    {
    vmbus_close(dev.channel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_resume(dev: *mut hv_device) -> c_int {
    static int mousevsc_resume(struct hv_device *dev)
    {
    int ret;
    ret = vmbus_open(dev.channel,
    INPUTVSC_SEND_RING_BUFFER_SIZE,
    INPUTVSC_RECV_RING_BUFFER_SIZE,
    core::ptr::null_mut(), 0,
    mousevsc_on_channel_callback,
    dev);
    if (ret)
    return ret;
    ret = mousevsc_connect_to_vsp(dev);
    return ret;
    }
    static const struct hv_vmbus_device_id id_table[] = {
// Mouse guid
    { HV_MOUSE_GUID, },
    { },
    };
    MODULE_DEVICE_TABLE(vmbus, id_table);
    static struct  hv_driver mousevsc_drv = {
    .name = KBUILD_MODNAME,
    .id_table = id_table,
    .probe = mousevsc_probe,
    .remove = mousevsc_remove,
    .suspend = mousevsc_suspend,
    .resume = mousevsc_resume,
    .driver = {
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
#[no_mangle]
unsafe extern "C" fn mousevsc_init() -> int __init {
    static int __init mousevsc_init(void)
    {
    int ret;
    ret = hid_register_driver(&mousevsc_hid_driver);
    if (ret)
    return ret;
    ret = vmbus_driver_register(&mousevsc_drv);
    if (ret)
    hid_unregister_driver(&mousevsc_hid_driver);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_exit() -> void __exit {
    static void __exit mousevsc_exit(void)
    {
    vmbus_driver_unregister(&mousevsc_drv);
    hid_unregister_driver(&mousevsc_hid_driver);
    }
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Microsoft Hyper-V Synthetic HID Driver");

    static struct mousevsc_dev *mousevsc_kunit_alloc_dev(struct kunit *test)
    {
    struct mousevsc_dev *input_dev;
    input_dev = kunit_kzalloc(test, sizeof(*input_dev), GFP_KERNEL);
    if (!input_dev)
    return core::ptr::null_mut();
    init_completion(&input_dev.wait_event);
    return input_dev;
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_device_info_zero_blength(test: *mut kunit) {
    static void mousevsc_device_info_zero_blength(struct kunit *test)
    {
    struct synthhid_device_info *info;
    struct mousevsc_dev *input_dev;
    input_dev = mousevsc_kunit_alloc_dev(test);
    KUNIT_ASSERT_NOT_NULL(test, input_dev);
    info = kunit_kzalloc(test, sizeof(*info), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, info);
    info.hid_descriptor.bLength = 0;
    mousevsc_on_receive_device_info(input_dev, info, sizeof(*info));
    KUNIT_EXPECT_EQ(test, input_dev.dev_info_status, -ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_device_info_valid_descriptor(test: *mut kunit) {
    static void mousevsc_device_info_valid_descriptor(struct kunit *test)
    {
    struct synthhid_device_info *info;
    struct mousevsc_dev *input_dev;
    u8 *report;
    input_dev = mousevsc_kunit_alloc_dev(test);
    KUNIT_ASSERT_NOT_NULL(test, input_dev);
    info = kunit_kzalloc(test, sizeof(*info) + 4, GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, info);
    info.hid_descriptor.bLength = sizeof(struct hid_descriptor);
    info.hid_descriptor.rpt_desc.wDescriptorLength = cpu_to_le16(4);
    report = (u8 *)(info + 1);
    memset(report, 0x42, 4);
    mousevsc_on_receive_device_info(input_dev, info, sizeof(*info) + 4);
    KUNIT_EXPECT_EQ(test, input_dev.dev_info_status, 0);
    KUNIT_EXPECT_EQ(test, input_dev.report_desc_size, 4);
    KUNIT_EXPECT_MEMEQ(test, input_dev.report_desc, report, 4);
    kfree(input_dev.hid_desc);
    kfree(input_dev.report_desc);
    }
#[no_mangle]
unsafe extern "C" fn mousevsc_device_info_report_desc_oob(test: *mut kunit) {
    static void mousevsc_device_info_report_desc_oob(struct kunit *test)
    {
    struct synthhid_device_info *info;
    struct mousevsc_dev *input_dev;
    u8 *report;
    input_dev = mousevsc_kunit_alloc_dev(test);
    KUNIT_ASSERT_NOT_NULL(test, input_dev);
    info = kunit_kzalloc(test, sizeof(*info) + 8, GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, info);
    info.hid_descriptor.bLength = sizeof(struct hid_descriptor);
    info.hid_descriptor.rpt_desc.wDescriptorLength = cpu_to_le16(64);
    report = (u8 *)(info + 1);
    memset(report, 0x42, 8);
    mousevsc_on_receive_device_info(input_dev, info, sizeof(*info) + 8);
    KUNIT_EXPECT_EQ(test, input_dev.dev_info_status, -EINVAL);
    kfree(input_dev.hid_desc);
    }
    static struct kunit_case mousevsc_test_cases[] = {
    KUNIT_CASE(mousevsc_device_info_zero_blength),
    KUNIT_CASE(mousevsc_device_info_valid_descriptor),
    KUNIT_CASE(mousevsc_device_info_report_desc_oob),
    {}
    };
    static struct kunit_suite mousevsc_test_suite = {
    .name = "hid_hyperv_mouse",
    .test_cases = mousevsc_test_cases,
    };
    kunit_test_suite(mousevsc_test_suite);

    module_init(mousevsc_init);
    module_exit(mousevsc_exit);
