use super::constant::*;

#[derive(Debug)]
// The type is following EDICT classification
// http://nihongo.monash.edu//jmdict_dtd_h.html
pub enum WordType {
    VerbSuru,
    VerbKuru,
    VerbGodanARU,
    VerbGodanB,
    VerbGodanG,
    VerbGodanK,
    VerbGodanKS, // S Special
    VerbGodanM,
    VerbGodanN,
    VerbGodanR,
    VerbGodanRI, // I Irregular
    VerbGodanS,
    VerbGodanT,
    VerbGodanU,
    VerbGodanUS, // U Special
    VerbIchidan,
    AdjectiveI,
    AdjectiveNa,
    //AdjectiveTaru, // TODO: implement this
    //AdverbTo, // TODO: implement this
    //Auxiliary, // TODO: implement this
    //AuxiliaryVerb, // TODO: implement this
    //AuxiliaryAdjective, // TODO: implement this
}

#[derive(Debug)]
pub struct WordEntity {
    pub dictionary_form: String,
    pub translation: String,
    pub word_type: WordType,
}

impl WordEntity {
    // dictionary form, short form, informal form
    pub fn imperfective_form(&self) -> String {
        match self.word_type {
            WordType::VerbSuru => {
                if self.dictionary_form.ends_with(SURU) {
                    return self.dictionary_form.clone();
                }
                [self.dictionary_form.clone(), String::from(SURU)].join("")
            }
            WordType::VerbKuru => {
                let stem = self.get_kuru_stem(self.dictionary_form.clone());
                [stem, String::from(KURU_KANA)].join("")
            }
            WordType::VerbGodanRI => {
                let stem = self.get_godan_ri_stem(self.dictionary_form.clone());
                [stem, String::from(ARU_KANA)].join("")
            }
            WordType::AdjectiveNa => self.get_adj_na_stem(self.dictionary_form.clone()),
            _ => self.dictionary_form.clone(),
        }
    }

    // negative form, short negative form, informal negative form
    pub fn imperfective_negative_form(&self) -> String {
        match self.word_type {
            WordType::VerbSuru => {
                let stem = self.get_suru_stem(self.dictionary_form.clone());
                [stem, String::from(SHINAI)].join("")
            }
            WordType::VerbKuru => {
                let stem = self.get_kuru_stem(self.dictionary_form.clone());
                [stem, String::from(KONAI)].join("")
            }
            WordType::VerbGodanARU => {
                let stem = self.get_godan_aru_stem(self.dictionary_form.clone());
                [stem, String::from(SARANAI)].join("")
            }
            WordType::VerbGodanB => {
                let stem = self.get_godan_b_stem(self.dictionary_form.clone());
                [stem, String::from(BANAI)].join("")
            }
            WordType::VerbGodanG => {
                let stem = self.get_godan_g_stem(self.dictionary_form.clone());
                [stem, String::from(GANAI)].join("")
            }
            WordType::VerbGodanK | WordType::VerbGodanKS => {
                let stem = self.get_godan_k_stem(self.dictionary_form.clone());
                [stem, String::from(KANAI)].join("")
            }
            WordType::VerbGodanM => {
                let stem = self.get_godan_m_stem(self.dictionary_form.clone());
                [stem, String::from(MANAI)].join("")
            }
            WordType::VerbGodanN => {
                let stem = self.get_godan_n_stem(self.dictionary_form.clone());
                [stem, String::from(NANAI)].join("")
            }
            WordType::VerbGodanR => {
                let stem = self.get_godan_r_stem(self.dictionary_form.clone());
                [stem, String::from(RANAI)].join("")
            }
            WordType::VerbGodanRI => {
                let stem = self.get_godan_ri_stem(self.dictionary_form.clone());
                [stem, String::from(NAI)].join("")
            }
            WordType::VerbGodanS => {
                let stem = self.get_godan_s_stem(self.dictionary_form.clone());
                [stem, String::from(SANAI)].join("")
            }
            WordType::VerbGodanT => {
                let stem = self.get_godan_t_stem(self.dictionary_form.clone());
                [stem, String::from(TANAI)].join("")
            }
            WordType::VerbGodanU | WordType::VerbGodanUS => {
                let stem = self.get_godan_u_stem(self.dictionary_form.clone());
                [stem, String::from(WANAI)].join("")
            }
            WordType::VerbIchidan => {
                let stem = self.get_ichidan_stem(self.dictionary_form.clone());
                [stem, String::from(NAI)].join("")
            }
            WordType::AdjectiveI => {
                let stem = self.get_adj_i_stem(self.dictionary_form.clone());
                [stem, String::from(KUNAI)].join("")
            }
            WordType::AdjectiveNa => {
                let stem = self.get_adj_na_stem(self.dictionary_form.clone());
                [stem, String::from(JANAI)].join("")
            }
            _ => String::from("TODO"),
        }
    }

