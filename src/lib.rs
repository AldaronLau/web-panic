#![recursion_limit = "512"]

use std::panic::PanicInfo;

use html::{
    content::{Header, Heading1},
    text_content::{Division, PreformattedText, ThematicBreak},
};
use web_sys::DomException;

/// Panic hook for use with [`std::panic::set_hook()`].
///
/// Opens a new screen in the DOM with panic information.
pub fn hook(panic_info: &PanicInfo) {
    let stack = DomException::new().unwrap().stack();
    let stack = trim_trace(&stack);
    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();
    let payload = format!("{panic_info}\n\n{stack}");
    let error = PreformattedText::builder()
        .style(
            "background-color:#d47031;\
             border-width:1px;\
             border-color:black;\
             border-style:solid;\
             padding:12px;\
             margin:auto 12px;\
             border-radius:8px;\
             font-size:18px;\
             font-family:monospace;\
             overflow-x: scroll;\
             overflow-y: hidden;\
             scrollbar-color: #FFFF #0000;\
             scrollbar-width: 4px;\
             ",
        )
        .push(payload)
        .build();
    let header = Header::builder()
        .style(
            "margin:0 12px 0;\
             border:0;\
             padding:0;\
             ",
        )
        .push(
            Heading1::builder()
                .style(
                    "margin:0;\
                     border:0;\
                     padding:0;\
                     font-size:22px;\
                     ",
                )
                .push("Oh, no — Something went wrongly!")
                .build(),
        )
        .build();
    let popup = Division::builder()
        .style(
            "background-color:#de742c;\
             color:black;\
             outline: 4px solid black;\
             width:100%;\
             max-width:1000px;\
             margin:12px auto;\
             border-radius:8px;\
             border:0;\
             padding:12px 0 12px;\
             overflow-y: scroll;\
             overflow-x: hidden;\
             scrollbar-color: #FFFF #0000;\
             scrollbar-width: 4px;\
             ",
        )
        .push(header)
        .push(
            ThematicBreak::builder()
                .style(
                    "margin:12px;\
                     color:black;\
                     border:0;\
                     outline:1px solid black;\
                     ",
                )
                .build(),
        )
        .push(error)
        .build();
    let div = Division::builder()
        .style(
            "display:grid;\
             height:100vh;\
             width:100%;\
             border:0;\
             margin:0;\
             padding:0;\
             position:absolute;\
             top:0;\
             left:0;\
             background-color:#180C08FF;\
             ",
        )
        .push(popup)
        .build();

    body.set_inner_html(&div.to_string());
}

fn trim_trace(stack_trace: &str) -> &str {
    const EXC: &str = "web_sys::features::gen_DomException::DomException::new";

    let Some(index) = stack_trace.find(EXC) else {
        return stack_trace;
    };
    let new = &stack_trace[index..];
    let Some(index) = new.find('\n') else {
        return stack_trace;
    };
    let stack_trace = &new[index + 1..];
    let Some(index) = stack_trace.find("std::rt::lang_start") else {
        return stack_trace;
    };

    &stack_trace[..index]
}
