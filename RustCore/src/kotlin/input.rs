#[derive(Debug)]
#[allow(dead_code)]
pub(super) enum Input {
    FilePath(String),
    FilterAdd,
    FilterMove,
    FilterDelete
}

impl Input {
    pub(super) fn from_byte_array<const N: usize>(buffer: [u8; N]) -> Input {
        match buffer[0] {
            b'p' => Input::FilePath(
                std::str::from_utf8(&buffer[1..])
                    .unwrap()
                    .trim_end_matches('\0')
                    .trim_end()
                    .trim_end_matches('\r')
                    .to_owned()
            ),
            _ => todo!()
        }
    }
}