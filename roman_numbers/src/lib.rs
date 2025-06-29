

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla,
        }
    }
}

#[derive(Debug)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mut num = n;
        let mut count = 0;
        let mut result : Vec<RomanDigit> = Vec::new();
        while num != 0 {
            let rest = num % 10;
            num /= 10;
            count += 1;
            if rest == 4 {
                result.push(RomanDigit::from(10_u32.pow(count)/2));
                result.push(RomanDigit::from(10_u32.pow(count-1)));
            }else if rest == 9 {
                result.push(RomanDigit::from(10_u32.pow(count)));
                result.push(RomanDigit::from(10_u32.pow(count-1)));
            }else if rest >= 5 {
                let k = rest - 5;
                for _i in 0..k {
                    result.push(RomanDigit::from(10_u32.pow(count-1)));
                }
                result.push(RomanDigit::from(10_u32.pow(count)/2));

            }else {
                for _i in 0..rest {
                    result.push(RomanDigit::from(10_u32.pow(count-1)));
                }
            }
        }
        result.reverse();
        Self(result)
    }
}