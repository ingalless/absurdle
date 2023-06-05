use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App /> });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (guesses, set_guesses) = create_signal(cx, Vec::new());
    view! { cx,
        <h1>"Absurdle"</h1>
        <ErrorBoundary
            fallback=|cx, errors| view! { cx,
                <div style="border: 1px solid red">
                    <p>"Errors: "</p>
                    <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                            .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
            }
        >
            <For
                each=guesses
                key=|guess| String::from(guess)
                view=move |cx, guess| { view! { cx, <Row guess /> } }
            />
            <Guess set_guesses=set_guesses />
        </ErrorBoundary>
    }
}

#[component]
fn Guess(cx: Scope, set_guesses: WriteSignal<Vec<String>>) -> impl IntoView {
    let (guess, set_guess) = create_signal(cx, "".to_string());
    let disabled = move || guess().len() != 5;

    view! { cx,
        <input
            prop:value=guess
            type="text"
            on:input=move |ev| {
                set_guess(event_target_value(&ev));
            }
            style="margin-top: 20px; padding: 6px;"
        />
        <button
            prop:disabled=disabled
            on:click=move |_| {
                set_guesses.update(move |guesses| {
                    guesses.push(guess());
                });
                set_guess("".to_string());
            }
        >
            "Send"
        </button>
    }
}

#[component]
fn Row(cx: Scope, guess: String) -> impl IntoView {
    view! {cx,
        <div style="display: flex">
        {guess.clone().chars().map(|c| view! { cx,
             <div style="display:flex; align-items: center; justify-content: center; border:1px solid black; height: 40px; width:40px">
                 {c.to_string()}
             </div> }
         ).collect_view(cx)}
        </div>
    }
}