    // past form, ta form, past informal form
    pub fn perfective_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // past negative form, ta negative form, past informal negative form
    pub fn perfective_negative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // masu form, long form, polite form
    pub fn formal_imperfective_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // masu negative form, long negative form, polite negative form
    pub fn formal_imperfective_negative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // masu past form, long past form, polite past form
    pub fn formal_perfective_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // masu past negative form, long past negative form, polite past negative form
    pub fn formal_perfective_negative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // e form
    pub fn informal_potential_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // e negative form
    pub fn informal_potential_negative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // e past form
    pub fn informal_perfective_potential_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // e past negative form
    pub fn informal_perfective_potential_negative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // e masu form
    pub fn formal_potential_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // e masen form
    pub fn formal_potential_negative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // e past masu form
    pub fn formal_perfective_potential_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // e past masen form
    pub fn formal_perfective_potential_negative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    pub fn informal_imperative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    pub fn informal_imperative_negative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    pub fn formal_imperative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    pub fn formal_imperative_negative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    pub fn volitional_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    pub fn volitional_negative_form(&self) -> String {
        // TODO: implement this
        self.dictionary_form.clone()
    }

    // others

    // stem
    fn get_suru_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(SURU))
    }

    fn get_kuru_stem(&self, word: String) -> String {
        if word.ends_with(KURU_KANA) {
            return String::from(word.trim_end_matches(KURU_KANA));
        }
        String::from(word.trim_end_matches(KURU))
    }

    fn get_godan_aru_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(GODAN_ARU_END))
    }

    fn get_godan_b_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(GODAN_B_END))
    }

    fn get_godan_g_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(GODAN_G_END))
    }

    fn get_godan_k_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(GODAN_K_END))
    }

    fn get_godan_m_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(GODAN_M_END))
    }

    fn get_godan_n_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(GODAN_N_END))
    }

    fn get_godan_r_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(GODAN_R_END))
    }

    fn get_godan_ri_stem(&self, word: String) -> String {
        if word.ends_with(ARU_KANA) {
            return String::from(word.trim_end_matches(ARU_KANA));
        }
        String::from(word.trim_end_matches(ARU))
    }

    fn get_godan_s_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(GODAN_S_END))
    }

    fn get_godan_t_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(GODAN_T_END))
    }

    fn get_godan_u_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(GODAN_U_END))
    }

    fn get_ichidan_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(ICHIDAN_END))
    }

    fn get_adj_i_stem(&self, word: String) -> String {
        String::from(word.trim_end_matches(ADJ_I_END))
    }

    fn get_adj_na_stem(&self, word: String) -> String {
        if word.ends_with(ADJ_NA_END) {
            return String::from(word.trim_end_matches(ADJ_NA_END));
        }
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestWordEntity {
        word_entity: WordEntity,
        imperfective_form: String,
        imperfective_negative_form: String,
    }

    lazy_static! {
        static ref TEST_WORDS: Vec<TestWordEntity> = vec![
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from(SURU),
                    translation: String::from("to do"),
                    word_type: WordType::VerbSuru,
                },
                imperfective_form: String::from(SURU),
                imperfective_negative_form: String::from(SHINAI),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("準備する"),
                    translation: String::from("to prepare"),
                    word_type: WordType::VerbSuru,
                },
                imperfective_form: String::from("準備する"),
                imperfective_negative_form: String::from("準備しない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("来る"),
                    translation: String::from("to prepare"),
                    word_type: WordType::VerbKuru,
                },
                imperfective_form: String::from("くる"),
                imperfective_negative_form: String::from("こない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("くる"),
                    translation: String::from("to prepare"),
                    word_type: WordType::VerbKuru,
                },
                imperfective_form: String::from("くる"),
                imperfective_negative_form: String::from("こない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("下さる"),
                    translation: String::from("to give"),
                    word_type: WordType::VerbGodanARU,
                },
                imperfective_form: String::from("下さる"),
                imperfective_negative_form: String::from("下さらない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("呼ぶ"),
                    translation: String::from("to call"),
                    word_type: WordType::VerbGodanB,
                },
                imperfective_form: String::from("呼ぶ"),
                imperfective_negative_form: String::from("呼ばない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("泳ぐ"),
                    translation: String::from("to swim"),
                    word_type: WordType::VerbGodanG,
                },
                imperfective_form: String::from("泳ぐ"),
                imperfective_negative_form: String::from("泳がない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("焼く"),
                    translation: String::from("to grill"),
                    word_type: WordType::VerbGodanK,
                },
                imperfective_form: String::from("焼く"),
                imperfective_negative_form: String::from("焼かない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("行く"),
                    translation: String::from("to go"),
                    word_type: WordType::VerbGodanKS,
                },
                imperfective_form: String::from("行く"),
                imperfective_negative_form: String::from("行かない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("読む"),
                    translation: String::from("to read"),
                    word_type: WordType::VerbGodanM,
                },
                imperfective_form: String::from("読む"),
                imperfective_negative_form: String::from("読まない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("死ぬ"),
                    translation: String::from("to die"),
                    word_type: WordType::VerbGodanN,
                },
                imperfective_form: String::from("死ぬ"),
                imperfective_negative_form: String::from("死なない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("走る"),
                    translation: String::from("to run"),
                    word_type: WordType::VerbGodanR,
                },
                imperfective_form: String::from("走る"),
                imperfective_negative_form: String::from("走らない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("有る"),
                    translation: String::from("to exist"),
                    word_type: WordType::VerbGodanRI,
                },
                imperfective_form: String::from("ある"),
                imperfective_negative_form: String::from("ない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("ある"),
                    translation: String::from("to show"),
                    word_type: WordType::VerbGodanRI,
                },
                imperfective_form: String::from("ある"),
                imperfective_negative_form: String::from("ない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("示す"),
                    translation: String::from("to show"),
                    word_type: WordType::VerbGodanS,
                },
                imperfective_form: String::from("示す"),
                imperfective_negative_form: String::from("示さない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("待つ"),
                    translation: String::from("to wait"),
                    word_type: WordType::VerbGodanT,
                },
                imperfective_form: String::from("待つ"),
                imperfective_negative_form: String::from("待たない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("使う"),
                    translation: String::from("to use"),
                    word_type: WordType::VerbGodanU,
                },
                imperfective_form: String::from("使う"),
                imperfective_negative_form: String::from("使わない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("問う"),
                    translation: String::from("to ask"),
                    word_type: WordType::VerbGodanUS,
                },
                imperfective_form: String::from("問う"),
                imperfective_negative_form: String::from("問わない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("食べる"),
                    translation: String::from("to eat"),
                    word_type: WordType::VerbIchidan,
                },
                imperfective_form: String::from("食べる"),
                imperfective_negative_form: String::from("食べない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("痛い"),
                    translation: String::from("painful"),
                    word_type: WordType::AdjectiveI,
                },
                imperfective_form: String::from("痛い"),
                imperfective_negative_form: String::from("痛くない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("簡単"),
                    translation: String::from("simple"),
                    word_type: WordType::AdjectiveNa,
                },
                imperfective_form: String::from("簡単"),
                imperfective_negative_form: String::from("簡単じゃない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("簡単な"),
                    translation: String::from("simple"),
                    word_type: WordType::AdjectiveNa,
                },
                imperfective_form: String::from("簡単"),
                imperfective_negative_form: String::from("簡単じゃない"),
            },
        ];
    }

    #[test]
    fn imperfective_form_test() {
        for test_word in TEST_WORDS.iter() {
            assert_eq!(
                test_word.word_entity.imperfective_form(),
                test_word.imperfective_form,
            )
        }
    }

    #[test]
    fn imperfective_negative_form_test() {
        for test_word in TEST_WORDS.iter() {
            assert_eq!(
                test_word.word_entity.imperfective_negative_form(),
                test_word.imperfective_negative_form,
            )
        }
    }
}
