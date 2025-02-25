mod lrc;
mod qrc;
mod utils;
mod yrc;
mod lys;

use std::borrow::Cow;

use serde::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricWord<'a> {
    pub start_time: usize,
    pub end_time: usize,
    pub word: Cow<'a, str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LyricLine<'a> {
    pub words: Vec<LyricWord<'a>>,
    #[serde(default, rename = "isBG")]
    pub is_bg: bool,
    #[serde(default)]
    pub is_duet: bool,
}

#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &'static str = r###"

/**
 * 解析 LyRiC 格式的歌词字符串
 * @param src 歌词字符串
 * @returns 成功解析出来的歌词
 */
export function parseLrc(src: string): LyricLine[];

/**
 * 将歌词数组转换为 LyRiC 格式的字符串
 * @param lines 歌词数组
 * @returns LyRiC 格式的字符串
 */
export function stringifyLrc(lines: LyricLine[]): string;

/**
 * 解析 YRC 格式的歌词字符串
 * @param src 歌词字符串
 * @returns 成功解析出来的歌词
 */
export function parseYrc(src: string): LyricLine[];

/**
 * 将歌词数组转换为 YRC 格式的字符串
 * @param lines 歌词数组
 * @returns YRC 格式的字符串
 */
export function stringifyYrc(lines: LyricLine[]): string;

/**
 * 解析 QRC 格式的歌词字符串
 * @param src 歌词字符串
 * @returns 成功解析出来的歌词
 */
export function parseQrc(src: string): LyricLine[];

/**
 * 将歌词数组转换为 QRC 格式的字符串
 * @param lines 歌词数组
 * @returns QRC 格式的字符串
 */
export function stringifyQrc(lines: LyricLine[]): string;

/**
 * 解析 Lyricify Syllable 格式的歌词字符串
 * @param src 歌词字符串
 * @returns 成功解析出来的歌词
 */
export function parseLys(src: string): LyricLine[];

/**
 * 将歌词数组转换为 Lyricify Syllable 格式的字符串
 * @param lines 歌词数组
 * @returns Lyricify Syllable 格式的字符串
 */
export function stringifyLys(lines: LyricLine[]): string;

/**
 * 一个歌词单词
 */
export interface LyricWord {
    /** 单词的起始时间 */
    startTime: number;
    /** 单词的结束时间 */
    endTime: number;
    /** 单词 */
    word: string;
};

/**
 * 一行歌词，存储多个单词
 * 如果是 LyRiC 等只能表达一行歌词的格式，则会将整行当做一个单词存储起来
 */
export interface LyricLine {
    /**
     * 该行的所有单词
     * 如果是 LyRiC 等只能表达一行歌词的格式，这里就只会有一个单词
     */
    words: LyricWord[];
    /**
     * 该行是否为背景歌词行
     * 此选项只有作为 Lyricify Syllable 文件格式导入导出时才有意义
     */
    isBG?: boolean;
    /**
     * 该行是否为对唱歌词行（即歌词行靠右对齐）
     * 此选项只有作为 Lyricify Syllable 文件格式导入导出时才有意义
     */
    isDuet?: boolean;
};

"###;
