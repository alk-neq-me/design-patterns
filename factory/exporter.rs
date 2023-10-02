use std::io;

trait VideoExporter {
  fn prepare_export(&self);

  fn do_export(&self);
}


trait AudioExporter {
  fn convert_audio(&self);

  fn do_export(&self);
}


/** For Video Exporter **/
struct HightQulatityVideoExporter;

impl VideoExporter for HightQulatityVideoExporter {
  fn prepare_export(&self) {
    println!("prepare video for hight qulatity");
  }

  fn do_export(&self) {
    println!("exporting video for hight qulatity");
  }
}


struct LowQulatityVideoExporter;

impl VideoExporter for LowQulatityVideoExporter {
  fn prepare_export(&self) {
    println!("prepare video for low qulatity");
  }

  fn do_export(&self) {
    println!("exporting video for low qulatity");
  }
}


/** For Audio Exporter **/
struct Mp3AudioExporter;

impl AudioExporter for Mp3AudioExporter {
  fn convert_audio(&self) {
    println!("Audio convert mp3");
  }

  fn do_export(&self) {
    println!("Audio exporting for Mp3");
  }
}


struct WaveAudioExporter;

impl AudioExporter for WaveAudioExporter {
  fn convert_audio(&self) {
    println!("Audio convert to wave");
  }

  fn do_export(&self) {
    println!("Audio exporting for Wave");
  }
}


/** Export Factory **/
trait ExporterFactory {
  fn make_video_exporter(&self) -> Box<dyn VideoExporter>;

  fn make_audio_exporter(&self) -> Box<dyn AudioExporter>;
}


struct HightQulatityExporter;

impl ExporterFactory for HightQulatityExporter {
  fn make_video_exporter(&self) -> Box<dyn VideoExporter> {
    Box::new(HightQulatityVideoExporter)
  }

  fn make_audio_exporter(&self) -> Box<dyn AudioExporter> {
    Box::new(Mp3AudioExporter)
  }
}


struct LowQulatityExporter;

impl ExporterFactory for LowQulatityExporter {
  fn make_video_exporter(&self) -> Box<dyn VideoExporter> {
    Box::new(LowQulatityVideoExporter)
  }

  fn make_audio_exporter(&self) -> Box<dyn AudioExporter> {
    Box::new(WaveAudioExporter)
  }
}


fn get_exporter() -> Result<Box<dyn ExporterFactory>, io::Error> {
  let mut qulatity = String::new();

  println!("Qulatity:");
  io::stdin().read_line(&mut qulatity)?;

  return match qulatity.trim() {
    "hight" => Ok(Box::new(HightQulatityExporter)),
    "low" => Ok(Box::new(LowQulatityExporter)),
    _ => Err(io::ErrorKind::NotFound.into())
  }
}


fn main() {
  let exporter = get_exporter().expect("Failed exporter");
  
  let video_exporter = exporter.make_video_exporter();
  let audio_exporter = exporter.make_audio_exporter();

  video_exporter.prepare_export();
  video_exporter.do_export();

  audio_exporter.convert_audio();
  audio_exporter.do_export();
}
