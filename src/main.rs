use calc::Calc;
use druid::widget::{ CrossAxisAlignment, Flex, Label, Painter };
use druid::{
    theme, AppLauncher, Color, Data, FontDescriptor, FontFamily, FontWeight, Lens,
    LocalizedString, RenderContext, Widget, WidgetExt, WindowDesc,
};

#[derive(Clone, Data, PartialEq)]
enum State {
    Set,
    Non
}

#[derive(Clone, Data, Lens)]
struct CalcState {
    show: String,
    value: String,
    state: State
}

fn show_lens(len: String) -> String {
    let valid = len.chars().into_iter()
        .map(|x| x.to_string()).collect::<Vec<_>>();
    if valid.len() > 25 {
        valid[valid.len()-25..].concat()
    } else { valid.concat() }
}

fn oper_repl(repl: String) -> String {
    repl.replace("÷", "/").replace("×", "*")
}

fn fun_button_label(fun: &str, label: String) -> impl Widget<CalcState> {
    let painter = Painter::new(|ctx, _, _env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &Color::rgb8(0x50, 0x85, 0x0));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }
        if ctx.is_active() {
            ctx.fill(bounds, &Color::rgb8(0x60, 0x95, 0x10));
        }
    });

    Label::new(fun.to_string())
        .with_font(FontDescriptor::with_weight(
                FontDescriptor::new(FontFamily::new_unchecked("Cantarell")),
                FontWeight::BOLD))
        .with_text_size(16.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut CalcState, _env| {
            if let State::Set = data.state {
                data.value = label.clone();
                data.show = label.clone();
                data.state = State::Non;
            } else if data.value == "0" {
                data.value = label.clone();
                data.show = label.clone();
            } else {
                data.value += &label;
                data.show = show_lens(data.value.clone());
            }
        })
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
        .with_font(FontDescriptor::with_weight(
                FontDescriptor::new(FontFamily::new_unchecked("Cantarell")),
                FontWeight::BOLD))
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut CalcState, _env| {
            match op {
                'C' => {
                    data.value = String::from("0");
                    data.show = String::from("0");
                },
                '←' => {
                    if data.value.len() == 1 {
                        data.value = String::from("0");
                        data.show = String::from("0");
                    } else {
                        data.value.pop();
                        data.show = show_lens(data.value.clone());
                    }
                },
                '=' => {
                    data.state = State::Set;
                    if data.value != "0" {
                        match Calc::new(oper_repl(data.value.clone())).run_round(Some(7)) {
                            Ok(valid) => { data.value = valid.clone(); data.show = show_lens(valid) },
                            Err(msg) => { data.value = String::from("0"); data.show = msg }
                        }
                    }
                },
                '.' => {
                    if let State::Set = data.state {
                        data.value = String::from("0");
                        data.show = String::from("0");
                        data.state = State::Non;
                    } else {
                        data.value += &label;
                        data.show = show_lens(data.value.clone());
                    }
                },
                ch @ '(' | ch @ '−' | ch @ 'π' => {
                    match data.state {
                        State::Set => {
                            data.state = State::Non;
                            if ch == '−' && data.value != "0" {
                                data.value += &label;
                                data.show = show_lens(data.value.clone());
                            } else {
                                data.value = label.clone();
                                data.show = label.clone();
                            }
                        },
                        State::Non => {
                            if data.value == "0" {
                                data.value = label.clone();
                                data.show = label.clone();
                            } else {
                                data.value += &label;
                                data.show = show_lens(data.value.clone());
                            }
                        }
                    }
                },
                _ => {
                    if let State::Set = data.state {
                        data.value += &label;
                        data.show = show_lens(data.value.clone());
                        data.state = State::Non;
                    } else {
                        data.value += &label;
                        data.show = show_lens(data.value.clone());
                    }
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
        .with_font(FontDescriptor::with_weight(
                FontDescriptor::new(FontFamily::new_unchecked("Cantarell")),
                FontWeight::BOLD))
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
        .on_click(move |_ctx, data: &mut CalcState, _env| {
            if let State::Set = data.state {
                data.value = digit.clone();
                data.show = digit.clone();
                data.state = State::Non;
            } else if data.value == "0" {
                data.value = digit.clone();
                data.show = digit.clone();
            } else {
                data.value += &digit;
                data.show = show_lens(data.value.clone());
            }
        })
}

fn flex_row<T: Data>(
    w1: impl Widget<T> + 'static,
    w2: impl Widget<T> + 'static,
    w3: impl Widget<T> + 'static,
    w4: impl Widget<T> + 'static,
    w5: impl Widget<T> + 'static,
    w6: impl Widget<T> + 'static,
    w7: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_flex_child(w1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w2, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w3, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w4, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w5, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w6, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w7, 1.0)
}

fn build_calc() -> impl Widget<CalcState> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
        .with_font(FontDescriptor::with_weight(
                FontDescriptor::new(FontFamily::new_unchecked("Consolas")),
                FontWeight::BOLD))
        .with_text_size(28.0)
        .lens(CalcState::show)
        .padding(5.0);
    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display)
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(
            flex_row(
                op_button('^'),
                op_button('('),
                op_button(')'),
                op_button('÷'),
                op_button_label('π', String::from("P")),
                op_button('←'),
                op_button('C'),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(String::from("7")),
                digit_button(String::from("8")),
                digit_button(String::from("9")),
                op_button('×'),
                fun_button_label("Cos", String::from("cos(")),
                fun_button_label("Sin", String::from("sin(")),
                fun_button_label("Tan", String::from("tan(")),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(String::from("4")),
                digit_button(String::from("5")),
                digit_button(String::from("6")),
                op_button_label('−', String::from("-")),
                fun_button_label("Cosh", String::from("cosh(")),
                fun_button_label("Sinh", String::from("sinh(")),
                fun_button_label("Tanh", String::from("tanh(")),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                digit_button(String::from("1")),
                digit_button(String::from("2")),
                digit_button(String::from("3")),
                op_button('+'),
                fun_button_label("Abs", String::from("abs(")),
                fun_button_label("Log", String::from("logx(")),
                fun_button_label("Sqrt", String::from("sqrt(")),
            ),
            1.0,
        )
        .with_spacer(1.0)
        .with_flex_child(
            flex_row(
                op_button('%'),
                digit_button(String::from("0")),
                op_button('.'),
                op_button('='),
                fun_button_label("Fac", String::from("fac(")),
                fun_button_label("Ln", String::from("ln(")),
                fun_button_label("Exp", String::from("exp(")),
            ),
            1.0,
        )
}

pub fn main() {
    let window = WindowDesc::new(build_calc)
        .window_size((392., 305.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-window-title")
            .with_placeholder("Senior Calculator")
        );
    let calc_state = CalcState {
        show: String::from("0"),
        value: String::from("0"),
        state: State::Non
    };
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(calc_state)
        .expect("launch failed");
}
