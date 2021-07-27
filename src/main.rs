fn main() {
    let books = vec![
        "this is my first book",
        "this is my second book",
        "this is my third thingy that is totally awesome",
    ];
    let m = Matcher::new(books);

    let res = m.search("book");
    assert_eq!(res, vec![0, 1]);

    let res = m.search("thingy");
    assert_eq!(res, vec![2]);

    let res = m.search("empty");
    assert_eq!(res, vec![]);
}

#[derive(Debug)]
pub struct Matcher {
    states: Box<[State; 256]>,
}

impl Matcher {
    pub fn new(texts: Vec<&str>) -> Matcher {
        let mut states = make_states();

        for (i, text) in texts.iter().enumerate() {
            let words: Vec<&str> = text.split_whitespace().collect();
            for word in words {
                let mut state = &mut Default::default();
                let mut cur_states = &mut states;

                for ch in word.bytes() {
                    state = &mut cur_states[ch as usize];

                    if state.trans.is_none() {
                        state.trans = Some(make_states());
                    }
                    cur_states = state.trans.as_mut().unwrap();
                }

                state.match_indexes.push(i);
            }
        }

        Matcher { states }
    }

    pub fn search(&self, word: &str) -> Vec<usize> {
        let mut state;
        let mut match_index = vec![];
        let mut cur_states = &*self.states;

        for byte in word.bytes() {
            state = &cur_states[byte as usize];

            if let Some(trans) = &state.trans {
                match_index = state.match_indexes.clone();
                cur_states = trans;
                continue;
            }
            return vec![];
        }

        match_index
    }
}

#[derive(Default, Debug)]
struct State {
    trans: Option<Box<[State; 256]>>,
    match_indexes: Vec<usize>,
}

fn make_states() -> Box<[State; 256]> {
    let states = [
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
        State {
            trans: None,
            match_indexes: vec![],
        },
    ];

    Box::new(states)
}
