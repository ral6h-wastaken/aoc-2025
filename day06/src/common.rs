use std::{fmt::format, str::FromStr};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Operator {
    PLUS,
    TIMES,
    UNDEFINED
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        match s {
            "*" => Ok(Self::TIMES),
            "+" => Ok(Self::PLUS),
            _ => Err("Operation not supported".to_string()),
        }
    }
}

impl TryFrom<&str> for Operator {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Operator::from_str(value)
    }
}

impl TryFrom<char> for Operator {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Operator::from_str(format!("{}", value).as_str())
    }
}

#[derive(Eq, PartialEq)]
pub struct Operation {
    operands: Vec<u64>,
    operator: Operator,
}

impl Operation {
    pub fn new() -> Self {
        Self { operands: vec![], operator: Operator::UNDEFINED }
    }

    pub fn add_operand(&mut self, to_add: u64) {
        self.operands.push(to_add);
    }

    pub fn set_operator(&mut self, operator: Operator) {
        self.operator = operator
    }

    pub fn calculate(&self) -> Result<u64, String> {
        match self.operator {
            Operator::PLUS => {
                        Ok(self.operands.iter().fold(0, |acc, next| acc + next))
                    }
            Operator::TIMES => {
                        Ok(self.operands.iter().fold(1, |acc, next| acc * next))
                    }
            Operator::UNDEFINED => Err("No operator was defined for this operation".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::{Operation, Operator};

    #[test]
    fn test_operation_plus() {
        let mut op = Operation::new();
        for i in 1..=10 { op.add_operand(i); }
        op.set_operator(Operator::PLUS);

        assert_eq!(op.calculate(), Ok(55))
    }

    #[test]
    fn test_operation_times() {
        let mut op = Operation::new();
        for i in 1..=5 { op.add_operand(i); }
        op.set_operator(Operator::TIMES);

        assert_eq!(op.calculate(), Ok(120))
    }
}
