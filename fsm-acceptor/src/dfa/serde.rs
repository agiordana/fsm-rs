use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::alphabet::Alphabet;
use crate::dfa::Dfa;

use super::State;

impl<A: Alphabet + Serialize> Serialize for Dfa<A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct DfaHelper<'a, A: Alphabet> {
            states: Vec<&'a State<A>>,
        }

        let helper = DfaHelper {
            states: self.states().collect(),
        };
        helper.serialize(serializer)
    }
}

impl<'de, A: Alphabet + Deserialize<'de>> Deserialize<'de> for Dfa<A> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct DfaHelper<A: Alphabet> {
            states: Vec<State<A>>,
        }

        let mut helper = DfaHelper::deserialize(deserializer)?;
        helper.states.sort_by_key(|state| state.id);
        let mut dfa = Dfa::new();
        let mut old2new = HashMap::with_capacity(helper.states.len());
        for state in &helper.states {
            let new_id = dfa.add_state(state.accepting);
            old2new.insert(state.id, new_id);
        }
        for (&old_from, &new_from) in &old2new {
            for (&symbol, old_to) in &helper.states[old_from].transitions {
                dfa.add_transition(new_from, symbol, old2new[old_to]);
            }
        }
        Ok(dfa)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_common::{decltype, generate_words};

    use super::*;

    #[test]
    fn test_dfa_serde() {
        let mut dfa = Dfa::new();
        let a = dfa.add_state(true);
        let b = dfa.add_state(false);
        dfa.add_transition(a, '1', a);
        dfa.add_transition(b, '0', b);
        dfa.add_transition(a, '0', b);
        dfa.add_transition(b, '0', a);

        let json = serde_json::to_string(&dfa).unwrap();
        let dfa2 = decltype(&dfa, serde_json::from_str(&json).unwrap());

        for word in generate_words(&['0', '1'], 10) {
            assert_eq!(dfa.accepts(word.chars()), dfa2.accepts(word.chars()));
        }
    }
}
