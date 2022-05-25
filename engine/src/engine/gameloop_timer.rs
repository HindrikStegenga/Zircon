use crate::*;
use std::time::*;
pub struct EngineGameloopTimer {
    // Amount of ticks the update is updated each second.
    pub update_tick_rate: u32,
    // Amount of frames that may be skipped.
    pub max_skipped_frames: u32,
    // Max framerate that the engine will be capped to.
    pub max_frame_rate: Option<u32>,

    pub previous_frame_instant: Instant,
    pub previous_second_instant: Instant,
    pub last_fixed_update_instant: Instant,

    pub frame_start_instant: Instant,
    pub current_delta_time: Duration,

    pub accumulated_time: Duration,

    pub previous_sleep_time: Duration,
    pub negative_sleep_time: Duration,
    // Counts total amount of update ticks in the past second.
    pub update_counter: u64,
    // Counts total amount of frames in the past second.
    pub frame_counter: u64,
    // Progress towards next update tick from 0 to 1.
    pub alpha: f32,

    pub total_sleep_time_last_second: Duration,
    pub total_frame_time_last_second: Duration,
}

impl EngineGameloopTimer {
    pub fn frame_start(&mut self) {
        let fixed_update_step_duration =
            Duration::from_millis(1000) / (self.update_tick_rate as u32);

        self.frame_start_instant = Instant::now();
        self.current_delta_time = self
            .frame_start_instant
            .duration_since(self.previous_frame_instant);
        self.accumulated_time += self.current_delta_time;

        if self.frame_start_instant > self.last_fixed_update_instant {
            let delta = self.frame_start_instant - self.last_fixed_update_instant;
            self.alpha =
                ((delta.as_nanos() as f64) / (fixed_update_step_duration.as_nanos() as f64)) as f32;
        }

        if let Some(max_frame_rate) = self.max_frame_rate {
            // Frame limiting happens here.
            let targeted_update_step_duration =
                Duration::from_millis(1000) / (max_frame_rate as u32);
            if self.previous_sleep_time > self.current_delta_time {
                // This only occurs when the frame limit has been changed.
                warn!("Frame limit changed!");
                self.previous_sleep_time = Duration::new(0, 0);
            }
            let delta_time_without_sleep = self.current_delta_time - self.previous_sleep_time;
            self.total_frame_time_last_second += delta_time_without_sleep;
            if targeted_update_step_duration > delta_time_without_sleep {
                // Our frame took less time than we want, so we need to sleep.
                // Also correct for previous frame sleep time, since that's included in the delta_time.
                let left_over = targeted_update_step_duration - delta_time_without_sleep;
                if left_over > self.negative_sleep_time {
                    let sleep_time = left_over - self.negative_sleep_time;
                    std::thread::sleep(sleep_time);
                    self.total_sleep_time_last_second += sleep_time;
                    self.previous_sleep_time = sleep_time;
                    self.negative_sleep_time = Duration::new(0, 0);
                } else {
                    std::thread::sleep(left_over);
                    self.total_sleep_time_last_second += left_over;
                    self.negative_sleep_time -= left_over;
                    self.previous_sleep_time = left_over;
                }
            } else {
                // Our frame took more time, hence we need to add negative sleep time.
                self.negative_sleep_time +=
                    delta_time_without_sleep - targeted_update_step_duration;
                self.previous_sleep_time = Duration::new(0, 0);
            }
        } else {
            self.total_frame_time_last_second += self.current_delta_time;
        }

        self.previous_frame_instant = self.frame_start_instant;
    }

    pub fn frame_end(&mut self) {
        self.frame_counter += 1;

        if self
            .frame_start_instant
            .duration_since(self.previous_second_instant)
            > Duration::from_millis(1000)
        {
            log!("Total update count: {}", self.update_counter);
            log!("Total frame count: {}", self.frame_counter);
            log!(
                "Avg. frametime: {:#?}",
                self.total_frame_time_last_second / (self.frame_counter as u32)
            );

            if self.max_frame_rate.is_some() {
                let sum = self.total_sleep_time_last_second + self.total_frame_time_last_second;
                let perc =
                    self.total_sleep_time_last_second.as_nanos() / (sum / 100 as u32).as_nanos();

                log!(
                    "Avg. sleep: {:#?} ({}%)",
                    self.total_sleep_time_last_second / (self.frame_counter as u32),
                    perc
                );
            }

            self.total_frame_time_last_second = Duration::new(0, 0);
            self.total_sleep_time_last_second = Duration::new(0, 0);
            self.previous_second_instant = self.frame_start_instant;
            self.update_counter = 0;
            self.frame_counter = 0;
        }
    }
}
