//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/input/linux-event-codes.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Input event codes
//
// *** IMPORTANT
// This file is not only included from C-code but also from devicetree source
// files. As such this file MUST only contain comments and defines.
//
// Copyright (c) 1999-2002 Vojtech Pavlik
// Copyright (c) 2015 Hans de Goede <hdegoede@redhat.com>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published by
// the Free Software Foundation.
//
// Device properties and quirks
//
pub const INPUT_PROP_POINTER: c_uint = 0x00	/* needs a pointer */;
pub const INPUT_PROP_DIRECT: c_uint = 0x01	/* direct input devices */;
pub const INPUT_PROP_BUTTONPAD: c_uint = 0x02	/* has button(s) under pad */;
pub const INPUT_PROP_SEMI_MT: c_uint = 0x03	/* touch rectangle only */;
pub const INPUT_PROP_TOPBUTTONPAD: c_uint = 0x04	/* softbuttons at top of pad */;
pub const INPUT_PROP_POINTING_STICK: c_uint = 0x05	/* is a pointing stick */;
pub const INPUT_PROP_ACCELEROMETER: c_uint = 0x06	/* has accelerometer */;
pub const INPUT_PROP_PRESSUREPAD: c_uint = 0x07	/* pressure triggers clicks */;
pub const INPUT_PROP_MAX: c_uint = 0x1f;

//
// Event types
//
pub const EV_SYN: c_uint = 0x00;
pub const EV_KEY: c_uint = 0x01;
pub const EV_REL: c_uint = 0x02;
pub const EV_ABS: c_uint = 0x03;
pub const EV_MSC: c_uint = 0x04;
pub const EV_SW: c_uint = 0x05;
pub const EV_LED: c_uint = 0x11;
pub const EV_SND: c_uint = 0x12;
pub const EV_REP: c_uint = 0x14;
pub const EV_FF: c_uint = 0x15;
pub const EV_PWR: c_uint = 0x16;
pub const EV_FF_STATUS: c_uint = 0x17;
pub const EV_MAX: c_uint = 0x1f;

//
// Synchronization events.
//
pub const SYN_REPORT: c_int = 0;
pub const SYN_CONFIG: c_int = 1;
pub const SYN_MT_REPORT: c_int = 2;
pub const SYN_DROPPED: c_int = 3;
pub const SYN_MAX: c_uint = 0xf;

//
// Keys and buttons
//
// Most of the keys/buttons are modeled after USB HUT 1.12
// (see http://www.usb.org/developers/hidpage).
// Abbreviations in the comments:
// AC - Application Control
// AL - Application Launch Button
// SC - System Control
//
pub const KEY_RESERVED: c_int = 0;
pub const KEY_ESC: c_int = 1;
pub const KEY_1: c_int = 2;
pub const KEY_2: c_int = 3;
pub const KEY_3: c_int = 4;
pub const KEY_4: c_int = 5;
pub const KEY_5: c_int = 6;
pub const KEY_6: c_int = 7;
pub const KEY_7: c_int = 8;
pub const KEY_8: c_int = 9;
pub const KEY_9: c_int = 10;
pub const KEY_0: c_int = 11;
pub const KEY_MINUS: c_int = 12;
pub const KEY_EQUAL: c_int = 13;
pub const KEY_BACKSPACE: c_int = 14;
pub const KEY_TAB: c_int = 15;
pub const KEY_Q: c_int = 16;
pub const KEY_W: c_int = 17;
pub const KEY_E: c_int = 18;
pub const KEY_R: c_int = 19;
pub const KEY_T: c_int = 20;
pub const KEY_Y: c_int = 21;
pub const KEY_U: c_int = 22;
pub const KEY_I: c_int = 23;
pub const KEY_O: c_int = 24;
pub const KEY_P: c_int = 25;
pub const KEY_LEFTBRACE: c_int = 26;
pub const KEY_RIGHTBRACE: c_int = 27;
pub const KEY_ENTER: c_int = 28;
pub const KEY_LEFTCTRL: c_int = 29;
pub const KEY_A: c_int = 30;
pub const KEY_S: c_int = 31;
pub const KEY_D: c_int = 32;
pub const KEY_F: c_int = 33;
pub const KEY_G: c_int = 34;
pub const KEY_H: c_int = 35;
pub const KEY_J: c_int = 36;
pub const KEY_K: c_int = 37;
pub const KEY_L: c_int = 38;
pub const KEY_SEMICOLON: c_int = 39;
pub const KEY_APOSTROPHE: c_int = 40;
pub const KEY_GRAVE: c_int = 41;
pub const KEY_LEFTSHIFT: c_int = 42;
pub const KEY_BACKSLASH: c_int = 43;
pub const KEY_Z: c_int = 44;
pub const KEY_X: c_int = 45;
pub const KEY_C: c_int = 46;
pub const KEY_V: c_int = 47;
pub const KEY_B: c_int = 48;
pub const KEY_N: c_int = 49;
pub const KEY_M: c_int = 50;
pub const KEY_COMMA: c_int = 51;
pub const KEY_DOT: c_int = 52;
pub const KEY_SLASH: c_int = 53;
pub const KEY_RIGHTSHIFT: c_int = 54;
pub const KEY_KPASTERISK: c_int = 55;
pub const KEY_LEFTALT: c_int = 56;
pub const KEY_SPACE: c_int = 57;
pub const KEY_CAPSLOCK: c_int = 58;
pub const KEY_F1: c_int = 59;
pub const KEY_F2: c_int = 60;
pub const KEY_F3: c_int = 61;
pub const KEY_F4: c_int = 62;
pub const KEY_F5: c_int = 63;
pub const KEY_F6: c_int = 64;
pub const KEY_F7: c_int = 65;
pub const KEY_F8: c_int = 66;
pub const KEY_F9: c_int = 67;
pub const KEY_F10: c_int = 68;
pub const KEY_NUMLOCK: c_int = 69;
pub const KEY_SCROLLLOCK: c_int = 70;
pub const KEY_KP7: c_int = 71;
pub const KEY_KP8: c_int = 72;
pub const KEY_KP9: c_int = 73;
pub const KEY_KPMINUS: c_int = 74;
pub const KEY_KP4: c_int = 75;
pub const KEY_KP5: c_int = 76;
pub const KEY_KP6: c_int = 77;
pub const KEY_KPPLUS: c_int = 78;
pub const KEY_KP1: c_int = 79;
pub const KEY_KP2: c_int = 80;
pub const KEY_KP3: c_int = 81;
pub const KEY_KP0: c_int = 82;
pub const KEY_KPDOT: c_int = 83;
pub const KEY_ZENKAKUHANKAKU: c_int = 85;
pub const KEY_102ND: c_int = 86;
pub const KEY_F11: c_int = 87;
pub const KEY_F12: c_int = 88;
pub const KEY_RO: c_int = 89;
pub const KEY_KATAKANA: c_int = 90;
pub const KEY_HIRAGANA: c_int = 91;
pub const KEY_HENKAN: c_int = 92;
pub const KEY_KATAKANAHIRAGANA: c_int = 93;
pub const KEY_MUHENKAN: c_int = 94;
pub const KEY_KPJPCOMMA: c_int = 95;
pub const KEY_KPENTER: c_int = 96;
pub const KEY_RIGHTCTRL: c_int = 97;
pub const KEY_KPSLASH: c_int = 98;
pub const KEY_SYSRQ: c_int = 99;
pub const KEY_RIGHTALT: c_int = 100;
pub const KEY_LINEFEED: c_int = 101;
pub const KEY_HOME: c_int = 102;
pub const KEY_UP: c_int = 103;
pub const KEY_PAGEUP: c_int = 104;
pub const KEY_LEFT: c_int = 105;
pub const KEY_RIGHT: c_int = 106;
pub const KEY_END: c_int = 107;
pub const KEY_DOWN: c_int = 108;
pub const KEY_PAGEDOWN: c_int = 109;
pub const KEY_INSERT: c_int = 110;
pub const KEY_DELETE: c_int = 111;
pub const KEY_MACRO: c_int = 112;
pub const KEY_MUTE: c_int = 113;
pub const KEY_VOLUMEDOWN: c_int = 114;
pub const KEY_VOLUMEUP: c_int = 115;

