pub const njd_set_long_vowel_kanji_range:&[u8] = &[
   2, 0xC0, 0xDF,
   3, 0xE0, 0xEF,
   4, 0xF0, 0xF7,
   0xFF, 0xFF, 0xFF//changed from -1 to 0xFF
];

pub const njd_set_long_vowel_table:&[&str] = &[
   "エイ", "エー",
   "ケイ", "ケー",
   "セイ", "セー",
   "テイ", "テー",
   "ネイ", "ネー",
   "ヘイ", "ヘー",
   "メイ", "メー",
   "レイ", "レー",
   "ゲイ", "ゲー",
   "ゼイ", "ゼー",
   "デイ", "デー",
   "ベイ", "ベー",
   "ペイ", "ペー",
   "ヱイ", "ヱー",
   "\0", "\0"
];
