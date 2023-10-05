use leptos::*;

fn main() {
    // _ = console_log::init_with_level(log::Level::Debug);
    // console_error_panic_hook::set_once();

    logging::log!("CSR mode - mounting to body.");

    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="p-5 flex flex-col justify-center items-center gap-4">
            <p>
                {move || count.get()}
            </p>
            <button class="px-3 py-2 bg-rose-800 rounded-md transition-colors hover:bg-rose-700" on:click=move |_| {
                set_count.update(|n| *n += 1);
            }>
                "Click me!"
            </button>
        </div>
    }
}
