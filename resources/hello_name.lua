function main()
    Rust.println("What is your name?")

    local name = Rust.readln()

    Rust.println("Hello "..name.."!")
end