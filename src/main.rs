use calc::Calc;
use druid::widget::{CrossAxisAlignment, Flex, Label, Painter};
use druid::{
    theme, AppLauncher, Color, Data, Lens, LocalizedString,
    RenderContext, Widget, WidgetExt,WindowDesc,
};

#[derive(Clone, Data, PartialEq)]
enum State {
    Set,
    Non
}

#[derive(Clone, Data, Lens)]
struct CalcState {
    value: String,
    state: State
}

fn op_button_label(op: char, label: String) -> impl Widget<CalcState> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }
        if ctx.is_active() {
            ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
        }
    });

    Label::new(op.clone().to_string())
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut CalcState, _env| {
            match op {
                'π' => {
                    if let State::Set = data.state {
                        data.value = label.clone();
                        data.state = State::Non;
                    } else if data.value.len() == 1 && data.value == "0" {
                        data.value = label.clone();
                    } else { data.value += &label; }
                },
                'C' => { data.value = "0".to_string(); },
                '←' => {
                    if data.value.len() == 1 {
                        data.value = "0".to_string();
                    } else { data.value.pop(); }
                },
                '=' => {
                    data.state = State::Set;
                    if data.value.len() == 1 && data.value == "0" {
                        data.value = "0".to_string();
                    } else {
                        match Calc::new(data.value.clone()).run_round(Some(7)) {
                            Ok(valid) => data.value = valid,
                            Err(msg) => data.value = msg
                        }
                    }
                },
                '.' => {
                    if let State::Set = data.state {
                        data.value = "0".to_string();
                        data.state = State::Non;
                    } else { data.value += &label; }
                },
                _ => {
                    if let State::Set = data.state {
                        data.value += &label;
                        data.state = State::Non;
                    } else if data.value.len() == 1 && data.value == "0" {
                        data.value = label.clone();
                    } else { data.value += &label; }
                },
            }
        })
}

fn op_button(op: char) -> impl Widget<CalcState> {
    op_button_label(op, op.to_string())
}

fn digit_button(digit: String) -> impl Widget<CalcState> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &env.get(theme::BACKGROUND_LIGHT));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }
        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x71, 0x71, 0x71));
        }
    });

    Label::new(digit.clone())
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut CalcState, _env| {
            if let State::Set = data.state {
                data.value = digit.clone();
                data.state = State::Non;
            }else if data.value.len() == 1 && data.value == "0" {
                data.value = digit.clone();
            } else { data.value += &digit; }
        })
}

fn flex_row<T: Data>(
    w1: impl Widget<T> + 'static,
    w2: impl Widget<T> + 'static,
    w3: impl Widget<T> + 'static,
    w4: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_flex_child(w1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w2, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w3, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w4, 1.0)
}

fn build_calc() -> impl Widget<CalcState> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(32.0)
        .lens(CalcState::value)
        .padding(5.0);
    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display)
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(
            flex_row(
                op_button('C'),
                op_button_label('π', "P".to_string()),
                op_button('←'),
                op_button_label('÷', '/'.to_string()),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(7.to_string()),
                digit_button(8.to_string()),
                digit_button(9.to_string()),
                op_button_label('×', '*'.to_string()),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(4.to_string()),
                digit_button(5.to_string()),
                digit_button(6.to_string()),
                op_button_label('−', '-'.to_string()),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(1.to_string()),
                digit_button(2.to_string()),
                digit_button(3.to_string()),
                op_button('+'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                op_button('%'),
                digit_button(0.to_string()),
                op_button('.'),
                op_button('='),
            ),
            1.0,
        )
}

pub fn main() {
    let window = WindowDesc::new(build_calc)
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-window-title").with_placeholder("Simple Calculator"),
        );
    let calc_state = CalcState {
        value: "0".to_string(),
        state: State::Non
    };
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(calc_state)
        .expect("launch failed");
}