pub const KEY_KPEQUAL: c_int = 117;
pub const KEY_KPPLUSMINUS: c_int = 118;
pub const KEY_PAUSE: c_int = 119;

pub const KEY_KPCOMMA: c_int = 121;
pub const KEY_HANGEUL: c_int = 122;

pub const KEY_HANJA: c_int = 123;
pub const KEY_YEN: c_int = 124;
pub const KEY_LEFTMETA: c_int = 125;
pub const KEY_RIGHTMETA: c_int = 126;
pub const KEY_COMPOSE: c_int = 127;

pub const KEY_AGAIN: c_int = 129;

pub const KEY_FRONT: c_int = 132;

pub const KEY_SETUP: c_int = 141;

pub const KEY_SENDFILE: c_int = 145;
pub const KEY_DELETEFILE: c_int = 146;
pub const KEY_XFER: c_int = 147;
pub const KEY_PROG1: c_int = 148;
pub const KEY_PROG2: c_int = 149;

pub const KEY_MSDOS: c_int = 151;

pub const KEY_CYCLEWINDOWS: c_int = 154;
pub const KEY_MAIL: c_int = 155;

pub const KEY_COMPUTER: c_int = 157;

pub const KEY_CLOSECD: c_int = 160;
pub const KEY_EJECTCD: c_int = 161;
pub const KEY_EJECTCLOSECD: c_int = 162;
pub const KEY_NEXTSONG: c_int = 163;
pub const KEY_PLAYPAUSE: c_int = 164;
pub const KEY_PREVIOUSSONG: c_int = 165;
pub const KEY_STOPCD: c_int = 166;
pub const KEY_RECORD: c_int = 167;
pub const KEY_REWIND: c_int = 168;

pub const KEY_ISO: c_int = 170;

pub const KEY_MOVE: c_int = 175;
pub const KEY_EDIT: c_int = 176;
pub const KEY_SCROLLUP: c_int = 177;
pub const KEY_SCROLLDOWN: c_int = 178;
pub const KEY_KPLEFTPAREN: c_int = 179;
pub const KEY_KPRIGHTPAREN: c_int = 180;

pub const KEY_F13: c_int = 183;
pub const KEY_F14: c_int = 184;
pub const KEY_F15: c_int = 185;
pub const KEY_F16: c_int = 186;
pub const KEY_F17: c_int = 187;
pub const KEY_F18: c_int = 188;
pub const KEY_F19: c_int = 189;
pub const KEY_F20: c_int = 190;
pub const KEY_F21: c_int = 191;
pub const KEY_F22: c_int = 192;
pub const KEY_F23: c_int = 193;
pub const KEY_F24: c_int = 194;
pub const KEY_PLAYCD: c_int = 200;
pub const KEY_PAUSECD: c_int = 201;
pub const KEY_PROG3: c_int = 202;
pub const KEY_PROG4: c_int = 203;

pub const KEY_SUSPEND: c_int = 205;

pub const KEY_PLAY: c_int = 207;
pub const KEY_FASTFORWARD: c_int = 208;
pub const KEY_BASSBOOST: c_int = 209;

pub const KEY_HP: c_int = 211;
pub const KEY_CAMERA: c_int = 212;
pub const KEY_SOUND: c_int = 213;
pub const KEY_QUESTION: c_int = 214;
pub const KEY_EMAIL: c_int = 215;
pub const KEY_CHAT: c_int = 216;
pub const KEY_SEARCH: c_int = 217;
pub const KEY_CONNECT: c_int = 218;

pub const KEY_SPORT: c_int = 220;
pub const KEY_SHOP: c_int = 221;
pub const KEY_ALTERASE: c_int = 222;

pub const KEY_BRIGHTNESSDOWN: c_int = 224;
pub const KEY_BRIGHTNESSUP: c_int = 225;
pub const KEY_MEDIA: c_int = 226;

pub const KEY_KBDILLUMTOGGLE: c_int = 228;
pub const KEY_KBDILLUMDOWN: c_int = 229;
pub const KEY_KBDILLUMUP: c_int = 230;

pub const KEY_DOCUMENTS: c_int = 235;
pub const KEY_BATTERY: c_int = 236;
pub const KEY_BLUETOOTH: c_int = 237;
pub const KEY_WLAN: c_int = 238;
pub const KEY_UWB: c_int = 239;
pub const KEY_UNKNOWN: c_int = 240;

