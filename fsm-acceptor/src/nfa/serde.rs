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

        let mut helper = NfaHelper::deserialize(deserializer)?;
        helper.states.sort_by_key(|state| state.id);
        let mut dfa = Nfa::new();
        let mut old2new = HashMap::with_capacity(helper.states.len());
        for state in &helper.states {
            let new_id = dfa.add_state(state.accepting);
            old2new.insert(state.id, new_id);
        }
        for (&old_from, &new_from) in &old2new {
            for (&symbol, all_old_to) in &helper.states[old_from].transitions {
                for old_to in all_old_to {
                    dfa.add_transition(new_from, symbol, old2new[old_to]);
                }
            }
            for old_to in &helper.states[old_from].epsilon_transitions {
                dfa.add_epsilon_transition(new_from, old2new[old_to]);
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

        for word in generate_words(&['0', '1'], 10) {
            assert_eq!(nfa.accepts(word.chars()), nfa2.accepts(word.chars()));
        }
    }
}
