use dfhack_remote;

#[cfg(feature = "test-with-df")]
mod withdf {
    use std::process::Child;
    use std::sync::Mutex;
    #[cfg(test)]
    lazy_static::lazy_static! {
        static ref DF_PROCESS: Mutex<Option<Child>> = Mutex::new(Option::<Child>::None);
    }

    #[ctor::ctor]
    fn init() {
        let port = portpicker::pick_unused_port().unwrap().to_string();
        std::env::set_var("DF_PORT", port);

        use std::{path::PathBuf, process::Command};
        let df_exe = PathBuf::from(std::env::var("DF_EXE").unwrap());
        let df_folder = df_exe.parent().unwrap();

        let df = Command::new(&df_exe)
            .args(["+load-save", "region1"])
            .current_dir(df_folder)
            .spawn()
            .unwrap();
        let mut process_guard = DF_PROCESS.lock().unwrap();
        process_guard.replace(df);
    }

    #[ctor::dtor]
    fn exit() {
        let mut process_guard = DF_PROCESS.lock().unwrap();
        let df = process_guard.take();
        if let Some(mut df) = df {
            df.kill().unwrap();
        }
    }

    #[test]
    fn get_version() {
        let mut client = dfhack_remote::DFHack::connect().unwrap();
        let version = client.core.get_df_version().unwrap();
        assert!(version.get_value().len() > 0);
    }
}

#[test]
fn sanity_check() {
    let mut message = dfhack_remote::messages::IntMessage::new();
    message.set_value(12);
    assert!(message.get_value() == 12);
}
