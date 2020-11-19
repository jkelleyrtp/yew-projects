use yew::{html, Html, Properties};
use yewtil::{Pure, PureComponent};

#[derive(PartialEq, Default, Debug, Clone, Properties)]
pub struct FizzBuzzProps {
    pub num: u32,
}

impl PureComponent for FizzBuzzProps {
    fn render(&self) -> Html {
        let text: String = match (self.num % 3, self.num % 5) {
            (0, 0) => format!("FizzBuzz"),
            (0, ..) => format!("Fizz"),
            (.., 0) => format!("Buzz"),
            _ => format!("{}", self.num),
        };

        html!(
            <li>
                {text}
            </li>
        )
    }
}
pub type FizzBuzz = Pure<FizzBuzzProps>;
