use i3ipc::{event::Event, reply::Node, I3Connection, I3EventListener, MessageError, Subscription};

fn do_a_split(connection: &mut I3Connection, node: &Node) -> Result<(), MessageError> {
    let (height, width) = (node.rect.2, node.rect.3);
    let command = if height > width { "splith" } else { "splitv" };
    connection.run_command(command).map(|_| ())
}

fn main() {
    let mut listener = I3EventListener::connect()
        .expect("Failed to connect to the i3 IPC interface as a listener");
    let mut connection =
        I3Connection::connect().expect("Failed to connect to the i3 IPC interface");

    listener
        .subscribe(&[Subscription::Window])
        .expect("Failed to subscribe to the i3 IPC interface");

    for event_result in listener.listen() {
        match event_result {
            Ok(Event::WindowEvent(e)) => {
                if let Err(err) = do_a_split(&mut connection, &e.container) {
                    eprintln!("{}", err);
                }
            }
            Ok(_) => {}
            Err(err) => eprintln!("{}", err),
        }
    }
}
