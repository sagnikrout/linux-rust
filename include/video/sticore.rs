//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/sticore.h
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
// generic STI structures & functions

pub const STI_DEV_NAME_LENGTH: c_int = 32;
pub const STI_MONITOR_MAX: c_int = 256;
pub const STI_FONT_HPROMAN8: c_int = 1;
pub const STI_FONT_KANA8: c_int = 2;
pub const ALT_CODE_TYPE_UNKNOWN: c_uint = 0x00	/* alt code type values */;
pub const ALT_CODE_TYPE_PA_RISC_64: c_uint = 0x01;
// The latency of the STI functions cannot really be reduced by setting
// this to 0;  STI doesn't seem to be designed to allow calling a different
// function (or the same function with different arguments) after a
// function exited with 1 as return value.
//
// As all of the functions below could be called from interrupt context,
// we have to spin_lock_irqsave around the do { ret = bla(); } while(ret==1)
// block.  Really bad latency there.
//
// Probably the best solution to all this is have the generic code manage
// the screen buffer and a kernel thread to call STI occasionally.
//
// Luckily, the frame buffer guys have the same problem so we can just wait
// for them to fix it and steal their solution.   prumpf
//

pub const STI_WAIT: c_int = 1;

// sti_font_xy() use the native font ROM !

// STI function configuration structs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_glob_cfg_ext {
    pub /: *mut *mut u8 curr_mon; / current monitor configured,
    pub /: *mut *mut u8 friendly_boot; / in friendly boot mode,
    pub /: *mut *mut s16 power; / power calculation (in Watts),
    pub /: *mut *mut s32 freq_ref; / frequency reference,
    pub /: *mut *mut *mut u32 sti_mem_addr; / pointer to global sti memory (size=sti_mem_request),
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_glob_cfg {
    pub /: *mut *mut s32 text_planes; / number of planes used for text,
    pub /: *mut *mut s16 onscreen_x; / screen width in pixels,
    pub /: *mut *mut s16 onscreen_y; / screen height in pixels,
    pub /: *mut *mut s16 offscreen_x; / offset width in pixels,
    pub /: *mut *mut s16 offscreen_y; / offset height in pixels,
    pub /: *mut *mut s16 total_x; / frame buffer width in pixels,
    pub /: *mut *mut s16 total_y; / frame buffer height in pixels,
    pub /: *mut *mut *mut u32 region_ptrs[STI_REGION_MAX]; / region pointers,
    pub /: *mut *mut s32 reent_lvl; / storage for reentry level value,
    pub /: *mut *mut *mut u32 save_addr; / where to save or restore reentrant state,
    pub /: *mut *mut *mut u32 ext_ptr; / pointer to extended glob_cfg data structure,
}

