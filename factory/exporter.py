from abc import ABC, abstractmethod

class VideoExpoter(ABC):
    @abstractmethod
    def prepare_export(self) -> None:
        """To prepafe export"""

    @abstractmethod
    def do_export(self) -> None:
        """Export video"""


class AudioExporter(ABC):
    @abstractmethod
    def convert_audio(self) -> None:
        """convert audio"""


class HightQulatityVideoExporter(VideoExpoter):
    def prepare_export(self) -> None:
        print("hight qulatity video export")

    def do_export(self) -> None:
        print("export hight video")


class LowQulatityVideoExporter(VideoExpoter):
    def prepare_export(self) -> None:
        print("hight low video export")

    def do_export(self) -> None:
        print("export low video")


class Mp3QulatityExporter(AudioExporter):
    def convert_audio(self) -> None:
        print("convert mp3 audio")


class WaveQulatityExporter(AudioExporter):
    def convert_audio(self) -> None:
        print("convert wave audio")


class ExporterFactory(ABC):
    @abstractmethod
    def make_video_exporter(self) -> VideoExpoter:
        """Return video export"""

    @abstractmethod
    def make_audio_exporter(self) -> AudioExporter:
        """Return audio export"""


class HightQulatityExporter(ExporterFactory):
    def make_video_exporter(self) -> VideoExpoter:
        return HightQulatityVideoExporter()

    def make_audio_exporter(self) -> AudioExporter:
        return Mp3QulatityExporter()

class LowQulatityExporter(ExporterFactory):
    def make_audio_exporter(self) -> AudioExporter:
        return WaveQulatityExporter()

    def make_video_exporter(self) -> VideoExpoter:
        return LowQulatityVideoExporter()


def main() -> None:
    qulatities = {
        "high_q": HightQulatityExporter(),
        "low_q": LowQulatityExporter()
    }

    exporter: ExporterFactory

    if (i := "low_q") in qulatities:
        exporter = qulatities[i]

        video_exporter = exporter.make_video_exporter()
        audio_exporter = exporter.make_audio_exporter()

        video_exporter.prepare_export()
        video_exporter.do_export()

        audio_exporter.convert_audio()


if __name__ == "__main__":
    main()
