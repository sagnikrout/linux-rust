//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/meson-ir.c
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
// Driver for Amlogic Meson IR remote receiver
//
// Copyright (C) 2014 Beniamino Galvani <b.galvani@gmail.com>
//

pub const IR_DEC_LDR_ACTIVE: c_uint = 0x00;

pub const IR_DEC_LDR_IDLE: c_uint = 0x04;

pub const IR_DEC_LDR_REPEAT: c_uint = 0x08;

pub const IR_DEC_BIT_0: c_uint = 0x0c;

pub const IR_DEC_REG0: c_uint = 0x10;

pub const IR_DEC_FRAME: c_uint = 0x14;
pub const IR_DEC_STATUS: c_uint = 0x18;

pub const IR_DEC_REG1: c_uint = 0x1c;

// Meson 6b uses REG1 to configure IR mode

// The following registers are only available on Meson 8b and newer
pub const IR_DEC_REG2: c_uint = 0x20;

// Meson 8b / GXBB use REG2 to configure IR mode

pub const IR_DEC_DURATN2: c_uint = 0x24;

pub const IR_DEC_DURATN3: c_uint = 0x28;

pub const IR_DEC_FRAME1: c_uint = 0x2c;

pub const DEC_MODE_NEC: c_uint = 0x0;
pub const DEC_MODE_RAW: c_uint = 0x2;
pub const DEC_MODE_RC6: c_uint = 0x9;
pub const DEC_MODE_XMP: c_uint = 0xE;
pub const DEC_MODE_UNKNOW: c_uint = 0xFF;

pub const IRQSEL_DEC_MODE: c_int = 0;
pub const IRQSEL_RISE_FALL: c_int = 1;
pub const IRQSEL_FALL: c_int = 2;
pub const IRQSEL_RISE: c_int = 3;

