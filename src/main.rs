use iced::{button, executor, text_input, Button, Column, Command, Element, Settings, Text, TextInput, Row, Align, Length, Container, Sandbox, Color};

#[derive(Debug)]
struct Fulgrim {
    requests: Vec<Request>,
    add_button: button::State,
    selected_request: Option<usize>,
    request_text_input: text_input::State,
}

#[derive(Debug, Clone)]
pub enum FulgrimMessage {
    Add,
    Remove,
    ChangeSelectedRequest(usize),
    ChangeRequestBody(String, usize),
}

#[derive(Debug, Clone)]
struct Request {
    state: button::State,
    name: String,
    body: String,
}

impl Sandbox for Fulgrim {
    type Message = FulgrimMessage;

    fn new() -> Self {
        Self {
                requests: Vec::new(),
                add_button: button::State::default(),
                selected_request: None,
                request_text_input: text_input::State::default(),
            }
    }

    fn title(&self) -> String {
        String::from("Fulgrim")
    }

    fn update(&mut self, message: Self::Message) {

        match message {
            FulgrimMessage::Add => self.requests.push(Request {
                state: button::State::default(),
                name: "New request".to_string(),
                body: String::new(),
            }),
            FulgrimMessage::Remove => {
                self.requests.pop();
            }
            FulgrimMessage::ChangeSelectedRequest(i) => {
                self.selected_request = Some(i);
            }
            FulgrimMessage::ChangeRequestBody(new_body, selected_request_index) => {
                self.requests[selected_request_index].body = new_body;
            }
        }
    }

    fn view(&mut self) -> Element<FulgrimMessage> {
        let mut main_container = Row::new();
        let mut top_row = Row::new();
        top_row = top_row.push(
            Button::new(&mut self.add_button, Text::new("Add request"))
                .on_press(FulgrimMessage::Add),
        ).width(Length::Fill);
        let mut center_row = Row::new();
        let mut center_column = Column::new();
        if let Some(selected_request_index) = self.selected_request {
            if let Some(current_request) = self.requests.get(selected_request_index) {
                let text_input = TextInput::new(
                    &mut self.request_text_input,
                    "Type something to continue...",
                    &current_request.body,
                    move |new_body| FulgrimMessage::ChangeRequestBody(new_body, selected_request_index),
                ).width(Length::Fill);
                center_column = center_column.push(text_input);
            }
        }
        let mut left_column = Column::new().padding(1).spacing(1);
        for (i, req) in self.requests.iter_mut().enumerate() {
            left_column = left_column.push(
                Button::new(&mut req.state, Text::new(&req.name))
                    .on_press(FulgrimMessage::ChangeSelectedRequest(i)),
            );
        }
        let right_column = Column::new();
        let bottom_row = Row::new();
        center_row = center_row.push(left_column).push(center_column).push(right_column);
        let container : Element<FulgrimMessage> = main_container.push(top_row).push(center_row).push(bottom_row).spacing(10).padding(10).into();
        container.explain(Color::BLACK)
    }
}

fn main() {
    Fulgrim::run(Settings::default());
}
