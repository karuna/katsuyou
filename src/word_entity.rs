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
pub struct WordEntity<'a> {
    pub dictionary_form: &'a str,
    pub word_type: WordType,
}

impl<'a> WordEntity<'a> {
    // dictionary form, short form, informal form
    pub fn imperfective_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // negative form, short negative form, informal negative form
    pub fn imperfective_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // past form, ta form, past informal form
    pub fn perfective_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // past negative form, ta negative form, past informal negative form
    pub fn perfective_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // masu form, long form, polite form
    pub fn formal_imperfective_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // masu negative form, long negative form, polite negative form
    pub fn formal_imperfective_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // masu past form, long past form, polite past form
    pub fn formal_perfective_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // masu past negative form, long past negative form, polite past negative form
    pub fn formal_perfective_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // e form
    pub fn informal_potential_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // e negative form
    pub fn informal_potential_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // e past form
    pub fn informal_perfective_potential_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // e past negative form
    pub fn informal_perfective_potential_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // e masu form
    pub fn formal_potential_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // e masen form
    pub fn formal_potential_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // e past masu form
    pub fn formal_perfective_potential_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // e past masen form
    pub fn formal_perfective_potential_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    pub fn informal_imperative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    pub fn informal_imperative_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    pub fn formal_imperative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    pub fn formal_imperative_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    pub fn volitional_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    pub fn volitional_negative_form(&self) -> &'a str {
        // TODO: implement this
        self.dictionary_form
    }

    // others
}
