setup-project

setup-project is a simple command-line tool written in Rust that helps you copy predefined folders (setup-folder or cargo-folder) to a specified destination path.

Features

Copy the contents of setup-folder or cargo-folder to a destination.

Recursively copies all files and subdirectories.

Simple and intuitive CLI.


Installation

1. Clone this repository:

git clone <repository_url>
cd setup-project


2. Build the project using cargo:

cargo build --release


3. The compiled binary will be located at ./target/release/setup-project.


4. Add the binary to your system's PATH (optional):

mv ./target/release/setup-project /usr/local/bin/



Usage

setup-project <command> <destination>

Commands

setup: Copies the setup-folder to the specified destination.

cargo: Copies the cargo-folder to the specified destination.


Examples

1. Copy setup-folder to /path/to/destination:

setup-project setup /path/to/destination


2. Copy cargo-folder to /path/to/destination:

setup-project cargo /path/to/destination



Folder Configuration

Replace "setup-folder" and "cargo-folder" in the code with the actual paths to the folders you want to copy.


Development

Prerequisites

Install Rust.


Run Tests

To ensure everything works correctly, you can run the included tests:

cargo test

Modify the Code

1. Open the project in your favorite Rust-compatible editor (e.g., Zed).


2. Edit the source files as needed.


3. Rebuild the binary using cargo build.



License

This project is licensed under the MIT License.

Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any changes or enhancements.


---

This README.md provides users with clear instructions for installing, using, and developing your project. Replace <repository_url> with the actual URL of your repository.




| Error Code                     | Message                                                                  |
|--------------------------------|--------------------------------------------------------------------------|
| otpErrorResendPeriod           | Cannot resend OTP now, please try again after 10 seconds                |
| otpErrorResendNotExists        | E-mail OTP verification with id '10' doesn't exist                      |
| otpErrorInvalid                | Invalid OTP                                                             |
| otpErrorExpired                | OTP Expired                                                             |
| otpErrorAccountBlocked         | Your E-mail address is blocked, please contact our support team         |
| otpErrorAccountLocked          | Account has been locked for 1 hours and 5 minutes                       |
| otpErrorIpBlocked              | Your IP address is blocked, please contact our support team             |
| passwordErrorAccountDisabled   | Your account is disabled                                                |
| passwordErrorAccountLocked     | Account has been locked for 10 minutes                                  |
| passwordErrorInvalid           | Invalid E-mail or password                                              |
| passwordErrorResetMismatch     | New password cannot be same as old password                             |
| passwordErrorResetInvalid      | Invalid current password                                                |
| passwordErrorAlreadyUsed       | Already used                                                            |
| passwordErrorNotValid          | Validation error                                                        |
