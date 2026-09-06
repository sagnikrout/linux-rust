//! Automatically rewritten from C to Rust
//! Source: drivers/video/logo/pnmtologo.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Convert a logo in ASCII PNM format to C source suitable for inclusion in
// the Linux kernel
//
// (C) Copyright 2001-2003 by Geert Uytterhoeven <geert@linux-m68k.org>
//

    static const char *programname;
    static const char *filename;
    static const char *logoname = "linux_logo";
    static const char *outputname;
    static FILE *out;

    static const char *logo_types[LINUX_LOGO_GRAY256+1] = {
    [LINUX_LOGO_MONO] = "LINUX_LOGO_MONO",
    [LINUX_LOGO_VGA16] = "LINUX_LOGO_VGA16",
    [LINUX_LOGO_CLUT224] = "LINUX_LOGO_CLUT224",
    [LINUX_LOGO_GRAY256] = "LINUX_LOGO_GRAY256"
    };
pub const MAX_LINUX_LOGO_COLORS: c_int = 224;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct color {
    pub red: c_uchar,
    pub green: c_uchar,
    pub blue: c_uchar,
}

    static const struct color clut_vga16[16] = {
    { 0x00, 0x00, 0x00 },
    { 0x00, 0x00, 0xaa },
    { 0x00, 0xaa, 0x00 },
    { 0x00, 0xaa, 0xaa },
    { 0xaa, 0x00, 0x00 },
    { 0xaa, 0x00, 0xaa },
    { 0xaa, 0x55, 0x00 },
    { 0xaa, 0xaa, 0xaa },
    { 0x55, 0x55, 0x55 },
    { 0x55, 0x55, 0xff },
    { 0x55, 0xff, 0x55 },
    { 0x55, 0xff, 0xff },
    { 0xff, 0x55, 0x55 },
    { 0xff, 0x55, 0xff },
    { 0xff, 0xff, 0x55 },
    { 0xff, 0xff, 0xff },
    };
    let mut logo_type: static int = LINUX_LOGO_CLUT224;
    static unsigned int logo_width;
    static unsigned int logo_height;
    static struct color **logo_data;
    static struct color logo_clut[MAX_LINUX_LOGO_COLORS];
    static unsigned int logo_clutsize;
    let mut is_plain_pbm: static int = 0;
