use iced::{Align, Button, Column, Element, Sandbox, Settings, Text, button};

pub fn main() -> iced::Result {
    Kobold::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Message {
    BackPressed,
    NextPressed,
}

struct Kobold {
    back_button: button::State,
    next_button: button::State,
    value: isize,
}

impl Sandbox for Kobold{
    type Message = Message; 

    fn new() -> Kobold{
        return Kobold {
            back_button: button::State::new(),
            next_button: button::State::new(),
            value: 0,
        }
    }
    
    fn title(&self) -> String {
        return String::from("Kobold - Project Manager")
    }
    
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::BackPressed => {
                self.value -= 1;
            }
            Message::NextPressed => {
                self.value += 1;
            }
        }
    }
    
    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.next_button, Text::new("Increment"))
                    .on_press(Message::NextPressed)
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.back_button, Text::new("Decrement"))
                    .on_press(Message::BackPressed)
            )
            .into()
    }
}