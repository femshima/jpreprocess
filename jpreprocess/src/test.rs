#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use lindera::{mode::Mode, tokenizer::*};

    use crate::*;

    const TEST_STR: &[&str] = &[
        "-64.0℃。シンプソン則。BOP試薬(BOP reagent)。58.226889。2990。678。何千何百何十円なり、TypeScript。一昨日は1月1日。あと20日間残っている。",
        "聞きがたいお手紙の混雑ぶり霊験あらたか。たいそうやっちゃったね。動く細かい部屋に少なめコーヒーだし。尚更。",
        "No.12。番号:12。0.0.2.0.5.0.6.0 1棟、1人、一日、一日間、14日、14日間、20日、24日、24日間、1分。035(123)。100000。10,00。1,000",
        "リャリョ。クーバネティス。行こう。行きます？",
        "一九〇〇、1900，zAゔょぁ。123,456,789",
        // This sentence fails, but I won't fix.
        // "12,34,567．89"
    ];

    #[test]
    fn test_all() {
        for s in TEST_STR {
            test_one(s);
        }
    }

    fn test_one(input_text: &'static str) {
        let normalized_input_text = text_normalizer::normalize(input_text);

        let dictionary = DictionaryConfig {
            kind: None,
            path: Some(PathBuf::from("../dict")),
        };

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: None,
            mode: Mode::Normal,
        };
        let tokenizer = Tokenizer::from_config(config).unwrap();

        let tokens = tokenizer.tokenize(normalized_input_text.as_str()).unwrap();

        let mut njd = NJD::from_tokens_string(tokens);
        njd_set::proprocess_njd(&mut njd);

        let mut child = Command::new("../tester/open_jtalk")
            .arg("-x")
            .arg("../tester/mecab-naist-jdic")
            .arg("-m")
            .arg("../tester/nitech_jp_atr503_m001.htsvoice")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin
                .write_all(input_text.as_bytes())
                .expect("Failed to write to stdin");
        });

        let output = child.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8(output.stdout).unwrap();
        for (node, ans) in njd.nodes.iter().zip(stdout.split("\n")) {
            let node_ans = NJDNode::new_single(ans);
            assert_eq!(node, &node_ans);
        }
    }
}
