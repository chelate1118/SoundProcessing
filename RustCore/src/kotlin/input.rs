use crate::sound::{filter::BandPass, *};

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum Input {
    FilePath(String),
    FilterAdd(BandPass),
    FilterMove(usize, BandPass),
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
            b'a' => Input::FilterAdd(
                BandPass::new(
                    (buffer[1]as i32 * 20) as f32,
                    1.0,
                    1.5
                )
                ),
            b'm' => Input::FilterMove(
                buffer[1] as usize,
                BandPass::new(
                    (buffer[2]as i32 * 20) as f32,
                    1.0,
                    1.5
                )
                ),
            _ => unreachable!()
        }
    }

    pub(super) fn apply(&self) {
        match self {
            Self::FilePath(path) => {
                play_audio_file(path)
            },
            Self::FilterAdd(filter) => {
                add_filter(*filter)
            },
            Self::FilterMove(index, filter) => {
                move_filter(*index, *filter)
            },
            _ => unreachable!()
        }
    }
}