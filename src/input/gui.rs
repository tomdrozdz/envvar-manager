use std::sync::mpsc::{channel, Sender};

use anyhow::{Context, Result};

use iced::widget::{button, center, row, text, text_input};
use iced::window::Level;
use iced::{window, Center, Element, Size, Task, Theme};

struct InputWindow {
    label: String,
    input: String,
    channel: Sender<String>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    InputSubmitted,
    FocusInput,
}

impl InputWindow {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
                Task::none()
            }
            Message::InputSubmitted => {
                self.channel
                    .send(self.input.clone())
                    .expect("Failed to send input from GUI");

                window::get_latest().and_then(window::close)
            }
            Message::FocusInput => text_input::focus("input"),
        }
    }

    fn view(&self) -> Element<Message> {
        let text_input = text_input(&self.label, &self.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::InputSubmitted)
            .id("input")
            .padding(10)
            .size(32);

        let button = button(text("Submit").size(32))
            .on_press(Message::InputSubmitted)
            .padding(10);

        let content = row![text_input, button]
            .align_y(Center)
            .spacing(10)
            .padding(20);

        center(content).into()
    }
}

pub fn get_input(prompt: &str) -> Result<String> {
    let (sender, receiver) = channel::<String>();
    let input_window = InputWindow {
        label: prompt.to_string(),
        input: String::new(),
        channel: sender,
    };

    iced::application(
        "EnvVar Manager: Input window",
        InputWindow::update,
        InputWindow::view,
    )
    .window_size(Size::new(900.0, 120.0))
    .centered()
    .level(Level::AlwaysOnTop)
    .theme(|_| Theme::Dark)
    .run_with(|| (input_window, Task::done(Message::FocusInput)))?;

    let output = receiver
        .recv()
        .context("Failed to receive input from GUI")?;

    Ok(output)
}