//
// struct meson_ir_protocol - describe IR Protocol parameter
//
// @hw_protocol: select IR Protocol from IR Controller
// @repeat_counter_enable: enable frame-to-frame time counter, it should work
// with @repeat_compare_enable to detect the repeat frame
// @repeat_check_enable: enable repeat time check for repeat detection
// @repeat_compare_enable: enable to compare frame for repeat frame detection.
// Some IR Protocol send the same data as repeat frame.
// In this case, it should work with
// @repeat_counter_enable to detect the repeat frame.
// @bit_order: bit order, LSB or MSB
// @bit1_match_enable: enable to check bit 1
// @hold_code_enable: hold frame code in register IR_DEC_FRAME1, the new one
// frame code will not be store in IR_DEC_FRAME1.
// until IR_DEC_FRAME1 has been read
// @count_tick_mode: increasing time unit of frame-to-frame time counter.
// 0 = 100us, 1 = 10us
// @code_length: length (N-1) of data frame
// @frame_time_max: max time for whole frame. Unit: MESON_HW_TRATE
// @leader_active_max: max time for NEC/RC6 leader active part. Unit: MESON_HW_TRATE
// @leader_active_min: min time for NEC/RC6 leader active part. Unit: MESON_HW_TRATE
// @leader_idle_max: max time for NEC/RC6 leader idle part. Unit: MESON_HW_TRATE
// @leader_idle_min: min time for NEC/RC6 leader idle part. Unit: MESON_HW_TRATE
// @repeat_leader_max: max time for NEC repeat leader idle part. Unit: MESON_HW_TRATE
// @repeat_leader_min: min time for NEC repeat leader idle part. Unit: MESON_HW_TRATE
// @bit0_max: max time for NEC Logic '0', half of RC6 trailer bit, XMP Logic '00'
// @bit0_min: min time for NEC Logic '0', half of RC6 trailer bit, XMP Logic '00'
// @bit1_max: max time for NEC Logic '1', whole of RC6 trailer bit, XMP Logic '01'
// @bit1_min: min time for NEC Logic '1', whole of RC6 trailer bit, XMP Logic '01'
// @duration2_max: max time for half of RC6 normal bit, XMP Logic '10'
// @duration2_min: min time for half of RC6 normal bit, XMP Logic '10'
// @duration3_max: max time for whole of RC6 normal bit, XMP Logic '11'
// @duration3_min: min time for whole of RC6 normal bit, XMP Logic '11'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_ir_protocol {
    pub hw_protocol: u8,
    pub repeat_counter_enable: bool,
    pub repeat_check_enable: bool,
    pub repeat_compare_enable: bool,
    pub bit_order: bool,
    pub bit1_match_enable: bool,
    pub hold_code_enable: bool,
    pub count_tick_mode: bool,
    pub code_length: u8,
    pub frame_time_max: u16,
    pub leader_active_max: u16,
    pub leader_active_min: u16,
    pub leader_idle_max: u16,
    pub leader_idle_min: u16,
    pub repeat_leader_max: u16,
    pub repeat_leader_min: u16,
    pub bit0_max: u16,
    pub bit0_min: u16,
    pub bit1_max: u16,
    pub bit1_min: u16,
    pub duration2_max: u16,
    pub duration2_min: u16,
    pub duration3_max: u16,
    pub duration3_min: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_ir_param {
    pub support_hw_decoder: bool,
    pub max_register: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_ir {
    pub param: *const meson_ir_param,
    pub reg: *mut regmap,
    pub rc: *mut rc_dev,
    pub lock: spinlock_t,
}

    static struct regmap_config meson_ir_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
    static const struct meson_ir_protocol protocol_timings[] = {
// protocol, repeat counter, repeat check, repeat compare, order
    {DEC_MODE_NEC, false, false, false, FRAME_LSB_FIRST,
// bit 1 match, hold code, count tick, len, frame time
    true, false, false, 32, 4000,
// leader active max/min, leader idle max/min, repeat leader max/min
    500, 400, 300, 200, 150, 80,
// bit0 max/min, bit1 max/min, duration2 max/min, duration3 max/min
    72, 40, 134, 90, 0, 0, 0, 0}
    };
#[no_mangle]
unsafe extern "C" fn meson_ir_nec_handler(ir: *mut meson_ir) {
    static void meson_ir_nec_handler(struct meson_ir *ir)
    {
    let mut code: u32 = 0;
    let mut status: u32 = 0;
    enum rc_proto proto;
    regmap_read(ir.reg, IR_DEC_STATUS, &status);
    if (status & DEC_STATUS_REPEAT) {
    rc_repeat(ir.rc);
    } else {
    regmap_read(ir.reg, IR_DEC_FRAME, &code);
    code = ir_nec_bytes_to_scancode(code, code >> 8,
    code >> 16, code >> 24, &proto);
    rc_keydown(ir.rc, proto, code, 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn meson_ir_hw_handler(ir: *mut meson_ir) {
    static void meson_ir_hw_handler(struct meson_ir *ir)
    {
    if (ir.rc.enabled_protocols & RC_PROTO_BIT_NEC)
    meson_ir_nec_handler(ir);
    }
#[no_mangle]
unsafe extern "C" fn meson_ir_irq(irqno: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t meson_ir_irq(int irqno, void *dev_id)
    {
    struct meson_ir *ir = dev_id;
    u32 duration, status;
    let mut rawir: ir_raw_event = {};
    spin_lock(&ir.lock);
    regmap_read(ir.reg, IR_DEC_STATUS, &status);
    if (ir.rc.driver_type == RC_DRIVER_IR_RAW) {
    rawir.pulse = !!(status & IR_DEC_STATUS_PULSE);
    regmap_read(ir.reg, IR_DEC_REG1, &duration);
    duration = FIELD_GET(IR_DEC_REG1_TIME_IV, duration);
    rawir.duration = duration * MESON_RAW_TRATE;
    ir_raw_event_store_with_timeout(ir.rc, &rawir);
    } else if (ir.rc.driver_type == RC_DRIVER_SCANCODE) {
    if (status & DEC_STATUS_VALID)
    meson_ir_hw_handler(ir);
    }
    spin_unlock(&ir.lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn meson_ir_hw_decoder_init(dev: *mut rc_dev, rc_type: *mut u64) -> c_int {
    static int meson_ir_hw_decoder_init(struct rc_dev *dev, u64 *rc_type)
    {
    u8 protocol;
    u32 regval;
    int i;
    unsigned long flags;
    const struct meson_ir_protocol *timings;
    struct meson_ir *ir = dev.priv;
    if (*rc_type & RC_PROTO_BIT_NEC)
    protocol = DEC_MODE_NEC;
    else
    return 0;
    for (i = 0; i < ARRAY_SIZE(protocol_timings); i++)
    if (protocol_timings[i].hw_protocol == protocol)
    break;
    if (i == ARRAY_SIZE(protocol_timings)) {
    dev_err(&dev.dev, "hw protocol isn't supported: %d\n",
    protocol);
    return -EINVAL;
    }
    timings = &protocol_timings[i];
    spin_lock_irqsave(&ir.lock, flags);
// Clear controller status
    regmap_read(ir.reg, IR_DEC_STATUS, &regval);
    regmap_read(ir.reg, IR_DEC_FRAME, &regval);
// Reset ir decoder and disable decoder
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_ENABLE, 0);
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_RESET,
    IR_DEC_REG1_RESET);
// Base time resolution, (19+1)*1us=20us
    regval = FIELD_PREP(IR_DEC_REG0_BASE_TIME, MESON_HW_TRATE - 1);
    regmap_update_bits(ir.reg, IR_DEC_REG0, IR_DEC_REG0_BASE_TIME, regval);
// Monitor timing for input filter
    regmap_update_bits(ir.reg, IR_DEC_REG0, IR_DEC_REG0_FILTER,
    FIELD_PREP(IR_DEC_REG0_FILTER, 7));
// HW protocol
    regval = FIELD_PREP(IR_DEC_REG2_MODE, timings.hw_protocol);
    regmap_update_bits(ir.reg, IR_DEC_REG2, IR_DEC_REG2_MODE, regval);
// Hold frame data until register was read
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_HOLD_CODE,
    timings.hold_code_enable ?
    IR_DEC_REG1_HOLD_CODE : 0);
// Bit order
    regmap_update_bits(ir.reg, IR_DEC_REG2, IR_DEC_REG2_BIT_ORDER,
    timings.bit_order ? IR_DEC_REG2_BIT_ORDER : 0);
// Select tick mode
    regmap_update_bits(ir.reg, IR_DEC_REG2, IR_DEC_REG2_TICK_MODE,
    timings.count_tick_mode ?
    IR_DEC_REG2_TICK_MODE : 0);
//
// Some protocols transmit the same data frame as repeat frame
// when the key is pressing. In this case, it could be detected as
// repeat frame if the repeat checker was enabled.
//
    regmap_update_bits(ir.reg, IR_DEC_REG2, IR_DEC_REG2_REPEAT_COUNTER,
    timings.repeat_counter_enable ?
    IR_DEC_REG2_REPEAT_COUNTER : 0);
    regmap_update_bits(ir.reg, IR_DEC_REG2, IR_DEC_REG2_REPEAT_TIME,
    timings.repeat_check_enable ?
    IR_DEC_REG2_REPEAT_TIME : 0);
    regmap_update_bits(ir.reg, IR_DEC_REG2, IR_DEC_REG2_COMPARE_FRAME,
    timings.repeat_compare_enable ?
    IR_DEC_REG2_COMPARE_FRAME : 0);
//
// FRAME_TIME_MAX should be larger than the time between
// data frame and repeat frame
//
    regval = FIELD_PREP(IR_DEC_REG0_FRAME_TIME_MAX,
    timings.frame_time_max);
    regmap_update_bits(ir.reg, IR_DEC_REG0, IR_DEC_REG0_FRAME_TIME_MAX,
    regval);
// Length(N-1) of data frame
    regval = FIELD_PREP(IR_DEC_REG1_FRAME_LEN, timings.code_length - 1);
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_FRAME_LEN, regval);
// Time for leader active part
    regval = FIELD_PREP(IR_DEC_LDR_ACTIVE_MAX,
    timings.leader_active_max) |
    FIELD_PREP(IR_DEC_LDR_ACTIVE_MIN,
    timings.leader_active_min);
    regmap_update_bits(ir.reg, IR_DEC_LDR_ACTIVE, IR_DEC_LDR_ACTIVE_MAX |
    IR_DEC_LDR_ACTIVE_MIN, regval);
// Time for leader idle part
    regval = FIELD_PREP(IR_DEC_LDR_IDLE_MAX, timings.leader_idle_max) |
    FIELD_PREP(IR_DEC_LDR_IDLE_MIN, timings.leader_idle_min);
    regmap_update_bits(ir.reg, IR_DEC_LDR_IDLE,
    IR_DEC_LDR_IDLE_MAX | IR_DEC_LDR_IDLE_MIN, regval);
// Time for repeat leader idle part
    regval = FIELD_PREP(IR_DEC_LDR_REPEAT_MAX, timings.repeat_leader_max) |
    FIELD_PREP(IR_DEC_LDR_REPEAT_MIN, timings.repeat_leader_min);
    regmap_update_bits(ir.reg, IR_DEC_LDR_REPEAT, IR_DEC_LDR_REPEAT_MAX |
    IR_DEC_LDR_REPEAT_MIN, regval);
//
// NEC: Time for logic '0'
// RC6: Time for half of trailer bit
//
    regval = FIELD_PREP(IR_DEC_BIT_0_MAX, timings.bit0_max) |
    FIELD_PREP(IR_DEC_BIT_0_MIN, timings.bit0_min);
    regmap_update_bits(ir.reg, IR_DEC_BIT_0,
    IR_DEC_BIT_0_MAX | IR_DEC_BIT_0_MIN, regval);
//
// NEC: Time for logic '1'
// RC6: Time for whole of trailer bit
//
    regval = FIELD_PREP(IR_DEC_STATUS_BIT_1_MAX, timings.bit1_max) |
    FIELD_PREP(IR_DEC_STATUS_BIT_1_MIN, timings.bit1_min);
    regmap_update_bits(ir.reg, IR_DEC_STATUS, IR_DEC_STATUS_BIT_1_MAX |
    IR_DEC_STATUS_BIT_1_MIN, regval);
// Enable to match logic '1'
    regmap_update_bits(ir.reg, IR_DEC_STATUS, IR_DEC_STATUS_BIT_1_ENABLE,
    timings.bit1_match_enable ?
    IR_DEC_STATUS_BIT_1_ENABLE : 0);
//
// NEC: Unused
// RC6: Time for halt of logic 0/1
//
    regval = FIELD_PREP(IR_DEC_DURATN2_MAX, timings.duration2_max) |
    FIELD_PREP(IR_DEC_DURATN2_MIN, timings.duration2_min);
    regmap_update_bits(ir.reg, IR_DEC_DURATN2,
    IR_DEC_DURATN2_MAX | IR_DEC_DURATN2_MIN, regval);
//
// NEC: Unused
// RC6: Time for whole logic 0/1
//
    regval = FIELD_PREP(IR_DEC_DURATN3_MAX, timings.duration3_max) |
    FIELD_PREP(IR_DEC_DURATN3_MIN, timings.duration3_min);
    regmap_update_bits(ir.reg, IR_DEC_DURATN3,
    IR_DEC_DURATN3_MAX | IR_DEC_DURATN3_MIN, regval);
// Reset ir decoder and enable decode
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_RESET,
    IR_DEC_REG1_RESET);
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_RESET, 0);
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_ENABLE,
    IR_DEC_REG1_ENABLE);
    spin_unlock_irqrestore(&ir.lock, flags);
    dev_info(&dev.dev, "hw decoder init, protocol: %d\n", protocol);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_ir_sw_decoder_init(dev: *mut rc_dev) {
    static void meson_ir_sw_decoder_init(struct rc_dev *dev)
    {
    unsigned long flags;
    struct meson_ir *ir = dev.priv;
    spin_lock_irqsave(&ir.lock, flags);
// Reset the decoder
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_RESET,
    IR_DEC_REG1_RESET);
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_RESET, 0);
// Set general operation mode (= raw/software decoding)
    if (of_device_is_compatible(dev.dev.of_node, "amlogic,meson6-ir"))
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_MODE,
    FIELD_PREP(IR_DEC_REG1_MODE,
    DEC_MODE_RAW));
    else
    regmap_update_bits(ir.reg, IR_DEC_REG2, IR_DEC_REG2_MODE,
    FIELD_PREP(IR_DEC_REG2_MODE,
    DEC_MODE_RAW));
// Set rate
    regmap_update_bits(ir.reg, IR_DEC_REG0, IR_DEC_REG0_BASE_TIME,
    FIELD_PREP(IR_DEC_REG0_BASE_TIME,
    MESON_RAW_TRATE - 1));
// IRQ on rising and falling edges
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_IRQSEL,
    FIELD_PREP(IR_DEC_REG1_IRQSEL, IRQSEL_RISE_FALL));
// Enable the decoder
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_ENABLE,
    IR_DEC_REG1_ENABLE);
    spin_unlock_irqrestore(&ir.lock, flags);
    dev_info(&dev.dev, "sw decoder init\n");
    }
