use crate::errors::*;

#[derive(Copy, Clone, PartialEq)]
pub enum LineSlice {
    WholeLine(usize),
    Slice(usize, usize, usize),
}

impl LineSlice {
    pub fn from(slice_raw: &str) -> Result<LineSlice> {
        LineSlice::parse_slice(slice_raw)
    }

    pub fn new() -> LineSlice {
        LineSlice::WholeLine(0)
    }

    fn parse_as_whole_line(slice_raw: &str) -> Result<LineSlice> {
        slice_raw
            .parse::<usize>()
            .map(LineSlice::WholeLine)
            .map_err(Error::from) // XXX. why is Error::from necessary?
    }

    fn parse_as_slice(slice_raw: &str) -> Result<LineSlice> {
        let bits: Vec<&str> = slice_raw.split(':').collect();
        if bits.len() == 3 {
            bits.iter()
                .map(|w| w.parse::<usize>().map_err(Error::from)) // XXX. why is Error::from necessary?
                .collect::<Result<Vec<_>>>()
                .map(|ws| LineSlice::Slice(ws[0], ws[1], ws[2]))
        } else {
            panic!("ill-formed argument") // XXX. is this the proper way to fail the parsin?
        }
    }

    pub fn parse_slice(slice_raw: &str) -> Result<LineSlice> {
        LineSlice::parse_as_whole_line(slice_raw).or(LineSlice::parse_as_slice(slice_raw))
    }
}
