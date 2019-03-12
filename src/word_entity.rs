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
                if self.dictionary_form.starts_with(ARU) {
                    return [stem, String::from(ARU)].join("");
                }
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
                if self.dictionary_form.ends_with(GODAN_ARU_SHA_END) {
                    return [stem, String::from(SHARANAI)].join("");
                }
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
        }
    }

    // past form, ta form, past informal form
    pub fn perfective_form(&self) -> String {
        match self.word_type {
            WordType::VerbSuru => {
                let stem = self.get_suru_stem(self.dictionary_form.clone());
                [stem, String::from(SHITA)].join("")
            }
            WordType::VerbKuru => {
                let stem = self.get_kuru_stem(self.dictionary_form.clone());
                [stem, String::from(KITA)].join("")
            }
            WordType::VerbGodanARU => {
                let stem = self.get_godan_aru_stem(self.dictionary_form.clone());
                if self.dictionary_form.ends_with(GODAN_ARU_SHA_END) {
                    return [stem, String::from(SHATTA)].join("");
                }
                if self.dictionary_form.ends_with(GODAN_ARU_SA_END) {
                    return [stem, String::from(SATTA)].join("");
                }
                [stem, String::from(TTA)].join("")
            }
            WordType::VerbGodanB => {
                let stem = self.get_godan_b_stem(self.dictionary_form.clone());
                [stem, String::from(NDA)].join("")
            }
            WordType::VerbGodanG => {
                let stem = self.get_godan_g_stem(self.dictionary_form.clone());
                [stem, String::from(IDA)].join("")
            }
            WordType::VerbGodanK => {
                let stem = self.get_godan_k_stem(self.dictionary_form.clone());
                [stem, String::from(ITA)].join("")
            }
            WordType::VerbGodanKS => {
                let stem = self.get_godan_k_stem(self.dictionary_form.clone());
                [stem, String::from(TTA)].join("")
            }
            WordType::VerbGodanM => {
                let stem = self.get_godan_m_stem(self.dictionary_form.clone());
                [stem, String::from(NDA)].join("")
            }
            WordType::VerbGodanN => {
                let stem = self.get_godan_n_stem(self.dictionary_form.clone());
                [stem, String::from(NDA)].join("")
            }
            WordType::VerbGodanR => {
                let stem = self.get_godan_r_stem(self.dictionary_form.clone());
                [stem, String::from(TTA)].join("")
            }
            WordType::VerbGodanRI => {
                let stem = self.get_godan_ri_stem(self.dictionary_form.clone());
                if stem.is_empty() {
                    if self.dictionary_form.starts_with(ARU) {
                        return String::from(ATTA_KANJI);
                    }
                    return String::from(ATTA);
                }
                [stem, String::from(TTA)].join("")
            }
            WordType::VerbGodanS => {
                let stem = self.get_godan_s_stem(self.dictionary_form.clone());
                [stem, String::from(SHITA)].join("")
            }
            WordType::VerbGodanT => {
                let stem = self.get_godan_t_stem(self.dictionary_form.clone());
                [stem, String::from(TTA)].join("")
            }
            WordType::VerbGodanU => {
                let stem = self.get_godan_u_stem(self.dictionary_form.clone());
                [stem, String::from(TTA)].join("")
            }
            WordType::VerbGodanUS => {
                let stem = self.get_godan_u_stem(self.dictionary_form.clone());
                [stem, String::from(UTA)].join("")
            }
            WordType::VerbIchidan => {
                let stem = self.get_ichidan_stem(self.dictionary_form.clone());
                [stem, String::from(TA)].join("")
            }
            WordType::AdjectiveI => {
                let stem = self.get_adj_i_stem(self.dictionary_form.clone());
                [stem, String::from(KATTA)].join("")
            }
            WordType::AdjectiveNa => {
                let stem = self.get_adj_na_stem(self.dictionary_form.clone());
                [stem, String::from(DATTA)].join("")
            }
        }
    }

    // past negative form, ta negative form, past informal negative form
    pub fn perfective_negative_form(&self) -> String {
        [
            self.trim_string(self.imperfective_negative_form(), I_KANA),
            String::from(KATTA),
        ]
        .join("")
    }

    // masu form, long form, polite form
    pub fn formal_imperfective_form(&self) -> String {
        match self.word_type {
            WordType::VerbSuru => {
                let stem = self.get_suru_stem(self.dictionary_form.clone());
                [stem, String::from(SHIMASU)].join("")
            }
            WordType::VerbKuru => {
                let stem = self.get_kuru_stem(self.dictionary_form.clone());
                [stem, String::from(KIMASU)].join("")
            }
            WordType::VerbGodanARU => {
                let stem = self.get_godan_aru_stem(self.dictionary_form.clone());
                if self.dictionary_form.ends_with(GODAN_ARU_SHA_END) {
                    return [stem, String::from(SHAIMASU)].join("");
                }
                [stem, String::from(SAIMASU)].join("")
            }
            WordType::VerbGodanB => {
                let stem = self.get_godan_b_stem(self.dictionary_form.clone());
                [stem, String::from(BIMASU)].join("")
            }
            WordType::VerbGodanG => {
                let stem = self.get_godan_g_stem(self.dictionary_form.clone());
                [stem, String::from(GIMASU)].join("")
            }
            WordType::VerbGodanK | WordType::VerbGodanKS => {
                let stem = self.get_godan_k_stem(self.dictionary_form.clone());
                [stem, String::from(KIMASU)].join("")
            }
            WordType::VerbGodanM => {
                let stem = self.get_godan_m_stem(self.dictionary_form.clone());
                [stem, String::from(MIMASU)].join("")
            }
            WordType::VerbGodanN => {
                let stem = self.get_godan_n_stem(self.dictionary_form.clone());
                [stem, String::from(NIMASU)].join("")
            }
            WordType::VerbGodanR => {
                let stem = self.get_godan_r_stem(self.dictionary_form.clone());
                [stem, String::from(RIMASU)].join("")
            }
            WordType::VerbGodanRI => {
                let stem = self.get_godan_ri_stem(self.dictionary_form.clone());
                if stem.is_empty() {
                    if self.dictionary_form.starts_with(ARU) {
                        return [ARU_STEM, RIMASU].join("");
                    }
                    return [ARU_STEM_KANA, RIMASU].join("");
                }
                [stem, String::from(RIMASU)].join("")
            }
            WordType::VerbGodanS => {
                let stem = self.get_godan_s_stem(self.dictionary_form.clone());
                [stem, String::from(SHIMASU)].join("")
            }
            WordType::VerbGodanT => {
                let stem = self.get_godan_t_stem(self.dictionary_form.clone());
                [stem, String::from(CHIMASU)].join("")
            }
            WordType::VerbGodanU | WordType::VerbGodanUS => {
                let stem = self.get_godan_u_stem(self.dictionary_form.clone());
                [stem, String::from(IMASU)].join("")
            }
            WordType::VerbIchidan => {
                let stem = self.get_ichidan_stem(self.dictionary_form.clone());
                [stem, String::from(MASU)].join("")
            }
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
        }
    }

    // masu negative form, long negative form, polite negative form
    pub fn formal_imperfective_negative_form(&self) -> String {
        match self.word_type {
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
            _ => [
                self.trim_string(self.formal_imperfective_form(), GODAN_S_END),
                String::from(SEN),
            ]
            .join(""),
        }
    }

    // masu past form, long past form, polite past form
    pub fn formal_perfective_form(&self) -> String {
        match self.word_type {
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
            _ => [
                self.trim_string(self.formal_imperfective_form(), GODAN_S_END),
                String::from(SHITA),
            ]
            .join(""),
        }
    }

    // masu past negative form, long past negative form, polite past negative form
    pub fn formal_perfective_negative_form(&self) -> String {
        match self.word_type {
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
            _ => [
                self.trim_string(self.formal_imperfective_form(), GODAN_S_END),
                String::from(SEN),
                String::from(DESHITA),
            ]
            .join(""),
        }
    }

    // e form
    pub fn informal_potential_form(&self) -> String {
        match self.word_type {
            WordType::VerbSuru => {
                let stem = self.get_suru_stem(self.dictionary_form.clone());
                [stem, String::from(DEKIRU)].join("")
            }
            WordType::VerbKuru => {
                let stem = self.get_kuru_stem(self.dictionary_form.clone());
                [stem, String::from(KORARERU)].join("")
            }
            WordType::VerbGodanARU => {
                let stem = self.get_godan_aru_stem(self.dictionary_form.clone());
                if self.dictionary_form.ends_with(GODAN_ARU_SHA_END) {
                    return [stem, String::from(SHARERU)].join("");
                }
                [stem, String::from(SARERU)].join("")
            }
            WordType::VerbGodanB => {
                let stem = self.get_godan_b_stem(self.dictionary_form.clone());
                [stem, String::from(BERU)].join("")
            }
            WordType::VerbGodanG => {
                let stem = self.get_godan_g_stem(self.dictionary_form.clone());
                [stem, String::from(GERU)].join("")
            }
            WordType::VerbGodanK | WordType::VerbGodanKS => {
                let stem = self.get_godan_k_stem(self.dictionary_form.clone());
                [stem, String::from(KERU)].join("")
            }
            WordType::VerbGodanM => {
                let stem = self.get_godan_m_stem(self.dictionary_form.clone());
                [stem, String::from(MERU)].join("")
            }
            WordType::VerbGodanN => {
                let stem = self.get_godan_n_stem(self.dictionary_form.clone());
                [stem, String::from(NERU)].join("")
            }
            WordType::VerbGodanR => {
                let stem = self.get_godan_r_stem(self.dictionary_form.clone());
                [stem, String::from(RERU)].join("")
            }
            WordType::VerbGodanRI => {
                let stem = self.get_godan_ri_stem(self.dictionary_form.clone());
                if stem.is_empty() {
                    if self.dictionary_form.starts_with(ARU) {
                        return [ARU_STEM, RERU].join("");
                    }
                    return [ARU_STEM_KANA, RERU].join("");
                }
                [stem, String::from(RERU)].join("")
            }
            WordType::VerbGodanS => {
                let stem = self.get_godan_s_stem(self.dictionary_form.clone());
                [stem, String::from(SERU)].join("")
            }
            WordType::VerbGodanT => {
                let stem = self.get_godan_t_stem(self.dictionary_form.clone());
                [stem, String::from(TERU)].join("")
            }
            WordType::VerbGodanU | WordType::VerbGodanUS => {
                let stem = self.get_godan_u_stem(self.dictionary_form.clone());
                [stem, String::from(ERU)].join("")
            }
            WordType::VerbIchidan => [
                self.trim_string(self.dictionary_form.clone(), ICHIDAN_END),
                String::from(RARERU),
            ]
            .join(""),
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
        }
    }

    // e negative form
    pub fn informal_potential_negative_form(&self) -> String {
        match self.word_type {
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
            _ => [
                self.trim_string(self.informal_potential_form(), GODAN_ARU_RU_END),
                String::from(NAI),
            ]
            .join(""),
        }
    }

    // e past form
    pub fn informal_perfective_potential_form(&self) -> String {
        match self.word_type {
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
            _ => [
                self.trim_string(self.informal_potential_form(), GODAN_ARU_RU_END),
                String::from(TA),
            ]
            .join(""),
        }
    }

    // e past negative form
    pub fn informal_perfective_potential_negative_form(&self) -> String {
        match self.word_type {
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
            _ => [
                self.trim_string(self.informal_potential_negative_form(), I_KANA),
                String::from(KATTA),
            ]
            .join(""),
        }
    }

    // e masu form
    pub fn formal_potential_form(&self) -> String {
        match self.word_type {
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
            _ => [
                self.trim_string(self.informal_potential_form(), GODAN_ARU_RU_END),
                String::from(MASU),
            ]
            .join(""),
        }
    }

    // e masen form
    pub fn formal_potential_negative_form(&self) -> String {
        match self.word_type {
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
            _ => [
                self.trim_string(self.formal_potential_form(), GODAN_S_END),
                String::from(SEN),
            ]
            .join(""),
        }
    }

    // e past masu form
    pub fn formal_perfective_potential_form(&self) -> String {
        match self.word_type {
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
            _ => [
                self.trim_string(self.formal_potential_form(), GODAN_S_END),
                String::from(SHITA),
            ]
            .join(""),
        }
    }

    // e past masen form
    pub fn formal_perfective_potential_negative_form(&self) -> String {
        match self.word_type {
            WordType::AdjectiveI => String::from(NOT_APPLICABLE),
            WordType::AdjectiveNa => String::from(NOT_APPLICABLE),
            _ => [
                self.trim_string(self.formal_potential_form(), GODAN_S_END),
                String::from(SEN),
                String::from(DESHITA),
            ]
            .join(""),
        }
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
        self.trim_string(word, SURU)
    }

    fn get_kuru_stem(&self, word: String) -> String {
        if word.ends_with(KURU_KANA) {
            return self.trim_string(word, KURU_KANA);
        }
        self.trim_string(word, KURU)
    }

    fn get_godan_aru_stem(&self, word: String) -> String {
        if word.ends_with(GODAN_ARU_SHA_END) {
            return self.trim_string(word, GODAN_ARU_SHA_END);
        }
        if word.ends_with(GODAN_ARU_SA_END) {
            return self.trim_string(word, GODAN_ARU_SA_END);
        }
        self.trim_string(word, GODAN_ARU_RU_END)
    }

    fn get_godan_b_stem(&self, word: String) -> String {
        self.trim_string(word, GODAN_B_END)
    }

    fn get_godan_g_stem(&self, word: String) -> String {
        self.trim_string(word, GODAN_G_END)
    }

    fn get_godan_k_stem(&self, word: String) -> String {
        self.trim_string(word, GODAN_K_END)
    }

    fn get_godan_m_stem(&self, word: String) -> String {
        self.trim_string(word, GODAN_M_END)
    }

    fn get_godan_n_stem(&self, word: String) -> String {
        self.trim_string(word, GODAN_N_END)
    }

    fn get_godan_r_stem(&self, word: String) -> String {
        self.trim_string(word, GODAN_R_END)
    }

    fn get_godan_ri_stem(&self, word: String) -> String {
        if word.ends_with(ARU_KANA) {
            return self.trim_string(word, ARU_KANA);
        }
        self.trim_string(word, ARU)
    }

    fn get_godan_s_stem(&self, word: String) -> String {
        self.trim_string(word, GODAN_S_END)
    }

    fn get_godan_t_stem(&self, word: String) -> String {
        self.trim_string(word, GODAN_T_END)
    }

    fn get_godan_u_stem(&self, word: String) -> String {
        self.trim_string(word, GODAN_U_END)
    }

    fn get_ichidan_stem(&self, word: String) -> String {
        self.trim_string(word, ICHIDAN_END)
    }

    fn get_adj_i_stem(&self, word: String) -> String {
        let stem = self.trim_string(word, ADJ_I_END);
        if stem != I_KANA {
            return stem;
        }
        return String::from(ADJ_YOI_KANA);
    }

    fn get_adj_na_stem(&self, word: String) -> String {
        if word.ends_with(ADJ_NA_END) {
            return self.trim_string(word, ADJ_NA_END);
        }
        word
    }

    fn trim_string(&self, word: String, word_ending: &str) -> String {
        let word_length: usize = word.chars().count();
        let ending_length: usize = word_ending.chars().count();
        word[..word
            .char_indices()
            .nth(word_length - ending_length)
            .unwrap()
            .0]
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    struct TestWordEntity {
        word_entity: WordEntity,
        imperfective_form: String,
        imperfective_negative_form: String,
        perfective_form: String,
        perfective_negative_form: String,
        formal_imperfective_form: String,
        formal_imperfective_negative_form: String,
        formal_perfective_form: String,
        formal_perfective_negative_form: String,
        informal_potential_form: String,
        informal_potential_negative_form: String,
        informal_perfective_potential_form: String,
        informal_perfective_potential_negative_form: String,
        formal_potential_form: String,
        formal_potential_negative_form: String,
        formal_perfective_potential_form: String,
        formal_perfective_potential_negative_form: String,
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
                perfective_form: String::from(SHITA),
                perfective_negative_form: String::from("しなかった"),
                formal_imperfective_form: String::from(SHIMASU),
                formal_imperfective_negative_form: String::from("しません"),
                formal_perfective_form: String::from("しました"),
                formal_perfective_negative_form: String::from("しませんでした"),
                informal_potential_form: String::from("できる"),
                informal_potential_negative_form: String::from("できない"),
                informal_perfective_potential_form: String::from("できた"),
                informal_perfective_potential_negative_form: String::from("できなかった"),
                formal_potential_form: String::from("できます"),
                formal_potential_negative_form: String::from("できません"),
                formal_perfective_potential_form: String::from("できました"),
                formal_perfective_potential_negative_form: String::from("できませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("準備する"),
                    translation: String::from("to prepare"),
                    word_type: WordType::VerbSuru,
                },
                imperfective_form: String::from("準備する"),
                imperfective_negative_form: String::from("準備しない"),
                perfective_form: String::from("準備した"),
                perfective_negative_form: String::from("準備しなかった"),
                formal_imperfective_form: String::from("準備します"),
                formal_imperfective_negative_form: String::from("準備しません"),
                formal_perfective_form: String::from("準備しました"),
                formal_perfective_negative_form: String::from("準備しませんでした"),
                informal_potential_form: String::from("準備できる"),
                informal_potential_negative_form: String::from("準備できない"),
                informal_perfective_potential_form: String::from("準備できた"),
                informal_perfective_potential_negative_form: String::from(
                    "準備できなかった"
                ),
                formal_potential_form: String::from("準備できます"),
                formal_potential_negative_form: String::from("準備できません"),
                formal_perfective_potential_form: String::from("準備できました"),
                formal_perfective_potential_negative_form: String::from("準備できませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("来る"),
                    translation: String::from("to prepare"),
                    word_type: WordType::VerbKuru,
                },
                imperfective_form: String::from(KURU_KANA),
                imperfective_negative_form: String::from(KONAI),
                perfective_form: String::from(KITA),
                perfective_negative_form: String::from("こなかった"),
                formal_imperfective_form: String::from(KIMASU),
                formal_imperfective_negative_form: String::from("きません"),
                formal_perfective_form: String::from("きました"),
                formal_perfective_negative_form: String::from("きませんでした"),
                informal_potential_form: String::from(KORARERU),
                informal_potential_negative_form: String::from("こられない"),
                informal_perfective_potential_form: String::from("こられた"),
                informal_perfective_potential_negative_form: String::from("こられなかった"),
                formal_potential_form: String::from("こられます"),
                formal_potential_negative_form: String::from("こられません"),
                formal_perfective_potential_form: String::from("こられました"),
                formal_perfective_potential_negative_form: String::from("こられませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("くる"),
                    translation: String::from("to prepare"),
                    word_type: WordType::VerbKuru,
                },
                imperfective_form: String::from(KURU_KANA),
                imperfective_negative_form: String::from(KONAI),
                perfective_form: String::from(KITA),
                perfective_negative_form: String::from("こなかった"),
                formal_imperfective_form: String::from(KIMASU),
                formal_imperfective_negative_form: String::from("きません"),
                formal_perfective_form: String::from("きました"),
                formal_perfective_negative_form: String::from("きませんでした"),
                informal_potential_form: String::from(KORARERU),
                informal_potential_negative_form: String::from("こられない"),
                informal_perfective_potential_form: String::from("こられた"),
                informal_perfective_potential_negative_form: String::from("こられなかった"),
                formal_potential_form: String::from("こられます"),
                formal_potential_negative_form: String::from("こられません"),
                formal_perfective_potential_form: String::from("こられました"),
                formal_perfective_potential_negative_form: String::from("こられませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("下さる"),
                    translation: String::from("to give"),
                    word_type: WordType::VerbGodanARU,
                },
                imperfective_form: String::from("下さる"),
                imperfective_negative_form: String::from("下さらない"),
                perfective_form: String::from("下さった"),
                perfective_negative_form: String::from("下さらなかった"),
                formal_imperfective_form: String::from("下さいます"),
                formal_imperfective_negative_form: String::from("下さいません"),
                formal_perfective_form: String::from("下さいました"),
                formal_perfective_negative_form: String::from("下さいませんでした"),
                informal_potential_form: String::from("下される"),
                informal_potential_negative_form: String::from("下されない"),
                informal_perfective_potential_form: String::from("下された"),
                informal_perfective_potential_negative_form: String::from("下されなかった"),
                formal_potential_form: String::from("下されます"),
                formal_potential_negative_form: String::from("下されません"),
                formal_perfective_potential_form: String::from("下されました"),
                formal_perfective_potential_negative_form: String::from("下されませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("いらっしゃる"),
                    translation: String::from("to go"),
                    word_type: WordType::VerbGodanARU,
                },
                imperfective_form: String::from("いらっしゃる"),
                imperfective_negative_form: String::from("いらっしゃらない"),
                perfective_form: String::from("いらっしゃった"),
                perfective_negative_form: String::from("いらっしゃらなかった"),
                formal_imperfective_form: String::from("いらっしゃいます"),
                formal_imperfective_negative_form: String::from("いらっしゃいません"),
                formal_perfective_form: String::from("いらっしゃいました"),
                formal_perfective_negative_form: String::from(
                    "いらっしゃいませんでした"
                ),
                informal_potential_form: String::from("いらっしゃれる"),
                informal_potential_negative_form: String::from("いらっしゃれない"),
                informal_perfective_potential_form: String::from("いらっしゃれた"),
                informal_perfective_potential_negative_form: String::from(
                    "いらっしゃれなかった"
                ),
                formal_potential_form: String::from("いらっしゃれます"),
                formal_potential_negative_form: String::from("いらっしゃれません"),
                formal_perfective_potential_form: String::from("いらっしゃれました"),
                formal_perfective_potential_negative_form: String::from("いらっしゃれませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("呼ぶ"),
                    translation: String::from("to call"),
                    word_type: WordType::VerbGodanB,
                },
                imperfective_form: String::from("呼ぶ"),
                imperfective_negative_form: String::from("呼ばない"),
                perfective_form: String::from("呼んだ"),
                perfective_negative_form: String::from("呼ばなかった"),
                formal_imperfective_form: String::from("呼びます"),
                formal_imperfective_negative_form: String::from("呼びません"),
                formal_perfective_form: String::from("呼びました"),
                formal_perfective_negative_form: String::from("呼びませんでした"),
                informal_potential_form: String::from("呼べる"),
                informal_potential_negative_form: String::from("呼べない"),
                informal_perfective_potential_form: String::from("呼べた"),
                informal_perfective_potential_negative_form: String::from("呼べなかった"),
                formal_potential_form: String::from("呼べます"),
                formal_potential_negative_form: String::from("呼べません"),
                formal_perfective_potential_form: String::from("呼べました"),
                formal_perfective_potential_negative_form: String::from("呼べませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("泳ぐ"),
                    translation: String::from("to swim"),
                    word_type: WordType::VerbGodanG,
                },
                imperfective_form: String::from("泳ぐ"),
                imperfective_negative_form: String::from("泳がない"),
                perfective_form: String::from("泳いだ"),
                perfective_negative_form: String::from("泳がなかった"),
                formal_imperfective_form: String::from("泳ぎます"),
                formal_imperfective_negative_form: String::from("泳ぎません"),
                formal_perfective_form: String::from("泳ぎました"),
                formal_perfective_negative_form: String::from("泳ぎませんでした"),
                informal_potential_form: String::from("泳げる"),
                informal_potential_negative_form: String::from("泳げない"),
                informal_perfective_potential_form: String::from("泳げた"),
                informal_perfective_potential_negative_form: String::from("泳げなかった"),
                formal_potential_form: String::from("泳げます"),
                formal_potential_negative_form: String::from("泳げません"),
                formal_perfective_potential_form: String::from("泳げました"),
                formal_perfective_potential_negative_form: String::from("泳げませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("焼く"),
                    translation: String::from("to grill"),
                    word_type: WordType::VerbGodanK,
                },
                imperfective_form: String::from("焼く"),
                imperfective_negative_form: String::from("焼かない"),
                perfective_form: String::from("焼いた"),
                perfective_negative_form: String::from("焼かなかった"),
                formal_imperfective_form: String::from("焼きます"),
                formal_imperfective_negative_form: String::from("焼きません"),
                formal_perfective_form: String::from("焼きました"),
                formal_perfective_negative_form: String::from("焼きませんでした"),
                informal_potential_form: String::from("焼ける"),
                informal_potential_negative_form: String::from("焼けない"),
                informal_perfective_potential_form: String::from("焼けた"),
                informal_perfective_potential_negative_form: String::from("焼けなかった"),
                formal_potential_form: String::from("焼けます"),
                formal_potential_negative_form: String::from("焼けません"),
                formal_perfective_potential_form: String::from("焼けました"),
                formal_perfective_potential_negative_form: String::from("焼けませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("行く"),
                    translation: String::from("to go"),
                    word_type: WordType::VerbGodanKS,
                },
                imperfective_form: String::from("行く"),
                imperfective_negative_form: String::from("行かない"),
                perfective_form: String::from("行った"),
                perfective_negative_form: String::from("行かなかった"),
                formal_imperfective_form: String::from("行きます"),
                formal_imperfective_negative_form: String::from("行きません"),
                formal_perfective_form: String::from("行きました"),
                formal_perfective_negative_form: String::from("行きませんでした"),
                informal_potential_form: String::from("行ける"),
                informal_potential_negative_form: String::from("行けない"),
                informal_perfective_potential_form: String::from("行けた"),
                informal_perfective_potential_negative_form: String::from("行けなかった"),
                formal_potential_form: String::from("行けます"),
                formal_potential_negative_form: String::from("行けません"),
                formal_perfective_potential_form: String::from("行けました"),
                formal_perfective_potential_negative_form: String::from("行けませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("読む"),
                    translation: String::from("to read"),
                    word_type: WordType::VerbGodanM,
                },
                imperfective_form: String::from("読む"),
                imperfective_negative_form: String::from("読まない"),
                perfective_form: String::from("読んだ"),
                perfective_negative_form: String::from("読まなかった"),
                formal_imperfective_form: String::from("読みます"),
                formal_imperfective_negative_form: String::from("読みません"),
                formal_perfective_form: String::from("読みました"),
                formal_perfective_negative_form: String::from("読みませんでした"),
                informal_potential_form: String::from("読める"),
                informal_potential_negative_form: String::from("読めない"),
                informal_perfective_potential_form: String::from("読めた"),
                informal_perfective_potential_negative_form: String::from("読めなかった"),
                formal_potential_form: String::from("読めます"),
                formal_potential_negative_form: String::from("読めません"),
                formal_perfective_potential_form: String::from("読めました"),
                formal_perfective_potential_negative_form: String::from("読めませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("死ぬ"),
                    translation: String::from("to die"),
                    word_type: WordType::VerbGodanN,
                },
                imperfective_form: String::from("死ぬ"),
                imperfective_negative_form: String::from("死なない"),
                perfective_form: String::from("死んだ"),
                perfective_negative_form: String::from("死ななかった"),
                formal_imperfective_form: String::from("死にます"),
                formal_imperfective_negative_form: String::from("死にません"),
                formal_perfective_form: String::from("死にました"),
                formal_perfective_negative_form: String::from("死にませんでした"),
                informal_potential_form: String::from("死ねる"),
                informal_potential_negative_form: String::from("死ねない"),
                informal_perfective_potential_form: String::from("死ねた"),
                informal_perfective_potential_negative_form: String::from("死ねなかった"),
                formal_potential_form: String::from("死ねます"),
                formal_potential_negative_form: String::from("死ねません"),
                formal_perfective_potential_form: String::from("死ねました"),
                formal_perfective_potential_negative_form: String::from("死ねませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("走る"),
                    translation: String::from("to run"),
                    word_type: WordType::VerbGodanR,
                },
                imperfective_form: String::from("走る"),
                imperfective_negative_form: String::from("走らない"),
                perfective_form: String::from("走った"),
                perfective_negative_form: String::from("走らなかった"),
                formal_imperfective_form: String::from("走ります"),
                formal_imperfective_negative_form: String::from("走りません"),
                formal_perfective_form: String::from("走りました"),
                formal_perfective_negative_form: String::from("走りませんでした"),
                informal_potential_form: String::from("走れる"),
                informal_potential_negative_form: String::from("走れない"),
                informal_perfective_potential_form: String::from("走れた"),
                informal_perfective_potential_negative_form: String::from("走れなかった"),
                formal_potential_form: String::from("走れます"),
                formal_potential_negative_form: String::from("走れません"),
                formal_perfective_potential_form: String::from("走れました"),
                formal_perfective_potential_negative_form: String::from("走れませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("有る"),
                    translation: String::from("to exist"),
                    word_type: WordType::VerbGodanRI,
                },
                imperfective_form: String::from("有る"),
                imperfective_negative_form: String::from("ない"),
                perfective_form: String::from("有った"),
                perfective_negative_form: String::from("なかった"),
                formal_imperfective_form: String::from("有ります"),
                formal_imperfective_negative_form: String::from("有りません"),
                formal_perfective_form: String::from("有りました"),
                formal_perfective_negative_form: String::from("有りませんでした"),
                informal_potential_form: String::from("有れる"),
                informal_potential_negative_form: String::from("有れない"),
                informal_perfective_potential_form: String::from("有れた"),
                informal_perfective_potential_negative_form: String::from("有れなかった"),
                formal_potential_form: String::from("有れます"),
                formal_potential_negative_form: String::from("有れません"),
                formal_perfective_potential_form: String::from("有れました"),
                formal_perfective_potential_negative_form: String::from("有れませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("ある"),
                    translation: String::from("to show"),
                    word_type: WordType::VerbGodanRI,
                },
                imperfective_form: String::from("ある"),
                imperfective_negative_form: String::from("ない"),
                perfective_form: String::from("あった"),
                perfective_negative_form: String::from("なかった"),
                formal_imperfective_form: String::from("あります"),
                formal_imperfective_negative_form: String::from("ありません"),
                formal_perfective_form: String::from("ありました"),
                formal_perfective_negative_form: String::from("ありませんでした"),
                informal_potential_form: String::from("あれる"),
                informal_potential_negative_form: String::from("あれない"),
                informal_perfective_potential_form: String::from("あれた"),
                informal_perfective_potential_negative_form: String::from("あれなかった"),
                formal_potential_form: String::from("あれます"),
                formal_potential_negative_form: String::from("あれません"),
                formal_perfective_potential_form: String::from("あれました"),
                formal_perfective_potential_negative_form: String::from("あれませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("示す"),
                    translation: String::from("to show"),
                    word_type: WordType::VerbGodanS,
                },
                imperfective_form: String::from("示す"),
                imperfective_negative_form: String::from("示さない"),
                perfective_form: String::from("示した"),
                perfective_negative_form: String::from("示さなかった"),
                formal_imperfective_form: String::from("示します"),
                formal_imperfective_negative_form: String::from("示しません"),
                formal_perfective_form: String::from("示しました"),
                formal_perfective_negative_form: String::from("示しませんでした"),
                informal_potential_form: String::from("示せる"),
                informal_potential_negative_form: String::from("示せない"),
                informal_perfective_potential_form: String::from("示せた"),
                informal_perfective_potential_negative_form: String::from("示せなかった"),
                formal_potential_form: String::from("示せます"),
                formal_potential_negative_form: String::from("示せません"),
                formal_perfective_potential_form: String::from("示せました"),
                formal_perfective_potential_negative_form: String::from("示せませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("待つ"),
                    translation: String::from("to wait"),
                    word_type: WordType::VerbGodanT,
                },
                imperfective_form: String::from("待つ"),
                imperfective_negative_form: String::from("待たない"),
                perfective_form: String::from("待った"),
                perfective_negative_form: String::from("待たなかった"),
                formal_imperfective_form: String::from("待ちます"),
                formal_imperfective_negative_form: String::from("待ちません"),
                formal_perfective_form: String::from("待ちました"),
                formal_perfective_negative_form: String::from("待ちませんでした"),
                informal_potential_form: String::from("待てる"),
                informal_potential_negative_form: String::from("待てない"),
                informal_perfective_potential_form: String::from("待てた"),
                informal_perfective_potential_negative_form: String::from("待てなかった"),
                formal_potential_form: String::from("待てます"),
                formal_potential_negative_form: String::from("待てません"),
                formal_perfective_potential_form: String::from("待てました"),
                formal_perfective_potential_negative_form: String::from("待てませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("使う"),
                    translation: String::from("to use"),
                    word_type: WordType::VerbGodanU,
                },
                imperfective_form: String::from("使う"),
                imperfective_negative_form: String::from("使わない"),
                perfective_form: String::from("使った"),
                perfective_negative_form: String::from("使わなかった"),
                formal_imperfective_form: String::from("使います"),
                formal_imperfective_negative_form: String::from("使いません"),
                formal_perfective_form: String::from("使いました"),
                formal_perfective_negative_form: String::from("使いませんでした"),
                informal_potential_form: String::from("使える"),
                informal_potential_negative_form: String::from("使えない"),
                informal_perfective_potential_form: String::from("使えた"),
                informal_perfective_potential_negative_form: String::from("使えなかった"),
                formal_potential_form: String::from("使えます"),
                formal_potential_negative_form: String::from("使えません"),
                formal_perfective_potential_form: String::from("使えました"),
                formal_perfective_potential_negative_form: String::from("使えませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("問う"),
                    translation: String::from("to ask"),
                    word_type: WordType::VerbGodanUS,
                },
                imperfective_form: String::from("問う"),
                imperfective_negative_form: String::from("問わない"),
                perfective_form: String::from("問うた"),
                perfective_negative_form: String::from("問わなかった"),
                formal_imperfective_form: String::from("問います"),
                formal_imperfective_negative_form: String::from("問いません"),
                formal_perfective_form: String::from("問いました"),
                formal_perfective_negative_form: String::from("問いませんでした"),
                informal_potential_form: String::from("問える"),
                informal_potential_negative_form: String::from("問えない"),
                informal_perfective_potential_form: String::from("問えた"),
                informal_perfective_potential_negative_form: String::from("問えなかった"),
                formal_potential_form: String::from("問えます"),
                formal_potential_negative_form: String::from("問えません"),
                formal_perfective_potential_form: String::from("問えました"),
                formal_perfective_potential_negative_form: String::from("問えませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("食べる"),
                    translation: String::from("to eat"),
                    word_type: WordType::VerbIchidan,
                },
                imperfective_form: String::from("食べる"),
                imperfective_negative_form: String::from("食べない"),
                perfective_form: String::from("食べた"),
                perfective_negative_form: String::from("食べなかった"),
                formal_imperfective_form: String::from("食べます"),
                formal_imperfective_negative_form: String::from("食べません"),
                formal_perfective_form: String::from("食べました"),
                formal_perfective_negative_form: String::from("食べませんでした"),
                informal_potential_form: String::from("食べられる"),
                informal_potential_negative_form: String::from("食べられない"),
                informal_perfective_potential_form: String::from("食べられた"),
                informal_perfective_potential_negative_form: String::from(
                    "食べられなかった"
                ),
                formal_potential_form: String::from("食べられます"),
                formal_potential_negative_form: String::from("食べられません"),
                formal_perfective_potential_form: String::from("食べられました"),
                formal_perfective_potential_negative_form: String::from("食べられませんでした"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("痛い"),
                    translation: String::from("painful"),
                    word_type: WordType::AdjectiveI,
                },
                imperfective_form: String::from("痛い"),
                imperfective_negative_form: String::from("痛くない"),
                perfective_form: String::from("痛かった"),
                perfective_negative_form: String::from("痛くなかった"),
                formal_imperfective_form: String::from(NOT_APPLICABLE),
                formal_imperfective_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_form: String::from(NOT_APPLICABLE),
                formal_perfective_negative_form: String::from(NOT_APPLICABLE),
                informal_potential_form: String::from(NOT_APPLICABLE),
                informal_potential_negative_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_potential_form: String::from(NOT_APPLICABLE),
                formal_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("いい"),
                    translation: String::from("good"),
                    word_type: WordType::AdjectiveI,
                },
                imperfective_form: String::from("いい"),
                imperfective_negative_form: String::from("よくない"),
                perfective_form: String::from("よかった"),
                perfective_negative_form: String::from("よくなかった"),
                formal_imperfective_form: String::from(NOT_APPLICABLE),
                formal_imperfective_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_form: String::from(NOT_APPLICABLE),
                formal_perfective_negative_form: String::from(NOT_APPLICABLE),
                informal_potential_form: String::from(NOT_APPLICABLE),
                informal_potential_negative_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_potential_form: String::from(NOT_APPLICABLE),
                formal_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("よい"),
                    translation: String::from("good"),
                    word_type: WordType::AdjectiveI,
                },
                imperfective_form: String::from("よい"),
                imperfective_negative_form: String::from("よくない"),
                perfective_form: String::from("よかった"),
                perfective_negative_form: String::from("よくなかった"),
                formal_imperfective_form: String::from(NOT_APPLICABLE),
                formal_imperfective_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_form: String::from(NOT_APPLICABLE),
                formal_perfective_negative_form: String::from(NOT_APPLICABLE),
                informal_potential_form: String::from(NOT_APPLICABLE),
                informal_potential_negative_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_potential_form: String::from(NOT_APPLICABLE),
                formal_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("良い"),
                    translation: String::from("good"),
                    word_type: WordType::AdjectiveI,
                },
                imperfective_form: String::from("良い"),
                imperfective_negative_form: String::from("良くない"),
                perfective_form: String::from("良かった"),
                perfective_negative_form: String::from("良くなかった"),
                formal_imperfective_form: String::from(NOT_APPLICABLE),
                formal_imperfective_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_form: String::from(NOT_APPLICABLE),
                formal_perfective_negative_form: String::from(NOT_APPLICABLE),
                informal_potential_form: String::from(NOT_APPLICABLE),
                informal_potential_negative_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_potential_form: String::from(NOT_APPLICABLE),
                formal_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("簡単"),
                    translation: String::from("simple"),
                    word_type: WordType::AdjectiveNa,
                },
                imperfective_form: String::from("簡単"),
                imperfective_negative_form: String::from("簡単じゃない"),
                perfective_form: String::from("簡単だった"),
                perfective_negative_form: String::from("簡単じゃなかった"),
                formal_imperfective_form: String::from(NOT_APPLICABLE),
                formal_imperfective_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_form: String::from(NOT_APPLICABLE),
                formal_perfective_negative_form: String::from(NOT_APPLICABLE),
                informal_potential_form: String::from(NOT_APPLICABLE),
                informal_potential_negative_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_potential_form: String::from(NOT_APPLICABLE),
                formal_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("簡単な"),
                    translation: String::from("simple"),
                    word_type: WordType::AdjectiveNa,
                },
                imperfective_form: String::from("簡単"),
                imperfective_negative_form: String::from("簡単じゃない"),
                perfective_form: String::from("簡単だった"),
                perfective_negative_form: String::from("簡単じゃなかった"),
                formal_imperfective_form: String::from(NOT_APPLICABLE),
                formal_imperfective_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_form: String::from(NOT_APPLICABLE),
                formal_perfective_negative_form: String::from(NOT_APPLICABLE),
                informal_potential_form: String::from(NOT_APPLICABLE),
                informal_potential_negative_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_form: String::from(NOT_APPLICABLE),
                informal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_potential_form: String::from(NOT_APPLICABLE),
                formal_potential_negative_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_form: String::from(NOT_APPLICABLE),
                formal_perfective_potential_negative_form: String::from(NOT_APPLICABLE),
            },
        ];
    }

    #[bench]
    fn imperfective_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.imperfective_form(),
                    test_word.imperfective_form,
                )
            }
        })
    }

    #[bench]
    fn imperfective_negative_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.imperfective_negative_form(),
                    test_word.imperfective_negative_form,
                )
            }
        })
    }

    #[bench]
    fn perfective_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.perfective_form(),
                    test_word.perfective_form,
                )
            }
        })
    }

    #[bench]
    fn perfective_negative_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.perfective_negative_form(),
                    test_word.perfective_negative_form,
                )
            }
        })
    }

    #[bench]
    fn formal_imperfective_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.formal_imperfective_form(),
                    test_word.formal_imperfective_form,
                )
            }
        })
    }

    #[bench]
    fn formal_imperfective_negative_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.formal_imperfective_negative_form(),
                    test_word.formal_imperfective_negative_form,
                )
            }
        })
    }

    #[bench]
    fn formal_perfective_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.formal_perfective_form(),
                    test_word.formal_perfective_form,
                )
            }
        })
    }

    #[bench]
    fn formal_perfective_negative_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.formal_perfective_negative_form(),
                    test_word.formal_perfective_negative_form,
                )
            }
        })
    }

    #[bench]
    fn informal_potential_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.informal_potential_form(),
                    test_word.informal_potential_form,
                )
            }
        })
    }

    #[bench]
    fn informal_potential_negative_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.informal_potential_negative_form(),
                    test_word.informal_potential_negative_form,
                )
            }
        })
    }

    #[bench]
    fn informal_perfective_potential_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.informal_perfective_potential_form(),
                    test_word.informal_perfective_potential_form,
                )
            }
        })
    }

    #[bench]
    fn informal_perfective_potential_negative_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word
                        .word_entity
                        .informal_perfective_potential_negative_form(),
                    test_word.informal_perfective_potential_negative_form,
                )
            }
        })
    }

    #[bench]
    fn formal_potential_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.formal_potential_form(),
                    test_word.formal_potential_form,
                )
            }
        })
    }

    #[bench]
    fn formal_potential_negative_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.formal_potential_negative_form(),
                    test_word.formal_potential_negative_form,
                )
            }
        })
    }

    #[bench]
    fn formal_perfective_potential_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.formal_perfective_potential_form(),
                    test_word.formal_perfective_potential_form,
                )
            }
        })
    }

    #[bench]
    fn formal_perfective_potential_negative_form_test(b: &mut Bencher) {
        b.iter(|| {
            for test_word in TEST_WORDS.iter() {
                assert_eq!(
                    test_word.word_entity.formal_perfective_potential_negative_form(),
                    test_word.formal_perfective_potential_negative_form,
                )
            }
        })
    }
}
