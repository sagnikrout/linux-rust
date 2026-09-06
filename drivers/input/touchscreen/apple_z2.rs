//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/apple_z2.c
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
// Apple Z2 touchscreen driver
//
// Copyright (C) The Asahi Linux Contributors
//

pub const APPLE_Z2_NUM_FINGERS_OFFSET: c_int = 16;
pub const APPLE_Z2_FINGERS_OFFSET: c_int = 24;
pub const APPLE_Z2_TOUCH_STARTED: c_int = 3;
pub const APPLE_Z2_TOUCH_MOVED: c_int = 4;
pub const APPLE_Z2_CMD_READ_INTERRUPT_DATA: c_uint = 0xEB;
pub const APPLE_Z2_REPLY_INTERRUPT_DATA: c_uint = 0xE1;
pub const APPLE_Z2_HBPP_CMD_BLOB: c_uint = 0x3001;
pub const APPLE_Z2_FW_MAGIC: c_uint = 0x5746325A;
pub const LOAD_COMMAND_INIT_PAYLOAD: c_int = 0;
pub const LOAD_COMMAND_SEND_BLOB: c_int = 1;
pub const LOAD_COMMAND_SEND_CALIBRATION: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_z2 {
    pub spidev: *mut spi_device,
    pub reset_gpio: *mut gpio_desc,
    pub input_dev: *mut input_dev,
    pub boot_irq: completion,
    pub booted: bool,
    pub index_parity: c_int,
    pub props: touchscreen_properties,
    pub fw_name: *const c_char,
    pub tx_buf: *mut u8,
    pub rx_buf: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_z2_finger {
    pub finger: u8,
    pub state: u8,
    pub unknown2: __le16,
    pub abs_x: __le16,
    pub abs_y: __le16,
    pub rel_x: __le16,
    pub rel_y: __le16,
    pub tool_major: __le16,
    pub tool_minor: __le16,
    pub orientation: __le16,
    pub touch_major: __le16,
    pub touch_minor: __le16,
    pub unused: [__le16; 2],
    pub pressure: __le16,
    pub multi: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_z2_hbpp_blob_hdr {
    pub cmd: __le16,
    pub len: __le16,
    pub addr: __le32,
    pub checksum: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_z2_fw_hdr {
    pub magic: __le32,
    pub version: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_z2_read_interrupt_cmd {
    pub cmd: u8,
    pub counter: u8,
    pub unused: [u8; 12],
    pub checksum: __le16,
}

    static void apple_z2_parse_touches(struct apple_z2 *z2,
    const u8 *msg, size_t msg_len)
    {
    int i;
    int nfingers;
    int slot;
    int slot_valid;
    struct apple_z2_finger *fingers;
    if (msg_len <= APPLE_Z2_NUM_FINGERS_OFFSET)
    return;
    nfingers = msg[APPLE_Z2_NUM_FINGERS_OFFSET];
    fingers = (struct apple_z2_finger *)(msg + APPLE_Z2_FINGERS_OFFSET);
    for (i = 0; i < nfingers; i++) {
    slot = input_mt_get_slot_by_key(z2.input_dev, fingers[i].finger);
    if (slot < 0) {
    dev_warn(&z2.spidev.dev, "unable to get slot for finger\n");
    continue;
    }
    slot_valid = fingers[i].state == APPLE_Z2_TOUCH_STARTED ||
    fingers[i].state == APPLE_Z2_TOUCH_MOVED;
    input_mt_slot(z2.input_dev, slot);
    if (!input_mt_report_slot_state(z2.input_dev, MT_TOOL_FINGER, slot_valid))
    continue;
    touchscreen_report_pos(z2.input_dev, &z2.props,
    le16_to_cpu(fingers[i].abs_x),
    le16_to_cpu(fingers[i].abs_y),
    true);
    input_report_abs(z2.input_dev, ABS_MT_WIDTH_MAJOR,
    le16_to_cpu(fingers[i].tool_major));
    input_report_abs(z2.input_dev, ABS_MT_WIDTH_MINOR,
    le16_to_cpu(fingers[i].tool_minor));
    input_report_abs(z2.input_dev, ABS_MT_ORIENTATION,
    le16_to_cpu(fingers[i].orientation));
    input_report_abs(z2.input_dev, ABS_MT_TOUCH_MAJOR,
    le16_to_cpu(fingers[i].touch_major));
    input_report_abs(z2.input_dev, ABS_MT_TOUCH_MINOR,
    le16_to_cpu(fingers[i].touch_minor));
    }
    input_mt_sync_frame(z2.input_dev);
    input_sync(z2.input_dev);
    }
#[no_mangle]
unsafe extern "C" fn apple_z2_read_packet(z2: *mut apple_z2) -> c_int {
    static int apple_z2_read_packet(struct apple_z2 *z2)
    {
    struct apple_z2_read_interrupt_cmd *len_cmd = (void *)z2.tx_buf;
    struct spi_transfer xfer;
    int error;
    size_t pkt_len;
    memset(&xfer, 0, sizeof(xfer));
    len_cmd.cmd = APPLE_Z2_CMD_READ_INTERRUPT_DATA;
    len_cmd.counter = z2.index_parity + 1;
    len_cmd.checksum =
    cpu_to_le16(APPLE_Z2_CMD_READ_INTERRUPT_DATA + len_cmd.counter);
    z2.index_parity = !z2.index_parity;
    xfer.tx_buf = z2.tx_buf;
    xfer.rx_buf = z2.rx_buf;
    xfer.len = sizeof(*len_cmd);
    error = spi_sync_transfer(z2.spidev, &xfer, 1);
    if (error)
    return error;
    if (z2.rx_buf[0] != APPLE_Z2_REPLY_INTERRUPT_DATA)
    return 0;
    pkt_len = (get_unaligned_le16(z2.rx_buf + 1) + 8) & 0xfffffffc;
    error = spi_read(z2.spidev, z2.rx_buf, pkt_len);
    if (error)
    return error;
    apple_z2_parse_touches(z2, z2.rx_buf + 5, pkt_len - 5);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_z2_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t apple_z2_irq(int irq, void *data)
    {
    struct apple_z2 *z2 = data;
    if (unlikely(!z2.booted))
    complete(&z2.boot_irq);
    else
    apple_z2_read_packet(z2);
    return IRQ_HANDLED;
    }
// Build calibration blob, caller is responsible for freeing the blob data.
    static const u8 *apple_z2_build_cal_blob(struct apple_z2 *z2,
    u32 address, size_t *size)
    {
    u8 *cal_data;
    int cal_size;
    size_t blob_size;
    u32 checksum;
    u16 checksum_hdr;
    int i;
    struct apple_z2_hbpp_blob_hdr *hdr;
    int error;
    if (!device_property_present(&z2.spidev.dev, CAL_PROP_NAME))
    return core::ptr::null_mut();
    cal_size = device_property_count_u8(&z2.spidev.dev, CAL_PROP_NAME);
    if (cal_size < 0)
    return ERR_PTR(cal_size);
    blob_size = sizeof(struct apple_z2_hbpp_blob_hdr) + cal_size + sizeof(__le32);
    u8 *blob_data __free(kfree) = kzalloc(blob_size, GFP_KERNEL);
    if (!blob_data)
    return ERR_PTR(-ENOMEM);
    hdr = (struct apple_z2_hbpp_blob_hdr *)blob_data;
    hdr.cmd = cpu_to_le16(APPLE_Z2_HBPP_CMD_BLOB);
    hdr.len = cpu_to_le16(round_up(cal_size, 4) / 4);
    hdr.addr = cpu_to_le32(address);
    checksum_hdr = 0;
    for (i = 2; i < 8; i++)
    checksum_hdr += blob_data[i];
    hdr.checksum = cpu_to_le16(checksum_hdr);
    cal_data = blob_data + sizeof(struct apple_z2_hbpp_blob_hdr);
    error = device_property_read_u8_array(&z2.spidev.dev, CAL_PROP_NAME,
    cal_data, cal_size);
    if (error)
    return ERR_PTR(error);
    checksum = 0;
    for (i = 0; i < cal_size; i++)
    checksum += cal_data[i];
    put_unaligned_le32(checksum, cal_data + cal_size);
// size = blob_size;
    return no_free_ptr(blob_data);
    }
    static int apple_z2_send_firmware_blob(struct apple_z2 *z2, const u8 *data,
    u32 size, bool init)
    {
    struct spi_message msg;
    struct spi_transfer blob_xfer, ack_xfer;
    int error;
    z2.tx_buf[0] = 0x1a;
    z2.tx_buf[1] = 0xa1;
    spi_message_init(&msg);
    memset(&blob_xfer, 0, sizeof(blob_xfer));
    memset(&ack_xfer, 0, sizeof(ack_xfer));
    blob_xfer.tx_buf = data;
    blob_xfer.len = size;
    blob_xfer.bits_per_word = init ? 8 : 16;
    spi_message_add_tail(&blob_xfer, &msg);
    ack_xfer.tx_buf = z2.tx_buf;
    ack_xfer.len = 2;
    spi_message_add_tail(&ack_xfer, &msg);
    reinit_completion(&z2.boot_irq);
    error = spi_sync(z2.spidev, &msg);
    if (error)
    return error;
// Irq only happens sometimes, but the thing boots reliably nonetheless
    wait_for_completion_timeout(&z2.boot_irq, msecs_to_jiffies(20));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_z2_upload_firmware(z2: *mut apple_z2) -> c_int {
    static int apple_z2_upload_firmware(struct apple_z2 *z2)
    {
    const struct apple_z2_fw_hdr *fw_hdr;
    let mut fw_idx: usize = sizeof(struct apple_z2_fw_hdr);
    int error;
    u32 load_cmd;
    u32 address;
    bool init;
    size_t size;
    const struct firmware *fw __free(firmware) = core::ptr::null_mut();
    error = request_firmware(&fw, z2.fw_name, &z2.spidev.dev);
    if (error) {
    dev_err(&z2.spidev.dev, "unable to load firmware\n");
    return error;
    }
    fw_hdr = (const struct apple_z2_fw_hdr *)fw.data;
    if (le32_to_cpu(fw_hdr.magic) != APPLE_Z2_FW_MAGIC || le32_to_cpu(fw_hdr.version) != 1) {
    dev_err(&z2.spidev.dev, "invalid firmware header\n");
    return -EINVAL;
    }
//
// This will interrupt the upload half-way if the file is malformed
// As the device has no non-volatile storage to corrupt, and gets reset
// on boot anyway, this is fine.
//
    while (fw_idx < fw.size) {
    if (fw.size - fw_idx < 8) {
    dev_err(&z2.spidev.dev, "firmware malformed\n");
    return -EINVAL;
    }
    load_cmd = le32_to_cpup(( __le32 *)(fw.data + fw_idx));
    fw_idx += sizeof(u32);
    if (load_cmd == LOAD_COMMAND_INIT_PAYLOAD || load_cmd == LOAD_COMMAND_SEND_BLOB) {
    size = le32_to_cpup(( __le32 *)(fw.data + fw_idx));
    fw_idx += sizeof(u32);
    if (fw.size - fw_idx < size) {
    dev_err(&z2.spidev.dev, "firmware malformed\n");
    return -EINVAL;
    }
    init = load_cmd == LOAD_COMMAND_INIT_PAYLOAD;
    error = apple_z2_send_firmware_blob(z2, fw.data + fw_idx,
    size, init);
    if (error)
    return error;
    fw_idx += size;
    } else if (load_cmd == LOAD_COMMAND_SEND_CALIBRATION) {
    address = le32_to_cpup(( __le32 *)(fw.data + fw_idx));
    fw_idx += sizeof(u32);
    const u8 *data __free(kfree) =
    apple_z2_build_cal_blob(z2, address, &size);
    if (IS_ERR(data))
    return PTR_ERR(data);
    if (data) {
    error = apple_z2_send_firmware_blob(z2, data, size, false);
    if (error)
    return error;
    }
    } else {
    dev_err(&z2.spidev.dev, "firmware malformed\n");
    return -EINVAL;
    }
    fw_idx = round_up(fw_idx, 4);
    }
    z2.booted = true;
    apple_z2_read_packet(z2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_z2_boot(z2: *mut apple_z2) -> c_int {
    static int apple_z2_boot(struct apple_z2 *z2)
    {
    int error;
    reinit_completion(&z2.boot_irq);
    enable_irq(z2.spidev.irq);
    gpiod_set_value(z2.reset_gpio, 0);
    if (!wait_for_completion_timeout(&z2.boot_irq, msecs_to_jiffies(20)))
    return -ETIMEDOUT;
    error = apple_z2_upload_firmware(z2);
    if (error) {
    gpiod_set_value(z2.reset_gpio, 1);
    disable_irq(z2.spidev.irq);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_z2_probe(spi: *mut spi_device) -> c_int {
    static int apple_z2_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct apple_z2 *z2;
    int error;
    z2 = devm_kzalloc(dev, sizeof(*z2), GFP_KERNEL);
    if (!z2)
    return -ENOMEM;
    z2.tx_buf = devm_kzalloc(dev, sizeof(struct apple_z2_read_interrupt_cmd), GFP_KERNEL);
    if (!z2.tx_buf)
    return -ENOMEM;
// 4096 will end up being rounded up to 8192 due to devres header
    z2.rx_buf = devm_kzalloc(dev, 4000, GFP_KERNEL);
    if (!z2.rx_buf)
    return -ENOMEM;
    z2.spidev = spi;
    init_completion(&z2.boot_irq);
    spi_set_drvdata(spi, z2);
// Reset the device on boot
    z2.reset_gpio = devm_gpiod_get(dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(z2.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(z2.reset_gpio), "unable to get reset\n");
    error = devm_request_threaded_irq(dev, z2.spidev.irq, core::ptr::null_mut(), apple_z2_irq,
    IRQF_ONESHOT | IRQF_NO_AUTOEN,
    "apple-z2-irq", z2);
    if (error)
    return dev_err_probe(dev, error, "unable to request irq\n");
    error = device_property_read_string(dev, "firmware-name", &z2.fw_name);
    if (error)
    return dev_err_probe(dev, error, "unable to get firmware name\n");
    z2.input_dev = devm_input_allocate_device(dev);
    if (!z2.input_dev)
    return -ENOMEM;
    z2.input_dev.name = (char *)spi_get_device_id(spi).driver_data;
    z2.input_dev.phys = "apple_z2";
    z2.input_dev.id.bustype = BUS_SPI;
// Allocate the axes before setting from DT
    input_set_abs_params(z2.input_dev, ABS_MT_POSITION_X, 0, 0, 0, 0);
    input_set_abs_params(z2.input_dev, ABS_MT_POSITION_Y, 0, 0, 0, 0);
    touchscreen_parse_properties(z2.input_dev, true, &z2.props);
    input_abs_set_res(z2.input_dev, ABS_MT_POSITION_X, 100);
    input_abs_set_res(z2.input_dev, ABS_MT_POSITION_Y, 100);
    input_set_abs_params(z2.input_dev, ABS_MT_WIDTH_MAJOR, 0, 65535, 0, 0);
    input_set_abs_params(z2.input_dev, ABS_MT_WIDTH_MINOR, 0, 65535, 0, 0);
    input_set_abs_params(z2.input_dev, ABS_MT_TOUCH_MAJOR, 0, 65535, 0, 0);
    input_set_abs_params(z2.input_dev, ABS_MT_TOUCH_MINOR, 0, 65535, 0, 0);
    input_set_abs_params(z2.input_dev, ABS_MT_ORIENTATION, -32768, 32767, 0, 0);
    error = input_mt_init_slots(z2.input_dev, 256, INPUT_MT_DIRECT);
    if (error)
    return dev_err_probe(dev, error, "unable to initialize multitouch slots\n");
    error = input_register_device(z2.input_dev);
    if (error)
    return dev_err_probe(dev, error, "unable to register input device\n");
// Wait for device reset to finish
    usleep_range(5000, 10000);
    error = apple_z2_boot(z2);
    if (error)
    return error;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_z2_shutdown(spi: *mut spi_device) {
    static void apple_z2_shutdown(struct spi_device *spi)
    {
    struct apple_z2 *z2 = spi_get_drvdata(spi);
    disable_irq(z2.spidev.irq);
    gpiod_direction_output(z2.reset_gpio, 1);
    z2.booted = false;
    }
#[no_mangle]
unsafe extern "C" fn apple_z2_suspend(dev: *mut device) -> c_int {
    static int apple_z2_suspend(struct device *dev)
    {
    apple_z2_shutdown(to_spi_device(dev));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_z2_resume(dev: *mut device) -> c_int {
    static int apple_z2_resume(struct device *dev)
    {
    struct apple_z2 *z2 = spi_get_drvdata(to_spi_device(dev));
    return apple_z2_boot(z2);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(apple_z2_pm, apple_z2_suspend, apple_z2_resume);
    static const struct of_device_id apple_z2_of_match[] = {
    { .compatible = "apple,j293-touchbar" },
    { .compatible = "apple,j493-touchbar" },
    {}
    };
    MODULE_DEVICE_TABLE(of, apple_z2_of_match);
    static struct spi_device_id apple_z2_of_id[] = {
    { .name = "j293-touchbar", .driver_data = (kernel_ulong_t)"MacBookPro17,1 Touch Bar" },
    { .name = "j493-touchbar", .driver_data = (kernel_ulong_t)"Mac14,7 Touch Bar" },
    {}
    };
    MODULE_DEVICE_TABLE(spi, apple_z2_of_id);
    static struct spi_driver apple_z2_driver = {
    .driver = {
    .name	= "apple-z2",
    .pm	= pm_sleep_ptr(&apple_z2_pm),
    .of_match_table = apple_z2_of_match,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .id_table = apple_z2_of_id,
    .probe    = apple_z2_probe,
    .remove   = apple_z2_shutdown,
    };
    module_spi_driver(apple_z2_driver);
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE("apple/dfrmtfw-*.bin");
    MODULE_DESCRIPTION("Apple Z2 touchscreens driver");
