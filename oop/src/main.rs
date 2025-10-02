use oop::gui::{Draw, Button, Screen};
use oop::blog1::Post as Post1;
use oop::blog2::Post as Post2;

#[allow(dead_code)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    "yes".into(),
                    "maybe".into(),
                    "no".into()
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "OK".into()
            }),
        ]
    };

    screen.run();

    let mut post1 = Post1::new();
    post1.add_text("I had an entire rotisserie chicken for breakfast today");
    assert_eq!("", post1.content());

    post1.request_review();
    assert_eq!("", post1.content());

    post1.approve();
    assert_eq!("I had an entire rotisserie chicken for breakfast today", post1.content());

    let mut post2 = Post2::new();
    post2.add_text("I had one (1) grain of rice for dinner tonight");

    let post2 = post2.request_review();

    let post2 = post2.approve();
    assert_eq!("I had one (1) grain of rice for dinner tonight", post2.content());
}