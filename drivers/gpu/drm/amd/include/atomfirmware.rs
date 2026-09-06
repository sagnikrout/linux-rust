//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/atomfirmware.h
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


// \
//
// File Name      atomfirmware.h
// Project        This is an interface header file between atombios and OS GPU drivers for SoC15 products
//
// Description    header file of general definitions for OS and pre-OS video drivers
//
// Copyright 2014 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// IMPORTANT NOTES
// If a change in VBIOS/Driver/Tool's interface is only needed for SoC15 and forward products, then the change is only needed in this atomfirmware.h header file.
// If a change in VBIOS/Driver/Tool's interface is only needed for pre-SoC15 products, then the change is only needed in atombios.h header file.
// If a change is needed for both pre and post SoC15 products, then the change has to be made separately and might be differently in both atomfirmware.h and atombios.h.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_bios_header_version_def {
    ATOM_MAJOR_VERSION        =0x0003,
    ATOM_MINOR_VERSION        =0x0003,
}

pub type uint32_t = c_ulong;

pub type uint16_t = c_ushort;

pub type uint8_t = c_uchar;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_crtc_def {
    ATOM_CRTC1      =0,
    ATOM_CRTC2      =1,
    ATOM_CRTC3      =2,
    ATOM_CRTC4      =3,
    ATOM_CRTC5      =4,
    ATOM_CRTC6      =5,
    ATOM_CRTC_INVALID  =0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_ppll_def {
    ATOM_PPLL0          =2,
    ATOM_GCK_DFS        =8,
    ATOM_FCH_CLK        =9,
    ATOM_DP_DTO         =11,
    ATOM_COMBOPHY_PLL0  =20,
    ATOM_COMBOPHY_PLL1  =21,
    ATOM_COMBOPHY_PLL2  =22,
    ATOM_COMBOPHY_PLL3  =23,
    ATOM_COMBOPHY_PLL4  =24,
    ATOM_COMBOPHY_PLL5  =25,
    ATOM_PPLL_INVALID   =0xff,
}

// define ASIC internal encoder id ( bit vector ), used for CRTC_SourceSel
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_dig_def {
    ASIC_INT_DIG1_ENCODER_ID  =0x03,
    ASIC_INT_DIG2_ENCODER_ID  =0x09,
    ASIC_INT_DIG3_ENCODER_ID  =0x0a,
    ASIC_INT_DIG4_ENCODER_ID  =0x0b,
    ASIC_INT_DIG5_ENCODER_ID  =0x0c,
    ASIC_INT_DIG6_ENCODER_ID  =0x0d,
    ASIC_INT_DIG7_ENCODER_ID  =0x0e,
}

// ucEncoderMode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_encoder_refclk_src_def {
    ENCODER_REFCLK_SRC_P1PLL      =0,
    ENCODER_REFCLK_SRC_P2PLL      =1,
    ENCODER_REFCLK_SRC_P3PLL      =2,
    ENCODER_REFCLK_SRC_EXTCLK     =3,
    ENCODER_REFCLK_SRC_INVALID    =0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_scaler_def {
    ATOM_SCALER_DISABLE          =0,  /*scaler bypass mode, auto-center & no replication*/
    ATOM_SCALER_CENTER           =1,  //For Fudo, it's bypass and auto-center & auto replication
    ATOM_SCALER_EXPANSION        =2,  /*scaler expansion by 2 tap alpha blending mode*/
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_operation_def {
    ATOM_DISABLE             = 0,
    ATOM_ENABLE              = 1,
    ATOM_INIT                = 7,
    ATOM_GET_STATUS          = 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_embedded_display_op_def {
    ATOM_LCD_BL_OFF                = 2,
    ATOM_LCD_BL_OM                 = 3,
    ATOM_LCD_BL_BRIGHTNESS_CONTROL = 4,
    ATOM_LCD_SELFTEST_START        = 5,
    ATOM_LCD_SELFTEST_STOP         = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_spread_spectrum_mode {
    ATOM_SS_CENTER_OR_DOWN_MODE_MASK  = 0x01,
    ATOM_SS_DOWN_SPREAD_MODE          = 0x00,
    ATOM_SS_CENTRE_SPREAD_MODE        = 0x01,
    ATOM_INT_OR_EXT_SS_MASK           = 0x02,
    ATOM_INTERNAL_SS_MASK             = 0x00,
    ATOM_EXTERNAL_SS_MASK             = 0x02,
}

// define panel bit per color
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_panel_bit_per_color {
    PANEL_BPC_UNDEFINE     =0x00,
    PANEL_6BIT_PER_COLOR   =0x01,
    PANEL_8BIT_PER_COLOR   =0x02,
    PANEL_10BIT_PER_COLOR  =0x03,
    PANEL_12BIT_PER_COLOR  =0x04,
    PANEL_16BIT_PER_COLOR  =0x05,
}

// ucVoltageType
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_dgpu_vram_type {
    ATOM_DGPU_VRAM_TYPE_GDDR5 = 0x50,
    ATOM_DGPU_VRAM_TYPE_HBM2  = 0x60,
    ATOM_DGPU_VRAM_TYPE_HBM2E = 0x61,
    ATOM_DGPU_VRAM_TYPE_GDDR6 = 0x70,
    ATOM_DGPU_VRAM_TYPE_HBM3 = 0x80,
    ATOM_DGPU_VRAM_TYPE_HBM3E = 0x81,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_dp_vs_preemph_def {
    DP_VS_LEVEL0_PREEMPH_LEVEL0 = 0x00,
    DP_VS_LEVEL1_PREEMPH_LEVEL0 = 0x01,
    DP_VS_LEVEL2_PREEMPH_LEVEL0 = 0x02,
    DP_VS_LEVEL3_PREEMPH_LEVEL0 = 0x03,
    DP_VS_LEVEL0_PREEMPH_LEVEL1 = 0x08,
    DP_VS_LEVEL1_PREEMPH_LEVEL1 = 0x09,
    DP_VS_LEVEL2_PREEMPH_LEVEL1 = 0x0a,
    DP_VS_LEVEL0_PREEMPH_LEVEL2 = 0x10,
    DP_VS_LEVEL1_PREEMPH_LEVEL2 = 0x11,
    DP_VS_LEVEL0_PREEMPH_LEVEL3 = 0x18,
}

pub const BIOS_STRING_LENGTH: c_int = 43;
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_string_def {
    asic_bus_type_pcie_string = "PCI_EXPRESS",
    atom_fire_gl_string       = "FGL",
    atom_bios_string          = "ATOM"
}

//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atombios_image_offset {
    OFFSET_TO_ATOM_ROM_HEADER_POINTER          = 0x00000048,
    OFFSET_TO_ATOM_ROM_IMAGE_SIZE              = 0x00000002,
    OFFSET_TO_ATOMBIOS_ASIC_BUS_MEM_TYPE       = 0x94,
    MAXSIZE_OF_ATOMBIOS_ASIC_BUS_MEM_TYPE      = 20,  /*including the terminator 0x0!*/
    OFFSET_TO_GET_ATOMBIOS_NUMBER_OF_STRINGS   = 0x2f,
    OFFSET_TO_GET_ATOMBIOS_STRING_START        = 0x6e,
    OFFSET_TO_VBIOS_PART_NUMBER                = 0x80,
    OFFSET_TO_VBIOS_DATE                       = 0x50,
}

//
// Common header for all tables (Data table, Command function).
// Every table pointed in _ATOM_MASTER_DATA_TABLE has this common header.
// And the pointer actually points to this header.
//
// Structure stores the ROM header.
//
// ==============================hw function portion======================================================================
//
// Structures used in Command.mtb, each function name is not given here since those function could change from time to time
// The real functionality of each function is associated with the parameter structure version when defined
// For all internal cmd function definitions, please reference to atomstruct.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_master_list_of_command_functions_v2_1 {
    pub //Function: uint16_t asic_init;,
    pub one: uint16_t cmd_function1; //used as an internal,
    pub one: uint16_t cmd_function2; //used as an internal,
    pub one: uint16_t cmd_function3; //used as an internal,
    pub //Function: uint16_t digxencodercontrol;,
    pub one: uint16_t cmd_function5; //used as an internal,
    pub one: uint16_t cmd_function6; //used as an internal,
    pub one: uint16_t cmd_function7; //used as an internal,
    pub one: uint16_t cmd_function8; //used as an internal,
    pub one: uint16_t cmd_function9; //used as an internal,
    pub //Function: uint16_t setengineclock;,
    pub //Function: uint16_t setmemoryclock;,
    pub //Function: uint16_t setpixelclock;,
    pub //Function: uint16_t enabledisppowergating;,
    pub one: uint16_t cmd_function14; //used as an internal,
    pub one: uint16_t cmd_function15; //used as an internal,
    pub one: uint16_t cmd_function16; //used as an internal,
    pub one: uint16_t cmd_function17; //used as an internal,
    pub one: uint16_t cmd_function18; //used as an internal,
    pub one: uint16_t cmd_function19; //used as an internal,
    pub one: uint16_t cmd_function20; //used as an internal,
    pub one: uint16_t cmd_function21; //used as an internal,
    pub one: uint16_t cmd_function22; //used as an internal,
    pub one: uint16_t cmd_function23; //used as an internal,
    pub one: uint16_t cmd_function24; //used as an internal,
    pub one: uint16_t cmd_function25; //used as an internal,
    pub one: uint16_t cmd_function26; //used as an internal,
    pub one: uint16_t cmd_function27; //used as an internal,
    pub one: uint16_t cmd_function28; //used as an internal,
    pub one: uint16_t cmd_function29; //used as an internal,
    pub one: uint16_t cmd_function30; //used as an internal,
    pub one: uint16_t cmd_function31; //used as an internal,
    pub one: uint16_t cmd_function32; //used as an internal,
    pub one: uint16_t cmd_function33; //used as an internal,
    pub //Function: uint16_t blankcrtc;,
    pub //Function: uint16_t enablecrtc;,
    pub one: uint16_t cmd_function36; //used as an internal,
    pub one: uint16_t cmd_function37; //used as an internal,
    pub one: uint16_t cmd_function38; //used as an internal,
    pub one: uint16_t cmd_function39; //used as an internal,
    pub one: uint16_t cmd_function40; //used as an internal,
    pub //Function: uint16_t getsmuclockinfo;,
    pub //Function: uint16_t selectcrtc_source;,
    pub one: uint16_t cmd_function43; //used as an internal,
    pub one: uint16_t cmd_function44; //used as an internal,
    pub one: uint16_t cmd_function45; //used as an internal,
    pub //Function: uint16_t setdceclock;,
    pub //Function: uint16_t getmemoryclock;,
    pub //Function: uint16_t getengineclock;,
    pub //Function: uint16_t setcrtc_usingdtdtiming;,
    pub //Function: uint16_t externalencodercontrol;,
    pub one: uint16_t cmd_function51; //used as an internal,
    pub one: uint16_t cmd_function52; //used as an internal,
    pub one: uint16_t cmd_function53; //used as an internal,
    pub processi2cchanneltransaction;//Function: u16,
    pub one: uint16_t cmd_function55; //used as an internal,
    pub one: uint16_t cmd_function56; //used as an internal,
    pub one: uint16_t cmd_function57; //used as an internal,
    pub one: uint16_t cmd_function58; //used as an internal,
    pub one: uint16_t cmd_function59; //used as an internal,
    pub //Function: uint16_t computegpuclockparam;,
    pub one: uint16_t cmd_function61; //used as an internal,
    pub one: uint16_t cmd_function62; //used as an internal,
    pub function: uint16_t dynamicmemorysettings; //Function,
    pub function: uint16_t memorytraining; //Function,
    pub one: uint16_t cmd_function65; //used as an internal,
    pub one: uint16_t cmd_function66; //used as an internal,
    pub //Function: uint16_t setvoltage;,
    pub one: uint16_t cmd_function68; //used as an internal,
    pub //Function: uint16_t readefusevalue;,
    pub one: uint16_t cmd_function70; //used as an internal,
    pub one: uint16_t cmd_function71; //used as an internal,
    pub one: uint16_t cmd_function72; //used as an internal,
    pub one: uint16_t cmd_function73; //used as an internal,
    pub one: uint16_t cmd_function74; //used as an internal,
    pub one: uint16_t cmd_function75; //used as an internal,
    pub //Function: uint16_t dig1transmittercontrol;,
    pub one: uint16_t cmd_function77; //used as an internal,
    pub processauxchanneltransaction;//Function: u16,
    pub one: uint16_t cmd_function79; //used as an internal,
    pub //Function: uint16_t getvoltageinfo;,
}

//
// Structures used in every command function
//
// Common header for all hw functions.
// Every function pointed by _master_list_of_hw_function has this common header.
// And the pointer actually points to this header.
//
// ==============================sw data table portion======================================================================
//
// Structures used in data.mtb, each data table name is not given here since those data table could change from time to time
// The real name of each table is given when its data structure version is defined
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_master_list_of_data_tables_v2_1 {
    pub position!*/: *mut *mut uint16_t utilitypipeline; / Offest for the utility to get parser info,Don't change this,
    pub multimedia_info: u16,
    pub smc_dpm_info: u16,
    pub sw_datatable3: u16,
    pub /: *mut *mut uint16_t firmwareinfo; / Shared by various SW components,
    pub sw_datatable5: u16,
    pub /: *mut *mut uint16_t lcd_info; / Shared by various SW components,
    pub sw_datatable7: u16,
    pub smu_info: u16,
    pub sw_datatable9: u16,
    pub sw_datatable10: u16,
    pub /: *mut *mut uint16_t vram_usagebyfirmware; / Shared by various SW components,
    pub /: *mut *mut uint16_t gpio_pin_lut; / Shared by various SW components,
    pub sw_datatable13: u16,
    pub gfx_info: u16,
    pub /: *mut *mut uint16_t powerplayinfo; / Shared by various SW components,
    pub sw_datatable16: u16,
    pub sw_datatable17: u16,
    pub sw_datatable18: u16,
    pub sw_datatable19: u16,
    pub sw_datatable20: u16,
    pub sw_datatable21: u16,
    pub /: *mut *mut uint16_t displayobjectinfo; / Shared by various SW components,
    pub /: *mut *mut uint16_t indirectioaccess; / used as an internal one,
    pub /: *mut *mut uint16_t umc_info; / Shared by various SW components,
    pub sw_datatable25: u16,
    pub sw_datatable26: u16,
    pub /: *mut *mut uint16_t dce_info; / Shared by various SW components,
    pub /: *mut *mut uint16_t vram_info; / Shared by various SW components,
    pub sw_datatable29: u16,
    pub /: *mut *mut uint16_t integratedsysteminfo; / Shared by various SW components,
    pub /: *mut *mut uint16_t asic_profiling_info; / Shared by various SW components,
    pub /: *mut *mut uint16_t voltageobject_info; / shared by various SW components,
    pub sw_datatable33: u16,
    pub sw_datatable34: u16,
}

// atom_dtd_format.modemiscinfo definition
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_dtd_format_modemiscinfo {
    ATOM_HSYNC_POLARITY    = 0x0002,
    ATOM_VSYNC_POLARITY    = 0x0004,
    ATOM_H_REPLICATIONBY2  = 0x0010,
    ATOM_V_REPLICATIONBY2  = 0x0020,
    ATOM_INTERLACE         = 0x0080,
    ATOM_COMPOSITESYNC     = 0x0040,
}

// utilitypipeline
// when format_revision==1 && content_revision==1, then this an info table for atomworks to use during debug session, no structure is associated with it.
// the location of it can't change
//
// Total 32bit cap indication
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_cooling_solution_id {
    AIR_COOLING    = 0x00,
    LIQUID_COOLING = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_firmware_info_v3_2 {
    pub table_header: atom_common_table_header,
    pub firmware_revision: u32,
    pub bootup_sclk_in10khz: u32,
    pub bootup_mclk_in10khz: u32,
    pub atombios_firmware_capability: uint32_t firmware_capability; // enum,
    pub /: *mut *mut uint32_t main_call_parser_entry; / direct address of main parser call in VBIOS binary.,
    pub address: uint32_t bios_scratch_reg_startaddr; // 1st bios scratch register dword,
    pub bootup_vddc_mv: u16,
    pub bootup_vddci_mv: u16,
    pub bootup_mvddc_mv: u16,
    pub bootup_vddgfx_mv: u16,
    pub mem_module_id: u8,
    pub /: *mut *mut uint8_t coolingsolution_id; /0: Air cooling; 1: Liquid cooling ...,
    pub reserved1: [u8; 2],
    pub mc_baseaddr_high: u32,
    pub mc_baseaddr_low: u32,
    pub atom_board_i2c_feature_id_def: uint8_t board_i2c_feature_id; // enum of,
    pub gpio_id: uint8_t board_i2c_feature_gpio_id; // i2c id find in gpio_lut data table,
    pub board_i2c_feature_slave_addr: u8,
    pub reserved3: u8,
    pub bootup_mvddq_mv: u16,
    pub bootup_mvpp_mv: u16,
    pub zfbstartaddrin16mb: u32,
    pub reserved2: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_firmware_info_v3_4 {
    pub table_header: atom_common_table_header,
    pub firmware_revision: u32,
    pub bootup_sclk_in10khz: u32,
    pub bootup_mclk_in10khz: u32,
    pub atombios_firmware_capability: uint32_t firmware_capability; // enum,
    pub /: *mut *mut uint32_t main_call_parser_entry; / direct address of main parser call in VBIOS binary.,
    pub address: uint32_t bios_scratch_reg_startaddr; // 1st bios scratch register dword,
    pub bootup_vddc_mv: u16,
    pub bootup_vddci_mv: u16,
    pub bootup_mvddc_mv: u16,
    pub bootup_vddgfx_mv: u16,
    pub mem_module_id: u8,
    pub /: *mut *mut uint8_t coolingsolution_id; /0: Air cooling; 1: Liquid cooling ...,
    pub reserved1: [u8; 2],
    pub mc_baseaddr_high: u32,
    pub mc_baseaddr_low: u32,
    pub atom_board_i2c_feature_id_def: uint8_t board_i2c_feature_id; // enum of,
    pub gpio_id: uint8_t board_i2c_feature_gpio_id; // i2c id find in gpio_lut data table,
    pub board_i2c_feature_slave_addr: u8,
    pub ras_rom_i2c_slave_addr: u8,
    pub bootup_mvddq_mv: u16,
    pub bootup_mvpp_mv: u16,
    pub zfbstartaddrin16mb: u32,
    pub VBIOS: uint32_t pplib_pptable_id; // if pplib_pptable_id!=0, pplib get powerplay table inside driver instead of from,
    pub rail)*1000/(mvdd_output_from_svi2): *mut uint32_t mvdd_ratio; // mvdd_raio = (real mvdd in power,
    pub strap: uint16_t hw_bootup_vddgfx_mv; // hw default vddgfx voltage level decide by board,
    pub strap: uint16_t hw_bootup_vddc_mv; // hw default vddc voltage level decide by board,
    pub strap: uint16_t hw_bootup_mvddc_mv; // hw default mvddc voltage level decide by board,
    pub strap: uint16_t hw_bootup_vddci_mv; // hw default vddci voltage level decide by board,
    pub m-watt: uint32_t maco_pwrlimit_mw; // bomaco mode power limit in unit of,
    pub m-watt: uint32_t usb_pwrlimit_mw; // power limit when USB is enable in unit of,
    pub kb.: uint32_t fw_reserved_size_in_kb; // VBIOS reserved extra fw size in unit of,
    pub pspbl_init_done_reg_addr: u32,
    pub pspbl_init_done_value: u32,
    pub done: uint32_t pspbl_init_done_check_timeout; // time out in unit of us when polling pspbl init,
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_firmware_info_v3_5 {
    pub table_header: atom_common_table_header,
    pub firmware_revision: u32,
    pub bootup_clk_reserved: [u32; 2],
    pub atombios_firmware_capability: uint32_t firmware_capability; // enum,
    pub /: *mut *mut uint32_t fw_protect_region_size_in_kb; / FW allocate a write protect region at top of FB.,
    pub address: uint32_t bios_scratch_reg_startaddr; // 1st bios scratch register dword,
    pub bootup_voltage_reserved: [u32; 2],
    pub mem_module_id: u8,
    pub /: *mut *mut uint8_t coolingsolution_id; /0: Air cooling; 1: Liquid cooling ...,
    pub 2:HW_BLT_PCI_IO_MODE: uint8_t hw_blt_mode; //0:HW_BLT_DMA_PIO_MODE; 1:HW_BLT_LITE_SDMA_MODE;,
    pub reserved1: u8,
    pub mc_baseaddr_high: u32,
    pub mc_baseaddr_low: u32,
    pub atom_board_i2c_feature_id_def: uint8_t board_i2c_feature_id; // enum of,
    pub gpio_id: uint8_t board_i2c_feature_gpio_id; // i2c id find in gpio_lut data table,
    pub board_i2c_feature_slave_addr: u8,
    pub ras_rom_i2c_slave_addr: u8,
    pub bootup_voltage_reserved1: u32,
    pub zfb_reserved: u32,
// if pplib_pptable_id!=0, pplib get powerplay table inside driver instead of from VBIOS
    pub pplib_pptable_id: u32,
    pub hw_voltage_reserved: [u32; 3],
    pub m-watt: uint32_t maco_pwrlimit_mw; // bomaco mode power limit in unit of,
    pub m-watt: uint32_t usb_pwrlimit_mw; // power limit when USB is enable in unit of,
    pub kb.: uint32_t fw_reserved_size_in_kb; // VBIOS reserved extra fw size in unit of,
    pub pspbl_init_reserved: [u32; 3],
    pub size: uint32_t spi_rom_size; // GPU spi rom,
    pub support_dev_in_objinfo: u16,
    pub disp_phy_tunning_size: u16,
    pub reserved: [u32; 16],
}

//
// lcd_info_v2_1.panel_misc definition
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_lcd_info_panel_misc {
    ATOM_PANEL_MISC_FPDI            =0x0002,
}

// uceDPToLVDSRxId
//
// atom_gpio_pin_assignment.gpio_id definition
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_gpio_pin_assignment_gpio_id {
    I2C_HW_LANE_MUX        =0x0f, /* only valid when bit7=1 */
    I2C_HW_ENGINE_ID_MASK  =0x70, /* only valid when bit7=1 */
    I2C_HW_CAP             =0x80, /*only when the I2C_HW_CAP is set, the pin ID is assigned to an I2C pin pair, otherwise, it's an generic GPIO pin */

// gpio_id pre-define id for multiple usage
// GPIO use to control PCIE_VDDC in certain SLT board
    PCIE_VDDC_CONTROL_GPIO_PINID = 56,
// if PP_AC_DC_SWITCH_GPIO_PINID in Gpio_Pin_LutTable, AC/DC switching feature is enable
    PP_AC_DC_SWITCH_GPIO_PINID = 60,
// VDDC_REGULATOR_VRHOT_GPIO_PINID in Gpio_Pin_LutTable, VRHot feature is enable
    VDDC_VRHOT_GPIO_PINID = 61,
// if VDDC_PCC_GPIO_PINID in GPIO_LUTable, Peak Current Control feature is enabled
    VDDC_PCC_GPIO_PINID = 62,
// Only used on certain SLT/PA board to allow utility to cut Efuse.
    EFUSE_CUT_ENABLE_GPIO_PINID = 63,
// ucGPIO=DRAM_SELF_REFRESH_GPIO_PIND uses  for memory self refresh (ucGPIO=0, DRAM self-refresh; ucGPIO=
    DRAM_SELF_REFRESH_GPIO_PINID = 64,
// Thermal interrupt output->system thermal chip GPIO pin
    THERMAL_INT_OUTPUT_GPIO_PINID =65,
}

// the real number of this included in the structure is calculated by using the (whole structure size - the header size)/size of atom_gpio_pin_lut
//
// VBIOS/PRE-OS always reserve a FB region at the top of frame buffer. driver should not write
// access that region. driver can allocate their own reservation region as long as it does not
// overlap firwmare's reservation region.
// if (pre-NV1X) atom data table firmwareInfoTable version < 3.3:
// in this case, atom data table vram_usagebyfirmwareTable version always <= 2.1
// if VBIOS/UEFI GOP is posted:
// VBIOS/UEFIGOP update used_by_firmware_in_kb = total reserved size by VBIOS
// update start_address_in_kb = total_mem_size_in_kb - used_by_firmware_in_kb;
// ( total_mem_size_in_kb = reg(CONFIG_MEMSIZE)<<10)
// driver can allocate driver reservation region under firmware reservation,
// used_by_driver_in_kb = driver reservation size
// driver reservation start address =  (start_address_in_kb - used_by_driver_in_kb)
// Comment1[hchan]: There is only one reservation at the beginning of the FB reserved by
// host driver. Host driver would overwrite the table with the following
// used_by_firmware_in_kb = total reserved size for pf-vf info exchange and
// set SRIOV_MSG_SHARE_RESERVATION mask start_address_in_kb = 0
// else there is no VBIOS reservation region:
// driver must allocate driver reservation region at top of FB.
// driver set used_by_driver_in_kb = driver reservation size
// driver reservation start address =  (total_mem_size_in_kb - used_by_driver_in_kb)
// same as Comment1
// else (NV1X and after):
// if VBIOS/UEFI GOP is posted:
// VBIOS/UEFIGOP update:
// used_by_firmware_in_kb = atom_firmware_Info_v3_3.fw_reserved_size_in_kb;
// start_address_in_kb = total_mem_size_in_kb - used_by_firmware_in_kb;
// (total_mem_size_in_kb = reg(CONFIG_MEMSIZE)<<10)
// if vram_usagebyfirmwareTable version <= 2.1:
// driver can allocate driver reservation region under firmware reservation,
// driver set used_by_driver_in_kb = driver reservation size
// driver reservation start address = start_address_in_kb - used_by_driver_in_kb
// same as Comment1
// else driver can:
// allocate it reservation any place as long as it does overlap pre-OS FW reservation area
// set used_by_driver_region0_in_kb = driver reservation size
// set driver_region0_start_address_in_kb =  driver reservation region start address
// Comment2[hchan]: Host driver can set used_by_firmware_in_kb and start_address_in_kb to
// zero as the reservation for VF as it doesn’t exist.  And Host driver should also
// update atom_firmware_Info table to remove the same VBIOS reservation as well.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vram_usagebyfirmware_v2_2 {
    pub table_header: atom_common_table_header,
    pub fw_region_start_address_in_kb: u32,
    pub used_by_firmware_in_kb: u16,
    pub reserved: u16,
    pub driver_region0_start_address_in_kb: u32,
    pub used_by_driver_region0_in_kb: u32,
    pub reserved32: [u32; 7],
}

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_object_record_type_id {
    ATOM_I2C_RECORD_TYPE = 1,
    ATOM_HPD_INT_RECORD_TYPE = 2,
    ATOM_CONNECTOR_CAP_RECORD_TYPE = 3,
    ATOM_CONNECTOR_SPEED_UPTO = 4,
    ATOM_OBJECT_GPIO_CNTL_RECORD_TYPE = 9,
    ATOM_CONNECTOR_HPDPIN_LUT_RECORD_TYPE = 16,
    ATOM_CONNECTOR_AUXDDC_LUT_RECORD_TYPE = 17,
    ATOM_ENCODER_CAP_RECORD_TYPE = 20,
    ATOM_BRACKET_LAYOUT_RECORD_TYPE = 21,
    ATOM_CONNECTOR_FORCED_TMDS_CAP_RECORD_TYPE = 22,
    ATOM_DISP_CONNECTOR_CAPS_RECORD_TYPE = 23,
    ATOM_BRACKET_LAYOUT_V2_RECORD_TYPE = 25,
    ATOM_RECORD_END_TYPE = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_connector_caps_record {
    pub ATOM_CONN_CAP_RECORD_TYPE: record_header; //record_type =,
    pub Not: uint16_t connector_caps; //01b if internal display is checked; 10b if internal BL is checked; 0 of,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_connector_speed_record {
    pub ATOM_CONN_SPEED_UPTO: record_header; //record_type =,
    pub @8.1Ghz.: uint32_t connector_max_speed; // connector Max speed attribute, it sets 8100 in Mhz when DP connector,
    pub reserved: u16,
}

// Bit maps for ATOM_ENCODER_CAP_RECORD.usEncoderCap
// The following generic object gpio pin control record type will replace JTAG_RECORD/FPGA_CONTROL_RECORD/DVI_EXT_INPUT_RECORD above gradually
// Definitions for GPIO pin state
// For GPIO_PIN_TYPE_OUTPUT the following is defined
// Indexes to GPIO array in GLSync record
// GLSync record is for Frame Lock/Gen Lock feature.
// override TMDS capability on this connector when it operate in TMDS mode.  usMaxTmdsClkRate = max TMDS Clock in Mhz/2.5
// define ATOM_CONNECTOR_LAYOUT_INFO.ucConnectorType to describe the display connector size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_bracket_layout_record_v2 {
    pub ATOM_BRACKET_LAYOUT_RECORD_TYPE: record_header; //record_type =,
    pub mm: uint8_t bracketlen; //Bracket Length in,
    pub mm: uint8_t bracketwidth; //Bracket Width in,
    pub numbering: uint8_t conn_num; //Connector,
    pub Mini): uint8_t mini_type; //Mini Type (0 = Normal; 1 =,
    pub reserved1: u8,
    pub reserved2: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_connector_layout_info_mini_type_def {
    MINI_TYPE_NORMAL = 0,
    MINI_TYPE_MINI = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_display_device_tag_def {
    ATOM_DISPLAY_LCD1_SUPPORT            = 0x0002, //an embedded display is either an LVDS or eDP signal type of display
    ATOM_DISPLAY_LCD2_SUPPORT            = 0x0020, //second edp device tag 0x0020 for backward compatibility
    ATOM_DISPLAY_DFP1_SUPPORT            = 0x0008,
    ATOM_DISPLAY_DFP2_SUPPORT            = 0x0080,
    ATOM_DISPLAY_DFP3_SUPPORT            = 0x0200,
    ATOM_DISPLAY_DFP4_SUPPORT            = 0x0400,
    ATOM_DISPLAY_DFP5_SUPPORT            = 0x0800,
    ATOM_DISPLAY_DFP6_SUPPORT            = 0x0040,
    ATOM_DISPLAY_DFPx_SUPPORT            = 0x0ec8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_display_object_path_v3 {
    pub ID: uint16_t display_objid; //Connector Object ID or Misc Object,
    pub disp_recordoffset: u16,
    pub encoder: uint16_t encoderobjid; //first encoder closer to the connector, could be either an external or internal,
    pub 0: uint16_t reserved1; //only on USBC case, otherwise always =,
    pub 0: uint16_t reserved2; //reserved and always =,
    pub 0: uint16_t reserved3; //reserved and always =,
// a supported device vector, each display path starts with this.the paths are enumerated in the way of priority,
// a path appears first
    pub device_tag: u16,
    pub 0: uint16_t reserved4; //reserved and always =,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_object_info_table_v1_5 {
    pub table_header: atom_common_table_header,
    pub supporteddevices: u16,
    pub number_of_path: u8,
    pub reserved: u8,
// the real number of this included in the structure is calculated by using the
// (whole structure size - the header size- number_of_path)/size of atom_display_object_path
    pub display_path: [atom_display_object_path_v3; ],
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_display_controller_info_v4_4 {
    pub table_header: atom_common_table_header,
    pub display_caps: u32,
    pub bootup_dispclk_10khz: u32,
    pub dce_refclk_10khz: u16,
    pub i2c_engine_refclk_10khz: u16,
    pub 0.001%: uint16_t dvi_ss_percentage; // in unit of,
    pub dvi_ss_rate_10hz: u16,
    pub 0.001%: uint16_t hdmi_ss_percentage; // in unit of,
    pub hdmi_ss_rate_10hz: u16,
    pub 0.001%: uint16_t dp_ss_percentage; // in unit of,
    pub dp_ss_rate_10hz: u16,
    pub atom_spread_spectrum_mode: uint8_t dvi_ss_mode; // enum of,
    pub atom_spread_spectrum_mode: uint8_t hdmi_ss_mode; // enum of,
    pub atom_spread_spectrum_mode: uint8_t dp_ss_mode; // enum of,
    pub ss_reserved: u8,
    pub available: uint8_t dfp_hardcode_mode_num; // DFP hardcode mode number defined in StandardVESA_TimingTable when EDID is not,
    pub available: uint8_t dfp_hardcode_refreshrate;// DFP hardcode mode refreshrate defined in StandardVESA_TimingTable when EDID is not,
    pub avablable: uint8_t vga_hardcode_mode_num; // VGA hardcode mode number defined in StandardVESA_TimingTable when EDID is not,
    pub avablable: uint8_t vga_hardcode_refreshrate;// VGA hardcode mode number defined in StandardVESA_TimingTable when EDID is not,
    pub dpphy_refclk_10khz: u16,
    pub hw_chip_id: u16,
    pub dcnip_min_ver: u8,
    pub dcnip_max_ver: u8,
    pub max_disp_pipe_num: u8,
    pub max_vbios_active_disp_pipum: u8,
    pub max_ppll_num: u8,
    pub max_disp_phy_num: u8,
    pub max_aux_pairs: u8,
    pub remotedisplayconfig: u8,
    pub dispclk_pll_vco_freq: u32,
    pub dp_ref_clk_freq: u32,
    pub us): uint32_t max_mclk_chg_lat; // Worst case blackout duration for a memory clock frequency (p-state) change, units of 100s of ns (0.1,
    pub (0.1us): uint32_t max_sr_exit_lat; // Worst case memory self refresh exit time, units of 100ns of ns,
    pub (0.1us): uint32_t max_sr_enter_exit_lat; // Worst case memory self refresh entry followed by immediate exit time, units of 100ns of ns,
    pub atom_dc_golden_table_vxx: uint16_t dc_golden_table_offset; // point of struct of,
    pub dc_golden_table_ver: u16,
    pub reserved3: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dce_info_caps_def {
// only for VBIOS
    DCE_INFO_CAPS_FORCE_DISPDEV_CONNECTED = 0x02,
// only for VBIOS
    DCE_INFO_CAPS_DISABLE_DFP_DP_HBR2 = 0x04,
// only for VBIOS
    DCE_INFO_CAPS_ENABLE_INTERLAC_TIMING = 0x08,
// only for VBIOS
    DCE_INFO_CAPS_LTTPR_SUPPORT_ENABLE = 0x20,
    DCE_INFO_CAPS_VBIOS_LTTPR_TRANSPARENT_ENABLE = 0x40,
}

// DFP hardcode mode number defined in StandardVESA_TimingTable when EDID is not available
// DFP hardcode mode refreshrate defined in StandardVESA_TimingTable when EDID is not available
// VGA hardcode mode number defined in StandardVESA_TimingTable when EDID is not avablable
// Worst case blackout duration for a memory clock frequency (p-state) change, units of 100s of ns (0.1 us)
// Worst case memory self refresh exit time, units of 100ns of ns (0.1us)
// Worst case memory self refresh entry followed by immediate exit time, units of 100ns of ns (0.1us)
//
// usCaps
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ext_display_path_cap_def {
    EXT_DISPLAY_PATH_CAPS__EXT_CHIP_MASK =		0x007E,
    AMD_EXT_DISPLAY_PATH_CAPS__EXT_CHIP_MASK =		0x007E,
    AMD_EXT_DISPLAY_PATH_CAPS__DP_FIXED_VS_EN =		(0x01 << 1),
    AMD_EXT_DISPLAY_PATH_CAPS__HDMI20_PI3EQX1204 =	(0x02 << 1),
    AMD_EXT_DISPLAY_PATH_CAPS__DP_EARLY_8B10B_TPS2 =	(0x03 << 1),
    AMD_EXT_DISPLAY_PATH_CAPS__HDMI20_TISN65DP159RSBT =	(0x04 << 1),
    AMD_EXT_DISPLAY_PATH_CAPS__HDMI20_PARADE_PS175 =	(0x06 << 1),
    EXT_DISPLAY_PATH_CAPS__DP_FIXED_VS_EN =		(0x07 << 1),
    EXT_DISPLAY_PATH_CAPS__HDMI20_PI3EQX1204 =		(0x08 << 1),   //PI redriver chip
    EXT_DISPLAY_PATH_CAPS__HDMI20_TISN65DP159RSBT =	(0x09 << 1),   //TI retimer chip
    EXT_DISPLAY_PATH_CAPS__AMD_INTERNAL =		(0x0a << 1),   //AMD internal customer chip placeholder
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_14nm_dpphy_dp_setting {
    pub atom_dp_vs_preemph_def: uint8_t dp_vs_pemph_level; //enum of,
    pub [15:8]deemph_gen1_nom: uint16_t margindeemph; //COMMON_MAR_DEEMPH_NOM[7:0]tx_margin_nom,
    pub //COMMON_SELDEEMPH60[31:24]deemph_6db_4: uint8_t deemph_6db_4;,
    pub [23:22]tx_binary_ron_code_offset: uint8_t boostadj; //CMD_BUS_GLOBAL_FOR_TX_LANE0 [19:16]tx_boost_adj [20]tx_boost_en,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_14nm_dpphy_dp_tuningset {
    pub phyf: uint8_t phy_sel; // bit vector of phy, bit0= phya, bit1=phyb, ....bit5 =,
    pub version: u8,
    pub atom_14nm_dpphy_dp_tuningset: uint16_t table_size; // size of,
    pub reserved: u16,
    pub dptuning: [atom_14nm_dpphy_dp_setting; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_14nm_dig_transmitter_info_header_v4_0 {
    pub table_header: atom_common_table_header,
    pub PCIEPhyTMDSHDMIMacroSettingsTbl: uint16_t pcie_phy_tmds_hdmi_macro_settings_offset; // offset of,
    pub UniphyVSEmphLookUpTbl: uint16_t uniphy_vs_emph_lookup_table_offset; // offset of,
    pub UniphyXbarSettingsTbl: uint16_t uniphy_xbar_settings_table_offset; // offset of,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_DCN_dpphy_dp_setting {
    pub atom_dp_vs_preemph_def: uint8_t dp_vs_pemph_level; //enum of,
    pub RDPCSTX_PHY_FUSE0/1/2/3[5:0](EQ_MAIN): uint8_t tx_eq_main; // map to,
    pub RDPCSTX_PHY_FUSE0/1/2/3[11:6](EQ_PRE): uint8_t tx_eq_pre; // map to,
    pub RDPCSTX_PHY_FUSE0/1/2/3[17:12](EQ_POST): uint8_t tx_eq_post; // map to,
    pub RDPCSTX_PHY_CNTL0.RDPCS_PHY_TX_VBOOST_LVL: uint8_t tx_vboost_lvl; // tx_vboost_lvl, map to,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_DCN_dpphy_dp_tuningset {
    pub phyf: uint8_t phy_sel; // bit vector of phy, bit0= phya, bit1=phyb, ....bit5 =,
    pub version: u8,
    pub atom_14nm_dpphy_dp_setting: uint16_t table_size; // size of,
    pub reserved: u16,
    pub dptunings: [atom_DCN_dpphy_dp_setting; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_i2c_reg_info {
    pub ucI2cRegIndex: u8,
    pub ucI2cRegVal: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_hdmi_retimer_redriver_set {
    pub HdmiSlvAddr: u8,
    pub HdmiRegNum: u8,
    pub Hdmi6GRegNum: u8,
    pub use: atom_i2c_reg_info HdmiRegSetting[9]; //For non 6G Hz,
    pub use.: atom_i2c_reg_info Hdmi6GhzRegSetting[3]; //For 6G Hz,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_n6_display_phy_tuning_set {
    pub display_signal_type: u8,
    pub phy_sel: u8,
    pub preset_level: u8,
    pub reserved1: u8,
    pub reserved2: u32,
    pub speed_upto: u32,
    pub tx_vboost_level: u8,
    pub tx_vreg_v2i: u8,
    pub tx_vregdrv_byp: u8,
    pub tx_term_cntl: u8,
    pub tx_peak_level: u8,
    pub tx_slew_en: u8,
    pub tx_eq_pre: u8,
    pub tx_eq_main: u8,
    pub tx_eq_post: u8,
    pub tx_en_inv_pre: u8,
    pub tx_en_inv_post: u8,
    pub reserved3: u8,
    pub reserved4: u32,
    pub reserved5: u32,
    pub reserved6: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_display_phy_tuning_info {
    pub table_header: atom_common_table_header,
    pub disp_phy_tuning: [atom_n6_display_phy_tuning_set; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uma_carveout_option {
    pub 64bits: char optionName[29]; //max length of string is 28chars + '\0'. Current design is for "minimum", "Medium", "High". This makes entire struct size,
    pub setting: uint8_t memoryCarvedGb; //memory carved out with,
    pub system: uint8_t memoryRemainingGb; //memory remaining on,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _flags {
    pub 1: uint8_t Auto :,
    pub 1: uint8_t Custom :,
    pub 6: uint8_t Reserved :,
    pub flags: },
    pub all8: u8,
    pub uma_carveout_option_flags: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_integrated_system_info_v2_3 {
    pub table_header: atom_common_table_header,
    pub atom_system_vbiosmisc_def: uint32_t vbios_misc; // enum of,
    pub atom_system_gpucapinf_def: uint32_t gpucapinfo; // enum of,
    pub system_config: u32,
    pub cpucapinfo: u32,
    pub 1%: uint16_t gpuclk_ss_percentage; // unit of 0.001%, 1000 mean,
    pub gpuclk_ss_type: u16,
    pub atom_sysinfo_dpphy_override_def: uint16_t dpphy_override; // bit vector, enum of,
    pub indication.: uint8_t memorytype; // enum of atom_dmi_t17_mem_type_def, APU memory type,
    pub channels: uint8_t umachannelnumber; // number of memory,
    pub htc_hyst_limit: u8,
    pub htc_tmp_limit: u8,
    pub dp_ss_control: uint8_t reserved1; //,
    pub gpu_package_id: u8,
    pub edp1_info: edp_info_table,
    pub edp2_info: edp_info_table,
    pub cpuid: u32,
    pub vram_bit_width: u32,
    pub reserved2: [u32; 6],
    pub extdispconninfo: atom_external_display_connection_info,
    pub UMACarveoutVersion: u8,
    pub UMACarveoutIndexMax: u8,
    pub UMACarveoutTypeDefault: u8,
    pub UMACarveoutIndexDefault: u8,
    pub Custom: uint8_t UMACarveoutType; //Auto or,
    pub UMACarveoutIndex: u8,
    pub UMASizeControlOption: [uma_carveout_option; 20],
    pub reserved3: [u8; 110],
}

// system_config
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_system_vbiosmisc_def {
    INTEGRATED_SYSTEM_INFO__GET_EDID_CALLBACK_FUNC_SUPPORT = 0x01,
}

// gpucapinfo
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_system_gpucapinf_def {
    SYS_INFO_GPUCAPS__ENABLE_DFS_BYPASS  = 0x10,
}

// dpphy_override
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_sysinfo_dpphy_override_def {
    ATOM_ENABLE_DVI_TUNINGSET   = 0x01,
    ATOM_ENABLE_HDMI_TUNINGSET  = 0x02,
    ATOM_ENABLE_HDMI6G_TUNINGSET  = 0x04,
    ATOM_ENABLE_DP_TUNINGSET  = 0x08,
    ATOM_ENABLE_DP_HBR3_TUNINGSET  = 0x10,
}

// lvds_misc
// memorytype  DMI Type 17 offset 12h - Memory Type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_dmi_t17_mem_type_def {
    OtherMemType = 0x01,                                  ///< Assign 01 to Other
    UnknownMemType,                                       ///< Assign 02 to Unknown
    DramMemType,                                          ///< Assign 03 to DRAM
    EdramMemType,                                         ///< Assign 04 to EDRAM
    VramMemType,                                          ///< Assign 05 to VRAM
    SramMemType,                                          ///< Assign 06 to SRAM
    RamMemType,                                           ///< Assign 07 to RAM
    RomMemType,                                           ///< Assign 08 to ROM
    FlashMemType,                                         ///< Assign 09 to Flash
    EepromMemType,                                        ///< Assign 10 to EEPROM
    FepromMemType,                                        ///< Assign 11 to FEPROM
    EpromMemType,                                         ///< Assign 12 to EPROM
    CdramMemType,                                         ///< Assign 13 to CDRAM
    ThreeDramMemType,                                     ///< Assign 14 to 3DRAM
    SdramMemType,                                         ///< Assign 15 to SDRAM
    SgramMemType,                                         ///< Assign 16 to SGRAM
    RdramMemType,                                         ///< Assign 17 to RDRAM
    DdrMemType,                                           ///< Assign 18 to DDR
    Ddr2MemType,                                          ///< Assign 19 to DDR2
    Ddr2FbdimmMemType,                                    ///< Assign 20 to DDR2 FB-DIMM
    Ddr3MemType = 0x18,                                   ///< Assign 24 to DDR3
    Fbd2MemType,                                          ///< Assign 25 to FBD2
    Ddr4MemType,                                          ///< Assign 26 to DDR4
    LpDdrMemType,                                         ///< Assign 27 to LPDDR
    LpDdr2MemType,                                        ///< Assign 28 to LPDDR2
    LpDdr3MemType,                                        ///< Assign 29 to LPDDR3
    LpDdr4MemType,                                        ///< Assign 30 to LPDDR4
    GDdr6MemType,                                         ///< Assign 31 to GDDR6
    HbmMemType,                                           ///< Assign 32 to HBM
    Hbm2MemType,                                          ///< Assign 33 to HBM2
    Ddr5MemType,                                          ///< Assign 34 to DDR5
    LpDdr5MemType,                                        ///< Assign 35 to LPDDR5
    LpDdr5xMemType,                                       ///< Assign 36 to LPDDR5x
}

// this Table is used starting from NL/AM, used by SBIOS and pass the IntegratedSystemInfoTable/PowerPlayInfoTable/SystemCameraInfoTable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_gfx_info_v2_3 {
    pub table_header: atom_common_table_header,
    pub gfxip_min_ver: u8,
    pub gfxip_max_ver: u8,
    pub max_shader_engines: u8,
    pub max_tile_pipes: u8,
    pub max_cu_per_sh: u8,
    pub max_sh_per_se: u8,
    pub max_backends_per_se: u8,
    pub max_texture_channel_caches: u8,
    pub regaddr_cp_dma_src_addr: u32,
    pub regaddr_cp_dma_src_addr_hi: u32,
    pub regaddr_cp_dma_dst_addr: u32,
    pub regaddr_cp_dma_dst_addr_hi: u32,
    pub regaddr_cp_dma_command: u32,
    pub regaddr_cp_status: u32,
    pub regaddr_rlc_gpu_clock_32: u32,
    pub rlc_gpu_timer_refclk: u32,
    pub active_cu_per_sh: u8,
    pub active_rb_per_se: u8,
    pub gcgoldenoffset: u16,
    pub rm21_sram_vmin_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_gfx_info_v2_7 {
    pub table_header: atom_common_table_header,
    pub gfxip_min_ver: u8,
    pub gfxip_max_ver: u8,
    pub max_shader_engines: u8,
    pub reserved: u8,
    pub max_cu_per_sh: u8,
    pub max_sh_per_se: u8,
    pub max_backends_per_se: u8,
    pub max_texture_channel_caches: u8,
    pub regaddr_cp_dma_src_addr: u32,
    pub regaddr_cp_dma_src_addr_hi: u32,
    pub regaddr_cp_dma_dst_addr: u32,
    pub regaddr_cp_dma_dst_addr_hi: u32,
    pub regaddr_cp_dma_command: u32,
    pub regaddr_cp_status: u32,
    pub regaddr_rlc_gpu_clock_32: u32,
    pub rlc_gpu_timer_refclk: u32,
    pub active_cu_per_sh: u8,
    pub active_rb_per_se: u8,
    pub gcgoldenoffset: u16,
    pub gc_num_gprs: u16,
    pub gc_gsprim_buff_depth: u16,
    pub gc_parameter_cache_depth: u16,
    pub gc_wave_size: u16,
    pub gc_max_waves_per_simd: u16,
    pub gc_lds_size: u16,
    pub gc_num_max_gs_thds: u8,
    pub gc_gs_table_depth: u8,
    pub gc_double_offchip_lds_buffer: u8,
    pub gc_max_scratch_slots_per_cu: u8,
    pub sram_rm_fuses_val: u32,
    pub sram_custom_rm_fuses_val: u32,
    pub cut_cu: u8,
    pub active_cu_total: u8,
    pub cu_reserved: [u8; 2],
    pub gc_config: u32,
    pub inactive_cu_per_se: [u8; 8],
    pub reserved2: [u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_gfx_info_v3_0 {
    pub table_header: atom_common_table_header,
    pub gfxip_min_ver: u8,
    pub gfxip_max_ver: u8,
    pub max_shader_engines: u8,
    pub max_tile_pipes: u8,
    pub max_cu_per_sh: u8,
    pub max_sh_per_se: u8,
    pub max_backends_per_se: u8,
    pub max_texture_channel_caches: u8,
    pub regaddr_lsdma_queue0_rb_rptr: u32,
    pub regaddr_lsdma_queue0_rb_rptr_hi: u32,
    pub regaddr_lsdma_queue0_rb_wptr: u32,
    pub regaddr_lsdma_queue0_rb_wptr_hi: u32,
    pub regaddr_lsdma_command: u32,
    pub regaddr_lsdma_status: u32,
    pub regaddr_golden_tsc_count_lower: u32,
    pub golden_tsc_count_lower_refclk: u32,
    pub active_wgp_per_se: u8,
    pub active_rb_per_se: u8,
    pub active_se: u8,
    pub reserved1: u8,
    pub sram_rm_fuses_val: u32,
    pub sram_custom_rm_fuses_val: u32,
    pub inactive_sa_mask: u32,
    pub gc_config: u32,
    pub inactive_wgp: [u8; 16],
    pub inactive_rb: [u8; 16],
    pub gdfll_as_wait_ctrl_val: u32,
    pub gdfll_as_step_ctrl_val: u32,
    pub reserved: [u32; 8],
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_smu_info_v3_2 {
    pub table_header: atom_common_table_header,
    pub smuip_min_ver: u8,
    pub smuip_max_ver: u8,
    pub smu_rsd1: u8,
    pub gpuclk_ss_mode: u8,
    pub sclk_ss_percentage: u16,
    pub sclk_ss_rate_10hz: u16,
    pub 0.001%: uint16_t gpuclk_ss_percentage; // in unit of,
    pub gpuclk_ss_rate_10hz: u16,
    pub core_refclk_10khz: u32,
    pub invalid: uint8_t ac_dc_gpio_bit; // GPIO bit shift in SMU_GPIOPAD_A configured for AC/DC switching, =0xff means,
    pub switching: uint8_t ac_dc_polarity; // GPIO polarity for AC/DC,
    pub invalid: uint8_t vr0hot_gpio_bit; // GPIO bit shift in SMU_GPIOPAD_A configured for VR0 HOT event, =0xff means,
    pub event: uint8_t vr0hot_polarity; // GPIO polarity for VR0 HOT,
    pub invalid: uint8_t vr1hot_gpio_bit; // GPIO bit shift in SMU_GPIOPAD_A configured for VR1 HOT event , =0xff means,
    pub event: uint8_t vr1hot_polarity; // GPIO polarity for VR1 HOT,
    pub invalid: uint8_t fw_ctf_gpio_bit; // GPIO bit shift in SMU_GPIOPAD_A configured for CTF, =0xff means,
    pub CTF: uint8_t fw_ctf_polarity; // GPIO polarity for,
    pub invalid: uint8_t pcc_gpio_bit; // GPIO bit shift in SMU_GPIOPAD_A configured for PCC, =0xff means,
    pub CTF: uint8_t pcc_gpio_polarity; // GPIO polarity for,
    pub smugoldenoffset: u16,
    pub gpupll_vco_freq_10khz: u32,
    pub bootup_smnclk_10khz: u32,
    pub bootup_socclk_10khz: u32,
    pub bootup_mp0clk_10khz: u32,
    pub bootup_mp1clk_10khz: u32,
    pub bootup_lclk_10khz: u32,
    pub bootup_dcefclk_10khz: u32,
    pub ctf_threshold_override_value: u32,
    pub reserved: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_smu_info_v3_3 {
    pub table_header: atom_common_table_header,
    pub smuip_min_ver: u8,
    pub smuip_max_ver: u8,
    pub waflclk_ss_mode: u8,
    pub gpuclk_ss_mode: u8,
    pub sclk_ss_percentage: u16,
    pub sclk_ss_rate_10hz: u16,
    pub 0.001%: uint16_t gpuclk_ss_percentage; // in unit of,
    pub gpuclk_ss_rate_10hz: u16,
    pub core_refclk_10khz: u32,
    pub invalid: uint8_t ac_dc_gpio_bit; // GPIO bit shift in SMU_GPIOPAD_A configured for AC/DC switching, =0xff means,
    pub switching: uint8_t ac_dc_polarity; // GPIO polarity for AC/DC,
    pub invalid: uint8_t vr0hot_gpio_bit; // GPIO bit shift in SMU_GPIOPAD_A configured for VR0 HOT event, =0xff means,
    pub event: uint8_t vr0hot_polarity; // GPIO polarity for VR0 HOT,
    pub invalid: uint8_t vr1hot_gpio_bit; // GPIO bit shift in SMU_GPIOPAD_A configured for VR1 HOT event , =0xff means,
    pub event: uint8_t vr1hot_polarity; // GPIO polarity for VR1 HOT,
    pub invalid: uint8_t fw_ctf_gpio_bit; // GPIO bit shift in SMU_GPIOPAD_A configured for CTF, =0xff means,
    pub CTF: uint8_t fw_ctf_polarity; // GPIO polarity for,
    pub invalid: uint8_t pcc_gpio_bit; // GPIO bit shift in SMU_GPIOPAD_A configured for PCC, =0xff means,
    pub CTF: uint8_t pcc_gpio_polarity; // GPIO polarity for,
    pub smugoldenoffset: u16,
    pub gpupll_vco_freq_10khz: u32,
    pub bootup_smnclk_10khz: u32,
    pub bootup_socclk_10khz: u32,
    pub bootup_mp0clk_10khz: u32,
    pub bootup_mp1clk_10khz: u32,
    pub bootup_lclk_10khz: u32,
    pub bootup_dcefclk_10khz: u32,
    pub ctf_threshold_override_value: u32,
    pub syspll3_0_vco_freq_10khz: u32,
    pub syspll3_1_vco_freq_10khz: u32,
    pub bootup_fclk_10khz: u32,
    pub bootup_waflclk_10khz: u32,
    pub smu_info_caps: u32,
    pub 0.001%: uint16_t waflclk_ss_percentage; // in unit of,
    pub smuinitoffset: u16,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_smu_info_v4_0 {
    pub table_header: atom_common_table_header,
    pub bootup_gfxclk_bypass_10khz: u32,
    pub bootup_usrclk_10khz: u32,
    pub bootup_csrclk_10khz: u32,
    pub core_refclk_10khz: u32,
    pub syspll1_vco_freq_10khz: u32,
    pub syspll2_vco_freq_10khz: u32,
    pub pcc_gpio_bit: u8,
    pub pcc_gpio_polarity: u8,
    pub bootup_vddusr_mv: u16,
    pub syspll0_vco_freq_10khz: u32,
    pub bootup_smnclk_10khz: u32,
    pub bootup_socclk_10khz: u32,
    pub bootup_mp0clk_10khz: u32,
    pub bootup_mp1clk_10khz: u32,
    pub bootup_lclk_10khz: u32,
    pub bootup_dcefclk_10khz: u32,
    pub ctf_threshold_override_value: u32,
    pub syspll3_vco_freq_10khz: u32,
    pub mm_syspll_vco_freq_10khz: u32,
    pub bootup_fclk_10khz: u32,
    pub bootup_waflclk_10khz: u32,
    pub smu_info_caps: u32,
    pub waflclk_ss_percentage: u16,
    pub smuinitoffset: u16,
    pub bootup_dprefclk_10khz: u32,
    pub bootup_usbclk_10khz: u32,
    pub smb_slave_address: u32,
    pub cg_fdo_ctrl0_val: u32,
    pub cg_fdo_ctrl1_val: u32,
    pub cg_fdo_ctrl2_val: u32,
    pub gdfll_as_wait_ctrl_val: u32,
    pub gdfll_as_step_ctrl_val: u32,
    pub bootup_dtbclk_10khz: u32,
    pub fclk_syspll_refclk_10khz: u32,
    pub smusvi_svc0_val: u32,
    pub smusvi_svc1_val: u32,
    pub smusvi_svd0_val: u32,
    pub smusvi_svd1_val: u32,
    pub smusvi_svt0_val: u32,
    pub smusvi_svt1_val: u32,
    pub cg_tach_ctrl_val: u32,
    pub cg_pump_ctrl1_val: u32,
    pub cg_pump_tach_ctrl_val: u32,
    pub thm_ctf_delay_val: u32,
    pub thm_thermal_int_ctrl_val: u32,
    pub thm_tmon_config_val: u32,
    pub smbus_timing_cntrl0_val: u32,
    pub smbus_timing_cntrl1_val: u32,
    pub smbus_timing_cntrl2_val: u32,
    pub pwr_disp_timer_global_control_val: u32,
    pub bootup_mpioclk_10khz: u32,
    pub bootup_dclk0_10khz: u32,
    pub bootup_vclk0_10khz: u32,
    pub bootup_dclk1_10khz: u32,
    pub bootup_vclk1_10khz: u32,
    pub bootup_baco400clk_10khz: u32,
    pub bootup_baco1200clk_bypass_10khz: u32,
    pub bootup_baco700clk_bypass_10khz: u32,
    pub reserved: [u32; 16],
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smudpm_i2ccontrollerconfig_t {
    pub enabled: u32,
    pub slaveaddress: u32,
    pub controllerport: u32,
    pub controllername: u32,
    pub thermalthrottler: u32,
    pub i2cprotocol: u32,
    pub i2cspeed: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smudpm_v4_5_i2ccontrollername_e {
    SMC_V4_5_I2C_CONTROLLER_NAME_VR_GFX = 0,
    SMC_V4_5_I2C_CONTROLLER_NAME_VR_SOC,
    SMC_V4_5_I2C_CONTROLLER_NAME_VR_VDDCI,
    SMC_V4_5_I2C_CONTROLLER_NAME_VR_MVDD,
    SMC_V4_5_I2C_CONTROLLER_NAME_LIQUID0,
    SMC_V4_5_I2C_CONTROLLER_NAME_LIQUID1,
    SMC_V4_5_I2C_CONTROLLER_NAME_PLX,
    SMC_V4_5_I2C_CONTROLLER_NAME_SPARE,
    SMC_V4_5_I2C_CONTROLLER_NAME_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smudpm_v4_5_i2ccontrollerthrottler_e {
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_TYPE_NONE = 0,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_VR_GFX,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_VR_SOC,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_VR_VDDCI,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_VR_MVDD,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_LIQUID0,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_LIQUID1,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_PLX,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smudpm_v4_5_i2ccontrollerprotocol_e {
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_VR_0,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_VR_1,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_TMP_0,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_TMP_1,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_SPARE_0,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_SPARE_1,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_COUNT,
}

// SECTION: BOARD PARAMETERS
// I2C Control
// SVI2 Board Parameters
// Telemetry Settings
// GPIO Settings
// LED Display Settings
// GFXCLK PLL Spread Spectrum
// GFXCLK DFLL Spread Spectrum
// UCLK Spread Spectrum
// SOCCLK Spread Spectrum
// Total board power
// Mvdd Svi2 Div Ratio Setting
// section: board parameters
// telemetry settings
// gpio settings
// gfxclk pll spread spectrum
// uclk spread spectrum
// fclk spread spectrum
// gfxclk fll spread spectrum
// i2c controller structure
// memory section
// total board power
// section: xgmi training
// reserved
// SECTION: BOARD PARAMETERS
// I2C Control
// SVI2 Board Parameters
// Telemetry Settings
// GPIO Settings
// LED Display Settings
// GFXCLK PLL Spread Spectrum
// GFXCLK DFLL Spread Spectrum
// UCLK Spread Spectrum
// SOCCLK Spread Spectrum
// Total board power
// Mvdd Svi2 Div Ratio Setting
// GPIO pins for I2C communications with 2nd controller for Input Telemetry Sequence
// Additional LED Display Settings
// Power Limit Scalars
// SECTION: Gaming Clocks
// uint32_t     GamingClk[6];
// SECTION: I2C Control
// SECTION: SVI2 Board Parameters
// SECTION: Telemetry Settings
// SECTION: GPIO Settings
// LED Display Settings
// SECTION: Clock Spread Spectrum
// GFXCLK PLL Spread Spectrum
// GFXCLK DFLL Spread Spectrum
// UCLK Spread Spectrum
// FCLK Spread Spectrum
// Section: Memory Config
// Section: Total Board Power
// SECTION: XGMI Training
// SECTION: Board Reserved
// SECTION: BOARD PARAMETERS
// Telemetry Settings
// Platform input telemetry voltage coefficient
// GPIO Settings
// UCLK Spread Spectrum
// FCLK Spread Spectrum
// I2C Controller Structure
// GPIO pins for I2C communications with 2nd controller for Input Telemetry Sequence
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_asic_profiling_info_v4_2 {
    pub table_header: atom_common_table_header,
    pub maxvddc: u32,
    pub minvddc: u32,
    pub avfs_meannsigma_acontant0: u32,
    pub avfs_meannsigma_acontant1: u32,
    pub avfs_meannsigma_acontant2: u32,
    pub avfs_meannsigma_dc_tol_sigma: u16,
    pub avfs_meannsigma_platform_mean: u16,
    pub avfs_meannsigma_platform_sigma: u16,
    pub gb_vdroop_table_cksoff_a0: u32,
    pub gb_vdroop_table_cksoff_a1: u32,
    pub gb_vdroop_table_cksoff_a2: u32,
    pub gb_vdroop_table_ckson_a0: u32,
    pub gb_vdroop_table_ckson_a1: u32,
    pub gb_vdroop_table_ckson_a2: u32,
    pub avfsgb_fuse_table_cksoff_m1: u32,
    pub avfsgb_fuse_table_cksoff_m2: u32,
    pub avfsgb_fuse_table_cksoff_b: u32,
    pub avfsgb_fuse_table_ckson_m1: u32,
    pub avfsgb_fuse_table_ckson_m2: u32,
    pub avfsgb_fuse_table_ckson_b: u32,
    pub max_voltage_0_25mv: u16,
    pub enable_gb_vdroop_table_cksoff: u8,
    pub enable_gb_vdroop_table_ckson: u8,
    pub enable_gb_fuse_table_cksoff: u8,
    pub enable_gb_fuse_table_ckson: u8,
    pub psm_age_comfactor: u16,
    pub enable_apply_avfs_cksoff_voltage: u8,
    pub reserved: u8,
    pub dispclk2gfxclk_a: u32,
    pub dispclk2gfxclk_b: u32,
    pub dispclk2gfxclk_c: u32,
    pub pixclk2gfxclk_a: u32,
    pub pixclk2gfxclk_b: u32,
    pub pixclk2gfxclk_c: u32,
    pub dcefclk2gfxclk_a: u32,
    pub dcefclk2gfxclk_b: u32,
    pub dcefclk2gfxclk_c: u32,
    pub phyclk2gfxclk_a: u32,
    pub phyclk2gfxclk_b: u32,
    pub phyclk2gfxclk_c: u32,
    pub acg_gb_vdroop_table_a0: u32,
    pub acg_gb_vdroop_table_a1: u32,
    pub acg_gb_vdroop_table_a2: u32,
    pub acg_avfsgb_fuse_table_m1: u32,
    pub acg_avfsgb_fuse_table_m2: u32,
    pub acg_avfsgb_fuse_table_b: u32,
    pub enable_acg_gb_vdroop_table: u8,
    pub enable_acg_gb_fuse_table: u8,
    pub acg_dispclk2gfxclk_a: u32,
    pub acg_dispclk2gfxclk_b: u32,
    pub acg_dispclk2gfxclk_c: u32,
    pub acg_pixclk2gfxclk_a: u32,
    pub acg_pixclk2gfxclk_b: u32,
    pub acg_pixclk2gfxclk_c: u32,
    pub acg_dcefclk2gfxclk_a: u32,
    pub acg_dcefclk2gfxclk_b: u32,
    pub acg_dcefclk2gfxclk_c: u32,
    pub acg_phyclk2gfxclk_a: u32,
    pub acg_phyclk2gfxclk_b: u32,
    pub acg_phyclk2gfxclk_c: u32,
}

//
// umc_info.umc_config
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_umc_config_def {
    UMC_CONFIG__ENABLE_1KB_INTERLEAVE_MODE  =   0x00000001,
    UMC_CONFIG__DEFAULT_MEM_ECC_ENABLE      =   0x00000002,
    UMC_CONFIG__ENABLE_HBM_LANE_REPAIR      =   0x00000004,
    UMC_CONFIG__ENABLE_BANK_HARVESTING      =   0x00000008,
    UMC_CONFIG__ENABLE_PHY_REINIT           =   0x00000010,
    UMC_CONFIG__DISABLE_UCODE_CHKSTATUS     =   0x00000020,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_umc_config1_def {
    UMC_CONFIG1__ENABLE_PSTATE_PHASE_STORE_TRAIN = 0x00000001,
    UMC_CONFIG1__ENABLE_AUTO_FRAMING = 0x00000002,
    UMC_CONFIG1__ENABLE_RESTORE_BIST_DATA = 0x00000004,
    UMC_CONFIG1__DISABLE_STROBE_MODE = 0x00000008,
    UMC_CONFIG1__DEBUG_DATA_PARITY_EN = 0x00000010,
    UMC_CONFIG1__ENABLE_ECC_CAPABLE = 0x00010000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_umc_info_v4_0 {
    pub table_header: atom_common_table_header,
    pub ucode_reserved: [u32; 5],
    pub umcip_min_ver: u8,
    pub umcip_max_ver: u8,
    pub vram_type: u8,
    pub umc_config: u8,
    pub mem_refclk_10khz: u32,
    pub clk_reserved: [u32; 4],
    pub golden_reserved: u32,
    pub umc_config1: u32,
    pub reserved: [u32; 2],
    pub channel_num: u8,
    pub channel_width: u8,
    pub channel_reserve: [u8; 2],
    pub umc_info_reserved: [u8; 16],
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_vram_module_v9 {
// Design Specific Values
    pub zeros: uint32_t memory_size; // Total memory size in unit of MB for CONFIG_MEMSIZE,
    pub not: uint32_t channel_enable; // bit vector, each bit indicate specific channel enable or,
    pub defined: uint32_t max_mem_clk; // max memory clock of this memory in unit of 10kHz, =0 means it is not,
    pub reserved: [u16; 3],
    pub mem_voltage: uint16_t mem_voltage; //,
    pub atom_vram_module_v9: uint16_t vram_module_size; // Size of,
    pub ID: uint8_t ext_memory_id; // Current memory module,
    pub atom_dgpu_vram_type: uint8_t memory_type; // enum of,
    pub module: uint8_t channel_num; // Number of mem. channels supported in this,
    pub CHANNEL_16BIT/CHANNEL_32BIT/CHANNEL_64BIT: uint8_t channel_width; //,
    pub _32Mx16: uint8_t density; // _8Mx32, _16Mx32, _16Mx16,,
    pub per.: uint8_t tunningset_id; // MC phy registers set,
    pub code: uint8_t vender_rev_id; // [7:4] Revision, [3:0] Vendor,
    pub 10=32ms,11=64ms): uint8_t refreshrate; // [1:0]=RefreshFactor (00=8ms, 01=16ms,,
    pub hbm_ven_rev_id: uint8_t hbm_ven_rev_id; //,
    pub reserved: uint8_t vram_rsd2; //,
    pub '0'.: char dram_pnstring[20]; // part number end with,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_vram_info_header_v2_3 {
    pub table_header: atom_common_table_header,
    pub setting: uint16_t mem_adjust_tbloffset; // offset of atom_umc_init_reg_block structure for memory vendor specific UMC adjust,
    pub setting: uint16_t mem_clk_patch_tbloffset; // offset of atom_umc_init_reg_block structure for memory clock specific UMC,
    pub Settings: uint16_t mc_adjust_pertile_tbloffset; // offset of atom_umc_init_reg_block structure for Per Byte Offset Preset,
    pub set: uint16_t mc_phyinit_tbloffset; // offset of atom_umc_init_reg_block structure for MC phy init,
    pub now: uint16_t dram_data_remap_tbloffset; // reserved for,
    pub tmrs: uint16_t tmrs_seq_offset; // offset of HBM,
    pub init: uint16_t post_ucode_init_offset; // offset of atom_umc_init_reg_block structure for MC phy init after MC uCode complete umc,
    pub vram_rsd2: u16,
    pub module: uint8_t vram_module_num; // indicate number of VRAM,
    pub umcip_min_ver: u8,
    pub umcip_max_ver: u8,
    pub usMcAdjustPerTileTblOffset: uint8_t mc_phy_tile_num; // indicate the MCD tile number which use in DramDataRemapTbl and,
    pub ucNumOfVRAMModule: atom_vram_module_v9 vram_module[16]; // just for allocation, real number of blocks is in,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_vram_module_v3_0 {
    pub density: u8,
    pub tunningset_id: u8,
    pub ext_memory_id: u8,
    pub dram_vendor_id: u8,
    pub dram_info_offset: u16,
    pub mem_tuning_offset: u16,
    pub tmrs_seq_offset: u16,
    pub reserved1: u16,
    pub dram_size_per_ch: u32,
    pub reserved: [u32; 3],
    pub dram_pnstring: [c_char; 40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_vram_info_header_v3_0 {
    pub table_header: atom_common_table_header,
    pub mem_tuning_table_offset: u16,
    pub dram_info_table_offset: u16,
    pub tmrs_table_offset: u16,
    pub mc_init_table_offset: u16,
    pub dram_data_remap_table_offset: u16,
    pub umc_emuinittable_offset: u16,
    pub reserved_sub_table_offset: [u16; 2],
    pub vram_module_num: u8,
    pub umcip_min_ver: u8,
    pub umcip_max_ver: u8,
    pub mc_phy_tile_num: u8,
    pub memory_type: u8,
    pub channel_num: u8,
    pub channel_width: u8,
    pub reserved1: u8,
    pub channel_enable: u32,
    pub channel1_enable: u32,
    pub feature_enable: u32,
    pub feature1_enable: u32,
    pub hardcode_mem_size: u32,
    pub reserved4: [u32; 4],
    pub vram_module: [atom_vram_module_v3_0; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_umc_register_addr_info {
    pub umc_register_addr:24: u32,
    pub umc_reg_type_ind:1: u32,
    pub umc_reg_rsvd:7: u32,
}

// atom_umc_register_addr_info.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_umc_register_addr_info_flag {
    b3ATOM_UMC_REG_ADD_INFO_INDIRECT_ACCESS  =0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_umc_reg_setting_id_config {
    pub memclockrange:24: u32,
    pub mem_blk_id:8: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_umc_reg_setting_data_block {
    pub block_id: atom_umc_reg_setting_id_config_access,
    pub u32umc_reg_data: [u32; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_umc_init_reg_block {
    pub umc_reg_num: u16,
    pub reserved: u16,
    pub umc_reg_num: atom_umc_register_addr_info_access umc_reg_list[1]; //for allocation purpose, the real number come from,
    pub umc_reg_setting_list: [atom_umc_reg_setting_data_block; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_vram_module_v10 {
// Design Specific Values
    pub zeros: uint32_t memory_size; // Total memory size in unit of MB for CONFIG_MEMSIZE,
    pub not: uint32_t channel_enable; // bit vector, each bit indicate specific channel enable or,
    pub defined: uint32_t max_mem_clk; // max memory clock of this memory in unit of 10kHz, =0 means it is not,
    pub reserved: [u16; 3],
    pub mem_voltage: uint16_t mem_voltage; //,
    pub atom_vram_module_v9: uint16_t vram_module_size; // Size of,
    pub ID: uint8_t ext_memory_id; // Current memory module,
    pub atom_dgpu_vram_type: uint8_t memory_type; // enum of,
    pub module: uint8_t channel_num; // Number of mem. channels supported in this,
    pub CHANNEL_16BIT/CHANNEL_32BIT/CHANNEL_64BIT: uint8_t channel_width; //,
    pub _32Mx16: uint8_t density; // _8Mx32, _16Mx32, _16Mx16,,
    pub per: uint8_t tunningset_id; // MC phy registers set,
    pub code: uint8_t vender_rev_id; // [7:4] Revision, [3:0] Vendor,
    pub 10=32ms,11=64ms): uint8_t refreshrate; // [1:0]=RefreshFactor (00=8ms, 01=16ms,,
    pub enable: uint8_t vram_flags; // bit0= bankgroup,
    pub reserved: uint8_t vram_rsd2; //,
    pub value: uint16_t gddr6_mr10; // gddr6 mode register10,
    pub value: uint16_t gddr6_mr1; // gddr6 mode register1,
    pub value: uint16_t gddr6_mr2; // gddr6 mode register2,
    pub value: uint16_t gddr6_mr7; // gddr6 mode register7,
    pub '0': char dram_pnstring[20]; // part number end with,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_vram_info_header_v2_4 {
    pub table_header: atom_common_table_header,
    pub setting: uint16_t mem_adjust_tbloffset; // offset of atom_umc_init_reg_block structure for memory vendor specific UMC adjust,
    pub setting: uint16_t mem_clk_patch_tbloffset; // offset of atom_umc_init_reg_block structure for memory clock specific UMC,
    pub Settings: uint16_t mc_adjust_pertile_tbloffset; // offset of atom_umc_init_reg_block structure for Per Byte Offset Preset,
    pub set: uint16_t mc_phyinit_tbloffset; // offset of atom_umc_init_reg_block structure for MC phy init,
    pub now: uint16_t dram_data_remap_tbloffset; // reserved for,
    pub reserved: uint16_t reserved; // offset of,
    pub init: uint16_t post_ucode_init_offset; // offset of atom_umc_init_reg_block structure for MC phy init after MC uCode complete umc,
    pub vram_rsd2: u16,
    pub module: uint8_t vram_module_num; // indicate number of VRAM,
    pub umcip_min_ver: u8,
    pub umcip_max_ver: u8,
    pub usMcAdjustPerTileTblOffset: uint8_t mc_phy_tile_num; // indicate the MCD tile number which use in DramDataRemapTbl and,
    pub ucNumOfVRAMModule: atom_vram_module_v10 vram_module[16]; // just for allocation, real number of blocks is in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_vram_module_v11 {
// Design Specific Values
    pub zeros: uint32_t memory_size; // Total memory size in unit of MB for CONFIG_MEMSIZE,
    pub not: uint32_t channel_enable; // bit vector, each bit indicate specific channel enable or,
    pub mem_voltage: uint16_t mem_voltage; //,
    pub atom_vram_module_v9: uint16_t vram_module_size; // Size of,
    pub ID: uint8_t ext_memory_id; // Current memory module,
    pub atom_dgpu_vram_type: uint8_t memory_type; // enum of,
    pub module: uint8_t channel_num; // Number of mem. channels supported in this,
    pub CHANNEL_16BIT/CHANNEL_32BIT/CHANNEL_64BIT: uint8_t channel_width; //,
    pub _32Mx16: uint8_t density; // _8Mx32, _16Mx32, _16Mx16,,
    pub per.: uint8_t tunningset_id; // MC phy registers set,
    pub reserved: uint16_t reserved[4]; //,
    pub code: uint8_t vender_rev_id; // [7:4] Revision, [3:0] Vendor,
    pub 10=32ms,11=64ms): uint8_t refreshrate; // [1:0]=RefreshFactor (00=8ms, 01=16ms,,
    pub enable: uint8_t vram_flags; // bit0= bankgroup,
    pub reserved: uint8_t vram_rsd2; //,
    pub value: uint16_t gddr6_mr10; // gddr6 mode register10,
    pub value: uint16_t gddr6_mr0; // gddr6 mode register0,
    pub value: uint16_t gddr6_mr1; // gddr6 mode register1,
    pub value: uint16_t gddr6_mr2; // gddr6 mode register2,
    pub value: uint16_t gddr6_mr4; // gddr6 mode register4,
    pub value: uint16_t gddr6_mr7; // gddr6 mode register7,
    pub value: uint16_t gddr6_mr8; // gddr6 mode register8,
    pub '0'.: char dram_pnstring[40]; // part number end with,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_gddr6_ac_timing_v2_5 {
    pub u32umc_id_access: u32,
    pub RL: u8,
    pub WL: u8,
    pub tRAS: u8,
    pub tRC: u8,
    pub tREFI: u16,
    pub tRFC: u8,
    pub tRFCpb: u8,
    pub tRREFD: u8,
    pub tRCDRD: u8,
    pub tRCDWR: u8,
    pub tRP: u8,
    pub tRRDS: u8,
    pub tRRDL: u8,
    pub tWR: u8,
    pub tWTRS: u8,
    pub tWTRL: u8,
    pub tFAW: u8,
    pub tCCDS: u8,
    pub tCCDL: u8,
    pub tCRCRL: u8,
    pub tCRCWL: u8,
    pub tCKE: u8,
    pub tCKSRE: u8,
    pub tCKSRX: u8,
    pub tRTPS: u8,
    pub tRTPL: u8,
    pub tMRD: u8,
    pub tMOD: u8,
    pub tXS: u8,
    pub tXHP: u8,
    pub tXSMRS: u8,
    pub tXSH: u32,
    pub tPD: u8,
    pub tXP: u8,
    pub tCPDED: u8,
    pub tACTPDE: u8,
    pub tPREPDE: u8,
    pub tREFPDE: u8,
    pub tMRSPDEN: u8,
    pub tRDSRE: u8,
    pub tWRSRE: u8,
    pub tPPD: u8,
    pub tCCDMW: u8,
    pub tWTRTR: u8,
    pub tLTLTR: u8,
    pub tREFTR: u8,
    pub VNDR: u8,
    pub reserved: [u8; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_gddr6_bit_byte_remap {
    pub //mmUMC_DPHY_ByteRemap: uint32_t dphy_byteremap;,
    pub //mmUMC_DPHY_BitRemap0: uint32_t dphy_bitremap0;,
    pub //mmUMC_DPHY_BitRemap1: uint32_t dphy_bitremap1;,
    pub //mmUMC_DPHY_BitRemap2: uint32_t dphy_bitremap2;,
    pub //mmUMC_APHY_BitRemap0: uint32_t aphy_bitremap0;,
    pub //mmUMC_APHY_BitRemap1: uint32_t aphy_bitremap1;,
    pub //mmUMC_PHY_DRAM: uint32_t phy_dram;,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_gddr6_dram_data_remap {
    pub table_size: u32,
    pub //UMC_PHY_PHYINTF_CNTL.INV_CK: uint8_t phyintf_ck_inverted[8];,
    pub bit_byte_remap: [atom_gddr6_bit_byte_remap; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_vram_info_header_v2_5 {
    pub table_header: atom_common_table_header,
    pub settings: uint16_t mem_adjust_tbloffset; // offset of atom_umc_init_reg_block structure for memory vendor specific UMC adjust,
    pub settings: uint16_t gddr6_ac_timing_offset; // offset of atom_gddr6_ac_timing_v2_5 structure for memory clock specific UMC,
    pub Settings: uint16_t mc_adjust_pertile_tbloffset; // offset of atom_umc_init_reg_block structure for Per Byte Offset Preset,
    pub set: uint16_t mc_phyinit_tbloffset; // offset of atom_umc_init_reg_block structure for MC phy init,
    pub mapping: uint16_t dram_data_remap_tbloffset; // offset of atom_gddr6_dram_data_remap array to indicate DRAM data lane to GPU,
    pub reserved: uint16_t reserved; // offset of,
    pub init: uint16_t post_ucode_init_offset; // offset of atom_umc_init_reg_block structure for MC phy init after MC uCode complete umc,
    pub settings: uint16_t strobe_mode_patch_tbloffset; // offset of atom_umc_init_reg_block structure for Strobe Mode memory clock specific UMC,
    pub module: uint8_t vram_module_num; // indicate number of VRAM,
    pub umcip_min_ver: u8,
    pub umcip_max_ver: u8,
    pub usMcAdjustPerTileTblOffset: uint8_t mc_phy_tile_num; // indicate the MCD tile number which use in DramDataRemapTbl and,
    pub ucNumOfVRAMModule: atom_vram_module_v11 vram_module[16]; // just for allocation, real number of blocks is in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_vram_info_header_v2_6 {
    pub table_header: atom_common_table_header,
    pub mem_adjust_tbloffset: u16,
    pub mem_clk_patch_tbloffset: u16,
    pub mc_adjust_pertile_tbloffset: u16,
    pub mc_phyinit_tbloffset: u16,
    pub dram_data_remap_tbloffset: u16,
    pub tmrs_seq_offset: u16,
    pub post_ucode_init_offset: u16,
    pub vram_rsd2: u16,
    pub vram_module_num: u8,
    pub umcip_min_ver: u8,
    pub umcip_max_ver: u8,
    pub mc_phy_tile_num: u8,
    pub vram_module: [atom_vram_module_v9; 16],
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_voltage_object_header_v4 {
    pub atom_voltage_type: uint8_t voltage_type; //enum,
    pub atom_voltage_object_mode: uint8_t voltage_mode; //enum,
    pub Object: uint16_t object_size; //Size of,
}

// atom_voltage_object_header_v4.voltage_mode
// ATOM_I2C_VOLTAGE_OBJECT_V3.ucVoltageControlFlag
#[repr(C)]
#[derive(Copy, Clone)]
pub union atom_voltage_object_v4 {
    pub gpio_voltage_obj: atom_gpio_voltage_object_v4,
    pub i2c_voltage_obj: atom_i2c_voltage_object_v4,
    pub svid2_voltage_obj: atom_svid2_voltage_object_v4,
    pub merged_voltage_obj: atom_merged_voltage_object_v4,
}

//
// set_voltage_parameters_v2_1.voltagemode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_set_voltage_command {
    ATOM_SET_VOLTAGE  = 0,
    ATOM_INIT_VOLTAGE_REGULATOR = 3,
    ATOM_SET_VOLTAGE_PHASE = 4,
    ATOM_GET_LEAKAGE_ID    = 8,
}

//
// ATOM_COMPUTE_CLOCK_FREQ.ulComputeClockFlag
//
// ReadEfuseValue input/output parameter
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu11_syspll_id {
    SMU11_SYSPLL0_ID            = 0,
    SMU11_SYSPLL1_0_ID          = 1,
    SMU11_SYSPLL1_1_ID          = 2,
    SMU11_SYSPLL1_2_ID          = 3,
    SMU11_SYSPLL2_ID            = 4,
    SMU11_SYSPLL3_0_ID          = 5,
    SMU11_SYSPLL3_1_ID          = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu11_syspll0_clock_id {
    SMU11_SYSPLL0_ECLK_ID     = 0,       //	ECLK
    SMU11_SYSPLL0_SOCCLK_ID   = 1,       //	SOCCLK
    SMU11_SYSPLL0_MP0CLK_ID   = 2,       //	MP0CLK
    SMU11_SYSPLL0_DCLK_ID     = 3,       //	DCLK
    SMU11_SYSPLL0_VCLK_ID     = 4,       //	VCLK
    SMU11_SYSPLL0_DCEFCLK_ID  = 5,       //	DCEFCLK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu11_syspll1_0_clock_id {
    SMU11_SYSPLL1_0_UCLKA_ID   = 0,       // UCLK_a
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu11_syspll1_1_clock_id {
    SMU11_SYSPLL1_0_UCLKB_ID   = 0,       // UCLK_b
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu11_syspll1_2_clock_id {
    SMU11_SYSPLL1_0_FCLK_ID   = 0,        // FCLK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu11_syspll2_clock_id {
    SMU11_SYSPLL2_GFXCLK_ID   = 0,        // GFXCLK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu11_syspll3_0_clock_id {
    SMU11_SYSPLL3_0_WAFCLK_ID = 0,       //	WAFCLK
    SMU11_SYSPLL3_0_DISPCLK_ID = 1,      //	DISPCLK
    SMU11_SYSPLL3_0_DPREFCLK_ID = 2,     //	DPREFCLK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu11_syspll3_1_clock_id {
    SMU11_SYSPLL3_1_MP1CLK_ID = 0,       //	MP1CLK
    SMU11_SYSPLL3_1_SMNCLK_ID = 1,       //	SMNCLK
    SMU11_SYSPLL3_1_LCLK_ID = 2,         //	LCLK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu12_syspll_id {
    SMU12_SYSPLL0_ID          = 0,
    SMU12_SYSPLL1_ID          = 1,
    SMU12_SYSPLL2_ID          = 2,
    SMU12_SYSPLL3_0_ID        = 3,
    SMU12_SYSPLL3_1_ID        = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu12_syspll0_clock_id {
    SMU12_SYSPLL0_SMNCLK_ID   = 0,			//	SOCCLK
    SMU12_SYSPLL0_SOCCLK_ID   = 1,			//	SOCCLK
    SMU12_SYSPLL0_MP0CLK_ID   = 2,			//	MP0CLK
    SMU12_SYSPLL0_MP1CLK_ID   = 3,			//	MP1CLK
    SMU12_SYSPLL0_MP2CLK_ID   = 4,			//	MP2CLK
    SMU12_SYSPLL0_VCLK_ID     = 5,			//	VCLK
    SMU12_SYSPLL0_LCLK_ID     = 6,			//	LCLK
    SMU12_SYSPLL0_DCLK_ID     = 7,			//	DCLK
    SMU12_SYSPLL0_ACLK_ID     = 8,			//	ACLK
    SMU12_SYSPLL0_ISPCLK_ID   = 9,			//	ISPCLK
    SMU12_SYSPLL0_SHUBCLK_ID  = 10,			//	SHUBCLK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu12_syspll1_clock_id {
    SMU12_SYSPLL1_DISPCLK_ID  = 0,      //	DISPCLK
    SMU12_SYSPLL1_DPPCLK_ID   = 1,      //	DPPCLK
    SMU12_SYSPLL1_DPREFCLK_ID = 2,      //	DPREFCLK
    SMU12_SYSPLL1_DCFCLK_ID   = 3,      //	DCFCLK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu12_syspll2_clock_id {
    SMU12_SYSPLL2_Pre_GFXCLK_ID = 0,   // Pre_GFXCLK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu12_syspll3_0_clock_id {
    SMU12_SYSPLL3_0_FCLK_ID = 0,      //	FCLK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atom_smu12_syspll3_1_clock_id {
    SMU12_SYSPLL3_1_UMCCLK_ID = 0,    //	UMCCLK
}

//
// when command = COMPUTE_MEMORY_PLL_PARAM or ADJUST_MC_SETTING_PARAM
// when command = COMPUTE_ENGINE_PLL_PARAM
//
// indicate which graphic encoder will be used.
// ucMiscInfo
// deep_color_ratio
//
// SetDCEClock input parameter for DCE11.2( ELM and BF ) and above
// ucDCEClkType
// ucDCEClkFlag when ucDCEClkType == DPREFCLK
// ucDCEClkFlag when ucDCEClkType == PIXCLK
//
// Structures used by BlankCRTC
//
// Structures used by enablecrtc
//
// Structure used by EnableDispPowerGating
//
// Structure used in setcrtc_usingdtdtiming
//
// Structures used by processi2cchanneltransaction
//
// ucFlag
// status
//
// Structures used by processauxchanneltransaction
//
// Structures used by selectcrtc_source
//
// Structures used by digxencodercontrol
//
// ucAction:
// define ucPanelMode
// ucDigId
//
// ucAction
// digfe_sel
// ucHPDSel
// ucDPLaneSet
//
// Structures used by ExternalEncoderControl V2.4
//
// ucAction
// ucConfig
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_acpi_description_header {
    pub signature: u32,
    pub //Length: uint32_t tableLength;,
    pub revision: u8,
    pub checksum: u8,
    pub oemId: [u8; 6],
    pub OemTableId: uint8_t oemTableId[8]; //UINT64,
    pub oemRevision: u32,
    pub creatorId: u32,
    pub creatorRevision: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uefi_acpi_vfct {
    pub sheader: amd_acpi_description_header,
    pub //0x24: uint8_t tableUUID[16];,
    pub structure.: uint32_t vbiosimageoffset; //0x34. Offset to the first GOP_VBIOS_CONTENT block from the beginning of the,
    pub structure.: uint32_t lib1Imageoffset; //0x38. Offset to the first GOP_LIB1_CONTENT block from the beginning of the,
    pub //0x3C: uint32_t reserved[4];,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfct_image_header {
    pub //0x4C: uint32_t pcibus;,
    pub //0x50: uint32_t pcidevice;,
    pub //0x54: uint32_t pcifunction;,
    pub //0x58: uint16_t vendorid;,
    pub //0x5A: uint16_t deviceid;,
    pub //0x5C: uint16_t ssvid;,
    pub //0x5E: uint16_t ssid;,
    pub //0x60: uint32_t revision;,
    pub //0x64: uint32_t imagelength;,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gop_vbios_content {
    pub vbiosheader: vfct_image_header,
    pub vbioscontent: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gop_lib1_content {
    pub lib1header: vfct_image_header,
    pub lib1content: [u8; 1],
}

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scratch_register_def {
    ATOM_DEVICE_CONNECT_INFO_DEF      = 0,
    ATOM_BL_BRI_LEVEL_INFO_DEF        = 2,
    ATOM_ACTIVE_INFO_DEF              = 3,
    ATOM_LCD_INFO_DEF                 = 4,
    ATOM_DEVICE_REQ_INFO_DEF          = 5,
    ATOM_ACC_CHANGE_INFO_DEF          = 6,
    ATOM_PRE_OS_MODE_INFO_DEF         = 7,
    ATOM_PRE_OS_ASSERTION_DEF      = 8,    //For GOP to record a 32bit assertion code, this is enabled by default in prodution GOP drivers.
    ATOM_INTERNAL_TIMER_INFO_DEF      = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scratch_device_connect_info_bit_def {
    ATOM_DISPLAY_LCD1_CONNECT           =0x0002,
    ATOM_DISPLAY_DFP1_CONNECT           =0x0008,
    ATOM_DISPLAY_DFP2_CONNECT           =0x0080,
    ATOM_DISPLAY_DFP3_CONNECT           =0x0200,
    ATOM_DISPLAY_DFP4_CONNECT           =0x0400,
    ATOM_DISPLAY_DFP5_CONNECT           =0x0800,
    ATOM_DISPLAY_DFP6_CONNECT           =0x0040,
    ATOM_DISPLAY_DFPx_CONNECT           =0x0ec8,
    ATOM_CONNECT_INFO_DEVICE_MASK       =0x0fff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scratch_bl_bri_level_info_bit_def {
    ATOM_CURRENT_BL_LEVEL_SHIFT         =0x8,
    ATOM_CURRENT_BL_LEVEL_MASK          =0x0000ff00,
    ATOM_DEVICE_DPMS_STATE              =0x00010000,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scratch_active_info_bits_def {
    ATOM_DISPLAY_LCD1_ACTIVE            =0x0002,
    ATOM_DISPLAY_DFP1_ACTIVE            =0x0008,
    ATOM_DISPLAY_DFP2_ACTIVE            =0x0080,
    ATOM_DISPLAY_DFP3_ACTIVE            =0x0200,
    ATOM_DISPLAY_DFP4_ACTIVE            =0x0400,
    ATOM_DISPLAY_DFP5_ACTIVE            =0x0800,
    ATOM_DISPLAY_DFP6_ACTIVE            =0x0040,
    ATOM_ACTIVE_INFO_DEVICE_MASK        =0x0fff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scratch_device_req_info_bits_def {
    ATOM_DISPLAY_LCD1_REQ               =0x0002,
    ATOM_DISPLAY_DFP1_REQ               =0x0008,
    ATOM_DISPLAY_DFP2_REQ               =0x0080,
    ATOM_DISPLAY_DFP3_REQ               =0x0200,
    ATOM_DISPLAY_DFP4_REQ               =0x0400,
    ATOM_DISPLAY_DFP5_REQ               =0x0800,
    ATOM_DISPLAY_DFP6_REQ               =0x0040,
    ATOM_REQ_INFO_DEVICE_MASK           =0x0fff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scratch_acc_change_info_bitshift_def {
    ATOM_ACC_CHANGE_ACC_MODE_SHIFT    =4,
    ATOM_ACC_CHANGE_LID_STATUS_SHIFT  =6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scratch_acc_change_info_bits_def {
    ATOM_ACC_CHANGE_ACC_MODE          =0x00000010,
    ATOM_ACC_CHANGE_LID_STATUS        =0x00000040,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scratch_pre_os_mode_info_bits_def {
    ATOM_PRE_OS_MODE_MASK             =0x00000003,
    ATOM_PRE_OS_MODE_VGA              =0x00000000,
    ATOM_PRE_OS_MODE_VESA             =0x00000001,
    ATOM_PRE_OS_MODE_GOP              =0x00000002,
    ATOM_PRE_OS_MODE_PIXEL_DEPTH      =0x0000000C,
    ATOM_PRE_OS_MODE_PIXEL_FORMAT_MASK=0x000000F0,
    ATOM_PRE_OS_MODE_8BIT_PAL_EN      =0x00000100,
    ATOM_ASIC_INIT_COMPLETE           =0x00000200,
    ATOM_PRE_OS_MODE_NUMBER_MASK      =0xFFFF0000,

}

//