#[no_mangle]
unsafe extern "C" fn die(fmt: *const c_char, ...) {
    static void die(const char *fmt, ...)
    __attribute__((noreturn)) __attribute((format (printf, 1, 2)));
    static void usage(void) __attribute((noreturn));
#[no_mangle]
unsafe extern "C" fn get_number(fp: *mut FILE) -> c_uint {
    static unsigned int get_number(FILE *fp)
    {
    int c, val;
// Skip leading whitespace
    do {
    c = fgetc(fp);
    if (c == EOF)
    die("%s: end of file\n", filename);
    if (c == '#') {
// Ignore comments 'till end of line
    do {
    c = fgetc(fp);
    if (c == EOF)
    die("%s: end of file\n", filename);
    } while (c != '\n');
    }
    } while (isspace(c));
// Parse decimal number
    val = 0;
    while (isdigit(c)) {
    val = 10*val+c-'0';
// some PBM are 'broken'; GiMP for example exports a PBM without space
// between the digits. This is Ok cause we know a PBM can only have a '1'
// or a '0' for the digit.
//
    if (is_plain_pbm)
    break;
    c = fgetc(fp);
    if (c == EOF)
    die("%s: end of file\n", filename);
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn get_number255(fp: *mut FILE, maxval: c_uint) -> c_uint {
    static unsigned int get_number255(FILE *fp, unsigned int maxval)
    {
    let mut val: c_uint = get_number(fp);
    return (255*val+maxval/2)/maxval;
    }
#[no_mangle]
unsafe extern "C" fn read_image() {
    static void read_image(void)
    {
    FILE *fp;
    unsigned int i, j;
    int magic;
    unsigned int maxval;
// open image file
    fp = fopen(filename, "r");
    if (!fp)
    die("Cannot open file %s: %s\n", filename, strerror(errno));
// check file type and read file header
    magic = fgetc(fp);
    if (magic != 'P')
    die("%s is not a PNM file\n", filename);
    magic = fgetc(fp);
    switch (magic) {
    case '1':
    case '2':
    case '3':
// Plain PBM/PGM/PPM
    break;
    case '4':
    case '5':
    case '6':
// Binary PBM/PGM/PPM
    die("%s: Binary PNM is not supported\n"
    "Use pnmnoraw(1) to convert it to ASCII PNM\n", filename);
    default:
    die("%s is not a PNM file\n", filename);
    }
    logo_width = get_number(fp);
    logo_height = get_number(fp);
// allocate image data
    logo_data = (struct color **)malloc(logo_height*sizeof(struct color *));
    if (!logo_data)
    die("%s\n", strerror(errno));
    for (i = 0; i < logo_height; i++) {
    logo_data[i] = malloc(logo_width*sizeof(struct color));
    if (!logo_data[i])
    die("%s\n", strerror(errno));
    }
// read image data
    switch (magic) {
    case '1':
// Plain PBM
    is_plain_pbm = 1;
    for (i = 0; i < logo_height; i++)
    for (j = 0; j < logo_width; j++)
    logo_data[i][j].red = logo_data[i][j].green =
    logo_data[i][j].blue = 255*(1-get_number(fp));
    break;
    case '2':
// Plain PGM
    maxval = get_number(fp);
    for (i = 0; i < logo_height; i++)
    for (j = 0; j < logo_width; j++)
    logo_data[i][j].red = logo_data[i][j].green =
    logo_data[i][j].blue = get_number255(fp, maxval);
    break;
    case '3':
// Plain PPM
    maxval = get_number(fp);
    for (i = 0; i < logo_height; i++)
    for (j = 0; j < logo_width; j++) {
    logo_data[i][j].red = get_number255(fp, maxval);
    logo_data[i][j].green = get_number255(fp, maxval);
    logo_data[i][j].blue = get_number255(fp, maxval);
    }
    break;
    }
// close file
    fclose(fp);
    }
#[no_mangle]
pub unsafe extern "C" fn is_black(c: color) -> c_int {
    static inline int is_black(struct color c)
    {
    return c.red == 0 && c.green == 0 && c.blue == 0;
    }
#[no_mangle]
pub unsafe extern "C" fn is_white(c: color) -> c_int {
    static inline int is_white(struct color c)
    {
    return c.red == 255 && c.green == 255 && c.blue == 255;
    }
#[no_mangle]
pub unsafe extern "C" fn is_gray(c: color) -> c_int {
    static inline int is_gray(struct color c)
    {
    return c.red == c.green && c.red == c.blue;
    }
#[no_mangle]
pub unsafe extern "C" fn is_equal(c1: color, c2: color) -> c_int {
    static inline int is_equal(struct color c1, struct color c2)
    {
    return c1.red == c2.red && c1.green == c2.green && c1.blue == c2.blue;
    }
#[no_mangle]
unsafe extern "C" fn write_header() {
    static void write_header(void)
    {
// open logo file
    if (outputname) {
    out = fopen(outputname, "w");
    if (!out)
    die("Cannot create file %s: %s\n", outputname, strerror(errno));
    } else {
    out = stdout;
    }
    fputs("/*\n", out);
    fputs(" *  DO NOT EDIT THIS FILE!\n", out);
    fputs(" *\n", out);
    fprintf(out, " *  Linux logo %s\n", logoname);
    fputs(" */\n\n", out);
    fputs("#include <linux/linux_logo.h>\n\n", out);
    fprintf(out, "static const unsigned char %s_data[] __initconst = {\n",
    logoname);
    }
#[no_mangle]
unsafe extern "C" fn write_footer() {
    static void write_footer(void)
    {
    fputs("\n};\n\n", out);
    fprintf(out, "const struct linux_logo %s __initconst = {\n", logoname);
    fprintf(out, "\t.type\t\t= %s,\n", logo_types[logo_type]);
    fprintf(out, "\t.width\t\t= %u,\n", logo_width);
    fprintf(out, "\t.height\t\t= %u,\n", logo_height);
    if (logo_type == LINUX_LOGO_CLUT224) {
    fprintf(out, "\t.clutsize\t= %u,\n", logo_clutsize);
    fprintf(out, "\t.clut\t\t= %s_clut,\n", logoname);
    }
    fprintf(out, "\t.data\t\t= %s_data\n", logoname);
    fputs("};\n\n", out);
// close logo file
    if (outputname)
    fclose(out);
    }
    static int write_hex_cnt;
#[no_mangle]
unsafe extern "C" fn write_hex(byte: c_uchar) {
    static void write_hex(unsigned char byte)
    {
    if (write_hex_cnt % 12)
    fprintf(out, ", 0x%02x", byte);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: write_hex_cnt) -> else {
    else if (write_hex_cnt)
    fprintf(out, ",\n\t0x%02x", byte);
    else
    fprintf(out, "\t0x%02x", byte);
    write_hex_cnt++;
    }
#[no_mangle]
unsafe extern "C" fn write_logo_mono() {
    static void write_logo_mono(void)
    {
    unsigned int i, j;
    unsigned char val, bit;
// validate image
    for (i = 0; i < logo_height; i++)
    for (j = 0; j < logo_width; j++)
    if (!is_black(logo_data[i][j]) && !is_white(logo_data[i][j]))
    die("Image must be monochrome\n");
// write file header
    write_header();
// write logo data
    for (i = 0; i < logo_height; i++) {
    for (j = 0; j < logo_width;) {
    for (val = 0, bit = 0x80; bit && j < logo_width; j++, bit >>= 1)
    if (logo_data[i][j].red)
    val |= bit;
    write_hex(val);
    }
    }
// write logo structure and file footer
    write_footer();
    }
#[no_mangle]
unsafe extern "C" fn write_logo_vga16() {
    static void write_logo_vga16(void)
    {
    unsigned int i, j, k;
    unsigned char val;
// validate image
    for (i = 0; i < logo_height; i++)
    for (j = 0; j < logo_width; j++) {
    for (k = 0; k < 16; k++)
    if (is_equal(logo_data[i][j], clut_vga16[k]))
    break;
    if (k == 16)
    die("Image must use the 16 console colors only\n"
    "Use ppmquant(1) -map clut_vga16.ppm to reduce the number "
    "of colors\n");
    }
// write file header
    write_header();
// write logo data
    for (i = 0; i < logo_height; i++)
    for (j = 0; j < logo_width; j++) {
    for (k = 0; k < 16; k++)
    if (is_equal(logo_data[i][j], clut_vga16[k]))
    break;
    val = k<<4;
    if (++j < logo_width) {
    for (k = 0; k < 16; k++)
    if (is_equal(logo_data[i][j], clut_vga16[k]))
    break;
    val |= k;
    }
    write_hex(val);
    }
// write logo structure and file footer
    write_footer();
    }
#[no_mangle]
unsafe extern "C" fn write_logo_clut224() {
    static void write_logo_clut224(void)
    {
    unsigned int i, j, k;
// validate image
    for (i = 0; i < logo_height; i++)
    for (j = 0; j < logo_width; j++) {
    for (k = 0; k < logo_clutsize; k++)
    if (is_equal(logo_data[i][j], logo_clut[k]))
    break;
    if (k == logo_clutsize) {
    if (logo_clutsize == MAX_LINUX_LOGO_COLORS)
    die("Image has more than %d colors\n"
    "Use ppmquant(1) to reduce the number of colors\n",
    MAX_LINUX_LOGO_COLORS);
    logo_clut[logo_clutsize++] = logo_data[i][j];
    }
    }
// write file header
    write_header();
// write logo data
    for (i = 0; i < logo_height; i++)
    for (j = 0; j < logo_width; j++) {
    for (k = 0; k < logo_clutsize; k++)
    if (is_equal(logo_data[i][j], logo_clut[k]))
    break;
    write_hex(k+32);
    }
    fputs("\n};\n\n", out);
// write logo clut
    fprintf(out, "static const unsigned char %s_clut[] __initconst = {\n",
    logoname);
    write_hex_cnt = 0;
    for (i = 0; i < logo_clutsize; i++) {
    write_hex(logo_clut[i].red);
    write_hex(logo_clut[i].green);
    write_hex(logo_clut[i].blue);
    }
// write logo structure and file footer
    write_footer();
    }
#[no_mangle]
unsafe extern "C" fn write_logo_gray256() {
    static void write_logo_gray256(void)
    {
    unsigned int i, j;
// validate image
    for (i = 0; i < logo_height; i++)
    for (j = 0; j < logo_width; j++)
    if (!is_gray(logo_data[i][j]))
    die("Image must be grayscale\n");
// write file header
    write_header();
// write logo data
    for (i = 0; i < logo_height; i++)
    for (j = 0; j < logo_width; j++)
    write_hex(logo_data[i][j].red);
// write logo structure and file footer
    write_footer();
    }
#[no_mangle]
unsafe extern "C" fn die(fmt: *const c_char, ...) {
    static void die(const char *fmt, ...)
    {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    die("\n"
    "Usage: %s [options] <filename>\n"
    "\n"
    "Valid options:\n"
    "	-h		  : display this usage information\n"
    "	-n <name>   : specify logo name (default: linux_logo)\n"
    "	-o <output> : output to file <output> instead of stdout\n"
    "	-t <type>   : specify logo type, one of\n"
    "					  mono	: monochrome black/white\n"
    "					  vga16   : 16 colors VGA text palette\n"
    "					  clut224 : 224 colors (default)\n"
    "					  gray256 : 256 levels grayscale\n"
    "\n", programname);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int opt;
    programname = argv[0];
    opterr = 0;
    while (1) {
    opt = getopt(argc, argv, "hn:o:t:");
    if (opt == -1)
    break;
    switch (opt) {
    case 'h':
    usage();
    break;
    case 'n':
    logoname = optarg;
    break;
    case 'o':
    outputname = optarg;
    break;
    case 't':
    if (!strcmp(optarg, "mono"))
    logo_type = LINUX_LOGO_MONO;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(optarg, _arg: "vga16")) -> else {
    else if (!strcmp(optarg, "vga16"))
    logo_type = LINUX_LOGO_VGA16;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(optarg, _arg: "clut224")) -> else {
    else if (!strcmp(optarg, "clut224"))
    logo_type = LINUX_LOGO_CLUT224;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(optarg, _arg: "gray256")) -> else {
    else if (!strcmp(optarg, "gray256"))
    logo_type = LINUX_LOGO_GRAY256;
    else
    usage();
    break;
    default:
    usage();
    break;
    }
    }
    if (optind != argc-1)
    usage();
    filename = argv[optind];
    read_image();
    switch (logo_type) {
    case LINUX_LOGO_MONO:
    write_logo_mono();
    break;
    case LINUX_LOGO_VGA16:
    write_logo_vga16();
    break;
    case LINUX_LOGO_CLUT224:
    write_logo_clut224();
    break;
    case LINUX_LOGO_GRAY256:
    write_logo_gray256();
    break;
    }
    exit(0);
    }
