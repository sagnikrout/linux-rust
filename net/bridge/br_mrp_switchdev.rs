//! Automatically rewritten from C to Rust
//! Source: net/bridge/br_mrp_switchdev.c
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

    static enum br_mrp_hw_support
    br_mrp_switchdev_port_obj(struct net_bridge *br,
    const struct switchdev_obj *obj, bool add)
    {
    int err;
    if (add)
    err = switchdev_port_obj_add(br.dev, obj, core::ptr::null_mut());
    else
    err = switchdev_port_obj_del(br.dev, obj);
// In case of success just return and notify the SW that doesn't need
// to do anything
//
    if (!err)
    return BR_MRP_HW;
    if (err != -EOPNOTSUPP)
    return BR_MRP_NONE;
// Continue with SW backup
    return BR_MRP_SW;
    }
#[no_mangle]
pub unsafe extern "C" fn br_mrp_switchdev_add(br: *mut net_bridge, mrp: *mut br_mrp) -> c_int {
    int br_mrp_switchdev_add(struct net_bridge *br, struct br_mrp *mrp)
    {
    struct switchdev_obj_mrp mrp_obj = {
    .obj.orig_dev = br.dev,
    .obj.id = SWITCHDEV_OBJ_ID_MRP,
    .p_port = rtnl_dereference(mrp.p_port).dev,
    .s_port = rtnl_dereference(mrp.s_port).dev,
    .ring_id = mrp.ring_id,
    .prio = mrp.prio,
    };
    if (!IS_ENABLED(CONFIG_NET_SWITCHDEV))
    return 0;
    return switchdev_port_obj_add(br.dev, &mrp_obj.obj, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn br_mrp_switchdev_del(br: *mut net_bridge, mrp: *mut br_mrp) -> c_int {
    int br_mrp_switchdev_del(struct net_bridge *br, struct br_mrp *mrp)
    {
    struct switchdev_obj_mrp mrp_obj = {
    .obj.orig_dev = br.dev,
    .obj.id = SWITCHDEV_OBJ_ID_MRP,
    .p_port = core::ptr::null_mut(),
    .s_port = core::ptr::null_mut(),
    .ring_id = mrp.ring_id,
    };
    if (!IS_ENABLED(CONFIG_NET_SWITCHDEV))
    return 0;
    return switchdev_port_obj_del(br.dev, &mrp_obj.obj);
    }
    enum br_mrp_hw_support
    br_mrp_switchdev_set_ring_role(struct net_bridge *br, struct br_mrp *mrp,
    enum br_mrp_ring_role_type role)
    {
    struct switchdev_obj_ring_role_mrp mrp_role = {
    .obj.orig_dev = br.dev,
    .obj.id = SWITCHDEV_OBJ_ID_RING_ROLE_MRP,
    .ring_role = role,
    .ring_id = mrp.ring_id,
    .sw_backup = false,
    };
    enum br_mrp_hw_support support;
    int err;
    if (!IS_ENABLED(CONFIG_NET_SWITCHDEV))
    return BR_MRP_SW;
    support = br_mrp_switchdev_port_obj(br, &mrp_role.obj,
    role != BR_MRP_RING_ROLE_DISABLED);
    if (support != BR_MRP_SW)
    return support;
// If the driver can't configure to run completely the protocol in HW,
// then try again to configure the HW so the SW can run the protocol.
//
    mrp_role.sw_backup = true;
    if (role != BR_MRP_RING_ROLE_DISABLED)
    err = switchdev_port_obj_add(br.dev, &mrp_role.obj, core::ptr::null_mut());
    else
    err = switchdev_port_obj_del(br.dev, &mrp_role.obj);
    if (!err)
    return BR_MRP_SW;
    return BR_MRP_NONE;
    }
    enum br_mrp_hw_support
    br_mrp_switchdev_send_ring_test(struct net_bridge *br, struct br_mrp *mrp,
    u32 interval, u8 max_miss, u32 period,
    bool monitor)
    {
    struct switchdev_obj_ring_test_mrp test = {
    .obj.orig_dev = br.dev,
    .obj.id = SWITCHDEV_OBJ_ID_RING_TEST_MRP,
    .interval = interval,
    .max_miss = max_miss,
    .ring_id = mrp.ring_id,
    .period = period,
    .monitor = monitor,
    };
    if (!IS_ENABLED(CONFIG_NET_SWITCHDEV))
    return BR_MRP_SW;
    return br_mrp_switchdev_port_obj(br, &test.obj, interval != 0);
    }
    int br_mrp_switchdev_set_ring_state(struct net_bridge *br,
    struct br_mrp *mrp,
    enum br_mrp_ring_state_type state)
    {
    struct switchdev_obj_ring_state_mrp mrp_state = {
    .obj.orig_dev = br.dev,
    .obj.id = SWITCHDEV_OBJ_ID_RING_STATE_MRP,
    .ring_state = state,
    .ring_id = mrp.ring_id,
    };
    if (!IS_ENABLED(CONFIG_NET_SWITCHDEV))
    return 0;
    return switchdev_port_obj_add(br.dev, &mrp_state.obj, core::ptr::null_mut());
    }
    enum br_mrp_hw_support
    br_mrp_switchdev_set_in_role(struct net_bridge *br, struct br_mrp *mrp,
    u16 in_id, u32 ring_id,
    enum br_mrp_in_role_type role)
    {
    struct switchdev_obj_in_role_mrp mrp_role = {
    .obj.orig_dev = br.dev,
    .obj.id = SWITCHDEV_OBJ_ID_IN_ROLE_MRP,
    .in_role = role,
    .in_id = mrp.in_id,
    .ring_id = mrp.ring_id,
    .i_port = rtnl_dereference(mrp.i_port).dev,
    .sw_backup = false,
    };
    enum br_mrp_hw_support support;
    int err;
    if (!IS_ENABLED(CONFIG_NET_SWITCHDEV))
    return BR_MRP_SW;
    support = br_mrp_switchdev_port_obj(br, &mrp_role.obj,
    role != BR_MRP_IN_ROLE_DISABLED);
    if (support != BR_MRP_NONE)
    return support;
// If the driver can't configure to run completely the protocol in HW,
// then try again to configure the HW so the SW can run the protocol.
//
    mrp_role.sw_backup = true;
    if (role != BR_MRP_IN_ROLE_DISABLED)
    err = switchdev_port_obj_add(br.dev, &mrp_role.obj, core::ptr::null_mut());
    else
    err = switchdev_port_obj_del(br.dev, &mrp_role.obj);
    if (!err)
    return BR_MRP_SW;
    return BR_MRP_NONE;
    }
    int br_mrp_switchdev_set_in_state(struct net_bridge *br, struct br_mrp *mrp,
    enum br_mrp_in_state_type state)
    {
    struct switchdev_obj_in_state_mrp mrp_state = {
    .obj.orig_dev = br.dev,
    .obj.id = SWITCHDEV_OBJ_ID_IN_STATE_MRP,
    .in_state = state,
    .in_id = mrp.in_id,
    };
    if (!IS_ENABLED(CONFIG_NET_SWITCHDEV))
    return 0;
    return switchdev_port_obj_add(br.dev, &mrp_state.obj, core::ptr::null_mut());
    }
    enum br_mrp_hw_support
    br_mrp_switchdev_send_in_test(struct net_bridge *br, struct br_mrp *mrp,
    u32 interval, u8 max_miss, u32 period)
    {
    struct switchdev_obj_in_test_mrp test = {
    .obj.orig_dev = br.dev,
    .obj.id = SWITCHDEV_OBJ_ID_IN_TEST_MRP,
    .interval = interval,
    .max_miss = max_miss,
    .in_id = mrp.in_id,
    .period = period,
    };
    if (!IS_ENABLED(CONFIG_NET_SWITCHDEV))
    return BR_MRP_SW;
    return br_mrp_switchdev_port_obj(br, &test.obj, interval != 0);
    }
#[no_mangle]
pub unsafe extern "C" fn br_mrp_port_switchdev_set_state(p: *mut net_bridge_port, state: u32) -> c_int {
    int br_mrp_port_switchdev_set_state(struct net_bridge_port *p, u32 state)
    {
    struct switchdev_attr attr = {
    .orig_dev = p.dev,
    .id = SWITCHDEV_ATTR_ID_PORT_STP_STATE,
    .u.stp_state = state,
    };
    if (!IS_ENABLED(CONFIG_NET_SWITCHDEV))
    return 0;
    return switchdev_port_attr_set(p.dev, &attr, core::ptr::null_mut());
    }
    int br_mrp_port_switchdev_set_role(struct net_bridge_port *p,
    enum br_mrp_port_role_type role)
    {
    struct switchdev_attr attr = {
    .orig_dev = p.dev,
    .id = SWITCHDEV_ATTR_ID_MRP_PORT_ROLE,
    .u.mrp_port_role = role,
    };
    if (!IS_ENABLED(CONFIG_NET_SWITCHDEV))
    return 0;
    return switchdev_port_attr_set(p.dev, &attr, core::ptr::null_mut());
    }
