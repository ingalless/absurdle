use leptos::*;
use rand::seq::IteratorRandom;

async fn get_target_word() -> String {
    let resp = reqwasm::http::Request::get("https://gist.githubusercontent.com/dracos/dd0668f281e685bad51479e5acaadb93/raw/6bfa15d263d6d5b63840a8e5b64e04b382fdb079/valid-wordle-words.txt")
        .send()
        .await
        .unwrap();
    let text = resp.text().await.unwrap();
    let lines = text.lines().map(|c| c);
    let word = lines.choose(&mut rand::thread_rng()).expect("No lines");
    word.to_string()
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App /> });
}

fn check_guess_against_word(cx: Scope, last_guess: String, target_word: Resource<(), String>) -> bool {
    match target_word.read(cx) {
        Some(w) => last_guess == w,
        None => false
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (guesses, set_guesses) = create_signal(cx, Vec::new());
    let (last_guess, set_last_guess) = create_signal(cx, "".to_string());

    let word = create_local_resource(cx, || (), |_| get_target_word());

    let has_won = move || check_guess_against_word(cx, last_guess(), word);

    let word_view = move || {
        view! {cx, <p>{word.read(cx)}</p>}
    };

    view! { cx,
        <h1>"Absurdle"</h1>
        {word_view}
        <Show
            when=has_won
            fallback=|cx| view! { cx, <p></p> }
        >
            <p>"You won!"</p>
        </Show>
        <For
            each=guesses
            key=|guess| String::from(guess)
            view=move |cx, guess| { view! { cx, <Row guess /> } }
        />
        <Guess set_guesses=set_guesses set_last_guess=set_last_guess />
    }
}

#[component]
fn Guess(cx: Scope, set_last_guess: WriteSignal<String>, set_guesses: WriteSignal<Vec<String>>) -> impl IntoView {
    let (guess, set_guess) = create_signal(cx, "".to_string());
    let disabled = move || guess().len() != 5;

    view! { cx,
        <form on:submit=move |ev| {
                ev.prevent_default();
                set_guesses.update(move |guesses| {
                    guesses.push(guess());
                });
                set_last_guess(guess());
                set_guess("".to_string());
            }
        >
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
            >
                "Send"
            </button>
        </form>
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