// Code 255 is reserved for special needs of AT keyboard driver
pub const BTN_MISC: c_uint = 0x100;
pub const BTN_0: c_uint = 0x100;
pub const BTN_1: c_uint = 0x101;
pub const BTN_2: c_uint = 0x102;
pub const BTN_3: c_uint = 0x103;
pub const BTN_4: c_uint = 0x104;
pub const BTN_5: c_uint = 0x105;
pub const BTN_6: c_uint = 0x106;
pub const BTN_7: c_uint = 0x107;
pub const BTN_8: c_uint = 0x108;
pub const BTN_9: c_uint = 0x109;
pub const BTN_MOUSE: c_uint = 0x110;
pub const BTN_LEFT: c_uint = 0x110;
pub const BTN_RIGHT: c_uint = 0x111;
pub const BTN_MIDDLE: c_uint = 0x112;
pub const BTN_SIDE: c_uint = 0x113;
pub const BTN_EXTRA: c_uint = 0x114;
pub const BTN_FORWARD: c_uint = 0x115;
pub const BTN_BACK: c_uint = 0x116;
pub const BTN_TASK: c_uint = 0x117;
pub const BTN_JOYSTICK: c_uint = 0x120;
pub const BTN_TRIGGER: c_uint = 0x120;
pub const BTN_THUMB: c_uint = 0x121;
pub const BTN_THUMB2: c_uint = 0x122;
pub const BTN_TOP: c_uint = 0x123;
pub const BTN_TOP2: c_uint = 0x124;
pub const BTN_PINKIE: c_uint = 0x125;
pub const BTN_BASE: c_uint = 0x126;
pub const BTN_BASE2: c_uint = 0x127;
pub const BTN_BASE3: c_uint = 0x128;
pub const BTN_BASE4: c_uint = 0x129;
pub const BTN_BASE5: c_uint = 0x12a;
pub const BTN_BASE6: c_uint = 0x12b;
pub const BTN_DEAD: c_uint = 0x12f;
pub const BTN_GAMEPAD: c_uint = 0x130;
pub const BTN_SOUTH: c_uint = 0x130;

pub const BTN_EAST: c_uint = 0x131;

pub const BTN_C: c_uint = 0x132;
pub const BTN_NORTH: c_uint = 0x133;

pub const BTN_WEST: c_uint = 0x134;

pub const BTN_Z: c_uint = 0x135;
pub const BTN_TL: c_uint = 0x136;
pub const BTN_TR: c_uint = 0x137;
pub const BTN_TL2: c_uint = 0x138;
pub const BTN_TR2: c_uint = 0x139;
pub const BTN_SELECT: c_uint = 0x13a;
pub const BTN_START: c_uint = 0x13b;
pub const BTN_MODE: c_uint = 0x13c;
pub const BTN_THUMBL: c_uint = 0x13d;
pub const BTN_THUMBR: c_uint = 0x13e;
pub const BTN_DIGI: c_uint = 0x140;
pub const BTN_TOOL_PEN: c_uint = 0x140;
pub const BTN_TOOL_RUBBER: c_uint = 0x141;
pub const BTN_TOOL_BRUSH: c_uint = 0x142;
pub const BTN_TOOL_PENCIL: c_uint = 0x143;
pub const BTN_TOOL_AIRBRUSH: c_uint = 0x144;
pub const BTN_TOOL_FINGER: c_uint = 0x145;
pub const BTN_TOOL_MOUSE: c_uint = 0x146;
pub const BTN_TOOL_LENS: c_uint = 0x147;
pub const BTN_TOOL_QUINTTAP: c_uint = 0x148	/* Five fingers on trackpad */;
pub const BTN_STYLUS3: c_uint = 0x149;
pub const BTN_TOUCH: c_uint = 0x14a;
pub const BTN_STYLUS: c_uint = 0x14b;
pub const BTN_STYLUS2: c_uint = 0x14c;
pub const BTN_TOOL_DOUBLETAP: c_uint = 0x14d;
pub const BTN_TOOL_TRIPLETAP: c_uint = 0x14e;
pub const BTN_TOOL_QUADTAP: c_uint = 0x14f	/* Four fingers on trackpad */;
pub const BTN_WHEEL: c_uint = 0x150;
pub const BTN_GEAR_DOWN: c_uint = 0x150;
pub const BTN_GEAR_UP: c_uint = 0x151;
pub const KEY_OK: c_uint = 0x160;
pub const KEY_SELECT: c_uint = 0x161;
pub const KEY_GOTO: c_uint = 0x162;
pub const KEY_CLEAR: c_uint = 0x163;
pub const KEY_POWER2: c_uint = 0x164;
pub const KEY_OPTION: c_uint = 0x165;
pub const KEY_INFO: c_uint = 0x166	/* AL OEM Features/Tips/Tutorial */;
pub const KEY_TIME: c_uint = 0x167;
pub const KEY_VENDOR: c_uint = 0x168;
pub const KEY_ARCHIVE: c_uint = 0x169;
pub const KEY_PROGRAM: c_uint = 0x16a	/* Media Select Program Guide */;
pub const KEY_CHANNEL: c_uint = 0x16b;
pub const KEY_FAVORITES: c_uint = 0x16c;
pub const KEY_EPG: c_uint = 0x16d;
pub const KEY_PVR: c_uint = 0x16e	/* Media Select Home */;
pub const KEY_MHP: c_uint = 0x16f;
pub const KEY_LANGUAGE: c_uint = 0x170;
pub const KEY_TITLE: c_uint = 0x171;
pub const KEY_SUBTITLE: c_uint = 0x172;
pub const KEY_ANGLE: c_uint = 0x173;
pub const KEY_FULL_SCREEN: c_uint = 0x174	/* AC View Toggle */;

pub const KEY_MODE: c_uint = 0x175;
pub const KEY_KEYBOARD: c_uint = 0x176;
pub const KEY_ASPECT_RATIO: c_uint = 0x177	/* HUTRR37: Aspect */;

