//! Automatically rewritten from C Header to Rust Module
//! Source: scripts/kconfig/qconf.h
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
// Copyright (C) 2002 Roman Zippel <zippel@linux-m68k.org>
//

extern "C" {
    pub fn writeSizes(key: QString&, value: QList<int>&) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum colIdx {
    promptColIdx, nameColIdx, dataColIdx
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum listMode {
    singleMode, menuMode, symbolMode, fullMode, listMode
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum optionMode {
    normalOpt = 0, allOpt, promptOpt
}

pub type Parent = class QTreeWidget;
extern "C" {
    pub fn reinit();
}
extern "C" {
    pub fn findConfigItem(: *mut menu) -> *mut ConfigItem;
}
extern "C" {
    pub fn keyPressEvent(e: *mut QKeyEvent);
}
extern "C" {
    pub fn mouseReleaseEvent(e: *mut QMouseEvent);
}
extern "C" {
    pub fn mouseDoubleClickEvent(e: *mut QMouseEvent);
}
extern "C" {
    pub fn focusInEvent(e: *mut QFocusEvent);
}
extern "C" {
    pub fn contextMenuEvent(e: *mut QContextMenuEvent);
}
extern "C" {
    pub fn setRootMenu(menu: *mut menu);
}
extern "C" {
    pub fn updateList();
}
extern "C" {
    pub fn setValue(item: *mut *mut ConfigItem, val: tristate);
}
extern "C" {
    pub fn changeValue(item: *mut *mut ConfigItem);
}
extern "C" {
    pub fn updateSelection();
}
extern "C" {
    pub fn saveSettings();
}
extern "C" {
    pub fn setOptionMode(action: *mut QAction);
}
extern "C" {
    pub fn setShowName(on: bool);
}
extern "C" {
    pub fn menuChanged(menu: *mut menu);
}
extern "C" {
    pub fn menuSelected(menu: *mut menu);
}
extern "C" {
    pub fn itemSelected(menu: *mut menu);
}
extern "C" {
    pub fn parentSelected();
}
extern "C" {
    pub fn gotFocus(: *mut menu);
}
extern "C" {
    pub fn showNameChanged(on: bool);
}
extern "C" {
    pub fn setAllOpen(open: bool);
}
extern "C" {
    pub fn setParentMenu();
}
extern "C" {
    pub fn menuSkip(: *mut menu) -> bool;
}
extern "C" {
    pub fn updateMenuList(parent: *mut ConfigItem, menu*: *mut struct);
}
extern "C" {
    pub fn updateMenuList(menu: *mut menu);
}
extern "C" {
    pub fn updateListForAll() -> static void;
}
extern "C" {
    pub fn updateListAllForAll() -> static void;
}
pub type Parent = class QTreeWidgetItem;
extern "C" {
    pub fn init();
}
extern "C" {
    pub fn updateMenu();
}
extern "C" {
    pub fn testUpdateMenu();
}
// TODO: Implement paintCell
pub type Parent = class QTextBrowser;
extern "C" {
    pub fn setInfo(menu: *mut menu);
}
extern "C" {
    pub fn saveSettings();
}
extern "C" {
    pub fn setShowDebug(_arg: bool);
}
extern "C" {
    pub fn clicked(&url: QUrl);
}
extern "C" {
    pub fn showDebugChanged(_arg: bool);
}
extern "C" {
    pub fn menuSelected(: *mut menu);
}
extern "C" {
    pub fn symbolInfo();
}
extern "C" {
    pub fn menuInfo();
}
extern "C" {
    pub fn debug_info(sym: *mut symbol) -> QString;
}
extern "C" {
    pub fn print_filter(&str: QString) -> static QString;
}
extern "C" {
    pub fn expr_print_help(data: *mut c_void, sym: *mut symbol, str: *const c_char) -> static void;
}
extern "C" {
    pub fn contextMenuEvent(event: *mut QContextMenuEvent);
}
pub type Parent = class QDialog;
extern "C" {
    pub fn saveSettings();
}
extern "C" {
    pub fn search();
}
extern "C" {
    pub fn conf_changed(_arg: bool) -> static void;
}
extern "C" {
    pub fn changeMenu(: *mut menu);
}
extern "C" {
    pub fn changeItens(: *mut menu);
}
extern "C" {
    pub fn setMenuLink(: *mut menu);
}
extern "C" {
    pub fn listFocusChanged();
}
extern "C" {
    pub fn goBack();
}
extern "C" {
    pub fn loadConfig();
}
extern "C" {
    pub fn saveConfig() -> bool;
}
extern "C" {
    pub fn saveConfigAs();
}
extern "C" {
    pub fn searchConfig();
}
extern "C" {
    pub fn showSingleView();
}
extern "C" {
    pub fn showSplitView();
}
extern "C" {
    pub fn showFullView();
}
extern "C" {
    pub fn showIntro();
}
extern "C" {
    pub fn showAbout();
}
extern "C" {
    pub fn saveSettings();
}
extern "C" {
    pub fn closeEvent(e: *mut QCloseEvent);
}
