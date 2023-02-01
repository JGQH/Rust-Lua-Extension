function main()
    local rolled_dices = 0
    local rolled_sum = 0

    while true do
        Rust.println("You've rolled " .. rolled_dices .. " dices that sum to " .. rolled_sum ..
            ", roll another one? [y/N]")

        local user_answer = Rust.readln()

        if user_answer == "y" or user_answer == "Y" then
            local roll = Rust.rand_int(1, 6)

            rolled_dices = rolled_dices + 1
            rolled_sum = rolled_sum + roll

            Rust.println("You rolled a " .. roll .. "!")
        else
            break
        end
    end

    Rust.println("Exiting game...")
end