pub const KEY_PC: c_uint = 0x178	/* Media Select Computer */;
pub const KEY_TV: c_uint = 0x179	/* Media Select TV */;
pub const KEY_TV2: c_uint = 0x17a	/* Media Select Cable */;
pub const KEY_VCR: c_uint = 0x17b	/* Media Select VCR */;
pub const KEY_VCR2: c_uint = 0x17c	/* VCR Plus */;
pub const KEY_SAT: c_uint = 0x17d	/* Media Select Satellite */;
pub const KEY_SAT2: c_uint = 0x17e;
pub const KEY_CD: c_uint = 0x17f	/* Media Select CD */;
pub const KEY_TAPE: c_uint = 0x180	/* Media Select Tape */;
pub const KEY_RADIO: c_uint = 0x181;
pub const KEY_TUNER: c_uint = 0x182	/* Media Select Tuner */;
pub const KEY_PLAYER: c_uint = 0x183;
pub const KEY_TEXT: c_uint = 0x184;
pub const KEY_DVD: c_uint = 0x185	/* Media Select DVD */;
pub const KEY_AUX: c_uint = 0x186;
pub const KEY_MP3: c_uint = 0x187;
pub const KEY_AUDIO: c_uint = 0x188	/* AL Audio Browser */;
pub const KEY_VIDEO: c_uint = 0x189	/* AL Movie Browser */;
pub const KEY_DIRECTORY: c_uint = 0x18a;
pub const KEY_LIST: c_uint = 0x18b;
pub const KEY_MEMO: c_uint = 0x18c	/* Media Select Messages */;
pub const KEY_CALENDAR: c_uint = 0x18d;
pub const KEY_RED: c_uint = 0x18e;
pub const KEY_GREEN: c_uint = 0x18f;
pub const KEY_YELLOW: c_uint = 0x190;
pub const KEY_BLUE: c_uint = 0x191;
pub const KEY_CHANNELUP: c_uint = 0x192	/* Channel Increment */;
pub const KEY_CHANNELDOWN: c_uint = 0x193	/* Channel Decrement */;
pub const KEY_FIRST: c_uint = 0x194;
pub const KEY_LAST: c_uint = 0x195	/* Recall Last */;
pub const KEY_AB: c_uint = 0x196;
pub const KEY_NEXT: c_uint = 0x197;
pub const KEY_RESTART: c_uint = 0x198;
pub const KEY_SLOW: c_uint = 0x199;
pub const KEY_SHUFFLE: c_uint = 0x19a;
pub const KEY_BREAK: c_uint = 0x19b;
pub const KEY_PREVIOUS: c_uint = 0x19c;
pub const KEY_DIGITS: c_uint = 0x19d;
pub const KEY_TEEN: c_uint = 0x19e;
pub const KEY_TWEN: c_uint = 0x19f;
pub const KEY_VIDEOPHONE: c_uint = 0x1a0	/* Media Select Video Phone */;
pub const KEY_GAMES: c_uint = 0x1a1	/* Media Select Games */;
pub const KEY_ZOOMIN: c_uint = 0x1a2	/* AC Zoom In */;
pub const KEY_ZOOMOUT: c_uint = 0x1a3	/* AC Zoom Out */;
pub const KEY_ZOOMRESET: c_uint = 0x1a4	/* AC Zoom */;
pub const KEY_WORDPROCESSOR: c_uint = 0x1a5	/* AL Word Processor */;
pub const KEY_EDITOR: c_uint = 0x1a6	/* AL Text Editor */;
pub const KEY_SPREADSHEET: c_uint = 0x1a7	/* AL Spreadsheet */;
pub const KEY_GRAPHICSEDITOR: c_uint = 0x1a8	/* AL Graphics Editor */;
pub const KEY_PRESENTATION: c_uint = 0x1a9	/* AL Presentation App */;
pub const KEY_DATABASE: c_uint = 0x1aa	/* AL Database App */;
pub const KEY_NEWS: c_uint = 0x1ab	/* AL Newsreader */;
pub const KEY_VOICEMAIL: c_uint = 0x1ac	/* AL Voicemail */;
pub const KEY_ADDRESSBOOK: c_uint = 0x1ad	/* AL Contacts/Address Book */;
pub const KEY_MESSENGER: c_uint = 0x1ae	/* AL Instant Messaging */;
pub const KEY_DISPLAYTOGGLE: c_uint = 0x1af	/* Turn display (LCD) on and off */;

