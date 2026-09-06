//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/w1/w1_netlink.h
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
// Copyright (c) 2003 Evgeniy Polyakov <zbr@ioremap.net>
//

//
// enum w1_cn_msg_flags - bitfield flags for struct cn_msg.flags
//
// @W1_CN_BUNDLE: Request bundling replies into fewer messagse.  Be prepared
// to handle multiple struct cn_msg, struct w1_netlink_msg, and
// struct w1_netlink_cmd in one packet.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum w1_cn_msg_flags {
    W1_CN_BUNDLE = 1,
}

//
// enum w1_netlink_message_types - message type
//
// @W1_SLAVE_ADD: notification that a slave device was added
// @W1_SLAVE_REMOVE: notification that a slave device was removed
// @W1_MASTER_ADD: notification that a new bus master was added
// @W1_MASTER_REMOVE: notification that a bus masterwas removed
// @W1_MASTER_CMD: initiate operations on a specific master
// @W1_SLAVE_CMD: sends reset, selects the slave, then does a read/write/touch
// operation
// @W1_LIST_MASTERS: used to determine the bus master identifiers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum w1_netlink_message_types {
    W1_SLAVE_ADD = 0,
    W1_SLAVE_REMOVE,
    W1_MASTER_ADD,
    W1_MASTER_REMOVE,
    W1_MASTER_CMD,
    W1_SLAVE_CMD,
    W1_LIST_MASTERS,
}

//
// struct w1_netlink_msg - holds w1 message type, id, and result
//
// @type: one of enum w1_netlink_message_types
// @status: kernel feedback for success 0 or errno failure value
// @len: length of data following w1_netlink_msg
// @id: union holding bus master id (msg.id) and slave device id (id[8]).
// @id.id: Slave ID (8 bytes)
// @id.mst: bus master identification
// @id.mst.id: bus master ID
// @id.mst.res: bus master reserved
// @data: start address of any following data
//
// The base message structure for w1 messages over netlink.
// The netlink connector data sequence is, struct nlmsghdr, struct cn_msg,
// then one or more struct w1_netlink_msg (each with optional data).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct w1_mst {
    pub id: __u32,
    pub res: __u32,
    pub mst: },
    pub id: },
    pub data: [__u8; ],
}

//
// enum w1_commands - commands available for master or slave operations
//
// @W1_CMD_READ: read len bytes
// @W1_CMD_WRITE: write len bytes
// @W1_CMD_SEARCH: initiate a standard search, returns only the slave
// devices found during that search
// @W1_CMD_ALARM_SEARCH: search for devices that are currently alarming
// @W1_CMD_TOUCH: Touches a series of bytes.
// @W1_CMD_RESET: sends a bus reset on the given master
// @W1_CMD_SLAVE_ADD: adds a slave to the given master,
// 8 byte slave id at data[0]
// @W1_CMD_SLAVE_REMOVE: removes a slave to the given master,
// 8 byte slave id at data[0]
// @W1_CMD_LIST_SLAVES: list of slaves registered on this master
// @W1_CMD_MAX: number of available commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum w1_commands {
    W1_CMD_READ = 0,
    W1_CMD_WRITE,
    W1_CMD_SEARCH,
    W1_CMD_ALARM_SEARCH,
    W1_CMD_TOUCH,
    W1_CMD_RESET,
    W1_CMD_SLAVE_ADD,
    W1_CMD_SLAVE_REMOVE,
    W1_CMD_LIST_SLAVES,
    W1_CMD_MAX
}

//
// struct w1_netlink_cmd - holds the command and data
//
// @cmd: one of enum w1_commands
// @res: reserved
// @len: length of data following w1_netlink_cmd
// @data: start address of any following data
//
// One or more struct w1_netlink_cmd is placed starting at w1_netlink_msg.data
// each with optional data.
//

extern "C" {
    pub fn w1_netlink_send(: *mut w1_master, : *mut w1_netlink_msg);
}
extern "C" {
    pub fn w1_init_netlink() -> c_int;
}
extern "C" {
    pub fn w1_fini_netlink();
}

