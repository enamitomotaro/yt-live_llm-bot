use crate::error::{Error, Result};
use anyhow::Context;
use cpal::traits::{DeviceTrait, HostTrait};
use once_cell::sync::Lazy;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{io::Cursor, sync::Mutex};

static STREAM: Lazy<Mutex<Option<OutputStreamHandle>>> = Lazy::new(|| Mutex::new(None));

fn handle() -> Result<OutputStreamHandle> {
    let mut g = STREAM.lock().unwrap();
    if g.is_none() {
        let host = cpal::default_host();
        let dev = host
            .output_devices()
            .context("list devices")?
            .find(|d| d.name().ok().filter(|n| n.contains("BlackHole")).is_some())
            .ok_or(Error::AudioDeviceNotFound)?;
        let (stream, h) = OutputStream::try_from_device(&dev).context("open BlackHole")?;
        *g = Some(h);
        std::mem::forget(stream);
    }
    Ok(g.as_ref().unwrap().clone())
}

pub fn play(wav: &[u8]) -> Result<()> {
    let sink = Sink::try_new(&handle()?).context("sink")?;
    sink.append(Decoder::new(Cursor::new(wav.to_vec())).context("decode")?);
    sink.sleep_until_end();
    Ok(())
}