pub const KEY_SPELLCHECK: c_uint = 0x1b0   /* AL Spell Check */;
pub const KEY_LOGOFF: c_uint = 0x1b1   /* AL Logoff */;
pub const KEY_DOLLAR: c_uint = 0x1b2;
pub const KEY_EURO: c_uint = 0x1b3;
pub const KEY_FRAMEBACK: c_uint = 0x1b4	/* Consumer - transport controls */;
pub const KEY_FRAMEFORWARD: c_uint = 0x1b5;
pub const KEY_CONTEXT_MENU: c_uint = 0x1b6	/* GenDesc - system context menu */;
pub const KEY_MEDIA_REPEAT: c_uint = 0x1b7	/* Consumer - transport control */;
pub const KEY_10CHANNELSUP: c_uint = 0x1b8	/* 10 channels up (10+) */;
pub const KEY_10CHANNELSDOWN: c_uint = 0x1b9	/* 10 channels down (10-) */;
pub const KEY_IMAGES: c_uint = 0x1ba	/* AL Image Browser */;
pub const KEY_NOTIFICATION_CENTER: c_uint = 0x1bc	/* Show/hide the notification center */;
pub const KEY_PICKUP_PHONE: c_uint = 0x1bd	/* Answer incoming call */;
pub const KEY_HANGUP_PHONE: c_uint = 0x1be	/* Decline incoming call */;
pub const KEY_LINK_PHONE: c_uint = 0x1bf   /* AL Phone Syncing */;
pub const KEY_DEL_EOL: c_uint = 0x1c0;
pub const KEY_DEL_EOS: c_uint = 0x1c1;
pub const KEY_INS_LINE: c_uint = 0x1c2;
pub const KEY_DEL_LINE: c_uint = 0x1c3;
pub const KEY_FN: c_uint = 0x1d0;
pub const KEY_FN_ESC: c_uint = 0x1d1;
pub const KEY_FN_F1: c_uint = 0x1d2;
pub const KEY_FN_F2: c_uint = 0x1d3;
pub const KEY_FN_F3: c_uint = 0x1d4;
pub const KEY_FN_F4: c_uint = 0x1d5;
pub const KEY_FN_F5: c_uint = 0x1d6;
pub const KEY_FN_F6: c_uint = 0x1d7;
pub const KEY_FN_F7: c_uint = 0x1d8;
pub const KEY_FN_F8: c_uint = 0x1d9;
pub const KEY_FN_F9: c_uint = 0x1da;
pub const KEY_FN_F10: c_uint = 0x1db;
pub const KEY_FN_F11: c_uint = 0x1dc;
pub const KEY_FN_F12: c_uint = 0x1dd;
pub const KEY_FN_1: c_uint = 0x1de;
pub const KEY_FN_2: c_uint = 0x1df;
pub const KEY_FN_D: c_uint = 0x1e0;
pub const KEY_FN_E: c_uint = 0x1e1;
pub const KEY_FN_F: c_uint = 0x1e2;
pub const KEY_FN_S: c_uint = 0x1e3;
pub const KEY_FN_B: c_uint = 0x1e4;
pub const KEY_FN_RIGHT_SHIFT: c_uint = 0x1e5;
pub const KEY_BRL_DOT1: c_uint = 0x1f1;
pub const KEY_BRL_DOT2: c_uint = 0x1f2;
pub const KEY_BRL_DOT3: c_uint = 0x1f3;
pub const KEY_BRL_DOT4: c_uint = 0x1f4;
pub const KEY_BRL_DOT5: c_uint = 0x1f5;
pub const KEY_BRL_DOT6: c_uint = 0x1f6;
pub const KEY_BRL_DOT7: c_uint = 0x1f7;
pub const KEY_BRL_DOT8: c_uint = 0x1f8;
pub const KEY_BRL_DOT9: c_uint = 0x1f9;
pub const KEY_BRL_DOT10: c_uint = 0x1fa;
pub const KEY_NUMERIC_0: c_uint = 0x200	/* used by phones, remote controls, */;
pub const KEY_NUMERIC_1: c_uint = 0x201	/* and other keypads */;
pub const KEY_NUMERIC_2: c_uint = 0x202;
pub const KEY_NUMERIC_3: c_uint = 0x203;
pub const KEY_NUMERIC_4: c_uint = 0x204;
pub const KEY_NUMERIC_5: c_uint = 0x205;
pub const KEY_NUMERIC_6: c_uint = 0x206;
pub const KEY_NUMERIC_7: c_uint = 0x207;
pub const KEY_NUMERIC_8: c_uint = 0x208;
pub const KEY_NUMERIC_9: c_uint = 0x209;
pub const KEY_NUMERIC_STAR: c_uint = 0x20a;
pub const KEY_NUMERIC_POUND: c_uint = 0x20b;
pub const KEY_NUMERIC_A: c_uint = 0x20c	/* Phone key A - HUT Telephony 0xb9 */;
pub const KEY_NUMERIC_B: c_uint = 0x20d;
pub const KEY_NUMERIC_C: c_uint = 0x20e;
pub const KEY_NUMERIC_D: c_uint = 0x20f;
pub const KEY_CAMERA_FOCUS: c_uint = 0x210;
pub const KEY_WPS_BUTTON: c_uint = 0x211	/* WiFi Protected Setup key */;
pub const KEY_TOUCHPAD_TOGGLE: c_uint = 0x212	/* Request switch touchpad on or off */;
pub const KEY_TOUCHPAD_ON: c_uint = 0x213;
pub const KEY_TOUCHPAD_OFF: c_uint = 0x214;
pub const KEY_CAMERA_ZOOMIN: c_uint = 0x215;
pub const KEY_CAMERA_ZOOMOUT: c_uint = 0x216;
pub const KEY_CAMERA_UP: c_uint = 0x217;
pub const KEY_CAMERA_DOWN: c_uint = 0x218;
pub const KEY_CAMERA_LEFT: c_uint = 0x219;
pub const KEY_CAMERA_RIGHT: c_uint = 0x21a;
pub const KEY_ATTENDANT_ON: c_uint = 0x21b;
pub const KEY_ATTENDANT_OFF: c_uint = 0x21c;
pub const KEY_ATTENDANT_TOGGLE: c_uint = 0x21d	/* Attendant call on or off */;
pub const KEY_LIGHTS_TOGGLE: c_uint = 0x21e	/* Reading light on or off */;
pub const BTN_DPAD_UP: c_uint = 0x220;
pub const BTN_DPAD_DOWN: c_uint = 0x221;
pub const BTN_DPAD_LEFT: c_uint = 0x222;
pub const BTN_DPAD_RIGHT: c_uint = 0x223;
pub const BTN_GRIPL: c_uint = 0x224;
pub const BTN_GRIPR: c_uint = 0x225;
pub const BTN_GRIPL2: c_uint = 0x226;
pub const BTN_GRIPR2: c_uint = 0x227;
pub const KEY_ALS_TOGGLE: c_uint = 0x230	/* Ambient light sensor */;
pub const KEY_ROTATE_LOCK_TOGGLE: c_uint = 0x231	/* Display rotation lock */;
pub const KEY_REFRESH_RATE_TOGGLE: c_uint = 0x232	/* Display refresh rate toggle */;
pub const KEY_BUTTONCONFIG: c_uint = 0x240	/* AL Button Configuration */;
pub const KEY_TASKMANAGER: c_uint = 0x241	/* AL Task/Project Manager */;
pub const KEY_JOURNAL: c_uint = 0x242	/* AL Log/Journal/Timecard */;
pub const KEY_CONTROLPANEL: c_uint = 0x243	/* AL Control Panel */;
pub const KEY_APPSELECT: c_uint = 0x244	/* AL Select Task/Application */;
pub const KEY_SCREENSAVER: c_uint = 0x245	/* AL Screen Saver */;
pub const KEY_VOICECOMMAND: c_uint = 0x246	/* Listening Voice Command */;
pub const KEY_ASSISTANT: c_uint = 0x247	/* AL Context-aware desktop assistant */;
pub const KEY_KBD_LAYOUT_NEXT: c_uint = 0x248	/* AC Next Keyboard Layout Select */;
pub const KEY_EMOJI_PICKER: c_uint = 0x249	/* Show/hide emoji picker (HUTRR101) */;
pub const KEY_DICTATE: c_uint = 0x24a	/* Start or Stop Voice Dictation Session (HUTRR99) */;
pub const KEY_CAMERA_ACCESS_ENABLE: c_uint = 0x24b	/* Enables programmatic access to camera devices. (HUTRR72) */;
pub const KEY_CAMERA_ACCESS_DISABLE: c_uint = 0x24c	/* Disables programmatic access to camera devices. (HUTRR72) */;
pub const KEY_CAMERA_ACCESS_TOGGLE: c_uint = 0x24d	/* Toggles the current state of the camera access control. (HUTRR72) */;
pub const KEY_ACCESSIBILITY: c_uint = 0x24e	/* Toggles the system bound accessibility UI/command (HUTRR116) */;
pub const KEY_DO_NOT_DISTURB: c_uint = 0x24f	/* Toggles the system-wide "Do Not Disturb" control (HUTRR94)*/;
pub const KEY_BRIGHTNESS_MIN: c_uint = 0x250	/* Set Brightness to Minimum */;
pub const KEY_BRIGHTNESS_MAX: c_uint = 0x251	/* Set Brightness to Maximum */;
//
// Keycodes for hotkeys toggling the electronic privacy screen found on some
// laptops on/off. Note when the embedded-controller turns on/off the eprivacy
// screen itself then the state should be reported through drm connecter props:
// https://www.kernel.org/doc/html/latest/gpu/drm-kms.html#standard-connector-properties
// Except when implementing the drm connecter properties API is not possible
// because e.g. the firmware does not allow querying the presence and/or status
// of the eprivacy screen at boot.
//
pub const KEY_EPRIVACY_SCREEN_ON: c_uint = 0x252;
pub const KEY_EPRIVACY_SCREEN_OFF: c_uint = 0x253;
pub const KEY_ACTION_ON_SELECTION: c_uint = 0x254	/* AL Action on Selection (HUTRR119) */;
pub const KEY_CONTEXTUAL_INSERT: c_uint = 0x255	/* AL Contextual Insertion (HUTRR119) */;
pub const KEY_CONTEXTUAL_QUERY: c_uint = 0x256	/* AL Contextual Query (HUTRR119) */;
pub const KEY_KBDINPUTASSIST_PREV: c_uint = 0x260;
pub const KEY_KBDINPUTASSIST_NEXT: c_uint = 0x261;
pub const KEY_KBDINPUTASSIST_PREVGROUP: c_uint = 0x262;
pub const KEY_KBDINPUTASSIST_NEXTGROUP: c_uint = 0x263;
pub const KEY_KBDINPUTASSIST_ACCEPT: c_uint = 0x264;
pub const KEY_KBDINPUTASSIST_CANCEL: c_uint = 0x265;
// Diagonal movement keys
pub const KEY_RIGHT_UP: c_uint = 0x266;
pub const KEY_RIGHT_DOWN: c_uint = 0x267;
pub const KEY_LEFT_UP: c_uint = 0x268;
pub const KEY_LEFT_DOWN: c_uint = 0x269;
pub const KEY_ROOT_MENU: c_uint = 0x26a /* Show Device's Root Menu */;
// Show Top Menu of the Media (e.g. DVD)
pub const KEY_MEDIA_TOP_MENU: c_uint = 0x26b;
pub const KEY_NUMERIC_11: c_uint = 0x26c;
pub const KEY_NUMERIC_12: c_uint = 0x26d;
//
// Toggle Audio Description: refers to an audio service that helps blind and
// visually impaired consumers understand the action in a program. Note: in
// some countries this is referred to as "Video Description".
//
pub const KEY_AUDIO_DESC: c_uint = 0x26e;
pub const KEY_3D_MODE: c_uint = 0x26f;
pub const KEY_NEXT_FAVORITE: c_uint = 0x270;
pub const KEY_STOP_RECORD: c_uint = 0x271;
pub const KEY_PAUSE_RECORD: c_uint = 0x272;
pub const KEY_VOD: c_uint = 0x273 /* Video on Demand */;
pub const KEY_UNMUTE: c_uint = 0x274;
pub const KEY_FASTREVERSE: c_uint = 0x275;
pub const KEY_SLOWREVERSE: c_uint = 0x276;
//
// Control a data application associated with the currently viewed channel,
// e.g. teletext or data broadcast application (MHEG, MHP, HbbTV, etc.)
//
pub const KEY_DATA: c_uint = 0x277;
pub const KEY_ONSCREEN_KEYBOARD: c_uint = 0x278;
// Electronic privacy screen control
pub const KEY_PRIVACY_SCREEN_TOGGLE: c_uint = 0x279;
// Select an area of screen to be copied
pub const KEY_SELECTIVE_SCREENSHOT: c_uint = 0x27a;
// Move the focus to the next or previous user controllable element within a UI container
pub const KEY_NEXT_ELEMENT: c_uint = 0x27b;
pub const KEY_PREVIOUS_ELEMENT: c_uint = 0x27c;
// Toggle Autopilot engagement
pub const KEY_AUTOPILOT_ENGAGE_TOGGLE: c_uint = 0x27d;
// Shortcut Keys
pub const KEY_MARK_WAYPOINT: c_uint = 0x27e;
pub const KEY_SOS: c_uint = 0x27f;
pub const KEY_NAV_CHART: c_uint = 0x280;
pub const KEY_FISHING_CHART: c_uint = 0x281;
pub const KEY_SINGLE_RANGE_RADAR: c_uint = 0x282;
pub const KEY_DUAL_RANGE_RADAR: c_uint = 0x283;
pub const KEY_RADAR_OVERLAY: c_uint = 0x284;
pub const KEY_TRADITIONAL_SONAR: c_uint = 0x285;
pub const KEY_CLEARVU_SONAR: c_uint = 0x286;
pub const KEY_SIDEVU_SONAR: c_uint = 0x287;
pub const KEY_NAV_INFO: c_uint = 0x288;
pub const KEY_BRIGHTNESS_MENU: c_uint = 0x289;
//
// Some keyboards have keys which do not have a defined meaning, these keys
// are intended to be programmed / bound to macros by the user. For most
// keyboards with these macro-keys the key-sequence to inject, or action to
// take, is all handled by software on the host side. So from the kernel's
// point of view these are just normal keys.
//
// The KEY_MACRO# codes below are intended for such keys, which may be labeled
// e.g. G1-G18, or S1 - S30. The KEY_MACRO# codes MUST NOT be used for keys
// where the marking on the key does indicate a defined meaning / purpose.
//
// The KEY_MACRO# codes MUST also NOT be used as fallback for when no existing
// KEY_FOO define matches the marking / purpose. In this case a new KEY_FOO
// define MUST be added.
//
pub const KEY_MACRO1: c_uint = 0x290;
pub const KEY_MACRO2: c_uint = 0x291;
pub const KEY_MACRO3: c_uint = 0x292;
pub const KEY_MACRO4: c_uint = 0x293;
pub const KEY_MACRO5: c_uint = 0x294;
pub const KEY_MACRO6: c_uint = 0x295;
pub const KEY_MACRO7: c_uint = 0x296;
pub const KEY_MACRO8: c_uint = 0x297;
pub const KEY_MACRO9: c_uint = 0x298;
pub const KEY_MACRO10: c_uint = 0x299;
pub const KEY_MACRO11: c_uint = 0x29a;
pub const KEY_MACRO12: c_uint = 0x29b;
pub const KEY_MACRO13: c_uint = 0x29c;
pub const KEY_MACRO14: c_uint = 0x29d;
pub const KEY_MACRO15: c_uint = 0x29e;
pub const KEY_MACRO16: c_uint = 0x29f;
pub const KEY_MACRO17: c_uint = 0x2a0;
pub const KEY_MACRO18: c_uint = 0x2a1;
pub const KEY_MACRO19: c_uint = 0x2a2;
pub const KEY_MACRO20: c_uint = 0x2a3;
pub const KEY_MACRO21: c_uint = 0x2a4;
pub const KEY_MACRO22: c_uint = 0x2a5;
pub const KEY_MACRO23: c_uint = 0x2a6;
pub const KEY_MACRO24: c_uint = 0x2a7;
pub const KEY_MACRO25: c_uint = 0x2a8;
pub const KEY_MACRO26: c_uint = 0x2a9;
pub const KEY_MACRO27: c_uint = 0x2aa;
pub const KEY_MACRO28: c_uint = 0x2ab;
pub const KEY_MACRO29: c_uint = 0x2ac;
pub const KEY_MACRO30: c_uint = 0x2ad;
//
// Some keyboards with the macro-keys described above have some extra keys
// for controlling the host-side software responsible for the macro handling:
// -A macro recording start/stop key. Note that not all keyboards which emit
// KEY_MACRO_RECORD_START will also emit KEY_MACRO_RECORD_STOP if
// KEY_MACRO_RECORD_STOP is not advertised, then KEY_MACRO_RECORD_START
// should be interpreted as a recording start/stop toggle;
// -Keys for switching between different macro (pre)sets, either a key for
// cycling through the configured presets or keys to directly select a preset.
//
pub const KEY_MACRO_RECORD_START: c_uint = 0x2b0;
pub const KEY_MACRO_RECORD_STOP: c_uint = 0x2b1;
pub const KEY_MACRO_PRESET_CYCLE: c_uint = 0x2b2;
pub const KEY_MACRO_PRESET1: c_uint = 0x2b3;
pub const KEY_MACRO_PRESET2: c_uint = 0x2b4;
pub const KEY_MACRO_PRESET3: c_uint = 0x2b5;
//
// Some keyboards have a buildin LCD panel where the contents are controlled
// by the host. Often these have a number of keys directly below the LCD
// intended for controlling a menu shown on the LCD. These keys often don't
// have any labeling so we just name them KEY_KBD_LCD_MENU#
//
pub const KEY_KBD_LCD_MENU1: c_uint = 0x2b8;
pub const KEY_KBD_LCD_MENU2: c_uint = 0x2b9;
pub const KEY_KBD_LCD_MENU3: c_uint = 0x2ba;
pub const KEY_KBD_LCD_MENU4: c_uint = 0x2bb;
pub const KEY_KBD_LCD_MENU5: c_uint = 0x2bc;
// Performance Boost key (Alienware)/G-Mode key (Dell)
pub const KEY_PERFORMANCE: c_uint = 0x2bd;
pub const BTN_TRIGGER_HAPPY: c_uint = 0x2c0;
pub const BTN_TRIGGER_HAPPY1: c_uint = 0x2c0;
pub const BTN_TRIGGER_HAPPY2: c_uint = 0x2c1;
pub const BTN_TRIGGER_HAPPY3: c_uint = 0x2c2;
pub const BTN_TRIGGER_HAPPY4: c_uint = 0x2c3;
pub const BTN_TRIGGER_HAPPY5: c_uint = 0x2c4;
pub const BTN_TRIGGER_HAPPY6: c_uint = 0x2c5;
pub const BTN_TRIGGER_HAPPY7: c_uint = 0x2c6;
pub const BTN_TRIGGER_HAPPY8: c_uint = 0x2c7;
pub const BTN_TRIGGER_HAPPY9: c_uint = 0x2c8;
pub const BTN_TRIGGER_HAPPY10: c_uint = 0x2c9;
pub const BTN_TRIGGER_HAPPY11: c_uint = 0x2ca;
pub const BTN_TRIGGER_HAPPY12: c_uint = 0x2cb;
pub const BTN_TRIGGER_HAPPY13: c_uint = 0x2cc;
pub const BTN_TRIGGER_HAPPY14: c_uint = 0x2cd;
pub const BTN_TRIGGER_HAPPY15: c_uint = 0x2ce;
pub const BTN_TRIGGER_HAPPY16: c_uint = 0x2cf;
pub const BTN_TRIGGER_HAPPY17: c_uint = 0x2d0;
pub const BTN_TRIGGER_HAPPY18: c_uint = 0x2d1;
pub const BTN_TRIGGER_HAPPY19: c_uint = 0x2d2;
pub const BTN_TRIGGER_HAPPY20: c_uint = 0x2d3;
pub const BTN_TRIGGER_HAPPY21: c_uint = 0x2d4;
pub const BTN_TRIGGER_HAPPY22: c_uint = 0x2d5;
pub const BTN_TRIGGER_HAPPY23: c_uint = 0x2d6;
pub const BTN_TRIGGER_HAPPY24: c_uint = 0x2d7;
pub const BTN_TRIGGER_HAPPY25: c_uint = 0x2d8;
pub const BTN_TRIGGER_HAPPY26: c_uint = 0x2d9;
pub const BTN_TRIGGER_HAPPY27: c_uint = 0x2da;
pub const BTN_TRIGGER_HAPPY28: c_uint = 0x2db;
pub const BTN_TRIGGER_HAPPY29: c_uint = 0x2dc;
pub const BTN_TRIGGER_HAPPY30: c_uint = 0x2dd;
pub const BTN_TRIGGER_HAPPY31: c_uint = 0x2de;
pub const BTN_TRIGGER_HAPPY32: c_uint = 0x2df;
pub const BTN_TRIGGER_HAPPY33: c_uint = 0x2e0;
pub const BTN_TRIGGER_HAPPY34: c_uint = 0x2e1;
pub const BTN_TRIGGER_HAPPY35: c_uint = 0x2e2;
pub const BTN_TRIGGER_HAPPY36: c_uint = 0x2e3;
pub const BTN_TRIGGER_HAPPY37: c_uint = 0x2e4;
pub const BTN_TRIGGER_HAPPY38: c_uint = 0x2e5;
pub const BTN_TRIGGER_HAPPY39: c_uint = 0x2e6;
pub const BTN_TRIGGER_HAPPY40: c_uint = 0x2e7;
// We avoid low common keys in module aliases so they don't get huge.

