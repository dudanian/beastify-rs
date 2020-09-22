use super::browser;
use browser::tabs::Tab;
use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::Element;
use web_sys::MouseEvent;

// we need to use the *same exact* JsValue when adding/removing CSS
thread_local! {
static HIDE_PAGE: JsValue = {JsValue::from("body > :not(.beastify-image) {
        display: none;
    }")};
}

macro_rules! object_value {
    (=> $val:expr) => {
        &JsValue::from($val)
    };
    (: $val:expr) => {
        $val
    };
}

// helper to create objects
// not efficient, but convenient
macro_rules! object {
    ($($key:literal $sep:tt $val:expr),*) => {
        {
        let o = js_sys::Object::new();
        $(
            js_sys::Reflect::set(
                &o,
                &JsValue::from_str($key),
                object_value!($sep $val),
            ).unwrap();
        )*
        o
    }
    };
}

fn listen_for_clicks() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // one of the rare times we actually want to use a Closure
    // since we need to convert it to a Function
    let cb = Closure::wrap(Box::new(move |e: MouseEvent| {
        spawn_local(async move {
            if let Some(target) = e.target() {
                if let Some(elem) = target.dyn_ref::<Element>() {
                    let p = browser::tabs::query(
                        object! {
                            "active" => true,
                            "currentWindow" => true
                        }
                        .as_ref(),
                    );

                    if elem.class_list().contains("beast") {
                        match JsFuture::from(p).await {
                            Ok(tabs) => beastify(Array::from(&tabs), elem.text_content().unwrap()),
                            Err(error) => report_error(error),
                        }
                    } else if elem.class_list().contains("reset") {
                        match JsFuture::from(p).await {
                            Ok(tabs) => reset(Array::from(&tabs)),
                            Err(error) => report_error(error),
                        }
                    }
                }
            }
        });
    }) as Box<dyn FnMut(MouseEvent)>);

    document
        .add_event_listener_with_callback("click", cb.into_js_value().as_ref().unchecked_ref())
        .unwrap();
}

fn beast_name_to_url(beast_name: &str) -> JsValue {
    let path = match beast_name {
        "Frog" => "icons/beasts/frog.jpg",
        "Snake" => "icons/beasts/snake.jpg",
        "Turtle" => "icons/beasts/turtle.jpg",
        _ => return JsValue::null(),
    };

    browser::runtime::get_url(path)
}

fn beastify(tabs: Array, text_content: String) {
    let tab = tabs.get(0);
    let tab: Tab = tab.unchecked_into();

    if let Some(id) = tab.id() {
        HIDE_PAGE.with(|hide_page| {
            let p = browser::tabs::insert_css(
                object! {
                    "code" : hide_page
                }
                .as_ref(),
            );

            spawn_local(async move {
                match JsFuture::from(p).await {
                    Ok(_) => {
                        let url = beast_name_to_url(&text_content);

                        let p = browser::tabs::send_message(
                            id,
                            object! {
                                "command" => "beastify",
                                "beastURL" : &url
                            }
                            .as_ref(),
                        );
                        JsFuture::from(p).await.unwrap();
                    }
                    _ => (),
                }
            });
        });
    }
}

fn reset(tabs: Array) {
    let tab = tabs.get(0);
    let tab: Tab = tab.unchecked_into();

    if let Some(id) = tab.id() {
        HIDE_PAGE.with(|hide_page| {
            let p = browser::tabs::remove_css(
                object! {
                    "code" : hide_page
                }
                .as_ref(),
            );

            spawn_local(async move {
                match JsFuture::from(p).await {
                    Ok(_) => {
                        let p = browser::tabs::send_message(
                            id,
                            object! {
                                "command" => "reset"
                            }
                            .as_ref(),
                        );
                        JsFuture::from(p).await.unwrap();
                    }
                    _ => (),
                }
            });
        });
    }
}

fn report_error(error: JsValue) {
    console::error_2(&JsValue::from("Could not beastify:"), &error);
}

fn report_execute_script_error(error: JsValue) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    if let Some(elem) = document.query_selector("#popup-content").unwrap() {
        elem.class_list().add_1("hidden").unwrap();
    }
    if let Some(elem) = document.query_selector("#error-content").unwrap() {
        elem.class_list().remove_1("hidden").unwrap();
    }
    console::error_2(
        &JsValue::from_str("Failed to execute beastify content script:"),
        &error,
    );
}

pub fn execute() {
    spawn_local(async {
        let p = browser::tabs::execute_script(&object! {
            "file" => "/beastify.js"
        });
        match JsFuture::from(p).await {
            Ok(_) => listen_for_clicks(),
            Err(err) => report_execute_script_error(err),
        }
    });
}
