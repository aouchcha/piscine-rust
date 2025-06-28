use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s  {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(())
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s  {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(())
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => {
                self.rh_factor.cmp(&other.rh_factor)
            }
            other_ordering => other_ordering,
        }
    }
}

impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antigen : String = String::new();
        let mut rh_factor : String = String::new();
        for c in s.chars() {
            if c.is_alphabetic() {
                antigen.push(c);
            }else {
                rh_factor.push(c)
            }
        }
        Ok(BloodType {
            antigen: antigen.parse()?,
            rh_factor: rh_factor.parse()?,

        })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A=> "A",
            Antigen::B=> "B",
            Antigen::AB=> "AB",
            Antigen::O=> "O",
        };
        let rh_factor = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f,"{}{}",antigen,rh_factor)
    }
    
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        match self.antigen {
            Antigen::A => {
                match other.antigen {
                    Antigen::A => match other.rh_factor {
                        RhFactor::Negative => {return true},
                        RhFactor::Positive => match self.rh_factor {
                            RhFactor::Positive => {return true},
                            _ => {return false},
                        }
                    },
                    Antigen::O => match other.rh_factor  {
                        RhFactor::Negative => {return true},
                        RhFactor::Positive => match self.rh_factor {
                            RhFactor::Positive => {return true},
                            _ => {return false},
                        }
                    },
                    _ => {return false}
                }
            },
            Antigen::B => {
                match other.antigen {
                    Antigen::B => match other.rh_factor  {
                        RhFactor::Negative => {return true},
                        RhFactor::Positive => match self.rh_factor {
                            RhFactor::Positive => {return true},
                            _ => {return false},
                        }
                    },
                    Antigen::O => match other.rh_factor  {
                        RhFactor::Negative => {return true},
                        RhFactor::Positive => match self.rh_factor {
                            RhFactor::Positive => {return true},
                            _ => {return false},
                        }
                    },
                    _ => {return false}
                }
            },
            Antigen::AB => {
               match other.rh_factor {
                RhFactor::Negative => {return true},
                RhFactor::Positive => match self.rh_factor {
                    RhFactor::Positive => {return true},
                    _ => {return false},
                }
               }
            },
            Antigen::O => {
                match other.antigen {
                    Antigen::O => match other.rh_factor {
                        RhFactor::Negative => {return true},
                        _ => match self.rh_factor {
                            RhFactor::Positive => {return true},
                            _ => {return false},
                        }
                    },
                    _ => {return false},
                }
            }
        }
	}

	pub fn donors(&self) -> Vec<Self> {
        match self.antigen {
            Antigen::A => match self.rh_factor {
                RhFactor::Negative => {return vec![
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                ]},
                RhFactor::Positive => {return vec![
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                ]}
            },
            Antigen::B =>  match self.rh_factor {
                RhFactor::Negative => {return vec![
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                ]},
                RhFactor::Positive => {return vec![
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                ]}
            },
            Antigen::AB => match self.rh_factor {
                RhFactor::Negative => {return vec![
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                ]},
                RhFactor::Positive => {return vec![
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    }, BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                ]}
            },
            Antigen::O => match self.rh_factor {
                RhFactor::Negative => {return vec![
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                ]},
                RhFactor::Positive => {return vec![
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                ]}
            },
        }
	}

	pub fn recipients(&self) -> Vec<BloodType> {
        match self.antigen {
            Antigen::A => match self.rh_factor {
                RhFactor::Negative => {return vec![
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                ]},
                RhFactor::Positive => {return vec![
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                ]}
            },
            Antigen::B =>  match self.rh_factor {
                RhFactor::Negative => {return vec![
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                ]},
                RhFactor::Positive => {return vec![
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                ]}
            },
            Antigen::AB => match self.rh_factor {
                RhFactor::Negative => {return vec![
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                ]},
                RhFactor::Positive => {return vec![
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                ]}
            },
            Antigen::O => match self.rh_factor {
                RhFactor::Negative => {return vec![
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    }, BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                ]},
                RhFactor::Positive => {return vec![
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                ]}
            },
        }
    }
}