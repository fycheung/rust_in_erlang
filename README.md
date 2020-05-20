Example use Rust build a nif for Erlang

> cargo build

    Finished dev [unoptimized + debuginfo] target(s) in 0.60s

> erl
Erlang/OTP 20 [erts-9.3] [source] [64-bit] [smp:2:2] [ds:2:2:10] [async-threads:10] [hipe] [kernel-poll:false]

Eshell V9.3  (abort with ^G)
1> c(test_inf).
Runs on library load
                    {ok,test_inf}
2> test_inf:add(1,2).
3
