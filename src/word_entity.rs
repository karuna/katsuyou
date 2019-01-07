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
        self.dictionary_form.clone()
    }

    // negative form, short negative form, informal negative form
    pub fn imperfective_negative_form(&self) -> String {
        match self.word_type {
            WordType::VerbSuru => {
                let stem = String::from(self.dictionary_form.clone().trim_end_matches("する"));
                [stem, String::from("しない")].join("")
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
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestWordEntity {
        word_entity: WordEntity,
        imperfective_negative_form: String,
    }

    lazy_static! {
        static ref TEST_WORDS: Vec<TestWordEntity> = vec![TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("する"),
                    translation: String::from("to do"),
                    word_type: WordType::VerbSuru,
                },
                imperfective_negative_form: String::from("しない"),
            },
            TestWordEntity {
                word_entity: WordEntity {
                    dictionary_form: String::from("準備する"),
                    translation: String::from("to prepare"),
                    word_type: WordType::VerbSuru,
                },
                imperfective_negative_form: String::from("準備しない"),
            },
        ];
    }

    #[test]
    fn imperfective_negative_form_test() {
        for test_word in TEST_WORDS {
            assert_eq!(
                test_word.word_entity.imperfective_negative_form(),
                test_word.imperfective_negative_form
            )
        }
    }
}
