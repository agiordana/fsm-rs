use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::alphabet::Alphabet;
use crate::moore::Moore;

use super::State;

impl<I: Alphabet + Serialize, O: Alphabet + Serialize> Serialize for Moore<I, O> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(rename = "Moore")]
        struct MooreHelper<'a, I: Alphabet, O: Alphabet> {
            states: Vec<&'a State<I,O>>,
        }

        let helper = MooreHelper {
            states: self.states().collect(),
        };
        helper.serialize(serializer)
    }
}

impl<'de, I: Alphabet + Deserialize<'de>, O: Alphabet + Deserialize<'de>> Deserialize<'de> for Moore<I, O> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "Moore")]
        struct MooreHelper<I: Alphabet, O: Alphabet> {
            states: Vec<State<I, O>>,
        }

        let helper = MooreHelper::deserialize(deserializer)?;
        let mut moore = Moore::new();
        let old2new: HashMap<_, _> = helper
            .states
            .iter()
            .map(|state| (state.id, moore.add_state(state.output)))
            .collect();
        for old_from_state in &helper.states {
            let new_from = old2new[&old_from_state.id];
            for (symbol, old_to) in old_from_state.transitions() {
                moore.add_transition(new_from, symbol, old2new[&old_to]);
            }
        }
        Ok(moore)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_common::{decltype, generate_strings};

    use super::*;

    #[test]
    fn test_moore_serde() {
        let mut moore = Moore::new();
        let a = moore.add_state(true);
        let b = moore.add_state(false);
        moore.add_transition(a, '1', a);
        moore.add_transition(b, '0', b);
        moore.add_transition(a, '0', b);
        moore.add_transition(b, '0', a);

        let json = serde_json::to_string(&moore).unwrap();
        let moore2 = decltype(&moore, serde_json::from_str(&json).unwrap());

        for word in generate_strings(&['0', '1'], 10) {
            assert_eq!(moore.accepts(word.chars()), moore2.accepts(word.chars()));
        }
    }
}
