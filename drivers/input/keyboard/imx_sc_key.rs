//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/imx_sc_key.c
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
// Copyright 2019 NXP.
//

pub const DEBOUNCE_TIME: c_int = 30;
pub const REPEAT_INTERVAL: c_int = 60;
pub const SC_IRQ_BUTTON: c_int = 1;
pub const SC_IRQ_GROUP_WAKE: c_int = 3;
pub const IMX_SC_MISC_FUNC_GET_BUTTON_STATUS: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_key_drv_data {
    pub keycode: u32,
    pub /: *mut *mut bool keystate; / true: pressed, false: released,
    pub check_work: delayed_work,
    pub input: *mut input_dev,
    pub key_ipc_handle: *mut imx_sc_ipc,
    pub key_notifier: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_key {
    pub hdr: imx_sc_rpc_msg,
    pub state: u32,
}

    static int imx_sc_key_notify(struct notifier_block *nb,
    unsigned long event, void *group)
    {
    struct imx_key_drv_data *priv =
    container_of(nb,
    struct imx_key_drv_data,
    key_notifier);
    if ((event & SC_IRQ_BUTTON) && (*(u8 *)group == SC_IRQ_GROUP_WAKE)) {
    schedule_delayed_work(&priv.check_work,
    msecs_to_jiffies(DEBOUNCE_TIME));
    pm_wakeup_event(priv.input.dev.parent, 0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_sc_check_for_events(work: *mut work_struct) {
    static void imx_sc_check_for_events(struct work_struct *work)
    {
    struct imx_key_drv_data *priv =
    container_of(work,
    struct imx_key_drv_data,
    check_work.work);
    struct input_dev *input = priv.input;
    struct imx_sc_msg_key msg;
    struct imx_sc_rpc_msg *hdr = &msg.hdr;
    bool state;
    int error;
    hdr.ver = IMX_SC_RPC_VERSION;
    hdr.svc = IMX_SC_RPC_SVC_MISC;
    hdr.func = IMX_SC_MISC_FUNC_GET_BUTTON_STATUS;
    hdr.size = 1;
    error = imx_scu_call_rpc(priv.key_ipc_handle, &msg, true);
    if (error) {
    dev_err(&input.dev, "read imx sc key failed, error %d\n", error);
    return;
    }
//
// The response data from SCU firmware is 4 bytes,
// but ONLY the first byte is the key state, other
// 3 bytes could be some dirty data, so we should
// ONLY take the first byte as key state.
//
    state = (bool)(msg.state & 0xff);
    if (state ^ priv.keystate) {
    priv.keystate = state;
    input_event(input, EV_KEY, priv.keycode, state);
    input_sync(input);
    if (!priv.keystate)
    pm_relax(priv.input.dev.parent);
    }
    if (state)
    schedule_delayed_work(&priv.check_work,
    msecs_to_jiffies(REPEAT_INTERVAL));
    }
#[no_mangle]
unsafe extern "C" fn imx_sc_key_action(data: *mut c_void) {
    static void imx_sc_key_action(void *data)
    {
    struct imx_key_drv_data *priv = data;
    imx_scu_irq_group_enable(SC_IRQ_GROUP_WAKE, SC_IRQ_BUTTON, false);
    imx_scu_irq_unregister_notifier(&priv.key_notifier);
    cancel_delayed_work_sync(&priv.check_work);
    }
#[no_mangle]
unsafe extern "C" fn imx_sc_key_probe(pdev: *mut platform_device) -> c_int {
    static int imx_sc_key_probe(struct platform_device *pdev)
    {
    struct imx_key_drv_data *priv;
    struct input_dev *input;
    int error;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    error = imx_scu_get_handle(&priv.key_ipc_handle);
    if (error)
    return error;
    if (device_property_read_u32(&pdev.dev, "linux,keycodes",
    &priv.keycode)) {
    dev_err(&pdev.dev, "missing linux,keycodes property\n");
    return -EINVAL;
    }
    INIT_DELAYED_WORK(&priv.check_work, imx_sc_check_for_events);
    input = devm_input_allocate_device(&pdev.dev);
    if (!input) {
    dev_err(&pdev.dev, "failed to allocate the input device\n");
    return -ENOMEM;
    }
    input.name = pdev.name;
    input.phys = "imx-sc-key/input0";
    input.id.bustype = BUS_HOST;
    input_set_capability(input, EV_KEY, priv.keycode);
    error = input_register_device(input);
    if (error) {
    dev_err(&pdev.dev, "failed to register input device\n");
    return error;
    }
    priv.input = input;
    platform_set_drvdata(pdev, priv);
    error = imx_scu_irq_group_enable(SC_IRQ_GROUP_WAKE, SC_IRQ_BUTTON,
    true);
    if (error) {
    dev_err(&pdev.dev, "failed to enable scu group irq\n");
    return error;
    }
    error = devm_add_action_or_reset(&pdev.dev, imx_sc_key_action, priv);
    if (error)
    return error;
    priv.key_notifier.notifier_call = imx_sc_key_notify;
    error = imx_scu_irq_register_notifier(&priv.key_notifier);
    if (error)
    dev_err(&pdev.dev, "failed to register scu notifier\n");
    return error;
    }
    static const struct of_device_id imx_sc_key_ids[] = {
    { .compatible = "fsl,imx-sc-key" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx_sc_key_ids);
    static struct platform_driver imx_sc_key_driver = {
    .driver = {
    .name = "imx-sc-key",
    .of_match_table = imx_sc_key_ids,
    },
    .probe = imx_sc_key_probe,
    };
    module_platform_driver(imx_sc_key_driver);
    MODULE_AUTHOR("Anson Huang <Anson.Huang@nxp.com>");
    MODULE_DESCRIPTION("i.MX System Controller Key Driver");
    MODULE_LICENSE("GPL v2");
