//! Automatically rewritten from C to Rust
//! Source: scripts/kconfig/lxdialog/util.c
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
// util.c
//
// ORIGINAL AUTHOR: Savio Lam (lam836@cs.cuhk.hk)
// MODIFIED FOR LINUX KERNEL CONFIG BY: William Roadcap (roadcap@cfw.com)
//

// Needed in signal handler in mconf.c
    int saved_x, saved_y;
    struct dialog_info dlg;
#[no_mangle]
unsafe extern "C" fn set_mono_theme() {
    static void set_mono_theme(void)
    {
    dlg.title.atr = A_BOLD;
    dlg.button_active.atr = A_REVERSE;
    dlg.button_inactive.atr = A_DIM;
    dlg.button_key_active.atr = A_REVERSE;
    dlg.button_key_inactive.atr = A_BOLD;
    dlg.button_label_active.atr = A_REVERSE;
    dlg.position_indicator.atr = A_BOLD;
    dlg.item_selected.atr = A_REVERSE;
    dlg.tag.atr = A_BOLD;
    dlg.tag_selected.atr = A_REVERSE;
    dlg.tag_key.atr = A_BOLD;
    dlg.tag_key_selected.atr = A_REVERSE;
    dlg.check.atr = A_BOLD;
    dlg.check_selected.atr = A_REVERSE;
    dlg.uarrow.atr = A_BOLD;
    dlg.darrow.atr = A_BOLD;
    }

    do {                               \
    dlg.dialog.fg = (f);       \
    dlg.dialog.bg = (b);       \
    dlg.dialog.hl = (h);       \
    } while (0)
#[no_mangle]
unsafe extern "C" fn set_classic_theme() {
    static void set_classic_theme(void)
    {
    DLG_COLOR(screen,                COLOR_CYAN,   COLOR_BLUE,   true);
    DLG_COLOR(shadow,                COLOR_BLACK,  COLOR_BLACK,  true);
    DLG_COLOR(dialog,                COLOR_BLACK,  COLOR_WHITE,  false);
    DLG_COLOR(title,                 COLOR_YELLOW, COLOR_WHITE,  true);
    DLG_COLOR(border,                COLOR_WHITE,  COLOR_WHITE,  true);
    DLG_COLOR(button_active,         COLOR_WHITE,  COLOR_BLUE,   true);
    DLG_COLOR(button_inactive,       COLOR_BLACK,  COLOR_WHITE,  false);
    DLG_COLOR(button_key_active,     COLOR_WHITE,  COLOR_BLUE,   true);
    DLG_COLOR(button_key_inactive,   COLOR_RED,    COLOR_WHITE,  false);
    DLG_COLOR(button_label_active,   COLOR_YELLOW, COLOR_BLUE,   true);
    DLG_COLOR(button_label_inactive, COLOR_BLACK,  COLOR_WHITE,  true);
    DLG_COLOR(inputbox,              COLOR_BLACK,  COLOR_WHITE,  false);
    DLG_COLOR(position_indicator,    COLOR_YELLOW, COLOR_WHITE,  true);
    DLG_COLOR(menubox,               COLOR_BLACK,  COLOR_WHITE,  false);
    DLG_COLOR(menubox_border,        COLOR_WHITE,  COLOR_WHITE,  true);
    DLG_COLOR(item,                  COLOR_BLACK,  COLOR_WHITE,  false);
    DLG_COLOR(item_selected,         COLOR_WHITE,  COLOR_BLUE,   true);
    DLG_COLOR(tag,                   COLOR_YELLOW, COLOR_WHITE,  true);
    DLG_COLOR(tag_selected,          COLOR_YELLOW, COLOR_BLUE,   true);
    DLG_COLOR(tag_key,               COLOR_YELLOW, COLOR_WHITE,  true);
    DLG_COLOR(tag_key_selected,      COLOR_YELLOW, COLOR_BLUE,   true);
    DLG_COLOR(check,                 COLOR_BLACK,  COLOR_WHITE,  false);
    DLG_COLOR(check_selected,        COLOR_WHITE,  COLOR_BLUE,   true);
    DLG_COLOR(uarrow,                COLOR_GREEN,  COLOR_WHITE,  true);
    DLG_COLOR(darrow,                COLOR_GREEN,  COLOR_WHITE,  true);
    }
#[no_mangle]
unsafe extern "C" fn set_blackbg_theme() {
    static void set_blackbg_theme(void)
    {
    DLG_COLOR(screen, COLOR_RED,   COLOR_BLACK, true);
    DLG_COLOR(shadow, COLOR_BLACK, COLOR_BLACK, false);
    DLG_COLOR(dialog, COLOR_WHITE, COLOR_BLACK, false);
    DLG_COLOR(title,  COLOR_RED,   COLOR_BLACK, false);
    DLG_COLOR(border, COLOR_BLACK, COLOR_BLACK, true);
    DLG_COLOR(button_active,         COLOR_YELLOW, COLOR_RED,   false);
    DLG_COLOR(button_inactive,       COLOR_YELLOW, COLOR_BLACK, false);
    DLG_COLOR(button_key_active,     COLOR_YELLOW, COLOR_RED,   true);
    DLG_COLOR(button_key_inactive,   COLOR_RED,    COLOR_BLACK, false);
    DLG_COLOR(button_label_active,   COLOR_WHITE,  COLOR_RED,   false);
    DLG_COLOR(button_label_inactive, COLOR_WHITE,  COLOR_BLACK, false);
    DLG_COLOR(inputbox,         COLOR_YELLOW, COLOR_BLACK, false);
    DLG_COLOR(position_indicator, COLOR_RED, COLOR_BLACK,  false);
    DLG_COLOR(menubox,          COLOR_YELLOW, COLOR_BLACK, false);
    DLG_COLOR(menubox_border,   COLOR_BLACK,  COLOR_BLACK, true);
    DLG_COLOR(item,             COLOR_WHITE, COLOR_BLACK, false);
    DLG_COLOR(item_selected,    COLOR_WHITE, COLOR_RED,   false);
    DLG_COLOR(tag,              COLOR_RED,    COLOR_BLACK, false);
    DLG_COLOR(tag_selected,     COLOR_YELLOW, COLOR_RED,   true);
    DLG_COLOR(tag_key,          COLOR_RED,    COLOR_BLACK, false);
    DLG_COLOR(tag_key_selected, COLOR_YELLOW, COLOR_RED,   true);
    DLG_COLOR(check,            COLOR_YELLOW, COLOR_BLACK, false);
    DLG_COLOR(check_selected,   COLOR_YELLOW, COLOR_RED,   true);
    DLG_COLOR(uarrow, COLOR_RED, COLOR_BLACK, false);
    DLG_COLOR(darrow, COLOR_RED, COLOR_BLACK, false);
    }
#[no_mangle]
unsafe extern "C" fn set_bluetitle_theme() {
    static void set_bluetitle_theme(void)
    {
    set_classic_theme();
    DLG_COLOR(title,               COLOR_BLUE,   COLOR_WHITE, true);
    DLG_COLOR(button_key_active,   COLOR_YELLOW, COLOR_BLUE,  true);
    DLG_COLOR(button_label_active, COLOR_WHITE,  COLOR_BLUE,  true);
    DLG_COLOR(position_indicator,  COLOR_BLUE,   COLOR_WHITE, true);
    DLG_COLOR(tag,                 COLOR_BLUE,   COLOR_WHITE, true);
    DLG_COLOR(tag_key,             COLOR_BLUE,   COLOR_WHITE, true);
    }
//
// Select color theme
//
#[no_mangle]
unsafe extern "C" fn set_theme(theme: *const c_char) -> c_int {
    static int set_theme(const char *theme)
    {
    let mut use_color: c_int = 1;
    if (!theme)
    set_bluetitle_theme();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(theme, 0: "classic") ==) -> else {
    else if (strcmp(theme, "classic") == 0)
    set_classic_theme();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(theme, 0: "bluetitle") ==) -> else {
    else if (strcmp(theme, "bluetitle") == 0)
    set_bluetitle_theme();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(theme, 0: "blackbg") ==) -> else {
    else if (strcmp(theme, "blackbg") == 0)
    set_blackbg_theme();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(theme, 0: "mono") ==) -> else {
    else if (strcmp(theme, "mono") == 0)
    use_color = 0;
    return use_color;
    }
#[no_mangle]
unsafe extern "C" fn init_one_color(color: *mut dialog_color) {
    static void init_one_color(struct dialog_color *color)
    {
    let mut pair: static int = 0;
    pair++;
    init_pair(pair, color.fg, color.bg);
    if (color.hl)
    color.atr = A_BOLD | COLOR_PAIR(pair);
    else
    color.atr = COLOR_PAIR(pair);
    }
#[no_mangle]
unsafe extern "C" fn init_dialog_colors() {
    static void init_dialog_colors(void)
    {
    init_one_color(&dlg.screen);
    init_one_color(&dlg.shadow);
    init_one_color(&dlg.dialog);
    init_one_color(&dlg.title);
    init_one_color(&dlg.border);
    init_one_color(&dlg.button_active);
    init_one_color(&dlg.button_inactive);
    init_one_color(&dlg.button_key_active);
    init_one_color(&dlg.button_key_inactive);
    init_one_color(&dlg.button_label_active);
    init_one_color(&dlg.button_label_inactive);
    init_one_color(&dlg.inputbox);
    init_one_color(&dlg.position_indicator);
    init_one_color(&dlg.menubox);
    init_one_color(&dlg.menubox_border);
    init_one_color(&dlg.item);
    init_one_color(&dlg.item_selected);
    init_one_color(&dlg.tag);
    init_one_color(&dlg.tag_selected);
    init_one_color(&dlg.tag_key);
    init_one_color(&dlg.tag_key_selected);
    init_one_color(&dlg.check);
    init_one_color(&dlg.check_selected);
    init_one_color(&dlg.uarrow);
    init_one_color(&dlg.darrow);
    }
//
// Setup for color display
//
#[no_mangle]
unsafe extern "C" fn color_setup(theme: *const c_char) {
    static void color_setup(const char *theme)
    {
    int use_color;
    use_color = set_theme(theme);
    if (use_color && has_colors()) {
    start_color();
    init_dialog_colors();
    } else
    set_mono_theme();
    }
//
// Set window to attribute 'attr'
//
#[no_mangle]
pub unsafe extern "C" fn attr_clear(win: *mut *mut WINDOW, height: c_int, width: c_int, attr: chtype) {
    void attr_clear(WINDOW * win, int height, int width, chtype attr)
    {
    int i, j;
    wattrset(win, attr);
    for (i = 0; i < height; i++) {
    wmove(win, i, 0);
    for (j = 0; j < width; j++)
    waddch(win, ' ');
    }
    touchwin(win);
    }
#[no_mangle]
pub unsafe extern "C" fn dialog_clear() {
    void dialog_clear(void)
    {
    int lines, columns;
    lines = getmaxy(stdscr);
    columns = getmaxx(stdscr);
    attr_clear(stdscr, lines, columns, dlg.screen.atr);
// Display background title if it exists ... - SLH
    if (dlg.backtitle != core::ptr::null_mut()) {
    int i, len = 0, skip = 0;
    struct subtitle_list *pos;
    wattrset(stdscr, dlg.screen.atr);
    mvwaddstr(stdscr, 0, 1, (char *)dlg.backtitle);
    for (pos = dlg.subtitles; pos != core::ptr::null_mut(); pos = pos.next) {
// 3 is for the arrow and spaces
    len += strlen(pos.text) + 3;
    }
    wmove(stdscr, 1, 1);
    if (len > columns - 2) {
    const char *ellipsis = "[...] ";
    waddstr(stdscr, ellipsis);
    skip = len - (columns - 2 - strlen(ellipsis));
    }
    for (pos = dlg.subtitles; pos != core::ptr::null_mut(); pos = pos.next) {
    if (skip == 0)
    waddch(stdscr, ACS_RARROW);
    else
    skip--;
    if (skip == 0)
    waddch(stdscr, ' ');
    else
    skip--;
    if (skip < strlen(pos.text)) {
    waddstr(stdscr, pos.text + skip);
    skip = 0;
    } else
    skip -= strlen(pos.text);
    if (skip == 0)
    waddch(stdscr, ' ');
    else
    skip--;
    }
    for (i = len + 1; i < columns - 1; i++)
    waddch(stdscr, ACS_HLINE);
    }
    wnoutrefresh(stdscr);
    }
//
// Do some initialization for dialog
//
#[no_mangle]
pub unsafe extern "C" fn init_dialog(backtitle: *const c_char) -> c_int {
    int init_dialog(const char *backtitle)
    {
    int height, width;
    initscr();		/* Init curses */
// Get current cursor position for signal handler in mconf.c
    getyx(stdscr, saved_y, saved_x);
    getmaxyx(stdscr, height, width);
    if (height < WINDOW_HEIGHT_MIN || width < WINDOW_WIDTH_MIN) {
    endwin();
    return -ERRDISPLAYTOOSMALL;
    }
    dlg.backtitle = backtitle;
    color_setup(getenv("MENUCONFIG_COLOR"));
    keypad(stdscr, TRUE);
    cbreak();
    noecho();
    dialog_clear();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_dialog_backtitle(backtitle: *const c_char) {
    void set_dialog_backtitle(const char *backtitle)
    {
    dlg.backtitle = backtitle;
    }
#[no_mangle]
pub unsafe extern "C" fn set_dialog_subtitles(subtitles: *mut subtitle_list) {
    void set_dialog_subtitles(struct subtitle_list *subtitles)
    {
    dlg.subtitles = subtitles;
    }
//
// End using dialog functions.
//
#[no_mangle]
pub unsafe extern "C" fn end_dialog(x: c_int, y: c_int) {
    void end_dialog(int x, int y)
    {
// move cursor back to original position
    move(y, x);
    refresh();
    endwin();
    }
// Print the title of the dialog. Center the title and truncate
// tile if wider than dialog (- 2 chars).
//
#[no_mangle]
pub unsafe extern "C" fn print_title(dialog: *mut WINDOW, title: *const c_char, width: c_int) {
    void print_title(WINDOW *dialog, const char *title, int width)
    {
    if (title) {
    let mut tlen: c_int = MIN(width - 2, strlen(title));
    wattrset(dialog, dlg.title.atr);
    mvwaddch(dialog, 0, (width - tlen) / 2 - 1, ' ');
    mvwaddnstr(dialog, 0, (width - tlen)/2, title, tlen);
    waddch(dialog, ' ');
    }
    }
//
// Print a string of text in a window, automatically wrap around to the
// next line if the string is too long to fit on one line. Newline
// characters '\n' are properly processed.  We start on a new line
// if there is no room for at least 4 nonblanks following a double-space.
//
#[no_mangle]
pub unsafe extern "C" fn print_autowrap(win: *mut *mut WINDOW, prompt: *const c_char, width: c_int, y: c_int, x: c_int) {
    void print_autowrap(WINDOW * win, const char *prompt, int width, int y, int x)
    {
    int newl, cur_x, cur_y;
    int prompt_len, room, wlen;
    char tempstr[MAX_LEN + 1], *word, *sp, *sp2, *newline_separator = 0;
    snprintf(tempstr, sizeof(tempstr), "%s", prompt);
    prompt_len = strlen(tempstr);
    if (prompt_len <= width - x * 2) {	/* If prompt is short */
    wmove(win, y, (width - prompt_len) / 2);
    waddstr(win, tempstr);
    } else {
    cur_x = x;
    cur_y = y;
    newl = 1;
    word = tempstr;
    while (word && *word) {
    sp = strpbrk(word, "\n ");
    if (sp && *sp == '\n')
    newline_separator = sp;
    if (sp)
// sp++ = 0;
// Wrap to next line if either the word does not fit,
    or it is the first word of a new sentence, and it is
    short, and the next word does not fit. */
    room = width - cur_x;
    wlen = strlen(word);
    if (wlen > room ||
    (newl && wlen < 4 && sp
    && wlen + 1 + strlen(sp) > room
    && (!(sp2 = strpbrk(sp, "\n "))
    || wlen + 1 + (sp2 - sp) > room))) {
    cur_y++;
    cur_x = x;
    }
    wmove(win, cur_y, cur_x);
    waddstr(win, word);
    getyx(win, cur_y, cur_x);
// Move to the next line if the word separator was a newline
    if (newline_separator) {
    cur_y++;
    cur_x = x;
    newline_separator = 0;
    } else
    cur_x++;
    if (sp && *sp == ' ') {
    cur_x++;	/* double space */
    while (*++sp == ' ') ;
    newl = 1;
    } else
    newl = 0;
    word = sp;
    }
    }
    }
//
// Print a button
//
#[no_mangle]
pub unsafe extern "C" fn print_button(win: *mut *mut WINDOW, label: *const c_char, y: c_int, x: c_int, selected: c_int) {
    void print_button(WINDOW * win, const char *label, int y, int x, int selected)
    {
    int i, temp;
    wmove(win, y, x);
    wattrset(win, selected ? dlg.button_active.atr
    : dlg.button_inactive.atr);
    waddstr(win, "<");
    temp = strspn(label, " ");
    label += temp;
    wattrset(win, selected ? dlg.button_label_active.atr
    : dlg.button_label_inactive.atr);
    for (i = 0; i < temp; i++)
    waddch(win, ' ');
    wattrset(win, selected ? dlg.button_key_active.atr
    : dlg.button_key_inactive.atr);
    waddch(win, label[0]);
    wattrset(win, selected ? dlg.button_label_active.atr
    : dlg.button_label_inactive.atr);
    waddstr(win, (char *)label + 1);
    wattrset(win, selected ? dlg.button_active.atr
    : dlg.button_inactive.atr);
    waddstr(win, ">");
    wmove(win, y, x + temp + 1);
    }
//
// Draw a rectangular box with line drawing characters
//
    void
    draw_box(WINDOW * win, int y, int x, int height, int width,
    chtype box, chtype border)
    {
    int i, j;
    wattrset(win, 0);
    for (i = 0; i < height; i++) {
    wmove(win, y + i, x);
    for (j = 0; j < width; j++)
    if (!i && !j)
    waddch(win, border | ACS_ULCORNER);
#[no_mangle]
pub unsafe extern "C" fn if(!j: i == height - 1 &&) -> else {
    else if (i == height - 1 && !j)
    waddch(win, border | ACS_LLCORNER);
#[no_mangle]
pub unsafe extern "C" fn if(1: !i && j == width -) -> else {
    else if (!i && j == width - 1)
    waddch(win, box | ACS_URCORNER);
#[no_mangle]
pub unsafe extern "C" fn if(1: i == height - 1 && j == width -) -> else {
    else if (i == height - 1 && j == width - 1)
    waddch(win, box | ACS_LRCORNER);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !i) -> else {
    else if (!i)
    waddch(win, border | ACS_HLINE);
#[no_mangle]
pub unsafe extern "C" fn if(1: i == height -) -> else {
    else if (i == height - 1)
    waddch(win, box | ACS_HLINE);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !j) -> else {
    else if (!j)
    waddch(win, border | ACS_VLINE);
#[no_mangle]
pub unsafe extern "C" fn if(1: j == width -) -> else {
    else if (j == width - 1)
    waddch(win, box | ACS_VLINE);
    else
    waddch(win, box | ' ');
    }
    }
//
// Draw shadows along the right and bottom edge to give a more 3D look
// to the boxes
//
#[no_mangle]
pub unsafe extern "C" fn draw_shadow(win: *mut *mut WINDOW, y: c_int, x: c_int, height: c_int, width: c_int) {
    void draw_shadow(WINDOW * win, int y, int x, int height, int width)
    {
    int i;
    if (has_colors()) {	/* Whether terminal supports color? */
    wattrset(win, dlg.shadow.atr);
    wmove(win, y + height, x + 2);
    for (i = 0; i < width; i++)
    waddch(win, winch(win) & A_CHARTEXT);
    for (i = y + 1; i < y + height + 1; i++) {
    wmove(win, i, x + width);
    waddch(win, winch(win) & A_CHARTEXT);
    waddch(win, winch(win) & A_CHARTEXT);
    }
    wnoutrefresh(win);
    }
    }
//
// Return the position of the first alphabetic character in a string.
//
#[no_mangle]
pub unsafe extern "C" fn first_alpha(string: *const c_char, exempt: *const c_char) -> c_int {
    int first_alpha(const char *string, const char *exempt)
    {
    int i, in_paren = 0, c;
    for (i = 0; i < strlen(string); i++) {
    c = tolower(string[i]);
    if (strchr("<[(", c))
    ++in_paren;
    if (strchr(">])", c) && in_paren > 0)
    --in_paren;
    if ((!in_paren) && isalpha(c) && strchr(exempt, c) == 0)
    return i;
    }
    return 0;
    }
//
// ncurses uses ESC to detect escaped char sequences. This resutl in
// a small timeout before ESC is actually delivered to the application.
// lxdialog suggest <ESC> <ESC> which is correctly translated to two
// times esc. But then we need to ignore the second esc to avoid stepping
// out one menu too much. Filter away all escaped key sequences since
// keypad(FALSE) turn off ncurses support for escape sequences - and that's
// needed to make notimeout() do as expected.
//
#[no_mangle]
pub unsafe extern "C" fn on_key_esc(win: *mut WINDOW) -> c_int {
    int on_key_esc(WINDOW *win)
    {
    int key;
    int key2;
    int key3;
    nodelay(win, TRUE);
    keypad(win, FALSE);
    key = wgetch(win);
    key2 = wgetch(win);
    do {
    key3 = wgetch(win);
    } while (key3 != ERR);
    nodelay(win, FALSE);
    keypad(win, TRUE);
    if (key == KEY_ESC && key2 == ERR)
    return KEY_ESC;
#[no_mangle]
pub unsafe extern "C" fn if(ERR: key != ERR && key != KEY_ESC && key2 ==) -> else {
    else if (key != ERR && key != KEY_ESC && key2 == ERR)
    ungetch(key);
    return -1;
    }
// redraw screen in new size
#[no_mangle]
pub unsafe extern "C" fn on_key_resize() -> c_int {
    int on_key_resize(void)
    {
    dialog_clear();
    return KEY_RESIZE;
    }
    struct dialog_list *item_cur;
    struct dialog_list item_nil;
    struct dialog_list *item_head;
#[no_mangle]
pub unsafe extern "C" fn item_reset() {
    void item_reset(void)
    {
    struct dialog_list *p, *next;
    for (p = item_head; p; p = next) {
    next = p.next;
    free(p);
    }
    item_head = core::ptr::null_mut();
    item_cur = &item_nil;
    }
#[no_mangle]
pub unsafe extern "C" fn item_make(fmt: *const c_char, ...) {
    void item_make(const char *fmt, ...)
    {
    va_list ap;
    struct dialog_list *p = malloc(sizeof(*p));
    if (item_head)
    item_cur.next = p;
    else
    item_head = p;
    item_cur = p;
    memset(p, 0, sizeof(*p));
    va_start(ap, fmt);
    vsnprintf(item_cur.node.str, sizeof(item_cur.node.str), fmt, ap);
    va_end(ap);
    }
#[no_mangle]
pub unsafe extern "C" fn item_add_str(fmt: *const c_char, ...) {
    void item_add_str(const char *fmt, ...)
    {
    va_list ap;
    size_t avail;
    avail = sizeof(item_cur.node.str) - strlen(item_cur.node.str);
    va_start(ap, fmt);
    vsnprintf(item_cur.node.str + strlen(item_cur.node.str),
    avail, fmt, ap);
    item_cur.node.str[sizeof(item_cur.node.str) - 1] = '\0';
    va_end(ap);
    }
#[no_mangle]
pub unsafe extern "C" fn item_set_tag(tag: c_char) {
    void item_set_tag(char tag)
    {
    item_cur.node.tag = tag;
    }
#[no_mangle]
pub unsafe extern "C" fn item_set_data(ptr: *mut c_void) {
    void item_set_data(void *ptr)
    {
    item_cur.node.data = ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn item_set_selected(val: c_int) {
    void item_set_selected(int val)
    {
    item_cur.node.selected = val;
    }
#[no_mangle]
pub unsafe extern "C" fn item_activate_selected() -> c_int {
    int item_activate_selected(void)
    {
    item_foreach()
    if (item_is_selected())
    return 1;
    return 0;
    }
    void *item_data(void)
    {
    return item_cur.node.data;
    }
#[no_mangle]
pub unsafe extern "C" fn item_tag() -> c_char {
    char item_tag(void)
    {
    return item_cur.node.tag;
    }
#[no_mangle]
pub unsafe extern "C" fn item_count() -> c_int {
    int item_count(void)
    {
    let mut n: c_int = 0;
    struct dialog_list *p;
    for (p = item_head; p; p = p.next)
    n++;
    return n;
    }
#[no_mangle]
pub unsafe extern "C" fn item_set(n: c_int) {
    void item_set(int n)
    {
    let mut i: c_int = 0;
    item_foreach()
    if (i++ == n)
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn item_n() -> c_int {
    int item_n(void)
    {
    let mut n: c_int = 0;
    struct dialog_list *p;
    for (p = item_head; p; p = p.next) {
    if (p == item_cur)
    return n;
    n++;
    }
    return 0;
    }
    const char *item_str(void)
    {
    return item_cur.node.str;
    }
#[no_mangle]
pub unsafe extern "C" fn item_is_selected() -> c_int {
    int item_is_selected(void)
    {
    return (item_cur.node.selected != 0);
    }
#[no_mangle]
pub unsafe extern "C" fn item_is_tag(tag: c_char) -> c_int {
    int item_is_tag(char tag)
    {
    return (item_cur.node.tag == tag);
    }
