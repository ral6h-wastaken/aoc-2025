use std::str::FromStr;

const L_SQ: char = '[';
const R_SQ: char = ']';
const L_BR: char = '{';
const R_BR: char = '}';
const L_PAREN: char = '(';
const R_PAREN: char = ')';

#[derive(Debug)]
pub struct Machine {
    target: LightDiagram,
    buttons: Vec<Button>,
    joltage_req: Vec<usize>,
}

impl Machine {
    pub fn target(&self) -> &LightDiagram {
        &self.target
    }

    pub fn buttons(&self) -> &[Button] {
        &self.buttons
    }

    pub fn joltage_req(&self) -> &[usize] {
        &self.joltage_req
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut states = Vec::<LightState>::new();
        let mut buttons = Vec::<Button>::new();
        let mut joltage_req = Vec::<usize>::new();

        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                L_SQ => {
                    // Parse light diagram until ]
                    while let Some(&next) = chars.peek() {
                        if next == R_SQ {
                            chars.next();
                            break;
                        }
                        let light_char = chars.next().unwrap();
                        if light_char != ' ' {
                            states.push(LightState::try_from(light_char)?);
                        }
                    }
                }
                L_PAREN => {
                    // Parse button targets until )
                    let mut targets = Vec::<usize>::new();
                    let mut num_str = String::new();

                    while let Some(&next) = chars.peek() {
                        if next == R_PAREN {
                            chars.next();
                            if !num_str.is_empty() {
                                targets.push(
                                    num_str
                                        .parse()
                                        .map_err(|e| format!("Invalid number: {}", e))?,
                                );
                            }
                            break;
                        }
                        let button_char = chars.next().unwrap();
                        if button_char == ',' {
                            if !num_str.is_empty() {
                                targets.push(
                                    num_str
                                        .parse()
                                        .map_err(|e| format!("Invalid number: {}", e))?,
                                );
                                num_str.clear();
                            }
                        } else if button_char.is_numeric() {
                            num_str.push(button_char);
                        }
                    }

                    buttons.push(Button { targets });
                }
                L_BR => {
                    // Parse joltage requirements until }
                    let mut num_str = String::new();

                    while let Some(&next) = chars.peek() {
                        if next == R_BR {
                            chars.next();
                            if !num_str.is_empty() {
                                joltage_req.push(
                                    num_str
                                        .parse()
                                        .map_err(|e| format!("Invalid number: {}", e))?,
                                );
                            }
                            break;
                        }
                        let jolt_char = chars.next().unwrap();
                        if jolt_char == ',' {
                            if !num_str.is_empty() {
                                joltage_req.push(
                                    num_str
                                        .parse()
                                        .map_err(|e| format!("Invalid number: {}", e))?,
                                );
                                num_str.clear();
                            }
                        } else if jolt_char.is_numeric() {
                            num_str.push(jolt_char);
                        }
                    }
                }
                _ => {} // Skip whitespace and other characters
            }
        }

        Ok(Machine {
            target: LightDiagram { states },
            buttons,
            joltage_req,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum LightState {
    ON,
    OFF,
}

impl TryFrom<char> for LightState {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::ON),
            '.' => Ok(Self::OFF),
            _ => Err(format!("Invalid char {value}")),
        }
    }
}

impl LightState {
    fn switch(&self) -> Self {
        match self {
            LightState::ON => LightState::OFF,
            LightState::OFF => LightState::ON,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct LightDiagram {
    states: Vec<LightState>,
}

#[derive(Debug)]
pub struct Button {
    targets: Vec<usize>,
}

impl LightDiagram {
    pub fn size(&self) -> usize {
        self.states.len()
    }

    pub fn new(states: Vec<LightState>) -> Self {
        Self { states }
    }

    pub fn compute(&self, btn: &Button) -> Self {
        let mut new_states = Vec::<LightState>::new();

        for (i, light) in self.states.iter().enumerate() {
            new_states.push(if btn.targets.contains(&i) {
                light.switch()
            } else {
                *light
            });
        }

        Self { states: new_states }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        let ld = LightDiagram {
            states: vec![
                LightState::ON,
                LightState::OFF,
                LightState::OFF,
                LightState::ON,
            ],
        };
        let btn = Button {
            targets: vec![1, 2],
        };

        let trg = LightDiagram {
            states: vec![
                LightState::ON,
                LightState::ON,
                LightState::ON,
                LightState::ON,
            ],
        };

        assert_eq!(ld.compute(&btn), trg)
    }
}
