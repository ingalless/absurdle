use leptos::*;
use rand::seq::IteratorRandom;
use gloo_net::http::Request;

async fn get_target_word() -> Result<String, Box<dyn std::error::Error>> {
    let resp = Request::get("https://gist.githubusercontent.com/dracos/dd0668f281e685bad51479e5acaadb93/raw/6bfa15d263d6d5b63840a8e5b64e04b382fdb079/valid-wordle-words.txt")
        .send()
        .await?
        .text()
        .await?;
    let lines = resp.lines().map(|c| c);
    let word = lines.choose(&mut rand::thread_rng()).expect("No lines");
    Ok(word.to_string())
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App /> });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (word, set_word) = create_signal(cx, "".to_string());
    let (guesses, set_guesses) = create_signal(cx, Vec::new());

    wasm_bindgen_futures::spawn_local(async move { 
        let target_word = get_target_word().await.expect("Failed to fetch word");
        set_word(target_word.clone());
        log!("{}", &target_word);
    });

    view! { cx,
        <h1>"Absurdle"</h1>
        <h1>{word.get()}</h1>
        <Show
            when=move || match guesses().last().cloned() {
                Some(w) => w == word(),
                None => false
            }
            fallback=|cx| view! {cx, <span></span>}
        >
            <p>"You win!"</p>
        </Show>
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
        <form on:submit=move |ev| {
                ev.prevent_default();
                set_guesses.update(move |guesses| {
                    guesses.push(guess());
                });
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
