enum CommunicationProtocol {
    Udp,
    Tcp,
}

enum MessageFormat {
    Raw,
    Scpi,
}

struct Interpreter<'a> {
    port: Option<u16>,
    address: Option<&'a str>,
    message_format: Option<MessageFormat>,
    communication_protocol: Option<CommunicationProtocol>,
}
