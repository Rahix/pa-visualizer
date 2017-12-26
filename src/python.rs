extern crate cpython;
extern crate framework;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate toml;

pub fn visualizer(
    config: ::std::sync::Arc<toml::Value>,
    audio_info: ::std::sync::Arc<::std::sync::RwLock<framework::AudioInfo>>,
    mut run_mode: framework::RunMode,
) {

    if let framework::RunMode::Rendering(_) = run_mode {
        warn!("Python visuallizer does not support rendering!");
    }

    let display_columns = config
        .get("DISPLAY_COLUMNS")
        .map(|v| {
            v.as_integer().expect("DISPLAY_COLUMNS must be an integer")
        })
        .unwrap_or(20) as usize;
    info!("DISPLAY_COLUMNS = {}", display_columns);

    let filename = config
        .get("PYTHON_SCRIPT")
        .map(|v| {
            String::from(v.as_str().expect("PYTHON_SCRIPT must be a string"))
        })
        .unwrap_or(String::from("visualizer.py"));
    info!("PYTHON_SCRIPT = \"{}\"", filename);

    let gil = cpython::Python::acquire_gil();
    let py = gil.python();

    // Setup globals/locals
    let locals = cpython::PyDict::new(py);
    let globals = py.eval("globals()", None, None)
        .unwrap()
        .extract::<cpython::PyDict>(py)
        .unwrap();
    globals.set_item(py, "COLUMNS", display_columns).unwrap();

    {
        // Read script and run it
        use std::io::Read;

        let mut f = ::std::fs::File::open(filename).expect("Can't open file");
        let mut script = String::new();
        f.read_to_string(&mut script).unwrap();
        py.run(&script, Some(&globals), Some(&locals)).unwrap();
    }

    // Copy set up locals into globals
    for (k, v) in locals.items(py) {
        globals.set_item(py, k, v).unwrap();
    }

    'mainloop: loop {
        {
            let ai = audio_info.read().expect("Couldn't read audio info");

            // Execute script
            locals
                .set_item(py, "__left_spectrum", &ai.columns_left)
                .unwrap();
            locals
                .set_item(py, "__right_spectrum", &ai.columns_right)
                .unwrap();
            if !py.eval(
                "frame(__left_spectrum, __right_spectrum)",
                Some(&globals),
                Some(&locals),
            ).unwrap()
                .extract::<bool>(py)
                .unwrap()
            {
                break 'mainloop;
            }
        }

        framework::sleep(&mut run_mode);
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    framework::start("configs/python.toml", visualizer);
}
