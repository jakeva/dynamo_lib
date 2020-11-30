pub struct SoundSystem {
    #[allow(dead_code)]
    device: rodio::Device,
    sink: rodio::Sink,
    spatial_sink: rodio::SpatialSink,
}

impl SoundSystem {
    pub fn new() -> Self {
        let device = rodio::default_output_device().unwrap();
        let sink = rodio::Sink::new(&device);
        sink.set_volume(0.5);

        let spatial_sink =
            rodio::SpatialSink::new(&device, [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);

        Self {
            device,
            sink,
            spatial_sink,
        }
    }

    #[inline]
    pub fn queue<S>(&self, sound: S)
    where
        S: rodio::Source + Send + 'static,
        S::Item: rodio::Sample,
        S::Item: Send,
    {
        self.sink.append(sound);
    }

    #[allow(dead_code)]
    #[inline]
    pub fn queue_spatial<S>(&self, sound: S, position: [f32; 3])
    where
        S: rodio::Source + Send + 'static,
        S::Item: rodio::Sample + Send + std::fmt::Debug,
    {
        self.spatial_sink.set_emitter_position(position);
        self.spatial_sink.append(sound);
    }
}
