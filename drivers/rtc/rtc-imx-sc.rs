//! Automatically rewritten from C to Rust
//! Source: drivers/rtc/rtc-imx-sc.c
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
// Copyright 2018 NXP.
//

pub const IMX_SC_TIMER_FUNC_GET_RTC_SEC1970: c_int = 9;
pub const IMX_SC_TIMER_FUNC_SET_RTC_ALARM: c_int = 8;
pub const IMX_SC_TIMER_FUNC_SET_RTC_TIME: c_int = 6;
pub const IMX_SIP_SRTC: c_uint = 0xC2000002;
pub const IMX_SIP_SRTC_SET_TIME: c_uint = 0x0;
pub const SC_IRQ_GROUP_RTC: c_int = 2;
pub const SC_IRQ_RTC: c_int = 1;
    static struct imx_sc_ipc *rtc_ipc_handle;
    static struct rtc_device *imx_sc_rtc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_timer_get_rtc_time {
    pub hdr: imx_sc_rpc_msg,
    pub time: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_timer_rtc_set_alarm {
    pub hdr: imx_sc_rpc_msg,
    pub year: u16,
    pub mon: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub __aligned(4): } __packed,
#[no_mangle]
unsafe extern "C" fn imx_sc_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int imx_sc_rtc_read_time(struct device *dev, struct rtc_time *tm)
    {
    pub msg: imx_sc_msg_timer_get_rtc_time,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub ret: c_int,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_TIMER: hdr->svc =,
    pub IMX_SC_TIMER_FUNC_GET_RTC_SEC1970: hdr->func =,
    pub 1: hdr->size =,
    pub true): ret = imx_scu_call_rpc(rtc_ipc_handle, &msg,,
    if (ret) {
    pub ret): dev_err(dev, "read rtc time failed, ret %d\n",,
    pub ret: return,
    }
    pub tm): rtc_time64_to_tm(msg.time,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn imx_sc_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int {
    static int imx_sc_rtc_set_time(struct device *dev, struct rtc_time *tm)
    {
    pub res: arm_smccc_res,
// pack 2 time parameters into 1 register, 16 bits for each
    arm_smccc_smc(IMX_SIP_SRTC, IMX_SIP_SRTC_SET_TIME,
    ((tm.tm_year + 1900) << 16) | (tm.tm_mon + 1),
    (tm.tm_mday << 16) | tm.tm_hour,
    (tm.tm_min << 16) | tm.tm_sec,
    pub &res): 0, 0, 0,,
    pub res.a0: return,
    }
#[no_mangle]
unsafe extern "C" fn imx_sc_rtc_alarm_irq_enable(dev: *mut device, enable: c_uint) -> c_int {
    static int imx_sc_rtc_alarm_irq_enable(struct device *dev, unsigned int enable)
    {
    pub enable): return imx_scu_irq_group_enable(SC_IRQ_GROUP_RTC, SC_IRQ_RTC,,
    }
#[no_mangle]
unsafe extern "C" fn imx_sc_rtc_set_alarm(dev: *mut device, alrm: *mut rtc_wkalrm) -> c_int {
    static int imx_sc_rtc_set_alarm(struct device *dev, struct rtc_wkalrm *alrm)
    {
    pub msg: imx_sc_msg_timer_rtc_set_alarm,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub ret: c_int,
    pub &alrm->time: *mut *mut rtc_time alrm_tm =,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_TIMER: hdr->svc =,
    pub IMX_SC_TIMER_FUNC_SET_RTC_ALARM: hdr->func =,
    pub 3: hdr->size =,
    pub 1900: msg.year = alrm_tm->tm_year +,
    pub 1: msg.mon = alrm_tm->tm_mon +,
    pub alrm_tm->tm_mday: msg.day =,
    pub alrm_tm->tm_hour: msg.hour =,
    pub alrm_tm->tm_min: msg.min =,
    pub alrm_tm->tm_sec: msg.sec =,
    pub true): ret = imx_scu_call_rpc(rtc_ipc_handle, &msg,,
    if (ret) {
    pub ret): dev_err(dev, "set rtc alarm failed, ret %d\n",,
    pub ret: return,
    }
    pub alrm->enabled): ret = imx_sc_rtc_alarm_irq_enable(dev,,
    if (ret) {
    pub ret): dev_err(dev, "enable rtc alarm failed, ret %d\n",,
    pub ret: return,
    }
    pub 0: return,
    }
    static const struct rtc_class_ops imx_sc_rtc_ops = {
    .read_time = imx_sc_rtc_read_time,
    .set_time = imx_sc_rtc_set_time,
    .set_alarm = imx_sc_rtc_set_alarm,
    .alarm_irq_enable = imx_sc_rtc_alarm_irq_enable,
}

    static int imx_sc_rtc_alarm_notify(struct notifier_block *nb,
    unsigned long event, void *group)
    {
// ignore non-rtc irq
    if (!((event & SC_IRQ_RTC) && (*(u8 *)group == SC_IRQ_GROUP_RTC)))
    return 0;
    rtc_update_irq(imx_sc_rtc, 1, RTC_IRQF | RTC_AF);
    return 0;
    }
    static struct notifier_block imx_sc_rtc_alarm_sc_notifier = {
    .notifier_call = imx_sc_rtc_alarm_notify,
    };
#[no_mangle]
unsafe extern "C" fn imx_sc_rtc_probe(pdev: *mut platform_device) -> c_int {
    static int imx_sc_rtc_probe(struct platform_device *pdev)
    {
    int ret;
    ret = imx_scu_get_handle(&rtc_ipc_handle);
    if (ret)
    return ret;
    device_init_wakeup(&pdev.dev, true);
    imx_sc_rtc = devm_rtc_allocate_device(&pdev.dev);
    if (IS_ERR(imx_sc_rtc))
    return PTR_ERR(imx_sc_rtc);
    imx_sc_rtc.ops = &imx_sc_rtc_ops;
    imx_sc_rtc.range_min = 0;
    imx_sc_rtc.range_max = U32_MAX;
    ret = devm_rtc_register_device(imx_sc_rtc);
    if (ret)
    return ret;
    imx_scu_irq_register_notifier(&imx_sc_rtc_alarm_sc_notifier);
    return 0;
    }
    static const struct of_device_id imx_sc_dt_ids[] = {
    { .compatible = "fsl,imx8qxp-sc-rtc", },
    {}
    };
    MODULE_DEVICE_TABLE(of, imx_sc_dt_ids);
    static struct platform_driver imx_sc_rtc_driver = {
    .driver = {
    .name	= "imx-sc-rtc",
    .of_match_table = imx_sc_dt_ids,
    },
    .probe		= imx_sc_rtc_probe,
    };
    module_platform_driver(imx_sc_rtc_driver);
    MODULE_AUTHOR("Anson Huang <Anson.Huang@nxp.com>");
    MODULE_DESCRIPTION("NXP i.MX System Controller RTC Driver");
    MODULE_LICENSE("GPL");
