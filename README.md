# gitbro

# code coverage

1. install cargo subcommand llvm-cov
    $> cargo install cargo-llvm-cov

2. generate report in console
    $> cargo llvm-cov test

3. generate report in html
    $> cargo llvm-cov --html