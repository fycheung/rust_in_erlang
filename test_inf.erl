-module (test_inf).

-export([add/2]).

-on_load(init/0).


init() ->
    ok = erlang:load_nif("target/debug/libtest_inf", none).

add(_X, _Y) ->
    exit(nif_library_not_loaded).
