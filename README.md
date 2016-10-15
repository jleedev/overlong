Encode and decode overlong UTF-8 text.

Examples

    $ printf 'c1 89 c0 a0 c1 b0 c1 b2 c1 a5 c1 a6 c1 a5 c1 b2 c0 a0 c1 96 c1 94 c1 86 c0 ad c0 b8 c1 af c1 b6 c1 a5 c1 b2 c1 a2 c1 b9 c1 b4 c1 a5' | xxd -r -p | ./target/release/overlong 
    I prefer VTF-8overbyte$

    $ printf 'ｎｅｒｄ　ｓｎｉｐｅ\n' | ./target/release/overlong Four | xxd -p 
    f08fbd8ef08fbd85f08fbd92f08fbd84f0838080f08fbd93f08fbd8ef08f
    bd89f08fbd90f08fbd85f080808a
    $
