// unimplemented!()
    // let form_field = |name: &str, placeholder: &str, type_: &str| -> Html<LoginMsg> {
    //     let ref input_id: String = format!("{}", rand::random::<u16>());
    //     markup!(
    //         label(
    //             text_transform: "uppercase"
    //             font_family: "'Source Sans Pro', sans-serif"
    //             font_size: "1em"
    //             color: "#656565"
    //             for = {input_id}
    //             text(name)
    //         )
    //         input(
    //             ::placeholder (
    //                 color: "#666"
    //             )
    //             text_transform: "lowercase"
    //             font_family: "'Source Sans Pro', sans-serif"
    //             font_size: "1em"
    //             width: "100%"
    //             outline: "none"
    //             border: "1px solid #b1b1b1"
    //             border_radius: "3px"
    //             padding_left: "8px"
    //             id = {input_id}
    //             placeholder={placeholder}
    //             font_size: "1.1em"
    //             padding: "2px"
    //             padding_left: "6px"
    //             type={type_}
    //             text("")
    //         )
    //     )
    // };
    // 
    // 
    // let panel = |title: &str, form: Html<LoginMsg>| -> Html<LoginMsg> {markup!(
    //     width: "100%"
    //     background_color: "#fff"
    //     max_width: "400px"
    //     padding: "12px"
    //     @media [min_width: "900px"] (
    //         margin: "0 auto"
    //         margin_top: "60px"
    //     )
    //     @media [max_width: "900px"] (
    //         margin: "0 auto"
    //         margin_top: "0"
    //     )
    //     h1(
    //         color: "#5a5a5a"
    //         font_family: "'Source Sans Pro', sans-serif"
    //         text_align: "center"
    //         margin: "0"
    //         padding_bottom: "20px"
    //         text(title)
    //     )
    //     self.append(vec![form])
    // )};
    // 
    // 
    // 
    // 
    // let user_login = panel("Log In", markup!(form|
    //     border_radius: "3px"
    //     // css.select("{self} > *")(
    //     //     margin_bottom: "18px"
    //     // ),
    //     // css.select("{self} > *:last-child")(
    //     //     margin_bottom: "0px"
    //     // )
    //     self.append(&[
    //         form_field("Account Name", "Name", "text"),
    //         form_field("Password", "Password", "password"),
    //     ])
    //     input(
    //         ::placeholder (
    //             color: "#666"
    //         )
    //         margin_top: "12px"
    //         color: "#5a5a5a"
    //         text_transform: "lowercase"
    //         font_family: "'Source Sans Pro', sans-serif"
    //         font_size: "1em"
    //         width: "100%"
    //         outline: "none"
    //         border: "1px solid #b1b1b1"
    //         border_radius: "3px"
    //         font_size: "1.1em"
    //         padding: "2px"
    //         padding_left: "6px"
    //         font_size: "1.2em"
    //         text_transform: "uppercase"
    //         type="submit"
    //         .click(move |event| {
    //             let event: web_sys::Event = From::from(event);
    //             event.prevent_default();
    //             LoginMsg::NoOp
    //         })
    //         text("Submit")
    //     )
    // ));
    // 
    // 
    // let create_account = panel("Create Account", markup!(form|
    //     border_radius: "3px"
    //     // css.select("{self} > *")(
    //     //     margin_bottom: "18px"
    //     // ),
    //     // css.select("{self} > *:last-child")(
    //     //     margin_bottom: "0px"
    //     // )
    //     self.append(&[
    //         form_field("Account Name", "Name", "text"),
    //         form_field("Password", "Password", "password"),
    //         form_field("Re-Enter Password", "Password", "password")
    //     ])
    //     input(
    //         ::placeholder (
    //             color: "#666"
    //         )
    //         margin_top: "12px"
    //         color: "#5a5a5a"
    //         text_transform: "lowercase"
    //         font_family: "'Source Sans Pro', sans-serif"
    //         font_size: "1em"
    //         width: "100%"
    //         outline: "none"
    //         border: "1px solid #b1b1b1"
    //         border_radius: "3px"
    //         font_size: "1.1em"
    //         padding: "2px"
    //         padding_left: "6px"
    //         font_size: "1.2em"
    //         text_transform: "uppercase"
    //         type="submit"
    //         .click(move |event| {
    //             let event: web_sys::Event = From::from(event);
    //             event.prevent_default();
    //             LoginMsg::NoOp
    //         })
    //         text("Submit")
    //     )
    // ));
    // 
    // markup!(
    //     background_color: "#ececec"
    //     width: "100%"
    //     height: "100%"
    //     @media [min_width: "900px"] (
    //         display: "grid"
    //         grid_template_columns: "0.5fr 1fr"
    //         grid_column_gap: "20px"
    //     )
    //     @media [max_width: "900px"] (
    //         display: "grid"
    //         grid_template_columns: "1fr"
    //         grid_row_gap: "20px"
    //     )
    //     div(
    //         width: "100%"
    //         height: "100%"
    //         background_color: "#fff"
    //         {user_login}
    //     )
    //     div(
    //         width: "100%"
    //         height: "100%"
    //         background_color: "#fff"
    //         {create_account}
    //     )
    // )
    markup!(
        h1(text("login"))
    )