#[no_mangle]
unsafe extern "C" fn meson_ir_probe(pdev: *mut platform_device) -> c_int {
    static int meson_ir_probe(struct platform_device *pdev)
    {
    const struct meson_ir_param *match_data;
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    void __iomem *res_start;
    const char *map_name;
    struct meson_ir *ir;
    int irq, ret;
    ir = devm_kzalloc(dev, sizeof(struct meson_ir), GFP_KERNEL);
    if (!ir)
    return -ENOMEM;
    match_data = of_device_get_match_data(dev);
    if (!match_data)
    return dev_err_probe(dev, -ENODEV, "failed to get match data\n");
    ir.param = match_data;
    res_start = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(res_start))
    return PTR_ERR(res_start);
    meson_ir_regmap_config.max_register = ir.param.max_register;
    ir.reg = devm_regmap_init_mmio(&pdev.dev, res_start,
    &meson_ir_regmap_config);
    if (IS_ERR(ir.reg))
    return PTR_ERR(ir.reg);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    if (ir.param.support_hw_decoder)
    ir.rc = devm_rc_allocate_device(&pdev.dev,
    RC_DRIVER_SCANCODE);
    else
    ir.rc = devm_rc_allocate_device(&pdev.dev, RC_DRIVER_IR_RAW);
    if (!ir.rc) {
    dev_err(dev, "failed to allocate rc device\n");
    return -ENOMEM;
    }
    if (ir.rc.driver_type == RC_DRIVER_IR_RAW) {
    ir.rc.allowed_protocols = RC_PROTO_BIT_ALL_IR_DECODER;
    ir.rc.rx_resolution = MESON_RAW_TRATE;
    ir.rc.min_timeout = 1;
    ir.rc.timeout = IR_DEFAULT_TIMEOUT;
    ir.rc.max_timeout = 10 * IR_DEFAULT_TIMEOUT;
    } else if (ir.rc.driver_type == RC_DRIVER_SCANCODE) {
    ir.rc.allowed_protocols = RC_PROTO_BIT_NEC;
    ir.rc.change_protocol = meson_ir_hw_decoder_init;
    }
    ir.rc.priv = ir;
    ir.rc.device_name = DRIVER_NAME;
    ir.rc.input_phys = DRIVER_NAME "/input0";
    ir.rc.input_id.bustype = BUS_HOST;
    map_name = of_get_property(node, "linux,rc-map-name", core::ptr::null_mut());
    ir.rc.map_name = map_name ? map_name : RC_MAP_EMPTY;
    ir.rc.driver_name = DRIVER_NAME;
    spin_lock_init(&ir.lock);
    platform_set_drvdata(pdev, ir);
    ret = devm_rc_register_device(dev, ir.rc);
    if (ret) {
    dev_err(dev, "failed to register rc device\n");
    return ret;
    }
    if (ir.rc.driver_type == RC_DRIVER_IR_RAW)
    meson_ir_sw_decoder_init(ir.rc);
    ret = devm_request_irq(dev, irq, meson_ir_irq, 0, "meson_ir", ir);
    if (ret) {
    dev_err(dev, "failed to request irq\n");
    return ret;
    }
    dev_info(dev, "receiver initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_ir_remove(pdev: *mut platform_device) {
    static void meson_ir_remove(struct platform_device *pdev)
    {
    struct meson_ir *ir = platform_get_drvdata(pdev);
    unsigned long flags;
// Disable the decoder
    spin_lock_irqsave(&ir.lock, flags);
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_ENABLE, 0);
    spin_unlock_irqrestore(&ir.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn meson_ir_shutdown(pdev: *mut platform_device) {
    static void meson_ir_shutdown(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct meson_ir *ir = platform_get_drvdata(pdev);
    unsigned long flags;
    spin_lock_irqsave(&ir.lock, flags);
//
// Set operation mode to NEC/hardware decoding to give
// bootloader a chance to power the system back on
//
    if (of_device_is_compatible(node, "amlogic,meson6-ir"))
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_MODE,
    FIELD_PREP(IR_DEC_REG1_MODE, DEC_MODE_NEC));
    else
    regmap_update_bits(ir.reg, IR_DEC_REG2, IR_DEC_REG2_MODE,
    FIELD_PREP(IR_DEC_REG2_MODE, DEC_MODE_NEC));
// Set rate to default value
    regmap_update_bits(ir.reg, IR_DEC_REG0, IR_DEC_REG0_BASE_TIME,
    FIELD_PREP(IR_DEC_REG0_BASE_TIME,
    MESON_HW_TRATE - 1));
    spin_unlock_irqrestore(&ir.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn meson_ir_resume(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int meson_ir_resume(struct device *dev)
    {
    struct meson_ir *ir = dev_get_drvdata(dev);
    if (ir.param.support_hw_decoder)
    meson_ir_hw_decoder_init(ir.rc, &ir.rc.enabled_protocols);
    else
    meson_ir_sw_decoder_init(ir.rc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson_ir_suspend(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int meson_ir_suspend(struct device *dev)
    {
    struct meson_ir *ir = dev_get_drvdata(dev);
    unsigned long flags;
    spin_lock_irqsave(&ir.lock, flags);
    regmap_update_bits(ir.reg, IR_DEC_REG1, IR_DEC_REG1_ENABLE, 0);
    spin_unlock_irqrestore(&ir.lock, flags);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(meson_ir_pm_ops, meson_ir_suspend, meson_ir_resume);
    static const struct meson_ir_param meson6_ir_param = {
    .support_hw_decoder = false,
    .max_register = IR_DEC_REG1,
    };
    static const struct meson_ir_param meson8b_ir_param = {
    .support_hw_decoder = false,
    .max_register = IR_DEC_REG2,
    };
    static const struct meson_ir_param meson_s4_ir_param = {
    .support_hw_decoder = true,
    .max_register = IR_DEC_FRAME1,
    };
    static const struct of_device_id meson_ir_match[] = {
    {
    .compatible = "amlogic,meson6-ir",
    .data = &meson6_ir_param,
    }, {
    .compatible = "amlogic,meson8b-ir",
    .data = &meson8b_ir_param,
    }, {
    .compatible = "amlogic,meson-gxbb-ir",
    .data = &meson8b_ir_param,
    }, {
    .compatible = "amlogic,meson-s4-ir",
    .data = &meson_s4_ir_param,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, meson_ir_match);
    static struct platform_driver meson_ir_driver = {
    .probe		= meson_ir_probe,
    .remove		= meson_ir_remove,
    .shutdown	= meson_ir_shutdown,
    .driver = {
    .name		= DRIVER_NAME,
    .of_match_table	= meson_ir_match,
    .pm = pm_ptr(&meson_ir_pm_ops),
    },
    };
    module_platform_driver(meson_ir_driver);
    MODULE_DESCRIPTION("Amlogic Meson IR remote receiver driver");
    MODULE_AUTHOR("Beniamino Galvani <b.galvani@gmail.com>");
    MODULE_LICENSE("GPL v2");