pub const KEY_MAX: c_uint = 0x2ff;

//
// Relative axes
//
pub const REL_X: c_uint = 0x00;
pub const REL_Y: c_uint = 0x01;
pub const REL_Z: c_uint = 0x02;
pub const REL_RX: c_uint = 0x03;
pub const REL_RY: c_uint = 0x04;
pub const REL_RZ: c_uint = 0x05;
pub const REL_HWHEEL: c_uint = 0x06;
pub const REL_DIAL: c_uint = 0x07;
pub const REL_WHEEL: c_uint = 0x08;
pub const REL_MISC: c_uint = 0x09;
//
// 0x0a is reserved and should not be used in input drivers.
// It was used by HID as REL_MISC+1 and userspace needs to detect if
// the next REL_* event is correct or is just REL_MISC + n.
// We define here REL_RESERVED so userspace can rely on it and detect
// the situation described above.
//
pub const REL_RESERVED: c_uint = 0x0a;
pub const REL_WHEEL_HI_RES: c_uint = 0x0b;
pub const REL_HWHEEL_HI_RES: c_uint = 0x0c;
pub const REL_MAX: c_uint = 0x0f;

//
// Absolute axes
//
pub const ABS_X: c_uint = 0x00;
pub const ABS_Y: c_uint = 0x01;
pub const ABS_Z: c_uint = 0x02;
pub const ABS_RX: c_uint = 0x03;
pub const ABS_RY: c_uint = 0x04;
pub const ABS_RZ: c_uint = 0x05;
pub const ABS_THROTTLE: c_uint = 0x06;
pub const ABS_RUDDER: c_uint = 0x07;
pub const ABS_WHEEL: c_uint = 0x08;
pub const ABS_GAS: c_uint = 0x09;
pub const ABS_BRAKE: c_uint = 0x0a;
pub const ABS_HAT0X: c_uint = 0x10;
pub const ABS_HAT0Y: c_uint = 0x11;
pub const ABS_HAT1X: c_uint = 0x12;
pub const ABS_HAT1Y: c_uint = 0x13;
pub const ABS_HAT2X: c_uint = 0x14;
pub const ABS_HAT2Y: c_uint = 0x15;
pub const ABS_HAT3X: c_uint = 0x16;
pub const ABS_HAT3Y: c_uint = 0x17;
pub const ABS_PRESSURE: c_uint = 0x18;
pub const ABS_DISTANCE: c_uint = 0x19;
pub const ABS_TILT_X: c_uint = 0x1a;
pub const ABS_TILT_Y: c_uint = 0x1b;
pub const ABS_TOOL_WIDTH: c_uint = 0x1c;
pub const ABS_VOLUME: c_uint = 0x20;
pub const ABS_PROFILE: c_uint = 0x21;
pub const ABS_SND_PROFILE: c_uint = 0x22;
pub const ABS_MISC: c_uint = 0x28;
//
// 0x2e is reserved and should not be used in input drivers.
// It was used by HID as ABS_MISC+6 and userspace needs to detect if
// the next ABS_* event is correct or is just ABS_MISC + n.
// We define here ABS_RESERVED so userspace can rely on it and detect
// the situation described above.
//
pub const ABS_RESERVED: c_uint = 0x2e;
pub const ABS_MT_SLOT: c_uint = 0x2f	/* MT slot being modified */;
pub const ABS_MT_TOUCH_MAJOR: c_uint = 0x30	/* Major axis of touching ellipse */;
pub const ABS_MT_TOUCH_MINOR: c_uint = 0x31	/* Minor axis (omit if circular) */;
pub const ABS_MT_WIDTH_MAJOR: c_uint = 0x32	/* Major axis of approaching ellipse */;
pub const ABS_MT_WIDTH_MINOR: c_uint = 0x33	/* Minor axis (omit if circular) */;
pub const ABS_MT_ORIENTATION: c_uint = 0x34	/* Ellipse orientation */;
pub const ABS_MT_POSITION_X: c_uint = 0x35	/* Center X touch position */;
pub const ABS_MT_POSITION_Y: c_uint = 0x36	/* Center Y touch position */;
pub const ABS_MT_TOOL_TYPE: c_uint = 0x37	/* Type of touching device */;
pub const ABS_MT_BLOB_ID: c_uint = 0x38	/* Group a set of packets as a blob */;
pub const ABS_MT_TRACKING_ID: c_uint = 0x39	/* Unique ID of initiated contact */;
pub const ABS_MT_PRESSURE: c_uint = 0x3a	/* Pressure on contact area */;
pub const ABS_MT_DISTANCE: c_uint = 0x3b	/* Contact hover distance */;
pub const ABS_MT_TOOL_X: c_uint = 0x3c	/* Center X tool position */;
pub const ABS_MT_TOOL_Y: c_uint = 0x3d	/* Center Y tool position */;
pub const ABS_MAX: c_uint = 0x3f;

