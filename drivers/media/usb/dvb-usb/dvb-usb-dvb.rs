//! Automatically rewritten from C to Rust
//! Source: drivers/media/usb/dvb-usb/dvb-usb-dvb.c
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
// dvb-usb-dvb.c is part of the DVB USB library.
//
// Copyright (C) 2004-6 Patrick Boettcher (patrick.boettcher@posteo.de)
// see dvb-usb-init.c for copyright information.
//
// This file contains functions for initializing and handling the
// linux-dvb API.
//

// does the complete input transfer handling
#[no_mangle]
unsafe extern "C" fn dvb_usb_ctrl_feed(dvbdmxfeed: *mut dvb_demux_feed, onoff: c_int) -> c_int {
    static int dvb_usb_ctrl_feed(struct dvb_demux_feed *dvbdmxfeed, int onoff)
    {
    struct dvb_usb_adapter *adap = dvbdmxfeed.demux.priv;
    int newfeedcount, ret;
    if (adap == core::ptr::null_mut())
    return -ENODEV;
    if ((adap.active_fe < 0) ||
    (adap.active_fe >= adap.num_frontends_initialized)) {
    return -EINVAL;
    }
    newfeedcount = adap.feedcount + (onoff ? 1 : -1);
// stop feed before setting a new pid if there will be no pid anymore
    if (newfeedcount == 0) {
    deb_ts("stop feeding\n");
    usb_urb_kill(&adap.fe_adap[adap.active_fe].stream);
    if (adap.props.fe[adap.active_fe].streaming_ctrl != core::ptr::null_mut()) {
    ret = adap.props.fe[adap.active_fe].streaming_ctrl(adap, 0);
    if (ret < 0) {
    err("error while stopping stream.");
    return ret;
    }
    }
    }
    adap.feedcount = newfeedcount;
// activate the pid on the device specific pid_filter
    deb_ts("setting pid (%s): %5d %04x at index %d '%s'\n",
    adap.fe_adap[adap.active_fe].pid_filtering ?
    "yes" : "no", dvbdmxfeed.pid, dvbdmxfeed.pid,
    dvbdmxfeed.index, onoff ? "on" : "off");
    if (adap.props.fe[adap.active_fe].caps & DVB_USB_ADAP_HAS_PID_FILTER &&
    adap.fe_adap[adap.active_fe].pid_filtering &&
    adap.props.fe[adap.active_fe].pid_filter != core::ptr::null_mut())
    adap.props.fe[adap.active_fe].pid_filter(adap, dvbdmxfeed.index, dvbdmxfeed.pid, onoff);
// start the feed if this was the first feed and there is still a feed
// for reception.
//
    if (adap.feedcount == onoff && adap.feedcount > 0) {
    deb_ts("controlling pid parser\n");
    if (adap.props.fe[adap.active_fe].caps & DVB_USB_ADAP_HAS_PID_FILTER &&
    adap.props.fe[adap.active_fe].caps &
    DVB_USB_ADAP_PID_FILTER_CAN_BE_TURNED_OFF &&
    adap.props.fe[adap.active_fe].pid_filter_ctrl != core::ptr::null_mut()) {
    ret = adap.props.fe[adap.active_fe].pid_filter_ctrl(adap,
    adap.fe_adap[adap.active_fe].pid_filtering);
    if (ret < 0) {
    err("could not handle pid_parser");
    return ret;
    }
    }
    deb_ts("start feeding\n");
    if (adap.props.fe[adap.active_fe].streaming_ctrl != core::ptr::null_mut()) {
    ret = adap.props.fe[adap.active_fe].streaming_ctrl(adap, 1);
    if (ret < 0) {
    err("error while enabling fifo.");
    return ret;
    }
    }
    deb_ts("submitting all URBs\n");
    usb_urb_submit(&adap.fe_adap[adap.active_fe].stream);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dvb_usb_start_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int {
    static int dvb_usb_start_feed(struct dvb_demux_feed *dvbdmxfeed)
    {
    deb_ts("start pid: 0x%04x, feedtype: %d\n", dvbdmxfeed.pid,
    dvbdmxfeed.type);
    return dvb_usb_ctrl_feed(dvbdmxfeed, 1);
    }
#[no_mangle]
unsafe extern "C" fn dvb_usb_stop_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int {
    static int dvb_usb_stop_feed(struct dvb_demux_feed *dvbdmxfeed)
    {
    deb_ts("stop pid: 0x%04x, feedtype: %d\n", dvbdmxfeed.pid, dvbdmxfeed.type);
    return dvb_usb_ctrl_feed(dvbdmxfeed, 0);
    }
#[no_mangle]
unsafe extern "C" fn dvb_usb_media_device_init(adap: *mut dvb_usb_adapter) -> c_int {
    static int dvb_usb_media_device_init(struct dvb_usb_adapter *adap)
    {

    struct media_device *mdev;
    struct dvb_usb_device *d = adap.dev;
    struct usb_device *udev = d.udev;
    mdev = kzalloc_obj(*mdev);
    if (!mdev)
    return -ENOMEM;
    media_device_usb_init(mdev, udev, d.desc.name);
    dvb_register_media_controller(&adap.dvb_adap, mdev);
    dev_info(&d.udev.dev, "media controller created\n");

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dvb_usb_media_device_register(adap: *mut dvb_usb_adapter) -> c_int {
    static int  dvb_usb_media_device_register(struct dvb_usb_adapter *adap)
    {

    return media_device_register(adap.dvb_adap.mdev);

    return 0;

    }
#[no_mangle]
unsafe extern "C" fn dvb_usb_media_device_unregister(adap: *mut dvb_usb_adapter) {
    static void dvb_usb_media_device_unregister(struct dvb_usb_adapter *adap)
    {

    if (!adap.dvb_adap.mdev)
    return;
    mutex_lock(&adap.dvb_adap.mdev_lock);
    media_device_unregister(adap.dvb_adap.mdev);
    media_device_cleanup(adap.dvb_adap.mdev);
    kfree(adap.dvb_adap.mdev);
    adap.dvb_adap.mdev = core::ptr::null_mut();
    mutex_unlock(&adap.dvb_adap.mdev_lock);

    }
#[no_mangle]
pub unsafe extern "C" fn dvb_usb_adapter_dvb_init(adap: *mut dvb_usb_adapter, adapter_nums: *mut c_short) -> c_int {
    int dvb_usb_adapter_dvb_init(struct dvb_usb_adapter *adap, short *adapter_nums)
    {
    int i;
    int ret = dvb_register_adapter(&adap.dvb_adap, adap.dev.desc.name,
    adap.dev.owner, &adap.dev.udev.dev,
    adapter_nums);
    if (ret < 0) {
    deb_info("dvb_register_adapter failed: error %d", ret);
    goto err;
    }
    adap.dvb_adap.priv = adap;
    ret = dvb_usb_media_device_init(adap);
    if (ret < 0) {
    deb_info("dvb_usb_media_device_init failed: error %d", ret);
    goto err_mc;
    }
    if (adap.dev.props.read_mac_address) {
    if (adap.dev.props.read_mac_address(adap.dev, adap.dvb_adap.proposed_mac) == 0)
    info("MAC address: %pM", adap.dvb_adap.proposed_mac);
    else
    err("MAC address reading failed.");
    }
    adap.demux.dmx.capabilities = DMX_TS_FILTERING | DMX_SECTION_FILTERING;
    adap.demux.priv             = adap;
    adap.demux.filternum        = 0;
    for (i = 0; i < adap.props.num_frontends; i++) {
    if (adap.demux.filternum < adap.fe_adap[i].max_feed_count)
    adap.demux.filternum = adap.fe_adap[i].max_feed_count;
    }
    adap.demux.feednum          = adap.demux.filternum;
    adap.demux.start_feed       = dvb_usb_start_feed;
    adap.demux.stop_feed        = dvb_usb_stop_feed;
    adap.demux.write_to_decoder = core::ptr::null_mut();
    if ((ret = dvb_dmx_init(&adap.demux)) < 0) {
    err("dvb_dmx_init failed: error %d", ret);
    goto err_dmx;
    }
    adap.dmxdev.filternum       = adap.demux.filternum;
    adap.dmxdev.demux           = &adap.demux.dmx;
    adap.dmxdev.capabilities    = 0;
    if ((ret = dvb_dmxdev_init(&adap.dmxdev, &adap.dvb_adap)) < 0) {
    err("dvb_dmxdev_init failed: error %d", ret);
    goto err_dmx_dev;
    }
    if ((ret = dvb_net_init(&adap.dvb_adap, &adap.dvb_net,
    &adap.demux.dmx)) < 0) {
    err("dvb_net_init failed: error %d", ret);
    goto err_net_init;
    }
    adap.state |= DVB_USB_ADAP_STATE_DVB;
    return 0;
    err_net_init:
    dvb_dmxdev_release(&adap.dmxdev);
    err_dmx_dev:
    dvb_dmx_release(&adap.demux);
    err_dmx:
    dvb_usb_media_device_unregister(adap);
    err_mc:
    dvb_unregister_adapter(&adap.dvb_adap);
    err:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_usb_adapter_dvb_exit(adap: *mut dvb_usb_adapter) -> c_int {
    int dvb_usb_adapter_dvb_exit(struct dvb_usb_adapter *adap)
    {
    if (adap.state & DVB_USB_ADAP_STATE_DVB) {
    deb_info("unregistering DVB part\n");
    dvb_net_release(&adap.dvb_net);
    adap.demux.dmx.close(&adap.demux.dmx);
    dvb_dmxdev_release(&adap.dmxdev);
    dvb_dmx_release(&adap.demux);
    dvb_usb_media_device_unregister(adap);
    dvb_unregister_adapter(&adap.dvb_adap);
    adap.state &= ~DVB_USB_ADAP_STATE_DVB;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dvb_usb_set_active_fe(fe: *mut dvb_frontend, onoff: c_int) -> c_int {
    static int dvb_usb_set_active_fe(struct dvb_frontend *fe, int onoff)
    {
    struct dvb_usb_adapter *adap = fe.dvb.priv;
    int ret = (adap.props.frontend_ctrl) ?
    adap.props.frontend_ctrl(fe, onoff) : 0;
    if (ret < 0) {
    err("frontend_ctrl request failed");
    return ret;
    }
    if (onoff)
    adap.active_fe = fe.id;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dvb_usb_fe_wakeup(fe: *mut dvb_frontend) -> c_int {
    static int dvb_usb_fe_wakeup(struct dvb_frontend *fe)
    {
    struct dvb_usb_adapter *adap = fe.dvb.priv;
    dvb_usb_device_power_ctrl(adap.dev, 1);
    dvb_usb_set_active_fe(fe, 1);
    if (adap.fe_adap[fe.id].fe_init)
    adap.fe_adap[fe.id].fe_init(fe);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dvb_usb_fe_sleep(fe: *mut dvb_frontend) -> c_int {
    static int dvb_usb_fe_sleep(struct dvb_frontend *fe)
    {
    struct dvb_usb_adapter *adap = fe.dvb.priv;
    if (adap.fe_adap[fe.id].fe_sleep)
    adap.fe_adap[fe.id].fe_sleep(fe);
    dvb_usb_set_active_fe(fe, 0);
    return dvb_usb_device_power_ctrl(adap.dev, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_usb_adapter_frontend_init(adap: *mut dvb_usb_adapter) -> c_int {
    int dvb_usb_adapter_frontend_init(struct dvb_usb_adapter *adap)
    {
    int ret, i;
// register all given adapter frontends
    for (i = 0; i < adap.props.num_frontends; i++) {
    if (adap.props.fe[i].frontend_attach == core::ptr::null_mut()) {
    err("strange: '%s' #%d,%d doesn't want to attach a frontend.",
    adap.dev.desc.name, adap.id, i);
    return 0;
    }
    ret = adap.props.fe[i].frontend_attach(adap);
    if (ret || adap.fe_adap[i].fe == core::ptr::null_mut()) {
// only print error when there is no FE at all
    if (i == 0)
    err("no frontend was attached by '%s'",
    adap.dev.desc.name);
    return 0;
    }
    adap.fe_adap[i].fe.id = i;
// re-assign sleep and wakeup functions
    adap.fe_adap[i].fe_init = adap.fe_adap[i].fe.ops.init;
    adap.fe_adap[i].fe.ops.init  = dvb_usb_fe_wakeup;
    adap.fe_adap[i].fe_sleep = adap.fe_adap[i].fe.ops.sleep;
    adap.fe_adap[i].fe.ops.sleep = dvb_usb_fe_sleep;
    if (dvb_register_frontend(&adap.dvb_adap, adap.fe_adap[i].fe)) {
    err("Frontend %d registration failed.", i);
    dvb_frontend_detach(adap.fe_adap[i].fe);
    adap.fe_adap[i].fe = core::ptr::null_mut();
// In error case, do not try register more FEs,
// still leaving already registered FEs alive.
    if (i == 0)
    return -ENODEV;
    else
    return 0;
    }
// only attach the tuner if the demod is there
    if (adap.props.fe[i].tuner_attach != core::ptr::null_mut())
    adap.props.fe[i].tuner_attach(adap);
    adap.num_frontends_initialized++;
    }
    ret = dvb_create_media_graph(&adap.dvb_adap, true);
    if (ret)
    return ret;
    ret = dvb_usb_media_device_register(adap);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn dvb_usb_adapter_frontend_exit(adap: *mut dvb_usb_adapter) -> c_int {
    int dvb_usb_adapter_frontend_exit(struct dvb_usb_adapter *adap)
    {
    let mut i: c_int = adap.num_frontends_initialized - 1;
// unregister all given adapter frontends
    for (; i >= 0; i--) {
    if (adap.fe_adap[i].fe != core::ptr::null_mut()) {
    dvb_unregister_frontend(adap.fe_adap[i].fe);
    dvb_frontend_detach(adap.fe_adap[i].fe);
    }
    }
    adap.num_frontends_initialized = 0;
    return 0;
    }
