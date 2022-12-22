use argh::FromArgs;

#[derive(Debug, Clone, FromArgs, Default)]
#[argh(description = "emo_shiro")]
pub struct EmoArgs {
    /// you can specify known keys
    #[argh(option, default = "String::from(\"kPH+bIxk5D2deZiIxcaaaA==\")")]
    pub key: String,
    /// apache-shiro encryption algorithm,default: CBC
    #[argh(option, short = 'm')]
    pub mode: Option<String>,
    /// the target
    #[argh(option, short = 't')]
    pub target: Option<String>,
    /// serialize file
    #[argh(option, short = 's')]
    pub ser: Option<String>,
    /// read the target from the file
    #[argh(option)]
    pub file: Option<String>,
    /// read the key from the file
    #[argh(option)]
    pub keys: Option<String>,
    /// export to the csv file
    #[argh(option)]
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
    /// enum chain mode
    #[argh(switch)]
    pub chain: bool,
    /// exploit mode
    #[argh(switch)]
    pub exploit: bool,
    /// dns identifier, default: 981tzg.ceye.io
    #[argh(option, default = "String::from(\"981tzg.ceye.io\")")]
    pub dns: String,
    /// select a payload
    #[argh(option, short = 'p')]
    pub payload: Option<String>,
    /// command to execute
    #[argh(option, short = 'c')]
    pub command: Option<String>,
    /// tomcat echo request header name
    #[argh(option)]
    pub echo_name: Option<String>,
    /// tomcat command request header name
    #[argh(option)]
    pub command_name: Option<String>,
    /// list all payload
    #[argh(switch, short = 'l')]
    pub list: bool,
}

fn default_thread() -> u32 {
    100_u32
}

fn default_timeout() -> u64 {
    10
}

impl EmoArgs {
    pub fn new() -> Self {
        let default: EmoArgs = argh::from_env();
        default
    }
    pub fn get_method(&self) {}
}