//
// Switch events
//
pub const SW_LID: c_uint = 0x00  /* set = lid shut */;
pub const SW_TABLET_MODE: c_uint = 0x01  /* set = tablet mode */;
pub const SW_HEADPHONE_INSERT: c_uint = 0x02  /* set = inserted */;
pub const SW_RFKILL_ALL: c_uint = 0x03  /* rfkill master switch, type "any";

pub const SW_MICROPHONE_INSERT: c_uint = 0x04  /* set = inserted */;
pub const SW_DOCK: c_uint = 0x05  /* set = plugged into dock */;
pub const SW_LINEOUT_INSERT: c_uint = 0x06  /* set = inserted */;
pub const SW_JACK_PHYSICAL_INSERT: c_uint = 0x07  /* set = mechanical switch set */;
pub const SW_VIDEOOUT_INSERT: c_uint = 0x08  /* set = inserted */;
pub const SW_CAMERA_LENS_COVER: c_uint = 0x09  /* set = lens covered */;
pub const SW_KEYPAD_SLIDE: c_uint = 0x0a  /* set = keypad slide out */;
pub const SW_FRONT_PROXIMITY: c_uint = 0x0b  /* set = front proximity sensor active */;
pub const SW_ROTATE_LOCK: c_uint = 0x0c  /* set = rotate locked/disabled */;
pub const SW_LINEIN_INSERT: c_uint = 0x0d  /* set = inserted */;
pub const SW_MUTE_DEVICE: c_uint = 0x0e  /* set = device disabled */;
pub const SW_PEN_INSERTED: c_uint = 0x0f  /* set = pen inserted */;
pub const SW_MACHINE_COVER: c_uint = 0x10  /* set = cover closed */;
pub const SW_USB_INSERT: c_uint = 0x11  /* set = USB audio device connected */;
pub const SW_MAX: c_uint = 0x11;

