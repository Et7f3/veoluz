use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub struct State {
    pub render_id: usize,
    pub last_rendered: usize,
    pub ctx: CanvasRenderingContext2d,
    pub image_data: web_sys::ImageData,
    pub config: shared::Config,
    pub buffer: Vec<u32>,
    pub workers: Vec<(web_sys::Worker, bool, Option<shared::messaging::Message>)>,
}

// umm I dunno if this is cheating or something
// I mean bad things could happen if I accessed the ctx
// from different threads
// but given that wasm doesn't yet have threads, it's probably fine.
unsafe impl Send for State {}

impl From<shared::Config> for State {
    fn from(config: shared::Config) -> Self {
        State {
            render_id: 0,
            last_rendered: 0,
            ctx: crate::ui::init(&config).expect("Unable to setup canvas"),
            image_data: web_sys::ImageData::new_with_sw(config.width as u32, config.height as u32)
                .expect("Can't make an imagedata"),
            buffer: vec![0_u32; config.width * config.height],
            workers: vec![],
            config,
        }
    }
}

pub fn make_image_data(
    config: &shared::Config,
    bright: &[u32],
) -> Result<web_sys::ImageData, JsValue> {
    let colored = shared::colorize(config, bright);

    let mut clamped = wasm_bindgen::Clamped(colored.clone());
    // let mut clamped = Clamped(state.buffer.clone());
    let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(clamped.as_mut_slice()),
        config.width as u32,
        config.height as u32,
    )?;

    Ok(data)
}

impl State {
    pub fn reset_buffer(&mut self) {
        self.buffer = vec![0_u32; self.config.width * self.config.height];
    }

    pub fn add_worker(&mut self, worker: web_sys::Worker) {
        self.workers.push((worker, false, None))
    }

    pub fn handle_render(
        &mut self,
        worker: usize,
        id: usize,
        array: js_sys::Uint32Array,
    ) -> Result<(), JsValue> {
        if id < self.last_rendered {
            // this is old data, disregard
            return Ok(());
        }
        if id > self.last_rendered {
            self.reset_buffer();
            self.last_rendered = id;
        }

        let mut bright = vec![0_u32; self.config.width * self.config.height];
        array.copy_to(&mut bright);
        for i in 0..bright.len() {
            self.buffer[i] += bright[i];
        }

        self.image_data = make_image_data(&self.config, &self.buffer)?;

        self.ctx.put_image_data(&self.image_data, 0.0, 0.0)?;

        let (worker, busy, queued) = &mut self.workers[worker];
        match queued {
            None => *busy = false,
            Some(message) => {
                worker.post_message(&JsValue::from_serde(message).unwrap())?;
                *queued = None
            }
        }

        Ok(())
    }

    pub fn debug_render(&mut self) -> Result<(), JsValue> {
        let brightness = shared::deterministic_calc(&self.config);
        self.image_data = make_image_data(&self.config, &brightness)?;

        self.ctx.put_image_data(&self.image_data, 0.0, 0.0)?;
        Ok(())
    }

    pub fn async_render(&mut self, small: bool) -> Result<(), JsValue> {
        self.render_id += 1;

        let message = shared::messaging::Message {
            config: self.config.clone(),
            id: self.render_id,
            count: if small { 10_000 } else { 100_000 },
        };
        if self.workers.is_empty() {
            return self.debug_render();
        }

        for (worker, busy, queued) in self.workers.iter_mut() {
            if *busy {
                *queued = Some(message.clone())
            } else {
                *busy = true;
                worker.post_message(&JsValue::from_serde(&message).unwrap())?;
            }
        }
        Ok(())
    }
}

lazy_static! {
    static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

pub fn withOptState<F: FnOnce(&mut Option<State>)>(f: F) {
    f(&mut STATE.lock().unwrap())
}

pub fn setState(state: State) {
    withOptState(|wrapper| *wrapper = Some(state))
}

pub fn with<R, F: FnOnce(&mut State) -> R>(f: F) -> R {
    match STATE.lock().unwrap().as_mut() {
        Some(mut state) => f(&mut state),
        None => {
            log!("!!! Error: tried to handle state, but no state found");
            panic!("No state found, must set state first")
        }
    }
}

pub fn try_with<F: FnOnce(&mut State) -> Result<(), wasm_bindgen::prelude::JsValue>>(f: F) {
    with(|state| crate::utils::try_log(|| f(state)))
}
