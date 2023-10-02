use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::alphabet::Alphabet;
use crate::mealy::Mealy;

use super::State;

impl<I: Alphabet + Serialize, O: Alphabet + Serialize> Serialize for Mealy<I, O> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(rename = "Moore")]
        struct MealyHelper<'a, I: Alphabet, O: Alphabet> {
            states: Vec<&'a State<I,O>>,
        }

        let helper = MealyHelper {
            states: self.states().collect(),
        };
        helper.serialize(serializer)
    }
}

impl<'de, I: Alphabet + Deserialize<'de>, O: Alphabet + Deserialize<'de>> Deserialize<'de> for Mealy<I, O> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "Moore")]
        struct MealyHelper<I: Alphabet, O: Alphabet> {
            states: Vec<State<I, O>>,
        }

        let helper = MealyHelper::deserialize(deserializer)?;
        let mut mealy = Mealy::new();
        let old2new: HashMap<_, _> = helper
            .states
            .iter()
            .map(|state| (state.id, mealy.add_state()))
            .collect();
        for old_from_state in &helper.states {
            let new_from = old2new[&old_from_state.id];
            for (symbol, old_to, out) in old_from_state.transitions() {
                mealy.add_transition(new_from, symbol, old2new[&old_to], out);
            }
        }
        Ok(mealy)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_common::{decltype, generate_strings};

    use super::*;

    #[test]
    fn test_mealy_serde() {
        let mut mealy = Moore::new();
        let a = mealy.add_state();
        let b = mealy.add_state();
        mealy.add_transition(a, '1', a, '2');
        mealy.add_transition(b, '0', b, '3');
        mealy.add_transition(a, '0', b, '4');
        mealy.add_transition(b, '0', a, '0');

        let json = serde_json::to_string(&mealy).unwrap();
        let mealy2 = decltype(&mealy, serde_json::from_str(&json).unwrap());

        for word in generate_strings(&['0', '1'], 10) {
            assert_eq!(mealy.accepts(word.chars()), mealy2.accepts(word.chars()));
        }
    }
}