//
// Misc events
//
pub const MSC_SERIAL: c_uint = 0x00;
pub const MSC_PULSELED: c_uint = 0x01;
pub const MSC_GESTURE: c_uint = 0x02;
pub const MSC_RAW: c_uint = 0x03;
pub const MSC_SCAN: c_uint = 0x04;
pub const MSC_TIMESTAMP: c_uint = 0x05;
pub const MSC_MAX: c_uint = 0x07;

//
// LEDs
//
pub const LED_NUML: c_uint = 0x00;
pub const LED_CAPSL: c_uint = 0x01;
pub const LED_SCROLLL: c_uint = 0x02;
pub const LED_COMPOSE: c_uint = 0x03;
pub const LED_KANA: c_uint = 0x04;
pub const LED_SLEEP: c_uint = 0x05;
pub const LED_SUSPEND: c_uint = 0x06;
pub const LED_MUTE: c_uint = 0x07;
pub const LED_MISC: c_uint = 0x08;
pub const LED_MAIL: c_uint = 0x09;
pub const LED_CHARGING: c_uint = 0x0a;
pub const LED_MAX: c_uint = 0x0f;

//
// Autorepeat values
//
pub const REP_DELAY: c_uint = 0x00;
pub const REP_PERIOD: c_uint = 0x01;
pub const REP_MAX: c_uint = 0x01;

//
// Sounds
//
pub const SND_CLICK: c_uint = 0x00;
pub const SND_BELL: c_uint = 0x01;
pub const SND_TONE: c_uint = 0x02;
pub const SND_MAX: c_uint = 0x07;

//
// ABS_SND_PROFILE values
//
pub const SND_PROFILE_SILENT: c_uint = 0x00;
pub const SND_PROFILE_VIBRATE: c_uint = 0x01;
pub const SND_PROFILE_RING: c_uint = 0x02;
