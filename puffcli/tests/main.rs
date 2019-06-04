extern crate assert_cmd;

#[cfg(test)]
mod integration {
    use std::process::Command;

    #[test]
    fn calling_without_anything_should_print_help() {
        let output = Command::new("./target/debug/puffcli login")
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "no command\n");
    }

    // #[test]
    // fn calling_rget_with_invalid_url() {
    //     let output = Command::new("./target/debug/rget")
    //         .arg("wwww.shouldnotwork.com")
    //         .output()
    //         .expect("failed to execute process");

    //     assert!(String::from_utf8_lossy(&output.stderr).contains(INVALID_URL_OUTPUT));
    // }
}