// STI init function structs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_init_flags {
    pub /: *mut *mut u32 wait : 1; / should routine idle wait or not,
    pub /: *mut *mut u32 reset : 1; / hard reset the device?,
    pub /: *mut *mut u32 text : 1; / turn on text display planes?,
    pub /: *mut *mut u32 nontext : 1; / turn on non-text display planes?,
    pub /: *mut *mut u32 clear : 1; / clear text display planes?,
    pub /: *mut *mut u32 cmap_blk : 1; / non-text planes cmap black?,
    pub /: *mut *mut u32 enable_be_timer : 1; / enable bus error timer,
    pub /: *mut *mut u32 enable_be_int : 1; / enable bus error timer interrupt,
    pub /: *mut *mut u32 no_chg_tx : 1; / don't change text settings,
    pub /: *mut *mut u32 no_chg_ntx : 1; / don't change non-text settings,
    pub /: *mut *mut u32 no_chg_bet : 1; / don't change berr timer settings,
    pub /: *mut *mut u32 no_chg_bei : 1; / don't change berr int settings,
    pub /: *mut *mut u32 init_cmap_tx : 1; / initialize cmap for text planes,
    pub /: *mut *mut u32 cmt_chg : 1; / change current monitor type,
    pub /: *mut *mut u32 retain_ie : 1; / don't allow reset to clear int enables,
    pub /: *mut *mut u32 caller_bootrom : 1; / set only by bootrom for each call,
    pub /: *mut *mut u32 caller_kernel : 1; / set only by kernel for each call,
    pub /: *mut *mut u32 caller_other : 1; / set only by non-[BR/K] caller,
    pub /: *mut *mut u32 pad : 14; / pad to word boundary,
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_init_inptr_ext {
    pub /: *mut *mut u8 config_mon_type; / configure to monitor type,
    pub /: *mut *mut u8 pad[1]; / pad to word boundary,
    pub /: *mut *mut u16 inflight_data; / inflight data possible on PCI,
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_init_inptr {
    pub /: *mut *mut s32 text_planes; / number of planes to use for text,
    pub structure*/: *mut *mut *mut u32 ext_ptr; / pointer to extended init_graph inptr data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_init_outptr {
    pub /: *mut *mut s32 errno; / error number on failure,
    pub /: *mut *mut s32 text_planes; / number of planes used for text,
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

// STI configuration function structs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_conf_flags {
    pub /: *mut *mut u32 wait : 1; / should routine idle wait or not,
    pub /: *mut *mut u32 pad : 31; / pad to word boundary,
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_conf_inptr {
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_conf_outptr_ext {
    pub /: *mut *mut u32 crt_config[3]; / hardware specific X11/OGL information,
    pub crt_hdw: [u32; 3],
    pub future_ptr: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_conf_outptr {
    pub /: *mut *mut s32 errno; / error number on failure,
    pub /: *mut *mut s16 onscreen_x; / screen width in pixels,
    pub /: *mut *mut s16 onscreen_y; / screen height in pixels,
    pub /: *mut *mut s16 offscreen_x; / offscreen width in pixels,
    pub /: *mut *mut s16 offscreen_y; / offscreen height in pixels,
    pub /: *mut *mut s16 total_x; / frame buffer width in pixels,
    pub /: *mut *mut s16 total_y; / frame buffer height in pixels,
    pub /: *mut *mut s32 bits_per_pixel; / bits/pixel device has configured,
    pub /: *mut *mut s32 bits_used; / bits which can be accessed,
    pub /: *mut *mut s32 planes; / number of fb planes in system,
    pub /: *mut *mut u8 dev_name[STI_DEV_NAME_LENGTH]; / null terminated product name,
    pub /: *mut *mut u32 attributes; / flags denoting attributes,
    pub /: *mut *mut *mut u32 ext_ptr; / pointer to future data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_rom {
    pub type: [u8; 4],
    pub res004: u8,
    pub num_mons: u8,
    pub revno: [u8; 2],
    pub graphics_id: [u32; 2],
    pub font_start: u32,
    pub statesize: u32,
    pub last_addr: u32,
    pub region_list: u32,
    pub reentsize: u16,
    pub maxtime: u16,
    pub mon_tbl_addr: u32,
    pub user_data_addr: u32,
    pub sti_mem_req: u32,
    pub user_data_size: u32,
    pub power: u16,
    pub bus_support: u8,
    pub ext_bus_support: u8,
    pub alt_code_type: u8,
    pub ext_dd_struct: [u8; 3],
    pub cfb_addr: u32,
    pub init_graph: u32,
    pub state_mgmt: u32,
    pub font_unpmv: u32,
    pub block_move: u32,
    pub self_test: u32,
    pub excep_hdlr: u32,
    pub inq_conf: u32,
    pub set_cm_entry: u32,
    pub dma_ctrl: u32,
    pub 4]: *mut *mut u8 res040[7,
    pub init_graph_addr: u32,
    pub state_mgmt_addr: u32,
    pub font_unp_addr: u32,
    pub block_move_addr: u32,
    pub self_test_addr: u32,
    pub excep_hdlr_addr: u32,
    pub inq_conf_addr: u32,
    pub set_cm_entry_addr: u32,
    pub image_unpack_addr: u32,
    pub pa_risx_addrs: [u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_rom_font {
    pub first_char: u16,
    pub last_char: u16,
    pub width: u8,
    pub height: u8,
    pub /: *mut *mut u8 font_type; / language type,
    pub bytes_per_char: u8,
    pub /: *mut *mut s32 next_font; / note: signed int,
    pub underline_height: u8,
    pub underline_pos: u8,
    pub res008: [u8; 2],
}

// sticore internal font handling
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_cooked_font {
    pub /: *mut *mut *mut sti_rom_font raw; / native ptr for STI functions,
    pub /: *mut *mut *mut void raw_ptr; / kmalloc'ed font data,
    pub next_font: *mut sti_cooked_font,
    pub width: int height,,
    pub refcount: c_int,
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_cooked_rom {
    pub raw: *mut sti_rom,
    pub font_start: *mut sti_cooked_font,
}

// STI font printing function structs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_font_inptr {
    pub /: *mut *mut *mut u32 font_start_addr; / address of font start,
    pub /: *mut *mut s16 index; / index into font table of character,
    pub /: *mut *mut u8 fg_color; / foreground color of character,
    pub /: *mut *mut u8 bg_color; / background color of character,
    pub /: *mut *mut s16 dest_x; / X location of character upper left,
    pub /: *mut *mut s16 dest_y; / Y location of character upper left,
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_font_flags {
    pub /: *mut *mut u32 wait : 1; / should routine idle wait or not,
    pub /: *mut *mut u32 non_text : 1; / font unpack/move in non_text planes =1, text =0,
    pub /: *mut *mut u32 pad : 30; / pad to word boundary,
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_font_outptr {
    pub /: *mut *mut s32 errno; / error number on failure,
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

// STI blockmove structs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_blkmv_flags {
    pub /: *mut *mut u32 wait : 1; / should routine idle wait or not,
    pub /: *mut *mut u32 color : 1; / change color during move?,
    pub /: *mut *mut u32 clear : 1; / clear during move?,
    pub /: *mut *mut u32 non_text : 1; / block move in non_text planes =1, text =0,
    pub /: *mut *mut u32 pad : 28; / pad to word boundary,
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_blkmv_inptr {
    pub /: *mut *mut u8 fg_color; / foreground color after move,
    pub /: *mut *mut u8 bg_color; / background color after move,
    pub /: *mut *mut s16 src_x; / source upper left pixel x location,
    pub /: *mut *mut s16 src_y; / source upper left pixel y location,
    pub /: *mut *mut s16 dest_x; / dest upper left pixel x location,
    pub /: *mut *mut s16 dest_y; / dest upper left pixel y location,
    pub /: *mut *mut s16 width; / block width in pixels,
    pub /: *mut *mut s16 height; / block height in pixels,
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_blkmv_outptr {
    pub /: *mut *mut s32 errno; / error number on failure,
    pub /: *mut *mut *mut u32 future_ptr; / pointer to future data,
}

// sti_all_data is an internal struct which needs to be allocated in
// low memory (< 4GB) if STI is used with 32bit STI on a 64bit kernel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_all_data {
    pub glob_cfg: sti_glob_cfg,
    pub glob_cfg_ext: sti_glob_cfg_ext,
    pub inq_inptr: sti_conf_inptr,
    pub /: *mut *mut sti_conf_outptr inq_outptr; / configuration,
    pub inq_outptr_ext: sti_conf_outptr_ext,
    pub init_inptr_ext: sti_init_inptr_ext,
    pub init_inptr: sti_init_inptr,
    pub init_outptr: sti_init_outptr,
    pub blkmv_inptr: sti_blkmv_inptr,
    pub blkmv_outptr: sti_blkmv_outptr,
    pub font_inptr: sti_font_inptr,
    pub font_outptr: sti_font_outptr,
// leave as last entries
    pub long)]: unsigned long save_addr[1024 / sizeof(unsigned,
// min 256 bytes which is STI default, max sti->sti_mem_request
    pub long)]: unsigned long sti_mem_addr[256 / sizeof(unsigned,
// do not add something below here !
}

// internal generic STI struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_struct {
    pub lock: spinlock_t,
// char **mon_strings;
    pub sti_mem_request: c_int,
    pub graphics_id: [u32; 2],
    pub rom: *mut sti_cooked_rom,
    pub font_unpmv: c_ulong,
    pub block_move: c_ulong,
    pub init_graph: c_ulong,
    pub inq_conf: c_ulong,
    pub /: *mut *mut int do_call64; / call 64-bit code,
// all following fields are initialized by the generic routines
    pub text_planes: c_int,
    pub regions: [region_t; STI_REGION_MAX],
    pub regions_phys: [c_ulong; STI_REGION_MAX],
    pub /: *mut *mut *mut sti_glob_cfg glob_cfg; / points into sti_all_data,
    pub wordmode: c_int,
    pub /: *mut *mut *mut sti_cooked_font font; / ptr to selected font (cooked),
    pub pd: *mut pci_dev,
// PCI data structures (pg. 17ff from sti.pdf)
    pub /: *mut *mut u8 rm_entry[16]; / pci region mapper array == pci config space offset,
// pointer to the parent device
    pub dev: *mut device,
// pointer to all internal data
    pub sti_data: *mut sti_all_data,
// pa_path of this device
    pub pa_path: [c_char; 24],
}

// sticore interface functions
extern "C" {
    pub fn sti_font_convert_bytemode(sti: *mut sti_struct, f: *mut sti_cooked_font);
}
// sticore main function to call STI firmware
// functions to call the STI ROM directly
