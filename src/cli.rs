use argh::FromArgs;

#[derive(Debug, Clone, FromArgs, Default)]
#[argh(description = "emo_shiro")]
pub struct EmoArgs {
    /// you can specify known keys
    #[argh(option, default = "default_key()")]
    pub key: String,
    /// apache-shiro encryption algorithm,default: CBC
    #[argh(option, short = 'm')]
    pub mode: Option<String>,
    /// only verify whether it is apache-shiro
    #[argh(option)]
    pub verify: Option<bool>,
    /// the target
    #[argh(option, short = 't')]
    pub target: Option<String>,
    /// read the target from the file
    #[argh(option)]
    pub file: Option<String>,
    /// read the key from the file
    #[argh(option)]
    pub keys: Option<String>,
    /// export to the csv file
    #[argh(option, short = 'c')]
    pub csv: Option<String>,
    /// proxy to use for requests (ex:[http(s)|socks5(h)]://host:port)
    #[argh(option)]
    pub proxy: Option<String>,
    /// set request timeout
    #[argh(option, default = "default_timeout()")]
    pub timeout: u64,
    /// number of concurrent threads
    #[argh(option, default = "default_thread()")]
    pub thread: u32,
}

fn default_thread() -> u32 {
    100_u32
}

fn default_timeout() -> u64 {
    10
}

fn default_key() -> String {
    String::from("kPH+bIxk5D2deZiIxcaaaA==")
}
impl EmoArgs {
    pub fn new() -> Self {
        let default: EmoArgs = argh::from_env();
        default
    }
    pub fn get_method(&self) {}
}
