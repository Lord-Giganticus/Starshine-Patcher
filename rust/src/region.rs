#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Region {
    USA = 69,
    PAL = 80,
    JAP = 74,
    KOR = 75,
    TWN = 87,
}

impl<T> From<T> for Region where T : Into<isize> {
    fn from(t: T) -> Self {
        let n: isize = t.into();
        use Region::*;
        match n {
            69 => USA,
            80 => PAL,
            74 => JAP,
            75 => KOR,
            87 => TWN,
            _ => USA
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Region::*;
        match *self {
            USA => write!(f, "USA"),
            PAL => write!(f, "PAL"),
            JAP => write!(f, "JAP"),
            KOR => write!(f, "KOR"),
            TWN => write!(f, "TWN")
        }
    }
}

