use std::io;
use std::process::ExitCode;

fn io_envkey2val(key: &'static str) -> impl Fn() -> String {
    move || std::env::var(key).unwrap_or_default()
}

fn io_path2sqlite() -> impl Fn() -> String {
    io_envkey2val("ENV_SQLITE_DB_FILE_PATH")
}

fn io_main() -> impl FnOnce() -> Result<(), io::Error> {
    || {
        let path2sqlite: String = io_path2sqlite()();
        if path2sqlite.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "empty path got",
            ));
        }

        rs_sqlite2kvpairs::path2sqlite2pairs2stdout4json(path2sqlite)
    }
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        let path2sqlite: String = io_path2sqlite()();
        eprintln!("ENV_SQLITE_DB_FILE_PATH: {path2sqlite}");
        ExitCode::FAILURE
    })
}
