/*
  無声子音: k ky s sh t ty ch ts h f hy p py
  Rule 0 フィラーは無声化しない
  Rule 1 助動詞の「です」と「ます」の「す」が無声化
  Rule 2 動詞，助動詞，助詞の「し」は無声化しやすい
  Rule 3 続けて無声化しない
  Rule 4 アクセント核で無声化しない
  Rule 5 無声子音(k ky s sh t ty ch ts h f hy p py)に囲まれた「i」と「u」が無声化
         例外：s->s, s->sh, f->f, f->h, f->hy, h->f, h->h, h->hy
*/

use phf::{phf_set, Set};

pub const FILLER:&str="フィラー";
pub const DOUSHI:&str="動詞";
pub const JODOUSHI:&str="助動詞";
pub const JOSHI:&str="助詞";
pub const KANDOUSHI:&str="感動詞";
pub const TOUTEN:&str="、";
pub const QUESTION:&str="？";
pub const QUOTATION:&str="’";
pub const SHI:&str="シ";
pub const MA:&str="マ";
pub const DE:&str="デ";
pub const CHOUON:&str="ー";
pub const SU:&str="ス";

pub const candidate_list1:Set<&'static str>=phf_set!{
   "スィ",                    /* s i */
   "ス",                       /* s u */
};

pub const next_mora_list1:Set<&'static str>=phf_set!{
   "カ",                       /* k ky */
   "キ",
   "ク",
   "ケ",
   "コ",
   "タ",                       /* t ty ch ts */
   "チ",
   "ツ",
   "テ",
   "ト",
   "ハ",                       /* h f hy */
   "ヒ",
   "フ",
   "ヘ",
   "ホ",
   "パ",                       /* p py */
   "ピ",
   "プ",
   "ペ",
   "ポ",
};

pub const candidate_list2:Set<&'static str>=phf_set!{
   "フィ",                    /* f i */
   "ヒ",                       /* h i */
   "フ",                       /* f u */
};

pub const next_mora_list2:Set<&'static str>=phf_set!{
   "カ",                       /* k ky */
   "キ",
   "ク",
   "ケ",
   "コ",
   "サ",                       /* s sh */
   "シ",
   "ス",
   "セ",
   "ソ",
   "タ",                       /* t ty ch ts */
   "チ",
   "ツ",
   "テ",
   "ト",
   "パ",                       /* p py */
   "ピ",
   "プ",
   "ペ",
   "ポ",
};

pub const candidate_list3:Set<&'static str>=phf_set!{
   "キュ",                    /* ky u */
   "シュ",                    /* sh u */
   "チュ",                    /* ch u */
   "ツィ",                    /* ts i */
   "ヒュ",                    /* hy u */
   "ピュ",                    /* py u */
   "テュ",                    /* ty u */
   "トゥ",                    /* t u */
   "ティ",                    /* t i */
   "キ",                       /* k i */
   "ク",                       /* k u */
   "シ",                       /* sh i */
   "チ",                       /* ch i */
   "ツ",                       /* ts u */
   "ピ",                       /* p i */
   "プ",                       /* p u */
};

pub const next_mora_list3:Set<&'static str>=phf_set!{
   "カ",                       /* k ky */
   "キ",
   "ク",
   "ケ",
   "コ",
   "サ",                       /* s sh */
   "シ",
   "ス",
   "セ",
   "ソ",
   "タ",                       /* t ty ch ts */
   "チ",
   "ツ",
   "テ",
   "ト",
   "ハ",                       /* h f hy */
   "ヒ",
   "フ",
   "ヘ",
   "ホ",
   "パ",                       /* p py */
   "ピ",
   "プ",
   "ペ",
   "ポ",
};

pub const mora_list:Set<&'static str>=phf_set!{
   "ヴョ",
   "ヴュ",
   "ヴャ",
   "ヴォ",
   "ヴェ",
   "ヴィ",
   "ヴァ",
   "ヴ",
   "ン",
   "ヲ",
   "ヱ",
   "ヰ",
   "ワ",
   "ロ",
   "レ",
   "ル",
   "リョ",
   "リュ",
   "リャ",
   "リェ",
   "リ",
   "ラ",
   "ヨ",
   "ョ",
   "ユ",
   "ュ",
   "ヤ",
   "ャ",
   "モ",
   "メ",
   "ム",
   "ミョ",
   "ミュ",
   "ミャ",
   "ミェ",
   "ミ",
   "マ",
   "ポ",
   "ボ",
   "ホ",
   "ペ",
   "ベ",
   "ヘ",
   "プ",
   "ブ",
   "フォ",
   "フェ",
   "フィ",
   "ファ",
   "フ",
   "ピョ",
   "ピュ",
   "ピャ",
   "ピェ",
   "ピ",
   "ビョ",
   "ビュ",
   "ビャ",
   "ビェ",
   "ビ",
   "ヒョ",
   "ヒュ",
   "ヒャ",
   "ヒェ",
   "ヒ",
   "パ",
   "バ",
   "ハ",
   "ノ",
   "ネ",
   "ヌ",
   "ニョ",
   "ニュ",
   "ニャ",
   "ニェ",
   "ニ",
   "ナ",
   "ドゥ",
   "ド",
   "トゥ",
   "ト",
   "デョ",
   "デュ",
   "デャ",
   "ディ",
   "デ",
   "テョ",
   "テュ",
   "テャ",
   "ティ",
   "テ",
   "ヅ",
   "ツォ",
   "ツェ",
   "ツィ",
   "ツァ",
   "ツ",
   "ッ",
   "ヂ",
   "チョ",
   "チュ",
   "チャ",
   "チェ",
   "チ",
   "ダ",
   "タ",
   "ゾ",
   "ソ",
   "ゼ",
   "セ",
   "ズィ",
   "ズ",
   "スィ",
   "ス",
   "ジョ",
   "ジュ",
   "ジャ",
   "ジェ",
   "ジ",
   "ショ",
   "シュ",
   "シャ",
   "シェ",
   "シ",
   "ザ",
   "サ",
   "ゴ",
   "コ",
   "ゲ",
   "ケ",
   "グ",
   "ク",
   "ギョ",
   "ギュ",
   "ギャ",
   "ギェ",
   "ギ",
   "キョ",
   "キュ",
   "キャ",
   "キェ",
   "キ",
   "ガ",
   "カ",
   "オ",
   "ォ",
   "エ",
   "ェ",
   "ウォ",
   "ウェ",
   "ウィ",
   "ウ",
   "ゥ",
   "イェ",
   "イ",
   "ィ",
   "ア",
   "ァ",
   "ー",
};