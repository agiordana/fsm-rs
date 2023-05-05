use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::alphabet::Alphabet;
use crate::nfa::Nfa;

use super::State;

impl<A: Alphabet + Serialize> Serialize for Nfa<A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(rename = "Nfa")]
        struct NfaHelper<'a, A: Alphabet> {
            states: Vec<&'a State<A>>,
        }

        let helper = NfaHelper {
            states: self.states().collect(),
        };
        helper.serialize(serializer)
    }
}

impl<'de, A: Alphabet + Deserialize<'de>> Deserialize<'de> for Nfa<A> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "Nfa")]
        struct NfaHelper<A: Alphabet> {
            states: Vec<State<A>>,
        }

        let helper = NfaHelper::deserialize(deserializer)?;
        let mut nfa = Nfa::new();
        let old2new: HashMap<_, _> = helper
            .states
            .iter()
            .map(|state| (state.id, nfa.add_state(state.accepting)))
            .collect();
        for old_from_state in &helper.states {
            let new_from = old2new[&old_from_state.id];
            for (symbol, old_to) in old_from_state.transitions() {
                nfa.add_transition(new_from, symbol, old2new[&old_to]);
            }
            for &old_to in old_from_state.next_epsilon() {
                nfa.add_epsilon_transition(new_from, old2new[&old_to]);
            }
        }
        Ok(nfa)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_common::{decltype, generate_strings};

    use super::*;

    #[test]
    fn test_nfa_serde() {
        let mut nfa = Nfa::new();
        let a = nfa.add_state(false);
        let b = nfa.add_state(true);
        nfa.add_epsilon_transition(a, a);
        nfa.add_transition(a, '0', a);
        nfa.add_transition(a, '1', a);
        nfa.add_transition(a, '1', b);
        nfa.add_transition(b, '0', a);
        nfa.add_transition(b, '1', b);

        let json = serde_json::to_string(&nfa).unwrap();
        let nfa2 = decltype(&nfa, serde_json::from_str(&json).unwrap());

        for word in generate_strings(&['0', '1'], 10) {
            assert_eq!(nfa.accepts(word.chars()), nfa2.accepts(word.chars()));
        }
    }
}
