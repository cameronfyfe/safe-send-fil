use fvm_shared::error::ExitCode;

pub struct Error {
    pub exit_code: Option<ExitCode>,
    pub msg: String,
}

impl Error {
    pub fn new<S>(exit_code: Option<ExitCode>, msg: S) -> Self
    where
        S: Into<String>,
    {
        let msg = msg.into();
        Self { exit_code, msg }
    }

    pub fn abort(&self) -> ! {
        fvm_sdk::vm::abort(
            self.exit_code.unwrap_or(ExitCode::USR_UNSPECIFIED).value(),
            Some(self.msg.as_str()),
        )
    }
}
