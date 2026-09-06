//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/raspberrypi-ts.c
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
// Raspberry Pi firmware based touchscreen driver
//
// Copyright (C) 2015, 2017 Raspberry Pi
// Copyright (C) 2018 Nicolas Saenz Julienne <nsaenzjulienne@suse.de>
//

pub const RPI_TS_DEFAULT_WIDTH: c_int = 800;
pub const RPI_TS_DEFAULT_HEIGHT: c_int = 480;
pub const RPI_TS_MAX_SUPPORTED_POINTS: c_int = 10;
pub const RPI_TS_FTS_TOUCH_DOWN: c_int = 0;
pub const RPI_TS_FTS_TOUCH_CONTACT: c_int = 2;

pub const RPI_TS_NPOINTS_REG_INVALIDATE: c_int = 99;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpi_ts {
    pub pdev: *mut platform_device,
    pub input: *mut input_dev,
    pub prop: touchscreen_properties,
    pub fw_regs_va: *mut void __iomem,
    pub fw_regs_phys: dma_addr_t,
    pub known_ids: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpi_ts_regs {
    pub device_mode: u8,
    pub gesture_id: u8,
    pub num_points: u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpi_ts_touch {
    pub xh: u8,
    pub xl: u8,
    pub yh: u8,
    pub yl: u8,
    pub /: *mut *mut u8 pressure; / Not supported,
    pub /: *mut *mut u8 area; / Not supported,
    pub point: [}; RPI_TS_MAX_SUPPORTED_POINTS],
}

#[no_mangle]
unsafe extern "C" fn rpi_ts_poll(input: *mut input_dev) {
    static void rpi_ts_poll(struct input_dev *input)
    {
    struct rpi_ts *ts = input_get_drvdata(input);
    struct rpi_ts_regs regs;
    let mut modified_ids: c_int = 0;
    long released_ids;
    int event_type;
    int touchid;
    int x, y;
    int i;
    memcpy_fromio(&regs, ts.fw_regs_va, sizeof(regs));
//
// We poll the memory based register copy of the touchscreen chip using
// the number of points register to know whether the copy has been
// updated (we write 99 to the memory copy, the GPU will write between
// 0 - 10 points)
//
    iowrite8(RPI_TS_NPOINTS_REG_INVALIDATE,
    ts.fw_regs_va + offsetof(struct rpi_ts_regs, num_points));
    if (regs.num_points == RPI_TS_NPOINTS_REG_INVALIDATE ||
    (regs.num_points == 0 && ts.known_ids == 0))
    return;
    for (i = 0; i < regs.num_points; i++) {
    x = (((int)regs.point[i].xh & 0xf) << 8) + regs.point[i].xl;
    y = (((int)regs.point[i].yh & 0xf) << 8) + regs.point[i].yl;
    touchid = (regs.point[i].yh >> 4) & 0xf;
    event_type = (regs.point[i].xh >> 6) & 0x03;
    modified_ids |= BIT(touchid);
    if (event_type == RPI_TS_FTS_TOUCH_DOWN ||
    event_type == RPI_TS_FTS_TOUCH_CONTACT) {
    input_mt_slot(input, touchid);
    input_mt_report_slot_state(input, MT_TOOL_FINGER, 1);
    touchscreen_report_pos(input, &ts.prop, x, y, true);
    }
    }
    released_ids = ts.known_ids & ~modified_ids;
    for_each_set_bit(i, &released_ids, RPI_TS_MAX_SUPPORTED_POINTS) {
    input_mt_slot(input, i);
    input_mt_report_slot_inactive(input);
    modified_ids &= ~(BIT(i));
    }
    ts.known_ids = modified_ids;
    input_mt_sync_frame(input);
    input_sync(input);
    }
#[no_mangle]
unsafe extern "C" fn rpi_ts_dma_cleanup(data: *mut c_void) {
    static void rpi_ts_dma_cleanup(void *data)
    {
    struct rpi_ts *ts = data;
    struct device *dev = &ts.pdev.dev;
    dma_free_coherent(dev, PAGE_SIZE, ts.fw_regs_va, ts.fw_regs_phys);
    }
#[no_mangle]
unsafe extern "C" fn rpi_ts_probe(pdev: *mut platform_device) -> c_int {
    static int rpi_ts_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct input_dev *input;
    struct rpi_firmware *fw;
    struct rpi_ts *ts;
    u32 touchbuf;
    int error;
    struct device_node *fw_node __free(device_node) = of_get_parent(np);
    if (!fw_node) {
    dev_err(dev, "Missing firmware node\n");
    return -ENOENT;
    }
    fw = devm_rpi_firmware_get(&pdev.dev, fw_node);
    if (!fw)
    return -EPROBE_DEFER;
    ts = devm_kzalloc(dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.pdev = pdev;
    ts.fw_regs_va = dma_alloc_coherent(dev, PAGE_SIZE, &ts.fw_regs_phys,
    GFP_KERNEL);
    if (!ts.fw_regs_va) {
    dev_err(dev, "failed to dma_alloc_coherent\n");
    return -ENOMEM;
    }
    error = devm_add_action_or_reset(dev, rpi_ts_dma_cleanup, ts);
    if (error) {
    dev_err(dev, "failed to devm_add_action_or_reset, %d\n", error);
    return error;
    }
    touchbuf = (u32)ts.fw_regs_phys;
    error = rpi_firmware_property(fw, RPI_FIRMWARE_FRAMEBUFFER_SET_TOUCHBUF,
    &touchbuf, sizeof(touchbuf));
    if (error || touchbuf != 0) {
    dev_warn(dev, "Failed to set touchbuf, %d\n", error);
    return error;
    }
    input = devm_input_allocate_device(dev);
    if (!input) {
    dev_err(dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    ts.input = input;
    input_set_drvdata(input, ts);
    input.name = "raspberrypi-ts";
    input.id.bustype = BUS_HOST;
    input_set_abs_params(input, ABS_MT_POSITION_X, 0,
    RPI_TS_DEFAULT_WIDTH, 0, 0);
    input_set_abs_params(input, ABS_MT_POSITION_Y, 0,
    RPI_TS_DEFAULT_HEIGHT, 0, 0);
    touchscreen_parse_properties(input, true, &ts.prop);
    error = input_mt_init_slots(input, RPI_TS_MAX_SUPPORTED_POINTS,
    INPUT_MT_DIRECT);
    if (error) {
    dev_err(dev, "could not init mt slots, %d\n", error);
    return error;
    }
    error = input_setup_polling(input, rpi_ts_poll);
    if (error) {
    dev_err(dev, "could not set up polling mode, %d\n", error);
    return error;
    }
    input_set_poll_interval(input, RPI_TS_POLL_INTERVAL);
    error = input_register_device(input);
    if (error) {
    dev_err(dev, "could not register input device, %d\n", error);
    return error;
    }
    return 0;
    }
    static const struct of_device_id rpi_ts_match[] = {
    { .compatible = "raspberrypi,firmware-ts", },
    {},
    };
    MODULE_DEVICE_TABLE(of, rpi_ts_match);
    static struct platform_driver rpi_ts_driver = {
    .driver = {
    .name = "raspberrypi-ts",
    .of_match_table = rpi_ts_match,
    },
    .probe = rpi_ts_probe,
    };
    module_platform_driver(rpi_ts_driver);
    MODULE_AUTHOR("Gordon Hollingworth");
    MODULE_AUTHOR("Nicolas Saenz Julienne <nsaenzjulienne@suse.de>");
    MODULE_DESCRIPTION("Raspberry Pi firmware based touchscreen driver");
    MODULE_LICENSE("GPL v2");
