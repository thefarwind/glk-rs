//! This module contains safe rust functions for glk functions. Function
//! and struct names have removed the glk prefix (which is now the
//! namespace) and been modified to match nominal rust name formatting.

use ::gen::*;

pub type WinId = winid_t;
pub type StrId = strid_t;
pub type FRefId = frefid_t;
pub type SChanId = schanid_t;

pub fn main() { unsafe { glk_main() } }
pub fn exit() { unsafe { glk_exit() } }
// TODO: Interrupt Handler
pub fn tick() { unsafe { glk_tick() } }
pub fn gestalt(sel: u32, val: u32) -> u32 { unsafe { glk_gestalt(sel, val) } }
// TODO: gestalt_ext
pub fn char_to_lower(input: u8) -> u8 { unsafe { glk_char_to_lower(input) } }
pub fn char_to_upper(input: u8) -> u8 { unsafe { glk_char_to_upper(input) } }

// Line Echo

pub fn set_echo_line_event(win: WinId, val: u32) {
    unsafe { glk_set_echo_line_event(win, val) }
}

// Line Terminators

// TODO: pub fn set_terminators_line_event

// Unicode

// Unicode Norm

// Image

pub fn image_draw(win: WinId, image: u32, val1: i32, val2: i32) -> u32 {
    unsafe { glk_image_draw(win, image, val1, val2) }
}
pub fn image_draw_scaled(
    win: WinId,
    image: u32,
    val1: i32,
    val2: i32,
    width: u32,
    height: u32,
) -> u32 {
    unsafe { glk_image_draw_scaled(win, image, val1, val2, width, height) }
}
// TODO: image_get_info
pub fn window_flow_break(win: WinId) { unsafe { glk_window_flow_break(win) } }
pub fn window_erase_rect(
    win: WinId,
    left: i32,
    top: i32,
    width: u32,
    height: u32,
) {
    unsafe { glk_window_erase_rect(win, left, top, width, height) }
}
pub fn window_fill_rect(
    win: WinId,
    color: u32,
    left: i32,
    top: i32,
    width: u32,
    height: u32,
) {
    unsafe { glk_window_fill_rect(win, color, left, top, width, height) }
}
pub fn window_set_background_color(win: WinId, color: u32) {
    unsafe { glk_window_set_background_color(win, color) }
}

// Sound

pub fn schannel_create(rock: u32) -> SChanId {
    unsafe { glk_schannel_create(rock) }
}
pub fn schannel_destroy(chan: SChanId) {
    unsafe { glk_schannel_destroy(chan) }
}
// TODO: schannel_iterate
pub fn schannel_get_rock(chan: SChanId) -> u32 {
    unsafe { glk_schannel_get_rock(chan) }
}
pub fn schannel_play(chan: SChanId, snd: u32) -> u32 {
    unsafe { glk_schannel_play(chan, snd) }
}
pub fn schannel_play_ext(
    chan: SChanId,
    snd: u32,
    repeats: u32,
    notify: u32,
) -> u32 {
    unsafe { glk_schannel_play_ext(chan, snd, repeats, notify) }
}
pub fn schannel_stop(chan: SChanId) {
    unsafe { glk_schannel_stop(chan) }
}
pub fn schannel_set_volume(chan: SChanId, vol: u32) {
    unsafe { glk_schannel_set_volume(chan, vol) }
}
pub fn sound_load_hint(snd: u32, flag: u32) {
    unsafe { glk_sound_load_hint(snd, flag) }
}

// Sound 2

pub fn schannel_create_ext(rock: u32, volume: u32) -> SChanId {
    unsafe { glk_schannel_create_ext(rock, volume) }
}
// TODO: schannel_play_multi
pub fn schannel_pause(chan: SChanId) {
    unsafe { glk_schannel_pause(chan) }
}
pub fn schannel_unpause(chan: SChanId) {
    unsafe { glk_schannel_unpause(chan) }
}
pub fn schannel_set_volume_ext(
    chan: SChanId,
    volume: u32,
    duration: u32,
    notify: u32,
) {
    unsafe { glk_schannel_set_volume_ext(chan, volume, duration, notify) }
}

// Hyperlinks

pub fn set_hyperlink(link: u32) { unsafe { glk_set_hyperlink(link) } }
pub fn set_hyperlink_stream(stream: StrId, link: u32) {
    unsafe { glk_set_hyperlink_stream(stream, link) }
}
pub fn request_hyperlink_event(win: WinId) {
    unsafe { glk_request_hyperlink_event(win) }
}
pub fn cancel_hyperlink_event(win: WinId) {
    unsafe { glk_cancel_hyperlink_event(win) }
}


// Datetime

pub type Time = glktimeval_t;

impl Time {
    pub fn new(high_sec: i32, low_sec: u32, microsec: i32) -> Time {
        Time{
            high_sec: high_sec,
            low_sec: low_sec,
            microsec:  microsec,
        }
    }
}

pub type Date = glkdate_t;

impl Date {
    pub fn new(
        year: i32,
        month: i32,
        day: i32,
        weekday: i32,
        hour: i32,
        minute: i32,
        second: i32,
        microsec: i32,
    ) -> Date {
        Date{
            year: year,
            month: month,
            day: day,
            weekday: weekday,
            hour: hour,
            minute: minute,
            second: second,
            microsec: microsec,
        }
    }
}

pub fn current_time(time: &mut Time) {
    unsafe { glk_current_time(time) }
}
pub fn current_simple_time(factor: u32) -> i32 {
    unsafe { glk_current_simple_time(factor) }
}
pub fn time_to_date_utc(time: &mut Time, date: &mut Date) {
    unsafe { glk_time_to_date_utc(time, date) }
}
pub fn time_to_date_local(time: &mut Time, date: &mut Date) {
    unsafe { glk_time_to_date_local(time, date) }
}
pub fn simple_time_to_date_utc(time: i32, factor: u32, date: &mut Date) {
    unsafe { glk_simple_time_to_date_utc(time, factor, date) }
}
pub fn simple_time_to_date_local(time: i32, factor: u32, date: &mut Date) {
    unsafe { glk_simple_time_to_date_local(time, factor, date) }
}
pub fn date_to_time_utc(date: &mut Date, time: &mut Time) {
    unsafe { glk_date_to_time_utc(date, time) }
}
pub fn date_to_time_local(date: &mut Date, time: &mut Time) {
    unsafe { glk_date_to_time_local(date, time) }
}
pub fn date_to_simple_time_utc(date: &mut Date, factor: u32) -> i32 {
    unsafe { glk_date_to_simple_time_utc(date, factor) }
}
pub fn date_to_simple_time_local(date: &mut Date, factor: u32) -> i32 {
    unsafe { glk_date_to_simple_time_local(date, factor) }
}

// Resource Stream

pub fn stream_open_resource(filenum: u32, rock: u32) -> StrId {
    unsafe { glk_stream_open_resource(filenum, rock) }
}
pub fn stream_open_resource_uni(filenum: u32, rock: u32) -> StrId {
    unsafe { glk_stream_open_resource_uni(filenum, rock) }
}
