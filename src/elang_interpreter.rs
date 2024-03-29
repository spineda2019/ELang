enum CommunicationProtocol {
    Udp,
    Tcp,
}

enum MessageFormat {
    Raw,
    Scpi,
}

pub struct ElangInterpreter<'a> {
    port: Option<u16>,
    address: Option<&'a str>,
    message_format: Option<MessageFormat>,
    communication_protocol: Option<CommunicationProtocol>,
}

impl<'a> ElangInterpreter<'a> {
    pub fn new() -> Self {
        Self {
            port: None,
            address: None,
            message_format: None,
            communication_protocol: None,
        }
    }

    pub fn launch_interactive_shell(&self) {}

    pub fn interpret_file(&self, file_name: &str) {}
}
