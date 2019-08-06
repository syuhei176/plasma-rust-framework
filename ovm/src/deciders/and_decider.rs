use crate::error::Error;
use crate::property_executor::PropertyExecutor;
use crate::types::{AndDeciderInput, Decider, Decision};
use crate::DecideMixin;
use bytes::Bytes;

pub struct AndDecider {}

impl AndDecider {
    pub fn new() -> Self {
        AndDecider {}
    }
}

impl Default for AndDecider {
    fn default() -> Self {
        AndDecider {}
    }
}

impl Decider for AndDecider {
    type Input = AndDeciderInput;
    fn decide(
        decider: &PropertyExecutor,
        input: &AndDeciderInput,
        _witness: Option<&Bytes>,
    ) -> Result<Decision, Error> {
        let left_decision = input
            .get_left()
            .decide(decider, Some(input.get_left_witness()))?;
        let right_decision = input
            .get_right()
            .decide(decider, Some(input.get_right_witness()))?;
        if !left_decision.get_outcome() {
            return Ok(left_decision);
        }
        if !right_decision.get_outcome() {
            return Ok(right_decision);
        }
        Ok(Decision::new(
            true,
            [
                &left_decision.get_implication_proof()[..],
                &right_decision.get_implication_proof()[..],
            ]
            .concat(),
        ))
    }

    fn check_decision(
        decider: &PropertyExecutor,
        input: &AndDeciderInput,
    ) -> Result<Decision, Error> {
        Self::decide(decider, input, None)
    }
}

#[cfg(test)]
mod tests {
    use super::AndDecider;
    use crate::deciders::preimage_exists_decider::Verifier;
    use crate::property_executor::PropertyExecutor;
    use crate::types::{AndDeciderInput, Decider, Decision, PreimageExistsInput, Property};
    use bytes::Bytes;

    #[test]
    fn test_decide() {
        let left = Property::PreimageExistsDecider(Box::new(PreimageExistsInput::new(
            Verifier::static_hash(&Bytes::from("left")),
        )));
        let left_witness = Bytes::from("left");
        let right = Property::PreimageExistsDecider(Box::new(PreimageExistsInput::new(
            Verifier::static_hash(&Bytes::from("right")),
        )));
        let right_witness = Bytes::from("right");
        let input = AndDeciderInput::new(left, left_witness, right, right_witness);
        let and_decider = Property::AndDecider(Box::new(input.clone()));
        let decider: PropertyExecutor = Default::default();
        let decided: Decision = decider.decide(&and_decider, None).unwrap();
        assert_eq!(decided.get_outcome(), true);
        let status = AndDecider::check_decision(&decider, &input).unwrap();
        assert_eq!(status.get_outcome(), true);
    }

}