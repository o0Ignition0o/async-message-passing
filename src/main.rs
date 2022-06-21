// I m going to put all of my commands here
// so I can have one big enum I can match on
enum Command {
    SayHello(String),
    Exit,
}

// say_hello takes a name, waits for a second, and then returns a string
async fn say_hello(name: String) -> String {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    format!("Hello {name}!")
}

fn main() {
    // this channel has 2 ends: a sender and a receiver
    let (command_sender, command_receiver) = std::sync::mpsc::sync_channel(10);
    let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(10);

    let thread = std::thread::spawn(move || {
        // Create the runtime
        let rt = tokio::runtime::Runtime::new().expect("couldnt create tokio runtime");

        // Spawn the root task
        rt.block_on(async {
            // Receive commands
            while let Ok(command) = command_receiver.recv() {
                match command {
                    Command::SayHello(name) => {
                        // run our async function
                        let hello_message = say_hello(name).await;
                        // send it through the response channel
                        response_sender
                            .send(hello_message)
                            .expect("couldn't send response");
                    }
                    Command::Exit => {
                        println!("received exit, closing the thread in 5 seconds");

                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        return;
                    }
                }
            }
        });
    });

    println!("sending say hello command");

    command_sender
        .send(Command::SayHello("jeremy".to_string()))
        .expect("couldn't send command");

    let response = response_receiver.recv().expect("couldn't receive response");

    println!("got response {response}");

    println!("sending exit command");

    // Send an exit command to let the thread stop
    command_sender
        .send(Command::Exit)
        .expect("couldn't send command");

    thread.join().expect("something didnt work in the thread");

    println!("thread has exited, quitting the program.");
}
