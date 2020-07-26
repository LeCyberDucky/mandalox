use iced::{image, Column, Container, Element, Image, Length, Sandbox, Settings};

fn main() {
    Mandalox::run(Settings::default())
}

// Application state
// #[derive(Default)] // We create own default below
struct Mandalox {
    // background: Image,
    background: image::Handle,
}

impl Default for Mandalox {
    fn default() -> Mandalox {
        Mandalox {
            // background: Image::new("bridge.jpg"),
            background: image::Handle::from_path("bridge.jpg"),
        }
    }
}

// User interactions and commands: Messages
#[derive(Debug, Clone, Copy)]
pub enum Message {
    LeftClick,
    // RightClick,
    // LeftDrag,
    // RightDrag,
    // ScrollUp,
    // ScrollDown,
    // Space,
}


impl Sandbox for Mandalox {
    type Message = Message;

    // Initialize application
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Mandalox")
    }

    // Update logic
    // Handles messages (user interactions or commands) and updates state of application
    fn update(&mut self, message: Message) {
        match message {
            Message::LeftClick => {
                println!("Click!");
            }
        }
    }

    // Returns the widgets to display in the application 
    // Widgets can produce messages based on user interaction
    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            // TODO: Figure out if this keeps loading image from disk. Is this necessary? 
            .push(Image::new(self.background.clone()));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

}






// use iced::{image, Column, Command, Container, Element, Image, Sandbox, Settings};

// fn main() {
//     Mandalox::run(Settings::default())
// }

// // Application state
// // #[derive(Default)]
// struct Mandalox {
//     //original_image: png,
//     background: Image,
// }

// impl Default for Mandalox {
//     fn default() -> Mandalox {
//         Mandalox {
//             background: Image::new("bridge.jpg"),
//         }
//     }
// }


// // User interactions: Messages
// #[derive(Debug, Clone, Copy)]
// pub enum Message {
//     LeftClick,
//     // right_click,
//     // left_drag,
//     // right_drag,
//     // scroll_up,
//     // scroll_down,
//     // space,
// }

// impl Sandbox for Mandalox {
//     type Message = Message;

//     // Initialize application
//     // Return initial state of app
//     fn new() -> Self {
//         Self::default()
//     }

//     fn title(&self) -> String {
//         String::from("Mandalox")
//     }

//     // Update logic 
//     // Handles messages (user interactions or commands) and updates state of application
//     // Any Command returned will be exectued immediately in the background (multithreading?)
//     fn update(&mut self, message: Message) {
//         match message {
//             Message::LeftClick => {
//                 // Left click action
//                 println!("Oy");
//             }
//         }
//     }

//     // Returns the widgets to display in the application
//     // Widgets can produce messages based on user interaction 
//     fn view(&mut self) -> Element<Message> {
//         let content = Column::new()
//             .push(
//                 Image::new(self.background.clone()),
//             )
//         // Column::new()
//         //     .push(
//         //         self.background.clone(),
//         //     )
//         //     .into()

//         // Container::new()
//         //     .push(
//         //         Image::new(&mut self.background),
//         //     )
//         //     .into()
//     }
// }

// // View logic
// impl Mandalox {
//     pub fn view(&mut self) -> //Image? 
//     // Image::new()
//     //  .push()
// }

// // Update logic
// impl Mandalox {
//     pub fn update(&mut self, message: Message) {
//         match message {
//             Message::left_click => {
//                 // Add logic for what happens on left click
//             }
//         }
//     }
// }




// fn main() {

// }

// #[derive(Debug)]
// struct Triangle {
//     corners: Vec<Point>,
// }

// #[derive(Debug)]
// struct Point {
//     x: u16,
//     y: u16,
// }


// // Edge detection
// // Edge simplification

// use iced::{button, Application, Button, Column, Command, Element, Settings, Text};

// fn main() {
//     Counter::run(Settings::default())
// }

// #[derive(Default)]
// struct Counter {
//     value: i32,
//     increment_button: button::State,
//     decrement_button: button::State,
// }

// #[derive(Debug, Clone, Copy)]
// enum Message {
//     IncrementPressed,
//     DecrementPressed,
// }

// impl Application for Counter {
//     type Message = Message;

//     fn new() -> (Self, Command<Message>) {
//         (Self::default(), Command::none())
//     }

//     fn title(&self) -> String {
//         String::from("A simple counter")
//     }

//     fn update(&mut self, message: Message) -> Command<Message> {
//         match message {
//             Message::IncrementPressed => {
//                 self.value += 1;
//             }
//             Message::DecrementPressed => {
//                 self.value -= 1;
//             }
//         }

//         Command::none()
//     }

//     fn view(&mut self) -> Element<Message> {
//         Column::new()
//             .push(
//                 Button::new(&mut self.increment_button, Text::new("Increment"))
//                     .on_press(Message::IncrementPressed),
//             )
//             .push(
//                 Text::new(self.value.to_string()).size(50),
//             )
//             .push(
//                 Button::new(&mut self.decrement_button, Text::new("Decrement"))
//                     .on_press(Message::DecrementPressed),
//             )
//             .into()
//     }
// }


