function main()
    while true do
        Rust.println("Choose:\n 1) Rock\n 2) Paper\n 3) Scissors!")

        local user_answer = Rust.readln()

        if user_answer == "1" or user_answer == "2" or user_answer == "3" then
            -- Avoid unnecesary logic for victory condition
            if (1 == Rust.rand_int(1, 3)) then
                Rust.println("You win!")
            else
                Rust.println("You lose!")
            end

            -- "Try again" option
            Rust.println("Try again? [y/N]")
            local continue = Rust.readln()

            if not (continue == "Y" or continue == "y") then
                break
            end
        else
            Rust.println("Choice is not valid.")
        end
    end

    Rust.println("Exiting game...")
end
